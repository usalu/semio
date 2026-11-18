# Asset power

Request: every asset states, next to size and weight, its power in watts: the complete sum of all elements (a person cooking includes the stove).

## Definition (final)

Total power = sum over all elements of their **nameplate power** plus the metabolic heat of living bodies in the default pose. Nameplates are authored once per element in `ratedPowerW` with `ratedPowerBasis` on any thermal region (tagged by that region's `source`/`reference`): `electrical-input` (appliances, devices, lamps, heat pumps: input, never heating or cooling capacity), `fuel-input` (burners, fires), `rated-output` (combustion engines, traction and drive motors, hoists, generating sets) or `passive` (0 W: radiators, coils, collectors, cylinders, valves, resistors). Released heat (`operating.heatW`) is never summed. The first version (summed released heat) was replaced because it understated machines (a 12 kW heat pump showed 1106 W, a 20 t wheel loader 13.4 kW).

## Design

- Schema first: `ThermalRegion.ratedPowerW?` (>= 0, only with `operating` or `hydronic`) and `ManifestEntry.powerW` (required) in `🧬️schema/🔣️.json` and the typed twin `🧬️schema/🟦️.ts`.
- Validator: `power-rated-invalid` (asset) and `manifest-power-mismatch` (manifest, 0.5 W tolerance).
- Manifest: `powerW` on all 152 entries (rounded to 0.1 W) so cards, sort and filter need no loaded asset; once an asset is loaded the app derives it from the asset.
- Authored data: see the audit below (the first version rated only `person-cooking` and `panel-radiator`).
- App: chip "Total power of all elements" on every card and compare pane (`formatPowerW`: one decimal below 10 W), fact "Power (total of all elements)" / "Leistung (Summe aller Elemente)" in "Dimensions and metadata", sort "Power" / "Leistung", range filter "Power" / "Leistung" in watts with address key `w=<min>-<max>`, en and de, native labelled inputs and chips with `title`.
- Known limitation: the gas hob is not rated yet (another agent owns it).

## Design delta of the audit

- Schema: `ThermalRegion.ratedPowerW` allowed on any region (engine blocks, fans, motors), plus `ratedPowerBasis` (`POWER_BASES`); validator `power-rated-invalid`; the fallback to `|operating.heatW|` is gone.
- 59 assets authored (all with operating heat or a heating circuit except the gas hob, which another agent rates): see the table. Ratings are catalogue-typical (`authored-estimate`), taken from the asset's own summary where it states one (kettle 2 kW, toaster 900 W, vacuum 700 W, patio heater 13 kW, desk monitor 25 W, laptop 65 W adapter), else a typical nameplate for the class (20 t wheel loader 150 kW, city car 80 kW, tram 480 kW, tower crane 55 kW hoist plus 15 kW slewing).
- Format: `formatPowerW`: watts below 1 kW, kilowatts of three significant digits above, en/de separators ("184 W", "1,68 kW", "150 kW"); overview intro and filter/sort texts name power in en and de.
- Open: `gas-hob` has no rating yet (another agent owns it; the test lists it as pending, the manifest shows 0 W until it is rated, then `powerW` must be updated).

## Final per-asset table (nonzero assets; total and largest elements)

| Category | Asset | Power | Elements |
|---|---|---|---|
| animals | dairy-cow | 918 W | body 918 W |
| animals | horse | 576 W | body 576 W |
| animals | pig | 139 W | body 139 W |
| animals | sheep | 98 W | body 98 W |
| animals | goat | 89 W | body 89 W |
| animals | roe-deer | 57 W | body 57 W |
| animals | labrador-dog | 57 W | body 57 W |
| animals | red-fox | 18 W | body 18 W |
| animals | domestic-cat | 13 W | body 13 W |
| animals | mallard-duck | 6.3 W | body 6.3 W |
| animals | herring-gull | 5.7 W | body 5.7 W |
| animals | hedgehog | 4.4 W | body 4.4 W |
| animals | pigeon | 2.6 W | body 2.6 W |
| animals | red-squirrel | 2.1 W | body 2.1 W |
| appliances | gas-boiler | 20.0 kW | burner 20.0 kW, controls 3 W |
| appliances | server-rack | 5.00 kW | servers 5.00 kW |
| appliances | electric-fan-heater | 2.00 kW | casing 2.00 kW |
| appliances | electric-kettle | 2.00 kW | water 2.00 kW |
| appliances | toaster | 900 W | casing 900 W |
| appliances | split-ac-indoor-unit | 700 W | casing 700 W |
| appliances | vacuum-cleaner | 700 W | body-shell 700 W |
| appliances | tower-server | 200 W | side-panel 200 W |
| appliances | kitchen-extractor-hood | 150 W | canopy 120 W, lamps 30 W |
| appliances | mvhr-unit | 75 W | impeller 75 W |
| appliances | laptop | 65 W | deck 65 W |
| appliances | pendant-lamp | 60 W | bulb-glass 60 W |
| appliances | circulation-pump | 45 W | motor 45 W |
| appliances | duct-fan | 28 W | hub 28 W |
| appliances | desktop-monitor | 25 W | housing 25 W |
| appliances | decentralised-hrv-fan | 4 W | hub 4 W |
| appliances | electricity-meter | 2.4 W | plate 2.4 W |
| appliances | room-thermostat | 0.7 W | casing 0.7 W |
| furniture | height-adjustable-desk | 225 W | drive-left 100 W, drive-right 100 W, monitor-housing 25 W |
| furniture | operating-table | 120 W | light-housing 120 W |
| humans | person-cooking | 1.68 kW | pot 1.50 kW, body 175 W |
| humans | jogger | 895 W | body 895 W |
| humans | cyclist | 414 W | body 414 W |
| humans | worker-carrying-box | 399 W | body 399 W |
| humans | kneeling-gardener | 348 W | body 348 W |
| humans | parent-with-stroller | 311 W | body 277 W, toddler 34 W |
| humans | skateboarder | 269 W | body 269 W |
| humans | seated-office-worker | 189 W | body 124 W, laptop 65 W |
| humans | seated-guitarist | 188 W | body 188 W |
| humans | courier-with-hand-truck | 177 W | body 177 W |
| humans | parent-carrying-toddler | 177 W | body 144 W, toddler 33 W |
| humans | person-on-crutches | 174 W | body 174 W |
| humans | tour-guide | 169 W | body 169 W |
| humans | child-on-kick-scooter | 169 W | body 169 W |
| humans | runner-with-blade-prosthesis | 155 W | body 155 W |
| humans | person-with-umbrella | 138 W | body 138 W |
| humans | pregnant-person | 135 W | body 135 W |
| humans | wheelchair-user | 133 W | body 133 W |
| humans | walking-commuter | 132 W | body 132 W |
| humans | elderly-with-cane | 126 W | body 126 W |
| humans | standing-phone-user | 122 W | body 117 W, phone 5 W |
| humans | person-with-white-cane | 119 W | body 119 W |
| humans | leaning-person | 119 W | body 114 W, phone-surface 5 W |
| humans | standing-adult | 117 W | body 117 W |
| humans | person-with-rollator | 114 W | body 114 W |
| humans | seated-reader | 98 W | body 98 W |
| humans | sleeping-person | 71 W | body 71 W |
| humans | running-child | 66 W | body 66 W |
| machinery | diesel-generator | 320 kW | silencer 320 kW |
| machinery | concrete-mixer-truck | 310 kW | engine-bay 310 kW |
| machinery | wheel-loader | 150 kW | engine-bay 150 kW |
| machinery | excavator | 110 kW | engine-compartment 110 kW |
| machinery | tower-crane | 70.0 kW | hoist-motor 55.0 kW, slew-drive 15.0 kW |
| machinery | forklift | 27.0 kW | drive-motor 15.0 kW, hydraulics 12.0 kW |
| machinery | oil-pumpjack | 22.0 kW | motor 22.0 kW |
| machinery | scissor-lift | 3.00 kW | motor-pump 3.00 kW |
| machinery | heat-pump-outdoor-unit | 2.85 kW | compressor-casing 2.70 kW, coil-guard 150 W |
| machinery | scroll-compressor | 2.50 kW | shell 2.50 kW |
| machinery | sorption-wheel | 1.53 kW | regen 1.50 kW, hub 30 W |
| objects | patio-heater | 13.0 kW | emitter-head 13.0 kW |
| objects | charcoal-grill | 8.00 kW | charcoal-bed 8.00 kW |
| objects | cooking-pot | 1.20 kW | water 1.20 kW |
| objects | retractable-bollard | 760 W | hydraulic-unit 750 W, beacon-led 10 W |
| objects | street-lamp | 60 W | lamp-housing 60 W |
| objects | tealight | 35 W | flame 35 W |
| vehicles | tram | 495 kW | traction-motors 480 kW, hvac-unit 15.0 kW |
| vehicles | city-bus | 262 kW | e-axle 250 kW, hvac-unit 12.0 kW |
| vehicles | delivery-van | 100 kW | engine-bay 100 kW |
| vehicles | city-car | 80.0 kW | engine-bay 80.0 kW |
| vehicles | vintage-car-1929 | 45.0 kW | engine-bay 45.0 kW |
| vehicles | motor-scooter | 8.00 kW | engine 8.00 kW |
| vehicles | electric-kick-scooter | 313 W | hub-motor 250 W, bms-controller 63 W |
| vehicles | cargo-bike | 250 W | drive-motor 250 W |
| vehicles | city-bicycle | 3 W | lamps 3 W |

0 W: appliances: 6; components: 18; furniture: 10; machinery: 5; objects: 9; vegetation: 16 (vegetation, passive elements and assets without a device; includes the pending gas-hob).

## Verification

- `🧪️tests/⚡️asset-power` (new, language-agnostic scenarios in `🥒️.feature`, hand-derived vectors `🧫️fixtures/⚡️asset-power/🔣️.json`; cross-checks: d3-array `fsum` for every sum, mathjs for the metabolic heat of all 42 bodies), `✅️asset-validation` (new case `power differs by more than 0.5 W`), `🧬️schema-twin`: pass. `🧬️schema-conformance`: pass.
- React: `🔎️search-filter-sort` (power filter, sort, number format), `🗺️address-roundtrip`, `📚️catalog-loading`, `🗂️grid-scroll`, `📖️detail-sections`, `🧭️viewpoint-pages`, `🧲️playground-drag`, `⚖️compare-state`, `🗜️catalog-payload`: pass.
- `tsc -p tsconfig.json --noEmit` of both packages: clean. `validate-all.ts`: failing 0 of 152 (baseline 0 of 152).

## Files changed

- `🧬️schema/🔣️.json`, `🧬️schema/🟦️.ts`
- `🔨️modules/🌡️thermal/⚡️power/🟦️.ts` (new), `📦️packages/🟦️typescript/🟦️.ts` (exports), `🔨️modules/🗂️catalog/🟦️.ts` (validator rules)
- `🗂️catalog/🔣️.json` (152 `powerW`), `🗂️catalog/👤️humans/person-cooking/🔣️.json`, `🗂️catalog/🧱️components/panel-radiator/🔣️.json`
- `🎯️targets/⚛️react/🔨️modules/🔎️search/🟦️.ts`, `🗃️browse/🟦️.tsx`, `⚖️compare/🟦️.tsx`, `🔬️detail/📐️dimensions/🟦️.tsx`, `📚️catalog/🟦️.ts` (`entryOf`: one line), `🌐️i18n/🔠️english/🧱️core.ts`, `📖️detail.ts`, `🔡️german/🧱️core.ts`, `📖️detail.ts`
- Tests and fixtures: `🧪️tests/⚡️asset-power/*`, `🧫️fixtures/⚡️asset-power/🔣️.json`, `🧫️fixtures/✅️asset-validation/🔣️.json`, react tests `🔎️search-filter-sort` (+ `.feature`), `🗺️address-roundtrip`, `📚️catalog-loading`, `🗂️grid-scroll`, `📖️detail-sections`, `🧭️viewpoint-pages`, `🧲️playground-drag`
- `README.md` (section Power); ticket helper `asset-power-totals.ts`.
