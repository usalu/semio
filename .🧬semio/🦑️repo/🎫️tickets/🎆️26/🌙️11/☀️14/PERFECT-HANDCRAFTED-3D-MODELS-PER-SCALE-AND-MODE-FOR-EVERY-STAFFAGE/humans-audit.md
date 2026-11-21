# Human staffage audit

The live catalog contains 28 figures. Each figure has four digital scales and three volume representations. The table records the baseline measured on November 20. The remaining findings concern thermal surface area. All other measured volume rules passed at that baseline.

| Figure | Body / thermal / structure solids at 1∶50 | Baseline thermal findings |
|---|---:|---:|
| [child-on-kick-scooter](r3-humans-child-on-kick-scooter.md) | 95 / 126 / 111 | 4 |
| [courier-with-hand-truck](r3-humans-courier-with-hand-truck.md) | 81 / 83 / 117 | 4 |
| [cyclist](r3-humans-cyclist.md) | 135 / 201 / 120 | 2 |
| [elderly-with-cane](r3-humans-elderly-with-cane.md) | 97 / 135 / 139 | 3 |
| [jogger](r3-humans-jogger.md) | 64 / 96 / 98 | 7 |
| [kneeling-gardener](r3-humans-kneeling-gardener.md) | 102 / 147 / 112 | 0 |
| [leaning-person](r3-humans-leaning-person.md) | 85 / 107 / 145 | 6 |
| [parent-carrying-toddler](r3-humans-parent-carrying-toddler.md) | 113 / 155 / 187 | 12 |
| [parent-with-stroller](r3-humans-parent-with-stroller.md) | 129 / 166 / 145 | 1 |
| [person-cooking](r3-humans-person-cooking.md) | 126 / 132 / 167 | 6 |
| [person-on-crutches](r3-humans-person-on-crutches.md) | 130 / 156 / 156 | 6 |
| [person-with-rollator](r3-humans-person-with-rollator.md) | 169 / 210 / 191 | 3 |
| [person-with-umbrella](r3-humans-person-with-umbrella.md) | 115 / 138 / 135 | 4 |
| [person-with-white-cane](r3-humans-person-with-white-cane.md) | 96 / 127 / 142 | 1 |
| [pregnant-person](r3-humans-pregnant-person.md) | 83 / 97 / 164 | 3 |
| [runner-with-blade-prosthesis](r3-humans-runner-with-blade-prosthesis.md) | 60 / 66 / 105 | 2 |
| [running-child](r3-humans-running-child.md) | 64 / 71 / 89 | 0 |
| [seated-guitarist](r3-humans-seated-guitarist.md) | 95 / 91 / 126 | 0 |
| [seated-office-worker](r3-humans-seated-office-worker.md) | 131 / 131 / 170 | 3 |
| [seated-reader](r3-humans-seated-reader.md) | 95 / 76 / 135 | 7 |
| [skateboarder](r3-humans-skateboarder.md) | 79 / 92 / 98 | 3 |
| [sleeping-person](r3-humans-sleeping-person.md) | 81 / 84 / 92 | 1 |
| [standing-adult](r3-humans-standing-adult.md) | 80 / 85 / 176 | 0 |
| [standing-phone-user](r3-humans-standing-phone-user.md) | 102 / 107 / 185 | 6 |
| [tour-guide](r3-humans-tour-guide.md) | 73 / 101 / 116 | 6 |
| [walking-commuter](r3-humans-walking-commuter.md) | 86 / 112 / 139 | 1 |
| [wheelchair-user](r3-humans-wheelchair-user.md) | 131 / 128 / 181 | 4 |
| [worker-carrying-box](r3-humans-worker-carrying-box.md) | 100 / 179 / 178 | 6 |

## Repairs verified in the first pass

