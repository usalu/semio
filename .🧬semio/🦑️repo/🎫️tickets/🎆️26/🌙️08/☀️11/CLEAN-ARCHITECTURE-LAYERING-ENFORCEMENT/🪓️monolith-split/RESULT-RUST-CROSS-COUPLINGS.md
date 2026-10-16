# Result: Rust Cross Couplings (P8 Rust side, hub shipped-fleet fixture, file-read residuals), prefix `rcc`

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Started 2026-10-16 06:41. Tools: `rcc-tools/`. Logs: `🗑️generated/rcc-*` (nothing deleted).

## 0. Verdict

| Target | Status |
| --- | --- |
| 1. P8 Rust (layout crates depend on the stdio drawing crates and `include_str!` the drawing DDL) | **Not done.** Evidence and the staged design are in section 2. The surface is 30 Rust files / 155 uses / 11 manifests / 11 stdio crates, and the port needs typed lowering in the scene, export and io crates. No compile was possible (section 6), so a hybrid half-slot state would have been unverifiable and is deliberately not left behind. No manifest, descriptor, registry or build-owner row changed. |
| 2. Hub shipped-fleet fixture (5 `include_str!` paths into `✏️s/…/🗄️stdio/🧪️tests/🧫️fixtures`) | **Done** (static; the compile proof is queued, section 6). The fixture stays with its owner (stdio, shared with the TS twin). The hub Rust test now reads it through the stdio-owned crate constant `semio_s_artifact_stdio_contract::testing::SHIPPED_FLEET_SQLITE_OWNERS` (section 3). No hub file reads an `✏️s` path for it any more, and no `✏️s` file reads hub internals. |
| 3a. gis `🧱️io-boundary` test reads plugin and dev-variant sources | **Done.** Test 1 was a weaker duplicate of a law the s dev owner already enforces; its one unique assertion moved there (section 4.1). A stale owner path in the remaining test was repaired. |
| 3b. framework vite artifacts tests read Play and demonstrator `📋️project.json` | **Done.** The Play half moved to Play, the demonstrator half to the demonstrator, both as bun tests with launch gates (section 4.2). |
| 3c. hub composition generator reads plugin artifact directories by data-declared paths | **Justified as a sanctioned composition-root read** (section 4.3). |

Cross-technology Rust `include_str!`/`include_bytes!`/`#[path]` count (command of section 5): **12 before, 7 after** (all remaining 7 are in hub composition roots; section 4.3).

## 1. Baselines (before any edit)

| Check | Result |
| --- | --- |
| Cross-technology Rust includes (hub + framework → `✏️s`/`🏢️semio-tech`/`♻️mit-bestand`), `rcc-tools/include_scan.py` over 9894 grep lines (9225 resolved directives) | **12**: 3 `include_bytes!` + 9 `include_str!`, all hub → `✏️s` (`rcc-include-before.tsv`) |
| Framework vite artifacts test (`bun test`, `rcc-p3b-framework-before.log`) | 43 tests, 41 pass, 2 fail (`neutral browser inventory …`, `neutral package inventory …`, both need `SEMIO_TICKET_DIR`; identical to `RESULT-SMALL-CROSS-EDGES.md`) |
| Hub gis `🧱️io-boundary` test | 1 file, 2 tests; test 2 already failed (stale framework owner path, see 4.1). Test 1 not separately run. |
| s dev ownership test `sDevCompositionOwnershipV1` | fails at its own line 22 (an import-shape assertion), before and after my edit; a peer-owned fault (`rcc-p3a-ownership.log`) |
| Layout / hub-stdio Rust compile baseline | **not obtainable**: every launcher call ended in `Insufficient physical compiler initial headroom` (section 6) |

## 2. P8 Rust: layout versus the stdio drawing crates

