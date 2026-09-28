# WP-LB2 — stdio csv Redo Arena Budget, Verb-Arg Law, LD Leftovers

Session 14 slice LB2 (continues [📓️wp-lb.md](📓️wp-lb.md) and [📓️wp-ld.md](📓️wp-ld.md)). Coordinator = main chat. Ports 8130–8139 / 6630–6639.
Scripts + prepared patches `wp-lb2/`, captures `wp-lb2/generated/` (expendable), private target `wp-lb2/target`, durable data
`.🧬semio/🌐hub/s14-lb2-*`. Native cargo only via `wp-lb2/cargo-lane.sh` (native lane, build-fleet-b, nice 15); overlay builds via the
overlay lane with a private build-dir. Landing rows: `📓️landing.md` § Session 14.

### Session 14c

Successor agent (2026-09-28 16:5x, after the 14:37 usage cut + app restart). Guest freeze ON (chain relaunched 16:55:46) → non-test
guest code = prepared patches (dry-run clean) for window 3; rule 22 test-only edits may land compile-atomic.

| # | Item | Status |
|---|---|---|
| 0 | Reconcile the docx/xlsx test port (predecessor's last step) | **done 17:01**: no docx/xlsx file changed after 14:36 except the regenerated set-snapshot quintet (generator run of 14:36, `t8-docx-xlsx-2.txt` GENERATE rc=0); the tree is committed by the 16:29 auto-commit → no half-applied hunk. Native `check --lib --tests` docx + xlsx **EXIT 0** 16:59 (`c1-check-docx-xlsx.txt`); lib tests docx 68/5 red, xlsx 49/19 red (`c1-test-docx-xlsx.txt`) = exactly the 14:36 set, all in NON-test code (item 1) |
| 0b | lb-p4 proof: `cargo test -p semio-s-plugin-stdio --test shipped_fleet` (native lane) | **EXIT 0** 17:14 (1 passed, `c2-test-shipped-fleet.txt`) — stdio `plugin()` assembles with the landed structural-row classification |
| 1 | 24 docx/xlsx lib reds after the peer's OPC `xml_parts` migration (coordinator counted 25) | **test half LANDED 17:34** (rule 22, 7 test files, landing row): 13 of the 24 were test code — xlsx Strict/Transitional analyzer/construction/composition tests built their packages in the BINARY lane (workbook.xml via `opc.set_part`, no root relationship) or relied on the old "encode regenerates Transitional XML" scope cut; docx `canonical_xml_authority…` pStyle red was an assertion bug (the edit is applied; the serializer wraps attributes, `contains("w:pStyle w:val=…")` missed it → quick-xml oracle now). **11 non-test reds → prepared p5** `lb2-p5-docx-xlsx-opc-reds.py` (12 files, dry run 0 problems; patched files rustfmt-clean with the repo config) + `lb2-p5-land.sh` (window 3: apply → check, auto-revert on red → both fixture writers → lib tests): xlsx demo `xl/styles.xml` → XML lane (fixed point proven); 4 grammar + 2 protocol facets rewritten to the real print forms (JSON-hex `set-snapshot`/cell `address`, `R[..]` error cells, JSON diff text `{}`, `OP_BINARY_FORMAT`+UTF-8 JSON frame); xlsx worksheet content-type check by ROLE (`XlsxSnapshot::worksheet_part_paths`, new `REL_TYPE_WORKSHEET_STRICT`) — the peer's `content_type.contains("worksheet")` filter could never see the violation; Strict VML ban over both lanes + new law; one `workbook_part_path`; dead imports/consts; docx assets regenerate through `zzz_write_native_docx_fixture` (demo already a fixed point, only relationships differ). All payload facets proven by temporary `lb2_probe` tests (removed): docx ops 18/18, xlsx ops 9/9, typed JSON diff grammars 4/4 each, utf8 diff protocols 4/4 each, worksheet-role prototype flags the retyped sheet and not the clean one (`c3-probe-1.txt`) |
| 2 | SDK table-kit half of the Home row-action fix (lands with WG11's wgpu TableRow painter) | **prepared, re-diffed 17:3x**: `lb2-p3-row-actions.py` dry run 5 files / 0 problems (`table_row_action` already always sets `label`; the law asserts it). WG11's painter `wp-wg11/wg11-table-row-painter-patch.py` is disjoint (10 files + 2 new, none of p3's). Joint window-3 runbook = `lb2-p3-wg11-joint-land.sh` (ONE native hold: p3 `--write` + WG11 `--apply` → check SDK + ui `wgpu-engine` `--lib --tests`, both halves revert on red → SDK table laws → WG11 ui laws → renderer-react typecheck + Interpreter vitest; rule-20 Home boot after) |
| 3 | ui table conformance fixture pair | **done 17:47**: Rust corpus 6/6 (14:33); the remaining TS red was a harness divergence (TS: deduped sorted set; Rust: ordered multiset) → TS aligned to the Rust definition + table TS test reads absent `children`; Interpreter vitest 144/144, typecheck 0 (landing row) |
| 4 | csv details-panel admission fix (p1) + verb-arg census law (p2 + p2b) | **window-3 list, re-diffed 17:3x**: p1 7 files / 0 problems, p2 1 file 3 hunks / 0, p2b 38 files / 0 |
| 5 | NEW (coordinator 17:5x, S18 served matrix) csv/tsv undo refused `stdio.table.cell-arguments: cell argument map capacity`, redo `ui.snapshot-details.arguments …` | redo = p1 (unchanged cause). Undo = same arena class in the TABLE path: the three stdio table argument builders reported arena refusals under their own codes (the SDK row window only ends early on `ui.fixed-capacity`), and `render_structural_table` built add-row/add-column AFTER the windowed body, so a drained arena took the controls (= the render) down. **Prepared p7** `lb2-p7-table-arena.py` (7 files, dry run 0; write/revert round trip exact in scratch): `table_arena(stage)` = `ui.fixed-capacity`, body passed as a closure and admitted after the controls (csv + tsv callers), law `🧪️tests/📊️table-arena-headroom` + fixture/schema (python-jsonschema valid; serde_json RFC 6901 oracle on every cell address; controls / third / half / complete headroom, en + de). Independent of p1 |
| 6 | NEW json/i-json `set-node` refused `snapshot-edit.schema-unregistered 's.stdio.json'` | **root-caused natively** (`c10-probe-1.txt`): after `semio_s_plugin_stdio::plugin()` NONE of `s.stdio.{json,xml,csv,tsv,txt,md,html}` is in the OS-wide schema catalog — `PluginBuilder::artifact(…)` (stdio, trinity, remodel, dag, writer, forms, mathematical, puzzle, fem, draw, playbook) keeps schemas/inferences/languages in `PluginRuntimeRegistry` marked "captured but unwired" and never publishes them; only `declare_artifact` does. Every stdio editor's snapshot-edit route (json `set-node`, every details-panel edit) resolves its schema there. **Prepared p6** `lb2-p6-declared-catalogs.py` (SDK `publish_declared_catalogs` after `commit_artifact_declarations` — schema + inference descriptors, same semantics; languages stay unwired on purpose: 51 single `register_language` overwrite sites + fn-address comparison would risk false conflicts at every describe; describe OUTPUTS unchanged, `ContributionSet` carries no schema catalogs; law in `shipped_fleet`: every shipped editor's document schema registered after `plugin()` + a revision-bound json `set-node` through the shipped editor lands, serde_json oracle — **witnessed failing on the current tree** `c12-p6-law-witness.txt`: `s.stdio.html … must publish`). Chain note: a conflicting intra-plugin descriptor in any `.artifact(…)` plugin would now fail its describe. **Test half LANDED 18:16**: json editor unit tests register the schema their fixture needs + 3 stale asserts → json 137/137 (was 7 red) |
| 7 | NEW xml file-source draft "every apply refused (DSL parser on XML text)" | **not reproducible on current source** (`c10-probe-1.txt`, `c13-xml-catalog-probe-2.txt`): `parse_dsl` falls back to XML import without a semio preamble; the curated catalog's rendered source applies (unchanged → no-op, text change → 1 mutation). The real current-source defect: the demo asset `🗣️.dsl.semio`/`🎒️.pack.semio` is the old 5-field wire → `xml_any_example_snapshot` silently opens an EMPTY document (`unwrap_or_default`), i.e. the served editor had no text nodes and an empty draft. **Landed 18:31** (test-only): law `the_natural_source_draft_applies_what_it_shows` (quick-xml oracle) + 2 stale xml tests; the 4 remaining xml reds = the asset → regenerated in window 3 by `lb2-w3-stdio-sets.sh` through `zzz_write_dsl_and_pack_fixtures`. Follow-up (not mine, 63 sites): every `PRIMARY_TEXT).unwrap_or_default()` example loader is a silent fallback |
| 8 | NEW duplicate DOM ids (tree `edit`, table `cell-N`) | the SDK keys are contract-correct (the ui contract: a key is "unique only among this node's own siblings (not surface-wide)"); the HOST's `uiNodeDomId = surface/key` assumes surface-wide keys. Keys stay (p3/WG11 `cell-<col>` contract untouched). **LANDED 19:01** p8 `lb2-p8-row-scoped-dom-ids.py` via `lb2-p8-land.sh` (typecheck 0, Interpreter vitest 146/146, Home boot 12.6 s / 0 pageerrors on serve 6630, stopped; `c16-p8-land.txt`): `UiInterpreterContext.domScope`, `uiNodeDomId(…, scope?)`, every `nodeDomId` call forwards the scope, tree rows hand their DOM id to their inline controls, table rows to their cells; laws in `🧪️tests/🪟️tree-windows` + `🧪️tests/📊️table` (Testing Library DOM oracle). Landed after S18's de run finished 18:59 (coordinator relay) |

#### Log 14c

- 16:58 start. Load 49 (chain), 8 rustc, 84 GiB free, native lane empty (2 slots).
- 17:0x reconcile done (item 0); shipped_fleet queued detached (pid 79643) → EXIT 0 17:14.
- 17:1x item 1 analysis: of the coordinator's list, the Strict/Transitional analyzers already READ `xml_part` — their tests built
  the workbook in the binary lane (test code); the composer tests encoded `snapshot.opc` directly (drops every XML part). Real
  non-test defects: xlsx demo styles part in the binary lane, stale grammar/protocol facets (both artifacts), the worksheet
  content-type filter (regression: it keys on the content type it is supposed to validate). Found in passing (not touched, report
  only): the peer's migration left ~312 dead-code items in docx/xlsx `🔺️diff` (old semantic diff engine + old text/binary codecs,
  185 docx / 127 xlsx `never used` warnings, `c1-check-docx-xlsx.txt`) → separate cleanup set for window 3+ (owner: peer/ST2).
- 17:2x temporary `lb2_probe` tests (test-only, env-gated `LB2_PROBE_DIR`) in both base unit files → all candidates proven;
  probes removed before the landing run.
- 17:34 item 1 test half landed (24 → 11 red, `c4-land-tests-1.txt`); p5 prepared + `lb2-p5-land.sh`.
- 17:3x coordinator relay (WG11 painter ready, one-step landing) → joint runbook `lb2-p3-wg11-joint-land.sh` written; p1/p2/p2b/p3 dry runs clean.
- 17:4x item 3: Interpreter vitest (`test long`) showed 2 reds on the edited table pair (TS action-id set semantics; plain row without
  `children`) → TS test-only fixes, 144/144 + typecheck 0. Found in passing: a stray empty directory literally named `$PWD` (27th 21:50,
  not mine) in `📺️renderer/…/⚛️react/📦️packages/🟦️typescript/` — reported, not touched.
- 17:5x coordinator: 4 new stdio items from S18's served matrix (rows 5–8 above). 18:0x native reproduction runs (`c7`, `c9`, `c10`
  probes; temporary `lb2_probe_*` tests, all removed — `git diff` of those files empty afterwards). 18:16 json test half landed; 18:24 p6 law
  witnessed failing; 18:31 xml test half landed; p6/p7/p8 prepared; window-3 runbook `lb2-w3-stdio-sets.sh` (xml regen → p6 → p7, each
  reverting on its own red) + `lb2-p5-land.sh` + `lb2-p3-wg11-joint-land.sh`; p8 = host TS lands with tsc + vitest + boot.
