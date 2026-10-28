# IFC4x3 Exchange Independent Review

Production source root: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io`.

Read-only audit. No production edits. Executable adversarial repro retained in `exchange-review-repro.ts`; it reads generated oracle exports. Run `bun <ticket>/exchange-review-repro.ts` from repository root. Direct shell fallback is necessary because cached repo CLI does not expose script/read operations, as already documented in plan.md. No generated files were created by this audit.

## Confirmed Findings

1. **P1 — All actions in factored assignment silently disappear.** `fem.ts:158` matches only exact IFCRELASSIGNSTOGROUP, ignoring its valid IFCRELASSIGNSTOGROUPBYFACTOR subtype. Replace the emitted load-case assignment with subtype and append factor 2. Expected: two original loads scaled by 2, or atomic unsupported outcome. Actual: successful partial artifact, one load case with zero loads. Repro result `["group_factor_2","partial",[0]]`. Refuse unsupported factor semantics before publishing any artifact.

2. **P1 — FEM import ignores sole analysis membership.** `fem.ts:111` checks analysis count but subsequent global entity scans import every structural point, curve member and load case. Remove the emitted assignment of nodes and member to the analysis. Expected: refusal of absent analysis membership, or projection restricted to actual membership. Actual: successful partial artifact with one element: `["no_analysis_members","partial",1]`. Foreign files containing unassigned analytical objects can fabricate analysis membership.

3. **P1 — BIM site transform is discarded without refusal.** `bim.ts:68` does not inspect Site.ObjectPlacement; `bim.ts:71–73` uses building RelativePlacement directly, without verifying PlacementRelTo matches its aggregate site or combining intermediate placement chains. Translate site 100m in X. Expected: equivalent supported coordinates or atomic refusal because site horizontal origin is not representable. Actual building remains `(1,2)` with partial success: `["site_translate100","partial",[{"x":1,"y":2}]]`. Project context origin/rotation and site rotation are likewise ignored. A building with independent absolute placement is also misinterpreted as relative to its aggregate site.

4. **P1 — BIM storey geometry is contradicted by returned authored parameters.** `bim.ts:75–82` never inspects storey ObjectPlacement or Elevation and never validates agreement of semio_BimStorey level/height with cumulative storey heights. Change emitted ground storey placement Z from 0 to 99, leaving authored Level=0. Expected refusal of incompatible coordinates. Actual partial success and unchanged Level=0: `["storey_translate99","partial"]`. Rotated or horizontally displaced storeys are equally accepted although target model cannot represent them.

5. **P2 — Engineering units validated by labels and magnitude, not dimensions.** `fem.ts:102–105`, `geometry.ts:56–77`. Change emitted MOMENTOFINERTIAUNIT derived element exponent from 4 to 1, giving metre rather than metre^4. Expected dimensional refusal. Actual partial success: `["inertia_m1_mislabeled_m4","partial"]`. `unitScale` computes numeric scale only; every unprefixed metre power returns 1. The same weakness admits incompatible derived density/modulus units. Validate full dimension vectors and named SI unit identity, including conversion dimensions; do not trust UnitType enum alone. This test is intentionally invalid engineering input to verify refusal, unlike the valid foreign-input cases above.

## Further Concrete Refusal Holes Found by Source Inspection

- `fem.ts:134–153`: IfcRelConnectsStructuralMember is never read on import. AppliedCondition, AdditionalConditions, SupportedLength and ConditionCoordinateSystem (including end releases/springs/connection axes) are dropped while every RIGID_JOINED_MEMBER becomes a fully rigid frame. Require exact supported relation semantics and both member endpoints; refuse unsupported connection conditions.
- `fem.ts:159–177`: action ObjectPlacement is not checked. Even GLOBAL_COORDS does not justify silently accepting unsupported action-local placement. Refuse unsupported placement and structural activity dependencies rather than ignoring them.
- `fem.ts:114` only refuses exact IFCSTRUCTURALSURFACEMEMBER/IFCSTRUCTURALLOADGROUP; unsupported structural subtypes or unassigned actions can remain unnoticed. Verify selected analysis has no omitted analytical object/load semantics before projection.
- `step.ts:97`: raw strings append UTF-16 code units without isWellFormed validation; malformed raw lone surrogate is accepted, although serialization rejects it at line232. X2/X4 escaped surrogate validation exists at line128. Add complete decoded-string validation at closing quote so parse/serialize contract is consistent.

## Boundaries and Confidence

No defect found in inspected quaternion matrix formula, determinant reflection winding or Z-up/Y-up rotation. Normal omission is explicitly diagnosed. Broad geometry support, MVD/EXPRESS evaluation and bounded synchronous accessor/serialization latency are already documented limitations and are not reported as new defects. Existing official-fixture oracle validation does not cover these adversarial spatial/analytical cases; successful self-roundtrips do not establish foreign-input semantic equivalence.
