# Catalog fit audit: handcrafted staffage for course clips

Read-only audit of `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/` for the clip work in ticket `HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS`. Read on 2026-11-01. No repository file was modified and no git command was run. Sources: every `🔣️.json` under the catalog (109 asset documents, the root manifest, `🌤️environments`, `🖨️print-profiles`), and the cast contract `contract.md` in this ticket folder.

## Snapshot

- **Assets:** 109 asset documents and 108 manifest entries. The 108 manifest entries match their asset files one-to-one by id, category, height and viewpoints, and every pose and clip slug they list exists in the asset file. The manifest lists only a curated subset of poses and clips (for example `jogger` lists 5 of 15 poses), so the asset file is the authority for slugs. `toaster` is authored but not yet in the manifest.
- **Categories:** humans 25, animals 14, vegetation 16, furniture 11, objects 11, vehicles 8, machinery 8, components 15, appliances 1 (`toaster`).
- **Appliances:** declared in the manifest (`🔌️appliances`). The folder now holds `toaster`, authored but without a manifest entry. Vacuum-cleaner and electric-fan-heater are not authored yet.
- **New assets at read time:** `standing-adult` (in the manifest) and `toaster` (not yet in the manifest) exist. Still missing: seated-office-worker, sleeping-person, person-cooking, laptop, desktop-monitor, pendant-lamp, vacuum-cleaner, electric-fan-heater.
- **Environments:** `🌤️environments/🔣️.json` has 0 presets. Thermal views that need an environment timeline cannot be cast yet.
- **Print profiles:** `🖨️print-profiles/🔣️.json` holds print profiles only, no assets.

## Column meanings

- **Thermal binding.** `MET lo–hi (base b)` means the asset carries a metabolic rate (activity MET per pose, base MET). `breath + plume emitters` marks living bodies. `heat-source emitters` lists exhaust, flue, vent or fan outlets. `heat source op` marks a thermal operating state that releases heat (heating, burner, charcoal, lid heat, refrigerant duty, lamp, hydraulics, beacon, elements). `state op` marks a power or operating state with no heat-source meaning (for example the desk monitor, engine load, phone state). `transpiration emitters` marks vegetation leaf heat and water vapour. `none` means no thermal binding.
- **Plan.** `yes` means `presentation.viewpoints` includes `plan`. Only 5 assets have it: `agave`, `bamboo-clump`, `hosta-clump`, `ivy-on-trellis`, `spiral-stair`. All others are elevation only.
- **Heights** are `heightM`, with width and depth in metres. Clip durations come from `durationS`; `(loop)` marks looping clips.

## Clip use mapping

Best available asset, pose or clip for each common need. Status: **OK** means fit as is. **Interim** means usable with a stated mismatch. **GAP** means nothing usable in the archive yet.

