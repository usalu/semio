# Chimney Camera Registration R40

Focused original-state registration audit on 2026-10-29 in the existing ticket. The source-selected inferred reconstruction was published after parent coordination. Parent owns ticket lifecycle, broad native exchange/schema integration, final analytical links, parking refresh and generated cleanup.

## Published Result

Source now has448 products. Existing BOILER-FLUE-VISIBLE is one continuous native IfcChimney atX6.64/Y11.50 fromZ0 to9.49m, with96 genuine hollow material sectors. Relocated OP-FLUE-FLOOR and new OP-FLUE-ROOF are independent420mm slab apertures. Stale shared_voids with the distant stair hatch is removed. No wall/canopy hole or hidden below-ground branch is added. The nominal40cm diameter remains sourced; its outside-envelope interpretation,60mm wall,XY±0.20/0.30m,top±0.20m and sleeve clearance remain explicitly inferred in English/German. Cap construction remains unresolved.

Primary lower-storey wall identity and the independent second-view lateral check select the B camera family. The final axis/top are a bounded inferred reconstruction, not measured original coordinates. Two other original functional roof chimneys remain unregistered. The modeled occupied box height is9.49m±0.20m; this is not asserted as the complete historical building height. SC-1 crest9.325m and stair canopy/weather top9.35m retain their distinct source evidence.

## Primary Evidence Improvement

