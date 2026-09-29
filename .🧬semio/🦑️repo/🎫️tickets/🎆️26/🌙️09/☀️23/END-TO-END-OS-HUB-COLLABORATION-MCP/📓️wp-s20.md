# WP-S20 — Command Reachability (1.8) And Export/Import Through The `s` UI (1.7)

Slice: S20 (session 14; continues T12's rows 1.7/1.8). Coordinator: `main`. Ports: hubs 8140–8149, serves 6640–6649.
Scripts/patches: `.tmp-ticket/wp-s20/`. Expendable captures: `.tmp-ticket/wp-s20/generated/`. Durable data: `.🧬semio/🌐hub/s14-s20-*/`.
Private cargo: `CARGO_TARGET_DIR=.tmp-ticket/wp-s20/target` (native lane, build-fleet-b).

## Session 15

Successor S20 (2026-09-29 19:2x, Opus 5.5). Rules: `📓️session-15-preamble.md` (29–33) + session-14 rules 1–28. GUEST FREEZE ON.

| # | Item | State | Evidence |
|---|---|---|---|
| R | reconcile predecessor (last active ~13:2x, session limit; not resumed in session 14) | **done 19:3x** — no orphan S20 lane job, no half-applied live hunk (all S20 work sits on the faults overlay); last overlay captures: framework fault laws **EXIT 0** 13:52; family checks RED: FH1 `fh1-H2` EXIT 101 13:48, FH2 `fh2-check3` EXIT 101 13:55 (24 × E0277 `?` → `dsl::Fault`), FH3 `fh3-all2` EXIT 101 13:59 (11 × E0308); helpers FH1–FH3 dead since ~13:2x | `.🧬semio/🌐hub/s14-s20-overlay-build/logs/0929-1{24917,25217,25749,30244}-*.txt` |
| 1 | rebase the faults overlay onto the post-round-3a live tree (`s20-overlay-land.py rebase`) | **done 19:37**: 651 clean · 72 merged · 13 token-merged (each verified = exactly our edits) · 7 hand-resolved + 1 accepted live deletion (`s20-rebase-resolve.py`); sync 870 copied / 114 created / 30 deleted; backup `s14-s20-rebase/backup-0929-193710` | `.🧬semio/🌐hub/s14-s20-rebase/report.json` |
| 2 | census + fix-ups on the rebased overlay; then L1's train union `check --lib --tests` (228 crates + 59 features) on the overlay | **census 22 → 1** (`s20-rebase-fixes.py`: cad/puzzle2d/3d/5d flag-value, jack query-too-large + 6 retired jack declarations, SDK presence read, P9 `app.command.no-effect` {action} / `app.notice` / `app.example.*` catalogued (617); `s20-rebase-pdf-page.py`: 44 free-text pdf-page refusals LB2's round 2 added → codes, 22 new en/de). Remaining 1 = `MutationMessageCode for &'static str` bridge (`FaultCode::new(self)`, pass-2 scope) → coordinator decision. Union check QUEUED 19:44 (overlay lane, behind LB2 p17 / C12 / P9) | `.🧬semio/🌐hub/s14-s20-sets/rebase/census-{1,2,3}.txt`, `s14-s20-overlay-build/logs/0929-194444-s15-union-1.txt` |
| 3 | row-12 landing plan dry run on live | **0 conflicts 21:08** (746 clean, 1 merged, 1 token-merged, 8 creates; earlier 19:5x also 0) (740 clean, 2 merged, 8 creates; the gitignored generated manifest TS is not landed → L1 regenerates typegen `framework-rs:generate` in the row-12 step) | `.🧬semio/🌐hub/s14-s20-landing/report.json` |
| 4 | exports through the `s` UI: layout, shooting native raster, cad brep | **layout PREPARED 21:3x** (`s20-patch-layout-export.py`, T7g (renumbered), dry-run clean on live + overlay, not compiled: freeze); shooting = in-guest raster path needed (`rasterize_svg_to_png_base64` is native-only) — open; cad = CD1's step-exchange set | `wp-s20/s20-patch-layout-export.py`, `📓️t6-queue.md` T7g |
| 5 | coordinator decision (a): pass-1 census excludes ONLY the pass-2 bridge (path+item `FAULT_PASS2_BRIDGE`) + FINDING: SDK app-origin raises → `Framework` (17) + law rule `framework-app-origin` | **done 20:2x**: self-test 48/48, census **0 violations PASS** | `wp-s20/s20-f1/f1-origin-and-bridge.py`, `s14-s20-sets/rebase/census-4.txt` |
| 6 | union-1 reds (43 errors / 10 crates) fixed: process3d flag-value, draw 24 free-text `ok_or`, os-mcp 3 gateway faults, 10 test helpers | **fixed 20:3x**, census 0 (`census-5.txt`); **union-2 QUEUED 20:35** | `wp-s20/s20-rebase-fixes-2.py`, `…/logs/0929-203523-s15-union-2.txt` |
| 7 | PASS 2 (mutation reports by code): spec §6, p2 overlay (APFS clone), codemod 3 583 report sites + 656 apply rejections → `MutationCode`; drift list for FH4 | **codemod done 21:0x**; FH4 on drift (133 left); **P2-F1 written 21:1x** (`p2-api.py`: `MutationCode` + reports/apply errors without message, bridge + census exclusion removed; `p2-consumers.py`: history wire, store, sync (7 literals), planner, run, MCP relay, wgpu conflict row = catalog text by code; `p2-ts.py`: kernel TS type, React conflict text by code; catalog texts + classes for the 7 codes); framework check QUEUED 21:17 on the p2 build-dir (seeded by APFS clone) | spec `📓️fault-localization-api.md` §6, `wp-s20/s20-p2/p2-codemod.py`, `s14-s20-sets/p2/drift-sites.json` |
| 8 | FAULT CLASSES (coordinator 20:5x): 7 classes, `.fault(code, class, text)`, record v3 `class`, os-mcp map by class, law | **applied 21:0x on the pass-1 overlay**: machinery (`class-machinery.py`: `FaultClass` + `FaultDefinition/FaultCatalogEntry.class`, guest-side `declared_fault_class` (catalog scanned line by line, no big allocation), builder `.fault(code, class, text)` + registration, record v3 `class` (Rust/schema/fixture/TS decoder/AJV twin), catalog schema, projection + generated TS, os-mcp `Fault.class` + map BY CLASS + `details.fault.class`, census rules `class-missing`/`class-unknown`, `framework-app-origin` narrowed to raise position); reviewed classes (FH5/6/7) + S20 pattern calls (`class-decisions.py`: `*closing*` → unavailable (36), window/route-stale → precondition-failed, stale handles → internal, 4 singles) + 9 retexts (`class-retext.py`) applied by `class-apply.py` (1 942 declarations, catalog 617). **Census 0 violations PASS** (`census-7.txt`), self-test 49/49, TS twin 23/23, tsc os 0 + repo 0; union-3 + fault laws QUEUED 21:05 | `wp-s20/s20-class/`, `s14-s20-sets/rebase/{census-7,tsc-os-2,tsc-repo-1}.txt`, `…/logs/0929-210514-s15-c3-*.txt` |
| 9 | pass-1 union-2 (L1's 228 crates + 59 features, after the union-1 fixes) | **GREEN 20:54** (EXIT 0) | `…/logs/0929-203523-s15-union-2.txt` |

### Session 15 log

- 19:2x read preambles 15/14, this report, `📓️fault-localization-api.md`, `📓️t6-queue.md` row 12, FH1–FH3 reports, L1 session-15 table
  (round 4 = C13 P4, C12 hub-order, SH2 P2, T7c P9, T7d LB2 p17, then row 12). Overlay change set vs its 08:53 baseline: 745 changed +
  9 created (non-`.🧬semio`), 0 deleted (`.🧬semio/🌐hub/s14-s20-sets/rebase/diff-0.json`).
- 19:3x `wp-s20/s20-overlay-land.py` gains `rebase [--write]` (overlay := live for every non-OURS file, OURS := `git merge-file --diff3`
  onto live; conflicts → `.🧬semio/🌐hub/s14-s20-rebase/conflicts/`, hand resolutions → `resolved/` + `resolved.json`; `--write` backs up
  OURS + baseline, writes overlay, new baseline = live at the rebase, and the merge BASE blobs `…overlay-faults.bases/` — so the next
  rebase (after round 4) and the landing plan no longer depend on git history).
- 19:3x rebase dry run: 21 line-level conflicts → `token_merge` (same 3-way over word/punct/space tokens) merged 13 (verified: token edit
  multiset live→merged == base→ours for all 13); 8 hand-resolved in `wp-s20/s20-rebase-resolve.py` (jack editor/text-edit/format/run-query:
  live side + raise conversions; jack window config deleted by C12's query-in-document → accepted; pdf 1.7 page imports + `req_text`;
  os plugin `📜️script.ts` oracles union; generated manifest TS = live + `faults` field + `FaultDefinition`). `rebase --write` 19:37.
- 19:4x census on the rebased overlay (`GIT_DIR=… GIT_WORK_TREE=<overlay> bun ./📜️script.ts verify faults`): 22 violations (all from
  post-08:53 live code) → fixed → **1** (the pass-2 bridge). Framework catalog regenerated by FH1's tools (`wp-fh1/catalog_texts.py` +4,
  `fh1-census.ts A`, `fh1-catalog.py` → 617). Hidden free-text helper calls the census cannot see: pdf 1.7 page (44) converted by hand
  table; the rest is the compiler's job (union check queued: `wp-s20/s20-f1/overlay-union.sh s15-union-1`).
