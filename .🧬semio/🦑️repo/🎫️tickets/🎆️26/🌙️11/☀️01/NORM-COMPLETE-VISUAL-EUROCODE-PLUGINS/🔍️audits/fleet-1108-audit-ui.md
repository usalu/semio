# 🔍 Fleet 1108 Audit UI: Norm Artifact Editors and Viewers (read-only)

Lane `audit-ui`. Read-only audit against `📌️important/artifact-rollout-checklist.md` steps 6, 7, 8, 8b-8e. No source edited.
Scanner: `TICKET/🧰️scripts/audit-ui-matrix.py` (python3 -I, heuristic greps over `✏️editor` and `👁️viewer`, non-test files).
Base path for every row: `/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/<ARTIFACT>/🏅️standards/🔖️1/🪆️subsets/✳️any`

Legend: E = editor, V = viewer. "5/5" = viewport, diagrams, properties, checks, outliner present as window dirs under `🎭️modes/*/🪟️windows/`. Live = `inferences::live` used in E/V. Macros = count of the seven DM/live macros (`norm_live_editor!`, `norm_live_acceptance!`, `norm_live_commands!`, `norm_lane_stores!`, `norm_exact_store_ownership!`, `norm_command_from_action!`, `norm_viewer_surface!`).

## 1. Artifact x item matrix

| Artifact | E windows | V windows | Legacy dirs | render_document_* / JSON body | DE labels (native) | Single-language labels | a11y (accessible_name in subset) | Live (`inferences::live`) | Live macro in inferences | Macros (of 7 incl. viewer) | Escape keybinding | Incremental units (`🧮️units`) | Scope (8b) | Field meta | Mutation `resolve` (step 5) | Scene dir |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ⚖️en1990 | 5/5 | 3/3 | none | 0 / 0 | 78 | none | 2 | yes (10) | yes | 7/7 | yes | yes | no | yes (E) | yes | yes |
| 🏋️en1991 | 5/5 | 3/3 | none | 0 / 0 | 79 | none | 2 | yes (10) | yes | 7/7 | yes | yes | no | yes (E, 346 specs) | yes | yes |
| 🏛️en1992 | 5/5 | 3/3 | none | 0 / 0 | 79 | none | 2 | yes (10) | yes | 7/7 | yes | yes | no | yes (E) | yes | yes |
| 🔩️en1993 (reference) | 5/5 | 3/3 | none | 0 / 0 | 77 | none | 1 | yes (10) | yes | 7/7 | yes | yes | yes | yes (E, 839 rows) | yes | yes |
| 🧩️en1994 | 5/5 | 3/3 | none | 0 / 0 | 77 | none | 1 | yes (10) | yes | 7/7 | yes | yes | no | yes (E) | yes | yes |
| 🪵️en1995 | 5/5 | 3/3 | none | 0 / 0 | 78 | none | 1 | yes (10) | yes | 7/7 | yes | yes | no | yes (E) | yes | yes |
| 🪨️en1996 | 5/5 | 3/3 | none | 0 / 0 | 78 | none | 1 | yes (10) | yes | 7/7 | yes | yes | no | yes (E) | yes | yes |
| 🌍️en1997 | 5/5 | 3/3 | none | 0 / 0 | 78 | none | 2 | yes (10) | yes | 7/7 | yes (mod+z, mod+shift+z, escape) | yes | no | THIN (E, 2 FieldSpec rows, delegates to checks lookup) | yes | yes |
| 🫨️en1998 | 5/5 | 3/3 | none | 0 / 0 | 78 | none | 2 | yes (10) | yes | 7/7 | yes | yes | no | MISPLACED (at `🏷️field-meta` under subset root, not under `✏️editor`) | yes | yes |
| 🪶️en1999 | 5/5 | 3/3 | none | 0 / 0 | 77 | none | 1 | yes (10) | yes | 7/7 | yes | yes | no | yes (E) | yes | yes |
| 🌬️din16798 | 0/5 (only legacy) | 0/3 (only legacy) | inputs, results, report | 0 / 0 | 21 | "Inputs"/"Results" hard-coded (E `🎭️modes/✏️edit/🦀️.rs:16`) | 0 | NO (evaluate at E `🦀️.rs:233-234`, V report `🦀️.rs:31`) | no | 2/7 | NO (keybindings: mod+z/redo only) | no | no | yes (E, 52 rows) | yes | NO scene |
| ⚡️din18599 | 0/5 (only legacy) | 0/3 (only legacy) | inputs, results, report | 0 / 0 | 21 | not verified (window literals not scanned) | 0 | NO (evaluate at E `🦀️.rs:243-244`, V report `🦀️.rs:31`) | no | 2/7 | NO | no | no | MISPLACED (E `🎭️modes/✏️edit/🏷️field-meta`, 102 rows) | yes | NO scene |
| 🧱️din4108 | 0/5 (only legacy) | 0/3 (only legacy) | inputs, results, report | 0 / 0 | 21 | not verified | 0 | NO (evaluate at E `🦀️.rs:235-236`, V report `🦀️.rs:31`) | no | 2/7 | NO | no | no | yes (E, 118 rows) | NO (0 files) | yes (unused by editor) |
| 📇️iso16757 | 0/5 (only legacy) | 0/3 (only legacy) | inputs, results, report | 0 / 0 | 21 | "Inputs"/"Results" hard-coded (E `🎭️modes/✏️edit/🦀️.rs:16`) | 19 aria hits, 0 accessible_name | NO (evaluate at E `🦀️.rs:231-232`, V report `🦀️.rs:31`) | no | 2/7 | NO | no | no | yes (E, 227 rows) | yes | NO scene |
| 🏭️vdi3805 | 0/5 (only legacy) | 0/3 (only legacy) | inputs, results, report | 0 / 0 | 21 | not verified | 0 | NO (evaluate at E `🦀️.rs:302-303`, V report `🦀️.rs:31`) | no | 2/7 | NO | no | no | yes (E, 112 rows) | yes | NO scene |

