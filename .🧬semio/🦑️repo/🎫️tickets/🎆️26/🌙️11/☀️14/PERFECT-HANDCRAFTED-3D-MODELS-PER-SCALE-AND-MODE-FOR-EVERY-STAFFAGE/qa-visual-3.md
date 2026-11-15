# Visual QA 3: humans and animals

Third independent visual reviewer (prefix `qa3-`). Method: for every asset, fresh sheets (`volumes-render.ts --no-numbers`) of every authored scale plus three axonometric directions (az -55 el 28 front-left-top, az 215 el -12 rear-right-low, az 90 el 4 straight front; humans and animals face the camera at az 90), body / thermal (cutaway) / structure, stitched per scale (`qa3-stitch.ts`: sheet + 3 rows) and per asset (`qa3-coarse.ts`: front-left-top and straight-front rows of 1:100, 1:200, 1:500). Scripts: `T/qa3-run.sh`, `T/qa3-all.sh`, `T/qa3-stitch.ts`, `T/qa3-coarse.sh`, `T/qa3-coarse.ts`; images under `T/🗑️generated/qa3/<slug>/`; working notes `T/🗑️generated/qa3/notes.md`.

Grades: A perfect, B minor (named), C reads wrong (named). Columns body / thermal / structure. `-` unavailable.

## Coverage, read this first (the review is NOT complete)

The image viewer (Read tool PreToolUse hook) stopped responding after 21 of the 42 assets, so the rest could not be looked at; I do not grade what I did not see. All 42 assets were rendered (the sheets exist under `T/🗑️generated/qa3/<slug>/`, state of about 23:10 on 2026-11-14).

- Graded (21): hedgehog, jogger, child-on-kick-scooter, cyclist, parent-carrying-toddler, person-with-rollator, person-with-white-cane, runner-with-blade-prosthesis, skateboarder, worker-carrying-box, running-child, horse, dairy-cow, sheep, red-fox, red-squirrel, goat, mallard-duck, herring-gull, pigeon, roe-deer.
- Rendered, NOT graded (21): humans: seated-office-worker, wheelchair-user, seated-guitarist, person-on-crutches, courier-with-hand-truck, kneeling-gardener, standing-adult, elderly-with-cane, leaning-person, pregnant-person, standing-phone-user, walking-commuter, person-cooking, person-with-umbrella, tour-guide, seated-reader, sleeping-person, parent-with-stroller; animals: domestic-cat, labrador-dog, pig. Of the Ready list only hedgehog and jogger were graded; domestic-cat, elderly-with-cane, leaning-person, pregnant-person, standing-adult are still open (their sources were changing in the last 15 minutes when I started anyway).
- Moving sources: while I worked, authors changed horse, dairy-cow, sheep, child-on-kick-scooter and person-with-rollator (source mtime 0-4 minutes old at about 23:15); my grades for these describe the state at about 23:08 and need a re-look.

## Summary of what is wrong most often (read this first)

1. **The coarser scales are the same figure with fewer parts, then boxes.** In humans, from 1:100 on, thermal is ONE grey region for torso, arms, legs (only face, hat, hands, shoes keep colours); clothing layers and skin vs cloth are gone (cyclist, rollator, white-cane, skateboarder, parent+toddler, worker, runner). At 1:500 the head is a hexagonal or rectangular column, hands are gone and the structure list is a solid copy of the body (column legs, cylinder arms).
2. **Humans are mannequins, not figures.** Box torso without waist or chest/shoulder shaping, tube arms with a visible shoulder cap sphere, flat pointed blade hands, straight uniform tube legs with knee ring seams (jogger, cyclist, child), oversized flat wedge shoes (white-cane, worker, runner), tiny heads (about 1/7.5 of height), no neck slope. Poses read when the pose carries them (cyclist, rollator, skateboarder, scooter) and fail when it does not (running-child standing stiffly, runner-with-blade upright).
3. **Child proportions are missing.** Child on scooter and running child have the head ratio and limb length of a small adult (head about 1/7, long legs); a child needs about 1/6 head, short legs, shorter forearms.
4. **Animals are built from stacked boxes/cylinders.** Torso is a segmented tube with visible seams (horse, cow, goat, fox, deer), no ribcage, no tucked belly, no shoulder blade or haunch mass; legs are uniform sticks with little joint form; necks are cones or stacks of rings (horse neck is giraffe-thin from the front). Heads at 1:500 turn into a flat floating disc (roe deer), a T-bar (goat) or a column.
5. **Detached appendages.** Ears of squirrel and roe deer hover above the head with a gap; wings of duck, gull and pigeon are separate leaf/blade/sausage solids laid on the flank, not folded wings with thickness; hair strands, breath and specks float next to the head (parent+toddler 100-500 thermal, white-cane 100 thermal at the foot).
6. **Thermal regions wrongly shaped**: a flat disc floating above the back in every four-legged animal (sheep, goat, deer, fox, cow: dorsal/wool region), a huge half-disc plane hanging from the rump to the hoof in the cow, a vertical grey plate standing through the back of the duck, a green cube floating at head height in worker-carrying-box 1:500 rear view.
7. **Structure joints are cartoon balls**: orange joint spheres bigger than the members they join (squirrel, sheep, roe deer, child 1:200); ball-chain instead of a skeleton; leg rods disconnected from the spine chain at 1:200 (sheep, cow). Where a spine chain with leg chains exists (horse, cow, goat, duck 1:50) it is the best list.
8. **Overlay vs 3D**: the green overlays were mostly right; where red (drawn, missing in 3D) shows, the 3D lacks hooves/feet (hedgehog front legs, pigeon tail and toes, dairy cow tail).