| Need | Best asset + pose | Clip (duration) | Thermal | Status and notes |
|---|---|---|---|---|
| Generic standing person | `standing-adult`, pose `relaxed` (1.67 m) | `idle-breathing` 5 s (loop) | MET 1.2–1.3 | OK. Alternative: `walking-commuter` pose `standing` (1.78 m). |
| Seated person at desk | none. Nearest: `seated-reader` pose `seated-upright` (1.34 m), which sits on a park bench, not a desk | `page-turn` 3.6 s | MET 1 | **GAP.** `seated-office-worker` is pending. `height-adjustable-desk` and `office-swivel-chair` have no occupant. |
| Sleeping person | none. `single-bed` pose `asleep` (0.95 × 2.12 × 0.9 m) has a 75 kg sleeper as a load but no human figure | none (bed clips only: `get-into-bed` 4 s, `raise-head-section` 6 s) | none | **GAP.** `sleeping-person` is pending. `red-fox` pose `curled-sleeping` is an animal, not a person. |
| Walking person | `walking-commuter`, pose `standing` (1.78 m) | `walk-cycle` 1.2 s (loop) | MET 1.2–2.8 | OK. Alternatives: `courier-with-hand-truck` `push-walk` 1.2 s (loop); `person-with-white-cane` `constant-contact-walk` 1.1 s (loop). |
| Running person | `jogger`, pose `initial-contact-l` (1.775 m) | `run-cycle` 0.72 s (loop) | MET 2–8 | OK. Alternatives: `runner-with-blade-prosthesis` `run-cycle` 0.7 s (loop); `running-child` `sprint-cycle` 0.52 s (loop), 1.135 m. |
| Group or audience | none. `tour-guide` poses `gesturing-to-group` and `turning-to-group` give one figure facing a group | `gesture-point` 8 s | MET 1.3–1.7 | **GAP.** No crowd or audience asset and no multi-figure composition. Repeat `standing-adult` or `seated-reader` instances as an interim. |
| Radiator | `panel-radiator`, pose `rated` (0.85 × 0.285 × 1.0 m) | `heat-up` 60 s; `thermostat-cycle` 40 s (loop) | heat source op `heating` (off 0, night 0.515, comfort 0.748, rated 1); emitters `convection-outlet` (vent) and `front-plume` | OK. Elevation only. |
| Window | none as a whole window. `tilt-window` (1.25 × 0.3 × 0.9 m sash, pose `tilted-open`); `sliding-door` (2.15 × 3.1 × 0.1 m, pose `open`); `curtain-wall-bay` (3.3 × 1.4 × 0.15 m) | `tilt-open-close` 6 s; `slide-open-close` 8 s; `shadow-rise` 12 s (loop) | none | **Interim.** Sashes and glazing components only. No window frame with a wall reveal. `tilt-window` representation 500 is unavailable. |
| Shading | `external-venetian-blind` (2.75 × 0.42 × 1.2 m), pose `lowered-horizontal`. Also `market-parasol` (2.46 × 2.6 × 2.6 m), pose `open-still`, and `tied-canopy` (3.14 × 6 × 1 m) | `raise-lower` 22 s (loop); `tilt-sweep` 22 s (loop); `sun-and-shade` 10 s (loop). Parasol: `wind-flap` 6 s; `sun-tilt` 8 s | none | OK for shading geometry and sun-shade clips. The shading assets carry no thermal binding, so they cannot show a solar heat effect in the thermal view. Elevation only. |
| Tree | `english-oak` (18.2 m), pose `calm`; for a garden or courtyard tree, `apple-tree` (6.35 m), pose `summer-fruit-set` | `wind-sway` 4 s (loop); `gust` 8 s; `diurnal-transpiration` 24 s (loop) (english-oak). apple-tree: `breeze-sway` 2.4 s (loop); `diurnal-transpiration` 24 s (loop) | transpiration emitters (sun and shade crowns) | OK. Elevation only. Plan view is not available for trees. |
| Car | `city-car` (1.5 × 3.9 × 1.68 m), pose `parked`; for a van, `delivery-van` (1.99 × 5.34 × 1.99 m), pose `parked` | `drive-by` 4.8 s; `park-and-open-hatch` 9 s (city-car). `drive-loop` 1.0076 s (loop) and `stop-and-unload` 12 s (delivery-van) | heat-source emitters `tailpipe` (exhaust); state op `engine-load` | OK. Elevation only. |
| Laptop | none | none | none | **GAP.** `laptop` is pending. No laptop geometry or state exists. |
| Monitor | partial: `height-adjustable-desk` has a `monitor` state (off, standby about 1 W, on 25 W) but no monitor geometry | none on the desk | state op `monitor` (0 / 0.04 / 1) | **GAP.** `desktop-monitor` is pending. |
| Lamp | `street-lamp` (6 m), pose `lit-night` | `ignition-at-dusk` 900 s; `pole-sway-in-gust` 8 s | heat source op `lamp` (rated 60 W, 40 W heat); emitter `lamp-plume` | OK for an exterior street lamp. Interior lamp is a **GAP**: `pendant-lamp` is pending. |
| Heater | exterior: `patio-heater` (2.25 m), pose `high`. Building services: `heat-pump-outdoor-unit` (1.45 m), pose `heating`. Interior: `panel-radiator` (see Radiator) | `ignite-and-warm-up` 900 s; `turn-down-and-shut-off` 1800 s (patio-heater). `heating-sweep` 20 s (loop) (heat pump) | heat source op `burner` (low 6 kW, high 13 kW heat input); heat source op `refrigerant-duty`; flue and plume emitters | OK for exterior and building-services heating. Interior electric heater is a **GAP**: `electric-fan-heater` is pending. |
| Cook | `charcoal-grill` (0.97 m), pose `grilling-closed` (object only, no person) | `lid-lift-cycle` 20 s (loop); `light-and-close` 1800 s | heat source op `charcoal` and `lid-heat`; flue and vent plume emitters | **Interim.** Heat source, no person. Also `toaster` (0.2 × 0.3 × 0.17 m), pose `toasting`, clip `toast-cycle` 300 s, heat source op `elements` (900 W) and a vent outlet; no person and not yet in the manifest. A cooking person is a **GAP**: `person-cooking` is pending. No oven or hob exists. |

**Notes on using the mapping.** Several clips are long for a course clip: `light-and-close` 1800 s, `ignite-and-warm-up` 900 s, `ignition-at-dusk` 900 s, `shade-to-sun` 7200 s and `sun-and-shade` 14400 s on `bicycle-stand`. Use them only as time-lapse or with a shortened clip. For a cast, the form is `{ "asset": "walking-commuter", "clip": "walk-cycle", "view": "dark-line", "viewpoint": "elevation", "scale": 50, "fps": 15 }`. Thermal casts need an environment preset, and `🌤️environments` has none yet.

## Gaps summary

1. Pending assets (announced, not present at read time): seated-office-worker, sleeping-person, person-cooking, laptop, desktop-monitor, pendant-lamp, vacuum-cleaner, electric-fan-heater.
2. No audience or group composition asset.
3. No whole window with frame and reveal (only sashes and glazing components).
4. No environment presets, so thermal transient timelines cannot be cast.
5. Appliances category holds one asset (`toaster`) and that asset is not yet in the manifest. No kitchen or cleaning appliance beyond it.
6. Shading assets have no thermal binding.
7. Plan view exists for only 5 assets, none of them a tree, person or vehicle.

## Asset table

Generated from the 109 asset JSON files, grouped by category, sorted by slug. Pose lists show runs of numbered poses as `first … last (count)`.

### humans (25)