- 18:4x coordinator: p6 FIRST in window 3 with p1/p7 + xml regen → `lb2-w3-stdio-sets.sh` = xml regen → p6 → p1 → p7, each
  `--write` backed up and `--revert`ed on its own red (revert round trip verified exact on a scratch copy); all six sets (p6, p1, p7,
  p3, p5, p8) applied in that order on one scratch copy with 0 problems and every patched Rust file parsing. `.artifact(…)` census for
  the describe watch (failure mode = `plugin-assembly.declaration-schema|inference`): trinity, remodel, raster, flow, process, norm, cad,
  dag, stdio, sequence, procedural, vcs, imperative, sourcing, note, forms, architect, shooting, mathematical, layout, puzzle, fem, draw,
  playbook, lowpoly, energy (+ gis, space, writer, block, demonstrator sub-builders). p8 lands after S18's de matrix (`lb2-p8-land.sh`:
  apply → typecheck + Interpreter vitest → Home boot on serve 6630 → stop; revert on red). S18: its pins select by key/label, unaffected.
- 19:01 p8 LANDED (landing row): typecheck 0, Interpreter 146/146, Home boot 12.6 s / 0 pageerrors, serve 6630 stopped, backup dropped.

### Session 14b

Successor agent (2026-09-28 12:0x, after the usage cut + app restart). Guest freeze ON since 12:02:46 → every guest-linked item is a
prepared patch (dry-run clean on the live tree) + an overlay proof; landing in window 3, compile-atomic.

