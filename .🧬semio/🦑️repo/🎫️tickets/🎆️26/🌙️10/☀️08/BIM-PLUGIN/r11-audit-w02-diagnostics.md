# 🔎️ R11 Audit — w02-diagnostics (WP-02) and u-fix

Status: WP-02 **PARTIAL** (data layer present; panel, chrome count, labels, JSON export, index wiring missing).
u-fix **PARTIAL** (9 fixed, 2 partial, 4 not fixed, runtime verification not done).

## WP-02
| Item | Present | Wired | Evidence |
|---|---|---|---|
| Per-element index + severity table | Yes | No | `⚠️diagnostics/🗂️index/🦀️.rs` `DiagnosticIndex` + unit test; `ModelInference` has only `diagnostics: Vec<Diagnostic>` (`💡️inferences/🦀️.rs:52`); editor calls only `compute_diagnostics` (`✏️editor/🔮️inference/🦀️.rs:59`) |
| RoofFallback code | Yes (5 codes) | Yes | `📐️validity/🦀️.rs:250-257`, messages 84-96 en+de |
| Panel `📌️panels/🚨️diagnostics` | No | No | — |
| Status count in `🎛️chrome` | No | No | `status()` line 31 |
| en+de panel labels | No | — | — |
| JSON export (`🚪️io/📝️text`) | No | No | — |
| CSV export | Yes | Yes | `🚪️io/📤️export/📊️csv/🦀️.rs` (`diagnostics_csv`, `report_csv`); its `#[path]` test file missing |
| .feature clean/defects | Yes | Yes | `🧪️tests/🗺️infer-bim-1-plan-and-diagnostics/🥒️.feature` |
| Shapely clash cross-check | Yes | Yes | `…/🐍️.py`, not re-run |

## Missing `#[path]` test targets (break `cargo test --lib`)
1. `💡️inferences/📋️schedules/🦀️.rs:126` 2. `✏️editor/…/🪟️windows/🧮️schedule/✏️edit/🦀️.rs:158`
3. `✏️editor/📌️panels/🌳️outliner/🖼️views/🦀️.rs:64` 4. `✏️editor/🧩️entities/🕰️phasing/🦀️.rs:47`
5. `🚪️io/📤️export/🏗️ifc/🔲️ceilings/🦀️.rs:61` 6. `🚪️io/📤️export/📊️csv/🦀️.rs:118`

## Last logs
w02 check0 (22:56): 11 editor errors (schedule window lifetimes, `✏️editor/🦀️.rs:589` arity) — absent later, probably fixed.
u-fix check8 (23:31): `🧵️gestures/🧱️chain/🦀️.rs:120` arity; `🕸️model-graph/🧮️compute/🦀️.rs:302,304` arity.
Coordinator 00:42: lib compiles; tests fail on missing schedules test file only (first one found).

## u-fix (r9-audit-ui items)
FIXED: 4 world preview, 5/15 hidden storeys, 6 roof shape/stair flight writable, 7 engagement_input, 10 hotkeys,
13 app_labels, 14 viewer labels, 16 outliner docstring, 17 projection validation.
PARTIAL: 8 (missing `SetElementClassification`/`RemoveElementClassification` commands), 11 (no arrow-key cursor).
NOT FIXED: 12 accessible names (`🧰️kit/🦀️.rs:224` canvas_surface, schedule surface), 18 schedule window vs panel,
19 hard-coded "wall" domain_granularity (`🧊️world/🦀️.rs:174`, `🧮️schedule/🦀️.rs:97`), 20 fifteen `📌️.empty.md`
placeholders. 21 runtime verification NOT DONE.

## Remaining tasks
WP-02: wire `DiagnosticIndex` into the model graph/ModelInference; panel `🚨️diagnostics` (group by storey/kind, severity
filter, click-select, registered in mount + command table); chrome count; en+de labels; JSON export; CSV `.feature`;
CSV test file; gate + shapely oracle.
u-fix: classification commands; accessible labels; arrow-key cursor; schedule stays a window (coordinator ruling);
per-row domain granularity; remove unused placeholders; runtime check.
