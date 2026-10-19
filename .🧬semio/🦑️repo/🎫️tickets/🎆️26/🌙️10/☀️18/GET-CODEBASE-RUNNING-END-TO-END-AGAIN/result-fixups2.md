# result-fixups2 (2026-10-19)

Executor `fixups2`. Inputs: `brief-1019.md`, `audit-verify-1019b.md`, `result-owners-gate.md`, `result-cache-s.md`.

## (0) Invalid `workspaceCommand` declarations

Cause: `ownedScriptRoutes` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🧭️routing/🧩️contributions/🟦️.ts`) requires every declaration to be an array of at least 2 words (`^[a-zA-Z0-9-]+$`) and globally unique. `git diff HEAD` shows the single-word fem declaration (`["verify-combination-stress"]`) was already in HEAD, not introduced by cache-s (cache-s only flipped `cache` and added `inputs`). The route validator is unchanged since HEAD. A repo-wide validator found 13 problems: 9 single-word declarations (fem and framework) and 4 duplicate declarations between sibling packages.

Fixed (all declaration arrays only; the `command` strings of the targets and the package scripts are unchanged):

| Manifest (target) | Old | New |
|---|---|---|
| fem `📦️packages/🟦️typescript` (`verify-combination-stress`) | `verify-combination-stress` | `verify fem-combination-stress` |
| fem `◻️2d` rust (`test-material-source|native`) | `test-material source|native` (duplicate with 3d) | `verify fem2d-material source|native` |
| fem `🧊️3d` rust (`test-material-source|native`) | `test-material source|native` | `verify fem3d-material source|native` |
| `🧰️framework/📦️packages/🟦️typescript` (`window-utility-check`) | single word | `verify window-utility-check` |
| `🧰️framework/📦️packages/🦀️rust` | `test-staged-payload-funding`, `test-retained-pack-encoding`, `test-window-utility-native` (single word) | `verify staged-payload-funding`, `verify retained-pack-encoding`, `verify window-utility-native` |
| `🔨️modules/🌱️value/✨️derive` rust | `value-admission-check`, `value-admission-check --native` | `verify value-admission-check`, `verify value-admission-check --native` |
| `🔨️modules/🌱️value` rust | `test-native-receiving-original-source`; `test-retained-equality source|native` (duplicate with artifact-reference) | `verify native-receiving-original-source`; `verify value-retained-equality source|native` |
| `🔨️modules/🗿️artifact-reference` rust | `test-retained-equality source|native` | `verify artifact-reference-retained-equality source|native` |
| os renderer react ts | `viewport-presence-check`, `operation-diagnostic-host-check` (single word) | `verify viewport-presence-check`, `verify operation-diagnostic-host-check` |

Nothing else referenced the old names (`git grep` outside the tickets: only the manifests and the package-internal script routers, which keep their own command words).

Validator: `C/fixups2-tools/validate-manifests.ts` (Ajv draft-07 and the repo `validateJsonSchemaSubset` against `🏗️build/🧬️schema/📋️project/🔣️.json`, plus the `workspaceCommand` rule and cross-manifest uniqueness). Usage: `bun C/fixups2-tools/validate-manifests.ts [root ...]`, default roots `✏️s` and `🧰️framework`. Result after the fix: `manifests=1456 workspaceCommands=171 problems=0` over `🌎️hub 🎓️teaching 🏢️semio-tech ♻️mit-bestand ✏️s 🧰️framework 👴️leutwiler`. Cross-check with the real route function: `C/fixups2-tools/probe-routes.ts` returns `routes=167` (admitted packages) with no throw.

`bun ./📜️script.ts verify build-owners` (repo root, `alarm 590`):
- first run (after the declaration fix): reached the check and failed with 1 finding `cargo-without-row` for a brand new package `✏️s/🔌️plugins/🔋️energy/…/🌈️window-layer-optics/📦️packages/🦀️rust/Cargo.toml` (created by a peer at 15:03, not by me, no owner row yet);
- second run, after the peer registered it: `[verify build-owners] passed.` exit 0.

## (1) Stale-cache holes (hub stdio and the same pattern elsewhere)

Reads, taken from the handlers (not from names):
- `artifact-directory-wiring check`: walks the whole `✏️s/🔌️plugins/🗄️stdio` tree (every file) and the text files again via `stdioWalkText`.
- `subset-directory-wiring check`: `stdioWalkText` over the same tree.
- `home-io-surface(-native)-check`: walks `🧰️framework`, `✏️s/🔨️modules`, `✏️s/🔌️plugins` (`.rs|.ts|.js|.json|Cargo.toml`) and every `Cargo.toml` in the repo; `prepareStdioComposition` reads the composition tree.
- snapshot-sqlite, `test`, `flow-retained-decode-check`, `composition-contribution-check`, `mutation-inventory`: bun tests, JSON fixtures and cargo tests over the stdio plugin tree (json/ts/schema), the hub stdio `🧪️tests` fixtures and the composition/bridge trees. The project `default` only held `✏️s/🔌️plugins/🗄️stdio/**/*.rs`, the crate and `📥️execution`.

Edits:
1. `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📋️project.json`: new named inputs `stdioTree` (project, whole hub stdio tree, whole stdio plugin tree minus `target`, `node_modules`, `🗑️generated`, `sharedGlobals`) and `stdioSurfaceReads` (framework/modules/plugins sources, all `Cargo.toml`, lock, toolchain, cargo config). `artifact-directory-wiring-check` and `subset-directory-wiring-check`: `inputs: ["stdioTree"]`, `outputs: []` kept. 31 other cached script targets: `["default","^default","stdioTree"]` (`home-io-surface*` also `stdioSurfaceReads`); the two former narrow explicit lists (composition, bridge) are subsumed by `stdioTree`.
2. Two latent handler bugs found while proving the checks (`🌎️hub/…/🦀️rust/📜️script.ts`):
   - `subset-directory-wiring` resolved its fixture from the hub tree (`../../🧪️tests/…/🧭️wiring`) where it does not exist (it lives under `✏️s/🔌️plugins/🗄️stdio/🧪️tests/…`), so the check threw ENOENT. Now `join(resolve(this.repoRoot, ARTIFACT_OWNER), "🧪️tests/🧫️fixtures/🧭️wiring/🗂️subset-directory-wiring")`.
   - `artifact-directory-wiring` visit skipped `node_modules` only when `isDirectory()`; bun's symlinked `node_modules` (41 of them in the stdio tree) then hit `EISDIR`. The exclusion now applies by name regardless of entry type.
3. Scan of the other `cache: true` `*check*|*wiring*|verify*|test-*-source|native` targets of `🌎️hub 🎓️teaching 🏢️semio-tech ♻️mit-bestand 👴️leutwiler` (tool `C/fixups2-tools/scan-wide-readers.ts`: import closure of the handler script, flags tree walks) and a manual read of every flagged handler. Fixed:
   - `🌎️hub/🧩️compositions/📕️norm/…/📋️project.json`: eight cached targets read `✏️s/🔌️plugins/📕️norm/**` (artifact leaves, descriptors, schemas, results config, tests) but only held `**/*.rs` of the hub composition; new named input `normTree`, `["default","^default","normTree"]` on all of them; `mutation-leaf-taxonomy-generate` got its exact read set (the `🗿️artifacts` tree) and its declared output now points to the real file `…/📕️norm/🧪️tests/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json` (it named a non-existent `…/🧫️fixtures/…`).
   - `🌎️hub/🧩️compositions/🌍️gis/…/📋️project.json`: checks read hub gis `🧪️tests` JSON/TS fixtures and schemas, but `default` only held `*.rs` there; new `gisTree` on all 12 cached check/test targets (not `native-codec-projection`: it reads Rust receipts and writes the checked-in projection, its declared output stays outside its inputs).
   - `🌎️hub/📦️packages/🟦️typescript/📋️project.json` `typecheck`: tsconfig includes `hub/**/*.ts` plus mapped framework sources; inputs now `default`, `^default`, hub/framework/`✏️s` `ts|tsx|json`, `tsconfig.json`, `externalDependencies: typescript` (same form as the `✏️s/🧑‍💻dev` typecheck).
   - Not changed (verified sound): hub `os-hub` source checks (readers only read `.rs` under hub, covered by `hub/**/*.rs`; others have explicit input lists), course `typecheck` (its `default` already enumerates the course tree and the mapped framework packages; an extra edit was reverted), root verify tasks (`inputs: ["default"]` = whole workspace), products in `🏢️semio-tech` and `♻️mit-bestand` (no tree-walking cached checks found).

## Verification

| What | Command | Result |
|---|---|---|
| Manifest validation (Ajv + repo subset validator + command rule), 1456 manifests | `bun C/fixups2-tools/validate-manifests.ts 🌎️hub 🎓️teaching 🏢️semio-tech ♻️mit-bestand ✏️s 🧰️framework 👴️leutwiler` | 0 problems |
| Real route loader | `bun C/fixups2-tools/probe-routes.ts` | `routes=167` |
| verify route | `bun ./📜️script.ts verify build-owners` | passed (first run: 1 peer finding, see above) |
| Wiring handlers, run for real with a script-process envelope | `bun C/fixups2-tools/run-script-with-envelope.ts <hub stdio rust dir> artifact-directory-wiring check jpg` and `subset-directory-wiring check semio mesh` | both pass, `0 files stale` (before the two script fixes: ENOENT and EISDIR) |
| Not run | `bun repo run …`, nx graph load, Bazel, cargo, `bun test` suites; the effective inputs after `project-assembly` are derived by reading, not observed |

## Open items

1. Run the nx graph once and check the new `stdioTree`, `stdioSurfaceReads`, `normTree`, `gisTree` named inputs resolve (brace globs and `!`-negations follow existing manifests; Cargo-owner targets may ignore explicit inputs per the cache-s note).
2. `home-io-surface-*check` calls `prepareStdioComposition`, which writes tracked hub files. The inputs now cover everything it reads, so a hit is sound, but a hit skips that regeneration; consider `cache: false` like `composition-prepare` if that is not wanted.
3. `norm` `component-budget-check` reads a built wasm component; its inputs approximate the component by sources (no `dependentTasksOutputFiles` link).
4. The scan only sees directory walks; plain `readFileSync` of paths outside the declared inputs in other handlers is not detected.
5. Tools: `C/fixups2-tools/` (`validate-manifests.ts`, `probe-routes.ts`, `rewrite-commands.ts`, `run-script-with-envelope.ts`, `scan-wide-readers.ts`). Temporary generated output was deleted.
