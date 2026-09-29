# WP-FH3 — Fault Localization Helper 3 (families C, E, F, G)

Slice: FH3 (session 14c, helper of S20; spec `📓️fault-localization-api.md`). Overlay only:
`.🧬semio/🌐hub/s14-s20-overlay-faults/`. Work dir (plans = idempotent edit scripts, fast census):
`.🧬semio/🌐hub/s14-fh3-work/` (`fam-census.ts <plugin…>` = the law's own `faultFactsOfText` + `faultLawViolations`
over the named plugins, with the cfg(test)-statement census bug worked around; `plans/<plugin>.py`).

## Session 14

| Family | Plugin | Sites (census 10:4x) | Codes declared | Violations → | Crate check |
|---|---|---|---|---|---|
| G | 🪐️space | 156 | 74 (studio engine 15, space index 6, home editor 49, home viewer 11) | 0 | pending (framework not green) |
| G | 🏛️architect | 59 | 32 (editor 32, viewer 1) | 0 | pending |
| G | 💡️reasoning | 18 (+21 raises the census bug hid) | 28 (editor 28, viewer 1) | 0 | pending |
| G | 🌿️vcs | 29 | 19 | 0 | pending |
| G | 💠️lowpoly | 32 (+17 `map_err(Fault::from)` the census does not see) | 35 | 0 | pending |
| G | 🌍️gis | 43 | 26 (terrain 7, map editor 11, map viewer 10) | 0 | pending |
| G | 📐️cad | 30 (+4 `map_err(Fault::from)`) | 19 (editor 19, viewer 1) | 0 | pending |
| G | 🗒️note | 46 | 27 (editor 27, viewer 1); unit law asserts the 4 new patch-blocks codes | 0 | pending |
| G | 🏭️process | 29 | 24 (editor 24, viewer 2) | 0 | pending |
| G | 📕️norm | 59 | 24, declared once in contract `app_surface::with_norm_faults` wrapping all 15 editors | 0 | pending |
| G | 📏️layout | 29 (+1 `map_err(Fault::from)`) | 21 | 0 | pending |

| F | 🖨️raster | 94 (+~25 hidden `map_err(Fault::from)`; 50 free-text `fault(msg)` helper sites) | 128 (editor 128, viewer 2); layer/validator codes through one `schema::edit_fault(&'static str)` map, composite errors through `RasterStackPreparation::fault` | 0 | pending |
| F | 🀄️wfc | 93 | 66 (2d 14, grid2d 10 + viewer 5, bitmap 19, 3d 8, grid3d 10) | 0 | pending |
| F | ➗️mathematical | 68 | 42 (editor 42, viewer 1) | 0 | pending |
| F | 📋️forms | 62 (+1 hidden) | 60 (editor 60, viewer 1); shared-vector reasons mapped by `placement_fault`/`patch_fault`/`choice_fault` (vectors unchanged) | 0 | pending |
| F | 🔱️trinity | 97 | 60 (jack 47 + viewer 2, rewriting 21 + viewer 1); jack + rewriting unit laws assert the 5 new patch-nodes/add-rule-clause codes | 0 | pending |