| # | Item | Status |
|---|---|---|
| 0 | Live tree carries no half-applied LB2 guest edit | **verified 12:1x**: p1/p2/p2b `--dry-run` report no "already applied" hunk; `/usr/bin/grep` for every LB2 marker (`take_arena_unbuilt_rows`, `ArenaStarvedSurfaces`, `details_arena_headroom`, `command_bridge`, `UNDECLARED_ARGUMENT_VOCABULARY`, …) over `🧰️framework` + `✏️s`: 0 hits; the 5 payload files do not exist |
| 1 | stdio csv redo `snapshot details UI admission failed` (p1) | patch re-anchored on the overnight tree (SDK export block reformatted), dry run **7 files / 0 problems**; overlay synced + applied 12:28; law + details unit tests queued (overlay lane, hold A) |
| 2 | stdio html/json/md/txt/xml "no snapshot schema is registered" (5 MCP mutations) | root cause gone from the tree: the Codex peer's `5bcb2da23da` resolves the edit schema from the editor's DOCUMENT schema (`s.stdio.<kind>`, registered by every one of the 5) instead of `{kind}.{standard}.{base\|any}` (never registered); native proof queued (`editor_catalog` laws of the text family, native lane) |
| 3 | Verb-arg law (p2 + p2b) | p2 re-anchored (the live bridge law now also skips `CANCEL_TYPED_OPERATION_ACTION_ID` → one `framework_owned_verb` predicate shared by both laws), dry runs 0 problems; overlay census queued (hold B) |
| 4 | LD leftover: conflict probe de 4/5 | probe criterion = committed DOMAIN ops on a census socket (C13's `HubProbeDocument.relayedEnvelopes`); the WAL reader is superseded; live run after the chain (item 6) |
| 5 | lb-p1 / lb-p4 still green | check default `--lib --tests` stdio plugin + semio artifact **EXIT 0** 13:43 (`i5-check-default-2.txt`); **`shipped_fleet` RED 14:25** (`i5-test-shipped-2.txt`): stdio `plugin()` panics — the peer's `structural_table_window_kind` rows (add/remove row/column, set-header) are `Unclassified` → csv/tsv (shipped) + wav fail assembly → CHAIN RISK (descriptors). Fix prepared **p4** `lb2-p4-structural-classification.py` (stamp `Migrated`; dry run 0 problems), overlay proof queued, escalated to the coordinator for W4 / priority. semio brep lib + `editor_catalog` + brep parity still queued |
| 6 | Item-3 live run orchestration | `live/run-conflict.sh up\|probe\|down` complete (own vigilant hub 8130 + link proxy 8131/8132 + serves 6630/6631); run scheduled after the chain publish (keeps the machine idle for the critical path) |
| 8 | docx/xlsx lib tests after the peer's OPC + `xml_parts` snapshot move (coordinator 13:38, rule 22: test-only, may land now) | **edited in the tree (test-only)**: docx — quick-xml 0.42 names are `&str` (7 sites), `🧹️clear-main-declaration` include path one level too deep, `encode_docx` via `crate::engine`; xlsx — mutations unit tests (projection via `project_workbook`, cell edits through lineage-bound `xlsx_cell_address`, absorb law re-expressed on XML-part deltas, field sweep restored over BOTH lanes incl. binary parts/content types/part-owned relationships in the `#[cfg(test)]` sweeps), schema unit tests (SST/unmodeled parts as XML authority, shrinking via `RemoveSheet`), outline + result-apply tests, set-snapshot quintet test (new XML-part diff assertions + `#[ignore] zzz_write_committed_quintet` generator + drift law); native check → generate → lib tests queued (`t8-docx-xlsx.sh`) |
| 9 | S18 matrix: 6 stdio editors (csv, tsv, json, json/i-json, xml, xml/valid) newly red, edits `[0,0,0,0]` (coordinator 14:1x) | **root-caused, no guest patch**: not S20's initializer (all four families override `build_document_store_initialization_job` with the bounded job; the example loads — `renderAfterRedo` shows the demo rows; "No data" is `render0`, captured before the pre-verb exactly as in S16's green run). The peer made `set-cell` (stdio `structural_table_window_kind`) and the SDK `set-node` revision-bound overnight — `revision` is DECLARED required (schema-first, consistent; every rendered cell/node binding carries it) — while the matrix pins `stdio.set-cell {row,column,value}` / `set-node {nodeId,value}` carry no revision → `command_from_action` refuses → 0 edits. Fix = S18's harness (resolve the live revision from the rendered binding, or drive the cell input); relayed |
| 10 | ui conformance corpus `🧩️component/📊️table` (peer 00:21 snapshot vs 09-25 expect, coordinator 14:1x, rule 22) | **edited in the tree (fixture-only)** by `lb2-t10-table-corpus.py`: snapshot brought to the agreed TableRow contract (the two `row-action-0` child buttons dropped — `rowActions` props stay the one representation; the editable row keeps its `cell-0`/`cell-1` Commit-bound, revision-guarded inputs), expectation DERIVED from the snapshot alone (nodeCount 5, shape, accessibility, actionIds openSpace + 2× set-cell; no generator existed — the script is it); Rust corpus harness queued (native lane, `t10-ui-conformance-1`) |
| 7 | Home rows carry each row action twice → item capacity 27 < 32-row viewport (S18 via coordinator; SDK half mine, wgpu TableRow painter WG11, land together) | prepared `lb2-p3-row-actions.py` (5 files, dry run 0 problems live + overlay): SDK drops `table_row_action_buttons` (both call sites), React Interpreter drops its `row-action-` key filter, law `home_shaped_rows_carry_actions_as_props_and_fill_the_default_window` + fixture/schema `🪟️window-kits/📊️table/🧫️fixtures/🏠️row-capacity` (python-jsonschema valid); contract agreed with WG11 (unchanged `TableRowProps`); overlay law queued (hold B) |