| slug | en title | H × W × D (m) | pose slugs | clip slugs (duration) | thermal binding | plan |
|---|---|---|---|---|---|---|
| `child-on-kick-scooter` | Child on Kick Scooter | 1.274 × 0.925 × 0.45 | gliding, pushing-off, braking, kick-00 … kick-43 (44) | kick-and-glide 2.2s (loop), glide-and-brake 4s | MET 2.2–5 (base 1); breath + plume emitters | no |
| `courier-with-hand-truck` | Courier with Hand Truck | 1.82 × 1.16 × 0.6 | waiting, tilted-back, parked-upright, walking-contact | push-walk 1.2s (loop), tilt-and-settle 5s | MET 1.5–3.2 (base 1.5); breath + plume emitters | no |
| `cyclist` | Cyclist | 1.79 × 1.74 × 0.65 | riding-upright, standing-on-pedals, coasting, dismounted-astride | pedal-cycle 4s, brake-and-stop 1.6s, dismount 2.4s | MET 1.8–7 (base 1.2); breath + plume emitters | no |
| `elderly-with-cane` | Elderly Person with Cane | 1.65 × 0.54 × 0.48 | stooped-standing, cane-planted, resting-on-cane, contact-left, loading-left, passing-left, contact-right, loading-right, passing-right | cane-gait 1.7s (loop), rest-on-cane 6s | MET 1.1–2 (base 1.2); breath + plume emitters | no |
| `jogger` | Jogger | 1.775 × 1.035 × 0.45 | initial-contact-l, loading-l, midstance-l, propulsion-l, toe-off-l, flight-l, initial-contact-r, loading-r, midstance-r, propulsion-r, toe-off-r, flight-r, stretch-stand, stretch-quad-r, stretch-hamstring | run-cycle 0.72s (loop), stretch 12s | MET 2–8 (base 1.5); breath + plume emitters | no |
| `kneeling-gardener` | Kneeling gardener | 1.145 × 1.835 × 0.5 | kneeling-weeding, kneeling-upright, reaching-trug, dig-up, dig-in, dig-pull | dig-cycle 1.6s (loop), reach-to-trug 4s | MET 1.8–4.2 (base 1.8); breath + plume emitters | no |
| `leaning-person` | Leaning Person | 1.73 × 0.53 × 0.45 | leaning-shoulder-wall, leaning-arms-folded, phone-call, pushing-off, standing-away | phone-call-idle 6s (loop), push-off 3.5s | MET 1–1.8 (base 1.1); breath + plume emitters; state op "phone" | no |
| `parent-carrying-toddler` | Parent carrying a toddler | 1.73 × 0.607 × 0.45 | standing-carry, heel-strike-l, walk-1 … walk-7 (6), midstance-l, hoisting | walk-carrying 1s (loop), shift-toddler 3s | MET 1.4–3 (base 1.4); breath + plume emitters; passenger physiology | no |
| `parent-with-stroller` | Parent with Stroller | 1.75 × 1.8 × 0.6 | pushing-stroller, pausing-with-stroller, checking-child, kerb-tilt, push-heel-strike-l, push-toe-off-r, push-midstance-l, push-heel-off-l, push-late-swing-r, push-heel-strike-r, push-toe-off-l, push-midstance-r, push-heel-off-r, push-late-swing-l | push-walk 0.9s (loop), stop-and-check-child 8s, kerb-tilt-up 3s | MET 1.5–3 (base 1.5); breath + plume emitters; passenger physiology | no |
| `person-on-crutches` | Person on Forearm Crutches | 1.83 × 0.556 × 0.65 | tripod-stand, crutches-forward, body-swing | three-point-gait 1.8s (loop), rest-on-crutches 6s (loop) | MET 1.5–3.4 (base 1); breath + plume emitters | no |
| `person-with-rollator` | Person with Rollator | 1.61 × 1.16 × 0.62 | stand-at-grips, walking-midstance, seated-rest, stoop-to-basket | rollator-walk 1.3s (loop), rest-at-the-grips 6s (loop) | MET 1–1.9 (base 1); breath + plume emitters | no |
| `person-with-umbrella` | Person with Umbrella | 2.09 × 1.05 × 1 | umbrella-up-still, umbrella-tilted-to-wind, umbrella-furled, gust-light, gust-heavy | rain-gust-sway 8s, close-umbrella 4s | MET 1.2–2.2 (base 1.3); breath + plume emitters | no |
| `person-with-white-cane` | Person with White Cane | 1.69 × 1.074 × 0.4 | constant-contact-stance, diagonal-technique, walking-midstance, cane-parked | constant-contact-walk 1.1s (loop), listen-and-scan 6s (loop) | MET 1.1–2.6 (base 1); breath + plume emitters | no |
| `pregnant-person` | Pregnant Person | 1.65 × 0.72 × 0.4 | hand-on-back, cradling-belly, stretching-back, waddle-contact | waddle-walk 1.3s (loop), rest-and-breathe 6s (loop) | MET 1.2–2.2 (base 1.3); breath + plume emitters | no |
| `runner-with-blade-prosthesis` | Runner with Blade Prosthesis | 1.79 × 0.41 × 0.3 | standing-ready, hands-on-hips, left-forefoot-strike, blade-midstance, flight, run-00 … run-20 (21) | run-cycle 0.7s (loop), recover-breathing 5s (loop) | MET 1.5–11.5 (base 1.6); breath + plume emitters | no |
| `running-child` | Running Child | 1.135 × 0.281 × 0.28 | standing-ready, mid-stride, airborne, toe-off, landing, crouch, jump-takeoff, jump-apex, flight-1, flight-2, flight-3 | sprint-cycle 0.52s (loop), standing-jump 2.0979s | MET 1.4–7 (base 1.4); breath + plume emitters | no |
| `seated-guitarist` | Seated guitarist | 1.495 × 1.107 × 0.55 | strumming, fretting-high, resting, tuning, strum-up | strum-loop 1s (loop), chord-change 4s, tune-up 3.5s | MET 1.2–1.8 (base 1.4); breath + plume emitters | no |
| `seated-reader` | Seated Reader | 1.34 × 1.02 × 0.5 | seated-upright, reclined-reading, reaching-for-cup, leaning-forward, rising, standing | page-turn 3.6s, stand-up 4s, lean-back 5s | MET 1–2.2 (base 1); breath + plume emitters | no |
| `skateboarder` | Skateboarder | 1.774 × 1.218 × 0.3 | cruising, pushing, crouch, ollie-air, push-plant, push-lift, push-swing, push-reach, pop, landing | push-cycle 2s (loop), ollie 1.7s, pump-cruise 1.6s (loop) | MET 2.5–6 (base 2.5); breath + plume emitters | no |
| `standing-adult` | Standing Adult | 1.67 × 0.328 × 0.4 | relaxed, arms-crossed, pointing, hands-warming | idle-breathing 5s (loop), shiver 1.2s (loop), fanning 2s (loop) | MET 1.2–1.3 (base 1.2); breath + plume emitters | no |
| `standing-phone-user` | Standing Phone User | 1.67 × 0.64 × 0.4 | texting, phone-call, photo-pose | scroll-and-sway 6s (loop), answer-call 4s | MET 1.2–1.4 (base 1.2); breath + plume emitters; state op "phone" | no |
| `tour-guide` | Tour Guide | 2.275 × 1.254 × 0.5 | standing-pointing, gesturing-to-group, holding-flag, waving, wave-swing-a, wave-swing-b, turning-to-group | gesture-point 8s, turn-and-wave 6s | MET 1.3–1.7 (base 1.3); breath + plume emitters | no |
| `walking-commuter` | Walking Commuter | 1.78 × 0.4 × 0.5 | standing, check-watch, contact-left, loading-left, passing-left, contact-right, loading-right, passing-right | walk-cycle 1.2s (loop), glance-at-watch 4s, idle-breathing 4s (loop) | MET 1.2–2.8 (base 1.2); breath + plume emitters | no |
| `wheelchair-user` | Wheelchair User | 1.45 × 1.09 × 0.7 | seated-level, hands-on-rims, push-end, ramp-ascent | push-cycle 4s, ramp-roll 2s, rest-in-chair 5s (loop) | MET 1.2–3.2 (base 1.2); breath + plume emitters | no |
| `worker-carrying-box` | Worker Carrying Box | 1.81 × 0.68 × 0.5 | squat-grip, lift-mid, carrying-box-low, carrying-box-chest, setting-down, walk-heel-strike-l, walk-toe-off-r, walk-midstance-l, walk-heel-off-l, walk-late-swing-r, walk-heel-strike-r, walk-toe-off-l, walk-midstance-r, walk-heel-off-r, walk-late-swing-l | squat-lift 3.2s, carry-walk 1.1s (loop), set-down 3s | MET 3.4–7 (base 1.5); breath + plume emitters | no |

