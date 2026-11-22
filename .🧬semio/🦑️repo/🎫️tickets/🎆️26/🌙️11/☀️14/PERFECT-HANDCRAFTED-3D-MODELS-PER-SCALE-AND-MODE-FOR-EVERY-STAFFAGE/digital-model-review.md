# Staffage model review

## Shipped catalog

On 21 November 2026 the live catalog contained 152 assets and 608 authored digital scales. Every scale had body, thermal and structural solids. The 50 print refusals belong to the paper representations. Their corresponding digital models retain physical dimensions and remain exportable. A catalog test now checks every asset at all four scales.

| Category | Assets | Authored digital scales |
| --- | ---: | ---: |
| Vegetation | 16 | 64 |
| Animals | 14 | 56 |
| Humans | 28 | 112 |
| Machinery | 16 | 64 |
| Vehicles | 9 | 36 |
| Components | 18 | 72 |
| Appliances | 24 | 96 |
| Furniture | 12 | 48 |
| Objects | 15 | 60 |

## Measurements and corrections

- The 1:50 playground swing has 152 alternating chain links. Each 8 mm wire now follows a continuous closed stadium curve. The thermal model uses the same metal surface. The structural model uses the tensile member between the hanger pin and the seat eye. Removing an unnecessary station on the straight side reduced the presentation GLB to 8,236,188 bytes, below the 8 MiB budget. The published geometry and all twelve exported scenes passed the geometry and reader checks.
- The fine wire previously measured as zero on a closed loft because the feature validator used the straight distance between its first and last sections. It now measures the length through the section centres. A Three.js path-length comparison checks a closed 140 mm path with an 8 mm wire.
- The tissue validator preserves anatomical dimensions when a paper shape combines several physiological regions. A separate printed region still goes through the minimum-feature check. Three targeted tests passed for this distinction, the closed wire and the existing texture and slender exemptions.
- The 1:200 laptop, gas meter and ventilation-unit drawings now cover their full band up to 1:350. The laptop has a legible lid outline, the gas meter has two separately legible connections and counter, and the ventilation unit shows the clear port throats. All three candidate drawings passed the product's range checks before their installation.
- The ceiling diffuser, picnic table and cooking pot each reopened in all twelve scenes on their current 21 November documents. Changes elsewhere in the shared catalog can invalidate earlier export evidence.
- The kernel now excludes disjoint hole boundaries before exact contact testing on perforated panels. The original implementation timed out on a 32-by-32 fixture. The corrected implementation passed all 125 polygon, extrusion and validation tests, including nested and touching outlines. The fixture compares remaining panel area with Three.js and detects a deliberately overlapping opening.

- The dining table now has four captive screw lifts on two stationary crossheads. The digital and printed 1:500 boards keep their 30 mm thickness and the stowed leaf clears the tops. Four lift stations passed 121 positions at all four scales, strict product validation passed, and an independent reader reopened all twelve presentation scenes in a 3,849,244-byte GLB. See [the mechanical review](r3-furniture-dining-table.md).

- The binary glTF encoder now reuses large identical accessors across meshes and modes after a byte-equivalent comparison. The same wheel loader fell from 11,585,904 to 4,984,652 bytes and all twelve scenes reloaded successfully. The 48 GLB tests pass, including an independent reader round trip with two different payloads deliberately sharing the same hash.

- With identical geometry shared in the binary, the cook, running child, sleeper, umbrella carrier and guide now export in 4,430,040 to 7,629,596 bytes. Independent readers reopened all 60 scenes without findings.

- The stepladder keeps its four 80 × 25 mm treads, 26 mm platform rim and 28 mm handrail at 1:200 and 1:500. Its coarse paper symbol shows alternate treads in front and side views. Each of the 39 sampled print sizes passed, along with product validation, the independent physical size checks and all twelve exported scenes (1,552,376 bytes). [Stepladder dimensions and print evidence](r3-objects-stepladder.md).
- The GLB line overlay can reference its source triangle positions and skin bindings. Feature edges now distinguish independently moving parts that meet in the rest pose. The 58 edge/GLB tests passed, including independently flexed and reordered skin bindings compared with Three.js.

- The 28 human figures passed individual GLB checks across 336 presentation scenes. The human audit records the 8 MiB limit, pose and thermal checks, and identifies older measurement evidence.
- The column-based volume measurement now merges intervals in place. On a 160,000-column four-solid test, the measured union stayed at 1.75 m³ and peak process memory fell from 279,392 to 250,224 KiB. Measuring windows of 128 × 128 columns then reduced peak memory to 131,904 KiB on the same 1.75 m³ example. Five existing union and distance-band tests passed.