#### Log 14b

- 12:09 start. Load 37, swap 4.5/6 GiB, 100 GiB free; native lane 5 deep (g12 holding), overlay lane free, wasm = chain.
- 12:1x last night's overlay proofs never compiled: the overlay was cloned 18:44 while the kernel was red (Codex, E0282/E0308 ×74) —
  `o-p1-proof-1`, `o-p2-proof-1`, `o-probe-2` all stop in `semio-framework-os-kernel`. `i4-test-laws-1` (shipped_fleet) **EXIT 0**
  21:04; `i4-test-semio-lib-1` cut at the restart (no EXIT).
- 12:2x overlay synced onto the live tree (`overlay-sync-2.sh`: clone changed files, top-up now also refreshes stale gitignored sources
  such as `🎨️styling/🔤️tokens/🦀️.rs`, NEW `lb2-overlay-prune.py` removes files the live tree no longer has); p1 + p2 + p2b applied
  (0 problems). Found: `📜️fleet-mutex.sh` waiters survive SIGTERM (the TERM trap only removes the ticket, the loop keeps waiting and
  would take a slot once the queue drains) — my two stray waiters were SIGKILLed (pids 63829, 64661, both mine). Coordinator fixed the
  mutex (exit on INT/TERM).
- 13:21 overlay hold A (p1 + p2 + p2b applied, private build-dir, 1 m 51 s): the SDK (incl. the reactor-turn re-dirty) and the stdio
  contract COMPILE; details unit tests **21/21** (`o2-p1-unit.txt`, EXIT 0); p1 law **FAILED on case 4** (`o2-p1-law.txt`): "3 rows under
  40 free collections, 3 complete" — cases 1–3 passed en + de, but the fixture's absolute `freeCollections` were calibrated on last
  night's details costs; the peer's rewrite shows untyped add controls only where `allows_collection_insert` holds (default false).
  Law made self-calibrating: each case measures what its complete render holds (`complete_cost`) and leaves a share free (`none`,
  `third`, `half`, `complete-plus-one-row` — the last must render WHOLE, i.e. no over-reservation); the test provider mirrors the DSL
  provider's `allows_collection_insert`. Fixture v2 validated with python-jsonschema 4.25.1. Re-run queued (hold B).
