# result-gofix (lane gofix, wave 1027b, 10-27 16:00-17:00): every Go module vets and tests green in a mirror except 2 envelope-only cases; 3 reds fixed in a staged batch, the 10-26 guard fix is confirmed live

`C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`, `M` = `🧰️framework/🛍️products/🦑️repo/🔨️modules`. Graph-free, no official run, no live edit, nothing landed. The earlier lane of the same name (10-18: tree/cli/graphql test fixes, `applyAutofixes` split) is kept at `C/gofix-tools/result-gofix-1018-archive.md`.

## FOR coordinator

1. Apply batch `gofix1` (10 files, compile-atomic, laws red on live, green in the mirror; no graph owner touched, no `📚️library/**`/`🏃️process/**` TS):
   - check/apply/revert: `python3 -I "$C/gofix-tools/staged/gofix1/stage.py" check|apply|revert` (keeps modes; `python3 -I C/xplat5-tools/tests/test_stage_mode.py <stage.py>` OK).
   - laws (mirror only): `GOFIX_GOROOT="$PWD/.cache/b/t/m/native/go/1.26.9/darwin-arm64-f9bb7c0a02506c5d" VERIFY_ROOT=<mirror root with the batch> sh "$C/gofix-tools/staged/gofix1/verify.sh"` (plans the 33 modules with the library planner, then `go vet` + `go test` of statutes, launcher, graphql, mcp, ⌨️cli with the scratch inside AND outside the checkout, then the client module with a 60 min timeout and the two envelope cases skipped).
   - product runs that prove it (one at a time, after a gate verdict): `G gf-go1 '@semio-tech/repo-launcher-go,@semio-tech/repo-graphql-go,repo-mcp,@semio-tech/repo-statutes-go,@semio-tech/repo-cli-go' 'test'` (expected green; before: launcher 2 red, graphql 2 red, mcp 21 red) and `G gf-go2 '@semio-tech/repo-client' 'test'` (the 565-case client module; only `TestCanonicalGoTestDispatcher*` can be red, see section 3).
   - side effect to expect: `git status` shows the fixture `…/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-fixable/🟦️.tsx` back at its HEAD bytes (the index copy `M` is the polluted one; the batch restores HEAD).