- The live inventory verified all 1,824 body, thermal and structural lists across 608 digital volumes. Every list contains geometry and every named body reference resolves. The review found 30 differences between the authoring sources and shipped volumes. After reconciling individual changes, all 608 source records match the live catalog.

- The [bookshelf](r3-furniture-bookshelf.md) preserves 3 mm hardboard, 18 mm boards and five pairs of articulated shelf halves. Each loaded midspan deflects 5.8 mm. It passed the full measured volume check and all twelve GLB scenes at 1,274,184 bytes.
- A Three.js measurement of the [market parasol](r3-objects-market-parasol.md) exposed a discrepancy between the inspector and the exported skin. The catalog geometry evaluator now blends the inverse-bind transforms for weighted solids and carries thermal section normals through that deformation. Across all scales, the open canopy exceeds 2.4 m and the furled cloth spans less than 0.65 m. The eight 0.5 mm gores have separate upper and lower exchange faces.

## Verification by source revision

The [per-model geometry checks](digital-volume-status.md) identify each measured source revision. The [export readbacks](digital-catalog-acceptance.md) record physical, thermal and structural scenes across all four digital scales. Independent checks of [surface temperature and flux](digital-catalog-heat.md) and [rig frames, mass centres and member resultants](digital-catalog-rig.md) cover the named poses. The dated records below document earlier stages of this work.

- [Human poses and thermal measurements](humans-audit.md)
- [Animal and vegetation measurements](living-current-audit.md)
- [Vehicles, machinery and building components](equipment-audit.md)
- [Individual model reviews](r5-ledger.md)

## Local resources

The parallel audit and export queues were stopped when the user requested memory headroom. Their process groups were checked after termination. macOS reported system-wide memory free percentage of 25 before cleanup and 73 immediately afterwards. Swap use fell from 8.7 GiB to 2.3 GiB and subsequently to 1.9 GiB. Completed per-asset reports remain in the ticket. Heavy local validation is serial. The reusable audit runner records the input and validator hashes, so completed unchanged results can be reused.

On 21 November, an unavailable remote scheduler led to guarded serial local checks. The watchdog measures aggregate memory of each process group and reaps descendants on ordinary completion and cancellation. A two-worker allocation regression exercised the combined memory limit. The 1:50 swing passed its full measured check at 705 MiB, including the continuous steel links. The standalone fine thermal check used 367 MiB. All four digital scales now pass. Staffage and neutral model TypeScript checks passed following the weighted paper extrusion change.

The [storage crate stack](r3-objects-storage-crate-stack.md) now has five independently open crates, each with an 8 mm floor, 3 mm drafted walls and a bearing rim. The independent reader reloaded all twelve scenes in 1,377,440 bytes. The [extractor hood](r3-appliances-kitchen-extractor-hood.md) has a pierced 0.6 mm stainless enclosure, a curved 24-vane blower, opposing check-valve leaves and three extraction settings. The full measured geometry and schema checks passed. The [equipment review](equipment-audit.md) records the concrete mixer's reduction to 7,466,840 bytes, including the complete wheel, tyre and drum details. An independent binary comparison preserved all 2,892,804 indexed corners. All fourteen animals have passed the twelve-scene independent reload and 8 MiB limit. The green hosta is undergoing further file-size work.

The [parasol review](r3-objects-market-parasol.md) includes a new check of seam continuity. The original gathered skin closes the silhouette but shortens the fabric transversely and separates neighbouring hems in the furled pose. The shipped model now has three radial pleats per gore. The independent seam and area checks passed in both clips and at 25 points along the opening stroke. All measured product checks and twelve GLB scenes passed, at 3,770,756 bytes.

The current hood GLB passes all twelve scenes at 1,590,348 bytes and passes all 39 sampled print scales. The swing passed all twelve at 3,114,548 bytes. Generalising the crate feet with a 0.20 mm contour fixed an intermediate-scale failure at 1:66. The crate passes all 52 print samples with its digital geometry intact. The hosta passed at 7,441,416 bytes. The ivy export exceeded the budget before its latest lamina revision.

All 152 documents passed the updated normative schema (digest prefix 16ae7503b29e). The fine architectural validator now measures paper extrusions in their projected view. A 600 × 340 × 2 mm display in a test measured 2 mm at its edge, compared with the previous 340 mm. Its LRU tessellation cache now accounts for size and holds up to 32 MiB. A 40 MiB geometry stream verified reuse of hot meshes and exact regeneration of evicted positions and indices.