- 13:1x item 2 root cause (read, not yet proven): at G11's run (7800 B3, built 09-27 13:56) `snapshot_schema_descriptor_for_dialect`
  looked up `{kind}.{standard}.{base|any}` (e.g. `s.stdio.html.5.any`) — never registered; the text kinds register `s.stdio.<kind>`.
  Codex `5bcb2da23da` (27 21:54) resolves from the editor's `DOCUMENT_SCHEMA` (`stdio.html` → `s.stdio.html`, owner-checked).
  The stdio `editor_catalog` gate carries a peer `println!("[DEBUG] editor=…")` (not mine; rule 19: nobody edits that gate now).
- 13:3x item 7 (coordinator): the duplicate exists because the wgpu reconcile never painted `TableRowProps.row_actions` (TableRow with
  children → bare horizontal Stack, cells dropped; childless → one Button, no actions). Split agreed: SDK half = p3 (mine), wgpu
  TableRow painter = WG11 (Table → retained Tree section, TableRow → tree item with column cells + trailing actions); contract deltas
  relayed (editable rows keep exactly one child per materialised cell `cell-<col>` / draft / read-only surface; `table_row_action`
  always sets `label`). Also: the live SDK test `table_kit_render_rows_builds_one_table_node_with_one_record_per_row` asserts
  `row.children.is_empty()` — red on the live tree since the duplicate landed; p3 makes it true again.