2. Optional batch `goyaml` (3 files: project + script for the only Go module without a test route, plus its row in the hot root file `📋️build-owners.json`): `python3 -I "$C/gofix-tools/staged/goyaml/stage.py" check|apply|revert`; the registry file is hot, so rebase before applying with `sh "$C/gofix-tools/rebuild-goyaml.sh"` (copies live registry into the mirror, runs the library's `reconcileBuildOwnerRegistry` scoped to the yaml directory, rebuilds the batch). Proof: `G gf-yaml '@semio-tech/repo-yaml-go' 'test'` (9 cases, a few seconds). Do NOT land it in the same group as other graph changes: a new project is a membership change that dirties its graph owner.
3. The Go gate list is in section 4 (official commands per project, in groups that mirror `prodmap` C2).

## 1. What was run, where, how (reproducible)

- Toolchain: the repo pins Go through `🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/🔣️.json` (`go 1.26.9`, `GOTOOLCHAIN=local`, work file `go.work`); `pinnedGoEnvironmentV1` provisions it under `.cache/b/t/m/native/go/1.26.9/darwin-arm64-f9bb7c0a02506c5d`, which is GOROOT of every run here (`go version go1.26.9 darwin/arm64`). Environment of each run: `GOROOT` pinned, `GOTOOLCHAIN=local`, `GOFLAGS=-mod=readonly`, `GOPROXY=off` (no module has an external requirement, so nothing is fetched), `GOWORK=<mirror>/go.work` (`off` for the owner-level server module outside `go.work`), private `GOCACHE`/`GOMODCACHE` under `C/🗑️generated/gofix/`, `nice -n 10`.
- Mirror `gofix` (`$TMPDIR/sm/gofix`, 16,412 files: `repo` product, `process`, `📁️filesystem`, `.devcontainer`, `.codex`, `.github`), created once, plus `git init && git add -A` inside the mirror (the guard and the git-backed cases need a repository around the checkout). Every run is `mirror.py run gofix --seal-allow <C/🗑️generated/gofix>`: macOS `sandbox-exec` denies every write below the live repo, and the guard confirmed "live tree unchanged over 16412 files / 13129 dirs" after each run. Mirror limitation found: the mirror skips gitignored files, but `M/💻️client/⌨️cli/🏷️entity-kinds/🐹️.go` is a gitignored GENERATED Go source (target `@semio-tech/repo-client:generate-entity-kinds`, a `dependsOn` of `repo-client:test`); without it `vet`/`test` of the client module fail with `undefined: EntityKindCatalog`. I copied it in. Anything outside the official route that runs `go test` there must generate it first.
- Plan: `go` cannot see the canonical layout (one `🐹️.go` per member directory), so every module is compiled through `-overlay` from the library planner `canonicalGoPlan` (the same function `runCanonicalGoTests` uses): `C/gofix-tools/plan.ts <root> <out>` writes 33 overlays + `modules.json`. Runner: `C/gofix-tools/run.py` (stdlib, streams `go test -json`, keeps only failing output, aborts above 500 MB RSS) via `C/gofix-tools/run.sh <label> inside|outside <steps> <only> <jobs> <ids>`. `inside` = `TMPDIR`/`SEMIO_TEST_ARTIFACT_DIR` under the checkout like the official route; `outside` = a realpath'd system temp dir.
- Modules: `git ls-files '*go.mod'` = 43; 33 are product modules (31 in `go.work` + the owner-level `M/🖥️server/🎛️coordinator` + `M/📚️library` already in the work file), the other 10 are ticket scratch tools (section 5).

## 2. Verdict per product module (33; vet = exit 0 for all 33 before and after)

Cases are top-level pass/fail/skip, base = live tree as of 16:00 (scratch inside the checkout), after = with batch `gofix1`.

| id | module (`M/…`) | owning project / official command | base | after |
|---|---|---|---|---|
| 0 | `/framework/🔨️modules/📁️filesystem/🛡️durability/…/🐹️go` | `@semio-tech/framework-filesystem-durability-go` (no targets: nothing runs it) | 0 tests | 0 tests |
| 1 | `⌨️cli/📦️packages/🐹️go` | `@semio-tech/repo-cli-go:test` (+`:build`, `:test-entrypoint-source`) | 115/0/1 | 115/0/1 |
| 2 | `🌳️tree/…` | `repo-tree-go:test` | 34/0/0 | same |
| 3 | `🎫️tickets/…` | `repo-tickets-go:test` | 34/0/1 | same |
| 4 | `🎯️goals/…` | `repo-goals-go:test` | 3/0/0 | same |
| 5 | `🏃️test-runner/…` | `repo-test-runner-go:test` (+`:build`) | 20/0/0 | same |
| 6 | `🏠️workspace/…` | `repo-workspace-go:test` | 46/0/1 | same |
| 7 | `🏠️workspace/…/🏖️sandbox` | covered by module 6 (project root one level up) | 0 tests | 0 tests |
| 8 | `💻️client/⌨️cli` (`repo/client`, 565 cases) | `@semio-tech/repo-client:test` (dependsOn `generate-entity-kinds`; NOT named `-go`) | 562/2/4 (2 envelope-only) | unchanged files; 460 passed before a 15 min timeout at load 66, no new red |
| 9 | `💻️client/⌨️cli/…/📦️main` | `repo-client:build` | 0 tests | 0 tests |
| 10 | `💻️client/🔌️mcp` | `repo-mcp:test` (+`build`, `install`, `test-install`) | 47/21/0 | 68/0/0 |
| 11 | `📊️metrics/…` | `repo-metrics-go:test` (+`:build`) | 0 tests | 0 tests |
| 12 | `📐️model/…` | `repo-model-go:test` | 97/0/0 | same |
| 13 | `📚️library` (`repo/go`) | no project | 0 tests | 0 tests |
| 14 | `📜️statutes/…` | `repo-statutes-go:test` | 34/0/0 | 34/0/0 (with the restored fixture) |
| 15 | `📝️todos/…` | `repo-todos-go:test` | 14/0/0 | same |
| 16 | `📡️events/…` | `repo-events-go:test` | 12/0/0 | same |
| 17 | `🔎️search/…` | `repo-search-go:test` | 12/0/0 | same |
| 18 | `🔗️graphql/…` | `repo-graphql-go:test` | 39/2/0 | 41/0/0 (new law included) |
| 19 | `🖥️server/🎛️coordinator` (`repo/server`, owner level, `GOWORK=off`) | `@semio-tech/repo-coordinator:test` (the TS project; `task-routes` runs this module, then vitest) | 27/0/0 | same |
| 20, 21 | `🖥️server/🎛️coordinator/…/🐹️go`, `…/📦️main` | `repo-coordinator-go:test|build|dev` | 0 tests | 0 tests |
| 22 | `🗂️codebase/…` | `repo-codebase-go:test` | 6/0/0 | same |
| 23 | `🗣️languages/…` | `repo-languages-go:test` | 8/0/0 | same |
| 24, 25 | `🚀️launcher/…/🐹️go`, `…/📦️main` | `repo-launcher-go:test` | 42/2/1 | 45/0/1 (new law included) |
| 26 | `🚚️move/…` | `repo-move-go:test` | 17/0/0 | same |
| 27 | `🧑️contributors/…` | `repo-contributors-go:test` | 6/0/0 | same |
| 28 | `🧩️providers/…` | `repo-providers-go:test` | 28/0/0 | same |
| 29 | `🧪️test/…/🐹️go` | none (project `repo-test-domain` has only TS routes) | 0 tests | 0 tests |
| 30 | `🧾️yaml/…/🐹️go` | NONE: the only Go module with 9 cases and no route | 9/0/0 | 9/0/0 (batch `goyaml` adds `@semio-tech/repo-yaml-go:test`) |
| 31 | `🪝️hooks/…` | `repo-hooks-go:test` | 80/0/0 | same |
| 32 | `🪪️identity/…` | `repo-identity-go:test` | 14/0/0 | same |

Scratch outside the checkout changes only the launcher case (it passes on base too, 44/0/1); graphql and mcp need their fixes in both modes. With `outside` after the batch: cli 115/0/1, mcp 68/0/0, graphql 41/0/0, launcher 45/0/1 after the batch.

## 3. Failure classification (every red of the sweep, with root cause and evidence)

Base reds: client 2 (envelope), mcp 21, graphql 2, launcher 2. Four independent causes besides the envelope pair.

1. **mcp, 21 cases (`TestBuildToolsAreDerivedFromTheCatalogue`, `TestServerListsDerivedBuildTools…`, …) = REAL BUG in the MCP policy.** `M/🔌️mcp/🖼️assets/🏗️build-surface.json` gives the new `fleet status` tool `"flags": []` and `"bind": {"--json": true}`; `Derive` requires a bound flag to be in the tool's accepted flags: `mcp: tool "fleet_status" binds --json, which it does not accept`. That error aborts the derivation, so on live the MCP server derives NO build tools (every consumer of `Derive` fails, not only the tests). Fix: drop `"flags": []` (the tool then accepts the catalogue flags `--json` and `--limit`, like `leases status` and `capacity status` which omit `flags` too). Also `TestNativeEntrypointVectorsCoverEveryCatalogueOperation`: see 2.
2. **launcher `TestRouteVectors` + mcp `TestNativeEntrypointVectorsCoverEveryCatalogueOperation` = STALE shared fixture/schema.** The operations catalogue (`📚️library/🏗️build/⚙️operations/🔣️catalog.json`, row 22) gained `fleet`; the language-agnostic fixture `M/⌨️cli/🧫️fixtures/🚪️entrypoint/🔣️.json` (`buildVerbs` + vectors) and its schema const `M/⌨️cli/🧬️schema/🚪️entrypoint/🔣️.json` were not extended: "the shared fixture and the operations catalogue declare different verbs". Fix: `fleet` after `leases` in both lists (schema const, fixture `buildVerbs`) plus the vector `{id: fleet, argv: [fleet, --opaque=value], kind: build}`. The TS law `M/⌨️cli/🧪️tests/🚪️entrypoint/🟦️.ts` (bun, in the mirror) stays 1 pass / 125 expects; module 1's fixture reader stays 115/0/1. NOTE for the Rust CLI/dispatcher implementations: they read the same fixture; whoever owns the Rust route table should confirm `fleet` routes to the build owner there too (not checked, no cargo).
3. **graphql `TestFixApplyAutofixes` + `TestFixMutationAppliesAutofixesOnDisk` = TWO stacked causes.**
   - (a) FIXTURE POLLUTION, tracked file repaired in place. `M/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder/🧪️file-fixable/🟦️.tsx` is modified in the index: the blank line after `#region 🧲️Header` is gone, so it equals `🧪️file-fixable-expected` and has nothing left to fix. The shared contract says otherwise: `🧪️tests/🧫️fixtures/🔍️analyze-breaches/🔣️breaches.json` line 10 expects the breach `code/section/wrong-format/newline-after-region` at line 2 of exactly this file, and `🩹️autofix-roundtrip/🥒️.feature` lines 5-7 say so in words (the Rust cases `🩹️autofix-roundtrip/🦀️.rs` and `🔬️unit/🦀️.rs` read it too). The author of the damage is the pre-10-26 test fence: with an empty `.git` directory the Fix cases ran against the REAL checkout and repaired its fixture. This is the "Go test writes into the real tree" class, on a tracked file. Fix: restore the HEAD bytes (batch file `07-…tsx`, base = polluted index copy) and add a law `TestFixableStatuteFixtureStaysUnfixed` (fixture != expected, and the first lines carry/lack the blank line); red on live, green after.
   - (b) NO REPOSITORY of its own. Since guard-g's `BindRoot` runs a real `git init` the cases see a truly empty repository: `codebase.LoadBundles()` returns 0 bundles and the `code` policy finds nothing (debug: `bundles=0 breachs=0`). Before, `git`/discovery leaked through to the checkout. Fix: `seedFixableTechnology` (in `🔬️unit/🐹️.go`, next to `withMonorepoFixture`, which does the same for the other cases) writes a one-technology repository (`compose/README.md`, `compose/js/AGENTS.md`, the fixture copy) and binds it; both cases use it.
   In `outside` mode two more graphql cases fail (`ticket namespace must be a physical directory`) only because my first runner issued `/var/folders/...` (macOS symlink); the runner now issues realpaths. Not a product defect, but a portability note: a native `go test` with an unresolved macOS `TMPDIR` fails the ticket-scope cases; `sandbox.Run` could `EvalSymlinks` the scratch variables (not done, no red on the official route).
4. **launcher `TestVersionWithoutLanguageOrRepository` = environment (scratch below the checkout) in a module without `sandbox.Run`.** The case needs a directory no repository resolves from, takes `t.TempDir()`, and the official route issues `TMPDIR` below the checkout, so `ResolveRoot` walks up into the checkout and finds the workspace (`code=2`, empty stdout). With the scratch outside it passes (44/0/1, measured). The launcher module has no external requirement on purpose (a standalone binary), so importing `sandbox` just for this would change its `go.mod`; instead the harness gets `emptyWorkspace(t)` (tries the issued temp dir, the user cache, `/tmp`, `/var/tmp`; accepts the first where `ResolveRoot` says not found) and the case uses it. New law `TestEmptyWorkspaceLiesOutsideACheckoutThatIssuesTheScratch` builds a fake checkout (the two root markers), points `TMPDIR/TMP/TEMP` into it, proves a directory made there resolves to the fake root (the premise) and that `emptyWorkspace` does not.
5. **client `TestCanonicalGoTestDispatcher` and `…Cancellation` = ENVELOPE-ONLY, not fixable from a bare `go test`.** They spawn `bun …/📜️script.ts go-test` which needs the official caller grants (`Original native caller capabilities required`, identical with scratch inside/outside). With the envelope injected (`SEMIO_SCRIPT_PROCESS_INVOCATION` from `C/chain-g-tools/wire.ts`, runner passthrough `GOFIX_ENV_<NAME>`) the child advances to the pinned-Go provisioning (it downloaded 64 MB into the mirror, deadline exceeded) and `Cancellation` then dies on `fork/exec /bin/ps: operation not permitted` (the mirror's sandbox seal). So they can only be judged on `repo-client:test` (official route, which also provisions Go). Both are excluded from `verify.sh` with `-skip`.

Not fixed, noted: nothing else red. `M/💻️client/🔌️mcp/.🧬semio/🦑️repo/🎫️tickets/…` (12 MB, Sep 12, gitignored) is an old nested ticket tree from an earlier mcp run; I did not touch it (not mine, not tracked) but it is the leftover of the same pollution class.

## 4. The sandbox guard (the task's path-prefix bug)

Already fixed LIVE by guard-g on 10-26 (`result-guard-g.md`; the prodmap/blocker text of 10-27 15:36 quotes the 18:41 red and is stale). Re-verified here on today's tree: `M/🏠️workspace/📦️packages/🐹️go/🏖️sandbox/🛡️root-guard/🐹️.go` flags a resolution only when `SameDirectory(real, ResolveRepoRoot(path))` (deepest existing ancestors compared with `os.SameFile`, so symlinks, dot segments, case and Unicode folding, drive letters agree); `PathWithin` and `RelocateScratch` use it; nested marked repositories pass, unmarked ones below the checkout are flagged, the guard aborts at the first touch.
- workspace module in the mirror, scratch inside: 46 pass / 0 fail / 1 skip (incl. the `SameDirectory` laws and the Python-oracle vector test); `python3 -I C/guard-g-tools/oracle.py M/🏠️workspace/🧪️tests/🧫️fixtures/📡️same-directory-vectors.json`: 17/17 agree (`folds-case=True folds-normalization=True mismatches=0`).
- Audit for other prefix-based containment: every `strings.HasPrefix(path, root…)` in product Go (tree nodes, query cache, ticket store, test-runner snapshot, ticket finish/reopen) compares virtual slash paths with a `root+"/"` segment boundary or an explicit `path == root`; none is a security boundary like the guard, none needs the identity rewrite. No new guard change is needed.
- No Go test writes into the real ticket tree, evidence: (1) all 33 modules ran with the scratch INSIDE the checkout under the macOS seal (EPERM on any write below the live repo except the declared generated dir): no EPERM failure, no `tests touched the real repository root` / `bound to the real repository root` in any log; (2) mirror delta after the sweep (`git status` against the post-copy index) shows only my intended edits; no `.🧬semio` directory appeared at the mirror root; (3) the mirror guard reported the live tree unchanged after every run (1,084 s run included). The only polluted tracked file found is the autofix fixture of 3(a), already damaged before.
- Still open (guard-g's list, unchanged): about 40 direct `workspace.SetRootDir(t.TempDir())` sites rely on the scratch relocation instead of `sandbox.BindRoot`; they abort safely if the relocation cannot apply. The launcher tests read the real checkout by design (read-only).

## 5. Official product commands per Go project (for the gate)

Wrapper `G()` = `python3 -I "$C/prodmap-tools/group.py" <label> <projects> <targets>` (see `result-prodmap.md`; one run-many group per call). All 24 projects that run Go cases plus the four owners found here:

```
G gf-go-a '@semio-tech/repo-workspace-go,@semio-tech/repo-tickets-go,@semio-tech/repo-graphql-go,@semio-tech/repo-tree-go,@semio-tech/repo-hooks-go,@semio-tech/repo-cli-go' 'test'
G gf-go-b '@semio-tech/repo-goals-go,@semio-tech/repo-test-runner-go,@semio-tech/repo-metrics-go,@semio-tech/repo-model-go,@semio-tech/repo-statutes-go,@semio-tech/repo-todos-go,@semio-tech/repo-events-go,@semio-tech/repo-search-go,@semio-tech/repo-codebase-go,@semio-tech/repo-languages-go,@semio-tech/repo-launcher-go,@semio-tech/repo-move-go,@semio-tech/repo-contributors-go,@semio-tech/repo-providers-go,@semio-tech/repo-identity-go,@semio-tech/repo-coordinator-go' 'test'
G gf-go-c 'repo-mcp' 'test'
G gf-go-d '@semio-tech/repo-client' 'test'            # module 8, 565 cases, 4-15 min depending on load; includes generate-entity-kinds
G gf-go-e '@semio-tech/repo-coordinator' 'test'        # module 19 (repo/server) + vitest
G gf-go-f '@semio-tech/repo-cli-go,@semio-tech/repo-test-runner-go,@semio-tech/repo-metrics-go,@semio-tech/repo-coordinator-go' 'build'
G gf-go-g '@semio-tech/repo-cli-go' 'test-entrypoint-source'
```

`prodmap` C2 listed 5 projects; this list is the 22 `repo-*-go` + `repo-mcp` + `repo-client` + `repo-coordinator`. Not run by any official command today: module 0 (durability, no target), 13 (`repo/go`, 0 cases), 29 (`🧪️test` Go, 0 cases) and, until `goyaml` lands, 30 (yaml, 9 cases). Walls measured here (graph-free `go test` after warm compile, load 35-66, so an upper bound for the test part only): tree 50 s, cli 27-164 s, tickets 19 s, hooks 16 s, graphql 10-24 s, mcp 5-16 s, launcher 3-6 s, client 105-902 s.

## 6. The 10 ticket-scoped Go modules (`git ls-files '*go.mod'` under `.🧬semio/`)

Throw-away analysis tools of old tickets, not product, not on any gate. Copied to `C/🗑️generated/gofix/ticketgo/` and vetted/tested with the same environment: `go-split`, `go-symbol-graph`, `gocli-tools`, `gocomptest`, `gofix2-tools/reach`, `gofix2-tools/rmdecl` build and vet clean (no test files); `gitignore-check` needs `github.com/bmatcuk/doublestar/v4` (no network allowed); `go-overlay-proof` imports an emoji path (`malformed import path … invalid char`, by design a layout experiment); `events-parity` has a relative `replace` that only resolves in place; `wp-t4/sdl` needs `go.work`. Left as is (other tickets' files).

## 7. State, tools, cleanup

- Batches (all base-hashed, mode-keeping): `C/gofix-tools/staged/gofix1` (10 files), `C/gofix-tools/staged/goyaml` (3 files, rebuild `C/gofix-tools/rebuild-goyaml.sh`).
- Tools: `C/gofix-tools/{plan.ts,run.py,run.sh,summary.py,make-batch.py,reconcile.ts,rebuild-goyaml.sh}`; run output `C/🗑️generated/gofix/` (logs, `gocache`, `plan`, `ticketgo`); the mirror `gofix` is DROPPED (my own scratch under `$TMPDIR`, `gocache`/`gomod` removed). To re-verify: `python3 -I C/mirror-tools/mirror.py create gofix --only repo process "🧰️framework/🔨️modules/📁️filesystem" .devcontainer .codex .github`, then in the mirror `git init -q . && git add -A`, copy the ignored generated `M/💻️client/⌨️cli/🏷️entity-kinds/🐹️.go` from live, apply the batch with `stage.py apply --root <mirror>` and run `verify.sh`.
- Registry law for `goyaml`: `GOFIX_WRITE=0 bun C/gofix-tools/reconcile.ts <mirror> <yaml dir>` prints `"changed": false, "findings": []` after the batch (before: `changed: true`); the unscoped whole-tree walk in the partial mirror lists unrelated findings (e.g. `🌎️hub/🧩️compositions`), so `verify.sh` of the batch scopes it. The batch's registry row is the library's own output (6 lines inserted at its sorted position).
- Not verified: a full green run of module 8 with the batch (nothing in it touches the files that module reads; its post-batch full run hit the 15 min timeout at load 66 after 460 passes); the two envelope cases; Windows/Linux (macOS arm64 only).