Measured state (all by `git grep`, `rcc-layout-stdio-rs-uses.txt`):
- 11 layout manifests (`✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/**/Cargo.toml`) declare stdio dependencies: 32 member lines over 11 stdio crates (drawing 10, base 8, png 4, svg 2, dwg-geometry 2, and one each of drawing-import, drawing-export, json, dwg-write, dwg-snapshot, dwg-read).
- 155 use sites in 30 Rust files. Production uses:
  - `🧬️schema` (model crate): `LayoutDrawingChild { handle: ArtifactChild<SemioDrawingSnapshot>, content: SemioDrawingSnapshot }`, `set_drawing_text`, `drawing_labels` (walk `DrawNode`), plus dead-looking dwg/svg/geometry imports.
  - sqlite snapshot: `include_str!` of the drawing `🗄️.sql` concatenated into `SCHEMA` and `content.project_sqlite_component`.
  - component codec, diff, the `set-drawing-text` mutation, `patch-frame`.
  - engine scene (`collect_drawing_text`, `collect_drawing_rasters`, `collect_draw_node`, PNG decode), export and export-json (walk `DrawNode`/`PathSegment`, geometry types), editor root, canvas (PNG).
  - io: dwg, dxf, json, svg and png import/export through `encode_drawing`/`decode_drawing`, `CONVERSION.register`.
- 1 runtime source include into a sibling plugin: the drawing DDL.

