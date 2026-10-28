# IFC4x3 Exchange Follow-Up Fixes

## Results

Both new defects in `exchange-reaudit.md` are fixed within the existing supported projection subset. The retained, unmodified `exchange-reaudit-repro.ts` now reports:

```json
{"test":"cut_height_mm","fidelity":"partial","height":1.2,"unchanged":true}
{"test":"shared_section","fidelity":"partial","ids":["ifc50"],"unchanged":true}
```

Its third case now successfully reexports the projected shared-section FEM artifact; it previously refused duplicate section identities. The executable regression additionally reimports that reexport and verifies one section, one material, two member references to each shared definition, and two unique support instances.

Changed production files: IFC4x3 any-subset `🚪️io/bim.ts` and `🚪️io/fem.ts`. Changed tests: `🧪️tests/bim.test.ts`, `fem.test.ts` and language-neutral `exchange.feature`. The coordinator's additions to `oracles.test.ts` were preserved. Retained ticket input script `exchange-followup-oracle.ts` and test-only `ifc-oracle.py` provide independent comparison. No facades, package suites, AGENTS, git state, worktrees or build infrastructure were modified by this worker.

## Explicit BIM Units

Authored storey properties retain their complete property records while resolving values, including `IfcPropertySingleValue.Unit`. `Height` and `CutHeight` require a typed length measure. A valid explicit unit overrides project length units and is converted using the existing scale-and-seven-dimension resolver. An absent explicit unit uses project length scale. Thus 1200mm becomes 1.2m and 3000mm becomes 3m even when project units are metres. Nonlength units, mismatched measure types, nonfinite conversion results and duplicate authored property names are refused before publication. Ordinal `Level` requires a typed integer with no physical unit. Multiple conflicting authored storey definitions are refused; repeated relations to the same property-set identity do not create ambiguity.

Existing placement checks still run after conversion. Converted storey heights must agree with IFC placement/elevation and cumulative authored levels. This fix does not relax unsupported spatial transformations.

## Canonical Shared FEM Definitions

Section import caches destination records by source property-set identity. Two members referencing the same section property set receive the same canonical section ID and one destination section record. Section export also caches the emitted property set by source FEM section ID, so export/reimport preserves sharing instead of expanding one shared section into separate definitions.

Materials were already emitted/imported once per material identity. Analogous paths now reject multiple distinct material assignments for one member and conflicting same-name engineering properties, while accepting repeated references to the same property identity across material property containers. Multiple distinct section definitions for one member are refused instead of selecting the first. Ratio/text properties reject inappropriate physical units. The explicit SI requirement for FEM dimensional engineering properties remains unchanged: supported SI values are admitted; unsupported non-SI engineering properties are refused rather than reinterpreted.

A shared boundary-condition definition now creates one support **instance per structural connection**, using the connection identity rather than the reusable condition identity. Two nodes sharing one condition therefore have distinct valid support IDs with the same restraints. Export also validates uniqueness of support IDs, load-case IDs and case-scoped load IDs. Additional material/member checkpoints retain cooperative progress and cancellation before destination publication; a regression cancels during canonical definition projection and verifies source immutability.

## Test-First and Independent Evidence

Six initial new regressions were executed before production fixes and all failed: five explicit BIM property variants and the shared section/material/boundary case. The valid millimetre case returned height3000 instead of3; incompatible unit/measure and duplicate property cases silently succeeded; shared section import returned two records for one ID. They now pass, along with analogous material sharing/conflict tests.

Final combined exchange run with `SEMIO_IFC_ORACLE_OUTPUT` enabled: **62 passed, 0 failed, 265 assertions** across five executable files. This includes the coordinator's shared editor geometry oracle and all earlier adversarial analytical/spatial guards.

IfcOpenShell **0.8.3** independently parsed and schema-validated the following generated source cases with zero diagnostics:

| Case | IFC entities | Independent assertions |
| --- | ---: | --- |
| `bim-mm.ifc` | 38 | Native IfcLengthMeasure plus explicit millimetre unit converts with IfcOpenShell unit utilities to height3m and cut-height1.2m |
| `fem-shared.ifc` | 67 | Two members share exactly one section property-set identity and one material; both nodes use fixed boundary conditions |
| `fem-shared-roundtrip.ifc` | 74 | Export/reimport retains one section definition and both member references; supports remain valid on both nodes |

The BIM/FEM tests also import the IfcOpenShell-reconstructed STEP outputs and assert the same destination values and canonical identities. All valid and invalid local regression cases check that the source graph remains unchanged.

From repository root, reproduce with:

1. `bun <ticket>/exchange-followup-oracle.ts`.
2. For `bim-mm`, `fem-shared` and `fem-shared-roundtrip`, run `python <ticket>/ifc-oracle.py <ticket>/🗑️generated/ifc083 <ticket>/🗑️generated/ifc-inputs/<name>.ifc <ticket>/🗑️generated/ifc-results` using the bundled Python3.12 executable documented in `exchange-implementation.md`.
3. Set `SEMIO_IFC_ORACLE_OUTPUT=<ticket>/🗑️generated/ifc-results`; run the IFC4x3 IO test directory with Bun.

All generated sources/dependencies/results remain in the shared ticket generated directory for coordinator verification. Direct commands use the already documented cached repo CLI limitation fallback.

The final focused strict TypeScript invocation reported no errors in changed BIM/FEM production modules. It remains blocked by concurrent newly optional `entity.complex` accesses in STEP/catalog and the pre-existing missing `@semio-tech/framework-interaction` workspace module. The coordinator was notified of those separately owned errors.

## Boundaries

The changes correct unit semantics and definition identity for already supported projections. Full authored BIM family exchange, general foreign analytical mappings, new geometry evaluators, EXPRESS/MVD validation and strict per-step allocation budgets are not introduced or claimed by this follow-up.