### animals (14)

| slug | en title | H × W × D (m) | pose slugs | clip slugs (duration) | thermal binding | plan |
|---|---|---|---|---|---|---|
| `dairy-cow` | Dairy Cow | 1.49 × 2.55 × 0.72 | standing-chewing, grazing-head-down, lying-resting, kneeling-front, lowering-hindquarters, walk-phase-0 … walk-phase-7 (8) | walk-cycle 1.4s (loop), cud-chewing 5s (loop), lie-down 11s, graze-and-chew 12s | MET 1.8–4 (base 2.1); breath + plume emitters | no |
| `domestic-cat` | Domestic Cat | 0.36 × 0.77 × 0.13 | standing-alert, sitting-tail-wrapped, stretch-stance, play-bow, stretch-deep, walk-phase-0 … walk-phase-7 (8), stalk-phase-0 … stalk-phase-7 (8) | walk-cycle 0.8s (loop), stalk-cycle 2.4s (loop), stretch 8s | MET 1.1–2.5 (base 1.3); breath + plume emitters | no |
| `goat` | Goat (Horned Doe) | 1.09 × 1.31 × 0.34 | standing-alert, browsing-upright, climbing-ledge, walk-phase-0 … walk-phase-11 (12) | walk-cycle 1.1s (loop), browse-rear 7s | MET 1.3–3.2 (base 1.3); breath + plume emitters | no |
| `hedgehog` | European Hedgehog | 0.128 × 0.28 × 0.15 | foraging, alert, rolled, walk-0 … walk-7 (8), rolling-half | forage-walk 0.8s (loop), roll-up 2.4s | MET 1.1–3 (base 1.5); breath + plume emitters | no |
| `herring-gull` | Herring Gull | 0.51 × 0.699 × 0.2 | standing, gliding, landing, gliding-up, gliding-down, touchdown, settled, approach-a, flare-key | soar 4s (loop), landing 3.2s | MET 1.5–5 (base 1.5); breath + plume emitters | no |
| `horse` | Horse | 2.37 × 2.91 × 0.62 | standing-alert, standing-rest, grazing-head-down, walk-phase-0 … walk-phase-7 (8) | walk-cycle 0.95s (loop), graze-and-swish 6.4s (loop) | MET 1.35–3.9 (base 1.5); breath + plume emitters | no |
| `labrador-dog` | Labrador Dog | 0.72 × 1.44 × 0.28 | standing-alert, sitting-waiting, sniffing-head-down, walk-phase-0 … walk-phase-7 (8), trot-phase-0 … trot-phase-7 (8) | walk-cycle 0.7s (loop), trot-cycle 0.45s (loop), sniff-ground 4s, sit-and-pant 6s | MET 1.2–6.7 (base 1.3); breath + plume emitters | no |
| `mallard-duck` | Mallard Drake | 0.46 × 0.6 × 0.2 | standing, swimming, dabbling, upending, wing-stretch, waddle-0 … waddle-7 (8), paddle-0 … paddle-5 (6) | waddle 0.8s (loop), paddle 0.7s (loop), dabble 4s (loop) | MET 1.5–3 (base 1.5); breath + plume emitters | no |
| `pig` | Domestic Pig (Finishing Pig) | 0.763 × 1.206 × 0.42 | standing-pig, rooting, lying-wallowing, walk-phase-0 … walk-phase-11 (12) | walk-cycle 1s (loop), root-cycle 4s (loop) | MET 1–2.8 (base 1.3); breath + plume emitters | no |
| `pigeon` | Rock Pigeon | 0.25 × 0.33 × 0.15 | standing-alert, wings-half-open, pecking-reach, pecking-strike, walk-phase-0 … walk-phase-11 (12), takeoff-crouch, takeoff-wings-up, takeoff-push, takeoff-liftoff, airborne-wings-up, airborne-wings-down, airborne-climb | walk-cycle 0.5s (loop), peck-cycle 2s (loop), take-off 0.72s | MET 1.3–6.5 (base 1.3); breath + plume emitters | no |
| `red-fox` | Red Fox | 0.626 × 1.173 × 0.16 | standing-alert, mousing-crouch, curled-sleeping, pounce-flight, pounce-land, trot-phase-0 … trot-phase-9 (10) | trot-cycle 0.5s (loop), mousing-pounce 3s | MET 1–8 (base 1.4); breath + plume emitters | no |
| `red-squirrel` | Red Squirrel | 0.275 × 0.21 × 0.12 | sitting-eating, running, climbing, nibble-b, bound-extension, bound-swing, bound-gather, bound-push, bound-flight, climb-a, climb-b, climb-c | nibble 2s (loop), bound 0.36s (loop), climb 2.4s | MET 1.5–7 (base 1.5); breath + plume emitters | no |
| `roe-deer` | Roe Deer | 1.08 × 1.46 × 0.2 | alert, grazing, alert-foreleg-raised, bounding-gathered, bounding-pushoff, walk-phase-0 … walk-phase-7 (8), bound-phase-0 … bound-phase-7 (8) | walk-cycle 1s (loop), bound-cycle 0.9s (loop), alarm-freeze 7s | MET 1.5–6.5 (base 1.5); breath + plume emitters | no |
| `sheep` | Sheep (Ewe in Full Fleece) | 0.823 × 1.27 × 0.44 | standing-resting, grazing, lying-ruminating, walk-phase-0 … walk-phase-11 (12) | walk-cycle 1.15s (loop), graze-cycle 4s (loop), ruminate-lying 4s (loop) | MET 1–3 (base 1.2); breath + plume emitters | no |