Authored two coarse kneeling-gardener models with separate joints, trowel, hand contacts and knee mat. The print exclusions remain in the catalog. Rebuilt cyclist, scooter child and skateboarder thoraces with paired ribs and anterior sternum attachments. Corrected the skater’s shoulder girdle, face and neck and opened the cyclist and scooter helmets at the rim. Routed the running child’s coarse skeleton through the ankle pivot. Repositioned rollator heel contacts and reconciled cycle advance with wheel circumference. Corrected the box worker’s pelvic, hip, arm and torso angles in the squat grip and checked foot, carton and hand-hole contacts. Rebuilt courier glove fingers, coarse bicycle spokes and closed the hob engraving profiles. The public GLB writer and an independent reader verified all twelve scene variants for each of the courier, cyclist and cook.

## Verification scope

The gallery includes all scales and representations, with alternate-pose comparisons at 1∶50 and 1∶500. Region-colour thermal previews identify surface assignments. Contact probes use evaluated joint transforms. The figure-specific measurements, pose observations and visual reviews are recorded in the linked author reports.

## November 21 live corrections

The jogger’s heel lifted 13.7 mm during midstance and its forefoot lifted 19.3 mm during the hamstring stretch. The toddler carrier’s forefoot lifted 8.0 mm, the stroller parent’s forefoot 10.8 mm and the pregnant figure’s trailing ball 23.0 mm. The seven poses now place all named floor supports within 0.01 mm of the same plane. For the stroller the solver also checked both wheels. The jogger’s knee supplies the midstance correction within its ankle range. The pregnant figure’s shank and ankle rotations compensate each other to preserve trailing-foot pitch.

The tour guide’s right wrist now starts at the cuff (201 mm along the forearm) and ends at the hand overlap (231 mm). Its exposed neck starts at the jacket collar, and the face ends at the cap and hair. On the umbrella carrier, neck and face instead follow the shirt collar and bun hairstyle. On the carried toddler, the face ends at the actual hat rim of each scale. On the blade runner, skin stops at each elbow and wrist joint. These cuts retain the full detailed body in the ordinary depiction. The guide’s two skin regions pass the product’s independent 1∶50 and 1∶100 tolerance check after these changes.

The reader’s bench has four 1.20 × 0.10 × 0.04 m seat slats, three 1.20 × 0.08 × 0.03 m back slats and two side frames with a tie. The stock area is 3.315 m² before the assembly joins and 3.166 m² after the timber interfaces. The cook’s spoon combines a 244 mm shank and a 58 × 41 mm oval bowl. Its estimated exterior area is 0.013 m².

