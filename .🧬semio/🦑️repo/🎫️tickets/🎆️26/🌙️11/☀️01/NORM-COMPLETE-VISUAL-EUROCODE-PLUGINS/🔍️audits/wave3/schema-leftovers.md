# Schema leftovers (W3-SCHEMA-LEFTOVERS)

No cargo/rustc/`repo run`, no git writes. The `repo` MCP server was not connected, so no ticket call was possible; work stays in this ticket.

## 1. en1997 set-field kinds
The kinds `change-characteristic-basis-{layer-id,parameter,basis}` already exist (schema/mutation folders and fixtures). The remaining `verify set-fields` gap (3 fields) is only the taxonomy rows, which `schema twins generate` writes. The leaf-kinds driver `en1997.py` was not applied: its dry run stages 22 unrelated kinds and stops on a missing EN+DE label of `uplift.permanent_stabilizing` (another owner's field). `change-layer-cpt-nkt` is covered by the same twins run. Batched script: `🧰️scripts/schema-leftovers-twins.sh` (generate then `check --strict`, serial, each behind heavy.sh, for en1997, en1994, en1998, en1999); log `🗑️generated/schema-leftovers/twins.log`.

## 2. ui-steppers
Fixed in the schema JSON (en1994, en1998, din4108): `evolve.py verify steppers` now reports 0 findings repo-wide (was 16, the 21 included ones fixed meanwhile).
- en1994: `CharacteristicAction/SlabAction.qAreaPa` label Area load q_k / Flaechenlast q_k, Pa shown as kN/m2 (factor 0.001, step 100, precision 2); `CompositeBeam.crackedLengthFraction` step 0.05, precision 2; `CompositeSlab.mFactor` unit N/mm2 (value 180), step 5, precision 0.
- din4108: `LayerDocument.mu`, `LayerSegment.mu` step 0.5, precision 1; `EnvelopeElement.layers` label Layers / Schichten (index description removed). din4108 has no twin folders.
- en1998: `System.baseShearResistanceN` and `Storey.permanentGkN` (kN display, step 10 kN), `Storey.stiffnessX` (kN/m, step 1000 kN/m, like stiffnessY), `Building.planRegular/elevationRegular` toggles with own labels, `Building.masonryWallAreaRatio` step 0.005, precision 3; all labels EN/DE.
- en1999 `bearingLength` was fixed earlier (step 0.005); only its twins run is in the batch.
Twins of en1994/en1998 are in the batch (field-meta labels derive from the schema).

## 3. en1991 constants
`FATIGUE_MODEL` was already spelled `_FLOATS`; `SUPPORT_COLLISION` (2 uses, lines 109 and 213 of the SQLite Rust codec) was not. `repair` did not cover it: new rule `l_repair.float_constants` (uses renamed to the declared `_FLOATS` name), applied with `evolve.py repair en1991 --apply`; `syntax.py` reports no undeclared constant. Note: that apply also ran the existing selector repairs of the rail-load-group and dust-explosion mutation vectors (set-field vectors address the row id and value of their before/after pair).

## 4. en1993
The `frameActions` read loop exists now. The remaining defect: declarators renamed by the clone (`frameActionRows`, `frameMemberBaseRows`) while the read loops kept `frame_action` / `frame_member_basis` (TS2304). The root cause in `l_codec.unique_ts` (collision rename) was not reproduced, so no generator fix; new repair rule `l_repair.renamed_declarators` gives the declarator the loop's name again (unused `...Rows` declarator with the best name match). Applied to the en1993 SQLite TypeScript; `repair all` is empty. The file is also edited by the en1993 owner; the apply was one read-modify-write.

## Tests
`test_evolve.py` (WaveSeven): `test_repair_renames_float_constants_to_the_declared_spelling`, `test_repair_restores_renamed_row_declarators` green, plus the existing repair tests.

## Unverified by compiler
The renamed constants and declarators; the field-meta/twins of the schema label changes until the batch finishes (see below).

## Twins batch result
