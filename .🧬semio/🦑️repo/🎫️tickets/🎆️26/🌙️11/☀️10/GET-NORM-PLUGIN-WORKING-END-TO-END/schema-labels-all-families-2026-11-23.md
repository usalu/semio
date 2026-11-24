# Schema labels in all Norm families (2026-11-23)

Goal: every mutation payload schema passes the input reader (`history_edit_inputs_resolve`). Audit: `audit-schema-labels.py` (mimic of the input reader; run `python3 audit-schema-labels.py [family]`), now 0 findings in all 15 families (before: en1990 11, din18599 1, din16798 10, en1991 7, en1992 25, en1996 6, en1995 2, en1998 38; en1999 was already 0 from the other agent, untouched here).

## Changes (schema-first, JSON only)
- Enum option labels of mutation payloads copied verbatim from the snapshot enum (`add`: `apply-option-labels.py`): en1990 `change-seismic-importance-class`, `change-traffic-group-group`; en1992 `change-member-kind`, `-punching-beta-method`, `-support`; en1996 `change-bracing-system`, `change-chase-kind`; en1995 `change-uls-equation` (the options were keyed `6.10` / `6.10ab` and did not match the enum `eq610` / `eq610ab`, replaced); en1998 `change-building-storey-stiffness-basis`, `change-member-effective-stiffness-basis`, `change-member-role`.
- Embedded-record field labels in `🧬️schema/📸️snapshot/🔣️.json` (`add-schema-labels.py`): en1991 (`ConstructionLoad.stage`, `ConstructionStage.activity`, `CraneHoist.releaseDevice`, `RoofSurface.face/shape`, `SpecialVehicle.designation`, `RailDynamic.maintenance`, wording of the matching `change-*` mutations), en1992 (`BarPoint.bend`, `ReinforcementGrade` 5 fields, `ConcreteGrade.fCk`, `PrestressSteel` 2 fields, strengths as Pa shown in N/mm² like the existing mutations), en1998 (28 fields of `En1998CriticalRegion`, `En1998IsolationSystem`, `En1998PierBase`, labels taken from the existing `change-*` mutations), din16798 (`UnoccupiedVentilation` 6 fields and 4 operation options, DIN EN 16798 terms), din18599 (`DhwSystem.dwellingCount`).
- en1990 `FactorOverride.factor` carried `options` without `enum` (input reader: "options only label enum values", found by the test, not by the audit): `enum` added from the option keys.

## Test infrastructure needed to run the law (not label content)
- The editor crates of en1990, en1991, en1992, en1995, en1996 exceeded the closed source boundary (33 of 32 files); they got the same `integration` target split the other families already have (`wave-tests-editor-split.py`, `wave-tests-editor-imports.py`).
- Editor unit tests of those families: `store::`/`protocol::` paths to `semio_os_kernel::`, `<..PlayApp as semio_plugin::ArtifactEditor>::examples()` (same fix as en1997; applied to every family unit test file that still had them), missing `checks` import in `context`, `view_state = Some(ViewModel{..}.into())`, en1990 assets test `scene_of(.., ViewOptions::none())`.

## Test results (`repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-<fam>-editor --offline history_edit_inputs_resolve`)
| family | result |
|---|---|
| en1990 | `unit_tests::context::history_edit_inputs_resolve ... ok`, 1 passed |
| en1991 | `unit_tests::history_edit_inputs_resolve ... ok`, 1 passed |
| en1995 | `unit_tests::history_edit_inputs_resolve ... ok`, 1 passed |
| en1996 | `unit_tests::history_edit_inputs_resolve ... ok`, 1 passed |
| en1998 | `unit_tests::history_edit_inputs_resolve ... ok`, 1 passed |
| din16798 | `unit_tests::history_edit_inputs_resolve ... ok`, 1 passed |
| en1992 | not run: `semio-s-artifact-norm-en1992-diff` does not compile (`En1992MembersOpeningsRows: serde::Serialize/Deserialize` not satisfied under `test-support`, `Opening` row of the model), another writer's area |
| din18599 | not run: `semio-s-artifact-norm-din18599-model` does not compile (`crate::document::child_identity_oracle` at `🧬️schema/📸️snapshot/🦀️.rs:65`, in-flight edit by another writer) |

The en1992 and din18599 schemas are audit-clean; rerun the command above once those crates compile (a payload defect the audit does not mimic, like the en1990 `options` case, would only show there).