Body physiology derives naked segment area from the subject’s skin area and skin fraction. The solver divides exterior area by naked segment area to obtain the local clothing factor. Veselá et al. scanned dressed and undressed segments and measured different factors for different garment fits. Their reported shoe factors span 1.2–1.4 across three low shoe types. These observations inform assessment of footwear and garments without prescribing one factor for the entire figure. [Local clothing thermal properties of typical office ensembles](https://pmc.ncbi.nlm.nih.gov/articles/PMC6244820/)

The live November 21 full-rule sweep verified the scooter child and hand-truck courier with zero findings. It was stopped after these two assets at the user’s request for RAM headroom. The baseline table above remains historical.

## Exterior and metabolic bookkeeping

The local clothing factor uses exterior area divided by naked skin area. The reviewer estimated garment area from shell dimensions, textile panels and stocking height. Surface measurements from the product provide a separate check. Comparisons refer to paired shoes except for the crutch user’s single sneaker. Areas below are square metres.

| Figure | Clothing decision and physical basis | Whole-subject exterior / skin, before → after |
|---|---|---|
| Parent with stroller | Two 284 × 103 mm sneakers, sole and vamp about 0.085 per foot. Allocated 0.170 to shoes and the balance to the torso covering. | 1.161 → 1.161 |
| Cook | The fitted jeans and canvas sneakers use the standing adult’s matching garments, 0.6109 and 0.1424. The hoodie receives 0.8207. | 1.090 → 1.090 |
| Rollator user | A 270 mm felt beret has a 0.057 crown and 0.032 exposed underside annulus. The size-37 low shoes include the welt, collar and socks, 0.166 per pair. | 1.174 → 1.174 |
| Umbrella carrier | Both 276 × 102 mm derby shoes contribute roughly 0.13 upper and 0.06 sole and heel. | 1.244 → 1.244 |
| White-cane user | Chelsea boots have a 102 mm shaft, about 0.04 per shaft, 0.045 per upper and 0.022 per heel and sole. Use 0.21 for the pair. | 1.126 → 1.126 |
| Tour guide | The 233 mm rear uppers join the approximately 100 mm forefoot pieces. The combined pair contributes approximately 0.14 from the upper and 0.058 from soles and heels. | 1.177 → 1.177 |
| Wheelchair user | Jeans have a 224 mm seat panel and seated hip/knee joints. Use 0.716 for fitted denim and 0.174 for two sneakers. The hoodie receives the remaining 0.9989. | 1.139 → 1.139 |
| Pregnant figure | The high waist and seat cover approximately 0.16. Two 0.40 m thigh panels at mean 0.44 m girth and two 0.30 m lower panels near 0.33 m girth bring the leggings to 0.70. Two short shafts add about 0.06 to paired 0.164 low uppers and soles. | 1.093 → 1.233 |
| Seated reader | A seated seat panel near 0.17 plus two tapered legs near 0.295 each give 0.76. Use 0.154 for low shoes and 0.902 for the sweater. The lidded cup has 0.038 passive exterior, including taper, bottom and stepped lid. | 1.095 → 1.197 |
| Office worker | The 240 mm seat plus longer, looser upper and lower trouser sections give 0.85. Use 0.182 for the shoes and 1.05 for the shirt. | 1.092 → 1.195 |
| Crutch user | Two loose, 190 mm-wide short legs and the seat make 0.456. A tall unilateral walker boot has an enclosing calf shell, rocker sole and straps, 0.33. Its opposite 300 × 110 mm sneaker contributes 0.105. | 1.144 → 1.345 |
| Parent carrying toddler | A double-wrapped scarf with 198/154 mm ends contributes 0.135. The long coat trunk and skirts contribute 1.12, straight jeans 0.75 and shafted ankle boots 0.24. The skin under the adult boot shafts moves from the jeans to the boots. Denim below the coat hem is approximately 39% exposed when standing and 41–45% during the walk. | 1.202 → 1.593 |
| Carried toddler | Rubber boots approximately 156 mm long with two 45 mm shafts contribute 0.095. The cuffs cover about 0.021 m² of lower shin originally assigned to the suit. | 1.374 → 1.374 |
| Blade runner | The 137 mm socket shell with mean girth near 0.31 m plus the trim, distal end and adapter gives 0.060. The armhole-to-wrist skin area is 0.2668. Its skin allocation follows this boundary at the singlet. The exposed silicone suspension cuff adds its own 0.055 exterior between the shorts and the carbon brim. | 1.030 → 1.051 |

Every row retains the original subject skin area and total metabolic share. Fraction transfers are local to the same subject. Six outfits have a larger total outer area after review. Keeping the former totals would compress other garment surfaces below their naked skin allocation or omit fabric from the exterior estimate. The parent’s large increase includes inner jeans covered by the long coat. Their reduced exposure is applied individually in all ten poses.

The jogger previously assigned exposed cotton socks to bare legs in thermal views. The two 65 mm cuffs are now a separate textile region, with the existing cotton sock layer, approximately 0.030 m² outer area and 0.0289 m² underlying skin. The former leg region is divided at the cuff in each digital scale. Each of the 15 poses now carries a textile exposure separately from bare-skin wettedness. At coarse drawing scales the merged lower-leg shape carries both regions.

The hard-hat suspension now has a genuine section cut at the open underside of the shell and rim. The blade runner’s leg skin ends at the shorts, liner or sock. Two previous repairs had been lost in regeneration. The cook’s hob engraving annuli are closed again and the child’s coarse metatarsal load path runs through the ankle after updating the heel and distal landing to the latest proportions. The current authoring inputs and ticket volume snapshots were synchronized.

The carried toddler’s face ends at the hat rim. A roughly 110 mm band with a circumference increasing from the jaw to the brow, plus exposed ears and nose, gives 0.045 m². The adjacent suit allocation absorbs the collar boundary correction, preserving the toddler’s total. The blade runner’s silicone cuff above the socket is its own 6 mm textile-equivalent polymer layer. Under the carbon shell the original 6 mm silicone liner and 4 mm shell still act in series. Above the brim only the silicone exchanges with the air. Each pose, 2D view and digital scale includes this distinction. The 1∶500 sleeper’s duvet now has separate thigh and calf profiles across the knee, so its calf covering follows the shank during the 105° curl.

A sequential standard-library source check inspected 28 live documents, 243 named poses and 21,185 volume solids across the three lists. It found that every region had an authored thermal solid at every scale, every named pose had a thermal binding for every region, every thermal solid resolved its body reference, and every subject’s skin and metabolic allocations summed to one within 0.001.

## Latest product checks

The full catalog schema, semantic validation, all measured volume rules, and active floor contacts passed for the blade runner, jogger and parent carrying the toddler. The guide, umbrella carrier and box worker passed the product thermal coverage check separately at 1∶50, 1∶100, 1∶200 and 1∶500. The combined umbrella test reached the host memory limit while checking the entire figure, so its four thermal results were obtained in fresh bounded processes. The standard-library source check is reproducible with `hu-coverage.py`.

### Secondary numerical check

Re-evaluating the existing round-five exterior measurements against the live declarations yielded 374 in-tolerance region/scale comparisons across 21 figures at 1∶50 and 1∶100 that did not receive new thermal geometry in this pass. This uses archived measurements. The six high-risk figures were checked again directly as recorded above. The updated 1∶500 sleeper passed fresh schema, semantic, active floor-contact and volume checks after the calf mound was split from the thigh.

### Surface temperatures, pose and climate

The product solver evaluated each of the 243 named human poses twice, first in the authored climate and then at 14 °C in shade. Every pose produced finite core and region surface temperatures, sensible heat and latent heat. Independent passengers remained separate subjects. In the shaded condition the jogger’s bare leg, sock cuff and shoe reached 25.97, 25.55 and 19.05 °C respectively. The blade runner’s bare leg, silicone cuff and carbon socket reached 31.42, 25.82 and 24.31 °C. These are model predictions for the specified inputs. The test input is `hu-room.ts`.

The GLB writer and independent reader checked all 36 body, structure and thermal scenes of the cook, running child and sleeper. All three passed the geometry checks. Before accessor sharing, the files occupied 12,108,696, 8,463,840 and 10,271,824 bytes respectively, each beyond the product's 8 MiB budget. After the shared-accessor repair those same exports occupied 6,472,584, 4,430,040 and 5,201,096 bytes and all 36 scenes passed the independent reader. The anatomical changes below were authored subsequently.

The localized thermal discrepancy diagnostic now prints the limit that the rule applies to the measured region. A focused product regression measured 0.178 m² against 2 m² for the feet at 1∶50 and reported the correct 30% limit in both languages.

## Vertebrae, ribs and skeletal materials

At 1∶50 all 28 primary figures now model twelve pairs of ribs. The two passenger figures have twelve further pairs each. The guide and umbrella carrier had closed rings with a solid ellipsoid for the entire 1∶500 chest. Their replacements follow independent costovertebral, lateral and sternal stations, retain the costal margin, and leave the two lowest pairs free at 1∶50. The initial product write and independent read of their 24 scenes passed, at 6,041,816 and 7,629,596 bytes respectively.

The courier, elderly cane user, gardener, both parents, crutch user, white cane user, blade runner, guitarist, office worker, reader, sleeper and wheelchair user each had six to ten annular ribs in the fine skeleton. Their replacement courses follow figure-specific vertebral heights, side reaches and anterior heights defined in `hu-rib-charts.py`. The elderly cane user’s more posterior thoracic curve, the seated reader’s narrower shoulders, the wheelchair user’s seated posture, the blade runner’s lean chest and the gardener’s flexed torso retain their original coordinates. At 1∶100 six pairs illustrate the upper opening, sternal wall, costal arch and floating ribs. At 1∶200 three pairs mark ribs two, seven and ten. At 1∶500 the two retained pairs meet a separate sternum.

The rollator user previously attached rib two near T1 and rib ten below T12. Each of the twelve new pairs begins on the middle of its own published vertebral body. The existing coarse spine carries the reduced rib courses. The scooter rider, cyclist, skateboarder, jogger, running child, phone user and box worker retain their hand-shaped fine arches through an open coarsest model. The first three also replace their filled 1∶200 mass. Torso structure remains open at each of the 112 primary human scales.

Both children have independent anatomy. The stroller passenger's spine runs cranially along local Y, 192.5 mm from the stroller's centre plane. The carried toddler uses a separate torso frame. The two sets of sternal lengths, rib widths, dorsal levels and shallow falls are defined in `hu-passenger-chest.py`. Each child's ribs follow the child through its named carrier or stroller poses.

Independent measurements in `hu-spine-check.py` tested each retained rib head against the corresponding spinal section, true ribs against the sternum, lower cartilage against rib seven and floating ends against the midline. The 16 full reconstructions passed all 64 scale checks. The tests compare distances and radii directly in the authored coordinates.

Twelve figures used skin as the material of anatomical members from cranium to toes. Across the four scales 2,999 assignments changed, including the nylon pack mass and cardboard carton mass. Bones now use the existing catalog bone material with density 1,900 kg/m³. The checks in `hu-materials.py` cover all 28 figures and 112 volumes. They resolve all body, thermal and structural material IDs. The live documents retain 1,205 outer skin solids and 381 explicit thermal skin solids. Abstract segment centre-of-mass markers and grasp contacts retain their diagrammatic roles. The source pass after the cartilage and crutch corrections covers 21,873 body, thermal and structural entries and all 243 named human poses.

The measured passenger and reduced-chest checks in `hu-other-chests.py` passed all 18 additional scale/subject combinations. A frontal and lateral comparison of all 28 detailed skeletons exposed interrupted anterior attachments in the box worker, standing adult, phone user, leaning figure, cook and pregnant figure. The minimum clearances between the original bony rib tip and the sternum reached 32 mm. Forty-four separate cartilage connections now bridge those gaps. Each bar starts inside its original bony end and terminates inside the local sternum. `hu-cartilage-proof.py` measured all 44 joints. The smallest overlap between the joined surfaces was 6.1 mm.

## Guarded product verification of all human exports

All 28 human documents passed schema, semantic, active floor-contact and structural rules. These include minimum features, slender-member qualification, structural coverage, differing scale geometry, decreasing model complexity, real solid definitions and resolved bone assignments. Each row below also passed the product GLB encoder, independent mesh and scene readers, triangle budgets and the 8 MiB file limit in its authored presentation pose. Every row contains four digital scales and all three representations.

| Figure | Named poses | GLB scenes | GLB bytes |
|---|---:|---:|---:|
| child-on-kick-scooter | 47 | 12 | 4,453,824 |
| courier-with-hand-truck | 4 | 12 | 5,264,852 |
| cyclist | 4 | 12 | 6,129,732 |
| elderly-with-cane | 9 | 12 | 6,839,848 |
| jogger | 15 | 12 | 4,929,468 |
| kneeling-gardener | 6 | 12 | 5,764,732 |
| leaning-person | 5 | 12 | 5,331,016 |
| parent-carrying-toddler | 10 | 12 | 7,940,736 |
| parent-with-stroller | 14 | 12 | 7,684,240 |
| person-cooking | 3 | 12 | 5,725,196 |
| person-on-crutches | 3 | 12 | 7,168,696 |
| person-with-rollator | 4 | 12 | 8,369,128 |
| person-with-umbrella | 5 | 12 | 6,806,008 |
| person-with-white-cane | 4 | 12 | 6,549,140 |
| pregnant-person | 4 | 12 | 5,262,820 |
| runner-with-blade-prosthesis | 27 | 12 | 4,411,956 |
| running-child | 11 | 12 | 3,879,156 |
| seated-guitarist | 5 | 12 | 4,802,904 |
| seated-office-worker | 3 | 12 | 6,318,804 |
| seated-reader | 6 | 12 | 4,704,464 |
| skateboarder | 10 | 12 | 4,368,188 |
| sleeping-person | 3 | 12 | 4,608,096 |
| standing-adult | 4 | 12 | 5,850,448 |
| standing-phone-user | 3 | 12 | 6,497,676 |
| tour-guide | 7 | 12 | 5,393,668 |
| walking-commuter | 8 | 12 | 6,983,004 |
| wheelchair-user | 4 | 12 | 6,124,500 |
| worker-carrying-box | 15 | 12 | 7,808,264 |

The courier originally had more thermal triangles at 1∶500 than 1∶200. The opaque wool cap now covers the cranial portion and the exposed thermal face stops at the cap rim in both scales. Both scales passed the full product volume checks. The rollator figure's first export occupied 8,809,940 bytes. In its cardigan, blouse, tights, stockings and intervening air layers, 823 interpolated interior rings reduced to 448. The outer body geometry and enclosed layer identities are preserved. Boundaries were sampled at 32 angles per ring. The largest displacement across discarded rings was 0.749 mm and the final file occupies 8,369,128 bytes. The combined rollator 1∶50 check reached the 1.15 GiB memory guard. Its fresh, isolated 1∶50 thermal coverage check passed.

### Forearm-crutch contact

The fine and intermediate ferrules previously ended 3.33 mm short of the physical tip anchors. The steel shaft and rubber contact profile now reach the same bearing point at all four scales. The handgrip starts inside the steel shaft. At 1∶500 the rubber grip and ferrule are now visible parts shared by the body, structural and thermal models. Their terminal guards widen at the display scale. The hand still grasps the 18 mm radius section and the contact pad at the ground keeps its 40 mm footprint.

Both 1∶200 and 1∶500 hands now have an open channel following the actual crutch grip axis. The opposing thumb and curled fingers form a continuous section around the handle. All three authored poses retain the same grip relationship. At 1∶200 a 17.0 mm opening contacts a 17.15 mm grip radius. At 1∶500 the opening is 16.39 mm against an 18.0 mm grip radius. The ferrule ends are within 0.02 mm of the tip anchors. The finished figure passed the full 1∶200 and 1∶500 volume checks, four-scale structural checks and all twelve GLB scenes, at 7,168,696 bytes. `hu-crutch-contact.py` independently measures the positions, opening, rotation, contact and rubber material bindings.

## Live limb proportions

These are distances in millimetres from shoulder to elbow, elbow to wrist, hip to knee and knee to ankle. Each distance uses all three local translation components of the immediate distal joint. Left and right values share a cell when they round to the same millimetre. The first section links the underlying individual anthropometric and posture reviews.

| Figure | Upper arm | Forearm | Thigh | Shank | Largest difference between joint-centre distance and authored length |
|---|---:|---:|---:|---:|---:|
| child-on-kick-scooter | 229 | 179 | 302 | 293 | 0.0 |
| courier-with-hand-truck | 337 | 265 | 444 | 434 | 0.0 |
| cyclist | 319 | 251 | 404 | 423 | 0.0 |
| elderly-with-cane | 307 | 242 | 405 | 395 | 0.0 |
| jogger | 336 | 279 | 445 | 434 | 0.0 |
| kneeling-gardener | 292 | 255 | 406 | 396 | 0.0 |
| leaning-person | 318 | 252 | 420 | 410 | 0.0 |
| parent-carrying-toddler | 319/317 | 250/253 | 400 | 418 | 5.0 |
| parent-with-stroller | 308 | 268 | 428 | 418 | 0.0 |
| person-cooking | 314 | 247 | 413 | 404 | 0.0 |
| person-on-crutches | 340 | 266 | 446 | 436 | 1.3 |
| person-with-rollator | 294 | 231 | 387 | 389 | 0.0 |
| person-with-umbrella | 318 | 249 | 420 | 418 | 1.9 |
| person-with-white-cane | 312 | 245 | 412 | 413 | 0.0 |
| pregnant-person | 310 | 244 | 407 | 397 | 0.0 |
| runner-with-blade-prosthesis | 330 | 260 | 435 | 425 | 0.0 |
| running-child | 185 | 146 | 256 | 262 | 0.0 |
| seated-guitarist | 327 | 257 | 430 | 435 | 0.0 |
| seated-office-worker | 330 | 260 | 435 | 438 | 0.0 |
| seated-reader | 312 | 245 | 411 | 413 | 0.0 |
| skateboarder | 335 | 258 | 418 | 438 | 0.0 |
| sleeping-person | 315 | 248 | 400 | 418 | 0.0 |
| standing-adult | 314 | 247 | 413 | 404 | 0.0 |
| standing-phone-user | 314 | 247 | 413 | 404 | 0.0 |
| tour-guide | 326 | 256 | 411 | 431 | 0.0 |
| walking-commuter | 330 | 260 | 435 | 425 | 0.0 |
| wheelchair-user | 323 | 255 | 426 | 430 | 0.0 |
| worker-carrying-box | 314 | 273 | 420 | 425 | 0.0 |

Across 223 measured natural-limb segments the largest difference was 5.0 mm. Longer three-dimensional spans reflect lateral placement of the distal joint. The right lower-leg prosthesis is excluded from the natural shank comparison.

## Independent whole-subject heat balance

The thermal solver ran the 243 named poses of all 28 human figures at their authored climate and at 14 °C in shade. Each run checked every surface temperature and heat flux. Summing the outer conduction and evaporation of each subject’s own physical regions, external work, respiration and storage reproduces metabolic generation. The 534 adult, child and passenger balances had a maximum absolute residual of 4.55e-13 W.

## Small garments, metal frames and enclosed air

A cross-check of every thermal solid's resolved material identified 37 garment air-gap declarations and 161 interior gap solids on eight figures that carried the jacket, trouser, hat or shoe material. Their original enclosed geometry and fitted resistance now refer to the trapped air. The carrier’s adult and toddler scalp surfaces remain skin. The adult hair is hair, and the external surface of the two knit hats, the guide’s cap, the skateboarder’s beanie and the leaning figure’s beanie is classified as clothing. The cook’s and pregnant figure’s eyebrow meshes and drawings now share the hair field.

The sleeper’s 1:50 pyjama shorts now have a separate cotton leg opening on each moving thigh, extending 119 and 113 mm from the hip. The inner thermal surface follows these cuffs and the actual cotton pelvis. The uncovered thigh and shin geometry beneath the duvet remains skin. The jogger’s 26 mm high sweatband exchanges through 0.0154 m² of nylon. The free hair at the crown and nape has separate surfaces cut at the garment edges. The guide’s 86 × 54 mm name badge and small attached clip share a passive 0.0052 m² surface, with its back against the jacket.

The elder’s and rollator user’s spectacles have independent passive heat fields, 0.00490 and 0.00429 m² respectively. The two rim toruses, actual temple lengths and 1 mm wire bridges determine the areas. The bridges span the inner rim edges over the nose, 23.062 and 21.584 mm respectively, and each temple starts on its outer rim. The 1:50 printed symbols and all four digital thermal scales carry the frame material. Coarser digital models instance the two equal rims while preserving the bridge and flared sides.


The five figures with pigmented brow meshes now use hair material. The exposed brows on the two knit-hat wearers follow the local skin temperature of the forehead. Pale, raised supraorbital anatomy on the other figures stays skin-covered. In the 14 °C shaded test, the spectacle frames measured 14.52 °C and 14.51 °C, while the adjacent faces measured 26.35 °C and 26.26 °C. The jogger’s nylon band reached 26.35 °C and free hair 24.24 °C. The guide’s passive badge reached 18.23 °C. In coarse print the visible temple and badge edge use their own profiles where legible, and the small remaining regions share the head or jacket outline. The latest source-hashed checks are listed in [the per-scale report](human-thermal-scales.md).

The final 28 human source digests each pass the strict thermal-coverage rule at 1:50, 1:100, 1:200 and 1:500, and independently parsed exports for all twelve scale and scene combinations. The four-scale table records every digest, result and GLB byte count. All 243 named poses in two climates returned finite thermal fields. All 534 individual energy balances closed to 4.55 × 10⁻¹³ W.

## Authoring-document reconciliation

All 112 human volume scales match between the live catalog and the ticket authoring documents. The retained 28 full catalog fingerprints were matched against the final heat-field proof before the authoring document comparison. The source comparison is reproducible with `hu-authoring-mirrors.py`.