Why the TS design does not transfer one to one: the TS twin only needed three codec facets (parse, native JSON, sqlite), because TS layout never walks drawing nodes. The Rust layout engine renders and exports the drawing as typed `DrawNode` trees (labels, rasters, strokes), and `LayoutSnapshot` derives its pack, DSL value and sqlite codecs statically over the concrete content type. A faithful twin therefore needs all of the following together:
1. In layout (the consumer owns the port): `LayoutDrawingContent` becomes an opaque framework value tree (identity `ToValue`/`FromValue`, generic pack), and layout declares one slot trait (`EmbeddedChildCodecV1`: `parse`, `native_json`, `sqlite_schema()`, `project`, `reconstruct`, `lower() -> LoweredDrawing`, `set_text`, `labels`). `LoweredDrawing` is a layout-owned neutral struct that the scene and export already compute their own outputs from (labels, rasters, paths with paint and transform).
2. `SCHEMA` becomes `OnceLock`-bound at first use (the DDL is the bound codec's), with the unbound-refusal and idempotent-rebind laws of the TS slot, mirrored in a Rust test.
3. The PNG decode in scene, canvas and export, and the dwg/dxf/svg/json/png import/export in io, move behind the same binding, or are reached through the framework conversion registry by `Dialect`, which `ArtifactComposition::reads()` already declares (`s.stdio.dwg`, `dxf`, `json`, `svg`).
4. The binding crate lives in the composition project (`🌎️hub/🧩️compositions/📏️layout`, whose `semio-hub-layout` already depends on the stdio dwg, dxf, json, pdf, png and svg crates, or `✏️s/🧪️tests/🏘️plugin-composition/📏️layout-drawing`, next to the TS binding). It implements the trait over the typed drawing and registers it once in `plugin()`.
5. Then 11 manifests lose 32 lines, the layout workspace `Cargo.toml` loses its 11 `[workspace.dependencies.semio-s-artifact-stdio-*]` tables, the three Cargo.lock files refresh, and the hub layout composition gains the binding dependency. No `build-owners` row changes (crate-to-owner rows are unchanged; only edges change).

Decision: not started. It touches about 30 files in 9 crates of the layout workspace and the hub composition, with no way to compile any of them in this window (section 6); the engine and export code are large one-line functions where a type error is likely. A half-installed slot (some crates through the port, some direct) would be a dual mechanism, which the greenfield rules forbid. Recommended order for the follow-up ticket: (a) land the trait and slot with its unbound/rebind test in the layout model crate, (b) switch model, diff, mutations, sqlite, component codec, (c) lower in scene and export, (d) move the io conversions, (e) remove the manifest edges and refresh locks. Option 1 of `RESULT-PLUGIN-CROSS-EDGES.md` (host the Semio drawing and base standards in the framework) removes the same 32 lines without the port, and is the cheaper alternative once the compiler is available.

## 3. Hub shipped-fleet fixture (target 2)

Finding: `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs` had 5 `include_str!`s of `✏️s/🔌️plugins/🗄️stdio/🧪️tests/🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json`, which `RESULT-SMALL-CROSS-EDGES.md` section 3 had re-pointed there when the TS twin moved to stdio. The fixture holds the authored natural texts and exact domain rows of the Binary, Txt and GeoJSON owners (`witnesses`) and is shared language-agnostically by the stdio TS test and this Rust test.

Rightful owner: stdio. The data is derived from stdio's own authored demos (`examples::demo::source()` is asserted equal to `naturalText`). The Rust test itself is a hub + stdio composition law: it installs the hub's assembled stdio packages (`packages()`, `semio_hub_stdio_catalog`) before it checks the sqlite routes, so it belongs in the hub composition test root and cannot move to `✏️s`. Moving the fixture to the hub would make the stdio TS test read hub internals; duplicating it would create drift.

Change (static): the stdio-owned crate `semio-s-artifact-stdio-contract-testing` (its description is "Original caller authorities for stdio artifact tests", re-exported as `semio_s_artifact_stdio_contract::testing`, which this hub test already uses) exports `pub const SHIPPED_FLEET_SQLITE_OWNERS: &str = include_str!("../../../🧪️tests/🧫️fixtures/🚢️shipped-fleet/🪶️sqlite/🧠️owners/🔣️.json")`. The include is now s to s. The hub test replaces its 5 path includes by `semio_s_artifact_stdio_contract::testing::SHIPPED_FLEET_SQLITE_OWNERS` (the hub package already depends on that crate with feature `testing`). The visible dependency is a Cargo edge instead of an invisible source-path read. The path exists (`ls`). `rcc-tools/p2_repoint.py` asserted exactly 5 replacements.

Files: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧰️testing/🦀️.rs` (+3 lines), `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs` (5 lines). No manifest, lockfile or build-owner change.

## 4. File-read residuals (target 3)

### 4.1 gis `🧱️io-boundary` (done)
Test 1 ("GIS receiving configuration selects its owner and all fixture URLs resolve") read the gismap vitest configuration and listed `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/*` with a regex. Everything it touched is s (plugin and dev variant); the hub added nothing. The s dev ownership test `sDevCompositionOwnershipV1` already walks the same directory with a stronger TypeScript-AST reader and resolves every fixture URL (`receivingFixtures > 8`). Fix: the one unique assertion (the gismap receiving configuration names `🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/👷️worker`) was added to that s test right after the `receivingFixtures` check, and test 1 was deleted from the hub file (unused `readdirSync` import removed). Verified separately: the gismap configuration contains the literal (bun check, true).
Test 2 stays in the hub (it reads hub receiver files and the framework schema oracle only). It was already failing: its expected owner was `🧬️schema/🔮️oracles/🪪️document/🟦️.ts`, while `compileSchemaScopeExport` lives in `…/✅️validator/🟦️.ts` and `📇️native-codecs` imports it from there. I corrected the expected path; result: `bun test` 1 pass, 0 fail, 32 expect().
The s ownership test still fails at its own earlier line 22 (peer-owned import-shape assertion, unchanged by me), so my added assertion is checked by the standalone bun probe above, not by that test's completion.

### 4.2 framework vite artifacts tests (done)
Two framework tests read the product project files:
- Barrier test: after its framework assertions on `componentStagePlanV1`, it read Play's `pages` release job. Moved to `🏢️semio-tech/🎡️play/🧪️tests/🧩️release-job-order/🟦️.ts` (Play reads its own `📋️project.json`).
- Engine producer test: it joined the framework engine catalog with Play's bundle job `after` list and the demonstrator's `prepare-prototype`, `prepare-release` and `build` `dependsOn`. Split by owner: the Play half is the second test of the Play file; the demonstrator half (with the independent in-memory SQLite witness) is `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧩️engine-producers/🟦️.ts`. The framework test keeps the catalog's own law (each source owner's `wasm` target declares `wasmPackageSourceV1`) and the inference-source check, and was renamed accordingly.
- Both products import the framework catalog JSON (product to framework, the allowed direction).
- Registered in both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (parsed with `Bun.JSONC`, 2 new entries each, orders 13.59 and 13.6 unique): `⚖️gate🏢️semio-tech🎡️playrelease-job-order`, `⚖️gate♻️mit-bestand🧺️demonstratorengine-producers` (`rcc-tools/launch_insert.py`, `launch-p3b.json`).
- Results: framework file 43 tests, 41 pass, the identical 2 `SEMIO_TICKET_DIR` failures (set unchanged); Play 2 pass; demonstrator 1 pass. `tsc --noEmit --strict` over the five touched TS files: 1 error, in the s ownership test line 16 (TS7006, not a line of mine).
- Remaining and not changed: the framework test's fixture `🧪️tests/🔣️.json` still names 4 Play/demonstrator script paths in `callers` and one Play file in `storePhaseCallers`, which the tests read as source (a closed caller roster of the framework API). That is a roster law; moving each row to its caller would need a framework-side registry port. Also the engine catalog names a hub project (`puzzle`) as a source owner, so the framework catalog test reads a hub project file. Both are separate framework to product data couplings, left and listed here.

### 4.3 hub composition reads of plugin artifacts (justified, sanctioned)
The 7 remaining cross-technology Rust includes, all in `🌎️hub/🧩️compositions/*`:
- gis `📇️native-codecs` ×2 and vcs `📇️native-codecs` ×1: `include_bytes!` of the artifact's `📡️.protocol.semio`. This is the composition receipt. The composition data (`📇️native-codecs/🔣️.json`) declares `protocolPath`, an independent TS oracle hashes that file (gis `proveGisNativeCodecReceipts`, not read in detail) and the vcs oracle (`🧪️tests/📇️native-codecs/🟦️.ts:92`, read) requires the Rust source to contain `include_bytes!("../../../../<protocolPath>")`, so the pinned bytes are those of the declared path. I tried the owner-side constants (`COMPONENT_PROTOCOL_SEMIO`, which exist and are public) and reverted when the vcs oracle showed the pin is the contract. No net change in these files.
- norm `🖥️app-surface` test ×1, stdio `📇️catalog` unit test ×1 (kit mutation schema), space `interactive-job-catalog` test ×2: two-sided contract fixtures owned by the plugin's contract or editor crate, read by the composition test that checks the hub assembly against them. They sit under hub `🧩️compositions` (the composition root, where hub and plugin meet), and the owners' own tests read the same files. A crate constant per fixture would add test data to production crates; recommended only together with a `testing` feature per plugin (stdio already has one, used for target 2).
- The hub composition generator and package scripts (`🌎️hub/🧩️compositions/*/📦️packages/🦀️rust/📜️script.ts`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts`) read plugin artifact directories through paths declared in composition data (`native-codecs/🔣️.json`, `protocolPath`, `artifacts = [...]` in component manifests `[package.metadata.semio.sources]`). The cruiser policy already exempts package scripts as composition roots, and the paths are data, validated by schema. Decision: legitimate composition data, no change.