## Session 14

| # | Item | Status |
|---|---|---|
| 1 | stdio csv redo `snapshot details UI admission failed` (details panel vs `UiValue` arena / `ui_value_headroom`) | prepared `wp-lb2/lb2-p1-arena-budget.py` (7 files incl. the reactor follow-up render; dry run 0 problems on tree + overlay); overlay probe + law run queued (overlay lane; load 86–97) |
| 2 | Verb-arg law (LB item 4): sound bridge-law census + per-app argument declarations | prepared: SDK law + type-erased `command_bridge` (`lb2-p2-declared-arguments.py`, 3 hunks) + one call per plugin (`lb2-p2-plugin-calls.py`, 38 files / 31 plugins); dry runs 0 problems; overlay census (note, block, puzzle, space, wfc) queued |
| 3 | LD leftovers: conflict probe de 4/5 criterion (winner's check-in transition), other open LD items | probe criterion rewritten (`wp-lb2/live/probe-s12-conflict.mjs`: committed DOMAIN ops from the hub WAL, transitions excluded); reader `wp-lb2/wal-ops` build queued (overlay); live re-run after that |
| 4 | lb-p1 (brep casing) + lb-p4 (stdio shipped guard, editor_catalog required-features) still green | **check green** 18:41 (`i4-check-default-1.txt`, EXIT 0, 7 m 20 s); law/lib/parity queued (native lane 6 deep) |

### Log

- 18:27 start. Read AGENTS.md, preambles 14/13/12, `📓️fleet-14-agents.md` (no "CHAIN LAUNCHED" yet), `📓️wp-lb.md`, `📓️wp-ld.md`,
  `📓️fleet-13-agents.md` from 14:00. Load 5.2, 0 rustc, 141 GiB free, swap 7.6/9.2 GB, native lane empty.
- 18:3x item 4: lb-p1 and lb-p4 are both still in the tree (brep enums carry `rename_all_fields = "camelCase"` ×3; stdio
  `editor_catalog` `required-features = ["full-app-catalog"]`, `shipped_fleet` test target, `artifact-app-testing` dev-dependency).
  Native re-verification chain launched detached (`wp-lb2/item4-verify.sh`, pid 40311): check default `--lib --tests` of stdio plugin +
  semio artifact, `shipped_fleet` law, semio lib brep tests, brep parity (`i4-*.txt`).
- 18:41 item 4 `cargo check --keep-going -p semio-s-plugin-stdio -p semio-s-artifact-stdio-semio --lib --tests` (native lane, build-fleet-b)
  **EXIT 0** (`i4-check-default-1.txt`): lb-p1 + lb-p4 still compile after the later peers' edits.
- 18:4x item 1 evidence (S16 capture `🧑‍💻dev/🤖️generated/🧮️program-matrix/s16-r1en/matrix.json`, row stdio/csv): en FAIL, de PASS (same
  build). The redo applied (`edits=[0,1,0,1]`) but its dispatch was refused: `input #52 redo refused: dispatch-failed … —
  ui.snapshot-details.arguments: snapshot details UI admission failed`; after the redo the details window shows only
  `Source Details` (14 chars). The code is `ui_args` (`🪟️details/🦀️.rs`): `UiMapBuilder::try_new()` → `None` = the process-wide
  `UiValue` arena has no free collection. Arena (`🖱️ui/🧬️contract/🎬️action`): ONE page = `UI_VALUE_PAGE_ROWS` (128) ×
  `UI_VALUE_ROW_COLLECTIONS` (5) = 640 collections / 1 792 pages. The SDK windowing (`tree_window_indexed_rows`) already ends a
  window early on `ui.fixed-capacity`, but the details panel reports arena refusals under its own code, so the refusal escapes as a
  fatal assembly error; nothing in the details panel reads `ui_value_headroom()`. Cost from source (demo 3×2 csv, schema not
  resolved in the guest — the capture shows the 7 untyped add buttons on the root): collection rows 9–11 maps (7 add buttons + template +
  up/down/remove), scalar rows 3–4 → ≈175 maps / 28 rows ≈ 6.3 per row (priced 5). Retirement of a previous generation is
  asynchronous (exact owners paged by the reactor, 256 units/turn), so a quick set-cell → undo → redo keeps several generations live;
  en (29 s) failed, de (41 s, more idle turns) passed — consistent with retirement lag. To be measured in the overlay probe.
- 18:44 overlay `.🧬semio/🌐hub/s14-lb2-overlay` = APFS clonefile of 77 770 tracked + untracked files (`wp-lb2/lb2-overlay.py`, 5 m 26 s,
  no data copied); 18:55 first build refused (`🎨️styling/🔤️tokens/🦀️.rs` is gitignored + generated) → `wp-lb2/lb2-overlay-missing.py`
  cloned the 2 587 missing source-like files (1 m 40 s). Probe test (`lb2_probe`, overlay-only, appended to the details unit tests):
  per-render collections/items/rows for csv 3×2 and 40×5, typed and untyped, generations admitted until refusal, drain back to base.
  Runner `wp-lb2/overlay-cargo.sh` (overlay lane, private build/target dirs `.🧬semio/🌐hub/s14-lb2-{build,target}`).
- 18:5x coordinator: a Codex peer edits `🪟️details` (in-flight 18:49: `ValueShape` provider, schema lookup) → my change stays narrowly in
  the admission path, re-diff before landing. Found + reported: the Codex stdio `📜️script.ts` TestScript gate asserts 36/36 shipped
  formats + 88 playground rows (fails on the lb-p3 tree, 7 formats / 9 rows) → coordinator: ST2 owns that conflict; not touched.
- 19:2x item 1 design (patch p1, `wp-lb2/lb2-p1-arena-budget.py`, `--dry-run` 6 files / 0 problems on the live tree and on the overlay):
  (a) SDK `tree_window_indexed_rows` starts a row only while `ui_value_headroom().rows() > 0` (the arena is ONE page shared by
  every panel and by the generations still being retired; a stop there stamps the full extent like every short run);
  (b) details `ui_args`/`pointer_argument` report the 4 arena refusals as the framework's `ui.fixed-capacity` (new
  `fn arena(stage)`; the other `ui.snapshot-details.*` codes stay), so the existing early-end path of the windowing catches them;
  (c) law `🧬️contract/🧪️tests/🎟️details-arena-headroom` (own test binary = own arena, no interference from parallel unit tests) +
  fixture/schema `✏️editing/{🧫️fixtures,🧬️schema}/🎟️details-arena-headroom`: csv-shaped documents, the arena held down to N free
  collections, render en + de → Ok, the details section stamps its full extent, fewer rows than the idle render, every bound
  pointer resolves in the document by serde_json's RFC 6901 `pointer` (third-party oracle; inserts: their parent), then the
  credit returns and the complete window comes back. No UX change, no arena growth.