- 19:5x landing plan (`s20-overlay-land.py plan`, BASE blobs from the rebase): 0 conflicts.
- 20:0x–20:5x coordinator decisions: (a) bridge exclusion by path+item; FINDING SDK app-origin → Framework (17) + rule
  `framework-app-origin`; pass 2 started (spec §6, p2 overlay clone 7 min, codemod 3 583 + 656 sites, FH4 on the drift list);
  fault classes (spec §7). EX1 relay answered (catalog has both `app.example.*`; EX1 retexts `unreadable` with {example}).
- 20:3x union-1 reds fixed (`s20-rebase-fixes-2.py`), union-2 GREEN 20:54 (only 17 crates re-checked: every other unit fresh).
- 21:0x class machinery + reviewed classes + pattern calls + retexts (see row 8). A broken splice of mine in the gitignored generated
  manifest TS (rebase resolution cut `FaultDefinition` at its first field doc) repaired in both overlays; resolver fixed.
- 21:1x union-3 red: 1 error (EditorBuilder's forwarding list `fault(code, text)` lacked the class) → fixed in `class-machinery.py`;
  union-4 + fault laws + p2 framework check queued; all three wait at the lane's load gate (1-min load 61: peers' builds + copies).
- Exports (item 4) analysis: layout `layout:out` refuses because the layout editor has no `build_snapshot_disposer` (SDK default
  `None` → "media export owner lacks an exact bounded snapshot disposer"); shooting `photos:out` calls
  `semio_framework_os::rasterize_svg_to_png_base64`, which is native-only (usvg/resvg) and refuses inside the wasm guest → needs an
  in-guest raster path; cad `brep:out` = CD1's `wp-cd1/step-exchange` set (new code `cad.export.failed` {reason} to declare at the
  row-12 rebase onto it).