## 5. Commands run (real results)
- Cross-technology include count: `git -c core.quotepath=false grep -n -E 'include_(str|bytes)!|#\[path' -- '🌎️hub' '🧰️framework'` then `python3 -I rcc-tools/include_scan.py <raw> <out>` (resolves relative literals and `concat!(env!("CARGO_MANIFEST_DIR"), …)` forms, keeps targets under `✏️s`, `🏢️semio-tech`, `♻️mit-bestand`): before `directives=9225 cross=12`, after `directives=9220 cross=7`.
- `bun test` on the framework artifacts file (before 41/43, after 41/43), the Play file (2 pass), the demonstrator file (1 pass), the hub gis file (1 pass).
- `tsc --noEmit --strict …` (`rcc-tools/tsc_run.sh`) over the five TS files: 1 pre-existing error (above).
- Scoped cruiser over the five TS files, exact command of `DEPENDENCY-DIRECTION-AUDIT-2.md` section 1 with the files as arguments: exit 86, 1110 violations, 834 modules. Rule lines for `hub-to-✏️s`, `framework-no-implementation`, `no-cross-plugin`, `framework-no-products`: **0**. The new tests add hygiene lines (P11 family): one `no-cross-technology-♻️mit-bestand-to-🧰️framework` (the demonstrator test importing the framework catalog) and two `no-cross-package-relative-@semio-tech/framework-os-dev` (Play and demonstrator test, same JSON import). Log `rcc-cruise-after.txt`.
- Launcher cargo: see section 6.