The [official indexed Savoye as Built book](https://uvadoc.uva.es/bitstream/handle/10324/68427/9788413202839.pdf?sequence=1), p118 figure27, explicitly locates the first-floor boiler chimney between the stair and pilotis19. Model19 is at8.128,12.572. This is a much better identity/adjacency constraint than the inherited wall-region guess4.70,9.40. It does not itself provide a metric chimney axis. Bookp117 describes the wall-adjacent cylindrical boiler flue in ground/first floors and its visually contrasting40cm diameter, distinct from the13cm sanitary stack and55mm stair tube. A40cm outside envelope is a reasonable interpretation of that comparison; the source does not specify the wall section or a roof sleeve. No hidden branch or bent offset follows from the caption.

A bounded two-request HTTPRange attempt for the official PDF header/tail failed before response. The full book was not downloaded or repeatedly fetched. Indexed official text supplied the caption; its figure27 pixels remain unrecovered in this pass.

## Actual Image Controls and Camera Family

Inspected original1931 plate28, plate30 and completed facade plate27 image1705. Research enlargement of the actual plate28 raster corrects the rough prior curved-edge control692,218: the visible light curved edge is approximately692,246, then710,225;734,211;765,201;797,195;850,190;910,188;970,187. Eight pixels is the explicit hand-trace uncertainty; it is not a surveyed contour. The near ramp points482,739 and766,739 are at the photographic crop boundary. They define edge-line directions ONLY and are not assigned to Y16.889 or to a transverse structural endpoint.

Portable input chimney-register-r40.json precedes the numerical fit and records all assumptions. The two deck-edge lines intersect at image578,327. Under the restricted level, zero-yaw camera family, RF-2B top width and its known pitch determine camera position for any focal length and inferred width. SciPy least_squares fits the projected actual SL-STAIR-CANOPY perimeter atZ9.35 to the sampled photographic edge. These are point-to-polyline residuals, not a full visibility-aware bundle adjustment. Principal point, focal length and flight width are not independently measured. The fitted rim center uses image818,130 and41px apparent diameter with the explicit40cm outside-diameter hypothesis.

|Hypothesis|Focal px|Flight width m|Component RMS px|Cylinder X,Y,topZ m|
|---|---:|---:|---:|---|
|A, canopy/wall-adjacent|244.868|1.29393|2.219|6.66812,10.30361,9.98836|
|B, separate roof penetration|473.404|1.13638|5.546|6.64260,11.18839,9.58456|

Starts200/400 converge toA; starts800/1600 converge toB. Their0.885m Y and0.404m topZ differences exceed a responsibly claimed tight reconstruction, even before camera rotation, edge identification and diameter interpretation uncertainty. A very wide cameraA also cannot be accepted without independent camera/lens controls. CandidateB has a less extreme focal length, but this is a prior preference, not source proof.

Both solutions predict inherited4.70,9.40 at imageX898.50/894.54 and diameter29.75/29.56px, substantially different from image818 and41px. This rejects that old XY within the stated camera hypotheses; it does not independently validate moving the entire lower trunk.

The western SC-1 path substituted for the stair perimeter, using an optimistic uniform9.35m crest and the same camera constraints, fits no better than42.270px RMS while reaching focal/width bounds2500/.8. This supports the stair-body interpretation within this camera family, but is not a proof covering arbitrary camera poses. The original primary roof-plan diagram remains an earlier arrangement and was not used as executed stack XY.

## Independent Wall Constraint and Second View

Bookp117 distinguishes the13cm sanitary pipe against the stair spiral from the boiler flue against the wall. Its wall phrase is “La que sí está junto a la pared”; p118 figure27 places the boiler between stair and pilotis19. Interpreting the stair guard itself as that wall would reverse the source distinction.

Actual native occupied geometry plus independent GEOS establishes that A is1.321m from ground W-G-ROOM-10 and1.163m from first W-SUITE-ENTRY, while its disk intersects each actual U guard's projected material by0.002881m². B is nearer the walls, although its unconstrainedY11.188 remains0.437/0.297m away. Source-selectedY11.50 gives native ground-wall gap0.125m, first W-PARENT-BATH-S gap0.060m and suite-entry corner gap0.026274m, with0 occupied overlap. Actual U-guard gap is1.162229m. These are model checks, not historical gap measurements.

Constrain the same camera solver toY11.50: focal549.094px, inferred flight width1.11551m, component RMS5.635px, computedX6.63922 and topZ9.49257. The clean authored values6.64/11.50/9.49 retain that uncertainty. The photo fit is compatible with the lower-storey wall constraint; the lower-residualA has the wrong ground/first wall adjacency.

Plate30 adds an independent approximate side-view check. The stair-body image center is about645 and cylinder756, while the18px-wide cylinder gives an approximate45px/m local scale using the40cm exterior-envelope hypothesis. Upper ramp pitch identifies increasingY toward image right. Weak-perspective lateral displacement111/45≈2.47m from stairY9.180 gives cylinderY≈11.65±0.40m. This favors the selected northward wall-adjacent family overA'sY10.304, which would give only about51px lateral displacement. The side-view azimuth, body-center trace and weak perspective are explicit inferences; this is not a calibrated second-camera PnP solution.

Treating40cm as lumen plus the inherited inferred60mm walls produces even larger changes: A exterior520mm gives5.96568,9.58692,10.70689; B gives5.94016,9.80282,10.43557. This is a sensitivity experiment, not a new assertion that40cm was lumen. It demonstrates why inherited wall assumptions cannot silently become historical measurement.

## Native Publication and Circulation

The continuous chimney keeps its existing canonical identity and ground containment. OccupiedZ governs every view; actual production catalogue, ground/first/roof plan cuts, section and axon were tested. Native material volume is0.607765641909m³; central air remains empty. Both sleeve profiles are fixed native source openings, independently compared with the portable96-vertex circle. Roof opening has canonical UUID5 identity, roof spatial containment, architecture/services partial-model membership and IfcRelVoidsElement; no filling relationship is invented.

All independent occupied solids were checked: walls, screens, slabs, columns, beams, foundations, stairs, railings, furnishings, details, fillings and services. Zero projected material overlap over commonZ conservatively proves zero physical overlap for this selected axis. The generic0.25m free clearance to every unhosted native column also passes. Attested wall adjacency permits the stated small positive wall gaps; no wall collision is ignored or exempted.

Existing ground tests exposed two stale consumers reading raw guard meshes without z0/z1. They now consume public normalized solids/footprints. Including real services exposed the old northern800mm body route crossing SOIL-STAIR; checking actualZ also includes column14, which is contained in cellar but spans ground. Only the assumed route control changed: start8.70,8.10→8.70,9.70→7.80,10.30→7.30,10.50, followed by the former northern path. The full occupied catalogue, including ramps, curtain, slabs and foundations, passes. Registered wall/grid, fixtures, stair, columns and services remain fixed. This is a teaching circulation test, not accessibility certification.

## Reproduction and Verification

TDD: portable model/🧪️tests/🚬️chimneys.json and four tests were authored first. The actual source-position test failed on the old4.70,9.40/6.34m points before source mutation. The approved scoped author then fresh-read447 source and published448 products. All new source fields use the existing schema and native geometry interfaces; no runtime API extension or external product dependency was needed.

A clean verification interpreter rebuilt the actual448-product building in41.442s. Four new contracts pass: independent source/sleeve registration; GEOS annular material plus SciPy hull volumes/quantities; all-native occupied material/generic column clearance/existing hall body; actual production full catalogue plus all three storey plan cuts, section and axon volume. Existing seven ground and six stair tests also pass:17/17. The strengthened final all-category hall body passed again after adding slabs/foundations. No route was narrowed and no registered geometry was moved to satisfy it.

Retained research inputs: chimney-register-r40.json/.py; chimney-wall-adjacency-r40.py; chimney-wall-constrained-r40.py (executed after the registration solver); chimney-proposal-clearance-r40.py. The final clearance input uses separate native-part comparisons, avoiding an expensive unnecessary whole-guard GEOS union. The earlier union observation was terminated by resetting the verification kernel; the bounded comparison completed and its output is retained.

Retained source/verification inputs: chimney-author-r40.mjs and standalone chimney-physical-r40.py. NumPy/SciPy/GEOS are the existing pinned authoring/test-only interface from python-validation.mjs. Generated camera, clearance and17-test ledgers plus inspected image crop/overlay remain only under 🗑️generated for parent review and cleanup. The overlay showsA red andB blue over original plate28; hidden full polygon edges are not interpreted as photographed building lines. Crop/enlargement preserved original grayscale pixels with nearest-neighbor scaling; colored diagnostic projections are separate.

Product files touched:
- model/🔣️.json: continuous chimney, native sleeves, evidence, spatial/void/model relations and inferred modeled height.
- model/🧪️tests/🚬️chimneys.json/.py: portable contract and independent tests.
- model/🧪️tests/🏠ground.json/.py: inferred route control and actual occupied geometry across storey containment labels.
- main 🧪️tests/🐍️.py: new function-test registration.

No AGENTS edit, shell command, Git mutation, worktree, IDE, source-file copy or runtime external library was introduced. Parent owns full native IFC SDK, complete schema, analytical-link and parking-cohort integration. The source initially read was447 products SHA51bd5530aa4ad750c38bf8e849af1c90a1c196bb3c3abd39d448781c6337173f; parent link-only refresh was59520f42d21688639957e15574ceaf9d49ebd8a620ac5c289c5d645b70ecfb71 before this scoped publication. Final producer SHA follows in the parent handoff; later field-only integration may change it without changing this geometry.

Published producer cohort:448 products,SHA2561c84ec6f32b97009eefd0628adac58b06921deb482f64386a2b971c6c3fb2cd8.