| E | 🌊️flow | 184 (+ extensions' Plugin-origin `extension.*` raises, +1 hidden) | 111 (editor 111, viewer 3) incl. 5 `extension.*` codes the 9 flow extensions raise | 0 | pending |
| E | 🌀️procedural | 123 (+~12 hidden) | 91 (generation2d 34 + viewer 3, generation3d 43 + viewer 27) | 0 | pending |
| E | 🎬️sequence | 101 | 70 (editor 70, viewer 1) | 0 | pending |

| C | 🧩️puzzle | 340 (+2 hidden `map_err(Fault::from)`) | 265 (2d 44+2, 3d 86+2, 5d 133+2 shared `puzzle-command-*`); texts from `plans/puzzle_texts.py` (task × condition templates + 60 handcrafted) | 0 | pending |

**ALL FOUR FAMILIES: official `verify faults` over the 20 FH3 plugins 11:4x → violations=0, oracle agrees** (tree: raises
3 742, declarations 1 916, catalog 476). Crate checks (`overlay-cargo.sh … check --offline --lib --tests -p <crates>`) wait for
S20's "framework green" (framework check `0929-104750-framework-lib` still QUEUED at 11:4x).

Family E official census 11:3x: 0 in E; the S20 code-based census then surfaced 11 `ArtifactInferenceExecutionError`
`FaultCode::new(lit)` raises in G (gismap inference ×10, cad aec-building ×1) → declared, G back to 0.

Family F official census 11:2x (F + G plugins named): **violations=0, oracle agrees** (tree: raises 3 199, declarations 1 105,
catalog 476).

Family G official census: `bun ./📜️script.ts verify faults <11 G plugins>` on the overlay 11:1x → **violations=0, oracle agrees**
(whole tree then: raises 2 892, declarations 481).

### Log

- 11:0x read preamble (rules 1–27), spec, census files. Framework compile `0929-104750-framework-lib` QUEUED.
- 11:1x FOUND census bug (relayed to S20): `rustProductionCode` blanks a statement-level `#[cfg(test)]` up to the next
  `{` even inside a string literal (reasoning wires `✏️editor/🦀️.rs:307` `eprintln!("[TRACE] … {}")`) → maskLiterals
  desyncs → 21 raises of that file invisible. My fam-census drops such cfg(test) print statements before scanning.
- 11:1x family G done (fam-census + official `verify faults` 0). Method: `plans/<plugin>.py` (exact anchored replacements,
  idempotent) + `autoconv.py` (literal-message `Fault::new(App, FaultCode::new(lit), "…")` → `app_fault(lit)`, bare error →
  `reason` parameter). Conventions: internal detail → `{reason}`; codes raised with different parameter sets split into
  distinct codes (e.g. lowpoly `media.import-empty/-encoding`, gis viewer `camera-missing/-unreadable/-out-of-range`, space
  `media.format-unknown`); `mutation.target-missing` keeps one parameter per plugin; shared codes declared identically on
  editor + viewer (child projection, initializer close, home directory-page family). Asked S20: MutationMessage codes
  (`&str` → FaultCode has no From on the overlay).
- NOTE for landing (S20): overlapping files with U6 T6 row target — cad `✏️editor/🦀️.rs` + config, process3d `✏️editor/🦀️.rs`;
  norm en1994/din4108 editors are edited live by a Cursor peer (my change there is one anchored line each).
- 11:2x family F done (0 official). New codes where one code carried different parameter sets or free-text helpers
  (`fault(msg)` in raster edit-pixels/selection → `raster.pixel-edit.*` / `raster.pixel-selection.*`, forms try-value
  helper, trinity patch-nodes). Tests updated to the new codes: trinity jack/rewriting unit (patch-nodes, add-rule-clause).
- 11:3x family E done (0). Tests switched to codes: flow add-widget (code + `reason` parameter), lowpoly refusal helper +
  delete laws, cad unknown-example, raster merge-down/duplicate-layer/edit-mask, space create-studio. INCIDENT (fixed): re-running
  `plans/forms.py` tripled the inserted `placement_fault`/`patch_fault`/`choice_fault` blocks (rep() re-applied because the
  new text contains the anchor) → deduplicated, rep() now skips when the new text is present.
- 11:4x family C done (0); all 20 plugins 0 in the official census. Pre-compile static checks: `into-scan.py` (text→Fault
  `.into()`/`ok_or("…")?` in Fault-returning fns: 3 hits, all inside String-returning closures = false positives); fixed one
  `&str.as_str()` (cad model-definition) before any compile. Known compile risks left for the crate check: `?`/`.into()` from
  String errors into `Fault` that no scan can see (the compiler lists them), fn-local imports in files with nested modules.
- NEXT (on S20 "framework green"): `overlay-cargo.sh fh3-<family> check --offline --lib --tests -p <crates of the family>`,
  fix every error/warning I introduced, re-run the census, record rc per plugin here.
- 12:03–12:50 crate check 1 (all 4 families in one batch: `overlay-cargo.sh fh3-all check --offline --keep-going --lib --tests
  -p <74 crates>`, log `.🧬semio/🌐hub/s14-s20-overlay-build/logs/0929-120339-fh3-all.txt`, queued 12:03, ran 12:48:52–12:50:12)
  → **EXIT 101, blocked upstream**: 8 stdio crates (wav, avi, mp3, mp4, tsv, html, epw, binary — family B/framework) fail
  `MutationOutcome::error(error.code: String, …)`: `MutationMessageCode` has no impl for `String`. Only the norm crates (15 of
  16, not din18599) and space-space were reached → **0 errors in my code there**; unused-import/variable warnings in them are
  pre-existing (live has the same lines). All other FH3 crates NOT REACHED. Relayed to S20 12:5x. Before the run: dropped the
  now-unused `entity` parameter of wfc2d `require_fresh` and the unused `argument` loop binding in architect adjacency.
- NEXT: re-queue the same batch once S20/FH2 fix the stdio `MutationMessageCode` break.
