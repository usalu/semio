# WP-LB2 — stdio csv Redo Arena Budget, Verb-Arg Law, LD Leftovers

Session 14 slice LB2 (continues [📓️wp-lb.md](📓️wp-lb.md) and [📓️wp-ld.md](📓️wp-ld.md)). Coordinator = main chat. Ports 8130–8139 / 6630–6639.
Scripts + prepared patches `wp-lb2/`, captures `wp-lb2/generated/` (expendable), private target `wp-lb2/target`, durable data
`.🧬semio/🌐hub/s14-lb2-*`. Native cargo only via `wp-lb2/cargo-lane.sh` (native lane, build-fleet-b, nice 15); overlay builds via the
overlay lane with a private build-dir. Landing rows: `📓️landing.md` § Session 14.

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
