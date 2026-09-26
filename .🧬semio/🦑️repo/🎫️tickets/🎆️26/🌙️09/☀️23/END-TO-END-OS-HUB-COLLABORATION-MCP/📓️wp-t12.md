# WP-T12: Outcome Switch Landing, T11 Frozen Follow-Ups, Inventory And Lib-Test Debt

Slice: T12 (session 11). Captures: **`.🧬semio/🌐hub/s11-t12-captures/`** (re-measured; see below). Inputs: `.tmp-ticket/wp-t12/*`. Ports 8060–8069 / 6560–6569.

> ⚠️ **12:19–12:35 low-disk sweep.** An external cleanup deleted the whole `.tmp-ticket/wp-t12/` folder (inputs, `generated/`, `backup/`, `target/`). The 43 inputs were restored byte-exact from the git index (read with `git show :path`, no git state touched); the jack additions made after the last auto-stage (vectors 35–38, decisions, harness arms) were rebuilt from the committed fixtures and this report. `generated/editor-survey.json` (derived from T11's deleted `wp-t11/generated/editor-paths-0.json`) is gone and not rebuildable; its registrations are committed. Every capture this report cites is re-measured by `wp-t12/remeasure.sh` into `.🧬semio/🌐hub/s11-t12-captures/<step>.txt` (started 12:47); capture names below that end in `-N.txt` refer to the lost originals, their re-measurement is the step of the same topic in that folder. **Re-measurement complete: §8.**
Inherits: T10 (`📓️wp-t10.md`, `wp-t10/`), T11 (`📓️wp-t11.md`, `wp-t11/`), T5, T8, T9.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 0. **S15 guest defects (coordinator priority, 07:1x)**: stdio ×9, curation ×2, trinity ×2, norm ×2, gis viewer, generation2d viewer, vcs viewer | **all fixed at the root, each with a law** (§S): stdio kit verbs registered + artifact publication authority (9 live laws), curation numeric args, norm snapshot decode, generation2d viewer owners (+6 preventive viewers, + draw), gis viewer camera lane, vcs viewer document row, trinity patchNodes/rail verbs. Native lib tests green per crate; **one wasm32-wasip2 check of all 15 touched guest plugins EXIT 0 (95 s, fleet mutex)**. Every package recorded in `wp-w1/requests/t12.txt` (items 4–9) | `wp-s15` matrix, `wp-w1/requests/s15.txt`, `check-s15-wasm-1.txt` |
| 1a. Landing: T10 outcome switch (`switch.py` + `manifest-align.py --write`), compile-atomic | **landed, green** (§1). Standalone bridges: the check sweep was stopped (5/37 green) because every bridge is regenerated for state-lane surfaces (§3) and rebuilt by the next inventory | `switch-apply.txt`, `check-switch-1.txt`, `check-switch-wasm-1.txt`, `tsc-switch-2.txt`, `check-switch-bridges-2.txt` |
| 1b. Landing: T11 frozen follow-ups | **landed, green, "T12 landing done" 01:08** (§2): binary one name `replace-byte-range`; dwg ac1018 duplicate leaves deleted; jpg/tiff baseline leaf fields `pub`. W2 messaged | `check-followups-1.txt`, `check-followups-oracle-1.txt`, `check-followups-wasm-1.txt` |
| 2. pdf 1.7 base runtime-only 44 | **done** (§6): the 44 kinds production dispatch offered without a manifest row are declared; lopdf writes one before/after pair per kind (60/60 observable, byte-reproducible, reader probe re-qualified 120/120 directions); 44 laws apply each kind, WRITE it with the subset's writer, read it back and require lopdf's after-document, then require the inverse to write back to the before-document: **44/44, crate lib 502/502**. Two laws needed two steps (a shading pattern's shading is its own collection item) — finding F3 | `pdf-lopdf-vectors-1/2/3.txt`, `pdf-lib.txt` |
| 3. Editor/viewer state-lane vocabularies (T11 §1: 39 real) | **done for 35 + gis camera; trinity rewriting blocked (F1)** (§3). Re-measured after the sweep: harness build EXIT 0, `editor-measure` EXIT 0, law::vector 5/5. Earlier state: Vectors: **86/86 measured, 0 findings** after the fix below (`editor-measure-2.txt`). Landed + checked: kernel `mutation_report_json` (`check-kernel-helper-1.txt`), `law::vector` in the stdio test-oracle crate (unit tests **5/5**, `test-law-vector-1.txt`). Harness built; vectors measured. **Bug found and fixed:** architect presence/config and imperative config wiped their record to defaults on an identical re-apply (§3a; `check-noop-1.txt`, W2 request filed). Since landed: 35 vocabularies registered (589 files), all 36 bridges regenerated, inventory clean; trinity's 5 wait on a taxonomy move + private paths (F1) | `editor-survey.json`, `editor-vectors.json`, `editor-measure-1.txt` |
| 4. Inventories + contract rerun (target 0 high) | **high 141 → 54, every T12-owned row gone** (§4). The 54 left: 31 story/benchmark fixture imports, 13 wgpu Shell test-layout rows, 8 sequence carrier-fixture rows, 1 fem3d digest (peers, attributed in §4) and trinity rewriting's window vocabulary (F1, a taxonomy decision). Medium 108: 107 surveyed `justified-reference-gap` + cad's deferral. Earlier step (12:51) was 141 → 126: state-lane fixtures keyed by surface (8 wildcard rows gone), stdio step cc6's stale hand-written subset bridge removed (3 outcome rows), gisterrain terrain config declared (capability without manifest gone), generation2d update-camera fixture re-copied byte-verbatim from its physical vector (2 digest rows), trinity jack's 4 state lanes registered. Remaining 126 split by owner in §4 | `contract` / `inventory` steps in the captures folder, `contract-4-rows-1251.json` |
| 5. remodeling wall-clock laws (5) | **fixed by root cause, 1301/1301 under load 18–21** (§5) | `lib-remodel-1.txt` (before: 2 red), `lib-remodel-serial-1.txt`, `remodel-probe-1.txt`, `lib-remodel-2.txt` |
| 6. raster concurrency flake | **fixed by root cause, 5/5 runs 228/228** (was 227/1): standalone control credits come from an explicit `RasterStandaloneControlPool`; production uses the process pool, each saturation law owns a private pool, and a snapshot root hands its pool to the inner retirement (`wp-t12/raster-pool.py`) | `check-raster-pool-1.txt`, `lib-raster-1.txt`, `lib-raster-2-5.txt` |
| 7. Lib tests of every touched plugin | **re-measured after the sweep** (`remeasure.txt`, `followup.txt` in the captures folder): see §8 for the per-crate table. One red was mine: the 00:41 outcome switch rewrote leaf `outcomeClasses` but left the literal copies in 5 `🔬️structural-correspondence` laws (curation, writer, gisterrain, imperative, space) — settled from the declarations (`wp-t12/structural-outcomes.py`) | `test-curation.txt` (red), `followup-lib-*.txt` |
| 8. Coordinator add-on: per-plugin `test quick` for gis, lowpoly, mathematical, wfc, fem, architect (audit P1-7) | **re-measured 14:3x: 6/6 green** (lowpoly and wfc after W2's descriptor refresh; gis after the fix below). gis was red once, and it was mine: `gismap_viewer_never_mutates` drove the generic `assert_viewer_never_mutates`, whose `expect("viewer adapter command succeeds")` cannot hold for a viewer whose only verb is a retained window-config write (S15's gis camera lane, `handle` refuses by design). The gis surface law now states the guarantee over the seam that decides it, exactly as the energy viewer's does: every verb the map viewer declares is `Migrated` (`followup-quick-gis.txt`, 10/10). Earlier run (03:xx): **gis green** (native codec receipts 2 + hostile 8; nextest 10/10; 758 s, almost all build; the 09-23 document-id fixture drift is gone). **lowpoly red, 2/3: `descriptor_is_fresh`** — the committed `🛂️.descriptor.semio` (09-24 16:32) differs from the live one in 4 bytes: `semantics.effects.destructive` of two actions and one `policy.approval` (peer action-semantics edits since; the 09-23 `ENOENT .tmp-wp-c3` is gone). Regenerating descriptors is W2's describe-all, so it is left to the rebuild. **mathematical green** (nextest 4/4, 623 s; the 09-23 budget timeout was build time). **wfc 27/28: `descriptor_is_fresh`** — the committed descriptor predates G10's landed `InferenceCommitBinding` (the inference `payload` object gained a field: 6 vs 5 members at byte 13 184); same class as lowpoly, refreshed by W2's describe-all. **fem green** (5/5, 545 s), **architect green** (3/3, 355 s). Net: 4/6 green; the 09-23 budget timeouts were build time under fleet load, not tests; lowpoly/wfc wait only on W2's descriptor refresh | `test-quick-<plugin>.txt` |

## Session 12

Slice T12, session 12 (2026-09-25 22:5x). Captures: **`.🧬semio/🌐hub/s12-t12-captures/`**. ABI freeze until W2's
`--packages all`. Hubs 8060–8069, serves 6560–6569 (none started yet).

| # | item | state | evidence |
|---|------|-------|----------|
| 1 | S15-routed guest failures | **trinity**: rewriting `addRuleClause` fixed + 2 laws (refuses 2nd WHERE/unknown/undecodable by name; kind=create publishes only EditRhs), rewriting lib 162/162, request 19. The "no ↶ / undo held" and `clearSelection` wedge were NOT guest defects — S15 found the host-side root (GraphWasmCanvas repainted every frame, ~98 ms/frame on an idle jack editor, so the undo landed ~20 s later) and fixed it; my 5 native round-trip laws stay as regression laws | `rewriting-lib-1.txt`, `jack-undo-1..4.txt`, `jack-clear-1.txt` |
| 1b | **editors that open no kind (coordinator add-on 23:0x + decision 23:3x)** | census 77 editors / 58 with a kind (staged). **Landed (ABI-safe):** wfc bitmap/grid2d/wfc2d/wfc3d + demonstrator playground declare their kind (+10 targets; request 18). **stdio: first landing (23:18) REVERTED at 23:5x** — the hub's `local-stdio-gis-open-v1` fence pins stdio at zero open targets and a stdio with targets would need a closed actor → all of stdio in ONE post-publish patch set (canonical kind ids `s.stdio.<x>` + declarations + fence + census law), in preparation. **trinity rewriting landed 00:5x** (after writer's lane; request 20). gisterrain → patch set (gis fence); allow-list (reasons in the law): space home/studio, playbook-module-procedural. Final census: §S12-3 | `open-target-census-staged-1.txt`, `editor-kinds-laws-1.txt`, `editor-kinds-laws-grid2d.txt` |
| 2 | space index `space_id` guest link (S15 §13) | **verified on the current tree**: fold selects the one space of an index without `space_id`; fix in tree since 17:55 and in restage4 (`space` rebuilt 18:15:24); fold laws **5/5** natively. Live proof = S15/G10 on 7800 | `space-fold-laws.txt` |
| 3 | contract → 0 high (F3, 37 Python oracles, sequence 8 + fem3d, 31 story imports) | **high 54 → 10 (00:0x) → 4 (04:31)**: pdf F3 row gone (inventory refreshed, 60/60), fem3d `__pycache__` removed, sequence off serde_json. **The 4 left need the rename slice** (§S12-4): trinity F1 (1) + `💻️os/🧫️fixtures/⚖️scale` executable owners imported by dev benchmarks/script (3, a root Cargo workspace member move — not during W2's lanes). Medium 104 (103 surveyed oracle gaps + cad deferral). Python oracles: 9 semio cases fixed at the root (host `step_fixture_uris`), drawing reference repaired (codemod damage) + 6 stale case fixtures; Rust adapters of 8 semio cases on the same host helper (parity chain running) | `contract-2.txt`, `oracle-*-1.txt`, `parity-progress.txt` |
| 4 | `test quick` for all 34 plugins + 26 extensions | **measured, 60/60 ran** (chain 00:46–02:21 + aec-building re-run 03:5x): **32 green, 28 red — every red is ONLY `descriptor_is_fresh`** (committed `🛂️.descriptor.semio` older than the tree: session-12 D1 descriptions, T12 kind/arg declarations, peers); 0 other failures (nextest summaries count 338 passed / 25 failed; the other 3 stale packages report through libtest). aec-building's first run hit a shared build-dir race (`extern location … does not exist`), green on re-run (8/8). Fix = W2's describe-all → request 23 | `quick-table-1.md`, `quick/` |
| 1c | **note verb args (G10 relay)** | **landed 01:3x** (after note's lane): 11 verbs declare their args, 4 block verbs refuse by name; census law red(11)→green, + 2 laws; note lib **400/400**; request 22. Cross-plugin heuristic census: 522 candidates in 27 plugins → framework law post-publish | `note-lib-1.txt`, `undeclared-args-census-1.txt` |
| 5 | plugin lib-test debt | wfc bitmap oracle-vector test: filesystem walk of `.🧬semio` (> 17 min) → committed fixture + `include_str!` (root fix); bitmap solve laws red only under load (runtime overrun quarantine, measured at load 34/103); remodel/raster fixed in session 11; F4 (`assert_viewer_never_mutates` vs retained-route viewers) waits for the framework landing window | `wfc-bitmap-lib-1.txt`, `wfc-bitmap-serial-1.txt` |
| 1d | **post-publish patch set** (stdio kind ids + declarations, gis terrain kind, hub fence, census law, bootstrap, rotation) | prepared, **dry run 79 files / 0 problems** (re-verified 04:4x); lands in the landing window after `--packages all` | §S12-1, `wp-t12/postpublish-open-kinds.py` |
| 3b | Python oracle reds (sweep #2: 25, all under `✏️s/🔌️plugins/**` → rule 20) | **fem3d ×7 cases: prepared + validated** (`wp-t12/fem3d-oracles/`: 200/200 rows, committed corpus 118/118 by status). **energy ×2 + procedural io: prepared + validated** (`wp-t12/outline-row-ids/`: platform refuses id-less outline rows; bestest 29/29, io 6/6; bestest's stale 10 °C ground clause replaced by §5.2's raised floor). **generation3d example-geometry: prepared + validated** (8/8). **image: root cause in the platform's Python host, LANDED** (TS host; `.venv` packages were hidden by `--system-site-packages`): image oracle 40/40, platform 117/118 + 1 new law. **energy model: prepared + validated** (1 148/1 148). **en1990: the reference is right** — finding F9 (persisted ids from `DefaultHasher`, 661 committed carriers), main messaged. **kit pack twin: prepared + validated** (writer byte-faithful, twin regenerated in scratch). **brep: prepared + validated** (40/40 in a scratch mirror; Rust-written solid re-encoded byte for byte). Left after the patches: en1990 (F9), 9 declared refusals | `kit-tower-pack.py`, `energy-reference-drift.py`, `fem3d-oracles/check.py`, `outline-row-ids/patch.py`, `example-geometry-paths.py`, `oracle-image-pth-2.txt` |
| 6 | trinity rewriting window vocabulary (F1) | **needs the rename slice** — recorded precisely in §S12-4 | §S12-4 |

### Log

- 22:57 start. Load 33, 11 rustc; W2 holds the wasm mutex (`component-release` of the 9 catalog-B2 packages).
- 22:5x state recovered from disk: the session-11 story move (`story-coordination-merge.py --write`, 17:26) is applied —
  fem, block, remodel `📖️stories/🧭️coordination/🧫️fixtures/*` are merged into `🧭️coordination/🟦️.ts`; tsc/vitest of it
  were not yet recorded. The space fold fix (session-11 request item 16) is in the tree (file mtime 17:55:06) and in
  restage4 (receipt `space` `rebuiltAt` 18:15:24).
- 23:0x space fold laws on the current tree: `cargo test -p semio-s-artifact-space-space --features component-app-assembly --lib -- fold_directory` **5/5** (218 s, `space-fold-laws.txt`).
- 23:0x coordinator add-on: stdio has no hub open target. Measured the one rule over every staged descriptor
  (`wp-t12/open-target-census.py`, a ticket-local replay of `app_opens_kind`/`descriptor_open_targets`; the law stays the
  hub's): **77 editor surfaces, 58 open a kind** (`open-target-census-staged-1.txt`). No kind: stdio 9 (all), wfc 4
  (bitmap, grid2d, wfc2d, wfc3d), gis gisterrain, trinity rewriting, demonstrator playground, space home + studio,
  playbook-module-procedural. Cause for stdio, two classes: (a) txt/tsv/html have no plugin-level kind and their editors
  declared none; (b) csv/json/xml/md ARE plugin-level kinds (the 26 native-codec kinds `stdio.<x>`), and the rule opens a
  plugin-level kind only through a dialect that names it — the dialects are `s.stdio.<x>` (canonical grammar, forced by
  `preflight_artifact_identity`), so an app-level declaration is ignored by design (fixture case
  `plugin-level-kind-no-dialect-names`). The source fix for (b) is the kind id `s.stdio.<x>` (gltf already is
  `s.stdio.gltf`; vcs moved to `s.vcs.vcs` in session 11), which rewrites the native-codec receipts → ABI freeze.
- 23:18 (after stdio's b-warm release ended, nx on gis) landed (a): `.artifact_kind(crate::artifact_kind())` in the txt,
  tsv and html editors; native check of the 3 crates with `component-app-assembly` EXIT 0 (57 s); laws
  `the_editor_declares_the_artifact_kind_it_edits` ×3 + editor tests 13/13, 12/12, 11/11 (83 s). Replaying the rule over
  the staged stdio descriptor with the three declarations: 6 targets (txt/tsv/html × editor + viewer). Request 17 in
  `wp-w1/requests/t12.txt`.
- 23:3x coordinator: csv/json/xml/md → kind id `s.stdio.<x>` as a post-publish patch set (approved; every
  fixture/receipt/manifest in one go); extend to wfc ×4, gisterrain, trinity rewriting, playbook-module-procedural;
  shells go to the census law's allow-list with a reason.
- 23:4x W2 switched to 3 release lanes at 23:16 (gis, note, animate | writer, draw, wfc | puzzle, block); stdio `.ok`.
  Landed before wfc's lane reached it: `.artifact_kind(crate::artifact_kind())` in the wfc bitmap, grid2d, wfc2d, wfc3d
  editors and the demonstrator playground editor (`wp-t12/editor-kind-declarations.py`), one law each. Tests:
  playground 12/12, wfc2d 45/45, wfc3d 56/56, grid2d 56/56, bitmap law ok; bitmap's 3 solver tests were red in the
  parallel run and pass alone (80 s) and its `the_python_oracle_vector_matches_the_rust_payload_shape` ran > 17 min at load
  30+ (killed, my pid 10726) — lib-test debt, not this change.
- 23:5x **stdio landing reverted** (the 6 files equal HEAD 19:34 again). Measured in the hub: `validate_bundle`'s
  `local-stdio-gis-open-v1` fence requires stdio = 26 native codecs + 0 open targets, and `📜️script.ts`'s gate laws
  all publish `stdio,gis`; open targets on stdio would also make the bootstrap build a stdio closed actor (39 MB release
  component; gis's actor was 121 KB under the 64 MiB bound). Not ABI in the freeze's letter, but it would change W2's
  B2 publish path mid-chain → the whole stdio change moves to the post-publish patch set. Request 17 withdrawn, 18 filed.
- 00:0x contract re-run (529 s, cached inventory): **high 10** (from 54): 3 fem3d case `__pycache__` (created 17:16 by my
  session-11 `🔨️run-fem3d-oracle.py` run without `PYTHONDONTWRITEBYTECODE`) — removed; 2 sequence `testing/dependency`
  (production imports `serde_json`, the crate its subset registers as the third-party oracle) — fixed below; 3 os dev
  benchmark/`🧑‍💻dev` script imports of `🧫️fixtures/⚖️scale/*` executable TS; pdf `set-pattern` outcomes (F3, stdio =
  hub-native codec crate → frozen); trinity rewriting window vocabulary (F1). Medium 104 (103 surveyed oracle gaps + cad
  deferral). The 31 story imports are gone (session-11 story move verified: tsc 0 story errors, fem viewport vitest 2/2).
- 00:1x S15 relay trinity: see status row 1 and §S12-2. 00:2x rewriting fix landed while writer's lane was still in the
  stdio crates; rewriting checked at once (EXIT 0, warnings present).
- 00:3x sequence: `wp-t12/sequence-first-party-json.py` — editor, node-graph command, script-window transient, wasm
  component, csv importer read/write JSON through `dsl::os_pack::json`; `serde_json` moved to `[dev-dependencies]`
  (tests + oracle); `From<serde_json::Error>` removed. Native `--lib --tests` EXIT 0, **lib 210/210**. wasm32 check
  (the component is cfg(wasm32)) queued through the mutex, nohup pid 57241, capture `sequence-wasm-check-1.txt`.
- 00:4x G10 relay (note args): census law `every_note_verb_that_reads_arguments_declares_them` written and measured
  **red with 11 offenders** (setGridSpacing, setGridSubdivisions, setGridOpacity, setSnapGridSpacing, setPencilWidth,
  setEraserRadius, setCameraZoom, moveBlock, deleteBlock, duplicateBlock, patchBlocks); codemod
  `wp-t12/note-verb-args.py` (declarations + by-name refusals) dry-run clean; applied once note's release lane is done
  (note is compiling in W2's lane 1). Two more laws written (declared-args decode, refusals).
- 00:4x chain launched (note last); 01:1x coordinator rule 18: every cargo/nx build under `nice -n 15` — reniced my
  chain (63376), the oracle sweep (84816) and the queued wasm check (57241) incl. children (macOS `renice -n` is
  relative: they sit at nice 20); the chain script now prefixes `nice -n 15` for every later package.
- 00:5x S15 relay 2 (trinity `clearSelection` wedges the lane): NOT reproduced — new jack law
  `clear_selection_on_an_empty_selection_settles_and_frees_the_lane` green (`jack-clear-1.txt`); S15's own
  `s15-lane-wedge-jack-trace2.json` shows the clear settling and the next verb starting; asked for the held verb's trace.
- 01:0x `test quick` measured so far: it runs ONLY `descriptor_is_fresh` at quick level (5–13 others filtered). Every
  red so far is a stale committed descriptor (D1's session-12 descriptions, my kind declarations), which W2's
  describe-all regenerates; gis and vcs are fresh and green. Table: `wp-t12/test-quick-table.py`.
- 01:1x Python oracle sweep relaunched (`test-oracle --implementation python`, exhaustive, `PYTHONDONTWRITEBYTECODE=1`
  so no `__pycache__` lands in case dirs; nohup pid 84816, capture `oracle-python-all.txt`). Session 11's 17:4x fixes
  (jack 25/25 + parity 25/25, block5d 83/83, lowpoly 35/35, procedural 2d/3d 29/29, block3d 69/75) were never written
  into this report; the sweep re-measures all of them.
- 01:2x census of verbs that READ arguments but declare none (heuristic over staged descriptors + `command_from_action`
  arms, `wp-t12/undeclared-args-census.py`, `undeclared-args-census-1.txt`): 522 candidates in 27 plugins (space 60,
  puzzle 84, wfc 38, procedural 34, block 33, lowpoly 31, …). Heuristic overcounts (note: 25 candidates vs 11 by the
  real law), so the real law belongs in `artifact_app_laws` (framework crate → post-publish), called by every app.
- Found on the way (plugin contracts, recorded for owners): reasoning `addRelationship` hard-coded `node-1 → node-2`
  (a peer's uncommitted working-tree edit at 05:2x already takes `sourceId`/`targetId` or two selected nodes and maps
  identities by `nodeId` — left to that peer); 42 commands are
  `BatchOnlyPendingRewrite` = unreachable in the running app (space studio 24, architect 8 incl. runAnalysis/Report/
  Validation/search/import/export, cad 5, home 4, animate 1); architect `runAnalysis` is a View action that emits an
  artifact mutation; the wires editor builds interaction args with `serde_json` in production (`✏️editor/🦀️.rs:108`).
- 01:3x note's lane done → `note-verb-args.py --write`; `nice -n 15 cargo check -p semio-s-artifact-note-note --lib --tests` EXIT 0 (132 s); `cargo test --lib` **400/400** incl. the census and the two block-verb laws (376 s). Request 22. Coordinator: trinity clearSelection item dropped (host-side root found and fixed by S15).
- 02:21 test-quick chain ALL_DONE (all three detached jobs ended on their own: chain, oracle sweep 62 min, sequence
  wasm32 check **EXIT 0**). 03:40 resumed after the usage-limit cut.
- 03:5x `quick-table-1.md` (60 packages): 32 green; 28 `descriptor_is_fresh`-only reds (writer, mathematical, wfc,
  procedural, flow, animate, shooting, demonstrator, sequence, fem, architect, process, lowpoly, reasoning, norm, playbook
  + its procedural module, imperative, remodel, energy, trinity, dag, stdio, note, puzzle, block, space, sourcing). At
  `quick` level the plugin crates run only the freshness law (others filtered). Request 23: describe-all.
- 03:4x F7 fixed: one blocking predicate `isBlockingBreach` (`📚️library/🟦️.ts`, used by `runPolicyExit`), and the
  platform law `every committed case satisfies the frozen contract's blocking rules` filters by it (it demanded zero rows
  of every priority, so 103 surveyed medium rows kept it red); `clean safety` timeout 5 s → 60 s (walks the repo).
  The sequence move off serde_json surfaced a real registry inconsistency: the `serde-json-sequence-carrier-reader`
  entry claimed `productionReachable: false` while `🔒️dependencies.json` classifies serde_json production-reachable in
  211 manifests → recorded as shrink-only `productionDebt` (owner `🎯️action-bus`, whose `optional_json_to_dsl` takes a
  `serde_json::Value`; plan in the entry). Platform suite **116/117**; the one red is the contract law's 2 remaining
  blocking rows (trinity F1 = rename slice, pdf F3 — see next line). Bun 1.3.14 segfaults (exit 133) when `bun test`
  gets an emoji path WITHOUT `./` (it treats it as a filter); with `./` it runs.
- 03:5x pdf inventory refreshed (`--artifact s.stdio.pdf`, 10 min): 1.7/base 60 runtime / 60 declared, 0 differences
  (set-pattern's refusal is in the tree since 18:14); the contract row reads the per-kind dispatch outcomes, re-run next.
- 03:5x oracle sweep (`oracle-python-all.txt`): 506 cases, 7 090 rows, passed 6 202, failed 601, errored 287, not
  exercised 177. Only 90 result directories survive in the cache, so the per-case table covers 83 cases
  (`oracle-sweep-1.md`): python 24 run / 11 red — all `🧿️semio` subsets + writer's declared refusal; rust 58 / 1 (txt).
  Root cause of 9 semio reds: each reference re-parsed its steps for ONE hard-coded scheme (`asset://`/`local://`)
  while the features moved their vectors to `shared://`. Fix (`wp-t12/semio-step-fixture-uris.py`): the Python host
  `Context.step_fixture_uris()` answers every fixture URI a scenario's steps name through the platform's one grammar
  (Python twin of `FIXTURE_URI_RE`), the nine references use it (their own scans deleted). Re-run: kit 30/46 → 45/46,
  brep 26/40 → 36/40, graph 23/34 → **34/34**, object 19/28 → **28/28**, table 18/26 → **26/26**, text 15/22 →
  **22/22**, model 23/34 → **34/34**, value → **29/29**, flow 27/40 → **40/40** (captures `oracle-<case>-1.txt`).
  Left (owners' reference maintenance, real disagreements): kit `identity-round-trip` (the pack twin decodes to a
  different kit than the text — reference pack decoder drift), brep `spec-vector-create-{vertex,edge,face}` + its
  identity round trip (`exactly 7 body lines, found 9` — the brep document gained two members the reference rejects);
  drawing (`KeyError 'group'` — the reference's node-kind table lacks `group`), image (Pillow not installed in the
  host Python), writer (declared refusal).
- 04:0x lib-test debt, wfc bitmap: `the_python_oracle_vector_matches_the_rust_payload_shape` walked the WHOLE
  `.🧬semio` tree (hub data roots, cargo build dir, caches) looking for a ticket file — the > 17 min "hang" of 23:5x.
  Root fix: the vector is committed as the fill tool's fixture
  `✏️editor/🎭️modes/✏️edit/🛠️tools/🌡fill/🧫️fixtures/🐍️python-fill-oracle/🔣️.json` (byte copy of the WFC ticket's
  `bitmap-fill-oracle-vector.json`, generated by that ticket's `📜️bitmap-fill-oracle.py`) and `include_str!`-ed.
  Bitmap lib 206/214 at load 34: the other 7 reds are solve laws ending in `job-session.terminal-fault` — the runtime's
  `StepOverrunLedger` quarantines after 4 consecutive over-ceiling WALL steps; serial at load 103 still 3 red, alone at
  load ~20 (23:5x) green. Environment-bound (10× oversubscription), not a code defect; recorded, not "fixed".
- 04:1x drawing reference (`🖊️mutate-semio-drawing/🐍️.py`) carried a rename codemod's damage: node kind `group` read
  `group-nodes` (the mutation id) and the transform key `scale` read `scale-node` → `KeyError 'group'` on every
  committed document; and its verb table used the pre-09-05 variant names (`Rotate`, `Scale`, `Group`, `Ungroup`,
  `Flatten`, `Unflatten`; the Rust enum is `RotateNode` … `UnflattenNode`). Repaired (`wp-t12/semio-drawing-node-kind.py`
  + verb table). The case's own 6 mutation fixtures (`🧫️fixtures/🖊️mutate-semio-drawing/*/🦠️mutation/🔣️.json`) still
  carried the old variant names — the RUST subject refused them too (`unknown variant Rotate`, measured in
  `parity-🖊️mutate-semio-drawing-1.txt`, 23/52 parity) → renamed to the enum's names (no digest registered on them).
  drawing, image, presentation `step_uris` now filter the host's `step_fixture_uris()`.
- 04:2x Rust side of the same drift: the Rust runner `Context::step_fixture_uris()` + `fixture_uris_in` (the grammar of
  `FIXTURE_URI_RE`), and the kit/object/graph/brep/table/text/drawing/image Rust adapters use it instead of their
  `asset://` scans (`wp-t12/semio-step-fixture-uris-rust.py`); presentation passed its oracle 46/46 unchanged.
  Parity chain for the 8 cases detached (pid 58646, `parity-progress.txt`).
- 04:31 **contract: high 4** (`contract-2.txt`, 5 m 53 s): trinity F1 + the 3 `⚖️scale` fixture imports. pdf F3's row is
  gone after the pdf inventory refresh. Medium 104.
- 05:07 semio parity chain (Rust subject + Python oracle, `parity-progress.txt`): graph **68/68, parity 34/34**; text
  **44/44, parity 22/22**; drawing Rust 51/52 (was 23/52 parity: the 6 renamed fixtures + host URIs), Python 52/52;
  image Rust 39/40, Python 25/40 (14 need Pillow in the host Python, 1 `spec-vector-no-mutation` has no registration in
  either adapter); kit Rust 37/46, Python 45/46; object Rust 22/28, Python 28/28; brep Rust 13/40, Python 36/40; table
  Python 26/26, Rust host does not compile (the adapter imports `semio_s_artifact_stdio_csv`, which the generated host
  never links: not the subject, not a contributed host package — pre-existing, since 09-24).
  Real subject failures left for the stdio owner (stdio = hub-native codec crate → post-publish): drawing
  `inverse-unflatten-node` does not restore the drawing; kit create-object/model/properties and object
  create-brep/mesh/properties refused `mutation.child-identity` ("Semio child identity or dialect mismatch"); brep 27
  rows `vertex: expected 3 fields, got 2` (vector shape vs Rust reader); kit pack twin ≠ text (Python reference).
- 05:1x full Python oracle sweep relaunched with its per-row results copied at the end (`oracle-sweep-run.sh`, pid
  66722; the cache keeps only the latest report, so the 01:1x sweep's per-case rows were overwritten).
- 05:3x jack full lib **220/220** (incl. the 5 session-12 round-trip laws; `jack-lib-1.txt`).
- 05:2x image adapters (Rust + Python) register the feature's plain scenario `spec-vector-no-mutation` (both handlers
  already implement the doc-string variant); verification waits (rule 20).
- 05:5x **coordinator rule 20: HARD guest freeze** until W2's `--packages all` — no edits under `✏️s/🔌️plugins/**`,
  guest-linked framework crates, root Cargo. My test-only edits inside stdio's 🧿️semio subsets (04:0x–05:2x) are
  release-input changes for nx (every plugin linking stdio semio recompiled its closure) — lesson recorded: test files
  inside a plugin crate dir are NOT free during a catalog build. Further guest work only as `wp-t12/` patches.
- 06:09 full Python oracle sweep #2 (`oracle-sweep-2.md`; the 600 MB raw row copy summarised and deleted): 506 cases,
  7 090 rows — passed **6 318** (was 6 202), failed 591, errored **181** (was 287). Python 111 cases / **25 red** (session
  11 end: 37), Rust 90 / 6, TypeScript 128 / 2. The 25: 9 declared refusals by design (identity round trips of carriers a
  second implementation cannot read, refused kinds), fem3d scenario-id drift ×5 cases (`hall-vector-*` rows, 51 rows) +
  solids shape drift ×2, energy scenario-id drift ×2 (`case-parameters-*`, `schema-validity-*`), procedural `read-1`,
  cad `example-geometry-3d` (8), image (Pillow absent), brep (vertex vectors), en1990 (applied vs refused), energy
  fenestration (6/1149), kit pack twin. All of them live under `✏️s/🔌️plugins/**` → rule 20 → prepared patches only.
- 08:40 resumed after the second usage cut; rule 20 still active (W2's `--packages all` failed 06:34, re-running).
- Findings for the allow-list decision: space `home` = launcher, snapshot `{schema, catalog_generation}` (no user
  content); space `studio` = the `s` shell's `SpaceApp`, snapshot is the framework-owned OS `WorkflowSnapshot` ("no
  document type of its own", `⚙️engine/🪐️space/🦀️.rs` header); `playbook-module-procedural` = a `playbook.blockKind`
  module whose snapshot is the host block's `ModuleRenderPayload` (foreign document codec). The demonstrator playground
  IS a document (`playground.playground` schema, mutations) → declared, not allow-listed.
- 08:5x **fem3d Python oracles (7 of the 25 sweep-#2 reds) → prepared patch `wp-t12/fem3d-oracles/patch.py`** (rule 20;
  dry run 7 files / 0 problems; refuses a second run). (a) load/material/boundary/analysis/mesh references register
  the `hall-vector` and `reject` outline bases (51 rows answered "no oracle registration"). (b) Registering `reject`
  exposed the real defect: the references applied **29 committed refusals** — they knew only duplicate ids and missing
  selected targets. Each gains `refuse`, run first in `apply_mutation`, written from the vocabulary's statement:
  `id-mismatch` on a renaming replace, `target-missing` on every foreign key (element start/end/material/section,
  solid material, support node, load carrier, combination term), `target-referenced` on every guarded delete (element
  ← member UDLs, solid ← area loads, section ← elements, material ← elements+solids, load case ← combinations;
  `delete-node` cascade-free by its own vector, supports/combinations leaves), `invariant` on the solver's value
  bounds (finite node; e,g,rho > 0 and nu in (-1, 0.5); section area/iy/iz/j > 0; meshable solid incl. zero-area
  outline and holes inside; finite factors/magnitudes; ≥ 1 mode and buckling factor, positive deformation scale).
  (c) `add-load` of a present load id is the committed no-op, not a refusal. (d) mesh + any-mesh features: the 4
  `create-solid`/`replace-solid` rows each predate `axis`, which `FemSolid` requires (no value default) → neither
  implementation could decode them; `"axis":"z"` added, tables re-aligned. **Scratch validation (`check.py`, real
  host `Context`, committed fixtures): 200/200 outline rows of the 6 cases, and the whole committed vector corpus
  agrees — applied 58/58 land on the after-model, rejected 49/49 raise, no-op 11/11 keep the before-model.**
- 09:0x **scenario-id drift (energy ×2, procedural `read-1`) → one root cause, prepared patch
  `wp-t12/outline-row-ids/patch.py`** (dry run 9 files / 0 problems; refuses a second run). A Scenario Outline row's
  id is `<base>-<id cell>`, and the parser silently fell back to the row INDEX when a table had no `id` column. Census
  over all 508 features: exactly these 3 features lack one (7 032 other rows all carry a kebab-case `id`), and all 3
  were red in BOTH roles (Rust and Python register `case-parameters-600`, `read-stl`; the plan said
  `case-parameters-1`, `read-1`). Patch: (A) platform `materializeScenario` refuses a row without a kebab-case `id`
  cell (the index fallback is gone) + platform law; (B) bestest/epJSON tables gain `| id | case |` (`600ff` keys the
  scenario, `600FF` keeps naming the committed model dir), procedural io's `format` column IS its id; (C) both energy
  adapters (Rust + Python) register the kebab id. (D) Registering bestest exposed what the index names hid: all 14
  `case-parameters-*` asserted a 10 °C ground temperature, yet no committed case touches the ground — models and the
  committed EnergyPlus translation expose the floor to outdoor air, NoSun/NoWind, no ground object (18 °C is the
  engine default for an unused boundary; the Rust projection never had the member). The reference now asserts §5.2's
  raised floor and projects exactly the Rust members. **Scratch validation:** patched parser over all 508 features:
  8 057 scenarios, 0 rows without an id; platform "feature profile" 6/6 (+1 new law); oracle rows through the real
  host: bestest 29/29, procedural io 6/6, epJSON 16/16 need the subject phase first (byte-decoding oracle, by design);
  Python registrations == parsed ids for all three cases.
- 09:1x **generation3d `example-geometry-3d-1` (8 rows "unregistered") → prepared patch
  `wp-t12/example-geometry-paths.py`** (dry run 2 files / 0 problems): the reference registers one handler per example
  it DISCOVERS and still globbed the retired `<example>/🧪️tests/🧩️example/🔣️.json`; the fixtures live at
  `<example>/🧫️fixtures/🧩️example/🔣️.json`, which the Rust subject (`include_str!`) and the TS lane already read.
  Scratch: **8/8 rows pass** through the real host; standalone `🐍️.py` "numpy + scipy agree with all 23 committed
  expectations across 8 examples".
- 09:1x **image "No module named 'PIL'" → root cause in the test platform's Python host (TS, not guest) → LANDED.**
  `.venv` HAS Pillow 12.2.0 (and numpy, scipy). But any case whose owner chain declares an external Python package
  (stdio: pypdf, simplejson, ifcopenshell) runs in a cache-local venv created from `.venv/bin/python3` with
  `--system-site-packages` — which exposes the `.venv`'s BASE (uv CPython) and hides every `.venv` package (proved in
  scratch: numpy and PIL both `ModuleNotFoundError`). Fix in `🖥️host/🏗️materialization/🟦️.ts`: new
  `pythonSiteDirectories` (the interpreter's own purelib/platlib via sysconfig); the env's site dir carries
  `semio-base-interpreter.pth` (`site.addsitedir(<base site>)`, so the base's own `.pth` files work too) instead of
  `--system-site-packages`; a stamp without `baseSites` is not current. Second finding while proving it: the `.venv`'s
  `ifcopenshell 0.8.4.post1` is uv's platform-less `py3-none-any` wheel that cannot import on darwin/py3.14 ("not
  built for darwin/64bit/python3.14") — pip picks the real `py314-none-macosx_11_0_arm64` wheel — so a declared
  package the base provides but cannot import is installed INTO the env with `--ignore-installed` (shadows the base).
  README + doc updated, composition fixture declares `pythonSiteDirectories`. **Verified:** new platform law "a
  cache-local Python host environment reuses every distribution its base interpreter provides instead of hiding it"
  1/1 (numpy and PIL resolve from `.venv`'s site dir, not the env); platform suite **117/118** (the 1 = trinity F1,
  unchanged); command-composition-source 7/10 — the 3 reds are peers' and pre-existing (taxonomy `🖥️host` context,
  `Bun` in `⚖️parity/📋️orchestration`, launch-route name); **image Python oracle through the platform: 40/40 passed**
  (`oracle-image-pth-2.txt`; sweep #2 had 27/40 red).
- 09:2x **en1990 (2 rows "applied vs refused") is NOT a reference defect — it exposed a cross-cutting one (finding
  F9, main messaged).** The shared norm Python vocabulary refuses `insert-variable-action` because `qK` is a composed
  child whose `childId` is "content-addressed by a function no specification states". True: `en1990_qk_scene_id`
  hashes the entries' JSON with `std::collections::hash_map::DefaultHasher`, whose algorithm Rust explicitly leaves
  unspecified across releases. Census: ~18 persisted-id minting sites (en1990 qK, din18599 climate, forms scene,
  sequence content, architect benchmarks/knowledge, block/curation catalogs, puzzle5d kind catalogs, puzzle3d
  objects/suggestions, remodeling mesh io, drawing, energy/binary/space inference digests …) and **661 committed
  files** (architect 539, block 82, norm 26, sequence 7, sourcing 5, puzzle 2) carry such ids — a toolchain bump can
  re-key every one, and no second implementation can mint them. Plan (§S12-4 rename-slice class, guest freeze):
  `hash::content_id(prefix, canonical_json)` = `prefix-` + first 16 hex of SHA-256 over the `pack::json` bytes (the
  hash crate already owns `sha256_hex` and `format_number_for_hash`), stated in the composition schema's `childId`;
  every site migrates; the 661 carriers are regenerated in one sweep; a law forbids `DefaultHasher` for persisted ids;
  the norm reference then mints child ids instead of refusing. Until then the en1990 red is the reference being right.
- 09:2x **energy fenestration (5 red of 1 149) → prepared patch `wp-t12/energy-reference-drift.py`** (dry run 3 files /
  0 problems; refuses a second run): the reference fell behind `Material.roughness` (in `create-material`'s payload)
  and `Fenestration.vertices_m` (created empty; `replace-fenestration-vertices` shapes it). Creation writes both;
  undoing a delete re-creates the aperture first and restores a non-empty polygon second (this reference applies
  inverse steps in list order; the Rust store replays reversed). Also drops two dead `if false { return Vec::new(); }`
  blocks in the Rust delete-fenestration / delete-shading-surface inverses. **Scratch: 1 148/1 148 registered rows pass**
  (the 1 unregistered is the subject-only identity round trip).
- 09:3x **kit "pack twin" (1 row, red in BOTH roles — the Rust subject makes the same check) → prepared patch
  `wp-t12/kit-tower-pack.py`**: the capsule tower's binary twin (08-26) predates the 09-20 text edit that made the
  properties child's target id equal its child id (`kit-props` → `props-01`); decoded, the two kits differ in exactly
  that member. The script regenerates the twin from the text through the reference's pack writer only after proving
  the writer byte-faithful (re-encodes the committed twin to the identical 50 019 bytes) and that the new twin decodes
  back to the text's kit; scratch: 50 019 → 50 018 bytes, second run refuses.
- 09:3x rule-20 check on the image case's Rust side (`spec-vector-no-mutation` subject registration): not run — load
  20–27 with W2's cargo fleet; the Python side is proven (40/40 through the platform, including those rows).
- 09:4x **brep (4 rows; the case is red in BOTH roles) → prepared patch `wp-t12/brep-reference-carrier.py`** (dry run 4
  files / 0 problems; refuses a second run). The brep v1 carrier grew per-item `tol`, a `coedges` line (p-curve
  `~curve2`/`-`, prange, loop ring) and `nextLabel` (committed grammar 09-24: 9 body lines, was 7); the Rust codec,
  schema, spec vectors and the Rust-written `✉️base` solid moved, but (1) the Python reference still read 7 lines and
  dropped `tol` in create verbs (3 spec vectors + identity red), (2) the feature's 9 create doc strings carry no `tol`,
  which the Rust payloads require (the Rust subject cannot decode them), (3) the concrete-forest pair was written by the
  reference in the OLD layout (the Rust codec cannot read it). The patch teaches the reference the grammar's new
  productions + the pack twin's matching fields, adds `"tol":1e-7` to the 9 doc strings (tables re-aligned), and
  re-emits the forest pair (tol 1e-7 on every vertex/edge/face, no coedges, nextLabel 0). **Proven in a scratch mirror
  of the subset: the patched reference re-encodes the Rust-written solid pair byte for byte (the cross-language pin),
  13/13 committed vectors agree by status, all 26 feature rows apply (+ the 13 inverse rows restore) on the re-emitted
  forest, and the whole case runs 40/40 through the real host.** Rust side (subject decode of the new forest and doc
  strings) to confirm with `test-parity --case 🧊️mutate-semio-brep` after landing (load 20–27 now).
  Remaining sweep-#2 Python reds after all prepared patches: en1990 (F9) and the 9 declared refusals (by design).
- 09:5x **Finding F10 — 7 cases have never run under the platform (sweep-#2 `problems`, not rows).** Census over all 508
  features' adapters: Python 120 files / **7 without `def adapter()`** (wfc: `mutate-wfc2d-1`, `mutate-wfc3d-1`,
  `mutate-bitmap-1`, `mutate-grid2d-1`, `mutate-wfc-grid3d-1`, `mount-contract`; surface: `web-mercator-tile-oracle`),
  Rust 389 / **5 without `pub fn adapter()`** (the same wfc cases minus wfc3d), TypeScript 154 / 2 (`wfc/mount-contract`,
  and `os/dev/activation`, to check). They are standalone replay scripts / "host-runner shape" stubs from the extraction
  tickets (`python3 🐍️.py` works; the platform host raises "must define `def adapter() -> Adapter`"), so their features'
  differential rows never execute in either role — and the contract does not notice (the 4 high rows are unrelated).
  Root fix, for the wfc owner + the landing window (guest-crate test files, rule 20): wire each case (Rust subject
  replays the committed quintet through the production mutation path and projects `(diff, messages, after)`; the
  Python `adapter()` registers `mutate`/`inverse` over the existing `mutate`/`inverse`/`apply_diff`), and add a platform
  contract rule "an adapter file defines its language's entry point" (high) in the SAME landing, so the gate goes
  red→green together instead of blocking W2's chain mid-publish. The other sweep-#2 `problems` (11 cases whose
  registered oracle is a TS third-party reader — jszip bcf ×3/docx, three gltf ×6/obj — "needs a typescript adapter")
  are the same class for the TS side.
- 10:4x **Regression check of the landed Python-host change over every case it touches** (26 Python cases run in a
  cache-local env: 21 stdio, 3 energy, 2 puzzle2d): `test-oracle --owner 🗄️stdio` (all 126 stdio cases, 70 min under
  load 34–47): executed 2 154, **passed 2 131, failed 23**, 16 not-exercised (11 = F10's missing TS adapters, 5 =
  recorded no-oracle decisions). The 23: Python brep 4 + kit 1 (both prepared above) and Rust-hosted oracles 18 —
  dxf r12 header 2 / entities 4 / tables 10 (owner item, diagnosed: the rows name targets the real input
  `asset://🚏️bus-shelter/🖊️.dxf` does not contain — layer `DIMS`, style `NOTES`, linetype `DASHED`, entity index 5 —
  so `remove-*` moves nothing and `set-*` finds nothing; the rows must name what the bus shelter carries) and
  txt utf-8 2 → **prepared patch `wp-t12/txt-oracle-refusal.py`**: the feature documents those two rows as a refusal
  (the fixture's last line is empty) and the reference states it as an `Err` naming the loss (asserted by its own unit
  laws), but the case adapter's ORACLE handlers propagated that `Err` as a failed row while the SUBJECT handlers already
  answer the documented refusal with the untouched document; one helper now admits exactly that kind + reason in both
  oracle handlers (anything else still fails). Rust test-host file → compile with the case's parity run at landing. Every cache-local case that sweep #2 had red for a hidden package
  is green now: image 40/40, ifc-4 15/15, ifc-2x3 8/8, i-json 22/22, pdf text 2/2 (ifcopenshell installed into the
  env past the `.venv`'s platform-less wheel). Energy mutate 1 143 + the 5 prepared, puzzle2d 124/124, third-party
  puzzle2d 6/6 (`oracle-stdio-pth-1.txt`, `oracle-pth-*.txt`).
- 10:5x **State: every T12 item that does not need W2's `--packages all` is done.** Waiting on the coordinator's DONE for
  the landing window, in this order (each script refuses a second run; `cargo check -p`/parity after each): (1)
  `postpublish-open-kinds.py`; (2) `outline-row-ids/patch.py` (platform + 3 features + 4 adapters together); (3)
  `fem3d-oracles/patch.py`, `example-geometry-paths.py`, `energy-reference-drift.py`, `kit-tower-pack.py`,
  `brep-reference-carrier.py`, `txt-oracle-refusal.py`; (4) re-run the 28 stale-descriptor `test quick`s and the
  Python oracle sweep. F9 (DefaultHasher ids) and F10 (unwired cases) are messaged to main for owners.

### S12-1. Post-publish patch set: every editor that edits a document opens a kind (`wp-t12/postpublish-open-kinds.py`)

**Dry run 00:0x: 79 files, 0 problems** (`python3 .tmp-ticket/wp-t12/postpublish-open-kinds.py --dry-run`). Not applied
(ABI freeze: part A rewrites the stdio native-codec receipts the hub links). Parts:

| part | what | hunks |
|---|---|---:|
| A | stdio kind ids `stdio.<x>` → `s.stdio.<x>` (the id the dialects, artifact definitions and codec ids already use; gltf's precedent): 35 `artifact_kind()` ids, 25 artifact-definition `native_factory.artifact_kind`, 25 receipts in `📜️native-codec-factories.json` + the receipt schema's kind enum, hub provider fixture + bin-unit viewer, TS trusted-stdio-catalog code/test/fixture/schema. Schemas, format kinds (`*_stdio_kinds`), codec/factory/language ids unchanged | 118 |
| B | txt/tsv/html editors declare their kind + law each (csv/json×2/xml×2/md then pair through their dialect) | 6 |
| C | gis terrain: `semio_s_artifact_gis_gisterrain::artifact_kind()` (`s.gis.gisterrain`, `gis.terrain` — its existing native codec) declared at plugin level + activation | 3 |
| D | hub fence `local-stdio-gis-open-v1` keeps the exact 2-package / 28-codec closure and requires the profile to open every package target; the hard-coded "gis map only, stdio none" target list is dropped — every target is validated against its verified descriptor by the one rule at load (`validate_descriptor_open_target`, trusted-catalog `:977`) and grants by role (`:1506`). Fence law renamed + terrain case (outside the profile → refused; inside → admitted) | 3 |
| E | census law `every_committed_editor_that_edits_a_document_opens_a_kind_through_the_one_rule` over every committed `🛂️.descriptor.semio` (isolated packages), allow-list with reasons in the law: space home (launcher, snapshot = catalog generation), space studio (the `s` shell's studio over the OS-owned `WorkflowSnapshot`), playbook-module-procedural (a `playbook.blockKind` module over the host block's payload) | 1 |
| F | bootstrap package table: stdio `opensDocuments: true` | 1 |
| G | `materializeTrustedStdioGisRotation` demanded exactly ONE profile target — already stale (the profile opens map editor + viewer since session 11); now mirrors the fence | 1 |

Landing order (landing window after `--packages all`): `--write` → `cargo check -p semio-s-plugin-stdio --features
full-app-catalog --lib --tests`, `-p semio-s-plugin-gis`, `-p semio-hub --lib --tests --features native-artifact-execution`
(+ wasm32 check of stdio and gis through the mutex) → stdio registry tests, stdio `native_openable_provider`, hub
`trusted_catalog` + `native_openable_provider` + bin-unit, TS trusted-stdio-catalog → W2 describe-all + generate (plugin
`🔣️.json`, `🛂️.descriptor.semio`, `trusted-stdio-catalog.json` regenerate) → census law (needs the fresh descriptors) →
release stdio, gis. Expected census after it: every editor opens a kind except the 3 allow-listed shells.
Trinity rewriting (`text.rewriting`) lands separately once writer's release is `.ok` (writer links trinity).

### S12-2. Trinity "no ↶, undo does nothing" (S15 session 12) — open

- Measured in the browser by S15 (`wp-s15/generated/s15-matrix-r4en.json`): jack `patchNodes` (after `selectAll`) and
  rewriting `addRuleClause kind=create` move Check in 0→1, their History row has no `framework.history.entry.N.revert`
  (↶) and the rail undo leaves Check in at 1 (`attempts … :-/-`, both lanes fail).
- In `build_history_view` (`🔌️plugin/🦀️.rs` ~25385) a row with op lines and `applied` is revertible exactly when
  `edit.actor == store.local_actor_id()` (or no actor); so in the browser the edit's actor differs from the document
  store's local actor. Nothing in the trinity guests sets an actor.
- NOT reproduced natively (all green, `jack-undo-1..4.txt`, `rewriting-lib-1.txt`): one and two selected nodes; the
  shell's own order `selectAll → patchNodes(empty nodeIds) → clearSelection → undo → redo`; the same after the host
  LOADS the curated Nakagin example through envelope ingress (swapped store); rewriting `addRuleClause create → undo →
  redo`. All with one actor (`local`) — the native harness cannot produce two actors.
- Browser reproduction by T12 failed: my copy of S15's probe (`wp-t12/t12-matrix.mjs`, captures `browser/`) against
  S15's serve 6540 opens trinity and loses its windows within 3.5 s, three runs, load 40–65.
- Asked S15 (via coordinator, 00:5x): after `patchNodes` and BEFORE the neutral dispatch, does the ↶ exist and is
  `action.undo` enabled; then one undo with no neutral dispatch.
- Landed on the way (real defects, laws): rewriting `addRuleClause` refuses by name (2nd WHERE / unknown kind /
  undecodable rule) and writes only the side it changed (no re-printed LHS in a create).

### S12-3. Open-target census (editors vs editors-with-kind)

| stage | editors | open a kind | no kind |
|---|---:|---:|---|
| staged restage4 descriptors (measured 23:0x) | 77 | 58 | stdio 9, wfc 4, gis gisterrain, trinity rewriting, demonstrator playground, space home + studio, playbook-module-procedural |
| + landed declarations (wfc ×4, playground, rewriting), after W2's describe | 77 | 64 | stdio 9, gisterrain, 3 shells |
| + post-publish patch set (§S12-1) | 77 | 74 | the 3 allow-listed shells (reasons in the law) |

Rows 2–3 are the rule replayed over the declarations (`open-target-census.py` logic), not yet measured on fresh
descriptors; the census law in the patch set measures it after W2's describe-all.

### S12-4. What the rename slice must do (T12 cannot land these under the freeze / without a taxonomy move)

1. **F1 trinity rewriting window vocabulary** (`contract` high row): `RewritingWindowConfig` lives at
   `♻️rewriting/…/✳️any/✏️editor/🪟️window/🎚️config` and is SHARED by the four graph windows of mode `✏️edit`
   (`👈️lhs`, `➡️rhs`, `⬅️before`, `⏭️after`). The surface grammar `<✏️editor|👁️viewer>[/🎭️modes/<mode>[/🪟️windows/<window>]]/<lane>`
   has no position for a mode-shared window config, so moving it under one window would be wrong. Decision needed:
   extend the grammar with `✏️editor/🎭️modes/<mode>/🪟️windows/🎚️config` (a config shared by the mode's windows), move
   the directory there, then update `✏️editor/🦀️.rs` (the `window_config` module path + the bounded-proof owner path
   string `…/🪟️window/🎚️config/🧵️job/🦀️.rs`), the two leaf `🔣️.json` `owner` fields (`🎥️set-camera`, `🔍️set-lod-mode`),
   the repo `📜️script.ts` reference, and register the vocabulary as a state lane like the 35 of session 11
   (`wp-t12/editor-register.py` pattern: surface manifest + decision + vectors + one case).
2. **`💻️os/🧫️fixtures/⚖️scale`** (3 contract high rows): executable owners (`📽️projection/🟦️.ts`,
   `📤️publication/🟦️.ts`, the `semio_framework_os_scale_fixture` crate + component) live under `🧫️fixtures` and are
   imported by the os dev benchmarks and the `🧑‍💻dev` script. Move the generator package to a non-fixture owner
   (the `🏭️generator` position, e.g. `💻️os/🏭️generator/⚖️scale`), keep only its generated output under `🧫️fixtures`;
   touches the root Cargo workspace member path, `🔣️taxonomy.json`, the library + dev `📋️project.json`,
   `.vscode/launch.json` (7076) and `🧫️fixtures/🧑‍💻os-dev-composition-ownership`. Not during W2's chain (workspace edit).
3. **pdf `remove-pattern`** (F3 rest): the reader lifts a shading pattern's inline shading into its own `<id>Shading`
   item, so removing the pattern leaves that shading behind and a round trip writes one extra `/Shading`. Reader/lift
   decision in a hub-native codec crate (frozen) → pdf owner, post-publish. (`set-pattern`'s refusal of a dangling
   shading/state landed in session 11 at 18:14 with 3 laws; the contract row still shows `[applied]` because the
   cached stdio inventory predates it — refresh the stdio inventory.)
4. **F9 persisted ids minted by `DefaultHasher`** (log 09:2x): one specified `hash::content_id(prefix, canonical
   json)` (SHA-256, first 16 hex), stated in the composition schema; migrate the ~18 minting sites; regenerate the 661
   committed carriers in one sweep (ticket codemod, not a repo migration); law: no persisted id from `DefaultHasher`;
   the norm Python vocabulary then mints `childId` instead of refusing (turns en1990's 2 rows green).
5. **Prepared T12 patches for the landing window** (each refuses a second run): `postpublish-open-kinds.py` (79 files),
   `fem3d-oracles/patch.py` (7), `outline-row-ids/patch.py` (9; includes a platform parser change that must land
   together with its 3 features), `example-geometry-paths.py` (2), `energy-reference-drift.py` (3; 2 are Rust dead-code
   deletions — `cargo check -p` the energy crate after applying), `kit-tower-pack.py` (1 regenerated binary fixture), `brep-reference-carrier.py` (4: reference, feature, forest DSL + pack;
   then `test-parity --case 🧊️mutate-semio-brep`).
   `txt-oracle-refusal.py` (1 Rust case adapter; then `test-parity --case 📝️mutate-txt-utf-8`).

## S. S15 guest defects

W2 release order (w2-final.sh): stdio, gis, animate (done .ok), architect (queued), block … trinity vcs wfc writer. Rule: edit a package only before its `START release` or after its `.ok`.

| defect | package (release state at edit) | root cause | fix | law | checks |
|---|---|---|---|---|---|
| sourcing/curation, demonstrator/curation: `curationSetCount` journals a row, no edit | sourcing-curation (sourcing + demonstrator not started) | the manifest declared `delta`/`value` as TEXT args; the rail staged `"1"`, the bridge reads numbers only → `NoOp` | `ActionArgDef::number` for both | `the_rail_stages_numeric_curation_counts_into_one_curated_edit` (schema is Number + numeric staging reaches the command) | native `--lib --tests` EXIT 0; law 3/3 with the arg-bridge laws (`check-curation-1.txt`, `test-curation-1.txt`) |
| procedural/generation2d viewer: guest panic `ordered-map root must be explicitly retired before drop` | procedural-generation2d (procedural not started) | the viewer did not declare `build_document_store_owners`, so the framework's generic bounded owners dropped the snapshot's `OrderedMap` roots plainly; the editor declares the artifact's own owners. Same gap found in 6 more viewers (raster, process3d, wires, writer, jack, rewriting; draw deferred: its catalog-b release is running) | every one of those viewers declares its artifact's owner catalogue (one function each) | `the_viewer_opens_and_closes_its_document_through_the_artifacts_owners`: **red before (the exact guest panic reproduced natively), green after** | native `--lib --tests` 7 crates EXIT 0; viewer-filtered lib tests 5+7+7+7+4+4+5 all pass (`test-gen2d-viewer-before/after.txt`, `check-viewers-1/2.txt`, `test-viewers-1.txt`) |
| norm din18599, en1990: `setSnapshot` refused `expected Enum/Float, found Absent at 1:1` | norm contract + 2 editors (norm not started) | manifest declares `snapshot` = document JSON (like 13 siblings), but the `text` arm of `norm_command_from_action!` passed that JSON to the DSL-text parser | the arm takes the artifact's `decode_*_snapshot_json`, decodes the declared JSON and carries it as the document's own escaped DSL text (payload/wire unchanged); undeclared `text` key dropped; `escape_op_text_field` is production code (`wp-t12/norm-set-snapshot.py`) | `the_declared_snapshot_argument_carries_the_documents_json` ×2 (committed ➡️after fixture through the rail args) | native `--lib --tests` EXIT 0; 7/7 (`check-norm-1.txt`, `test-norm-1.txt`) |
| stdio ×9 (csv, tsv `set-cell`; txt, md, html `replace-text`; json base/i-json, xml base/valid `set-node`): refused `interactive-job.missing-factory` | stdio editors (stdio released .ok before the edit; the editor code compiles only under the stdio plugin's `component-app-assembly`, so no in-flight component build links it) | the window kits mint the three verbs as `Migrated` Mutations and each editor bridged them in `command_from_action`, but no editor registered them as retained routes, so `qualified_tool_proof` found no app/framework registration. The live law then found a second gap: no editor owned an artifact one-item publication authority, so the `Artifact` lane failed `interactive-job.publication-authority-missing` | per editor: `<X>_KIT_ACTION_ID` in the retained roster, an `Artifact`-lane publication contract, the verb in `bounded_first_step_tool_proofs!`, extent 1, the reducer and `handle` share one pure `<x>_emit`, `build_tool_job` binds the command's own id; `build_artifact_store_one_item_preparation_factory`; text kits admit 32 KiB wires (under the 64 KiB guest ceiling); md/html answer an unparseable buffer with `stdio.<x>.invalid-text` instead of an empty emit (`wp-t12/stdio-kit-verbs.py`, `stdio-kit-publication.py`, `stdio-kit-laws.py`) | `the_kit_verb_edits_the_document_through_its_exact_retained_factory` ×9: registered app, example loaded as the host applies `LoadDocument`, verb dispatched with rail-staged text args, settled through the host publication loop, read back from the snapshot (red first: `publication-authority-missing`); `replace_text_refuses_text_that_is_not_the_artifacts_dsl` (html; md parses any text as markdown) | native `cargo check -p semio-s-plugin-stdio` EXIT 0; lib tests of the 7 crates all green (`check-stdio-kit-1.txt`, `test-stdio-kit-1/2.txt`, `test-csv-kit-1.txt`) |
| vcs viewer paints an empty history tree (0 text) | vcs-vcs viewer (vcs in W2's rest batch, not started; no other component links it) | diagnosed: the viewer's only window rendered the checkpoint forest alone, and a document without checkpoints (every freshly opened viewer) is an empty roster, for which `TreeWindowKit` paints no row at all — not a projection gap | the document itself is the tree's root row (`$`, its title, the language-neutral `—` when untitled) with the checkpoint forest beneath | `a_document_without_checkpoints_still_paints_its_own_row` (row + projection text) and the updated nesting law | vcs artifact lib 126/126, `cargo check -p semio-s-plugin-vcs` EXIT 0 (`test-vcs-viewer-1.txt`) |
| gis viewer drops `setCamera` (`no window kind declares it`) | gis-gismap viewer (gis released .ok before the edit; no batch-b component links it) | the read-only map window declared no actions, but the `TiledMapHost` dispatches `setCamera` after every pan of ANY map window; the viewer had no config lane to hold a camera (`NoConfig`) | the energy viewer's pattern: the map window declares `setCamera` (Migrated view action, `camera` arg); a viewer-owned window config `gis.mapviewerwindowcfg` (`GisMapViewerWindowConfig { camera: {x,y,zoom} }`, schema-first `🔣️.json` + `set-camera` leaf) retained per window; one retained route on the WindowConfig lane only (ViewEmit untouched, `handle` refuses loudly); render publishes the retained camera or the host's fit-to-world default. **Freeze note:** additive local-only window-config record, no artifact pack/codec/ABI change | `a_dispatched_pan_is_retained_by_its_window_and_rendered_back` (registered viewer, host-shaped args, settle, render reads it back), `the_map_window_declares_the_hosts_camera_verb`, the factory join law, refusal/codec/inverse/no-op laws | `cargo check -p semio-s-plugin-gis` EXIT 0; gismap lib 285/285 (`test-gis-viewer-1/2.txt`, `test-gis-lib-1.txt`) |
| trinity jack + rewriting: `patchNodes` (live nodeIds) moves nothing; `textSelect` refused `missing start`; `nodeGraphEdit` refused `missing operationsJson` from the rail | trinity-jack, trinity-rewriting (trinity not started; writer, which embeds both, had finished its batch-b build) | `patch_nodes` answered an empty emit whenever no requested id named a live node (S15's harvested id was not a jack node id; a comma list parsed as ONE id), so the press journalled nothing and said nothing; `textSelect`/`textEdit` are the text host's gesture verbs and `nodeGraphEdit` the node-graph host's (each needs a caret range, a buffer or a `surfaceId` + `operations`), yet all three were `in_palette` rail rows — `textEdit` pressed there would even have emptied the query | `patchNodes`: empty `nodeIds` = the `ast`/`graph` selection, comma/space lists split, every non-applicable request refused by name (`mutation.target-missing`, `app.command.invalid-args`, graph-validation detail), `nodeIds` optional in the form; the three gesture verbs `in_palette: false` (still declared for their hosts), `nodeGraphEdit` out of the context menu's transform group | `patch_nodes_from_the_rail_renames_the_selection_or_the_listed_nodes` / `…patches_the_selection_or_the_listed_nodes` (registered app, real `interactionSelect`, rail-shaped args, settle, read back), the two refusal laws, `the_text_gesture_verbs_are_kept_off_the_rail`, `the_node_graph_gesture_verb_is_kept_off_the_rail_and_the_transform_group` | jack lib 216/216, rewriting lib 160/160, `cargo check -p semio-s-plugin-trinity -p semio-s-plugin-writer` EXIT 0 (`test-trinity-jack-1.txt`, `test-trinity-rewriting-1.txt`) |
| draw viewer owners (deferred until draw's `.ok`) | draw-drawing viewer (draw `.ok` 08:18, W2 idle) | the viewer used the framework's generic bounded owners instead of the drawing's own catalogue (the gap that panicked the generation2d viewer) | declares `crate::spr::drawing_document_store_owners()` like the editor | `the_viewer_opens_and_closes_its_document_through_the_artifacts_owners` — **green with and without the fix** on the default drawing (red-check `test-draw-viewer-before.txt`), so this one is preventive, not a reproduced fault | draw viewer tests 5/5 (`test-draw-viewer-1.txt`) |

## 1. T10 outcome switch

- `wp-t12/switch.py` is `wp-t10/switch.py` with its capture path redirected to `wp-t12/generated/` (T10's evidence stays intact). The dry run on the current tree: **2,544 files, 0 problems** (T10 measured 2,545 on 09-24; one leaf fewer on disk). Applied unchanged at 00:41.
- Residue scan before applying (`wp-t12/residue.py`, `residue-0.txt`): no leaf descriptor outside the T10 table speaks the severity vocabulary; every Rust site naming `MutationOutcomeClass::{Info,Warning,Error,Fatal}` is in the switch's reach (36 bridges, replication, derive, spr, 14 hand descriptors).
- `wp-t12/manifest-align.py --write` (T10's with the capture redirected): **1,898 rows aligned, 682 already agreed**; re-run afterwards: 2,580 same. The 1 unresolved row was binary `splice` (fixed in §2).
- TS: `tsc` on the test platform failed once (`outcomeClasses: [...outcomes]` was a `Set<string>`). Fix: `isMutationOutcomeClass` type guard beside `MUTATION_OUTCOME_CLASSES`; the scaffolder's outcome set is `Set<MutationOutcomeClass>`; `outcomeClassesOf` and the vector reader use the guard. `tsc-switch-2.txt`: 0 errors.
- Compile gate:
  - native `cargo check --lib`, ONE invocation, 134 `-p`: replication, dsl-derive, os-kernel, os-config, every artifact crate holding a changed leaf JSON (97, mapped by `wp-t12/crates-of.py`), 36 plugin crates. **EXIT 0 in 6 m 59 s**, 520 warning lines (`check-switch-1.txt`).
  - wasm32-wasip2 through the fleet mutex: replication, plugin stdio, gis, cad. **EXIT 0 in 3 m 32 s** (`check-switch-wasm-1.txt`).
  - `--tests` for replication + dsl-derive + os-kernel (spr unit test, derive fixture): first run hit a peer race (H9's `pending_is_empty` landed in replication mid-compile, then H9's plugin-crate edit briefly broke `semio-framework-plugin`); I stopped my loop (pids 71069/74971, mine) and reran after H9 compiled: **green** (`check-switch-bridges-2.txt`).
  - 37 standalone bridge workspaces (each its own `[workspace]`): 5 green before I stopped the sweep; every bridge is regenerated (§3) and rebuilt by the inventory.

## 2. T11 frozen follow-ups

**Binary: one name.** The leaf was `replace-byte-range` (the derive requires `semanticKind == to_kebab(variant)` and a verb-entity kind), but the wire tag (`#[value(rename = "splice")]`), DSL keyword, grammars, catalog, manifest row, fixture directory, feature, adapter and oracle said `splice`. Now everything says `replace-byte-range` / wire `replaceByteRange` / `ReplaceByteRange`; `splice` survives only for the diff's byte operation (`ByteSplice`, `BinaryDiff.splices`, `Vec::splice`). `wp-t12/binary-one-name.py`, 14 changes:
- aggregate: rename attribute and its excuse removed; `KINDS`;
- leaf: keyword `replace-byte-range`, label `("replace-byte-range", "Bytebereich ersetzen")`;
- text grammar/EBNF/ANTLR, TS union, proto enum (`REPLACE_BYTE_RANGE`);
- vocabulary unit test: the retired `splice` keyword must now fail to parse;
- catalog kinds, manifest row (`id`/`operation` `replace-byte-range`, outcomes `applied, rejected` from the leaf table), fixture `replace-byte-range-applied`, fixture dir `✂️splice` → `✂️replace-byte-range`;
- feature rows/ids (`zero-length-replacement`, …, `@id-invalid-replace-byte-range`);
- adapter: kind arms, the subject's import of the non-existent `splice` module (host compile debt) → `replace_byte_range`, and registration by Scenario Outline base id (`vector`, `invalid-replace-byte-range`) instead of hand-kept id lists;
- oracle: kind arm, helper `replace_range`, unit test names.

**dwg ac1018 duplicate leaves** (`wp-t12/dwg-ac1018-leaves.py`): AC1018's vocabulary is `pub use` of AC1024's. Deleted its `📸️set-snapshot` and `🏷️set-version-info` leaf directories and the crate-root mounts of the duplicated set-snapshot `diff`/`inverse`/`mutation` helpers (no caller). The one AC1018-specific fixture case (auxiliary save counter) moved to the vocabulary-level `🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs` (include paths re-rooted). `schema generate` rerun: the two dead scopes are gone (it also picked up two peer scopes, `framework.trace-record`, `s.stdio.json.rfc8259.geojson`).

**jpg/tiff baseline subject hosts** (`wp-t12/leaf-fields-pub.py`): leaf payload fields are the schema's public contract; the 20 `pub(crate)` fields in 15 jpg/tiff baseline leaves (exactly T11's E0451 list) are `pub`.

**Compile gate:** native `--lib --tests` stdio binary/dwg/jpg/tiff: EXIT 0 (91 warnings); stdio test-oracle crate (standalone) `--features oracles --lib --tests`: EXIT 0; wasm32-wasip2 plugin stdio + the four artifacts: EXIT 0 (2 m 25 s).

## 3. Editor/viewer state-lane vocabularies

**What exists.** T11 found 39 real editor/viewer vocabularies (config, presence, transient lanes; `wp-t12/generated/editor-survey.json`) that production dispatches through the plugin's lane stores and that no catalog registers (`unregistered-mutation-vocabulary`). T11 landed the schema half (`MutationManifest.surface`, `surfaceProblem`, surface-keyed inventory cache).

**Design (schema-first, one authority per fact):**
- **Manifest** at the surface owner (`<subset>/<surface>/🔮️oracles/🔣️.json`) with `surface: "<surface>"`; catalog with the subset's profile dirs; a surveyed no-oracle decision per lane (generic state containers — electron-store/confy, y-protocols/liveblocks, zustand/im — cannot express the typed per-kind change, no-op guard or inverse) → `justified-reference-gap` (medium) instead of a high row; one `fixture/v2` manifest per vector.
- **Vectors** as quintets under `<surface>/🧫️fixtures/<leaf>/<status-emoji><kind>-<status>/` (outside `🧫️fixtures/🧬️mutations`, so the v1 leaf-local vector registry does not claim them). 33 converted from the owners' committed `🔁️mutation-contracts.json`/`🔁️mutations.json`, 66 hand-authored (`wp-t12/editor-vectors-authored.py`); the production diff and diagnostics are measured by a ticket-local harness (`wp-t12/vector-harness`, `editor-measure.py`) and recorded only when the produced snapshot equals the hand-written after, the inverse restores the before and the diagnostics match the declared class.
- **One case per vocabulary** at the hosting subset (`<subset>/🧪️tests/<lane-emoji>mutate-<catalog>`; a case under the surface would be `case-above-subset`), registered by Scenario Outline base id (`mutate`, `inverse`, `keep` for no-op vectors).
- **Production bridge** per vocabulary, one line: `<aggregate>_report_json` → `store::os_store::test_support::mutation_report_json::<S, M>` (new kernel helper, the generic form of the gis/equation/energy hand copies). The case host links only the artifact crate and the stdio test-oracle crate, so the laws live in `semio_s_plugin_stdio_test_oracle::law::vector` (new, unit-tested).
- **Bridges** (`wp-t12/bridges.py`, derived from `wp-t5/bridges.py`): a 6-column `COORDINATES` with the surface, a 5th `list-mutations` argument, the surface echoed in the inventory, editor/viewer aggregates listed, and the document answer excluding every leaf below a `✏️editor`/`👁️viewer` segment. A dry generation into the scratchpad kept every existing aggregate row; the only dropped coordinates are the ones T10/T11 deleted on purpose (gisterrain window config, the stdio sibling subsets).
- **Platform:** `wildcard-subset-owner` / `unsplit-artifact-subset` no longer apply to state-lane manifests (their mutations change an editor surface, not a document subset); tsc 0 errors.

**Landed (11:0x).** The gis viewer's new camera lane joined the set (survey index 39, two vectors measured 88/88 with 0
findings, no-oracle decision authored like the energy viewer's). `editor-register.py` wrote **589 files for 35
vocabularies** (committed quintets, one `🔮️oracles/🔣️.json` per surface with catalog + state-lane manifest + decision +
fixture manifests, one exhaustive case per vocabulary, one `<aggregate>_report_json` bridge per aggregate);
`cargo check` of the 20 owning crates `--lib --tests` with the gating features: EXIT 0. Trinity's 5 stay unregistered
(no authored vectors: private module paths and the `🪟️window` taxonomy they sit in). `manifests.ts` now sees 200
manifests, 35 of them state lanes. `bridges.py` learned that a state-lane aggregate below a
`#[cfg(feature = "component-app-assembly")]` mount needs that feature on the bridge's dependency (gis, procedural,
fem, block, wfc bridges); all 36 bridges regenerated.

### 3a. Found by the vector harness: whole-record lanes wiped on a no-op

`ArchitectPresence`, `ArchitectConfig` and `ImperativeConfig` are whole-record lanes (the diff type IS the record, so
`apply` replaces the record). Their five leaves answered an unchanged value with `MutationOutcome::empty()` plus a warned
`mutation.no-op`; the store applies every op's diff (`replay_mutations`), so the default diff replaced the record with
its defaults — re-applying the value a collaborator already had erased their presence/search/run output. Measured: the
no-op vectors produced `{activeRegister: "elements", graphCamera…0/0/1}` instead of the unchanged record (`editor-measure-1.txt`).
Fix (`wp-t12/whole-record-noop.py`): the leaves return `MutationOutcome::new(base.clone())` with the same warning.
`cargo check -p semio-s-artifact-architect-program -p semio-s-artifact-imperative-procedure --lib --tests`: EXIT 0.
Guest rebuild requested in `wp-w1/requests/t12.txt`. The generation3d config vectors were wrong on my side (the preview
camera is `{position,target,fov}`, not six scalars) and were corrected, not the code.

## 4. Contract: 141 → 54 high, and who owns the 54

Measured by `test-contract` (captures `contract.txt` 14:30, `followup-contract.txt`; rows `contract-rows.json`,
`followup-contract-rows.json`). What removed the 87 rows T12 owned, each by fixing the evidence or the declaration,
never by exempting a row:

| rows | fix |
|---:|---|
| 44 `runtime-only-mutation` pdf 1.7 base | §6 |
| 12 state-lane wildcard/outcome/capability rows | §3 (surface-keyed fixtures, cc6 stale bridge removed, gisterrain terrain config declared) |
| 9 `mutation-without-fixture` os.config ui-preferences | §7a: catalog `os-config-ui-preferences-1-any` over the 18 committed per-kind quintets + the claiming host case |
| 7 `fixture-generator-unregistered` jpg | Pillow generator ledger (`wp-t12/jpg-pillow-generator.py`) |
| 5 `mutation-without-fixture` cad object kinds | §7d: the fixture rule now honours the catalog's own `deferredKinds` (the debt stays visible once, as cad's medium `mutation-kinds-deferred`); F2 |
| 3 energy/remodel coverage rows | `mutationCoverageBreaches` counts a row named by a registered vector scenario id for that vector's kind |
| 3 layout `rotate-frame` | §7c |
| 2 generation2d digests | fixtures re-copied byte-verbatim from the physical vector |
| 1 remodel `mutation-kind-undeclared` (136 rows) | §7b |
| 1 trinity jack vocabulary | its 4 state lanes registered |

**The 54 that remain are not T12's:**

| rows | owner | what it is |
|---:|---|---|
| 31 `production-fixture-dependency` | remodel stories (12), fem stories (6), block stories (5), ui stories/elements (3), os dev benchmarks + `🧑‍💻dev` script (3), infinite + IconRenderHost stories (2) | `📖️stories/…/🧪️.story.tsx` and dev benchmarks import `🧫️fixtures` data; the rule wants it under `📚️examples` or out of fixtures — each owner's move |
| 13 wgpu Shell test layout | WG7/WG8 (`🐚️Shell/🧪️tests/📂️wgpu-document-relay`, `🎯️targets/🧊️wgpu/🦀️.rs:29097`) | 11 inline test bodies, 1 non-canonical `#[path]`, 1 case name |
| 8 sequence carrier fixtures | sequence owner | generator oracle `serde-json-sequence-carrier-reader` and profile `semantic-sequence-carrier-v1` referenced but not registered |
| 1 fem3d digest | fem owner | `expected.results.json` rehashed after a benchmark change |
| 1 trinity rewriting window vocabulary | trinity owner (F1) | needs a taxonomy move before it can be registered |

## 5. Remodeling wall-clock laws

**Measured first.** The ticket-local probe (`wp-t12/remodel-probe`, production `BoundedStillDecoder`, debug build, serial)
times every PNG worker step: 512×512 uniform 519 steps, worst 2.1 ms; 4096×64 max-row 581 steps, worst 0.74 ms — the
T11 "PNG laws fail serially" no longer holds (the decoder is now the framework's `PngScanlineDecoder` at 4 096 inflate
units per step). The full lib run at load 9–12 failed 2 other laws on ONE step each (texture bake step 1574, a feature
detect microstep at 8.33 ms); both pass 3/3 run alone at the same load. So the per-step work is bounded and the red rows
were single descheduled readings.

**Root cause: the laws asserted a stricter property than the runtime.** Production's `StepOverrunLedger`
(`⏱️trace`, `SUSTAINED_OVERRUN_QUARANTINE_STEPS` = 4, documented there as the one authority every target can run,
since `wasi:clocks` and browsers have no thread-CPU clock) records every over-ceiling step and quarantines only a run of
four consecutive ones. The 29 worker-step laws asserted every single wall reading < 8 ms. They now admit each measured
step through one test helper that applies exactly that ledger rule (`engine/🧪️tests/🔬️step-ceiling`, codemod
`wp-t12/remodel-step-ceiling.py`): the ceiling stays 8 ms, a step that genuinely costs more fails after four steps, a
descheduled reading is recorded and forgotten. Law for the helper itself:
`a_descheduled_step_is_recorded_and_a_sustained_run_fails`. Trade-off stated plainly: a law whose loop has fewer than
four steps can no longer fail on one slow step — exactly as production would not quarantine it.

**Result:** `cargo test -p semio-s-artifact-remodel-remodeling --lib` 1301/1301 (load 18–21), helper law 1/1. Test-only
change (the helper is `#[cfg(test)]`); no guest rebuild needed.

## 6. pdf 1.7 base: the 44 kinds nobody declared

Ticket 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE widened `PdfMutation` from 16 to 60 kinds (page boxes and user unit,
content operators and annotations by index, fonts/images/forms/graphics states/shadings/patterns/colour spaces/property
lists, embedded files, outlines, named destinations, page labels, output intents, AcroForm, optional content, viewer
settings, metadata, document id, encryption, catalog extras). Production dispatch offered all 60; the manifest owned
16, and the 44 had one test each (`semantic_identity_is_owned_by_this_leaf`).

- **Declared** (schema-first): 44 manifest rows, outcomes and variant read from each leaf's own `🔣️.json`.
- **Independent evidence**: the lopdf engine (`🏭️generator/🔁️codec`, depends on `lopdf` and nothing else) got an
  arrangement and a COS edit per kind (resources on the first page, catalog entries and name trees, trailer `/ID`,
  RC4-128 encryption) and a projection that renders every page dictionary, the catalog, the trailer `/ID` and whether the
  file was encrypted. 60/60 pairs observable; the 16 existing pairs regenerate byte-identically; two consecutive runs
  diff equal after one fix — lopdf fills the 16 arbitrary `/U` padding bytes (ISO 32000-1 Algorithm 5) at random, the
  engine zeroes them through the COS API. Reader probe re-qualified: (before,before) equal 60/60, (before,after)
  unequal 60/60. 44 fixture manifests from the generator's own `manifests` command.
- **Laws** (`🧬️schema/🧬️mutations/🧪️tests/⚖️lopdf-vectors`, #[cfg(test)]): decode lopdf's before/after through the
  subset's reader, derive the payload from what lopdf changed, apply, **write with the subset's writer, read back**,
  require lopdf's after-document on every typed lane (the retained COS carrier is writer freedom), then apply the
  inverse steps and require the before-document. **42/44 green on the first run**; set-pattern and remove-pattern needed
  two steps because the reader lifts a shading pattern's inline shading into its own collection item (`P9Shading`) —
  the laws now apply SetShading+SetPattern / RemovePattern+RemoveShading; the single-step behaviour is finding F3.
  Final: **44/44, crate lib 502/502** (`pdf-lopdf-vectors-3.txt`, `pdf-lib.txt`).
- Nothing in production changed (no stdio rebuild; `wp-w1/requests/t12.txt` item 13).

## 7. Other rows and defects closed this afternoon

**7a. os.config ui-preferences.** The 18 per-kind quintets existed; no catalog registered them and no case claimed them.
Catalog `os-config-ui-preferences-1-any` (9 kinds × `sets-*`/`keeps-*`), host case `🎨️mutate-os-config-ui-preferences`
(feature, Rust + TS adapters: each `sets-*` row must move exactly its declared member, each `keeps-*` row must raise one
`mutation.no-op` Warning and move nothing, inverses restore the before-record, a round trip reads locale, driver scale and
keybinding off the typed value), taxonomy member name. TS subject 28/28; Rust subject in §8.
**The identity sibling's TS adapter was broken**: it read `🧪️tests/<name>` fixture dirs that do not exist (6/6 errored)
and both adapters still asserted a session `token` (`session-ada-0001`, `user-ada`) the `Identity` record no longer has
(it carries `email`); both fixed against the committed record.

**7b. remodel.** `mutate-remodeling-1` named its rows `<kind>-realworld`, `-noop`, `-missing` …, which the catalog does
not declare (1 high row listing 100 scenarios). Every row now carries the registered scenario id of the vector its
`vector` column runs (`wp-t12/remodel-row-ids.py`: 136 mutate + 136 inverse rows, unambiguous and unique per role; the
vector-less commit-reconstruction rows keep their kind id), and both adapters' lists follow. Python oracle 275/275 at
exhaustive; Rust subject in §8.

**7c. layout `rotate-frame`** (added 09-23 without evidence): quintet from move-frame's committed before-document, leaf
vector test (#[cfg(test)] mount), catalog vector, two case rows, both adapters (`wp-t12/layout-rotate-frame.py`).

**7d. Test platform** (`🧪️test/🟦️.ts`, tsc 0 errors; platform suite 115/117 — the 2 reds are F7):
- `mutationCoverageBreaches`: a row named by a registered vector scenario id covers that vector's kind (exact match
  through `vectors[]`, never a prefix).
- `mutationFixtureBreaches`: a kind the capability's catalog defers is not reported a second time; law
  `a kind the capability's catalog lists in deferredKinds is left to mutation-kinds-deferred, and only that kind`
  (9/9 in its block).

**7e. My own fallout.** The 00:41 outcome switch left 17 literal outcome copies in 5 structural-correspondence laws
(§8); settled from the declarations. My S15 gis camera lane made the plugin's generic viewer law unsatisfiable (§Status 8);
restated as the energy viewer's retained-route law.

## 8. Re-measured tests (after the 12:19 sweep)

`remeasure.sh` (12:47–14:36) and `followup.sh` (15:24–) in `wp-t12/`, captures `<step>.txt` / `followup-<step>.txt`.

| check | result |
|---|---|
| native `cargo check` of the 134 switch crates `--lib --tests` | EXIT 0 |
| native check of the 16 S15 plugin crates | EXIT 0 |
| wasm32-wasip2 check: replication, cad + the 16 S15 plugins (fleet mutex) | EXIT 0 |
| tsc on the test platform + the new host case | 0 errors |
| stdio test-oracle crate `--features oracles`; `law::vector` | EXIT 0; 5/5 |
| stdio kit crates (7) | green |
| curation lib | red (outcome literals, §7e) → **154/154** |
| norm (3 crates), S15 viewers (8 crates, `viewer` filter), gismap, vcs, trinity jack + rewriting, architect + imperative | green |
| raster lib ×5 | 5/5 green |
| remodeling lib; remodel probe | green; EXIT 0 |
| vector harness build; editor measure | EXIT 0; EXIT 0 |
| writer lib; gisterrain lib; space lib | **176/176; 55/55; 51/51** |
| pdf lib; lopdf vector laws | **502/502; 44/44** |
| layout lib | red once (my rotate-frame test's include paths carried move-frame's emoji; the codemod replaced `move-frame` before `🕹️move-frame`) → fixed in file and script → **399/399** incl. the 4 rotate-frame vector laws |
| test quick gis, lowpoly, mathematical, wfc, fem, architect | 6/6 green (gis after §Status 8) |
| case `🎨️mutate-os-config-ui-preferences` (Rust + TS) | **56/56, parity 28/28** |
| case `🎚️mutate-os-config-identity` (Rust + TS) | **12/12, parity 6/6** (TS was 0/6 before §7a) |
| case `📸️mutate-remodeling-1` (Rust + Python) | **550/550, parity 275/275** |
| case `📐️mutate-layout-1` | Rust subject 53/53; Python oracle was 30/53 — 11 kinds read the retired `<leaf>/🧪️tests/<vector>` fixture paths; fixed to the feature's `🧫️fixtures` vectors → **105/106, parity 52/53** (`identity-round-trip` is subject-only by the adapters' own design: the example is `.dsl.semio` and the reference carries no DSL codec) |
| Python oracles with the same rot (`wp-t12/oracle-vector-paths.py` + hand edits) | wires 0/21 → **20/21**, space 0/9 → **8/9**, procedure 0/9 → **8/9** (also the wire's `{owner: null, slot: null}` root path), shooting 0/63 → **62/63** (also a NameError: `_leaf_root` bound `document` and read `fixture`), process3d 1/33 → **32/33**, dag 1/29 → **28/29**, home 1/3 → **2/3**, vcs 1/13 → **12/13** (plus two diff members `VcsDiff` no longer has), gismap 36/37 → **37/37** (example read through its `asset://` URI); each remaining row is the subject-only round trip. writer 6/9 → case **17/18, parity 8/9**: the snapshot's new `text` member, and `edit-text`'s no-op branch is now adjudicated because the body is carried (feature narrative updated); the last row is the placeholder-grammar refusal |
| lopdf pdf engine rebuilt; 60 pairs regenerated | identical to the committed fixtures |
| platform suite (bun) | 115/117 (F7) |

### 8b. Repo-wide oracle sweep

Because three Python references of plugins I had touched turned out 0/N (retired fixture paths), I ran every
case's reference role once (`test-oracle` at exhaustive, 506 cases, 7 090 rows) — **`📓️wp-t12-oracle-sweep.md`**
has the per-case table. After the fixes above: Python 111 cases, 37 with non-passing rows; Rust 90 / 6;
TypeScript 128 / 2. The remainder is owners' work, bucketed there (document shape drift, scenario id drift,
behaviour disagreements, declared refusals, oracle-only runs of byte-decoding references).

## Findings

- **F1 trinity rewriting** — its window vocabulary sits at `✏️editor/🪟️window/🎚️config`, outside the surface grammar
  `<✏️editor|👁️viewer>[/🎭️modes/<mode>[/🪟️windows/<window>]]/<lane>`; registering it needs the owner to move the window
  config under `🎭️modes/<mode>/🪟️windows/<window>/🎚️config` (a production taxonomy move).
- **F2 cad object kinds** — no wire vector can exist while the pane's child materialization is absent from every codec;
  the real fix is a persisted child resolver (`store::LinkResolver`/`ChildStoreFactory`), after the freeze.
- **F3 pdf set-pattern** — accepts a pattern whose shading id names no shading; the writer emits it with an
  unresolvable shading (re-read as `shading: ""`) and the outcome is still `applied` (outcomeClasses `["applied"]`).
  remove-pattern leaves the lifted `<id>Shading` behind, so a round trip writes one extra `/Shading` resource. Proposed:
  refuse (`rejected`) and remove the owned shading — leaf outcome and lift/lower decisions for the pdf owner.
- **F4** `artifact_app_laws::assert_viewer_never_mutates` cannot serve retained-route viewers (energy, gis): a refusal
  emits nothing and should satisfy "never mutates". Not edited now: every guest links that crate and W2 is restaging.
- **F5** the os.config host cases say `🎚️config` "has no crate of its own"; `semio-framework-os-config` exists since
  09-08. Moving all four cases to `🎚️config/🧪️tests` in one pass needs a taxonomy member kind for that owner.
- **F6** (earlier) ~80 stdio editors expose editable-kit verbs without bridges (bcf `set-cell` is a no-op); JSON
  `set-node` always writes strings (S15 staged `"\"S15\""`); unused imports in stdio set-snapshot inverse files; the
  Pillow ledger says 11.3.0 while the generator ran 12.2.0.
- **F7** the platform test `every committed case satisfies the frozen contract` asserts zero rows of every priority, so
  it is red on the 107 surveyed medium rows; `clean safety` hit its 5 s timeout under fleet load.
- **F8 jack's Python reference** still decodes the retired hex-member `trinity.jack.dsl v1` carrier, while the
  committed scene (`🖼️assets/🎬️demo/🗣️.dsl.semio`, 09-24) is the nested record notation: 17 rows error with
  `non-hexadecimal number found in fromhex()`, and `spec-vector-create-node` disagrees (the vector declares a
  refusal, the reference applies). A reader/printer for the record notation is the fix (iso16757/vdi3805 refuse by
  clause for the same notation).
- **F9** the oracle phase is the only place these references run; nothing in the contract notices a reference that
  reads a retired path — 9 cases read retired fixture paths (most of them 0/N). A contract rule that resolves every `shared://`/`asset://`
  URI a reference builds against the case's plan would catch the whole class statically.

## Processes (pids)

- 12:47 `remeasure.sh` (nohup, mine; ALL_DONE 14:36); 15:24 `followup.sh` (nohup, mine; ALL_DONE 16:15); both left no process.
- One overlap against my own rule: at 13:24–13:29 the pdf laws (foreground) ran while the remeasure's queued wasm32 check
  got the fleet mutex — two T12 cargos for ~5 min. No other overlap.
- The 16:0x wait loop was moved to background by the tool timeout; it only polled a file and exited.
- `inventory` in the follow-up exited 1 on ONE bridge (animate): a peer's 15:52 edit of
  `🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs` did not compile at that moment (E0407/E0432); every other inventory
  row matched (0 differences) and the contract used animate's 14:28 inventory. Re-run alone at 16:3x once the
  peer's edit compiled: animate 3 surfaces, 0 differences (`inventory-animate.txt`) — the inventory is clean.

- native switch check 57805/57807 (exited 0)
- wasm switch check (mutex) exited 0
- bridge loop run 1: 71069 + cargo 74971, stopped by me (peer race)
- bridge loop run 2: 80205 + orphan cargo 89516, stopped by me (kernel `--tests` green; bridges superseded by regeneration)
- inventory run 1: 89919/89921 (+ bridge cargo 96931, rustc 99905), stopped by me after writer 4/4 (bridges are regenerated first)
- test-quick chain: 4320 (exited, ALL_DONE)
- vector-harness builds: foreground/detached, exited (build-harness-1 failed on feature-gated editor modules, -2 green)
- 03:1x network outage cut the turn; no orphaned T12 process found at 03:26

## Files changed (T12)

- Switch: 2,544 files listed in `wp-t12/switch-files.txt`; 153 manifests in `wp-t12/manifest-files.txt`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts` (guard).
- Binary: `💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/` — see §2 list.
- dwg: `🖊️dwg/🦀️.rs`, `4️⃣ac1018/…/🧬️mutations/🦀️.rs`, new `…/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs`, deleted `…/📸️set-snapshot/`, `…/🏷️set-version-info/`.
- jpg/tiff: 15 leaf `🦀️.rs` under the two `🧱️baseline` subsets.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` (regenerated).
- S15 (§S) and state lanes (§3): see `wp-w1/requests/t12.txt` items 4–12 and `editor-register.py`'s 589 files.
- pdf (§6), under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/`:
  `🏭️generator/🔁️codec/🦀️.rs`, `🏭️generator/🔁️codec/🏗️generate/🦀️.rs`, `🏭️generator/📜️script.ts`,
  `🔮️oracles/🔣️.json`, `🧬️schema/🧬️mutations/🦀️.rs` (test mount), new `🧬️schema/🧬️mutations/🧪️tests/⚖️lopdf-vectors/🦀️.rs`,
  44 new `🧫️fixtures/<leaf>/{⬅️before,➡️after}.pdf`.
- os.config (§7a): `🧰️framework/🛍️products/💻️os/🎚️config/🔮️oracles/🔣️.json`; new
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/{🥒️.feature,🦀️.rs,🟦️.ts}`;
  `…/🧪️tests/🎚️mutate-os-config-identity/{🥒️.feature,🦀️.rs,🟦️.ts}`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (one member name).
- remodel (§7b): `✏️s/🔌️plugins/📸️remodel/…/✳️any/🧪️tests/📸️mutate-remodeling-1/{🥒️.feature,🦀️.rs,🐍️.py}`.
- layout (§7c): the 11 files listed by `layout-rotate-frame.py`, the rotate-frame test's include paths, `📐️mutate-layout-1/🐍️.py` (oracle paths).
- test platform (§7d): `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts`, `…/🧪️test/🧬️schema/🔣️.json`, `…/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts`.
- structural laws (§7e): `🧪️tests/🔬️structural-correspondence/🦀️.rs` of sourcing-curation, writer, gis-gisterrain, imperative-procedure, space-space.
- gis: `✏️s/🔌️plugins/🌍️gis/🧪️tests/🔬️surface/🦀️.rs`.
- Python oracles: `📡️mutate-wires-1`, `🪐️mutate-s-space-1`, `🛟️mutate-procedure-1`, `🎥️mutate-shooting-1` (`🐍️.py` each).
- Ticket inputs: `wp-t12/{remeasure.sh,followup.sh,catalog-problems.ts,remodel-row-ids.py,structural-outcomes.py,oracle-vector-paths.py,layout-rotate-frame.py,pdf-kinds.json,tsconfig.t12.json}`.
