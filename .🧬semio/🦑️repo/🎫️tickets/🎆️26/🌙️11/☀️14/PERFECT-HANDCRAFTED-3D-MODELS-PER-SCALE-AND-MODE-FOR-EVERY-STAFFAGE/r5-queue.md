# Round 5 Queue (2026-11-20, Opus 5.5 medium, cap 20 concurrent)

Brief: `r5-brief.md`. Ledger: `r5-ledger.md`.

## Running (wave 1)
H01 standing-adult, walking-commuter · H02 jogger, running-child · H03 elderly-with-cane, person-on-crutches · H04 person-with-umbrella, person-with-white-cane · H05 parent-with-stroller, parent-carrying-toddler · H06 pregnant-person, person-with-rollator · H07 seated-office-worker, seated-reader · H08 seated-guitarist, sleeping-person · H09 wheelchair-user, runner-with-blade-prosthesis · H10 kneeling-gardener, cyclist · H11 child-on-kick-scooter, skateboarder · H12 courier-with-hand-truck, worker-carrying-box · H13 person-cooking, leaning-person · H14 standing-phone-user, tour-guide
A01 horse, dairy-cow · A02 goat, pig · A03 sheep, roe-deer · A04 labrador-dog, domestic-cat · A05 red-fox, hedgehog · A06 red-squirrel, herring-gull

## Queued (start one per freed slot, tick when started)
- [x] A07 animals/mallard-duck, animals/pigeon
- [x] V01 vehicles/city-car, vintage-car-1929, delivery-van
- [x] V02 vehicles/city-bus, tram, electric-kick-scooter
- [x] V03 vehicles/cargo-bike, city-bicycle, motor-scooter
- [x] M01 machinery/excavator, wheel-loader, forklift, concrete-mixer-truck
- [x] M02 machinery/tower-crane, scissor-lift, oil-pumpjack, diesel-generator
- [x] M03 machinery/heat-pump-outdoor-unit, evaporative-humidifier, finned-heat-exchanger, plate-heat-exchanger
- [x] M04 machinery/expansion-valve, scroll-compressor, sorption-wheel, solar-thermal-collector
- [x] P01 vegetation/apple-tree, english-oak, scots-pine, date-palm
- [x] P02 vegetation/weeping-willow, japanese-maple, lombardy-poplar, london-plane-pollarded
- [x] P03 vegetation/small-leaved-lime, multi-stem-birch, hornbeam-hedge, ivy-on-trellis
- [x] P04 vegetation/agave, bamboo-clump, hosta-clump, miscanthus-grass
- [x] F01 furniture/office-swivel-chair, cafe-chair, bar-stool, folding-deck-chair
- [x] F02 furniture/dining-table, park-bench, picnic-table, height-adjustable-desk
- [x] F03 furniture/single-bed, three-seat-sofa, bookshelf, operating-table
- [x] E01 appliances/gas-boiler, hot-water-cylinder, circulation-pump, mvhr-unit, decentralised-hrv-fan
- [x] E02 appliances/duct-fan, ceiling-diffuser, split-ac-indoor-unit, electric-fan-heater, gas-meter
- [x] E03 appliances/server-rack, tower-server, desktop-monitor, laptop, electric-kettle
- [x] E04 appliances/gas-hob, pendant-lamp, vacuum-cleaner, toaster, kitchen-extractor-hood
- [x] E05 appliances/electricity-meter, room-thermostat, shower-head, thermostatic-radiator-valve
- [x] C01 components/cantilever-balcony, curtain-wall-bay, tilt-window, sliding-door, external-venetian-blind
- [x] C02 components/etics-brick-wall-12, etics-brick-wall-30, masonry-wall-with-lintel, solid-brick-wall-1960, glulam-beam
- [x] C03 components/spiral-stair, stair-flight, tied-canopy, steel-column
- [x] C04 components/panel-radiator, glass-balustrade, sectional-garage-door, wheelchair-ramp
- [x] O01 objects/bicycle-stand, charcoal-grill, market-parasol, playground-swing, shopping-trolley
- [x] O02 objects/storage-crate-stack, waste-bin, chalkboard, cooking-pot, drinking-fountain
- [x] O03 objects/patio-heater, retractable-bollard, stepladder, street-lamp, tealight