## Table: asset x scale x (body / thermal / structure)

| asset | 50 | 100 | 200 | 500 |
|---|---|---|---|---|
| hedgehog (Ready) | C- / B / B- | C / B / B- | - | - |
| jogger (Ready) | B- / B+ / B | B- / B / B | B- / B- / B | B- / B- / C+ |
| child-on-kick-scooter | B- / A- / B | B- / B / B | B- / B- / B | B- / B- / B- |
| cyclist | B / B+ / B+ | B / B- / B | B / B- / B | B- / B- / B |
| parent-carrying-toddler | B- / B / B- | B- / C+ / B- | B- / C+ / B- | C+ / C+ / C+ |
| person-with-rollator | B / B+ / B | B / B- / B | B- / B- / B- | B- / B- / B- |
| person-with-white-cane | B- / B / B | B- / B- / B | B- / B- / B- | B- / B- / B- |
| runner-with-blade-prosthesis | B- / B / B- | B- / B / B- | B- / B / B | C+ / B- / B- |
| skateboarder | B / B+ / B | B / B- / B | B- / B- / B | B- / B- / B- |
| worker-carrying-box | B- / B+ / B | B- / B- / B- | B- / B- / B- | B- / C+ / B- |
| running-child | C+ / B / B | C+ / B- / B | C+ / B- / B- | C+ / B- / B- |
| horse | B- / B / B | C+ / B / B | C+ / B- / B- | C / B- / B- |
| dairy-cow | C+ / C+ / B | C+ / C+ / B | C+ / C / B- | C+ / C / B- |
| sheep | B- / B- / B | B- / B- / B | B- / B- / B- | B- / B- / B- |
| red-fox | B- / B / B | B- / B / B | B- / B- / B- | - |
| red-squirrel | C+ / C+ / C+ | C / C+ / C+ | - | - |
| goat | B / B / B | B- / B / B | B- / B- / B- | C / B- / B- |
| mallard-duck | B- / C+ / B | B- / C+ / B- | B- / C+ / B- | - |
| herring-gull | C+ / B- / B- | B- / B- / B- | B- / B- / B- | - |
| pigeon | C+ / B- / B- | C+ / B- / B- | C / B- / B- | - |
| roe-deer | B / B / B- | B- / B / B- | B- / B / B- | C / B- / B- |

(Not graded, see Coverage: 21 assets.)

## Instructions per asset (every B and C)

**hedgehog** (Ready; disagree with green: body C-). 50/100: the foraging front legs are two huge barrels flanking the snout (front view: two giant discs) and shorter than the drawn legs (red feet in the overlay); rebuild forelegs as 40 mm-wide tapered legs with paws at ground level, drawn height. Back is one smooth ellipsoid: spines are not a texture here, add a spine-coat body solid with a rim (spine fringe over the forehead and flanks) as an offset shell, plus eye and ear stubs on the face, snout cone shorter. Structure 100: copy of the body; model ribs/spine chain and four leg chains.

**jogger** (Ready; mostly agree, B-). Torso: loft with waist and shoulder slope instead of a cuboid with collar flap; arms: add elbow bend and a real hand (mitten shape, 4 fingers merged, thumb); legs: loft thigh/calf with taper, remove knee rings; shorts as hem with flare not dark boxes. 500: head octagon -> small ellipsoid, structure must be members, not the body.

