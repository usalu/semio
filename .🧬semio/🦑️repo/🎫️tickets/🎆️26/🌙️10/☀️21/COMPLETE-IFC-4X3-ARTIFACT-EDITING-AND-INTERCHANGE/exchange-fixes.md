# IFC4x3 Exchange Independent Review Fixes

## Scope

Responds to `exchange-review.md` and the retained executable `exchange-review-repro.ts`. Changes stay within the existing supported TypeScript exchange subset. Unsupported foreign semantics are refused atomically rather than approximated or silently omitted. No facades, package suites, AGENTS, build infrastructure, worktrees or git state were modified by this fix worker.

Production files changed under the IFC4x3 any-subset IO root: `geometry.ts`, `fem.ts`, `bim.ts`, `step.ts`. Tests changed: `🧪️tests/exchange.feature`, `fem.test.ts`, `bim.test.ts`, `step.test.ts`, `oracles.test.ts`. Retained ticket `ifc-oracle.py` was extended to assert actual physical unit dimensions independently.

## Test-First Evidence

Added the language-neutral foreign-input scenarios before implementation. Initial executable BIM/FEM regression run confirmed the old importer returned `partial` artifacts for the adversarial cases; all six BIM spatial cases and the eight original FEM cases failed their required atomic-refusal assertions. Additional regressions now cover member auxiliary conditions, supported length, condition axes, incorrect SI name/measure type and duplicate load assignment.

## Five Confirmed Defects

| Audit repro | Original wrong result | Fixed result |
| --- | --- | --- |
| Inertia unit labelled m4 but built from metre^1 | Successful FEM partial artifact | `unsupported`, `exchange.unit-dimensions`; no artifact |
| Sole analysis has no item membership assignment | One fabricated analysis member | `unsupported`, `fem.membership`; no artifact |
| Valid IfcRelAssignsToGroupByFactor with factor 2 | Load case returned with every load removed | `unsupported`, `fem.group-factor`; no artifact |
| Site placement translated +100m in X | Building origin remained `(1,2)` | `unsupported` with `bim.site-placement` in diagnostic; no artifact |
| Ground storey placement raised to Z=99 | Returned authored Level=0 | `unsupported` with `bim.storey-placement` in diagnostic; no artifact |

Executed the original unmodified audit repro after fixes:

```json
[["inertia_m1_mislabeled_m4","unsupported"],["no_analysis_members","unsupported",null],["group_factor_2","unsupported",null],["site_translate100","unsupported",[]],["storey_translate99","unsupported"]]
```

Command from repository root: `bun <ticket>/exchange-review-repro.ts`. It reads the retained generated independent-oracle baseline inputs. No original repro source changes were needed.

## Physical Unit and Analytical Validation

`IfcGeometryGraph.unitMeasure` now returns a scale plus a seven-component physical dimension vector in EXPRESS order: length, mass, time, electric current, thermodynamic temperature, amount of substance, luminous intensity. SI names must match their physical UnitType. Derived-unit element dimensions are multiplied by their integral exponents and summed, and the resulting vector must match the declared derived type. Conversion-based units require matching declared exponents, nominal measure type dimensions and factor unit dimensions. Cycles, unsupported unit identities/types, invalid exponents and nonpositive/nonfinite scales are refused. `unitScale` delegates to this dimension-preserving resolver.

FEM engineering properties require compatible typed nominal measures, explicit SI unit categories, dimensions and scale. Load forces, moments and UDLs require exactly one compatible project SI FORCEUNIT, TORQUEUNIT or LINEARFORCEUNIT when used. Incorrect pressure-unit identities or a length measure masquerading as a torsional constant are refused.

The selected analysis must explicitly contain all and only supported structural items. Only distinct direct structural load cases are supported. Structural actions must be explicitly assigned to selected cases, with no duplicate action in a case. Subtype-aware group discovery finds factored assignments and refuses them explicitly; they cannot disappear through exact-name filtering. Unsupported structural item subtypes, omitted load groups and unassigned actions are refused.

Every member needs exactly two ordinary IfcRelConnectsStructuralMember relations identifying its actual geometric endpoints. AppliedCondition, AdditionalConditions, SupportedLength, ConditionCoordinateSystem, missing endpoints and specialized connection relations are refused. Shared/nonidentity structural contexts or point placements, member-local placements, action placements/representations/destabilizing semantics, local load/support axes, nonconstant member actions and unsupported load factors remain explicit unsupported boundaries. Existing supported frame roll follows the native FEM reference basis.

## Spatial Validation

BIM import requires identity project geometry-context and site frames, consistent explicit horizontal true north, a site directly aggregated under the project, and building/storey placements directly relative to their respective aggregate parent placements. Unsupported site translation/rotation and absolute or unrelated building placements are refused before destination publication.

Storey placements must contain only a vertical translation, agree with IFC Elevation, and agree with the cumulative heights implied by the authored level/height properties. Levels must be unique within a building. Storey rotations, horizontal displacement, Z=99 contradictions and incompatible authored levels are refused. More general foreign placement chains may be geometrically equivalent, but remain outside this explicitly admitted subset until implemented.

The STEP raw-string reader also validates the complete decoded Unicode string when its closing quote is reached. Raw lone high or low surrogates now fail with `step.string` before publication, matching escaped-string and serializer behavior.

## Final Verification

Full exchange suite with independent oracle outputs enabled: **48 passed, 0 failed, 189 assertions** across the five executable suite files. The twenty adversarial BIM/FEM cases verify `unsupported`, absence of an artifact, an error diagnostic and exact source graph immutability. Existing representable BIM/FEM/GLB roundtrips remain successful.

IfcOpenShell **0.8.3** independently revalidated all generated representable cases with zero schema diagnostics: 64-entity FEM export, 37-entity BIM spatial export, 11-entity Khronos Box import and 1-entity Binary PixelTexture. Additional independent assertions confirm FEM material/section values, topology, load actions and unit dimension vectors for modulus, density, area and torsional constant; BIM geographic angle/storey elevations remain correct. The TypeScript oracle suite now imports the IfcOpenShell-reconstructed BIM/FEM STEP graphs and verifies their supported projected values as well. Existing complete official 35/3161/188-entity graph differentials and the always-executed three.js GLB bounds oracle still pass.

Reproduction uses the same scripts/runtime and retained `🗑️generated/ifc083` and `🗑️generated/ifc-results` folders documented in `exchange-implementation.md`. The worker did not delete shared generated data needed by other workers.

Focused strict TypeScript checking reports only the previously documented unresolved external workspace module `@semio-tech/framework-interaction` through the actual FEM snapshot's transitive imports; no new converter TypeScript error was reported. Direct Bun/TypeScript/Python commands use the already documented infrastructure fallback because the cached repo CLI does not expose the needed script/read routes.