Notes on the matrix:
- No `render_document_editor` or `render_document_json` usage in any of the 30 editor/viewer trees.
- No JSON dumping in editor or viewer bodies detected (`to_string_pretty`, `serde_json::to_string`, `json!`, `<pre`, and "json" mentions in `🎭️modes` windows all zero).
- Single-language labels: only the two English layout titles above are confirmed. A broader literal scan flags 10 to 20 English literals per artifact, but those are mostly bilingual catalogue rows (`label_en`/`label_de`) and field-meta choices (3-argument form), so they are false positives. Needs a human pass before claiming completeness.
- Step 8b Scope (`struct Scope`, `check_scoped`) exists only in en1993. All other live artifacts use the 8b-b `🧮️units` form instead (`UnitSource` implementations present). Acceptable per the brief, but the checklist text for 8b is not met verbatim.

## 2. Gap ranking per artifact (worst first)

1. 🌬️din16798: no viewport, diagrams, properties, checks or outliner window in E or V; legacy `📥️inputs`, `📊️results`, `📊️report` still present; no live wiring; no live macros (2/7); no escape keybinding; English-only window titles at `✏️editor/🎭️modes/✏️edit/🦀️.rs:16`.
2. ⚡️din18599: same as din16798 (legacy windows, evaluate-based viewer, no live wiring, 2/7 macros); field meta misplaced under `✏️editor/🎭️modes/✏️edit/🏷️field-meta`.
3. 🧱️din4108: same legacy set; additionally no `pub fn resolve` in `🧬️schema/🧬️mutations` (step 5 not done, so handles cannot commit); scene dir exists but nothing wires it into the editor.
4. 📇️iso16757: same legacy set; English-only window titles at `✏️editor/🎭️modes/✏️edit/🦀️.rs:16`; 19 aria/accessibility hits but no `accessible_name` in the scene.
5. 🏭️vdi3805: same legacy set and gaps as din4108 minus the unused scene dir.
6. 🔩️en1993 (reference): path budget violations at 6 mutation fixture files (`🧪️tests/🧫️fixtures/🧬️mutations/🧱️change-material-ductility-ultimate-strain/✅apply/🔣️.*.json`, `🧱️change-buckling-length-column-stiffness-a/b`); source budget OVER in three crates (see section 3). Otherwise full visual, live, macros, keybinding.
7. 🏋️en1991: 10 path budget violations (longest 245 bytes, `⚙️change-dust-explosion-static-opening-pressure/✅apply/🔣️.mutation.json`); otherwise complete.
8. 🫨️en1998: `🏷️field-meta` sits at the subset root instead of `✏️editor/`; no editor-side field meta at the expected location.
9. 🌍️en1997: field meta is thin (15-line file delegating to `checks::lookup_field_meta`); verify coverage of every handle write path (step 8).
10. ⚖️en1990, 🏛️en1992, 🧩️en1994, 🪵️en1995, 🪨️en1996, 🪶️en1999: complete on this audit's heuristics (5/5 editor, 3/3 viewer, live, 7/7 macros, escape keybinding, units). Only the 8b Scope gap noted above.

## 3. Path-length and source-budget results

Path budget (`✏️s/🔌️plugins/📕️norm/⚖️compliance/🧪️tests/📏️path-budget/🐍️.py`, budget 240 bytes): exit 1. 61385 paths scanned, longest 245 bytes, 16 over budget.
- 🏋️en1991: 10 paths. Mutation vectors under `🧪️tests/🧫️fixtures/🧬️mutations/⚙️change-dust-explosion-static-opening-pressure/✅apply/` (`.mutation`, `.outcome`, `.before`, `.after`, `.diff` JSON, 241-245 bytes), plus `⚙️change-rail-load-group-assumed-longitudinal` and `⚙️change-rail-load-group-factor-longitudinal` (241-243 bytes).
- 🔩️en1993: 6 paths under `🧪️tests/🧫️fixtures/🧬️mutations/🧱️change-material-ductility-ultimate-strain/` and `🧱️change-buckling-length-column-stiffness-a/b` (241-242 bytes).
- Tool output was not kept; rerun the command to reproduce.

Source boundary (`TICKET/🧰️scripts/source-boundary-scan.py`, 262144 bytes / 32 files / 4000 lines): exit 0, 3 crates over budget, all over on file count:
- `semio-s-artifact-norm-en1993-mut-fastened-joints`: 96140 bytes, 67 files, 1670 lines. Path `…/🔩️en1993/…/✳️any/🧬️schema/🧬️mutations/🪛fastened-joints/📦️packages/🦀️rust`.
- `semio-s-artifact-norm-en1993-mut-buckling-lengths`: 48127 bytes, 34 files, 815 lines. Path `…/🔩️en1993/…/✳️any/🧬️schema/🧬️mutations/🪜buckling-lengths/📦️packages/🦀️rust`.
- `semio-s-artifact-norm-en1993-mut-fatigue-stresses`: 46951 bytes, 34 files, 818 lines. Path `…/🔩️en1993/…/✳️any/🧬️schema/🧬️mutations/⏳fatigue-stresses/📦️packages/🦀️rust`.
No other crate exceeds the budget.

## 4. Reproduce

`python3 -I TICKET/🧰️scripts/audit-ui-matrix.py` prints the per-artifact raw counts.