**child-on-kick-scooter**: child proportions: head 1/6 of height (raise head radius about 15 %), shorten legs and arms about 8 %, mitten hands. Deck as thick slab with a fender and brake plate, wheels with tyre torus.

**cyclist**: hands/feet right; arms: remove ball shoulder caps, merge into deltoid; torso loft with a waist; 100/200/500: bring back clothing regions in thermal (jacket vs trousers vs skin, as at 1:50); 500 wheels: use rim torus 12+ segments, not twisted rope.

**parent-carrying-toddler**: parent's holding arm must wrap the toddler (forearm under the thighs, hand at the toddler's back); coat as tube with a waist and hem opening, legs visible; toddler head larger (1/4 of its height) and legs hanging with knees bent. Thermal 100-500: separate parent and toddler regions again, remove the floating hair/breath fragments near the head, 500 structure: members not a solid.

**person-with-rollator**: keep; torso/arms as for humans above; thermal 100-500: split person into clothing/skin regions; 500: hands should touch the grips (currently balls beside the grips).

**person-with-white-cane**: shoes as shaped shoes about 70 % of the current length; remove horizontal band rings on the jacket (hem and belt as thin lofts); head hood with face opening; cane body tone to white if tones allow; thermal 100: remove the stray specks at the foot; 500: shoulders/torso wedge stack -> one loft.

**runner-with-blade-prosthesis**: pose is not a runner at all (upright, hanging arms): lean 8-10 degrees, forward stride with the blade leg back, arms at 90 degrees with swing; blade: a carbon J-spring 20 mm thick with taper instead of a flat strip; torso thicker (currently a pole), arms attached to shoulders not hugging the torso; remove the chest bib rectangle. 500: blade and foot as separate readable shapes.

**skateboarder**: arms: add elbow bends (both arms currently straight lines from the shoulder), hands mitten; board 15 % longer, wheels bigger; thermal 100+: split hoodie/trousers/shoes.

**worker-carrying-box**: second arm and hand must visibly hold the box side; box with tape line/handle holes (decor on body only if allowed) else plain; boots 70 % of the current length, not splayed; waist on the torso; 500 thermal: remove the green cube at head height (box thermal solid misplaced: check its transform).

**running-child**: pose standing-ready: if the asset is a running child, author a running pose (forward lean, arm bend, one foot off the ground); proportions as child (head 1/6, shorter legs); chest square patch remove; shoes as small shoes not slippers; 1:200/1:500 head as ellipsoid not cylinder.

**horse**: neck: build a tapered loft from the withers (wide, 300 mm) to the poll, with a crest curve; head with a deep jaw and broad muzzle, ears shorter and set wide; barrel: oval loft with ribcage depth (belly lower than the elbow line, tuck at the flank), remove the box seams, shoulder and haunch masses as ellipsoids blended into the barrel; legs: cannon bone thinner than forearm, hock angle on hind leg, hooves 50 % larger; add mane (hair kind, thin crest solid) and a hanging tail with a root at the croup. 100-500: body at least the same overall mass, front view must not read as a plank; 500: neck as angled wedge, not a vertical stick.

**dairy-cow**: head: broad muzzle and forehead lofts, ears horizontal, horn stubs; neck and head are not one cone; barrel as one oval loft with a pelvis and shoulder shape (hip bones as a subtle ridge, not box corners), udder as a single bag solid with teats; thermal: remove the huge red half-disc hanging from the rump to the hooves (fix the transform/size of the region solid), thermal hide patches stay; legs: hocks bent, thicker upper legs. 200: connect leg rods to the spine chain (joints at the rod ends).

**sheep**: wool as an uneven fleece volume (loft with rounded shoulder, flank, rump, chest; slightly narrower waist) instead of a capsule; head wedge with a muzzle, ears, neck short but present; thin black legs with hock bend; thermal: the dorsal disc must lie on the back, not float above; 200: connect leg rods to the chain; remove oversized orange balls.

**red-fox**: body with chest depth and a tucked belly, neck section behind the head; tail as a bushy ellipsoid-lofted brush (not a plank) with a white tip; legs with joint structure (hock), black stockings by tone; ears as triangles on the skull.