The monitor and vacuum have passed all twelve current export scenes each. The monitor has four VESA fixings, a rigid tilting screen and separate front and back heat fields. The vacuum retains the real hose, wand bore and floor brush dimensions across all four scales. See [desk](r3-furniture-height-adjustable-desk.md) and [vacuum](r3-appliances-vacuum-cleaner.md). The ivy export passed at 6,990,104 bytes. All 28 human assets match their four authored digital scales. See [human review](humans-audit.md).

The [waste bin](r3-objects-waste-bin.md) passes all twelve export scenes at 1,305,728 bytes after restoring its true pedestal, floor and open hinge clearances. The [patio heater](r3-objects-patio-heater.md) retains its 8 mm gas-valve spindle, and the [deck chair](r3-furniture-folding-deck-chair.md) retains its turned dowel and prop stock across the scale series.

The [cylinder](r3-appliances-hot-water-cylinder.md) and [charcoal grill](r3-objects-charcoal-grill.md) have separately measured service bores, coil/pocket and fuel bed/ash paths. All 984 paper checks per subject passed. A fresh household batch independently reloaded all 612 scenes across 51 assets. The inventory found 608 populated digital volumes, all matching their authoring documents and all references resolved. [Validation memory and cleanup](validation-memory.md) records the worker limits and process-group checks.

The [pollarded plane](r3-vegetation-london-plane-pollarded.md) retains its 6.80 m crown and young branch centrelines. All twelve GLB scenes were read back. The [server rack](r3-appliances-server-rack.md) now has continuous folded side returns, four equipment rails, two power strips, moving perforated leaves and real caster bearings in all four scale models. Its twelve exported scenes were independently verified. The twin valve unions of the [panel radiator](r3-components-panel-radiator.md), both sorption-wheel feet and the car exhaust passed their revised print-band checks.

The [external venetian blind](r3-components-external-venetian-blind.md) now exposes the weather-side pane as its heat-transfer surface. Two 4 mm panes, a 16 mm sealed gas space, and the 12 mm edge spacer are present across all four models and twelve exported scenes.

The [Scots pine](r3-vegetation-scots-pine.md) and [weeping willow](r3-vegetation-weeping-willow.md) have been published. The former retains the leader beneath its needle pads. The latter carries eight separately tapered bark-covered switches through its hanging sprays. Both passed independent twelve-scene export checks. [Botanical dimensional fixtures](digital-woody-stations.json) record the working sections and attachments.

Visual comparison of the blind at 1:500 exposed missing slats. The shipped physical, thermal and drawn models now retain all eighteen. The [date palm](r3-vegetation-date-palm.md) carries the changing width and bend of all 32 blade segments into its coarse articulated drawing. Both revised bicycle documents also passed their whole-asset checks.

The [bamboo](r3-vegetation-bamboo-clump.md) retains 2,938 individually positioned fine leaves and the load-bearing culm. The [Lombardy poplar](r3-vegetation-lombardy-poplar.md), [English oak](r3-vegetation-english-oak.md) and [multi-stem birch](r3-vegetation-multi-stem-birch.md) retain the measured spread and joint motion of their crowns as scale decreases. The [hosta](r3-vegetation-hosta-clump.md) uses the actual depths of its cupped leaf stations. The [cargo bike](r3-vehicles-cargo-bike.md) now assigns 2.83 m² of external plywood to the heat model. The [sectional garage door](r3-components-sectional-garage-door.md) passes its printed band endpoints through 1:1000.

The courier’s sack truck now uses a 578 mm turned axle, through-bored steel hubs and two retaining rings. At every scale the shaft and wheel were independently measured at 20 mm and 20.4 mm bore, respectively. The [human thermal table](human-thermal-scales.md) again records all 112 physical thermal measurements and all 336 independently read scenes after the courier and sleeper revisions. [Export receipts](digital-catalog-acceptance.md) identify the exact checked file per asset.

The oak’s 56 lobed leaf blades and the willow’s pointed hanging sprays now pass all twelve physical projection checks per tree. The pumpjack valve, stuffing box and adjustable rod passed independent bore, surface and trajectory checks. The jogger’s print failure reproduced from 13 projected shapes. All fourteen polygon union tests passed after preserving the close tread corners in the slab calculation, and all twelve jogger paper samples passed. See the [oak](r3-vegetation-english-oak.md), [willow](r3-vegetation-weeping-willow.md) and [pumpjack](r3-machinery-oil-pumpjack.md) records.

