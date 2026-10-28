# Shared Unit And Traversal Generator Plan

Read-only source map plus authored unit-traversal-vectors.json and unit-traversal-oracle.py. Existing IfcOpenShell0.9.0.post1 calculate_unit_scale normal API independently agrees4/4 actual ADD2 named length graphs: metre1,millimetre0.001,foot0.3048,inch via foot0.0254. Generated/unit-traversal retains original authored physical graphs/exactunitIDs/oracle metadata. No reinstall/runtime library or native proof.

## Mounted callers and scope

geometry extraction calls Graph.lengthScale before products. BIM calls lengthScale for model coordinates and unitMeasure on individual property units requiring length dimensions. FEM requires lengthScale==1 and unitScale/unitMeasure==1 plus expected physical type for typed analytical properties/load units. Refactor must preserve these deliberate supported-projection policies; accepting conversions numerically does not mean FEM endpoint starts supporting nonSI graphs.

Current lengthScale cold filters all entities for exactlyone Project, then maps/filters every UnitAssignment member. unitMeasure clones active Set recursively and synchronously loops all DerivedUnitElements, while placement synchronously recurses up to256 links. Fixed recursion depth bounds do not bound derived fanout or source project/assignment scans.

Neutral inputs include SI/conversion chains above plus actual permitted derived MASSDENSITYUNIT kg/mm³ scale1e9 dimensions[-3,1,0,0,0,0,0] and MODULUSOFELASTICITYUNIT kg/m/s² scale1 dimensions[-1,1,-2,0,0,0,0]. These are derived unitMeasure cases, not project LENGTHUNIT: IfcDerivedUnitEnum has no LENGTHUNIT. Do not fabricate a derived length enum to test valid project units. Derived numeric expectations are dimensional algebra vectors, not a claim that calculate_unit_scale is a general derived-domain oracle; direct thirdparty derived comparison is not executed here. Use actual allowed named elements plus independently measured property conversion or schema validation before treating full derived cases as verified.

## One algorithm, two runners

Add Graph.lengthScaleSteps(), unitMeasureSteps(unit), placementSteps(value) generators over the same private captured source/index. Keep sync reference wrappers as a simple generator drain and async mounted callers through a shared original-control runner; do not duplicate unit rules/transform logic. Each yield is actual entity/member/edge/derivedfold step, with small fixed7dimension and4x4matrix math bounded. Retain active path with push/pop instead of whole Set cloning per edge; preserve cycle/depth/domain/exponent/finitepositive scale checks. Conversion factor wrapper and UnitComponent role must be exact; no arbitrary typed scalar acceptance widening.

Async generator runner must preserve original callback failure scope, signal/limits and finally close; do not turn caller Error into exchange.units/unsupported-product. Product/representation lists should likewise expose shared steps instead of map/filter arrays. Existing byte/source ownership remains Graph.capture; generator interfaces do not mint receipts or native work wallets.

## Exact cancellation and fault frontiers

For projects scan, unit assignment scan, conversion edge, derived member/fold, placement edge/fold, representations and items: initial checkpoint before reading first element, interior after exact original batch, completion before publishing result. Test long unrelated project/entity rows and long assignment/derived/representation fanouts, not only deep chains. Keep exactlyone-project/unit ambiguity and absent source refusal deterministic. Cancel at each frontier must reject io.cancelled with no artifact; progress throwing original Error/undefined/domain-shaped sentinel must reject exact identity before domain conversion. Mutating original control after capture must not replace signal/callback/limits.

Neutral refusal rows still needed for duplicate/missing project or length units, assignment wrongdeclaration, missing unit ref, unknown prefix/type/dimension, zero/negative/nonfinite conversion, unrelated measure wrapper, duplicate/cyclic conversion, derived mismatched dimensions/exponent and placement cycle. Existing unit fixtures in BIM/FEM should remain regression coverage for accepted physical roles and deliberate SI-only endpoint restrictions. Independent helper acceptance never proves complete EXPRESS or correct original native accounting.