**red-squirrel**: ears attached to the skull (touching, base inside the head), tail as an S-curved loft (3 sections) rising behind the back with a tuft; paws at the mouth with fingers; hind feet shorter; thermal ear solid must sit on the ear, not above the head; structure: thin members (spine, limbs) with small joint spheres (at most 1.3 x member diameter); 100: recover arms and hind feet (currently a pin with ears); fill tail S-curve.

**goat**: body as in the horse (single barrel loft, rib depth); neck as tapered loft; keep horns and beard; legs thicker above the knee; 500: horns/ears as a pair of small wedges on the skull, never a flat bar above the head; thermal ear plate must touch the ear.

**mallard-duck**: neck shorter by 30 % and tapered (thick at the breast), head round with a flat bill, wing as a folded wing with thickness on the body (loft partly inside the body), tail short upturned point; legs more central, thicker thighs hidden in the flank; thermal: remove the vertical grey plate through the back (check its rotation); 100/200: wings attached to the body, not diverging leaf sticks.

**herring-gull**: bill yellow and hooked with a red spot at the tip (two solids), head as a rounded ellipsoid with a brow, wings folded along the back with primaries beyond the tail (loft lying on the back, thickness 25 mm at 1:50), no two blades sticking up at 20 degrees; webbed feet plates with three toes; keel visible under the breast; structure: wings must be joined to the body chain via the shoulder joint.

**pigeon**: wings as folded wings lying over the back/tail (loft with taper), not boxes on the flank; head as an ellipsoid with a short neck and a bill with a cere (white), eye; tail a short fan as 3 overlapping thin plates at the correct height (matches the drawn tail: overlay red); feet with three toes forward and one back at 40 mm; 200: restore legs and a clear head, no bean bag.

**roe-deer**: torso with a ribcage and a clear flank tuck; neck tapered; head with muzzle and eye; hind leg with hock angle; add a white rump patch (tone) and a tail stub; ears attached to the skull; 500: head must be a wedge with muzzle and ears (currently a floating disc); thermal ears touch the head.

## Disagreements with the green validator

- **hedgehog**: Ready, but body C- at 1:50 and 1:100 (barrel forelegs, no spines).
- **jogger**: Ready; agree on B-, but 1:500 structure (solid copy of the body, C+) is not a load path.
- Every overlay was green or nearly so for the assets that read wrong (cow, horse, squirrel, pigeon); the validator cannot see floating ears, flat discs above the back, hanging rump half-disc, T-bar heads or boxy torsos.

## Family consistency of the humans (judged on the 10 graded humans)

One family: yes, in construction and style, not in quality. They share the same kit (box torso, tube limbs with ball shoulder caps, blade hands, wedge shoes, hat/cap/hood variants), so they read as figures of one system. They do not share quality of proportion: cyclist and skateboarder are good, the rest vary:

- **Outliers (worse)**: running-child (standing upright instead of running, adult proportions), runner-with-blade-prosthesis (skinny pole, upright), parent-carrying-toddler (long tube coat, toddler floating in front), person-with-white-cane (oversized flat shoes, armoured jacket bands), worker-carrying-box (splayed boots, box in front of a pole body).
- **Outliers (better)**: cyclist, person-with-rollator, skateboarder (pose and props clearly modelled).
- Shoes differ most (wedge boot, slipper, boat, small shoe), head sizes differ (cap vs hood), hand shape is a pointed blade everywhere (one shared flaw).
- Remaining 18 humans were not viewed; no verdict on them.

## Family consistency of the animals (graded 11)

One family again (segmented barrel + stick legs + cone head), uneven: goat and roe-deer best, sheep next, horse/cow/fox middling, squirrel/pigeon/gull/hedgehog worst. Birds (duck, gull, pigeon) each have different wing construction (leaf, blade, box slab): no shared bird kit, so they do not look alike.

## Recurring weaknesses (action list)

1. Introduce a shared torso/limb/hand/shoe kit upgrade for humans (waist loft, deltoid, mitten hand, shaped shoe, neck), then rebuild coarser scales from it.
2. Thermal at 1:100-1:500: keep skin vs clothing regions; fix floating or misplaced solids (discs above animal backs, cow rump half-disc, duck plate, worker cube, ears).
3. Structure at coarse scale: members (spine, limbs) with joints at joints, never the body copy; joint spheres at most 1.3 x member diameter.
4. Animal barrel as one loft with ribcage and flank tuck; stop stacking cylinders; heads with muzzle and attached ears at every scale.
5. Birds: one folded-wing construction (loft lying on the back with thickness) shared by duck, gull, pigeon.
6. Child and parent proportions.