- 21:2x–21:3x waiting: union-4 (pass 1), fault laws, p2 framework check all hold/queue the overlay lane but sit in its load gate
  (1-min load 47–61 from peers). Layout export set prepared (SDK `bounded_snapshot_disposer` reusing the SDK's bounded config
  value retirement; layout opt-in; law) and registered as T7f.
- 21:5x row-12 proof-1 (native 228 + wasm32-wasip2 178 in one ticket): 2 errors total — shared puzzle retained module still
  called `TypedOperationFault::of_fault(fault)` (→ `of_declared`), MCP quick law's `Fault` literal lacked `class` (+ its expected
  `details.fault` gains `class`); wasm32 had no wasm-only error. Fault laws run was blocked by the same MCP test. Fixed in
  `class-machinery.py`; proof-2 (native + wasm32 + laws in one ticket) queued 21:53.
- 21:5x P2-F1 round 1 (`p2-framework-1`): replication map delta still passed code + message → `p2-fixups-1.py` (map apply →
  frozen codes, neutral fixture + TS twin). FH4 DONE (0 drift in its 12 families, census 0); the 13 remaining framework/hub/
  sourcing/reasoning drift sites fixed by me (`p2-fixups-2.py`); census rule `app-mutation-code` added on p2 (`p2-census.py`,
  self-test 48/48). `p2-framework-2` queued 21:53. Open P2 notes (FH4): builders' `Diagnostic::error("mutation.apply", …)`
  (diagnostic codes are outside the fault law — a later pass), gltf rejection record's unused `detail` field.

## Session 14

| # | Item | State | Evidence |
|---|---|---|---|
| 1 | Reachability census on the current tree (42 unreachable in session 12) | **measured 18:4x: 4/1166 left** (source gate) — animate `exportVideoFromDeck` (AV2) + space-home `bindSpaceFile`/`importSpace`/`deleteVirtualFileSystemNode` (SH2); descriptor census adds procedural `listFlowExtensions` (unclassified plugin command, undispatched output). The other 43 (session-12 census 48 incl. demonstrator re-hosts) are `migrated` with a palette row | `wp-s20/generated/verify-interactivity-commands-1.txt`, `census-1.{json,txt}` |
| 1b | Wire a control / implement the interactive job for every still-unreachable command (prepared patches, land in window 3) | open | §log |
| 3 | Framework export/import of every kind's canonical document (coordinator decision 20:0x, owner S20) | design below; host half in progress | §Design |
| 4 | Unowned guest reds (note/shooting import args, architect CSV picker, process format registration, puzzle2d/5d budget, remodel/shooting silent exports, forms description) + cad solid export seam | prepared patches in progress | §log |
| 2 | Export/import through the `s` UI per kind: harness (`📜️script.ts` verb + nx target + launch row), run local + 7800, route reds | **harness written + type-checked + smoke-run** (`verify io`, os-dev); nx target + launch rows relayed to R10 (rule 17); full local run in flight; 7800 run next | §log, `.🧬semio/🌐hub/s14-s20-io/` |

### Session 14c