The [crutch model](r3-humans-person-on-crutches.md) has functioning telescoping stock and a measured 0.2 mm radial sliding clearance on both sides. The [birch elevation](r3-vegetation-multi-stem-birch.md) now follows the full branch profile with 7.5 mm hard-edge error at 1:100. Its previous result exceeded the 30 mm limit. The [local memory checks](validation-memory.md) cover abandoned workers and fragmented reservations.

The [cyclist](r3-humans-cyclist.md) now carries a welded rear rack at every scale, with twin rails, rear dropout legs and an open deck. A Three.js test joins the model to its dimensions and checks guard clearance and the physical, thermal and structural references. Its twelve scenes passed independent readback.

The [numeric thermal audit](digital-catalog-heat.md) covers all 152 models in two climates across 936 named poses. All 14,522 surface fields passed the independent emission and flux comparisons. The cyclist, crutch user and English oak were also rendered with the shared temperature ramp at all four scales. Whole-model measurement status is recorded [by source revision](digital-volume-status.md).

The [walking commuter](r3-humans-walking-commuter.md) now has open, curved eyelids around the iris and pupil, alar wings around recessed nostrils and separate upper and lower lip contours at 1:50 and 1:100. The [rig verification](digital-catalog-rig.md) also covers every named pose in the catalogue.

[Nine further cervical studies](digital-cervical-review.md) cover 85 named poses. All 28 primary human figures now have a neck joint and a separate skull joint.

[Child passenger anatomy and motion](digital-passenger-review.md) covers the toddler on the hip and the child in the stroller. Seven exported FBX animations were reopened and compared with the posed joints.

The [seated domestic cat](r3-animals-domestic-cat.md) now wraps its five-part tail around the paw and retains correctly sized corneas in the thermal scene. The [tower crane](r3-machinery-tower-crane.md) passed the fine thermal exposure calculation. The [scale report](digital-volume-status.md) records strict checks for all 152 published sources. The [render index](digital-art-index.md) identifies the exact sources used in the four-scale body, temperature and structural review.

The [herring gull](r3-animals-herring-gull.md) now folds and spreads 44 distinct flight feathers over a measured 1.394 m gliding span. A check against the physical mesh exposed missing [wheelchair axle plates](r3-humans-wheelchair-user.md), [bicycle cranks](r3-humans-cyclist.md) and an exposed [pollard heartwood core](r3-vegetation-london-plane-pollarded.md). Their corrected sources passed at all four scales.

The [doe](r3-animals-goat.md) has a continuous articulated torso and separately posed lower-resolution limb segments. The [hornbeam hedge](r3-vegetation-hornbeam-hedge.md) now carries individually shaped leaves across six moving crowns and follows cross-row wind. The [crane heel](r3-machinery-tower-crane.md) now shows the actual tube and pin seats. The independent [mesh containment pass](digital-member-coverage.md) covers 608 current scales.

## Browsing doe

The published doe now carries a divided gland and teats on a compliant suspension. An independent probe measured the exposed skin in all 15 poses and the three scene types were checked across all four scales. [Measurements and reviewed revision](r3-animals-goat.md)

## November 21 follow-up review

The Labrador’s lumbar column and the rider’s running gear were inspected as separate engineering details as well as in context. The sheep’s profiled fleece and the patio heater’s mobile base were published after independent geometric checks. [Labrador](r3-animals-labrador-dog.md) · [skateboard](r3-humans-skateboarder.md) · [sheep](digital-sheep-fleece.md) · [heater](digital-patio-heater-wheels.md)

## November 21 anatomical and mechanical corrections

The [individual long-bone review](digital-human-long-bones.md) records actual shaft diameters. The [carried-child review](digital-toddler-proportions.md) measures the child independently from the adult. Individual reports cover the [umbrella and all human figures](digital-human-art-ledger.md), [animal poses and anatomy](animal-four-scale-audit.md), [road wheels and vehicle cabins](digital-road-wheel-review.md), [tower crane](r3-machinery-tower-crane.md) and [household objects and furniture](digital-equipment-art-ledger.md). Whole-catalog export, heat, structural and visual results are kept separately because different checks can be current on different source revisions.

## November 21 running gear and skeleton poses

The eight repaired adult skeletons were inspected on the installed files while walking, using a walking aid, gesturing or lifting a box. The coarse structures retain the separate, slender forearm and leg shafts where the former 75 mm capsules obscured them. The crane, mixer, forklift, crawler and scissor lift were inspected in physical, surface-temperature and structural views. [Adult dimensions](digital-human-long-bones.md) · [Moving joints](digital-running-gear.md) · [Per-model observations](digital-large-form-review.md)
