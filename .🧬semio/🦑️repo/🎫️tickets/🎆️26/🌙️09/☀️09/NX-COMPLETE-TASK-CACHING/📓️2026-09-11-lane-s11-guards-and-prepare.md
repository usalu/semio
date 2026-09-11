# Lane S11 — Generator freshness guards + dev `prepare` caching (2026-09-11)

Session `⚪83140bef0e504e03b0b3380912b12a0e`. Repo MCP failed to connect (`repo`: invalid initialize params,
`semio`: connection closed); ticket bookkeeping kept on disk manually, per the coordinator/S8 pattern.

Scope: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` — generator `checkTarget` freshness guards
(A), `playgroundPreparationTargets()` `prepare-*` caching (B), and the language-agnostic cache-contract
fixtures/tests for both (C). Background: `📓️2026-09-11-single-shared-cache-plan.md`,
`📓️2026-09-11-coordinator-log.md`, `📓️2026-09-11-lane-s8-nx-input-precision.md` (the
`targetScriptClosure`/`genericTargetCommandInputs`/`genericCommandFallbackInputs` mechanism this lane builds
on), `📓️2026-09-11-lane-s3-os-dev.md` (the wgpu `prepare`/`activate`/`stageWgpuPluginModules` design this
lane's B decision depends on), `📓️2026-09-09-cache-contract-policy.md`, `📓️2026-09-09-leftover-authored-cache.md`.

## Critical finding first: Nx cannot hash a gitignored file, no matter how it's listed

Before touching A's design, I ran the empirical experiment the brief demanded (throwaway git-initialized
fixture workspaces under `🗑️generated/s11/hash-fixture-git`, `-fresh`, deleted after use — see "Hash
experiment" below for the full trail). The naive design — list `contract.outputRoots` as plain
`{workspaceRoot}/path` (or `path/**/*`) glob strings directly in a target's `inputs` array, expecting them to
survive `default`'s `!...generated.../**/*` exclusion because they're a separate array entry — **does not
work**, for a reason that has nothing to do with named-input exclusion:

- Nx's task hasher only ever hashes files it can see through its own git-based workspace file map. A file
  matched by `.gitignore` is invisible to that map — **every entry in `inputs`, named or literal, silently
  fails to see it**, regardless of exclusion patterns elsewhere.
- Every `outputRoots` entry in every `generatorContracts[*]` in `🔣️taxonomy.json` is either
  `"inclusion": "ignored"` (matches `.gitignore` — confirmed for `ui-axes`'s two roots via
  `git check-ignore -v`, `git ls-files` returns nothing for either) or `"tracked"`. The `"ignored"` ones are
  the exact files the freshness guard exists to check, and they are exactly the files Nx cannot hash via a
  fileset pattern.
- Proven with a properly `git init`+committed throwaway fixture (not the ticket's own gitignored tree, which
  produces a *different*, misleading failure mode — see below): editing a **tracked** nested file changes the
  task hash through any pattern (named or literal, root or nested — nesting/literal-vs-named were red herrings
  from an earlier, git-uninitialized attempt at the fixture). The moment the same file is added to
  `.gitignore` and edited again, the hash stops changing, even though the target's `inputs` array explicitly
  names it. This reproduces with 100% consistency across three independent throwaway fixtures.
- A `{ runtime: "<command>" }` input is not subject to this at all — Nx just spawns the command and hashes its
  stdout, every time, which means it reads the file live off disk regardless of git status. Verified this
  *does* pick up an edit to a gitignored file (`4723281749744620977` → after edit, hash unchanged when using a
  glob input; switching the same target to a `runtime` digest command, the hash changed
  `16309845402695517258` → `4673901043320102508` on the identical edit).

So `outputRootInputs()` builds a `{ runtime: "node -e <digest script> <absolute path>" }` entry per output
root instead of a glob. The digest script (`OUTPUT_ROOT_DIGEST_SCRIPT`, a compact self-contained
`node -e` one-liner, no external deps) walks the path with `lstatSync` (files and directories both, symlinks
silently skipped, a missing path hashes to the sha256 of nothing rather than erroring), so it's correct for
both file-shaped (`README.md`) and directory-shaped (`🤖️generated/`) `outputRoots`. It's applied uniformly to
every `outputRoots` entry regardless of its declared `inclusion` — simpler than branching, and correct for
`"tracked"` roots too (marginally more overhead: one extra `node` spawn per output root per hash, in line with
the existing `{ runtime: 'node -p "process.platform.concat(process.arch)"' }`-style fingerprints already used
throughout this file for platform/toolchain probes).

## A. Generator freshness guards — design

`generatorContractInputs(contract)` (new): the contract's own `inputPatterns` as `{workspaceRoot}/path`
entries, plus — when `contract.inputDiscovery` is set — the `{ dependentTasksOutputFiles }` fingerprint and
its owning target as a `dependsOn` entry. Factored out of the existing generate-target contract loop (lines
~643–649 before this change) so the same discovery inputs can be reused for the checkTarget guard.

`outputRootInputs(output, workspaceRoot)` (new): the `{ runtime: ... }` digest entry described above.

Two call sites in `projectWithDefaults()`:
- The **generate**-target contract loop (was lines 643–656): unchanged in effect, refactored to call
  `generatorContractInputs`; the nested `if (contract.checkTarget) { declared[check].cache = false }` block
  was **removed as dead code** — `targetPolicy()`'s cacheable-family branch
  (`{ inputs: [...], outputs: [], ...target, cache: true }`) always puts `cache: true` *after* the `...target`
  spread, so it unconditionally overrides whatever `cache` a target object carried in; every real
  `checkTarget` name matches `cacheableFamily` (`check`/`check-*` prefix), so this early forcing never had any
  observable effect — confirmed by reading the function, not assumed.
- The **checkTarget guard** loop (was lines 671–675, the one with real effect since it runs *after*
  `targetPolicy` has already computed `normalized[check]`): now, instead of unconditionally forcing
  `cache: false`, it takes whatever inputs the main normalize loop already assigned (native script closure via
  `nativeSources`/`nativeTargetCommandInputs` for the exact-name `check` targets, or `default` +
  `genericTargetCommandInputs` for `check-*` targets), appends `generatorContractInputs(contract).inputs` and
  `contract.outputRoots.map(o => outputRootInputs(o, workspaceRoot))`, adds the fingerprint's `dependsOn` when
  present, and sets `cache: true`.

## A latent bug this unmasked, found and fixed: `cacheableFamily`'s `/generator/` substring match

Not part of A/B's original scope, but a direct, provable consequence of B's `POLICY.uncached` edit (see
below), so squarely this lane's responsibility to fix. `cacheableFamily` had:

```
... || /generator/.test(name) && !name.includes("generator-inputs")
```

unanchored — matching "generator" **anywhere** in the name, not just as a prefix (unlike every other
alternative in the same function, which is anchored to the start via the leading `^(...)`). There is a real
playground variant literally named `generator` (`prepare-generator-react-dev`,
`prepare-generator-wgpu-dev`, …). While `"prepare"` was in `POLICY.uncached`, `matchesUncached` short-circuited
`targetPolicy` before ever reaching `cacheableFamily`, so this was dormant. The moment `"prepare"` came out of
`POLICY.uncached` (B, below), `prepare-generator-wgpu-dev` — which must stay `cache: false` (see B) — started
matching `cacheableFamily` via the substring hit and got forced `cache: true`, silently defeating its own
authored `cache: false`. Caught by a broad sweep of **every** `prepare-*-react-*`/`prepare-*-wgpu-*` pair
across all 61 playground variants (see "Verification" below), not just the one variant I'd spot-checked first
— a narrower check would have missed it. Fixed by anchoring the same way every other family in the function
already is: `/^generator(?:-|$)/` instead of `/generator/`. Confirmed no fixture or real target relies on the
old unanchored behavior (`generator-inputs` is the only real name containing "generator" besides this one
playground-variant collision; it stays excluded either way). I left the sibling `/(?:^|-)contract(?:-|$)/`
clause untouched — no evidence it's broken today (no target/variant literally named `contract`), and touching
it isn't needed for this lane's scope.

## B. Dev `prepare` caching — design

Read `PreparationScript`/`ActivationScript`/`stageWgpuPluginModules` in
`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts` (owned by lane S3, who built the wgpu path this decision
depends on).

- **React's `PreparationScript`** performs zero writes: it reads `browserModuleRoot(profile)` (the
  `materialize-<profile>` Nx output) and the fonts asset, asserts the session/descriptor/bridge/support files
  it expects are present, and throws if not. It's a pure validator over its dependency closure — safe to
  cache with `outputs: []` (an Nx cache hit just means "the last time these exact dependency outputs existed,
  the validation passed" — skips redoing the check, restores nothing, same convention already used for
  `catalog-smoke`-style checks per lane S3's report).
- **WGPU's `PreparationScript`** additionally calls `stageWgpuPluginModules`, which copies into `pluginOutRoot`
  = `PLUGIN_MODULES_ROOT` — a **fixed, non-variant/profile-scoped** path
  (`🧑‍💻dev/🔌️plugin-modules/`) that a running `dev`/hot-swap session also writes into directly. A cache
  hit there could restore/skip writing a *different* variant's stale content into that shared live directory —
  exactly lane S3's stated reason for leaving the `plugin` target uncached in `📋️project.json`. So WGPU's
  `prepare-*` targets are **left `cache: false`**, documented in a docstring on `playgroundPreparationTargets`
  itself (not an inline comment — CLAUDE.md forbids comments inside definitions).

Design in code: `prepare-<variant>-react-<profile>` gets `cache: true`, `outputs: []`,
`inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }]` (own `dependsOn` closure — session,
support, fonts, every selected engine's `wasm`, every selected plugin's `materialize-<profile>` — hashed
transitively, exactly as the task specified) on top of the existing `dependsOn` list, unchanged. The main
`projectWithDefaults` normalize loop then appends the target's own script closure
(`genericTargetCommandInputs`) automatically, same mechanism S8 built, since this target is neither native nor
artifact-TypeScript and I set `policy.inputs` to something non-`undefined` so the `?? ["default","^default"]`
fallback never fires — the inputs stay precise (no `default`/`production` blanket).

`POLICY.uncached` (`⚡️caching/🔣️policy.json`): removed `"prepare"` — kept `"activate"` (per the brief: activate
publishes into a live runtime root/receipt and stays uncached; it's already cheap when nothing changed via its
own SHA-256 content-compare short-circuit in `publishActivatedExtension`, no code change needed there).
Audited every *other* `prepare`/`prepare-*` target in the repo before removing this (`rg -n '"prepare'` across
every `📋️project.json`): root `workspace:prepare` and mit-bestand-demonstrator's `prepare-dev`/`prepare-release`/
`prepare-e2e` all declare `cache: false` explicitly already, so `targetPolicy`'s last-resort branch
(`{...target, cache: target.cache !== false}`) leaves them exactly as authored — removing `"prepare"` from the
policy list only changes behavior for targets that don't already pin `cache` explicitly, i.e. only my new
react `prepare-*` targets (explicit `true`) and wgpu `prepare-*` targets (explicit `false`, unchanged from
before).

`activate-*`/`serve-*`/`dev-*` (react and wgpu): untouched, `cache: false, continuous`/`parallelism: false` as
before.

## Hash experiment (throwaway fixtures, `🗑️generated/s11/hash-fixture*`, deleted after use)

1. First attempt (`hash-fixture`, `hash-fixture-fresh`) was **not** its own git repo — just a directory under
   the ticket's own `🗑️generated/`, which is itself blanket-gitignored (`.gitignore:20:🗑️generated`). This
   produced a *different*, misleading symptom: even a plain `{ "default": ["{projectRoot}/**/*"] }` positive
   glob failed to see edits to any **nested** file, while root-level files worked fine — a red herring caused
   by the fixture having no real git history at all (nothing for Nx's git-based walker to discover past the
   top level).
2. Redone as its own git repo (`hash-fixture-git`: `git init`, committed once) to remove that confound. With
   real git history, a **tracked** nested file's edits are picked up correctly through `default`, a bare
   literal glob, or a named+wrapped glob — nesting and literal-vs-named were never the issue.
3. `git rm --cached` the same file, add it to `.gitignore`, commit, edit again: hash stops changing through
   every glob-style input (`nestedExact`: `2645817146417409707` unchanged before/after edit). Switch the same
   target to a `{ runtime: "node -e <digest> <path>" }` input: hash changes on the same edit
   (`16309845402695517258` → `4673901043320102508`).
4. This is the exact shape of every real `outputRoots` entry (`"inclusion": "ignored"`, gitignored, sitting
   inside an otherwise-tracked parent directory) — confirmed directly against the two real `ui-axes` output
   paths via `git check-ignore -v` / `git ls-files` (both return "ignored"/nothing).

## Verification

### Plugin health
`node --check 🟨️.mjs` and `NX_DAEMON=false bunx nx show projects` (exit 0) after every edit, including
immediately after the `cacheableFamily` fix.

### `nx show project <p> --json`
- `@semio-tech/ui-rs:check` (exact-name native bucket): `cache: true`, inputs include
  `nativeSources`/`^nativeSources` (own script closure), the three `inputPatterns` entries
  (`📋️project.json`, `📜️script.ts`, `🔣️ui-axes.json`), and two `{ runtime: "node -e … <abs path>" }` entries
  for the two `outputRoots` (`generated.rs`, `ui-axes.ts`), resolved to real absolute paths.
- `@semio-tech/plugin-registry:check-generated` (generic bucket, has `inputDiscovery`): `cache: true`,
  `dependsOn` includes `repo:generator-inputs`, inputs include the
  `{ dependentTasksOutputFiles: ".../📇️registry/🔣️.json" }` fingerprint plus two `runtime` digest entries
  (`.vscode/launch.json`, the `🤖️generated` directory).
- `@semio-tech/framework-os-dev` (prepare/wgpu/activate targets): every `prepare-<variant>-react-<dev|release>`
  across all 61 playground variants (`prepare-aggregator-…` through `prepare-writer-…`) shows `cache: true`;
  every sibling `prepare-<variant>-wgpu-<profile>` shows `cache: false`.

### Standalone assertion replay against the live 700-project graph
`testCacheContracts()` (the full suite — see "Left undone" below) aborts before reaching either A's or B's
assertions on a pre-existing, unrelated failure. Rather than not verify at all, I wrote a standalone script
(scratch, not kept) that calls the same `inventory(root)` the real test uses and replays the *exact* predicates
I added to `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` (see C) against the live graph:

- All **14/14** `generatorContracts[*].checkTarget` entries in `🔣️taxonomy.json` (not just the 4 covered by
  `🧫️fixtures/nx-contract/🔣️.json`'s `vectors.generators`): `cache: true`, every `inputPatterns` entry present,
  every `outputRoots` entry present as a `runtime` digest naming its resolved absolute path,
  `plugin-registry` (the one with `inputDiscovery`) additionally has the fingerprint `dependentTasksOutputFiles`
  input and `dependsOn` — **all PASS**.
- All **122** `prepare-*-react-*` targets (61 variants × dev/release): `cache: true`, `outputs: []`,
  `{ dependentTasksOutputFiles: "**/*", transitive: true }` present. All 122 matching `prepare-*-wgpu-*`
  siblings: `cache: false`. **All PASS** — this run is what caught the `cacheableFamily` bug above (an earlier,
  single-variant spot check had missed it).

### Real `bunx nx run` cache-hit evidence
- `@semio-tech/ui-rs:check`: run 1 (`--skip-nx-cache`) executes for real ("ui axes are fresh …"); run 2 (no
  flag, empty cache from the skip) misses (`Cache: 0/1 hit`); run 3 →
  `[existing outputs match the cache, left as is]`, "Nx read the output from the cache instead of running the
  command for 1 out of 1 tasks", `Cache: 1/1 hit (100%)`.
- `@semio-tech/plugin-registry:check-generated`: run 1 executes for real (32.3s, `repo:generator-inputs` +
  itself both run); run 2 → `✔ nx run repo:generator-inputs` (reruns — correctly still uncached) then
  `[existing outputs match the cache, left as is]` for `check-generated` itself, "1 out of 2 tasks" from cache,
  `Cache: 1/2 hit (50%)`.
- `@semio-tech/framework-os-dev:prepare-draw-react-dev` (smallest playground variant, 8 upstream deps — real
  build, `CARGO_PROFILE_WASM_DEV_DEBUG=false`, shared cache dir, no private `CARGO_TARGET_DIR`): run 1 cold,
  11m39s, real component/materialize builds for `draw-plugin`, 0/18 hit. Run 2 (~1 min later) landed mid a
  burst of concurrent repo activity from other lanes — `puzzle-plugin:wasm` and `draw-plugin:component-dev`/
  `materialize-dev` themselves re-ran (their own upstream sources had changed, not mine), so `prepare-draw-…`
  correctly propagated a miss too (9/18 hit — real dependency content changed, so a miss here is *correct*
  behavior, not a bug). Run 3, immediately after, with the tree quiet: `@semio-tech/draw-plugin:component-dev`,
  `:materialize-dev`, and every other dependency show `[local cache]`, and
  `prepare-draw-react-dev` itself shows `[existing outputs match the cache, left as is]` — "Nx read the output
  from the cache instead of running the command for 16 out of 18 tasks", `Cache: 16/18 hit (89%)`, 16.0s
  instead of 11m39s. The 2 non-hits are `repo:generator-inputs` (intentionally uncached) and one other
  always-live discovery step, not `prepare` itself.

### Language-agnostic fixtures/tests (C)
- `⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`: flipped the checkTarget-guard assertion (was
  `assert.equal(...cache, false, "... must inspect current bytes")`) to assert `cache === true` plus the new
  soundness predicates (`inputPatterns` present, each `outputRoots` entry present as a `runtime` digest naming
  its resolved path, and — when `inputDiscovery` is set — the fingerprint `dependentTasksOutputFiles` input
  and `dependsOn`). Flipped the `prepare-<variant>-react-<profile>` assertion from `cache === false` to
  `cache === true` plus the `{ dependentTasksOutputFiles: "**/*", transitive: true }` input; added a new
  assertion that the matching `prepare-<variant>-wgpu-<profile>` sibling stays `cache === false`.
- `🔌️nx-plugin/🧪️tests/🎯️precise-command-inputs/🟦️.ts` (S8's fixture, generic mechanism, unaffected by A/B):
  ran it — `[DEBUG] Precise command inputs: closure excludes unrelated files, matches esbuild oracle, changes
  hash only for closure edits, and falls back safely when unparseable PASS`.
- `bun ⚡️caching/📜️script.ts test` (the full `testCacheContracts()` suite, S7's ~1300-line file): **ran it —
  aborts before reaching either A's or B's new assertions**, on a pre-existing, unrelated failure:
  `AssertionError: catalog-smoke must not keep authored cache when policy is authoritative (false !== true)`
  at line ~649, in a `vectors.policies` loop that predates this session (`git diff --stat` on the fixture file
  is empty; `targetPolicy`/`cacheableFamily`/`matchesUncached`/`verifyCommand`/the `catalog-smoke` fixture row
  are all untouched by my diff — the only `cacheableFamily` line I touched is the `/generator/` anchoring
  above, unrelated to `catalog-smoke`). Root cause as far as I can tell without digging further: the test
  harness builds `options.command` as `` `bun ./📜️script.ts ${row.target}` `` rather than using the fixture
  row's own `command` field ("bun ./📜️script.ts verify catalog"), so `verifyCommand()` never sees the word
  "verify" and `catalog-smoke` falls through to the "respect authored cache" branch instead of being forced
  `true`. Flagged as a spawned background task (`task_d40aef56`, title "Fix catalog-smoke policy assertion
  mismatch in cache-contracts test") for whoever owns `targetPolicy`/this fixture, rather than fixed inline —
  not mine to unilaterally decide which of "fix the harness" or "fix the fixture/policy" is correct without
  more context, and fixing it isn't this lane's assigned scope. Compensated with the standalone
  `inventory(root)`-based replay above, which exercises the *exact* predicates I added to this same file
  against the real, live 700-project graph.

## Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` — `generatorContractInputs`,
  `outputRootInputs`, `OUTPUT_ROOT_DIGEST_SCRIPT` (new); the two `projectWithDefaults` contract loops (A);
  `cacheableFamily`'s `/generator/` → `/^generator(?:-|$)/` anchoring fix; `playgroundPreparationTargets`'s
  react `prepare-*` target + docstring (B); `cacheInternals` export list gained `generatorContractInputs`,
  `outputRootInputs`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json` — removed `"prepare"` from
  `uncached` (kept `"activate"`).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts` — flipped
  and extended the checkTarget-guard and `prepare-*-react-*`/`prepare-*-wgpu-*` assertions (C).