Successor agent (28 21:1x, WINDOW 3 OPEN; L1 applies guest sets in trains, rule 24). All 8 prepared sets re-dry-run clean on the live
tree 21:2x (initializer 69 files, poll-yield, document-verbs 10, chunk-staging 6, retire-load-request 13, cad-solids, process-formats,
silent-exports).

| # | Item | State | Evidence |
|---|---|---|---|
| 1 | chunk staging PHASE 2 (generation3d + puzzle3d drop private stagings, architect CSV picker, puzzle2d/5d import limits) — lands with phase 1 in T1 | **prepared, dry-run clean 21:5x (27 files), relayed to L1** (script + crates + laws); compile/laws = L1's T1 proof (overlay skipped: rule 23 disk, 107 GiB) | `wp-s20/s20-patch-chunk-staging-2.py`, `wp-s20/s20-chunk-staging-2/{old,new}/` |
| 2 | raster Import Document hang → native root cause + prepared guest fix | repro law in the tree (test-only, end of raster `✏️editor/🧪️tests/🔬️unit/🦀️.rs`); 3 native runs lost (peer red 21:58, panics 22:42/00:31) — NOT yet run | `wp-s20/s20-raster-archive/` |
| 3 | io-matrix `matrixVerbOf` → rendered edits (`driveRenderedEdit`, `pluginEdits`/`kindEdits`) | **landed 21:3x** (host TS dev harness); tsc rc 0 21:3x and again on the post-T1 tree 23:2x (0 errors) | landing row (captures in `wp-s20/generated/` lost in the 01:14 sweep) |
| 4 | `verify io` on hub 7800 (p24) en + de, publish records, route reds | **en partial** (10/12 pinned kinds before the 00:31 panic; verdicts below); de not run (panics, then window closed) — rerun on the ALL-34 catalog after the 07:18 chain | `.🧬semio/🌐hub/s14-s20-io/s20c-hub7800-en-3/` |
| 5 | T6 set: app refusal crosses the interactive-job boundary as a TYPED RECORD `semio.typed-operation-fault.v1` (coordinator decisions 08:0x + 08:3x) | **v2 prepared, dry-run clean 08:3x** (18 files, 4 new); AJV twins + TS decoder ran green on the new fixtures in a scratch copy (14/14, 10/10, 7/7), strict tsc of the TS block rc 0; approved by main 08:4x; relayed L1 + R10 (2 new dirs) | `wp-s20/s20-patch-fault-code.py`, `wp-s20/s20-fault-code/`, `.🧬semio/🌐hub/s14-s20-sets/fault-code-dry-2.txt` |
| 7 | fault localization: apps declare every refusal code with en/de text; hosts/agents render by code; GLOBAL law | **F1 on overlay 11:1x** (`.🧬semio/🌐hub/s14-s20-overlay-faults/`): tightened `FaultCode`/`Fault` types, `app_fault`/`with_parameter`, `AppDefinition.faults` + `.fault` + build validation, framework catalog + schema, `verify faults` (self-test 43/43, oracle agrees), record v3 (row 10 merged into T6 row 12), host `faultTextV1` (vitest 16/16 + PluginRuntime 135/135), Rust `fault_text` law, MCP `details.fault`; F2 codemods (1 287 code-like, 103 double paths, 46 literal `Fault::new`, 50 framework `Fault::from`). Helpers FH1–FH3 on F3 (census-only). Framework overlay check QUEUED (overlay lane) | spec `📓️fault-localization-api.md` §1–4; scripts `wp-s20/s20-f1/`; census `.🧬semio/🌐hub/s14-s20-sets/census/` |
| 6 | all S20 set scripts idempotent (L1 finding) | **done 08:1x**: result-before-anchor checks + set-level landing guard in all 10 scripts; every landed set answers `landed` | `wp-s20/s20-idempotence-check.py` |

#### Session 14c log
- 11:2x–12:1x census made CODE-BASED (coordinator): every `FaultCode::new("…")`/`app_fault("…")` literal is a raise of its owner
  (helpers take `FaultCode`, new `app_refusal(code)`); `Fault::from` caught as a path; nested/in-string `#[cfg(test)]` fixed (FH1/FH3
  reports); self-test 46/46. Mutation reports: `MutationMessageCode` trait (literal or FaultCode) for pass 1 compile; coordinator
  decision: pass 2 localizes reports (constructors `FaultCode` only, ~3.7k sites). FRAMEWORK GREEN 12:02 on the overlay
  (`check --lib` replication, os-kernel, framework, plugin SDK, os-mcp; 3 stamped iterations: `MutationApplyError` code forwarded as
  `received`, 2 SDK `?`-over-String sites → `plugin.action.argument-missing`, MCP `FaultTexts` path; 36 own `unused_qualifications`
  removed, `fix-qualifications.py`). Helpers FH1–FH3 at 0 census violations for their families, crate checks running (FIFO).
  Framework `--tests` queued (framework test `Fault::from` converted, `f2-framework-tests.py`). Landing tool `s20-overlay-land.py plan`
  validated against today's live: 688 clean, 1 merged, 1 approximate-base merge (`📜️script.ts`), 8 creates, 0 conflicts.

