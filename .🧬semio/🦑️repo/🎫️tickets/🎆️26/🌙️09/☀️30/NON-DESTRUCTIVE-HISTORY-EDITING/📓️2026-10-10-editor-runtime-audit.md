# Editor Runtime Audit — 2026-10-10

Read-only source inspection; no native build, test, browser journey or runtime credit. Existing October 6 cohort reports record compilation failures and zero assertions; they do not certify current source.

## Exact Current Native Obligation

Current source census using `rg` across plugin standards finds **148 registration files**, **145 direct macro calls**, **9 child macro calls**, **444 expected named assertions**, and **96 artifact roots with laws**. The historical phrase “148 editors” means registration files, not 148 direct editors. Preserve the all-artifact obligation independently of this matrix.

`native-matrix/📜️script.ts:68–82` discovers only artifacts with standards, Rust Cargo package and a law registration; it cannot detect missing laws. Line 90 reports files; 108 counts calls; 124 requires three distinct direct assertions plus one per child law. Execute canonical `bun nx exec --projects=workspace --excludeTaskDependencies --skip-nx-cache -- bun <ticket>/native-matrix/📜️script.ts run all all`; `execute-group ... named` alone does not verify receipts.

An independent concrete `ArtifactEditor for` source census finds **149 declarations across 97 artifact roots**. Comparing editor names and artifact roots against direct and child registration arguments yields two declaration candidates absent from named macro arguments: `XlsxTransitionalEditor` at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional/✏️editor/🦀️.rs:100`, and **BimModelApp** at `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:738`. The former is covered by its multiline direct law at transitional editor unit tests lines 45–47 (`super::XlsxTransitionalEditor`), which the single-line argument census misses; the latter artifact has no direct or child macro anywhere in its standards and is a concrete current matrix coverage gap. Existing BIM undo/redo tests do not supply history-edit acceptance.

Dag, Reasoning Wires and Imperative Procedure are child-only artifact roots, with their child laws respectively at editor unit tests lines 26, 68 and 29. Their direct parent mutation workflow is excluded from the three direct assertions. Child coverage must remain explicit rather than calling it full direct-input coverage.

## Workflow Coverage And Limits

Plugin history acceptance owner `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs:408–500` explicitly checks preview equals fold through the edited mutation with downstream excluded (439), accepts, settles replay, discards blocked candidate trials by exit with no supersession (460–463), finalizes and commits overwrite or named alternative (494–500). Reload comparisons occur at 1065–1197 and 1267 onward. These are source assertions, not executed evidence.

Conflict law intentionally allows PASS with “no input change ... conflicts downstream” (1724), “no parent-lane leaf” (1749), or “no dependents” (1798), and searches finite cases (30–35). A green name therefore does not prove every editor exercised warning, fatal repair, or every leaf. `assert_input_schemas_resolve` (1420) proves schema resolution; it does not by itself prove pointer/keyboard selection or drag behavior. Keep per-leaf and per-control receipts.

Puzzle2D concrete editor at its editor owner line 4413 has direct law at unit tests line 52. Its selection-plus-offset runtime obligation is explicitly live step21, documented at dev probe lines 65–68: one drag row, reference list Use selection/removal, dx/dy grid steps, preview after each, accept downstream replay. Native macro green cannot replace this interaction evidence.

## Live Obligation

Dev probe `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️time-travel/🟦️.ts:123–124,6470` defines 24 ordered steps, not 24 editors. EN/DE × React/wgpu are independent executions of those steps. Documentation includes final choice (17–23), fatal withdrawal (24–25), editing targets to repair fatal (32–34), warning (35–37), and Puzzle2D selection/offset step21 (65–68). Existing canonical EN/DE commands are recorded in `📓️2026-10-10-current-runtime-audit.md`.

Universal step24 selects one edit-enabled history row (6230–6250), samples control roles with up to three controls per role (6260–6270), may record NOT-REACHED/NOT-OPERABLE (6260–6300), and excludes successful reference-list operation when there is no current selection. It is not an exhaustive editor/input census. Mirror inventory suppresses selection buttons/chips as separate inputs and merges controls by pointer (`🪞️inventory/🟦️.ts:24–38`). Preserve specific reference interactions and excluded/inoperable evidence.

Fresh wgpu boot failure may continue with stored native terminology for diagnosis, explicitly without fresh-profile success credit (6484–6499). No current live execution was inspected or run here. Remaining runtime obligation is all 444 strict assertions, the uncovered BIM editor, complete per-artifact/per-input coverage, and all 24 live steps in EN/DE on both renderers with actual preview, accept/discard, warning/fatal repair and final save-choice receipts.