### vegetation (16)

| slug | en title | H × W × D (m) | pose slugs | clip slugs (duration) | thermal binding | plan |
|---|---|---|---|---|---|---|
| `agave` | Agave | 1.43 × 2.52 × 2.4 | summer-turgid, night-stomata-open, drought-infolded, frost-sag, fresh-breeze | wind-sway 0.95s (loop), gust 5s | transpiration emitters (no MET) | yes |
| `apple-tree` | Apple Tree, Orchard Standard | 6.35 × 6.73 × 6.2 | summer-fruit-set, breeze-with-fruit, harvest-laden, gale-windfall | breeze-sway 2.4s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `bamboo-clump` | Bamboo Clump | 4.57 × 3.03 × 1.7 | summer-calm, fresh-breeze, storm-bent, wet-snow, winter-overcast | wind-sway 2.9s (loop), gust 6.5s | transpiration emitters (no MET) | yes |
| `date-palm` | Date Palm | 12.5 × 6.87 × 7 | calm, windy, storm | wind-sway 4.4s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `english-oak` | English Oak | 18.2 × 18.35 × 15 | calm, windy, storm, wet-snow | wind-sway 4s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `hornbeam-hedge` | Hornbeam Hedge | 1.88 × 3.14 × 0.7 | calm, windy, storm, snow | wind-sway 1.6s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `hosta-clump` | Hosta Clump | 0.62 × 0.88 × 0.85 | summer-full, spring-emerging, rain-bowed, autumn-collapse | wind-flutter 1.2s (loop), gust 6s | transpiration emitters (no MET) | yes |
| `ivy-on-trellis` | Ivy on Trellis | 2.11 × 1.91 × 0.25 | summer-calm, summer-night, breezy, storm, winter-snow | wind-sway 1.6s (loop), gust 7s | transpiration emitters (no MET) | yes |
| `japanese-maple` | Japanese Maple, Multi-Stem | 4.42 × 5.27 × 5.2 | summer-calm, summer-breeze, autumn-colour, autumn-snow, storm | breeze-sway 1.8s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `lombardy-poplar` | Lombardy Poplar | 26 × 4.16 × 4.4 | calm, windy, storm | wind-sway 3.2s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `london-plane-pollarded` | London Plane, Pollarded | 8.9 × 6.81 × 6.5 | summer-calm, summer-breeze, midday-heat, autumn-leaf-fall, gale | breeze-sway 3s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `miscanthus-grass` | Miscanthus Grass | 1.75 × 2.14 × 1.8 | calm, windy, storm, snow | wind-sway 2.2s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `multi-stem-birch` | Multi-stem Birch | 13 × 9.26 × 7 | calm, windy, storm, ice | wind-sway 2.8s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `scots-pine` | Scots Pine | 22.2 × 12.15 × 8 | calm, windy, storm, snow | wind-sway 3.6s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `small-leaved-lime` | Small-Leaved Lime | 14.19 × 9.06 × 9 | summer-calm, summer-breeze, drought-stress, autumn-leaf-fall, wet-snow, gale | breeze-sway 4s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |
| `weeping-willow` | Weeping Willow | 11.1 × 16.23 × 14 | calm, windy, storm | wind-sway 5.2s (loop), gust 8s, diurnal-transpiration 24s (loop) | transpiration emitters (no MET) | no |