- 09:0x–11:1x F1 on the faults overlay (session 14c successor after the usage cut): diagnostic types tightened (`f1-tighten.py`:
  `FaultCode::new(&'static str)`, `Fault::new(origin, FaultCode, msg)`, no `From` into `Fault`/`FaultCode`, `FaultCode::received`,
  boxed `FaultParameters` keeps `Fault` ≤ 112 B, `Diagnostic::new(code, severity, …)`); `f1-fault-new-literal.py` (46 sites);
  `f2-framework-from.py` (50 framework `Fault::from` → catalogued framework raises); `f1-code-types.py` (runtime-string code
  sites → `&'static str`, `PluginAssemblyError.code: &'static str`, hub codes `received`); `f1-manifest.py` += `validate_fault_definitions`
  (wired into `try_build_definition`), `FaultCatalog`, `framework_fault_catalog()`, `fault_text` + law `🛂️manifest/🧪️tests/🔬️fault-text`;
  `verify faults` census (`runFaultCensus` + `faultLawViolations`, orchestration module; gate in root `📜️script.ts`; fixture cases in
  `🧮️source-census`; oracle `git grep -o` agrees; runs on the overlay via `GIT_DIR`/`GIT_WORK_TREE`); record v3 (`parameters`) via
  `s20-fault-code-v3-overlay.py` (v2 snapshot kept), v3 set dry-run clean on live (18 files); host render-by-code (`faultTextV1`,
  `declaredFaultsV1`, fixture `⚠️diagnostic/🧫️fixtures/🧯️fault/🔣️text.json`); MCP `details.fault` (`f1-mcp.py`). Census on the overlay:
  3 665 violations / 1 917 raises / 0 declarations before F3 (per family in `census/family-A..H.json`). Helper reports fixed: cfg(test)
  statement blanking, doubled `semio_framework_plugin::` paths (`f2-double-path.py`, 103 sites). Framework `cargo check` queued on
  the overlay lane 10:47 (5th); coordinator: no reprioritisation, helpers proceed census-only.

- 21:1x read preamble 14 (+14b/14c, rules 1–24), window-3 plan, fleet tail (pending relay: matrixVerbOf), L1 report (T1 holds
  `s20-chunk-staging` until phase 2 exists). Dry runs of all 8 sets clean.
- 21:3x item 3 landed: `🧑‍💻dev/🧪️tests/🚪️io-matrix/🟦️.ts` — `matrixVerbOf` returns `{verb, args, edit}` (edit from
  `kindEdits[<key>.<verb>]` → `kindEdits[<base>.<verb>]` → `pluginEdits[<origin>.<verb>]`), `runImport` moves the document through
  `driveRenderedEdit` when an edit is pinned (new `driveMove`, refusals + notices captured). tsc rc 0 (`generated/tsc-io-3.txt`).
