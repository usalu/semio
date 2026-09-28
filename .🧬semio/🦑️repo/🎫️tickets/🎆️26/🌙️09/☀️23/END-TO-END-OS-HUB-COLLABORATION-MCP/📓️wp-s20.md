# WP-S20 — Command Reachability (1.8) And Export/Import Through The `s` UI (1.7)

Slice: S20 (session 14; continues T12's rows 1.7/1.8). Coordinator: `main`. Ports: hubs 8140–8149, serves 6640–6649.
Scripts/patches: `.tmp-ticket/wp-s20/`. Expendable captures: `.tmp-ticket/wp-s20/generated/`. Durable data: `.🧬semio/🌐hub/s14-s20-*/`.
Private cargo: `CARGO_TARGET_DIR=.tmp-ticket/wp-s20/target` (native lane, build-fleet-b).

## Session 14

| # | Item | State | Evidence |
|---|---|---|---|
| 1 | Reachability census on the current tree (42 unreachable in session 12) | **measured 18:4x: 4/1166 left** (source gate) — animate `exportVideoFromDeck` (AV2) + space-home `bindSpaceFile`/`importSpace`/`deleteVirtualFileSystemNode` (SH2); descriptor census adds procedural `listFlowExtensions` (unclassified plugin command, undispatched output). The other 43 (session-12 census 48 incl. demonstrator re-hosts) are `migrated` with a palette row | `wp-s20/generated/verify-interactivity-commands-1.txt`, `census-1.{json,txt}` |
| 1b | Wire a control / implement the interactive job for every still-unreachable command (prepared patches, land in window 3) | open | §log |
| 3 | Framework export/import of every kind's canonical document (coordinator decision 20:0x, owner S20) | design below; host half in progress | §Design |
| 4 | Unowned guest reds (note/shooting import args, architect CSV picker, process format registration, puzzle2d/5d budget, remodel/shooting silent exports, forms description) + cad solid export seam | prepared patches in progress | §log |
| 2 | Export/import through the `s` UI per kind: harness (`📜️script.ts` verb + nx target + launch row), run local + 7800, route reds | **harness written + type-checked + smoke-run** (`verify io`, os-dev); nx target + launch rows relayed to R10 (rule 17); full local run in flight; 7800 run next | §log, `.🧬semio/🌐hub/s14-s20-io/` |

### Session 14b

Successor agent (28 12:1x). Guest freeze ON (chain 12:02). Serve: S18's 6540 reused through `ensureDevServe` (never stopped by me);
no hub runs (7800/B3 cannot open current-tree documents since the 12:55 channel-version alignment — coordinator 13:4x).

| # | Item | State | Evidence |
|---|---|---|---|
| 0 | Reconcile predecessor's in-flight edits | **done**: the framework Export/Import Document host half was complete in the tree (ShellHelpers region 📤️DocumentTransfer, ShellHost handlers + palette gating, TaskManager lane `documentTransfer`, ⚛️react en/de labels, I18n keys); `verify io` routing present. tsc 2060 files strict rc 0; rule-20 boot `ready:s`, 0 pageerrors; landing rows added (were missing) | `wp-s20/generated/tsc-transfer-2.txt`, `.🧬semio/🌐hub/s14-s20-io/s20b-doc-1.log` |
| 1 | `verify io`: mp4 (FFmpeg oracle, AV2 §S20) + `archive` format + framework document round trip in EVERY editor row + ONE acceptance record (R10) | **landed (harness TS), measured**: full local en run **32/75 kinds pass; document round trip 40/75 (+ note 1/1 on rerun = 41/75), own formats 1/19, reach 11/12** (only animate `exportVideoFromDeck`). mp4 oracle proven (x264 file ok, truncated refused) | `.🧬semio/🌐hub/s14-s20-io/s20b-local-en-2/{table.md,io-matrix.json}`, `s20b-note-1.log`, `wp-s20/generated/probe-mp4.ts` |
| 2 | Decision (b) SDK half: `exportArtifactDocument`/`importArtifactDocument` framework-reserved verbs (manifest Rust+TS ids + definitions, SDK window chain, reserved predicate/kind, guest refusal `framework.document-transfer.shell-owned`, MCP dedup (Chrome audience), React + wgpu reserved lists, ShellHost intercept, os commands removed, harness presses the rail rows) | **prepared patch, dry-run clean 13:1x** (38 hunks, 10 files); spans agreed with P9 (P9 lands first) | `wp-s20/s20-patch-document-verbs.py`, `generated/patch-document-verbs-dry-2.txt` |
| 3 | cad saveCurrent step/obj/stl → real solids (pane-model child seam) | **prepared patch, dry-run clean** (`collect_pane_solids` via `edit::cad_pane_working_scene`; `pane_world_solids` = viewport's mesh rule + instance TRS; empty pane refused `cad.export.empty-pane`; law `current_pane_exports_its_real_solids`) — compile proof at window 3 | `wp-s20/s20-patch-cad-solids.py`, `generated/patch-cad-solids-dry-1.txt` |
| 4a | NEW (measured): trait-default `build_document_store_initialization_job` = `Err(envelope)` refuses every whole-document load (Import Document, example switch) of 30 editors | **prepared patch, dry-run clean**: default = bounded job (paired with the bounded default owners) in `ArtifactApp`/`ArtifactEditor`/`ArtifactViewer` + removal of 69 plugin overrides that repeat it exactly | `wp-s20/s20-patch-initializer.py`, `generated/patch-initializer-dry-4.txt` |
| 4b | process3d export formats (`unknown process export format kind`) | **prepared patch, dry-run clean**: formats read from the linked stdio codec crates, not the per-guest global catalog | `wp-s20/s20-patch-process-formats.py` |
| 4c | remodel `exportQcReport` / shooting export silent success | **prepared patch, dry-run clean** (refusals `remodeling.qc-report.missing`, `shooting.export.nothing-to-export`; QC file typed `.json`). Live: shooting `exportActiveShot` now writes an SVG (163 KB), so only the no-asset case remains | `wp-s20/s20-patch-silent-exports.py` |
| 4d | note/shooting `loadRequest`, architect CSV picker, puzzle2d/5d import budget | **not patched — proposal** (§14b log 13:4x): the framework Import Document covers the canonical round trip; these per-kind whole-document pickers never matched their own exports | §14b log |
| 6 | Decision (1): retire note/shooting `loadRequest` pickers | **prepared patch, dry-run clean** (45 hunks, 13 files + delete note `📥️load-request`): command modules/enum rows, decoders, retained ids, publication/execution contracts, manifest rows, tests, note cohort fixture (routeCount was already drifted 35 vs 34 commands → 33), shooting retained-command-limits fixture + schema (38 → 37 routes), io-matrix pins | `wp-s20/s20-patch-retire-load-request.py`, `generated/patch-retire-load-request-dry-2.txt` |
| 7 | Decision (2): framework chunk staging for file-open imports | **prepared patch phase 1, dry-run clean** (6 files): kernel `ImportStaging` (2 runs × ≤ 128 chunks = 4 MiB, typed `file-import.*` refusals, restart/retransmission/gap/LRU), 10 language-agnostic `stagingCases` + 3 Rust laws (fixture cross-checked by a Python port: all outcomes agree), SDK admits args in `dispatch_action`/`dispatch_command`, host `dispatchOpenedFiles` progress + cancel as a Tasks-window task sharing one tracker with Import Document. Phase 2 (adopters: generation3d/puzzle3d drop private stagings, architect CSV picker, puzzle2d/5d contracts) prepared next, lands with phase 1 | `wp-s20/s20-patch-chunk-staging.py`, `wp-s20/s20-chunk-staging/`, `generated/patch-chunk-staging-dry-1.txt` |
| 8 | Decision (3): raster Import Document never answers | **root cause narrowed + host fix prepared**: live repro (`probe-document-import.ts`): a raster archive whose op log carries the Demo image edit (one edit, 3 ops, a 25 KB PNG string) stalls at `1/1 running` forever — also when imported into a fresh editor (content-dependent, not a leak); the same image as an example's initial snapshot loads, and an empty raster archive imports in < 2 s. So the stall is in `RasterStoreInitializationAuthority`'s edit-replay phases (SeedHistory…CommitApplied) — native repro law at window 3 (native lane) before the guest fix. Host half fixed: the poll loop re-polled on microtasks only (page 5 s timers measured at ~35 s during the stall, Cancel unreachable) → macrotask yield | `wp-s20/s20-patch-archive-poll-yield.py`, `generated/probe-raster-{1,2,3}.txt`, archive `generated/probe-raster-2/*.semio-archive` |

**Window-3 landing order** (each: dry-run → apply → native check/test of touched crates → tsc + one boot → io-matrix rerun):
1. `s20-patch-initializer.py` · 2. `s20-patch-archive-poll-yield.py` · 3. `s20-patch-document-verbs.py` (after P9) ·
4. `s20-patch-chunk-staging.py` + phase 2 · 5. `s20-patch-retire-load-request.py` · 6. `s20-patch-process-formats.py` ·
7. `s20-patch-silent-exports.py` · 8. `s20-patch-cad-solids.py` · 9. raster native repro law → guest fix · 10. census re-measure.

#### Session 14b log

- 12:1x read preamble 14 (+14b), AGENTS.md, fleet tail, this report. Predecessor's last tree edits: TaskManager 20:32 (auto-commit
  655, 21:54), ShellHelpers/ShellHost/⚛️react/I18n document-transfer hunks present after the overnight peer edits (grep: every symbol
  has its importer). `tsc -p wp-s20/tsconfig.transfer.json` rc 0, 0 errors, 2060 files, strict (`generated/tsc-transfer-2.txt`).
- 12:3x harness (dev test, not in the shell bundle): `IoFormat` += `mp4` (`judgeMp4File`) and `archive` (`.semio-archive`, framework
  codec); animate pin → `mp4`; EVERY editor row drives the framework pair from the palette (`command.os.os.exportDocument` →
  download → `importDocument` through the file chooser → new window → export again → byte compare → close); pass = document ok ∧
  own pins ok ∧ reach ok. tsc rc 2 = only the peer's `🌎️hub/🤝️integration-harness/🟦️.ts(463)` `BunServerWebSocket`.
- 12:4x `verify io --serve http://127.0.0.1:6540/ --tag s20b-doc-1 --only draw/drawing,puzzle/puzzle3d,note/note`: boot `ready:s`,
  census 60/60 loaded, 75 editors, 0 faults → rule-20 proof for the predecessor's host edits; draw 28 383 B + note 925 B archives
  round-trip byte-identical; puzzle3d refused `artifact-store.persisted-initializer-refused`.
- 12:4x–13:1x relays: P9 (SDK spans, then reserved-predicate route), R10 (one `io-matrix` record per run; `command-reachability-live`
  dropped), AV2 (mp4 landed 12:3x).
- 12:5x–13:5x full run `--tag s20b-local-en-2` (pid 62265, 65 min, 6540). Document round trip causes (35 reds):
  **30 × `artifact-store.persisted-initializer-refused`** (space/space, block 2d/3d/5d, demonstrator playground/puzzle3d, forms, gis
  terrain, layout, norm ×15, playbook-module-procedural, puzzle 2d/3d/5d, remodel, wfc bitmap) → item 4a; generation2d +
  demonstrator/generation3d `plugin.internal.document-archive-replacement.initializer-failed` and procedural/generation3d **wasm trap
  `unreachable`** on archive load (owner S19); raster import never answers (chooser answered, no outcome in 180 s; owner raster);
  note = palette-row flake → rerun `s20b-note-1` document ok (identical 1 142 B). Own formats: cad step/obj/stl 88 B spatial DSL
  (item 3), process3d 4× unknown format (4b), remodel qc no file (4c), note/shooting picker `missing field json`, puzzle2d/5d
  too large to import, animate video `not-ui-safe` (AV2, window 3). Expected after 4a: 71/75 document round trips.
- 13:4x proposal (coordinator decision): retire the per-kind whole-document pickers note/shooting `loadRequest` (they accept
  `.dsl/.spk/.ops` but decode JSON under `json`, never their own export) in favour of the framework Import Document; architect CSV
  and puzzle2d/5d JSON imports need a framework chunk-staging helper (the file-open import arrives as `{payload, name, chunk,
  chunkCount}` chunks of 32 KiB; generation3d and puzzle3d each hand-roll one) — a separate SDK item, not a per-kind patch.
- 13:5x coordinator decisions: (1) retire note/shooting `loadRequest`, (2) framework chunk staging is mine (SDK spans via P9),
  (3) raster Import Document is mine. 14:0x–14:3x prepared (1) and (2) phase 1 (see table; staging fixture cross-checked by a Python
  port). (3) live repro on 6540 (S18's serve, restarted 12:4x): empty raster → import ok 1.2 s; Demo seated → export 25 901 B op log
  (one edit with an embedded `image/png` 25 KB) → import `1/1 running` for > 10 min, page timers 7× slow; the same archive into a
  fresh editor stalls too. Host poll loop patched (macrotask yield); guest stall narrowed to the raster initializer's edit replay.

### Session 14 log

- 18:3x slice start; read preambles 14/13/12, AGENTS.md, `📓️fleet-14-agents.md` (no CHAIN LAUNCHED line yet), `📓️wp-t12.md`,
  `📓️audit-s13-os-frontend.md` §4, `📓️acceptance-s13.md` 1.7/1.8, `📓️audit-s13-plugins.md`, `📓️wp-p8.md`, `📓️landing.md` (LC landed P8's
  cad / space-studio / space-home sets 27 05:34; architect's 8 migrated in the tree by P8 26 05:xx).
- 18:4x census, two measures on the current tree:
  (a) permanent source gate (V1) `cd <repo> && bun ./📜️script.ts verify interactivity commands` → rc 1, 69 s, **declarations=1166
  unreachable=4 oracle=agrees**: space 3 (`bindSpaceFile`, `importSpace`, `deleteVirtualFileSystemNode`), animate 1
  (`exportVideoFromDeck`); architect 22/22, cad 36/36, space studio all migrated (`generated/verify-interactivity-commands-1.txt`).
  (b) descriptor census `python3 wp-s20/s20-census.py generated/census-1.json` over the 60 `🔣️.json` descriptors the final
  chain's describe step wrote 16:15–17:03 (layout/robotic/flow-text from 13:3x): 1542 verbs, **5 dispatch-dead** = the 4 above
  + procedural plugin command `listFlowExtensions` (`unclassified`; plugin commands bypass the UI gate, so it dispatches, but
  the shell drops a command's `output`, so the palette row shows nothing). Session-12 list (P8 `census-committed.json`, 48
  incl. 5 demonstrator re-hosts of cad) re-checked row by row: 43 now `migrated`, every agent-facing one `inPalette` (the 4
  without palette are surface verbs: studio `workflowEngagementSubmit`, `nodeGraphEdit`, `importMediaPayload`,
  `compiledDagEngagementSubmit`). Owners: AV2 (`📓️wp-av2.md` item: real video export), SH2 (`📓️wp-sh2.md` item 1: SH1's
  25-file IO-owning job payload). Live proof that the migrated ones dispatch from their control = item 2's harness
  (`command-reachability-live` column).
- 18:5x coordinator: rule 17 (R10 harness contract). RELAY R10 sent: verb `verify io` in os-dev, new dir
  `🧑‍💻dev/🧪️tests/🚪️io-matrix/`, fixture `🧑‍💻dev/🧫️fixtures/🚪️io-matrix.json`, target `io-matrix`, checks `io-matrix` +
  `command-reachability-live`.
- 19:0x harness (rule 17 shape, tree edits, TS host-only, open during the freeze):
  - NEW `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🚪️io-matrix/🟦️.ts` (`runIoMatrixCli`/`runIoMatrix`, pure `judgeFile`,
    `compareExports`, `formatOfFileName`, `hubKindKey`, `readIoMatrixPins`); NEW fixture `🧑‍💻dev/🧫️fixtures/🚪️io-matrix.json`
    (`semio.os-dev.io-matrix/v1`, 19 programs: exports/imports/reach); `✅️verification/🟦️.ts` routes `verify io`;
    `🗂️hub-document-sweep/🟦️.ts` exports `stagedKinds`/`openSweepSpace`/`createKind` (reused by `--hub`, no fork).
  - Per editor of the live catalog probe: open from the palette → seat first example (navbar) → each pinned export pressed in
    the Actions rail, every downloaded file judged by a third-party parser (Chromium DOMParser/createImageBitmap, pdf.js,
    three.js OBJ/STL/PLY loaders; STEP/CSV/text/glTF by structure) → each `reach` verb pressed (must not be refused
    `not-ui-safe`) → each import: program-matrix verb moves the document, export again (divergence witness), import through
    the file picker (Playwright `filechooser`) or staged argument, export again → identical / equivalent / differs.
    Kinds without pins = `NO CONTROL` rows. Records `io-matrix` + `command-reachability-live` (en + de, `blocked` without a
    serve or, with `--hub`, without `OS_HUB_PROBE_EMAIL`/`OS_HUB_PROBE_PASSWORD`); serve via S18's `withDevServe`.
  - tsc (`wp-s20/tsconfig.s20.json`, 1077 files): rc 0 / 0 errors (`generated/tsc-io-1.txt`); after the 19:1x fixes the only
    error is a peer's `🌎️hub/🤝️integration-harness/🟦️.ts(449)` `BunServerWebSocket` (not mine, `generated/tsc-io-2.txt`).
- 19:0x smoke `bun ./📜️script.ts verify io --serve http://127.0.0.1:6540/ --tag s20-smoke-1 --only architect/program,puzzle/puzzle2d,draw/drawing`
  (reused S18's local-only serve 6540 — swap 21.5/22.5 GB, load 95: no second vite; log `.🧬semio/🌐hub/s14-s20-io/smoke-1.log`):
  census 60/60 plugins loaded, 75 editors, 19 pinned. **reach 4/4** (architect runAnalysis/runReport/runValidation/search dispatch
  from their rows). draw pdf (pdf.js 1 page) + svg (DOMParser) ok. Reds, all real UI facts: architect `importRegistersCsv`
  refused `architect.import-csv-invalid` — its CSV argument is a single-line field, so the file's newlines never reach the
  verb (no file-picker control for CSV import); architect `importProgramRequest` picker not answered within the harness
  window (a direct probe sees the chooser in < 10 s: `wp-s20/probe-request.ts`, `generated/probe-architect-request-1.txt`) →
  harness now waits ≤ 30 s for the chooser; puzzle2d import WORKS (node removed) but the re-export adds
  `"meta":{"kindCompatibility":[]}` → comparator now reports `equivalent` when only empty defaults differ; example seat picked
  `__none__` → filter fixed.
- 19:2x rule 20 (host TS edit → tsc + one serve boot): my edits are under `🧑‍💻dev/🧪️tests/` + `🧑‍💻dev/🧫️fixtures/` (dev
  harness, not imported by the shell bundle), so rule 20's boot check does not apply; tsc as above.
- 19:5x **full local run** `bun ./📜️script.ts verify io --serve http://127.0.0.1:6540/ --tag s20-local-en-1` (41 min, log
  `.🧬semio/🌐hub/s14-s20-io/local-en-1.log`, report + every file + per-row console `.🧬semio/🌐hub/s14-s20-io/s20-local-en-1/`):
  **io-matrix FAIL 5/19 pinned kinds pass, 56 editor kinds without any export/import control; command-reachability-live
  FAIL 11/12** (only animate `exportVideoFromDeck`, `UI dispatch rejected … BatchOnlyPendingRewrite`). Per row:

  | kind | result | red (measured) | owner |
  |---|---|---|---|
  | animate/presentation | FAIL | `exportVideoFromDeck` refused `not-ui-safe` (BatchOnlyPendingRewrite) | AV2 |
  | architect/program | FAIL | DSL export → picker import → re-export **identical**; reach 4/4; CSV import: the staged `csv` argument is a single-line field, the file's newlines never reach the verb → `architect.import-csv-invalid`; no picker control for CSV | S20 (unowned) |
  | cad/cad, demonstrator/cad | PASS* | *pins were `any`: `saveCurrent format=step/obj/stl` all download `cad.current.spatial.dsl` (88 B, `objects:[]`) — no STEP/OBJ/STL is ever written (P8's routed stub: no pane-model child seam, `collect_pane_solids` empty); pins now name the formats → red next run | coordinator (design, deferred since P8) |
  | draw/drawing | PASS | pdf (pdf.js 1 page) + svg (DOMParser); no import control | — |
  | demonstrator+procedural/generation3d | FAIL | example seated, flow never evaluates (`ExtrudeCurve` inputs `?`, empty preview) → `exportDocument` refused `generation3d.io.export … no preview geometry` for obj/stl/ply/gltf | S19 (flow extensions) |
  | demonstrator+process/process3d | FAIL | `exportModel` refused `unknown process export format kind step/obj/stl/glb`: the guest never registers the stdio format descriptors its exporter looks up (its unit test registers them by hand) | S20 (unowned) |
  | puzzle/puzzle3d, demonstrator/puzzle3d | PASS | JSON export → picker import → **identical** | — |
  | puzzle/puzzle2d, puzzle/puzzle5d | FAIL | exports of the Nakagin example (124 KB / 206 KB) cannot be re-imported: `The fixture is too large to import (puzzle2d-import-chunk)` / `That file is larger than one import may carry` | S20 (unowned) |
  | forms/forms | FAIL* | *my pin: the file is `.forms.dsl` text (description says "JSON fixture" → description drift); no import control | S20 pin fixed; description → patch |
  | layout/layout | FAIL* | png/svg/pdf ok; package is a ZIP (3 entries) — harness had no ZIP oracle → added (fflate) | harness fixed |
  | lowpoly/lowpoly | FAIL* | OBJ/PLY/STL ok (three.js); OBJ import restores the same 71 vertices/57 faces in another order → harness now compares triangles (three.js) | harness fixed |
  | note/note | FAIL | picker import dispatches `setFixtureJson {payload, name}` → `missing field json`; invalid text is an empty success | S20 (unowned) |
  | shooting/shooting | FAIL | `loadRequest` import → `importSnapshotJson` `missing field json`; `exportActiveShot` writes nothing and says nothing | S20 (unowned) |
  | remodel/remodeling | FAIL | `exportQcReport` with no QC result: no file, no refusal (empty success) | S20 (unowned) |

### Design — framework Export/Import Document (item 3, 20:1x)

Every artifact kind already has ONE canonical, event-sourced native form that the framework owns end to end: the
**document archive** (`DocumentArchivePack` = root envelope pack + its `spr` op log + every recursively owned member
envelope; `encodeDocumentArchiveBytes`/`decodeDocumentArchiveBytes` in `💻️os/🟦️.ts`, version byte 1, bounded by
`DOCUMENT_ARCHIVE_MAXIMUM_BYTES`). The guest protocol already reads it (`AppCommand::ReadDocumentArchive`) and loads it
with progress + cancellation (`LoadDocumentArchive` → `PollDocumentArchiveLoad` → `CancelDocumentArchiveLoad` →
`AcknowledgeDocumentArchiveLoad`; `AppChannelClient.loadDocumentArchive(archive, signal, progress)`); folder persistence
and the editor↔viewer switch (`prepareDocumentSurfaceV1`: capture → createApp → restore) use exactly this pair. So the
framework answer needs no per-kind code and no guest change for the React shell:

- **Export Document** (`os.exportDocument`, en "Export Document" / de "Dokument exportieren", category `artifact`): the
  focused program's archive (`readAppDocumentArchive`) → `<kind>-<utc stamp>.semio-archive`
  (`application/vnd.semio.document-archive`). The file carries the op log, i.e. the events, not a state dump.
- **Import Document…** (`os.importDocument`, en "Import Document…" / de "Dokument importieren…"): host file picker → decode →
  a NEW instance of the same program (`createApp`) → `loadDocumentArchive` with progress + cancellation as a Task-manager
  task (lane `documentTransfer`) → the instance opens as a spawned program window. The focused document is never
  overwritten (no CRUD replace): an import is "open this event log as a document", exactly like a folder/hub open.
- Oracle: export → import → export is byte-identical (the archive is canonical); io-matrix pins the pair for EVERY
  editor, so the 56 "no control" kinds become measurable rows.
- Schema-first half for agents/descriptors (window 3, SDK): the pair is declared as framework-reserved verbs in every
  app manifest (`exportArtifactDocument`/`importArtifactDocument`, Shell, HostOnly, Migrated, en+de) so rail rows and MCP
  descriptors list them; the host intercepts them onto the same two handlers, and the os commands above are removed in
  that same patch (one implementation). Agents already have `artifact_export` over the semio MCP.
- Per-kind extra formats (STEP/OBJ/PDF/…) stay domain extensions/stdio codecs on top (the existing per-kind verbs).