### furniture (11)

| slug | en title | H × W × D (m) | pose slugs | clip slugs (duration) | thermal binding | plan |
|---|---|---|---|---|---|---|
| `bar-stool` | Bar stool with footrest ring | 0.87 × 0.42 × 0.42 | unoccupied, occupied, turned-90, seat-proof-load, turned-270-key, occupied-turned-key | swivel-turn 6s (loop), sit-and-turn 5s, stand-up 3s | none | no |
| `bookshelf` | Bookshelf with adjustable shelves | 1.8 × 0.8 × 0.3 | empty, loaded, shelf-adjusted, top-two-loaded-key | load-the-shelves 5s, re-pin-shelf 4s, unload-shelves 3s | none | no |
| `cafe-chair` | Café Chair | 0.88 × 0.44 × 0.5 | upright-empty, pulled-out, tilted-on-two-legs, seated-pulled-out, seated-occupied | pull-out-and-sit 2.8s, tip-back 3s | none | no |
| `dining-table` | Dining table with extension leaf | 0.75 × 1.6 × 0.9 | closed, extended, set-for-dinner, edge-test-load, pulled-apart-key | extend-table 4s, retract-table 4s, lay-the-table 6s | none | no |
| `folding-deck-chair` | Folding deck chair with canvas sling | 0.93 × 0.78 × 0.58 | unfolded, folded, reclined-seated, sling-proof-load, half-folded-key | fold-chair 4s, unfold-chair 4s, sit-and-recline 4s | none | no |
| `height-adjustable-desk` | Height-adjustable Desk | 1.18 × 1.6 × 0.8 | seated-height, mid-height, standing-height | raise-and-lower 30s, load-on-surface 4s | state op "lift-drive"; state op "monitor" | no |
| `office-swivel-chair` | Office Swivel Chair | 1.08 × 0.6 × 0.6 | seat-low, seat-high, reclined | swivel-rotation 4s, height-adjust 6s | none | no |
| `park-bench` | Park Bench | 0.85 × 1.8 × 0.6 | unloaded, occupied-one, occupied-two | occupant-sit-down 2s, sun-exposure-cycle 60s (loop) | none | no |
| `picnic-table` | Picnic table with fold-down benches | 0.75 × 1.52 × 1.8 | deployed, stowed, occupied, bench-proof-load, half-folded-key | fold-benches 3s, deploy-benches 3s, sit-down-together 5s | none | no |
| `single-bed` | Single bed with slatted base | 0.95 × 2.12 × 0.9 | flat, head-raised, asleep, reading-in-bed, half-raised-key | raise-head-section 6s, get-into-bed 4s, sit-up-to-read 6s | none | no |
| `three-seat-sofa` | Three-seat sofa | 0.85 × 2 × 0.9 | unoccupied, one-seated, two-seated, lounging, seat-proof-load, impact-key, recovering-key | sit-down-and-settle 2.5s, stand-up-and-recover 4s, lie-down-on-sofa 5s | none | no |

### appliances (1)

| slug | en title | H × W × D (m) | pose slugs | clip slugs (duration) | thermal binding | plan |
|---|---|---|---|---|---|---|
| `toaster` | Two-Slice Toaster | 0.2 × 0.3 × 0.17 | idle, toasting, popped | toast-cycle 300s, cool-down 600s | heat-source emitters: vent; heat source op "elements" | no |

### objects (11)

| slug | en title | H × W × D (m) | pose slugs | clip slugs (duration) | thermal binding | plan |
|---|---|---|---|---|---|---|
| `bicycle-stand` | Bicycle Stand (Sheffield Hoop) | 0.8 × 0.75 × 0.0483 | empty-shade, empty-sun, bike-leaning, lock-pulled | bike-parked 12s, sun-and-shade 14400s | none | no |
| `charcoal-grill` | Charcoal Kettle Grill | 0.97 × 0.67 × 0.64 | cold-closed, lit-lid-open, grilling-closed, embers-vents-low | light-and-close 1800s, lid-lift-cycle 20s (loop) | heat-source emitters: flue, vent; heat source op "charcoal"; heat source op "lid-heat" | no |
| `drinking-fountain` | Cast Drinking Fountain | 1 × 0.4 × 0.4 | idle-sun, drinking, idle-shade | press-and-drink 14s, shade-to-sun 7200s | heat-source emitters: vent; state op "tap" | no |
| `market-parasol` | Market Parasol | 2.46 × 2.6 × 2.6 | open-still, tilted-to-sun, furled | wind-flap 6s, sun-tilt 8s | none | no |
| `patio-heater` | Propane Patio Heater | 2.25 × 0.8 × 0.8 | off-afternoon, low, high, high-in-breeze | ignite-and-warm-up 900s, turn-down-and-shut-off 1800s | plume emitter; heat-source emitters: flue; heat source op "burner" | no |
| `playground-swing` | Playground Swing | 2.35 × 2.78 × 1.2 | rest, pendulum-forward, pendulum-back | pendulum-swing 2.7656s (loop), occupant-push 4s | none | no |
| `retractable-bollard` | Retractable Bollard (Section) | 1.9 × 0.56 × 0.4 | raised, lowered, moving-up, raised-nudged | raise-and-lower 30s, nudge-test 10s | heat source op "hydraulics"; heat source op "beacon" | no |
| `shopping-trolley` | Shopping Trolley | 1 × 1.05 × 0.57 | empty, loaded, gate-raised | load-groceries 16s, nesting-swing 12s (loop) | none | no |
| `stepladder` | Aluminium Stepladder | 1.5 × 0.68 × 0.46 | open-empty, open-in-use, rated-load-test | climb-and-work 12s, rated-load-ramp 20s | none | no |
| `street-lamp` | Street Lamp | 6 × 1.2 × 0.3 | dusk-off, lit-night, dimmed-night, lit-in-wind, day-off | ignition-at-dusk 900s, pole-sway-in-gust 8s | plume emitter; heat source op "lamp" | no |
| `waste-bin` | Waste Bin | 1.03 × 0.53 × 0.5 | closed-empty, lid-open, full-load | lid-open-close 3s, waste-fill 6s | none | no |

