# WP-FH2 — Fault Localization Helper, Families B (🗄️stdio) And D (🏗️fem, 🖍️draw)

Slice: FH2 (session 14c, helper of S20's fault localization, spec `📓️fault-localization-api.md` §3–§4). Overlay only:
`.🧬semio/🌐hub/s14-s20-overlay-faults/` (S20-owned APFS clone). Captures: `.🧬semio/🌐hub/s14-fh2-*`.

## Session 14

| Plugin | Sites (census 0) | Codes declared | Violations | Crate check | State |
|---|---|---|---|---|---|
| 🗄️stdio | 651 (app-raise-form 363, fault-from 130, undeclared 158) + ~300 helper-wrapped raises the census could not see | 223 (54 shared contract + 169 in 21 artifact crates) | **0** (11:3x) | pending framework green | census done; compile queued |
| 🏗️fem | 172 (fault-from 48, undeclared 124) | 126 (fem2d 59, fem3d 67; 5 shared `fem.*`) | **0** (11:4x) | pending framework green | census done; compile queued |
| 🖍️draw | 169 (app-raise-form 54, fault-from 98, undeclared 17) + 29 geometry/fill `Err("…")` sentences | 115 (editor 104, viewer 11) | **0** (11:5x) | pending framework green | census done; compile queued |

### Log

- 10:5x read preamble (rules 1–27), spec, census families B/D. Census 0 on the overlay
  (`verify faults 🗄️stdio 🏗️fem 🖍️draw`): 992 violations in my plugins — capture `.🧬semio/🌐hub/s14-fh2-census-0.{txt,json}`,
  per-site dump `.🧬semio/🌐hub/s14-fh2-sites-0.txt`. Framework compile queued by S20 (10:47), crate checks wait on it.
- 10:5x question to S20: app schema validators build `dsl::Diagnostic` with `FaultCode::new(code)` (~60 stdio 🧬️schema
  conformance sites) — findings, not refusals; proposal `Diagnostic::error(..)` constructor form (or a framework
  `Diagnostic::new`).
- 11:0x stdio codemods (scripts + backups `.🧬semio/🌐hub/s14-fh2-work/`): `b1-editing.py` — contract `SnapshotEditError` removed, every
  editing function returns `Fault` with a literal `app_fault("snapshot-edit.…")` (+ `path`/`key`/`argument` parameters), `edit_fault`
  helper gone, `path_error` without free text; `b2-diagnostics.py` — 65 schema/io `Diagnostic { code: FaultCode::new(..) … }` →
  `Diagnostic::new(code, severity, span, message)` (S20 added it, findings not refusals); `b3-stdio.py` — every
  `Fault::new(App, FaultCode::new("…"), text)` / per-editor `*_edit_fault(code, text)` / zip+wav+png `fault(code, text)` helper →
  `app_fault` + parameters from the former `format!` values; merged true duplicates: 18 `*.unhandled-action` +
  `app.command.unsupported` + 102 `action '…' is not …` sentences → `stdio.editor.unhandled-action {action}`; per-subset copies
  (`stdio.xlsx.strict.*`, `stdio.xml-valid.*`, `stdio.i-json.*`) → artifact code; kebab `stdio-<a>-…-tool-mismatch` /
  `…-snapshot-edit-routed-to-native-reducer` / `…-native-edit-command-mismatch` → `stdio.editor.*`; `b4-pdf-page.py` — pdf page
  editor's one free-text code `stdio.pdf.page-edit` (86 sentences) → 62 `stdio.pdf.page.*` codes; semio mesh/brep set-vertex
  sentences → `stdio.semio.*.set-vertex.*`; `window_kit_document_revision_matches` replaces the code-taking revision helper.
- 11:3x declarations (`b5-declare.py`, texts `texts_stdio.py`, idempotent): shared codes on every editor through
  `snapshot_edit_actions_with` → `stdio_editor_faults`; artifact codes via one `editor_faults` per crate root, called by each
  editor of the crate (png now uses `snapshot_edit_actions_with` too). Census `verify faults 🗄️stdio` → **0 violations**
  (`.🧬semio/🌐hub/s14-fh2-census-stdio-3.txt`). Tests updated to assert codes (editing unit, deflate, semio mesh/brep).
  Patch-snapshot mutation kinds (7) now map a refused patch to `MutationOutcome::error(error.code, "", [])`.
- 11:4x S20 census rule change (every `FaultCode::new("lit")` counts, `.map_err(Fault::from)` caught): stdio re-run found 3 gltf
  inference codes (`ArtifactInferenceExecutionError::new(FaultCode::new(..))`) → declared on the gltf editor. S20 answered: Mutation
  codes take `impl MutationMessageCode` (`&'static str` or `FaultCode`), not in the law yet.
- 11:4x fem (`b6-fem.py`, `b7-fem-declare.py`, texts `texts_fem.py`): 48 `Fault::from("code: sentence")`/`format!` → `app_fault` with
  `field`/`value`/`flag`/`action` parameters; computed `format!("{fault}.window-…")` / `fem3d.{fault}.window-context-required` →
  literal codes (`fem.window.required|stale|kind`, per-command context codes); fem2d addressed-window helpers lose their prefix
  parameter; `TryFrom<&str>` + `.map_err(Fault::from)` → `*.loop-mode-unknown {mode}`, `*.waveform-unknown {waveform}`,
  `*.results.mode-unknown {mode}`; `fem.editor.unhandled-action {action}`. Declarations appended to `create_fem2d_app` /
  `create_fem3d_app`. Census fem → **0**.
- 11:5x draw (`b8-draw.py`, `b9-draw-declare.py`, texts `texts_draw.py`): ~100 sentence refusals → 70 `drawing.*` codes (merged true
  duplicates, e.g. 4 "unlock and show … before …" → `drawing.selection.locked-or-hidden`, 5 singular-transform sentences →
  `drawing.transform.singular`); path/fill geometry edits (`edit_path`, `patch_path_point`, `translate_*`, `edit_fill` …) return
  `Fault` with 29 literal `drawing.path.*`/`drawing.fill.*` codes instead of `&'static str` (their tests only check `is_err`);
  `app.command.*` → `drawing.command.arguments-invalid {action}` / `drawing.editor.unhandled-action {action}`; owned-schema
  initializer close → `app_fault("artifact-store.initializer-close")` declared with the framework catalog text. Census draw → **0**.
- 11:5x **all three plugins: `verify faults 🗄️stdio 🏗️fem 🖍️draw` = 0 violations, no oracle disagreement in my paths**
  (`.🧬semio/🌐hub/s14-fh2-census-6.txt`). Unused `FaultCode`/`FaultOrigin` imports removed in 7 files. Next: crate checks once S20
  reports framework green (S20's framework check still QUEUED since 10:47 — lane).
- 11:5x coordinator: stop until S20 reports framework green (usage). State frozen for resume below.

### Resume (for the successor / resumed FH2)

- **Files**: 259 overlay files under stdio (194) / fem (43) / draw (22) differ from the overlay baseline (includes S20's F2 codemod
  edits in these plugins): `.🧬semio/🌐hub/s14-fh2-work/fh2-files.txt` (from `wp-s20/s20-overlay-diff.py diff`, full diff JSON
  `.🧬semio/🌐hub/s14-fh2-work/overlay-diff.json`). Backup of my plugins' `🦀️.rs` taken after b1/b2, before b3:
  `.🧬semio/🌐hub/s14-fh2-work/pre-b3.tgz`; editing/patch originals `editing.orig.rs` / `patch.orig.rs`.
- **Scripts** (all in `.🧬semio/🌐hub/s14-fh2-work/`, re-runnable; b5/b7/b9 are idempotent): `rs.py`/`fmt.py` helpers, `b1-editing.py`,
  `b2-diagnostics.py`, `b3-stdio.py`, `b4-pdf-page.py`, `b5-declare.py` (+`stdio-codes.json`, `texts_stdio.py`), `b6-fem.py`,
  `b7-fem-declare.py` (+`fem-codes.json`, `texts_fem.py`), `b8-draw.py`, `b9-draw-declare.py` (+`draw-codes.json`, `texts_draw.py`).
  To change a text: edit `texts_*.py`, re-run the matching declare script, re-run the census.
- **Census** (static, no compile): `cd .🧬semio/🌐hub/s14-s20-overlay-faults && GIT_DIR=/Users/ueli/Documents/semio/.git GIT_WORK_TREE="$PWD"
  NX_DAEMON=false bun ./📜️script.ts verify faults 🗄️stdio 🏗️fem 🖍️draw` → last result 0 (11:5x).
- **Pending checks** (after S20 "framework green"; overlay lane, one invocation, foreground):
  `zsh .tmp-ticket/wp-s20/s20-f1/overlay-cargo.sh fh2 check --offline --lib --tests -p semio-s-artifact-stdio-contract -p <stdio artifact
  crates touched> -p <fem 2d/3d crates> -p semio-s-artifact-draw-drawing` (crate names from each `📦️packages/🦀️rust/Cargo.toml`;
  touched stdio artifacts: html epw zip mp4 bcf binary csv tsv xlsx pdf docx md xml png pptx wav txt tiff deflate gltf json semio +
  the diagnostics-only ones ifc svg step jpg). Expected fix-ups the compiler may still ask for: a top-level
  `use semio_framework_plugin::app_fault;` not visible inside an inline `mod {}` (29 files got that line), `with_parameter` value types
  (`&String`/`&&str` → add `.as_str()`), leftover unused variables/`path` params after dropping free text, tests that compared
  `SnapshotEditError.code` (&str) now `Fault.code.0` (updated: editing unit, deflate, semio mesh/brep).
- **Then**: run the family laws that assert codes (stdio contract editing unit + patch fixtures, deflate/mesh/brep unit, draw geometry
  + fill fixtures, fem set-result-animation/display units) with `--lib --tests` per crate; note T6 overlap by intersecting
  `fh2-files.txt` with the LB2 (`wp-lb2/lb2-p9…p14`) and U6 (`wp-u6/u6-row-target.py`) file lists for S20's 3-way rebase.
- 12:0x S20 FRAMEWORK GREEN; coordinator: run family checks. First launch `fh2-check1` cancelled by me (zsh did not word-split the
  crate list → only the first crate had `-p`; killed my own waiter pids 80343/80348, ticket trap removed its queue entry).
  Relaunched 12:13 `fh2-check2`: `check --offline --keep-going --lib --tests` over 40 crates (draw-drawing, fem-2d, fem-3d, stdio
  contract + 37 artifact crates), log `.🧬semio/🌐hub/s14-s20-overlay-build/logs/0929-121354-fh2-check2.txt` (queued behind 6).
- 12:2x **T6 overlap** (files FH2 changed ∩ files the T6 sets read/edit; LB2 via instrumented `--dry-run` on the live tree — p9's
  list is an over-approximation because it READS 3 331 files): U6 row-target 2 (stdio contract root, wav edit-audio); LB2 p9 11
  (contract root, epw/binary/wav/semio roots, ifc cobie/cv20/sav io, semio base/video/audio io); p11 14 (epw, gif ×2, xml valid
  editor+schema, png, jpg ×2, pptx ×3 + root, tiff ×2); p12 12 (10 pdf editors + pdf root + gif 89a); p10/p13/p14 0.
  JSON: `.🧬semio/🌐hub/s14-fh2-work/t6-overlap-{u6,lb2}.json`. My hunks in them are raise-site replacements + appended
  declaration blocks (pdf/pptx roots: `editor_faults` fn + the `window_kit_document_revision_matches` call sites).
- 12:52–12:53 `fh2-check2` ran: EXIT 101 — 26 errors, only 9 crates reached (the rest depend on failed ones): 8 ×
  `String: MutationMessageCode` (wav avi mp3 mp4 tsv html epw binary: framework `MutationApplyError.code: String` forwarded into
  `MutationOutcome::error`) + contract patch test `{error}` on `Fault` (no Display). Log copy `.🧬semio/🌐hub/s14-fh2-work/check2.log`.
- 12:5x fixes: all 53 stdio `apply_*_mutation` copies → framework `diff(mutation, &*snapshot).apply_to(snapshot)` (list
  `s14-fh2-work/apply-to-files.txt`; apply rejection now Fatal like the 108 other plugin sites); patch test `{error:?}`;
  `split_parent` lost its unused `path`. FH1's `describe()` hunk in the patch module kept. Relayed to S20. Re-queued `fh2-check3`.
