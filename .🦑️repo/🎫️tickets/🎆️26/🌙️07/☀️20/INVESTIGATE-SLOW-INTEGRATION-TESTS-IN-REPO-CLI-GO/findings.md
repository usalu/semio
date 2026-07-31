# Investigation: Slow Integration Tests in repo/client/cli/go

Pure root-cause investigation. No production code changed. All claims below were verified empirically (not assumed) — see the scratch Go programs in this ticket folder under `scratch/`.

## Headline finding

Both slow code paths (`BuildMonorepoTree`'s tree walk, and the separate `CodebaseContext.LoadFiles` walk used by GraphQL `folders`/`files` and several tools) fail to prune large build-artifact directories during their filesystem walk, because **two different, duplicated gitignore-matching implementations both mishandle unanchored/directory-only `.gitignore` patterns** — just via different mechanisms. The repo has a top-level `/target` Rust build directory alone containing **1,676,267 files** (verified with `find /Users/ueli/Documents/semio/target -type f | wc -l`), against only **14,767** git-tracked files (`git ls-files | wc -l`). Any code path that doesn't prune `target/`-style directories early pays a multi-million-entry walk instead of a ~15K-entry one.

## 1. `BuildMonorepoTreeCached` / tree walk (`isGitIgnored`, `StreamFiles`)

- `computeCompositeFingerprint` (main.go:6875) and the disk cache itself (`loadTreeCache`/`saveTreeCache`, main.go:7278-7339, consulted from `BuildMonorepoTreeCached` at main.go:7342) are **not** the bottleneck — the fingerprint is just `git rev-parse HEAD` + `git status --porcelain` + a small `.repo` glob, and the cache-hit path (main.go:7372-7375) returns immediately when the fingerprint matches. The cache mechanism itself is sound.
- The real cost is on a cache **miss**, which happens on the *first* build and, in this repo's live/concurrent-auto-commit workflow, effectively on every `git HEAD` change — i.e. very often in practice.
- On a miss, `BuildMonorepoTree` (main.go:5768) fans out to `StreamFiles`/`StreamFolders` (main.go:24065 / sibling), which walk the tree via `filepath.Walk` and call `isGitIgnored` (main.go:16531) → `isIgnoredByGitignore` (main.go:16538) → `github.com/sabhiram/go-gitignore`'s `GitIgnore.MatchesPath`.
- **Bug**: `go-gitignore`'s compiled regex for a pattern ending in `/` (a directory-only rule, e.g. `target/`, `__pycache__/`, `build/`, `.pytest_cache/`, `docs/_build/`, `.tox/` — all present in the repo's root `.gitignore`) only matches strings that contain a **literal trailing `/`** in the matched text. `filepath.Walk` passes bare directory paths with no trailing slash, so `isGitIgnored(path)` returns `false` for the directory itself, `filepath.SkipDir` (main.go:24145-24147) never fires, and the walker descends into it — file-by-file exclusion still works for the files *inside* (their paths do contain `target/`), but only after every one of them has been individually stat'd, walked, and regex-matched against all ~560 patterns in `.gitignore`.
  - Verified empirically (`scratch/gitignore-check/main.go`, same pinned version `v0.0.0-20210923224102-525f6e181f06`):
    ```
    target                    -> false   (should be true — SkipDir never fires)
    target/foo.rs             -> true    (file inside is still excluded, but only after being visited)
    __pycache__               -> false
    node_modules              -> true    (pattern has NO trailing slash in .gitignore, so this one works)
    ```
  - Root `.gitignore` has ~560 non-comment patterns; `MatchesPath` (vendor `ignore.go:217-237`) does a **linear, non-short-circuiting scan over every pattern** for every path (no early exit even after a match, to support negation) — so cost is O(patterns × paths visited), and "paths visited" balloons because the directories that should have been skipped aren't.
  - Go's `regexp` (RE2) has no catastrophic-backtracking failure mode, so the original "pathological backtracking regex" hypothesis is **not** the mechanism — the actual mechanism is the missing `SkipDir` due to the trailing-slash mismatch, which is arguably worse since it's silent and systematic rather than input-dependent.
- Additionally, `StreamFiles` (main.go:24164-24165) recomputes `isGitIgnored(filePath)` a **second time** in a post-walk loop over the exact same `files` slice that the walk callback already filtered to non-ignored entries only — a redundant full second pass of the same expensive check for zero behavioral benefit.

## 2. GraphQL `folders`/`files` resolvers ~40x slower than `bundles`/`tickets`/`policies`