## Afterwards
Independent visual review pass (fresh eyes) on every asset, catalog-wide `r3-validate --volumes` sweep, `repo test` for staffage, staffage-react, staffage-site, framework-3d; export-models; ticket close.

## Rule conflicts to resolve before close (coordinator)
- C31 vs B4 (solar-thermal-collector): a sealed glazed box makes the visible absorber and headers a closed cavity (outer area ~0), C31 says declare `hidden`, B4 reserves `hidden` for interior parts. Resolution needed in the validator: transparent closures (glazing) must not seal the flood for areas of regions behind them, or the region counts its full mesh area.
- Scale-range gaps: machinery/evaporative-humidifier 1:200 ends 1:280, finned- and plate-heat-exchanger 1:200 ends 1:220, heat-pump 1:500 ends 1:900 (check whether a coarser band follows); appliances/laptop 1:320–1:350; appliances/mvhr-unit 1:200 drawing narrowed to 1:210, nothing covers 1:210–1:350 (no 1:500 drawing). Second pass must draw a coarser representation or redesign so every denominator is covered. Sweep all assets for range gaps.

## Second pass queue (start one per freed slot)
- [x] R01 root: validator rule against valid-range gaps between an asset's drawing bands (product `validateRanges`), language-agnostic test vectors; then repair every flagged asset by redrawing (never by narrowing).
- [x] R02 root: C31 flood treats transparent closures (glazing) as walls, sealing visible regions (solar-thermal-collector absorber/headers); fix in `🧊️volumes/🌊️flood` + tests + design.md amendment; re-check all glazed assets (curtain-wall-bay, tilt-window, sliding-door, vehicles, chalkboard…).
- [x] R03 area reviewer: work through every open entry in `area-review.md` per rules C16–C31 (hand sums, segment shares C27), apply decisions, re-run `volumes-check --product` on each touched asset.
- [x] R04 root: full 3D bone rest orientation + roll and 3D pose rotations in rig schema/product/exports (spiral-stair radial treads, cat tail curl, fox curl on side, gull wing feather bones, forearm twist limits); amendment E4.
- [x] S01 second pass humans A: coarse scales 1:200/1:500 to A + unopened sheets: standing-adult, walking-commuter, jogger, running-child, elderly-with-cane, person-on-crutches, person-with-umbrella
- [ ] S02 second pass humans B: person-with-white-cane, parent-with-stroller, parent-carrying-toddler, pregnant-person, person-with-rollator, seated-office-worker, seated-reader
- [ ] S03 second pass humans C: seated-guitarist, sleeping-person, wheelchair-user, runner-with-blade-prosthesis, kneeling-gardener (1:200/1:500 2D missing), cyclist, child-on-kick-scooter
- [ ] S04 second pass humans D: skateboarder, courier-with-hand-truck, worker-carrying-box, person-cooking, leaning-person (shank -3.1 %), standing-phone-user, tour-guide (no neck bone, trunk bone mid-body)
- [ ] S05 second pass animals A: goat (body not remodelled), pig, sheep, roe-deer (1:500 C+), dairy-cow, horse
- [ ] S06 second pass animals B: labrador-dog, domestic-cat (tail curl), red-fox, hedgehog, red-squirrel (third tail bone), herring-gull (wing feather bones, gliding C+)
- [ ] then second pass for every non-living group whose ledger line is below A at any scale
- Waste-bin and chalkboard expose a C31 blind spot: real gaps narrower than a coarse scale's flood cell (lid notches, 4 mm leaf gap) seal cavities at 1:200/1:500. R02 owns this (glazed/sealed cavities); forward it.