## 6. Compile proof (queued)
Command queued under the fleet mutex of `📓️WAVE5-BRIEF.md` ("Compile admission is single-slot"): `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-hub-stdio --lib --tests --keep-going --message-format=short --manifest-path 🌎️hub/Cargo.toml` (`rcc-tools/headroom_retry.sh` retries every 10 minutes, up to 6 times).
- 06:41 to 06:48: at least seven unmutexed launcher attempts (`rcc-base-layout-lib.attempts`) were refused with `Insufficient physical compiler initial headroom` (no compile ran). I stopped that loop when the coordinator's single-slot message arrived.
- 06:52 under the mutex: refused again, `availableBytes 4262895616` against `requiredBytes 4294967296` (short by 32 MB); the retirement plan found no unit it may retire. Mutex released at once (`rcc-mutex.log`). Log `rcc-hub-stdio-tests-attempt1-refused.log`.
- Retries every 10 minutes under the mutex: 07:03:57, 07:14:01 and 07:24:04 all refused with the same `availableBytes 4262895616` (the image did not change). I stopped the retry loop at 07:25 (a background child would die with this turn anyway); the mutex is released and nobody holds it from me. Re-run the command above when headroom returns.
Not compiled, therefore not claimed: the new constant, the 5 re-pointed hub reads (path existence is checked, type correctness is by inspection: a `&'static str` replaces the `include_str!` expression of the same type).

## 7. Files changed
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧰️testing/🦀️.rs`
- `🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs`
- `🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🧱️io-boundary/🟦️.ts`
- `✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🏷️ownership/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🧪️tests/🟦️.ts`
- new: `🏢️semio-tech/🎡️play/🧪️tests/🧩️release-job-order/🟦️.ts`, `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧩️engine-producers/🟦️.ts`
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (2 entries each)
- tools: `rcc-tools/` (`include_scan.py`, `p2_repoint.py`, `p2b_protocol_const.py` (applied then reverted), `p3a_gis_receiver.py`, `p3b_move_product_laws.py`, `launch_insert.py`, `launch-p3b.json`, cargo/tsc wrappers).
Several files above also carry uncommitted edits of peers (`RESULT-SMALL-CROSS-EDGES.md` work and others); I touched only the lines described.

## 8. Open points and hand-offs
- P8 Rust follow-up ticket (section 2), or option 1 (framework formats module).
- Compile proof of target 2 (section 6): run the queued command when the image has 4 GiB headroom; expected result: `semio-hub-stdio` lib and `shipped_fleet` test target check with 0 errors. If it fails, the only changed code is the one constant and the 5 expressions.
- Peer-owned faults seen: s dev ownership test line 16/22 (`row` implicit any; import-shape assertion), framework artifacts test needs `SEMIO_TICKET_DIR` for two tests.
- No `[DEBUG] ` lines were added to any source.
