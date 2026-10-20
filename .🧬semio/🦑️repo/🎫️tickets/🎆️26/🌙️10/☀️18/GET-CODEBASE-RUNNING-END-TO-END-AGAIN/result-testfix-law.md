# Result testfix-law (wave 1020d, resumed 2026-10-20 ~18:45) - STAGED, waiting for GO

READY: `python3 C/testfix-law-tools/apply.py` (dry run: 43 operations, 100 anchored hunks, 0 drift at 18:48), then `--write`, then `C/testfix-law-tools/live.sh` (34 edited suites, 2 jobs, live). Nothing was written outside C/ and `/private/tmp/testfix-law/` (overlay; one test process at a time).

## Method
Overlay `/private/tmp/testfix-law/` (copy of the 238 law-drift suites' trees; this wave added `🌎️hub`, `🎓️teaching`, `🏢️semio-tech` from the live tree and found a git index copy; node_modules relinked). Baseline `testfix-law-tools/base-overlay.jsonl`: 238 suites, 166 green / 72 red. After overlay completion + staged edits: 29 of the 72 reds are green (some only because the overlay was incomplete, e.g. hub path, git index; not all 29 are attributable to edits). 43 still red, classified below.

## Staged edits (43 ops; every edit is law/fixture/doc truth, no weakened assertion)
New in this wave (verified green in overlay):
- `🧪️tests/🧱️checked-in-asset-contracts`: MCP configs/bootstraps now start the repo MCP server as `repo mcp` (env `SEMIO_REPO_MCP_CLIENT`), bootstraps run `bun repo run repo-mcp:install`, `repo run workspace:setup`, `repo run @semio-tech/repo-developer-tools:setup-debugger`; handshake test spawns the command declared in `.mcp.json`.
- `🖱️ui/📦️packages/🦀️rust/📋️project.json`: `test-text-editor-source` gets the explicit `cwd` its siblings have (General Nx entry law). Real defect fix, not a law edit.
- process `📊️profile` corpus: consumer moved to `📦️artifacts/🟦️typescript/🟦️.ts` (also fixes `routing/🧪️tests`).
- store `📄️catalog-retirement`: slice end marker no longer depends on a docstring without emoji.
- `local-hub/🎫️session` test reads the broker at its new owner `../🗄️broker/🟦️.ts`.
- vscode `🏗️custody`: reads router + `🔬️typescript-package-commands` (commands moved) and worker `🟦️.ts` (script is a router now).
- `⚡️caching/🕸️graph/✅️verification` roster: `projectInputs` has 9 parameters (declaration + runtime).
- cargo `🛠️preparation` law: profile existence read from `renderCargoProfileConfigurationV1` (profiles are no longer authored in `.cargo/config.toml`).
- `🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`: `runFreshComponentCheckScript` imported dynamically (runtime-boundary law: 0 static test imports). Real code fix.
- `🕹️selection-set.json`: call site `leftover_selected_ids_of(self.state)` lives in `👁️view/🔍️lane-views/🦀️.rs`.
- `🦀️rust/📥️intent` fixture+test: binary `repo` (was `semio-repo`); dashboard `compile` is `driver-owned` uncached (`cacheClass`/`cacheReason` rows).
- build graph facts roster (`storagePorts`) gains `requireCurrentRepositoryBuildGraphConsumerV1`, `buildGraphWriterOptionalQuotaFactsV1`, `withRepositoryBuildGraphConsumerV1`.
- `🖱️ui/🧪️tests/🧭️router-ownership` (+fixture): the 23 census declarations moved verbatim to `🖱️ui/🟦️.ts` (sha unchanged); the two classes `CheckUiPrimitivesScript`/`CheckChromeI18nScript` are now thin routers: new row sha for the router class plus `runnerBodySha256` proving the original `run` body (verified byte-equal, `context.`->`this.`, from commit 4dab415).
- `🧪️tests/🔧️build-surfaces`: MCP native staging law expects `spawnSync("repo", ["run", target, "--output=stream"])`.
- `🥒️gherkin-description-inline-code`: router text via `readRegisteredTestRoute`.
- `📥️dependencies/🧪️tests/📥️consumption`: harness extracts `repositoryCargoCompilerInputsV1`, binds `wasmPackageCargoConsumptionV1`, supplies `pkg.files` and `CARGO_BUILD_BUILD_DIR`.
- `🧪️tests/🧱️rust-source-direction/🧫️fixture-ownership`: pinned sha of paint `✏️editing/🔣️.json` re-recorded (file unchanged since its first commit; the old sha never matched) and the paint editing schema `spans` items are objects `{start,length,coverage}` (the consumer `Paint2dHost` and fixture use objects; the schema wrongly said triples).
- `.gitignore`: `**/🌐️browser-bundles/**` (was only under `🧫️fixtures`), so the generated jco bundles are ignored.
Inherited 1020c ops (21): dag/trinity sqlite snapshot law+fixtures, process timers/budget, plugin units fixtures, db crate-boundaries, caching script/owner-cmd-policy/bootstrap invocation, build inputs, reference-coverage/coordinate/json-owner/leading-grapheme/path-collection/route-context, energy/architect fem editors. Re-verified: all green except crate-partition, plugin units, plugins fixture-placement (see below).
Dropped: the inherited `🎯️source-demand/🟦️.ts` hunks (peer edited that suite 17:33; against the live fixture the suite has 9 other reds: peer-active).

## Verification (overlay)
Edited-suite run `.v1`: 26 of 34 green. Red among edited: `routing/🧪️tests` (1 timing flake, 55/56 green; passed in the previous run), `plugin units` (7), `🦀️rust/📥️intent` (1: stale-import law, see Real defects), `🔧️build-surfaces` (1: native host artifact path), `🚧️boundary` (2), `crate-partition` (3), `plugins fixture-placement` (1). NOT run: anything live, cargo, wasm, Go, vitest.

## Still red (43 of 72), by owner/class (exact causes)
Install-gated (needs `lock --all` + `sync --all`): wfc bitmap snapshot (`@semio-tech/wfc-bitmap` unlinked), forms consumers (`forms-js`), `🧱️dependency-direction` (`framework-filesystem` export `ERR_PACKAGE_PATH_NOT_EXPORTED`).
Envelope / genuine-birth (testfix-defect harness `runGenuineBunTestV1`; re-run after its apply): custodian (30 s timeout under load), publication witness (26), `🏎️overlap`, `component-pass/📥️inputs`, `nx/📦️graph` + `nx/📦️outputs` purpose (2), `🛠️tools/📦️sdk/📥️snapshot` (23), `🎯️selected/📦️snapshot`, `🚪️admission`, `📊️reporting`, `🧵️retained-command`, `📐️profile/♻️preparation` (cargo unit), `🚪️command` (mutation list command), `⏱️process-budgets` (nested owners in overlay).
Cargo / Go / native gated: fem `🔬️neutral` + `📐️surface` (need `SEMIO_FEM_NEUTRAL_RESULTS` from a cargo run), sqlite-snapshot `🤝️interoperability` (oracle default path `.🧬semio/🦑️repo/⚡️cache/cargo/target` is the retired layout; target dir is now `rust/fixed/m/cargo/shared/target`, binary needs a cargo build), bim gltf `🪑️components` fixtures never committed (`BIM_BLESS=1` cargo test + `bun 🟦️.ts write`; fails plugins fixture-placement), mesh `📦️targets/🧪️packaging` + `✂️clipping` (28 `Cargo.lock` files stale: `semio-framework-3d` lists journal/quality/surface, `mesh-document` misses `value-native-owned`; manifests == corpus, so regenerate locks with cargo offline), `🛡️durability` (Go test builds a plain module; member dirs `🐚️unix-durability`/`🪟️windows-durability` are only overlay-mapped by the canonical Go plan: test must use the canonical overlay), `📚️library-search-path` (command-line length oracle depends on repo path length; overlay path shorter), cargo `🪆️ownership` (hangs 250 s, cargo metadata).
Real defects (code/layout, owner needed; none weakened):
- `✏️s/🧪️tests/🗂️native-ownership`: `semio-s-fem-analyses` Cargo.toml depends on `semio-s-plugin-stdio`, `semio-s-artifact-stdio-semio-base` (and teaching-proctor-base); its test imports `semio_s_plugin_stdio::AssemblyPlan` (name-collision from a codemod; the real `AssemblyPlan` is private to `semio_s_fem_analyses_assembly`). Needs cargo check.
- `🚧️boundary` (General): `replication/🎮️mutation/🧳️retained/🧪️tests` reads 3 fixtures from `🛍️products/💻️os/…/🗣️dsl/✨️derive` (General -> Specific); `🧰️framework/📦️packages/🟦️typescript/🛍️products/…` facades import product modules (8 refusals). Needs fixture move + exports-stage.
- `🛍️products/🧪️tests/🧭️fixture-placement`: product `🧫️fixtures/🔣️.json` beside sources outside `🧪️tests` (tool-run, presence, window config, declaration view...).
- `⚖️laws/📇️indexed-generated-output`: 36 jco bundle files under `🧪️testing/🧩️jcoprobe/🌐️browser-bundles/` are tracked in git. `.gitignore` is staged; the user must `git rm --cached` them (I run no git modifying command).
- `🥾️cross-platform-bootstrap`: tracked ticket paths > 213 units: `staged/🧰️framework/…` trees of bzl-rust-tools, cache-fix-tools, ... (delete those staged copies after apply) plus `🖥️host/…/pkg/package.json` `cloneResolvable`.
- plugin `📦️units`: Rust test-body sha256 preservation hashes stale (restructure); re-baselining would weaken the preservation law, owner must regenerate from the original bodies.
- `🧬️semantic-source-pairs`: emitted descriptor JSON vs pack forms disagree (descriptor emitter).
- `🖥️server/🎛️coordinator/🏗️builder/🏃️execution/🧪️tests`: next compiler script input closure is 1230 files, limit 192 (input-precision).
- `🔧️build-surfaces` last red: `nativeHostArtifactPathV1` joins the declared output `.🧾️store/native-build.json` as a directory (declared output became a receipt file; own-output/demo-bzl peers).
- `🦀️rust/📥️intent` last red: `refuses stale imports` law mocks `node:fs.readFileSync` but the orchestration helper reads through `captureBoundedSourceRevisionV1` (`openSync/readSync`); the law needs a temp-copy tree oracle.
- `📡️replication/🧪️tests/🏛️crate-partition`: `🎮️mutation/🧳️retained/🧱️row/**` and `📏️body` are not mounted by `🧳️retained/🦀️.rs` (half-done split); mutation-messages tests 232899 > 230000 bytes; `HistoryFoldSet` impls owned by another crate.
Peer-active / owner-blocked: `🎯️source-demand` (peer 17:33), `📥️inputs/📋️data` (inference refactor: producers delegate, `withManagedGraphSourceReadBatchV1` moved), `🤖️agent-client-configuration` (root dashboard declaration still `bun ./📜️script.ts dev mcp stdio`; checked-in configs already `repo mcp` with env; generator needs `env` support + declaration/schema/fixture change, collides with cli-zt and the peer-modified root `📋️project.json`), `🚚️readme-move-source-authority` (fixture base plan authority kind vs `exact-owner-current-source-approval-v1`, executor `repo:script`).

## After GO
```
python3 C/testfix-law-tools/apply.py && python3 C/testfix-law-tools/apply.py --write
C/testfix-law-tools/live.sh          # live results in C/🗑️generated/testfix-law/live.jsonl
```
Then the user runs `git rm --cached` for the 36 jcoprobe `🌐️browser-bundles` files. Re-run the red list after install sync, testfix-defect harness, mesh lock regeneration.

Files: tools `C/testfix-law-tools/{apply.py,stage.py,track.py,run.py,live.sh,suites-edited.txt,base-overlay.jsonl,staged/}`; overlay `/private/tmp/testfix-law/` (delete when done). No `[DEBUG]` lines left.