### vehicles (8)

| slug | en title | H × W × D (m) | pose slugs | clip slugs (duration) | thermal binding | plan |
|---|---|---|---|---|---|---|
| `cargo-bike` | Cargo Bike | 1.16 × 2.69 × 0.65 | parked-on-stand, loaded-on-stand, rolling-loaded, ride-start, ride-end | roll-by 6s, load-box 10s | state op "assist"; state op "rolling"; state op "braking" | no |
| `city-bicycle` | Dutch City Bicycle | 1.09 × 1.87 × 0.62 | parked-on-stand, riding, riding-night, braking-stop, loop-start, loop-mid, loop-end, rolled-to-stop, parked-stopped, rolling-in | ride-loop 1.0397s (loop), brake-and-park 5s | state op "rolling"; state op "pedalling"; state op "braking"; state op "lighting" | no |
| `city-bus` | City Bus | 3.14 × 12 × 2.55 | level-cruise, at-stop-level, kneeling-at-stop, braking, arriving, pull-away-8, pull-away-18 | arrive-kneel-doors 14s, depart 11s, layover-shutdown 6s | heat-source emitters: vent; state op "traction"; state op "rolling"; state op "braking"; state op "hvac" | no |
| `city-car` | City Car | 1.5 × 3.9 × 1.68 | parked, driving, hatch-open, bonnet-open, braking-nose-down, lane-start, lane-cruise-entry, lane-exit, crawl-in | drive-by 4.8s, park-and-open-hatch 9s | heat-source emitters: exhaust; state op "engine-load"; state op "rolling"; state op "braking" | no |
| `delivery-van` | Delivery Van | 1.99 × 5.34 × 1.99 | parked, side-door-open, rear-doors-open, driving-loaded, braking-nose-down, loop-start, loop-mid, loop-end, stop-in, stop-out | drive-loop 1.0076s (loop), stop-and-unload 12s | heat-source emitters: exhaust; state op "engine-load"; state op "rolling"; state op "braking" | no |
| `electric-kick-scooter` | Electric Kick Scooter | 1.11 × 1.09 × 0.43 | riding-cruise, braking-stop, standby-unfolded, folded-charging, loop-start, loop-mid, loop-end | ride-loop 0.7323s (loop), fold-and-charge 5s | state op "drive"; state op "rolling"; state op "braking"; state op "charging" | no |
| `motor-scooter` | Motor Scooter 125 | 1.15 × 1.85 × 0.74 | parked-on-stand, cruising, braking-nose-down, idling-seat-open, loop-start, loop-mid, loop-end, rolling-in, rolled-to-stop, parked-stopped | ride-loop 0.6939s (loop), stop-and-shut-down 7s | heat-source emitters: exhaust; state op "engine-load"; state op "rolling"; state op "braking" | no |
| `tram` | Low-Floor Tram Section | 5 × 15.45 × 2.4 | running, at-stop-doors-open, braking-stop, depot-pantograph-down, loop-start, loop-mid, loop-end, stop-in, stop-out | run-loop 1.3577s (loop), stop-and-open-doors 14s, pantograph-cycle 12s | heat-source emitters: fan, vent; state op "traction"; state op "rolling"; state op "braking"; state op "hvac" | no |

### machinery (8)

