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