- 21:3x–21:5x item 1 PHASE 2 prepared: `wp-s20/s20-patch-chunk-staging-2.py` (dry-run clean, 27 files; whole-file swaps guarded by
  sha256, spans kept verbatim in `s20-chunk-staging-2/old/`). Findings that shaped it: (a) generation3d's private chunk extent was
  `PUBLIC_INVOCATION_STRING_BYTES` = 4 096 B while the host slices 32 KiB — every generation3d pick above 4 KiB was refused
  `generation3d-import-chunk` live; (b) phase 1 alone would REGRESS puzzle3d (the SDK hands its import a whole 145 KB payload that its
  own 32 KiB chunk check refuses) → phases land together; (c) the puzzle stagings were process-global statics (cross-document leakage);
  (d) the typed-command pipeline is contiguous by construction (`encode_op` → `Vec<u8>`, `begin_exact_wire` reserves
  `Vec<ToolWirePage>` of the payload size), so a reassembled import is one contiguous block like a document-archive load
  (`parent_spr`); the paged puzzle parsers bought nothing once the payload is whole → deleted. Set content: generation3d
  (`ImportDocument {name, payload}`, one size rule `generation3d-import-capacity` = one Artifact-lane edit, owner/route/decoder/
  description, surface fixture + TS twin lose the chunk ledger, laws incl. the placeholder `📥️import-document` test file now real);
  puzzle 2d/3d/5d (whole-payload decode, `Capacity`/`Payload` localized notices, 2d `import_too_large` en/de added, 3d/5d dead
  `import_incomplete` removed, shared `PUZZLE_IMPORT_TOTAL_BYTES`/`PUZZLE_IMPORT_RAW_BYTES` = 2×256 KiB + 8 KiB in the retained routes
  and their proofs, SDK-lane laws: chunks through `handle_action` → one applied edit, gap = typed `file-import.gap`); architect CSV
  picker `importRegistersCsvRequest` (RequestId 132, `.csv,text/csv`, text) → `importRegistersCsv {payload, strategy=upsert}` (the
  `csv` single-line field renamed to the framework's `payload`), appended as the LAST command variant (wire hex pins unchanged),
  catalogue shortcut, declared-verbs 22 → 23, law `import_registers_csv_request_opens_a_csv_file_picker`; io-matrix CSV pin → picker;
  kernel doc on `import_chunk_arguments`. Relayed to L1 21:5x (apply right after phase 1 in T1).
- 21:5x item 2: repro law `a_demo_edit_archive_loads_back_through_the_document_archive_door` appended to raster
  `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (test-only, rule 22; pre-image `wp-s20/s20-raster-archive/unit-before.rs`): seat demo → read the
  document archive → (1) drive `raster_document_store_initialization_job` over the parsed pack/spr envelope, bounded 400 000 steps;
  (2) the whole archive door (begin/poll/maintenance, 240 s wall bound) → Ready + emblem present. Queued in the native lane 21:58
  (`generated/raster-archive-law-1.txt`, runner `wp-s20/s20-raster-archive/run-law.sh`).
- 21:58 item 4: root `📜️script.ts verify io` is NOT the harness (it fell through to the repo verify gate, which died in
  dependency-cruiser ENAMETOOLONG on `🌎️hub/…/🗑️generated/test-artifacts/linked-ancestor-publication-owner-*` recursion — a peer's
  test artifacts; nothing ran). Runner `wp-s20/s20-io-hub.sh <en|de> <tag> [hub]` = os-dev `bun ./📜️script.ts verify io --serve
  :6640 --hub … --out .🧬semio/🌐hub/s14-s20-io` (creds from the hub recipe into env only).
- 22:1x hub smoke `s20c-hub7800-smoke-2` (draw, note on 7800/p24): census creatable 17+ kinds (the 09-27 run saw 0); framework
  document round trip 2/2 byte-identical over hub documents (623 B / 528 B); own formats 0/2: on a HUB document `setActiveExample`
  is refused `action-guest-refused`, the next press `action-state-unconfirmed`, and every later action `action-owner-mismatch`
  ("The document owner changed — try again.") — the shell's action owner never recovers after one guest refusal on a hub doc; plus
  React `Cannot update a component while rendering a different component` in `FrameworkOsShellInner`. Full en run launched 22:15
  (`s20c-hub7800-en-1.log`).
- 23:3x **hub-document action cascade (routed to S18 by the coordinator) — evidence**, all under `.🧬semio/🌐hub/s14-s20-io/`:
  first refusal = row `draw/drawing` right after the navbar seats example `demo` on a freshly created hub document:
  `s20c-hub7800-en-2/draw-drawing.console.txt` line 10 (`21:24:13.912 input #10 setActiveExample refused: dispatch-failed … —
  action-guest-refused`), then line 12 (`input #11 exportDocument … action-state-unconfirmed`), then line 13 onward every action
  `action-owner-mismatch`; screenshot `s20c-hub7800-en-2/draw-drawing.png`; same sequence in `s20c-hub7800-en-1/draw-drawing.console.txt`
  (20:17:51 / 20:18:05 / 20:19:08 …, + `Cannot update a component while rendering a different component … FrameworkOsShellInner`
  at 20:20:16) and `s20c-hub7800-smoke-2/draw-drawing.console.txt` (20:10:55 …). Mechanism (host, `🏪️store/👷️worker/🟦️.ts`
  ~l.2465–2475): a typed-operation page on the FAULT lane only increments `publication.refusals` (`typedOperationPageAnswerV1`
  returns `refused: boolean`, `🎭️actor/🖼️wire-turn/🟦️.ts`) → `action-guest-refused` WITHOUT the guest's fault code/message (it is
  dropped — no console line, no notice); the next action's failure after `invoked` is not an explicit refusal → `this.close()` closes
  the document browser actor → every later action fails the owner guard (`documentBinding`/child gone) as `action-owner-mismatch`.
  Why `setActiveExample` refuses: not observable from the page (the fault text is dropped). The guest verb is draw's
  `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/…/✏️editor/🎮️commands/🖼️set-active-example/🦀️.rs`, which answers with ONE
  `Effect::LoadDocument` (`drawing_reset_document_effect`, a whole-document replace) instead of event-sourced mutations — on a shared
  hub document executed by the browser actor that is the likely refused shape (same verb seats `demo` fine on a local document,
  `s20b-local-en-2`). Owners: S18 (cascade: surface the fault-lane payload + don't close the actor on a guest fault), U7 (example
  loaders that replace via `LoadDocument` — draw drawing is one).
- 23:3x 7800 document column = **expected red until the next chain (pre-T1 guests)**: since T1 landed `s20-document-verbs` the
  harness presses the SDK-reserved Export/Import Artifact Document rail rows, which the p24 guests do not declare (`pressed: absent`).