| slug | en title | H × W × D (m) | pose slugs | clip slugs (duration) | thermal binding | plan |
|---|---|---|---|---|---|---|
| `concrete-mixer-truck` | Concrete mixer truck, 9 m3 | 3.95 × 9 × 2.55 | transit-agitating, parked-off, charging, discharging, chute-swing | agitate-at-rest 10s (loop), discharge-sequence 16s | heat-source emitters: exhaust, vent; state op "engine-load"; state op "drum-drive"; state op "driving"; state op "braking" | no |
| `diesel-generator` | Silenced container diesel generator, 400 kVA | 3.57 × 6.06 × 2.44 | off, idle, half-load, rated-load, loading-up | load-cycle 12s (loop), start-run-stop 26s | heat-source emitters: exhaust, fan; state op "engine-load"; state op "cooling-fan" | no |
| `excavator` | Excavator | 3.43 × 9.85 × 2.99 | stowed, digging, raised-boom | dig-cycle 9s (loop), dump-and-stow 9s, start-and-idle 8s | heat-source emitters: exhaust; state op "engine-load"; state op "hydraulic" | no |
| `forklift` | Forklift | 2.26 × 4.07 × 1.2 | lowered-forks, lifted-load, tilted-mast, approach, travel-accelerated, travel-cruised, travel-stopped | lift-pallet 10s, drive-with-load 7.5s | heat-source emitters: vent; state op "traction"; state op "lifting"; state op "rolling"; state op "braking" | no |
| `heat-pump-outdoor-unit` | Air-source heat pump outdoor unit, 12 kW | 1.45 × 0.72 × 1.1 | heating, cooling, defrosting, off, serviced | heating-sweep 20s (loop), defrost-sequence 30s | heat-source emitters: fan, vent; state op "compressor"; state op "fan-speed"; heat source op "refrigerant-duty"; state op "defrost" | no |
| `scissor-lift` | Electric scissor lift, 5.8 m platform height | 2.2 × 1.85 × 0.81 | stowed, mid-raise, raised-working, lowering | raise-cycle 24s (loop), start-lift-stop 22s | plume emitter; heat-source emitters: vent; state op "power-draw"; state op "lift-pump" | no |
| `tower-crane` | Tower Crane | 48.85 × 71.8 × 2 | parked-weathervane, lifting-load, max-reach, skip-at-ground | lift-and-traverse 22s, unload 16s | heat-source emitters: vent; state op "hoisting"; state op "slewing" | no |
| `wheel-loader` | Articulated wheel loader, 20 t | 3.4 × 8.9 × 2.95 | carry, dig, dump-high, parked-off, raising | load-cycle 12s (loop), start-and-lift 10s | heat-source emitters: exhaust, vent; state op "engine-load"; state op "hydraulic"; state op "driving" | no |

### components (15)

| slug | en title | H × W × D (m) | pose slugs | clip slugs (duration) | thermal binding | plan |
|---|---|---|---|---|---|---|
| `cantilever-balcony` | Cantilever Balcony | 1.3 × 1.8 × 1 | self-weight, distributed-live-load, point-load-tip | loading-ramp 6s, moving-point-load 6s, sun-and-shade 8s (loop) | none | no |
| `curtain-wall-bay` | Curtain Wall Bay | 3.3 × 1.4 × 0.15 | dead-load, cradle-at-mid, seismic-in-plane, shaded-by-overhang | cradle-pass 8s, shadow-rise 12s (loop), seismic-pulse 6s | none | no |
| `external-venetian-blind` | External Venetian Blind | 2.75 × 0.42 × 1.2 | lowered-horizontal, retracted, lowered-cut-off-45, lowered-closed, closed-in-gust, lift-1 … lift-4 (4), tilt-1 … tilt-5 (5) | raise-lower 22s (loop), tilt-sweep 22s (loop), sun-and-shade 10s (loop) | none | no |
| `glass-balustrade` | Glass Balustrade | 1.1 × 0.3 × 2 | self-weight-unloaded, handrail-line-load, crowd-impact-point, wind-pressure | handrail-push 3s, impact-pulse 2s | none | no |
| `glulam-beam` | Glulam Beam | 0.62 × 8 × 0.16 | self-weight, distributed-live-load, point-load-midspan | moving-point-load 6s, loading-ramp 5s | none | no |
| `masonry-wall-with-lintel` | Masonry Wall with Precast Lintel | 2.75 × 3 × 0.445 | dead-load, floor-line-load, beam-over-opening, shaded-morning | load-build-up 6s, beam-sweep 8s, sun-and-shade 12s (loop) | none | no |
| `panel-radiator` | Panel Radiator Type 22 | 0.85 × 0.285 × 1 | off, night, comfort, rated, leaned-on | heat-up 60s, thermostat-cycle 40s (loop) | plume emitter; heat-source emitters: vent; heat source op "heating" | no |
| `sectional-garage-door` | Sectional Garage Door | 2.861 × 2.62 × 2.5 | closed, half-open, open, closed-wind, travel-1 … travel-7 (7) | open-close 32s (loop), wind-gust 8s, sun-and-shade 12s (loop) | none | no |
| `sliding-door` | Sliding Door | 2.15 × 3.1 × 0.1 | closed, half-open, open | slide-open-close 8s, slide-open 4s, sun-and-shade 8s (loop) | none | no |
| `spiral-stair` | Steel Spiral Stair | 3.733 × 1.6 × 1.6 | self-weight, walker-on-tread-8, point-load-top-tread, rail-push | climb 14s, live-load-ramp 6s, sun-and-shade 8s (loop) | none | yes |
| `stair-flight` | Stair Flight | 2.9 × 3.885 × 1.2 | self-weight, crowd-live-load, point-load-tread | loading-ramp 5s, moving-point-load 6s | none | no |
| `steel-column` | Steel Column | 3 × 0.3 × 0.2 | axial-self-weight, eccentric-load, lateral-wind | loading-ramp 5s, eccentric-load-sweep 6s | none | no |
| `tied-canopy` | Tied Canopy | 3.14 × 6 × 1 | self-weight, snow-load, wind-uplift | loading-ramp 4s, wind-gust 6s | none | no |
| `tilt-window` | Tilt Window | 1.25 × 0.3 × 0.9 | closed, tilted-open, fully-tilted | tilt-open-close 6s, wind-pressure-pulse 4s, sun-and-shade 8s (loop) | none | no |
| `wheelchair-ramp` | Wheelchair Ramp 1:12 with Landings | 1.471 × 10.5 × 1.4 | self-weight, crowd-live-load, wheelchair-on-run-2, handrail-push | wheelchair-ascent 10s, live-load-ramp 6s, sun-and-shade 8s (loop) | none | no |
