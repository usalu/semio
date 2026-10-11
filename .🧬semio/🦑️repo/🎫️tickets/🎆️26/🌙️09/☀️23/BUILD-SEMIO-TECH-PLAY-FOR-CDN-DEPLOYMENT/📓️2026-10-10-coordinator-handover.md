# 2026-10-10 Coordinator Handover (main)

Read with `📓️2026-10-10-redeploy-fleet-rules.md` (timeline + decisions) and `📓️2026-10-10-migration-corrections.md` (fleet-wide API decisions #1–#50).

## Why this is big
09:07 fast-forward to origin c44e964 brought a half-applied grant-based retained-clone protocol migration (value/job/replication/store/plugin + all 35 plugins) plus merged/missing files (2d kerning, plugin crate). Reconstructed forward with a Sonnet fleet; Haiku for audits.

## Coordination mechanics
- Cargo gate: `bash <ticket>/🚦️cargo-slot.sh <agent> -- env CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/<agent>/target CARGO_BUILD_BUILD_DIR=…/build CARGO_BUILD_JOBS=3 cargo …` (6 slots).
- Status files: `🗑️generated/coord/{os,core}.status`. Plugin artifacts are standalone workspaces → `--manifest-path <artifact>/Cargo.toml`; check with `--all-features` / `component-app-assembly`.
- Budget rule #45: release path first (lib native + wasm32 with component features), tests after the cold release. Park the fleet at ~88% of the 5h window.
- Rule #46: any executor may add derive-only lines cross-scope.

## Agent roster (SendMessage ids)
| agent | id | scope | state @19:13 |
|---|---|---|---|
| core-tests | aac758f1b1a0a9f7d | value, pack, job, replication, async, tool-run, tool-machine, action-bus, io, layout-run | libs green; test builds pending (replication 12, tool-machine 5, job, pack 1 fail) |
| fd-init | a1dc519f76d050e12 | store tests + replay initializer | initializer default; canonical-edit/durable_group tests pending |
| fd-window-edit | ac5c953b122e90bf8 | store snapshot-clone/paged/mutation_apply/config_apply | done; ReplaceEdit footprint item maybe pending |
| derive-ext | a690caed55ea85661 | canonical derive + pack-json tree | done (#44, #48 hex_word, #49 tree) |
| pl-a/pl-b/pl-c/pl-d | ac8b72ac192504708 / addd1d489d53ef76e / adae7dd7d960258ba / a34d94f9f890fe7d3 | plugin crate / jobs+macros / host+os host / plugin tests | plugin lib + host + os host green; plugin tests 33 errs (pl-d); pl-c TODO: delete compat media entry points |
| os-domains | a29b29004c0b7419c | 💻️os domain crates | flow/playbook/workflow/os-flow green; renderer (not in release) + tests later; review GenerationPlayRoot manual RetireOwned |
| fd-ui | ab86add1e0aba24ce | ui | green |
| stdio-a | a5ad187b054b531a4 | stdio semio/gltf/pdf/zip/deflate/step | all green all-features (semio −bcf) |
| stdio B (rn-stdio) | a834e1a9397d5884f | other stdio formats + registry contract | 12 green; xlsx/pptx native; svg/bmp/png/jpg/tiff/gif/bcf/wav fixes unchecked; epw/stl/obj/ply/binary unrun; then diff_apply→mutation_apply swap (347 RetainedClone) |
| stdio-c | a56a24dea5d1534be | docx, ifc | green |
| mg-trinity | a63906dcf7ee3ee7f | trinity | RELEASE PATH GREEN |
| mg-raster | a45c94bb136b1985a | raster | native green; wasm pending (mesh-engine fixed) |
| mg-draw | a9c4682e28a70efde | draw | 216 errors; context ~900k → REPLACE with fresh draw-host + draw-schema |
| rn-proc | a5490d0971adc4164 | puzzle, process, spatial-kernel, TriMesh | puzzle3d native green; 2d/5d need feature forwarding; process3d re-run |
| mg-procspace | adc98dca314765694 | procedural, space | space green; gen2d native green, wasm 44; gen3d geometry → NEW g3d-geo |
| rn-misc | a714ef42a3de23800 | cad, sourcing, gis, lowpoly, sequence, layout | curation/gisterrain/gismap green; others pending; layout waits forms |
| mg-misc2 | a1cd2b202f150d086 | block, dag, note, remodel, playbook, forms, reasoning, demonstrator, vcs, flow | block×4/note/dag/reasoning green; forms fix pending |
| mg-rest | a66a4516b8fa4e628 | math, shooting, bim, architect, norm, imperative, writer | context ~960k → REPLACE fresh |
| rn-wfc | a8d3d69b6f6f6b54b | wfc, energy, fem, animate (+fem engine) | wfc×5/fem2d/3d/animate native default green; energy 13; context ~840k → consider fresh |

## Next phases
1. Finish release path for every plugin crate (lib native + wasm32 component features).
2. Cold release: `bash <ticket>/🔁️run-fresh-2026-10-10.sh <n>` (detached via python double fork; log `.🧬semio/🦑️repo/⚡️cache/play-fleet/coordinator/fresh-<n>.log`), fix pipeline failures (describe/materialize/catalog/vite).
3. play unit tests, `test-release-routing`, publication audit, `test-release` (149 Chromium cases) against `dist/pages`.
4. Deferred test triage waves, perf follow-ups (rules log), close ticket (`ticket_close` with explicit path; files list; delete `🗑️generated`).

## 19:15 wave (after reset)
Resumed: stdio B, mg-raster, rn-proc, rn-misc, mg-misc2, rn-wfc, mg-procspace. NEW fresh: draw-host a99794d2446e2ca34, draw-schema a3ea65462dcd23c83, rest2 a27681168b93257ac (replaces mg-rest), g3d-geo a089b0ce4a60b8993. Briefs in scratchpad (`brief-<name>.md`, template `fresh.md`).
NEW 19:35: puzzle5d ae2e39230f0da0aef, puzzle2d aa8e8c2313e77b74f (rn-proc keeps 3d + process3d). Release-path green so far: trinity, raster; native: lowpoly, puzzle3d, process3d, curation, gisterrain, gismap, space(+wasm), block×4, note, dag, reasoning-wires, wfc×5, fem2d/3d, animate, imperative core/procedure/extensions(+wasm), norm contract(+wasm)+6.

## 20:20 key discovery + new agents
The DEPLOYED plugin components are the 35 `semio-hub-<plugin>` crates in `🌎️hub/🧩️compositions/<plugin>/📦️packages/🦀️rust` (registry plugins.json lists semio-hub-bim/draw/puzzle). They depend on the `✏️s/🔌️plugins/*` crates. New executors: hub-a a567886cc4450686a (trinity, raster, draw, stdio, space, block, note, dag, reasoning, imperative, norm, procedural), hub-b a662a58a21d04679b (puzzle, process, cad, gis, sourcing, lowpoly, sequence, layout, wfc, fem, animate, energy), hub-c ab206bf274b03f2c9 (flow, playbook, forms, demonstrator, vcs, remodel, mathematical, shooting, bim, architect, writer).
Release-path GREEN (plugin crates): trinity, raster, draw (+bridge), all stdio (incl. semio all-features), space-core, space, home, generation2d, puzzle2d; native-only so far: puzzle3d, process3d, curation, gisterrain, gismap, cad, lowpoly, sequence, wfc×5, fem2d/3d, animate, block×4, note, dag, reasoning, imperative core/procedure/ext(+wasm), norm contract(+wasm)+6.

## 21:15 SNAPSHOT (for main after context compaction)
Usage: 5h window 29% (resets 00:10 local), weekly 57%. Park fleet at ~88%.
RELEASE-PATH GREEN plugin crates (native+wasm): trinity, raster, draw(+bridge), all stdio, space-core, space, home, generation2d, generation3d, puzzle2d, puzzle5d, wfc×5, fem2d/3d, energy, animate, curation, gisterrain, sequence, cad, lowpoly, imperative ext. Native-only: puzzle3d, process3d (rn-proc wasm pending), gismap (wasm re-check), block×4 default-features only (hub features red → mg-misc2), note, dag, reasoning, imperative core/procedure, norm contract(+wasm)+6.
OPEN plugin work: mg-misc2 a1cd2b202f150d086 (forms→layout, remodel, playbook, demonstrator, vcs, flow+ext, catalog, block with component features); rest2 a27681168b93257ac (norm en1992 one-liner + rest, math, shooting, bim, architect, writer); rn-proc a5490d0971adc4164 (puzzle3d/process3d wasm); rn-misc a714ef42a3de23800 (gismap wasm, layout after forms).
HUB (deployed components): hub-a a567886cc4450686a (native green: trinity(+wasm), note, draw, raster, procedural, imperative, reasoning, dag; hub-space 116 own errors in progress; blocked: norm, block), hub-b a662a58a21d04679b (no report yet), hub-c ab206bf274b03f2c9 (idle; resume when writer/forms/remodel/cad/vcs/flow/math/shooting/bim/architect artifacts green — cad is green now). Hub crates live in the HUB workspace `🌎️hub/Cargo.toml`.
NEXT after all hub crates green: cold release (`🔁️run-fresh-2026-10-10.sh <n>` detached; log in play-fleet/coordinator), then unit/routing/publication/test-release.
main's own edits so far: tool-run/tool-machine Cargo.toml dup keys, lockfiles relock, play site build-fresh + `runOwnedNxInvocationV1` (bootstrap), os host async dep, graph `BorrowedDslField for PropertyValue`, plugin `PluginApp` bare names + `mod app` re-export, `window_transient_transfer!` dup impl removal.
NEW 22:00: os-wasm a388e6af2ac7521d5 (framework wasm-pack/wasm-bindgen release targets, e.g. semio-framework-os-flow-core:wasm). Trial build #4 running since 21:10 (log play-fleet/coordinator/fresh-4.log).

## 22:50
os-wasm DONE (10 wasm-bindgen crates green on wasm32-unknown-unknown; `1<<32` usize overflow fixed in step/ifc/docx; framework/Cargo.lock wasm-bindgen pinned back to 0.2.126 = installed CLI; cad/trinity locks relocked). PIPELINE BLOCKER: cargo provenance receipt OOM (`📚️library/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`) → NEW receipt ac96a7e52b9695b4a (schema-first redesign: interned paths, hashes, streamed write). derive-ext added flatten (50) + external named-variant views (51, supersedes #13). rest2: norm×15, writer, math, shooting, architect, imperative GREEN native+wasm; bim in progress. hub-c checking writer/math/shooting/architect hubs. Trial build #4 still running (will fail on receipt; kill + relaunch #5 after receipt fix + bim).

## 23:05 STATE (pre-compaction) — READ THIS FIRST
- ALL plugin artifact crates release-path GREEN (native + wasm32). HUB compositions 34/35 green; semio-hub-bim being checked by hub-c (ab206bf274b03f2c9).
- PIPELINE BLOCKER: provenance receipt OOM → receipt executor ac96a7e52b9695b4a. When it reports done: launch cold build #5 (`python3` double-fork of `<ticket>/🔁️run-fresh-2026-10-10.sh 5`, see earlier launch snippet in rules log; monitor `.🧬semio/🦑️repo/⚡️cache/play-fleet/coordinator/fresh-5.{log,status}`), fix pipeline failures, then play unit tests, test-release-routing, publication audit, test-release (149 Chromium cases).
- EDITING WAVE (runtime, parallel): plugins still binding refusing `bounded_config_store_one_item_preparation_factory` or interim stdio `diff_apply_preparation_factory`: writer, wfc(5), animate, shooting, bim, architect, layout, norm, playbook(2), imperative, remodel, energy, dag, note, stdio(32), hub space. Swap per corrections #36–#39 + derive-ext 51: config lanes → `store::snapshot_clone_preparation::config_apply_preparation_factory`, document lanes → `store::mutation_apply_preparation_factory` (snapshot needs RetainedClone), non-mutating → None. Dispatched to: swap-a (fresh, non-stdio plugins) + stdio B a834e1a9397d5884f (stdio + delete diff_apply).
- Usage @23:05: 5h 39% (reset 00:10), weekly 60%.
swap-a = af8be2c7e11b9ca7c (non-stdio editing wave). stdio B resumed for diff_apply → mutation_apply.
inv-fix = a8668fd512bcf21df (missing invocations: describe fresh-component, browser-bundle, trunk test). Cold build #5 running since 23:26 (fresh-5.log).

## 00:30 (2026-10-11) — compaction imminent
Cold build #5 running since 23:26 (45/~321 tasks @00:27, no failures yet; monitor via `fresh-5.status`). Active agents: swap-a af8be2c7e11b9ca7c (editing wave non-stdio), stdio B a834e1a9397d5884f (diff_apply → mutation_apply). All others idle/parked. Usage: 5h 1% (reset 05:10), weekly 62%. NEXT: on build #5 exit → if failures: route by task/project to owners (see roster) and relaunch #6; if exit 0: run play `test` unit, `test-release-routing`, publication audit, `test-release` (see `📓️2026-10-10-release-verification-path.md` for exact launch names), then deferred test triage + close ticket.
- 00:45 stdio B: diff_apply deleted, 30 sites → mutation_apply (~370 RetainedClone derives); 28 B crates green. NEW RED: stdio-pdf (needs RetainedClone for non-Copy arrays → core-tests). Runtime bug: bmp/png publication preflight 1 MiB vs sealer walk → fd-window-edit. Build #5 may fail pdf tasks → relaunch #6 after core-tests fix.

## 01:00 status
- swap-a DONE (📓️2026-10-10-swap-a.md): every non-stdio plugin off refusing bounded factory; config lanes → config_apply, doc lanes → mutation_apply (norm via shared generic); native+wasm --lib green, hubs wasm green; proofs shooting/dag preparation_law pass. Runtime caveats: layout/remodel/bim mutation_apply capacity grant must cover snapshot clone; writer base cap 32 KiB; config payloads >~123 KB refused (imperative SetRunOutput, architect SetConfig) → watch in test-release.
- core-tests DONE: array RetainedClone; pdf green (corrections #52).
- Build #5 running (36 projects @00:58, no failures). swap-a edits landed mid-build → #6 needed anyway for a coherent artifact set.
- Active: fd-window-edit ac5c953b122e90bf8. Waiter bevnqwev0 on fresh-5.status.
- 01:35 fd-window-edit DONE (corrections #38 second entry): store footprint now live-capacity, document clone uncharged; bmp+png preparation_law green; CONFIG_EDIT_RETAINED_BASELINE_BYTES 131072. Store changed mid-build → #6 mandatory.
- 01:35 LAUNCHED bulk-clone a0f3fef29461bfd9d (Sonnet): chunked bitwise Vec<T> RetainedClone (pixel editors 27M turns/25 s per 1024x512 edit). Report 📓️2026-10-11-bulk-clone.md. Land BEFORE build #6.
- 02:45 bulk-clone DONE phase 1 (corrections #53): bitwise Vec chunked clone, 4 MiB Vec<u16> 66 turns; value lib 305/0; png/bmp laws green; 1024x512 png edit 27M→2.1M turns, rest = per-sample png validation. RESUMED same agent (a0f3fef29461bfd9d) phase 2: page owned validation/inverse work across all pixel editors + launch.json entry for bulk-vec-retained suite.
- Build #5 @02:44: 46 projects, still no failures (slow: load ~21 from peers). Keep it running to discover late-stage (catalog/prepare/site) failures; #6 after bulk-clone phase 2.
- 03:15 bulk-clone phase 2 DONE (corrections #54): paged png/bmp validation+paint, `#[retained_clone(bitwise)]` derive, 1024x512 png edit 5,015 turns; launch.json entries added (group 4_gate). RESUMED phase 3: bulk scalar arrays in canonical sealer (bmp 64x40 paint = 922k turns).
- Build #5 @03:12: 46 projects, alive (synchronize step), log 1.78 GB — spam = rustc "failed to compute checksum … Is a directory" dep-info lines (flow extensions; directory paths tracked as dep-info inputs). FOLLOW-UP: find the tracker (build.rs rerun-if-changed / tracked_path on directories) and track files instead.
- 03:45 depinfo audit DONE (📓️2026-10-11-depinfo-directory-audit.md): compile-time resource observer `read_dir` (🏃️process/…/🧮️compiler/🦀️.rs:95,99) tracks directories via proc_macro::tracked::path for the DSL derive `$id` index walk; checksum-freshness (.cargo/config.toml:18) makes rustc log EISDIR. DECISION: NOT removing directory tracking in this ticket — directory entries are the only new-file signal for the walk (cargo falls back to mtime for checksum-less entries), so removal risks stale builds; noise is log-only. Proper fix (generated tracked schema index replacing the walk) = separate ticket.
