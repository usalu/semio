# Facade R18 Contract and Evidence

The official Savoye as Built p96 excerpt recorded in source-audit-r17.md establishes exterior render 25mm, outer hollow-clay leaf 80mm, inner leaf 55mm and internal plaster 15mm; finished totals are 290mm at cantilever facades and 260mm elsewhere. The 115/85mm residual cavity is an inferred arithmetic remainder, not a measured clear cavity at concrete lintels. Source family mapping is pending parent correlation.

The proposed schema contract adds explicit exterior-side orientation and material-layer void semantics. Occupied wall parts must subtract all opening unions in every solid leaf, preserve cavity offsets, and drive quantities and all projections. Tests will compare shared parts against Shapely unions.

No exact sash profile or pane-module schedule has been verified. Physical perimeter members may only be labelled inferred. A generic two-metre pane grid cannot be attributed to the source. Hanging L lintel envelopes alone cannot establish unknown toe thickness; U support and 11cm finished corner posts need correctly mapped locations.

## Physical Filling Evidence and Next Contract

Carpentry evidence was retrieved as indexed book transcription; the official university PDF index still exposes only the contents for these queries. [University book](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?sequence=1); [indexed transcription](https://es.scribd.com/document/1072987232/9788413202839). Primary drawing pixels remain unavailable.

Printed p203 describes timber door jambs with65×40mm L envelopes and hollow35mm leaves (two5mm plywood skins over25mm battens). Printed p205 explicitly identifies203cm and tabulated widths as free passage between frames. Printed p210 identifies original black-painted oak ribbon assemblies, fixed/sliding zones, and40mm original exterior setback; later aluminium replacements must not be conflated. Other windows are predominantly fixed metal (p214). Actual L rebate geometry, batten spacing, pane divisions and salon sliding mechanism remain unmeasured here.

A shared filling assembly will own perimeter frame solids and physical panel leaves; no generic two-metre module grid. Rough aperture and free passage must remain distinct. Existing assumed door-width values cannot silently become smaller clear widths after adding frames. Exact unknown sections will remain inferred envelope geometry.

## Implemented Physical Ownership

The four first-floor facades now use upper29cm cantilever and26cm noncantilever screens, lower20cm finished parapets, and14cm-high single-leaf upstands (12cm finished thickness). Their exterior faces align and their band stations follow actual opening sills. Upper115/85mm and lower25mm air widths are inferred remainders. Both8cm and15cm interior partition stacks remain explicitly inferred. Layer air is excluded from all occupied solids and solid-volume quantities; gross solid material and gross envelope volume have distinct fields.

The32 filling assemblies comprise31 native windows/doors and the curved curtain. Original ribbon frames use timber; other glazing uses metal. Every native filling has one schema-backed construction type. Timber leaves have separate plywood skins and perimeter battens, with their hollow core excluded; batten spacing remains unresolved. Source rough door cuts now include frame envelopes while retained clear widths remain explicit. The constrained son doorway remains900mm rough and820mm clear. The unified garage front has one opening and two frameless panel trains; its composition,35mm thickness and equal train split remain inferred, with its head tied directly to the actual slab soffit.

The curved curtain follows its three authored disconnected paths with continuous mitered head/sill bands and end jambs:15 parts, with no invented intermediate pane grid. Its polyline offsets are independently checked against Shapely mitered buffers. All7 IFC representation modes, physical quantities, and axonometric/plan/section/elevation consumers use shared occupied parts. Wall STEP export emits those physical occupied parts, rather than a full-depth bounding slab.

Five cavity-wall tests and three physical-filling tests passed through the portable Python harness with Shapely. Three updated main representation/STEP tests passed; the fourth quantity test was initially blocked by stale renamed suite tags in main cases, owned by the chapter author. Three drawing providers compile. A native Manim render remains the parent gate. Structural-and-filling subtraction is implemented by the parent through cached prism ownership. Regional leaf and air volumes use that ownership; an independent GEOS audit is being rerun while the parent stabilizes boundary recombination.

Historical limits remain: exact sash and rebate profiles, original pane-module divisions, L lintel toe profile, U support section/placement, corner hangers and the full sliding mechanism are not measured by this implementation. The physical perimeter envelopes are explicitly inferred; this is not a completion certificate for historic joinery.

## Files

- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔌️exchange/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧬️schema/🔣️.schema.json
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧊️axon/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/📋️plan/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/✂️section/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧱️walls.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🧱️walls.json
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🪟️fillings.py
- 🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🪟️fillings.json
- Shared model source JSON (coordinated facade/types/materials/aperture edits).

## Newly Recovered Vertical Scope and Garage Evidence

A later targeted index query recovered printed p181 in the same book transcription: the29/26cm families describe the screen ABOVE the ribbon, while the parapet BELOW is described separately at20cm finished, and terrace screens at16cm single-leaf masonry. Therefore applying the29/26cm stack across the entire wall height is an intermediate overgeneralization that needs a vertical region contract and corrected source ownership. The earlier audit excerpt omitted this distinction.

Printed p222 additionally describes TWO trains of sliding garage panels forming one vehicle front, with the trains accumulating behind an oblique side and no visible intermediate framing. The existing three separate garage apertures and generic perimeter framing are therefore an authored simplification, not a verified as-built assembly. That topology needs source-plan mapping before exact remodeling.

## Final Source Delta and Remaining Checks

The three separate garage holes and their false intermediate masonry piers were replaced by OP-GARAGE and one native GARAGE-SLIDING door with two panel trains. Three static vehicle access reserves now reference portions of that continuous clear aperture. The aperture binding test passed; the older static three-car poses fail against the expanded actual column register, which is being reassessed by the model author without moving the building to force a fit.

Six cavity/regional wall tests and four physical-filling tests passed, along with three independent garage contracts. The five modified main wall/filling/STEP methods include an independent regional solid/air GEOS integration; the last combined run waits on the parent boundary recombination fix. Native rendering remains the parent gate.

Additional files: model/tests/🚪️garage.json and 🚪️garage.py; model/tests/🚗️parking.json and 🚗️parking.py.

## Frozen Handoff Validation

Current source: 374 elements, revision 3. Six wall-region/cavity contracts, five physical-filling contracts, and three garage contracts passed in bounded fresh Python batches. All three function-test modules are registered in the standard BIM load_tests hook. Five modified main baseline methods passed together in 37.561 seconds. The full independent regional GEOS quantity oracle separately passed in 10.168 seconds. Three analysis links were regenerated from current physical quantities; consistency_issues returned an empty list. The chapter author is running a fresh whole-baseline and six-chapter preparation audit from this freeze.

The representation takeoff now uses actual clear glass dimensions, physical glass face areas, and actual glass/gross-wall share. Glass hatches come from shared filling parts. Localized quantity metadata distinguishes rough cuts, clear openings, actual material volumes, regional envelope bounds and the maximum wall bounding volume.

STEP coverage now also includes native selected window/door/curtain fragments with actual multipart geometry, types and material aspects. See native-step-r18.md for independent IFC-engine validation and remaining complete-model/host-context limitations.

Remaining source omissions include exact L lintel toes and inner-leaf U supports, 11cm corner/hanger placement, full 6cm sill shelf footprints, original sash/rebate and pane-module schedules, and slider hardware. The single-leaf 16cm terrace screen and open terrace-edge attribution still require horizontal source-plan correlation. Perimeter, train split, garage composition and unmeasured section details remain explicitly inferred. Native Manim rendering is still an independent parent gate.

No further facade or source mutations are planned from this agent. Source shelf/lintel work and native slab-host service openings continue under the parent.
                                                