- 19:2x finding (for the Codex details owner, not changed): the S16 capture's first details render shows the 7 untyped add
  buttons on the csv ROOT, although the csv snapshot schema (`additionalProperties: false`, typed `items`) would give none —
  i.e. `snapshot_schema` did not resolve `s.stdio.csv` in that guest render (HEAD code). Typed, the demo costs ≈ 81 maps
  (≈ 2.9 / row, within the 5 priced); untyped ≈ 175 (≈ 6.3 / row). The peer is rewriting `snapshot_schema` right now.
- 19:3x item 2 design (patch p2, `wp-lb2/lb2-p2-declared-arguments.py`, dry run 1 file / 0 problems): the SDK bridge law that 21
  plugins already call (`assert_declared_actions_bridge_to_commands`) also asserts `declared_verbs_reading_undeclared_arguments`
  — base = staged declared defaults; probe = each key of `UNDECLARED_ARGUMENT_VOCABULARY` ∪ every key the app declares anywhere,
  not declared by this verb, under 5 value shapes; any different answer (other encoded command / refusal ↔ command) is an observed
  read. The framework-owned verb list becomes one const shared by both laws (its rationale leaves the loop body). Heuristic re-run on
  the current tree: 508 candidates (`heuristic-census-1.txt`); e.g. block2d's 9 include `addHandleKind`, whose command is the empty
  `AddHandleKind {}` (a heuristic false positive).