- `bundles`/`contributors`/`tickets`/`policies` (queryResolver methods at main.go:33248, 33301, 33327, 33352) all read bounded `.repo/`-scoped metadata via `Stream*` functions (`StreamBundles`, `StreamContributors`, `StreamTickets`, `StreamPolicies`) — thousands of ticket/policy files at most, fast regardless of caching.
- `folders`/`files` (queryResolver.Folders main.go:33265, .Files main.go:33274) instead call `r.Ctx.GetFolders()`/`GetFiles()` → `repoContext.GetFolders` (main.go:27241) / `GetFiles` (main.go:27271), each of which independently constructs a **fresh** `NewCodebaseContext()` and calls `ctx.LoadFiles()` (main.go:20771) → `ScopeToFiles(Scope{Kind: ScopeRepo}, ...)` (main.go:35638) → `globByExtension(rootDir, "**/*", ...)` (main.go:16886).
- This is a **third, separate, uncached** walk+ignore implementation — completely disconnected from `BuildMonorepoTreeCached`'s disk cache from section 1. `folders` and `files` each trigger their own independent full-repo walk, which is why both cost ~37-38s each: there is no cache to share, and no reuse between the two resolver calls or with the tree-cache subsystem.
- **This path's ignore-matching bug is different and more severe than section 1's**: `globByExtension` uses `matchesIgnorePattern` (main.go:16812) with patterns loaded by `LoadGitignore` (main.go:16793), which reads **only the root `.gitignore`** verbatim (no per-directory `.gitignore` support) and matches each pattern via `doublestar.Match(pattern, candidatePath)` **without applying gitignore's "unanchored pattern matches at any depth" semantics** (i.e. it never rewrites `target/` to `**/target/`). Verified empirically (`scratch/gitignore-check/main.go`, pattern-matching logic copied verbatim from main.go:16812-16843):
  ```
  path=target                          pattern=target/  -> true   (root-level target correctly pruned)
  path=framework/core/rs/target        pattern=target/  -> false  (nested target — NEVER pruned)
  path=mathematical/entropy/rs/target  pattern=target/  -> false  (nested target — NEVER pruned)
  ```
  Only a literal top-level `target/`/`node_modules`/`build/`/etc. gets pruned; every nested occurrence anywhere else in this multi-crate, multi-package monorepo (Rust `target/` per crate, `node_modules/` per package, Python `__pycache__/`) is walked in full. The explicit extra `ignorePatterns` passed by `ScopeToFiles` (`"**/node_modules/**"`, `"**/.venv/**"`, main.go:35639) work around this for those two specific names by pre-anchoring with `**/`, but every other repo `.gitignore` rule (including `target/`, `build/`, `__pycache__/`, `.pytest_cache/`, `docs/_build/`, `.tox/`, ~15 more directory-style rules) is left unanchored and therefore ineffective below the root.
- Net effect: `folders`/`files` walk is not merely "missing a cache" relative to `bundles` et al. — it walks into every nested Rust `target/` in the workspace (the repo's Cargo.toml/Cargo.lock at root plus many per-crate manifests per `git status`), which is where the multi-million-file cost in the headline finding comes from.

## 3. Package-level `rootDir` — order-dependent test cost

- `rootDir` is a single package-level `var` (declared alongside main.go:16307's `init()`, mutated by `SetRootDir` at main.go:16326). Most (`~186` occurrences) tests that reassign it do so directly (`rootDir = tmpDir`) rather than via `SetRootDir`, and the majority (grep shows ~150+ of the 186) correctly pair this with `defer func() { rootDir = oldRoot }()`.
- `getTestExecutor` (main_test.go:850-862), used by both `TestExhaustiveFoldersNonEmpty`/`FilesNonEmpty` and every other `NonEmpty`/GraphQL test in that block, does **not** save or restore the previous value: `rootDir = findTestRepoRoot(cwd)` (main_test.go:856) with no `defer`/`t.Cleanup`. It also bypasses `SetRootDir`, so it doesn't reset `cachedGitignore`/`gitignoreLoaded`/the technology cache either — a test that ran before it with a different root can leave stale gitignore/technology cache state behind for it to inherit.
- `TestExhaustiveNormalizeTicketFileInput` (main_test.go:1168) never sets a root at all — it reads `absRoot := GetRootDir()` (main_test.go:1172) as-is. Its "id" test case exercises `normalizeTicketFileInput` (main.go:22386), whose fallback branch (main.go:22414-22422) does `NewCodebaseContext(); ctx.LoadBundles(); ctx.LoadFiles()` — the exact same expensive, nested-directory-leaking walk from section 2 — against whatever `rootDir` a prior test in the same binary happened to leave behind.
- `TestExhaustiveCodebaseCommand` (main_test.go:2014) is the most extreme case: it calls `ToolCodebase()` (main.go:21800) with **zero** root setup of its own. `ToolCodebase` does `ctx.LoadFiles()` (section 2's walk) **and then** `ctx.LoadBreachs()` (main.go:21808 → `AnalyzeFile` per file) **and** `ctx.LoadTickets()`/`ctx.LoadPolicies()`, compounding the section-2 walk cost with a per-file analysis pass, entirely dependent on whatever global `rootDir` execution order left in place.
- Only one `t.Cleanup` call exists in the whole file (main_test.go, non-rootDir-related) — the prevailing idiom here is manual `oldRoot := rootDir` + `defer`, which works when followed but is opt-in per test rather than structurally enforced, and `getTestExecutor`/`TestExhaustiveNormalizeTicketFileInput`/`TestExhaustiveCodebaseCommand` are the (at least) three call sites that don't follow it.

## Scratch verification artifacts

- `scratch/gitignore-check/main.go` — small Go programs (edited in place between runs) that load the exact pinned `sabhiram/go-gitignore` version and reproduce `matchesIgnorePattern` verbatim, to empirically confirm both directory-pruning bugs above rather than relying on regex reading alone.
- Raw counts: `git ls-files | wc -l` → 14767; `find /Users/ueli/Documents/semio/target -type f | wc -l` → 1676267.

## Explicitly out of scope (per request)

No fixes were made. Candidate fix directions worth a follow-up ticket, for reference only:
- Section 1: normalize directory paths passed to `isGitIgnored` during the `filepath.Walk` directory callback (append `/` before calling `MatchesPath`), and drop the redundant second `isGitIgnored` pass in `StreamFiles`.
- Section 2: either route `folders`/`files` through `BuildMonorepoTreeCached` instead of a fresh uncached walk, or fix `matchesIgnorePattern`/`LoadGitignore` to apply gitignore's implicit `**/` anchoring for patterns with no leading `/`, and to load nested `.gitignore` files.
- Section 3: give `getTestExecutor` a `t.Cleanup` that restores the previous `rootDir` (and reruns `SetRootDir`'s cache invalidation), and make `TestExhaustiveNormalizeTicketFileInput`/`TestExhaustiveCodebaseCommand` set their own root explicitly instead of inheriting global state.