## Left undone / flagged

- `bun ⚡️caching/📜️script.ts test` (the full `testCacheContracts()` suite) cannot currently run to completion
  for anyone, on a pre-existing `catalog-smoke` policy mismatch unrelated to this lane — spawned as a
  background task (`task_d40aef56`) rather than fixed here.
- The `/(?:^|-)contract(?:-|$)/` clause in `cacheableFamily` has the *same* structural mid-string-match shape
  as the `/generator/` bug I fixed, but I found no evidence it's currently wrong (no target or playground
  variant literally named `contract`) — left untouched rather than changed speculatively.
- Two concurrent, unrelated edits landed in `⚡️caching/🔣️policy.json` while I worked (a new `"uncachedExact"`
  bucket, `"format"` moved into it) — not mine, left untouched, confirmed via re-reading the file before every
  edit per the multi-agent rules.
- `prepare-draw-react-dev`'s cache-hit proof (run 3) needed a third run because run 2 landed during a burst of
  unrelated concurrent lane activity that genuinely changed `draw-plugin`/`puzzle-plugin`'s own upstream
  sources — documented above as expected behavior, not a defect, but flagging it since a reader skimming only
  run 2's log would see a miss where run 3 shows a clean hit.

## Scratch cleanup

`🗑️generated/s11/hash-fixture*` (three throwaway git-initialized Nx fixture workspaces used for the hash
experiment) deleted after their evidence was captured in this report. `🗑️generated/s11/*.json`/`*.log` (the
`nx show project` JSON dumps and the five `bunx nx run` logs referenced above) kept as raw evidence backing
this report's Verification section.