- 08:0x resume after the usage cut, 4 kernel panics and the 01:14 sweep (rules 25–27; `wp-s20/generated/` is gone — captures now
  only under `.🧬semio/🌐hub/s14-s20-*`). T1 (incl. S20 initializer/poll-yield/document-verbs/chunk-staging 1+2/retire-load-request)
  and T3 (cad-solids, process-formats, silent-exports) are landed; LW1: cad 5/5, process3d export 2/2, shooting export 5/5, remodeling
  32/1 (`export_qc_report_is_a_no_op_without_a_report` asserted the old silent success; the refusal's code was the wrapper
  `interactive-job.app-owned-output` with `remodeling.qc-report.missing` inside the text — `.🧬semio/🌐hub/s14-lw1-logs/s20-laws-1.txt`).
- 08:1x item 5 (T6, prepared): root cause = `retained-command` `reducer_fault_detail` flattened the app's `Fault` into prose
  (`retained command reducer rejected operation: <code> <message>`) and `ArtifactBoundedToolFault::from_payload` kept job details as
  message-only bytes → the fault page carried `interactive-job.app-owned-output`; only the agent preview re-parsed the prose; puzzle's
  retained job DROPPED reducer faults for a fixed sentence. The set frames every reducer refusal as `<code>\u{1f}<message>` (the page
  framing `decode_typed_operation_fault_page` already splits), parses that framing in `from_payload`, answers agents with origin `App`
  for a coded refusal, removes the prose helpers, and updates the language-agnostic agent-lane preview fixture + AJV twin, the
  retained-command laws and the remodeling law (now asserts the domain code and that it is NOT in the text). Gap noted: `Fault.message`
  is one language (the app's); hosts localize by the structural code.
- 08:1x item 6: L1 found "apply" reported for already-inserted hunks (anchor still present inside the insertion). Fixed in all 10 scripts:
  a hunk whose RESULT is present is `applied` before its anchor is considered, plus a set-level landing guard (`LANDED` markers: all
  present → `landed: nothing to apply`; partial → CONFLICT, no write) — needed because T5's codemods reworded inserted docstrings, so a
  per-hunk result check alone cannot see a landed insert. All 9 landed sets answer `landed` on the live tree; the T6 set is proven
  idempotent in memory (`wp-s20/s20-idempotence-check.py`).
- **7800 (p24) io verdicts per format, en** (`.🧬semio/🌐hub/s14-s20-io/s20c-hub7800-en-3/`, 10 of 12 pinned kinds; guests are p24 =
  pre-window-3, so every verdict below is on the OLD guest code; document column = expected red until the next chain (pre-T1 guests)):

  | kind | format verdicts on a hub document | cause (measured) → owner |
  |---|---|---|
  | draw/drawing | pdf, svg: refused (`action-state-unconfirmed`, then `action-owner-mismatch`) | example seat `setActiveExample` refused on the hub doc → browser actor closed → cascade (S18; U7: draw's example = one `LoadDocument`) |
  | layout/layout | png, svg, pdf, package: pressed, NO file, no refusal, no dispatch line | hub-doc action never reaches the guest (C12 gate `[DEBUG] c12 gate … pair:false`) → C12/S18 |
  | puzzle/puzzle2d, puzzle/puzzle5d | json: pressed, no file, no refusal | same silent no-dispatch as layout → C12/S18 |
  | puzzle/puzzle3d | json: refused `action-refused`; no example seated (0 examples offered on the hub doc) | S18 |
  | cad/cad | step/obj/stl: `action-owner-mismatch`; reach 4/4 dispatched | cascade after the example seat (S18) |
  | process/process3d | step/obj/stl/glb: refused `process3d.media.export unknown process export format kind` | fixed by S20 process-formats (landed T3, not on p24) |
  | remodel/remodeling | qc: no file, no refusal | fixed by S20 silent-exports (landed T3) |
  | animate/presentation | video: `interactive-job.not-ui-safe` (BatchOnlyPendingRewrite) | fixed by AV2 video export (landed T3) |
  | procedural/generation3d | created document never opened | S19 / hub creation |
  | architect/program, note/note | not reached (panic 00:31) | rerun on ALL |
- 08:3x item 5 v2 (coordinator correction 08:3x: no string-packed side channel): the refusal is the typed record
  `semio.typed-operation-fault.v1` {schema, code, origin, message} — schema `🔌️plugin/🧬️schema/🧯️typed-operation-fault/🔣️.json`,
  Rust `app::TypedOperationFault` (`of_fault`/`encode`/`decode`/`into_fault`) carried by the job fault detail AND all 8 Fault-lane
  page producers (the completion path used to publish `fault.as_bytes()` = message only, the cancellation and latest-wins paths raw
  text); `ArtifactBoundedToolFault` keeps code + origin + message and publishes its record; agent preview + registered fixture answer
  the record's own code/origin; TS twin `decodeTypedOperationFaultV1` + `typedOperationPageFaultV1` in wire-turn (scan faults and
  page answers are records), React router rejects with `TypedOperationFaultError{fault}`. Fixture `🧫️fixtures/🧯️typed-operation-
  fault.json` (7 record + 7 decode cases) + Rust law + AJV twin (registered in the plugin SDK `test` script); agent-lane fixture moves
  to records (10 cases). Pre-validation without cargo (guest freeze): both AJV twins and the TS decoder run green against the new
  fixtures in a scratch copy; strict tsc of the TS block rc 0; every removed Rust symbol has no user left after the patch; set proven
  idempotent in memory.
- 08:4x item 7 census (`wp-s20/s20-fault-census.py`): see the table row; plan = F1 mechanism + ratchet ledger, then F2…Fn per family.

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

### Design — shooting `photos:out` raster in guests (session 15, for the coordinator's decision)

Facts: `photos:out` renders the shot as a semio drawing (a `Path` frame with fill + stroke, a scaled PNG emblem `Image`, a
centred `Text` label) → SVG → `semio_framework_os::rasterize_svg_to_png_base64` (usvg/resvg, native-only; `wasm32-wasip2` returns
"SVG rasterization requires the native semio-framework-os host"). The framework raster module `🖌️raster` rasterizes on the GPU
(vello/wgpu, native-only); `🔤️typeset` outlines text through Typst/usvg (native-only); the only first-party fonts are the
Libertinus OTF (CFF outlines) under `📚️compiler/🌍️world/🔤️fonts`; `🔲️pixels` has PNG encode/decode + bilinear resize.

- **A (recommended): first-party CPU tier in `🖌️raster`**, target-neutral (native + wasm32-wasip2 + browser guest):
  `cpu::rasterize(&VectorScene, width, height, background) -> RGBA8` — nonzero scanline fill with exact-area coverage
  anti-aliasing over flattened `BezPath`s, stroke expansion (miter/round joins, butt caps) to fill polygons, an `ImageOp`
  (bilinear blit via `🔲️pixels`), bounded work (`step(budget)` → progress + cancellation for the export job); text through a
  first-party OpenType reader (cmap + hmtx + CFF Type 2 charstrings → `BezPath`) over the embedded Libertinus Sans/Serif, simple
  left-to-right advance layout (no shaping beyond cmap; the label is Latin). Shooting renders its semio drawing through it on
  EVERY tier (one output); native resvg becomes the third-party oracle (per-pixel tolerance law on shared fixtures).
  Cost ~1–1.5 days; reusable by layout/draw/raster exports and animate's guest video frames.
- **B: host capability** (guest asks the host to rasterize SVG): new WIT import + native impl (resvg exists) + browser impl
  (OffscreenCanvas is async; components import sync) → ABI change across all components, two host implementations, no
  guarantee the browser twin renders like resvg. Not recommended.
- **Decision 21:4x: A, built by the new slice RS1.** My design above did NOT account for the existing first-party pieces. Pointers
  for RS1, so it does not have to find them again (read-only survey, 21:4x):
  - **Scene input: draw.** `flatten_drawing_document_to_scene_nodes` (the drawing canvas projection) plus the PDF painter
    `drawing_document_to_pdf`, in `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs`.
    The painter walks scene nodes (even-odd fills, shadings, ExtGState alpha, RGB images + SMask, text) into PDF OPERATORS, not
    pixels. Reuse its node walk and paint model as the rasterizer's input contract. Its text is PDF Helvetica, which has no
    outlines.
  - **Target: `🖌️raster`.** It already has the target-neutral scene types (`FillOp`, `StrokeOp`, `DrawOp`, `VectorScene`) and a
    GPU-only `SceneRasterizer` (vello/wgpu). The CPU tier consumes the same `VectorScene`: ONE rasterizer for every guest, never
    shooting-only.
  - **Text: the ui crate.** `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📝️text/🦀️.rs` has the glyph atlas and WG11's T7a
    exact advances + T7b `text::PairKerning` (GPOS/`kern` pairs, Chromium as oracle). Its faces are
    `🖼️assets/🔤️fonts/*/🔤️outline.ttf` (TrueType glyf, simpler than the Libertinus CFF I proposed — drop that part).
    CAUTION: that module is built on parley/fontique/swash (third-party crates behind the ui interface, wgpu target). It is
    not a first-party OpenType reader, and whether it links into `wasm32-wasip2` guests without breaking the "no third-party
    runtime dependency in the shipped component" rule is unverified. RS1 must settle with the coordinator: either reuse behind
    the interface in guests, or extract a first-party glyf/cmap/hmtx reader and keep the same advance + kerning source, so
    exported text measures like both shells.
  - **Oracle:** `semio_framework_os::rasterize_svg_to_png_base64` (usvg/resvg, native) on shared fixtures, with a per-pixel
    tolerance law; `🔲️pixels` provides `encode_png` / `decode_png` / `resize_bilinear`.