- 19:3x item 3: harness copied to `wp-lb2/live/` (captures → `wp-lb2/generated`, data roots `s14-lb2-*`). Criterion rewritten: the
  head also moves for every history transition (auto check-in, undo), so "the hub committed only the winner's edit" now counts the
  document's committed DOMAIN operations (after − before, read after the head is quiet for 8 s) with `wp-lb2/wal-ops` (ticket-local
  reader: db `replay_document` + `decode_wal_command`, commands inside committed transactions only, classified by the replication
  crate's `is_history_transition`); the normal-policy row requires 2 domain ops from 2 actors. `bun build --no-bundle` parses.
- 20:0x item 2 coverage: only 15 hand-written bridge-law calls exist (10 plugins; `bridge-callers-census.py` → `bridge-callers-census-1.txt`),
  so a per-app law alone would leave most apps unchecked. p2 therefore adds `command_bridge` to `ArtifactCodecTableV1` (the per-app
  table of type-erased answers every registration already records — one construction site, `artifact_codec_table::<A>()`) and
  `assert_every_registered_app_reads_only_declared_arguments(plugin)`, which probes every app a plugin registers (editors + viewers of
  every declared subset, every `PluginBuilder` app). p2b (`lb2-p2-plugin-calls.py`) adds ONE call per plugin: appended to the 28
  mounted `🧪️tests/🔬️surface` files; process + forms get the surface mount, the file and the SDK `artifact-app-testing` dev feature;
  stdio gets `🧪️tests/🔑️declared-arguments` + `[[test]] declared_arguments`. The typed bridge law keeps calling the same check.
- 20:2x p1 extended (same script, now 7 files): a short window would stay short — the React host's tree-window request is a function
  of geometry only (`treeWindowBodyRequestsV1`), so after an arena-cut render it never asks again. The SDK windowing now counts the
  rows it leaves unbuilt for want of arena credit (`take_arena_unbuilt_rows`, thread-local, re-exported; the capacity-refusal branch
  counts only when the arena is the cause, `ui_value_headroom().rows() == 0`), and the reactor turn notes each short surface after its
  render (`ArenaStarvedSurfaces`, fixed `DIRTY_RENDER_CAPACITY` slots) and re-dirties it next to the deferred-surface re-dirty as soon as
  `ui_value_headroom().rows()` can price the missing rows — each follow-up builds more, a surface the arena cannot help is not re-rendered.
  The law asserts `take_arena_unbuilt_rows()` > 0 for every arena-shortened window and 0 for whole ones.
