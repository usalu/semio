# Animal and Vegetation Models

The 30 catalog entries have authored body, thermal and structural models at 1:50, 1:100, 1:200 and 1:500. The ten previously unavailable digital scales now carry their own geometry. Existing print limits remain on the small species and on the hosta. Dimensions and physiological surface estimates remain in physical units.

## Animal geometry

| Entry | Work on the models |
|---|---|
| Dairy cow | Attached the pastern and hoof of each coarse leg to its own distal joint so that the cow can roll onto the hoof during a step. |
| Domestic cat | Built a digital 1:500 cat around the spine, four articulated legs, five tail segments and cupped ears. Kept the tail width continuous at its joints. |
| Goat | Rebuilt coarse cannons, hooves, ears, horns and beard. Closed the rib profiles so the chest cavity remains hollow in exports. |
| Hedgehog | Built 1:200 and 1:500 coats with mobile dorsal and flank regions. Kept the pointed snout, four short legs, ears and distal toes below the folded rim. |
| Herring gull | Built the coarse bill, two-part neck, long primaries and two webbed feet with their own thermal surfaces. |
| Horse | Replaced the coarse lower-leg blocks with individual cannons, pasterns and hooves. Restored the coffin bone inside each moving hoof. |
| Labrador dog | Gave each paw and plantar pad its own binding at 1:200 and 1:500. Restored the coarse hanging ears and closed the rib profiles. |
| Mallard duck | Built the drake's broad bill, breast, rump and paired webbed feet at 1:500. Restored primary feathers and rump to the intermediate model. |
| Pig | Separated the four hooves from the coarse lower legs and restored the ears, curled tail and nasal disc. |
| Pigeon | Preserved the cere, short beak, wing points and rear-facing hallux in the new digital model and the intermediate drawing-scale model. |
| Red fox | Gave the 1:500 figure independent ears, a three-part brush and four legs. Restored thigh, shoulder and upper-arm articulation to the 1:200 figure. |
| Red squirrel | Built separate intermediate and site-scale models around the seated rump, grasping hands, long hind feet and curved two-part tail. |
| Roe deer | Restored the coarse pasterns, pointed hoof ends, projecting ears and branched antler beams. The antlers participate in the structural overlay. |
| Sheep | Brought the dark hoof contacts, ears and tail back into the coarse fleece silhouette. The skeleton follows the feet to ground. |

## Vegetation geometry

| Entry | Work on the models |
|---|---|
| Agave | The 28 distal blades taper to a terminal point at the two coarsest scales. |
| Apple tree | The coarse fruiting-crown clusters move with the low and high scaffold branches and the leader tip. |
| Bamboo clump | Preserved all 2,920 individual laminae, six culms and internodes. Each blade follows six sections through its attachment, shoulder, maximum width, taper and tip. |
| Date palm | Restored the flared basal root collar beneath the segmented trunk at 1:500. |
| English oak | Bound lower, middle and leader crown clusters to their separate foliage members. |
| Hornbeam hedge | Replaced the single rigid linear canopy with six overlapping clipped crowns that each follow their own stool. |
| Hosta clump | The previously unavailable 1:500 model and corrected 1:200 model carry nine petioles and nine tapered blades. Inner and outer leaves retain their thermal regions. |
| Ivy on trellis | Open rails and posts support two tiers of independently moving foliage from four climbing stems at the coarse scale. |
| Japanese maple | The eastern and western fans of each crown tier can move independently at 1:500. |
| Lombardy poplar | The columnar silhouette retains fourteen lateral swellings carried by the corresponding ascending fans and apex. |
| London plane, pollarded | The coarse low leaf clusters, upright regrowth and top follow the appropriate pollard heads. |
| Miscanthus grass | Replaced the coarse stacked mass and broad intermediate plates with four articulated flowering culms, tapering panicles and arching ribbon blades. |
| Multi-stem birch | The three stems and their upper branches carry separate crown masses at intermediate and coarse scales. Distributed crown mass markers replace the oversized central marker. |
| Scots pine | The intermediate and coarse structural views retain the woody axes under the needle platforms. |
| Small-leaved lime | Split the clear stem from the hinged lower leader. Replaced stacked crown prisms with tapered leaf clusters and restored the stem's thermal region at 1:200. Documented the scale decisions in English and German. |
| Weeping willow | Restored eastern and western canopy arches that carry the pendant shoots. |

## Verification

The before-change contract test found ten missing digital scales. After authoring, all 120 living scales passed checks for bound bones and references. All new digital-only models cover every declared thermal region and emitter and have a structural member binding for each rig bone. All seven larger mammals have four moving foot-end bones represented in both body and structural geometry at 1:500.

The rendering harness tessellated all three lists at all four scales for each entry, rejected missing or non-finite meshes and produced 360 inspection panels. A separate GLB pass used the catalog model oracle (watertightness, finite triangles and monotonic budgets) and reloaded the presentation-pose output with the independent readers. The first complete post-repair pass had zero geometry or GLB findings across 30 entries. The crown rebindings made while the scan was underway have a separate follow-up pass.

The full cross-representation tests also compare the 3D envelope to the independently authored printed outline and the exposed surface area to independently specified thermal areas. Their findings are recorded below as they complete.

