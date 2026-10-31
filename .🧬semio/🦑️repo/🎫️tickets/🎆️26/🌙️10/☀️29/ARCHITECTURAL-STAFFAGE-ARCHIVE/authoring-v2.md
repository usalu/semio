# Content Authoring Guide v2 (every content worker)

S = `/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage`, T = this ticket folder. Read first: `S/🧬️schema/🟦️.ts` (v2, normative
types with docs), [phase2-plan.md](phase2-plan.md), and for engine status `T/core-v2-status.md`, `T/thermal-v2.md`, `T/transport-v2.md`
(written by the engine workers as they land). The v1 guide [authoring.md](authoring.md) still holds for rigs, poses, clips, masses,
contacts, loads and provenance — everything below **replaces** its geometry/LOD/thermal parts.

## Files and ownership

- One file per asset: `S/🗂️catalog/<emoji><category>/<asset-id>/🔣️.json` = `{ "$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument",
  "version": "semio.staffage/v2", "asset": { … } }`. Edit **only** the asset folders assigned to you. Never edit the manifest.
- Everything is handcrafted. Scratch scripts in `T/🧰️scripts/<your-worker-id>/` may compute coordinates (mirroring, unit conversion,
  offsetting), transform JSON or render previews; you design and review every shape and number. No generic simplification algorithm may
  produce a representation. Generated previews go to `T/🗑️generated/<your-worker-id>/`.
- Tools (once `T/core-v2-status.md` says they are ready):
  `repo tools exec bun -- S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id>` (validation + evaluation + suitability + all views) and
  `repo tools exec bun -- S/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset <id> --scale 500|200|100|50 [--sheet] [--print-size] --out <dir>`.
  Look at the PNGs with the Read tool at print size **and** zoomed; iterate.

## Representations — the heart of this phase

Every asset decides all four keys `"500" "200" "100" "50"`. Each `authored` representation is designed for its scale on paper:
a 1.80 m person is 3.6 / 9 / 18 / 36 mm tall. All representations bind to the same rig bones (poses and clips animate every scale) and
must reproduce `dimensions` exactly in the default pose (no enlarging for legibility). Per representation author:

- `geometry.elevation` (and `plan` where the asset offers plan) — its own shapes. Design per scale:
  - **1:500**: one composed silhouette (1–4 closed paths), no or one internal edge; merge limbs/parts whose gap < profile min gap;
    thin parts (spokes, poles, cane, fingers, glazing bars) either thickened *only if* that stays within the real outline tolerance,
    merged into a mass, or dropped — say which. Pens ≈ 0.13–0.18 body, detail usually `null`.
  - **1:200**: main masses separated (head/hair, torso+clothing, arm, legs per side, main accessory), 1–3 key internal edges, no texture.
  - **1:100**: articulated clothing outlines, hands as mitten forms with thumb, shoes, hair mass, accessory construction; restrained
    hatching (material indication) where spacing ≥ profile min hatch spacing.
  - **1:50**: full articulation where printable: finger groups, seams, folds, faces in profile, bark, tread, glazing bars, joinery.
  Do not copy one set and delete shapes: redraw contours so each scale's silhouette reads well (e.g. a 1:500 tree crown outline is a
  carefully composed lobed path, not the union of the 1:50 foliage masses).
- `pens` (paper mm, per role; `null` = role not drawn), optional `penMm` per shape, `hatches` (paper mm spacing/pen, angle).
- `decisions` (en+de, specific to this asset and scale): `silhouette`, `retained`, `merged`, `omitted`, `thinElements`, `hatching`,
  `styles` (how dark/light line/solid read), `overlays` (how thermal/structural overlays map onto this geometry — merged regions etc.).
- `validRange`: custom denominators this drawing is valid for (contains its own; neighbouring ranges should not overlap; typical:
  500 → 350…1000, 200 → 150…350, 100 → 75…150, 50 → 20…75; shrink when a drawing breaks earlier).
- Thermal binding of geometry: every filled shape that shows a surface carries `region` (one region) or `regions` (a merged form at
  small scale, e.g. the 1:500 silhouette lists all body regions). At every authored scale every drawn region must appear.
- `unavailable` when the asset cannot be meaningfully printed (e.g. hedgehog at 1:500 = 0.5 mm): `reason`, `limitingFeature`,
  `limitingSizeM`, `alternatives` (another scale, a symbol, or another asset). Never leave a scale undecided.
- `symbols` (optional): conventional signs (tree plan symbol, bollard dot, north-arrow-like markers), with `semantics` and
  `dimensionalNote` stating what the size means. Symbols are separately identified, never silently used for a scale.
- Default print profile is `office-laser` (see `S/🗂️catalog/🖨️print-profiles/🔣️.json`); the check reports measured min stroke, gap and
  feature; an authored representation should pass the default profile — if it cannot, redesign or declare it unavailable.

## Physical scenario (scale-independent)

- **Thermal regions v2**: `model` body | leaf | passive | operating | conditioned; `geometry` for convection; `skyViewFactor`;
  `heatCapacityJM2K` (sets the response time constant; skin+clothing ~ 10–30 kJ/(m²K), car body steel ~ 6 kJ/(m²K), engine block much
  larger, leaves ~ 0.5–1 kJ/(m²K)); `layers` innermost first with `resistanceM2KW` (clothing: ISO 9920, 1 clo = 0.155 m²K/W, include the
  enclosed air layer), `vapourResistanceM2PaW` (clothing: `I/(16.5e-3·i_m)` with `i_m ≈ 0.38` unless sourced), fur/feather insulation
  from literature (cite); body regions carry `body { segment, metabolicShare, skinFraction }` (shares and skin fractions each sum to 1);
  bare skin has `layers: []`. `areaM2` = outer surface area (clothing area factor included).
- **Do not author temperatures.** No region has a fixed temperature (except reference fixtures). Bodies get `thermal.body`
  (`BodyPhysiology`): mass, skin area (DuBois for humans), basal metabolism, `metWM2` (58.15 for humans; Kleiber-based for animals, cite),
  set points, sweating/panting, skin blood flow limits (Gagge two-node: 6.3 basal, max 90 L/(m²h) humans; animals cite or estimate).
- Machines: `thermal.operating` keys with named levels (off / idle / rated / …); `operating` regions with `heatW` released into that
  surface at level 1 (engine block, exhaust pipe, motor, lamp, heater); exhausts as `exhaust` emitters with `volumeFlowM3S`,
  `temperatureRiseK`, `vapourKgPerKg` at level 1 and `operatingKey`. Cite manufacturer/literature or tag `authored-estimate`.
- Vegetation: foliage regions `model: "leaf"` with `leaf { stomatalConductanceMaxMS, leafAreaIndex, photosynthesis }` (CAM opens at night),
  bark/stem `passive`; a `transpiration` emitter per crown with `region` and distributed `anchors` (≥ 5 across the crown).
- Components: `conditioned` elements (wall, window, door) with `layers` giving the U-value and `conditioned.interiorTempC`; heated parts
  (radiator) as `operating`; sun-exposed vs shaded surfaces via pose `sunlitFraction`.
- **Respiration** (humans, animals): `thermal.respiration` with nostril and/or mouth emitters (`kind: "breath"`, anchors on the head bone,
  `directionDeg` relative to the head bone: nose ≈ 60° below the face direction, mouth ≈ 0…−20°, guided by Gupta et al. 2010 — angles/areas are UNVERIFIED, tag authored-estimate, see T/references-physiology.md; `diameterM`
  from opening area: nostrils ≈ 0.71 cm² → 9.5 mm, mouth ≈ 1.2 cm² → 12.4 mm for adult humans; scale for animals), `exhaledTempC`,
  `exhaledRelativeHumidity`, `patterns` (rest, active, …: rate, tidal volume, I/pause/E/pause fractions, route) and `motion` (chest /
  abdomen bones with small `angleDeg`/`dy` per tidal volume — subtle, e.g. 1–2° / 3–6 mm for an adult at 0.5 L). Typical: adult rest
  12–15 bpm / 0.5 L / I 0.38, P 0.04, E 0.45, P 0.13; walking 18–22 bpm / 1.0–1.3 L; running 35–45 bpm / 2–2.5 L; child higher rate,
  lower volume; dog 15–30 bpm, ≈ 10–15 mL/kg, panting 200–300 bpm small volume; horse 8–16 bpm, 5–6 L; cow 26–50 bpm 3–4 L;
  cat 20–30 bpm ≈ 10 mL/kg; small birds cite. Every pattern cites its source.
- Bodies also get a `plume` emitter at the crown/back anchor (free convection above the warm body; strength from the solved sensible heat).
- **Pose bindings (explicit)**: every named pose and key pose binds `thermal.regions` for **every** region (`exposedFraction`,
  `sunlitFraction`, optional `contact` with seat/ground temperature and fraction, optional `wettedness`), `thermal.emitters` for every
  non-breath emitter, bodies `activityMet` + `breathing` (pattern id), machines `operating` for every key. Clips may add tracks
  (`breathing` index step track, `operating:<id>`, `activityMet`, `emitter:<id>`, `region:<id>:sunlitFraction`).
- Clip events no longer contain `inhale`/`exhale` (respiration is derived). Machine clips mark `ignition`/`shutdown`/`operating`.

## Quality bar for new assets

Same as v1 plus: distinct identity (activity, clothing, proportions, mobility aid, structure, operating state, material), ≥ 3 named poses,
≥ 2 clips (≥ 1 looping where meaningful), rig per category, masses summing to `dimensions.massKg`, supports/contacts, loads, materials with
sources, full provenance (`literature` only with a real, checkable reference; else `authored-estimate`/`illustrative`), titles/summary in
natural English and German, `completion: "authored"`, `presentation.defaultScale`.

## Assignments

| Worker | Assets |
|---|---|
| CV-H1 convert | walking-commuter, seated-reader, running-child, wheelchair-user |
| CV-H2 convert | elderly-with-cane, cyclist, worker-carrying-box, parent-with-stroller |
| CV-H3 convert | jogger, person-with-umbrella, leaning-person, tour-guide |
| CV-A convert | labrador-dog, domestic-cat, horse, pigeon, roe-deer, dairy-cow |
| CV-V convert | english-oak, lombardy-poplar, multi-stem-birch, scots-pine, date-palm, weeping-willow, hornbeam-hedge, miscanthus-grass |
| CV-FO convert | park-bench, cafe-chair, office-swivel-chair, height-adjustable-desk (furniture); street-lamp, market-parasol, waste-bin, playground-swing (objects) |
| CV-VM convert | city-car, city-bus, cargo-bike (vehicles); excavator, tower-crane, forklift (machinery) |
| CV-C convert | cantilever-balcony, glulam-beam, steel-column, stair-flight, sliding-door, tilt-window, glass-balustrade, tied-canopy |
| NW-H1 new | person-with-rollator, person-on-crutches, person-with-white-cane, child-on-kick-scooter |
| NW-H2 new | skateboarder, kneeling-gardener, parent-carrying-toddler, seated-guitarist |
| NW-H3 new | standing-phone-user, courier-with-hand-truck, pregnant-person, runner-with-blade-prosthesis |
| NW-A1 new | sheep, goat, pig, red-fox |
| NW-A2 new | mallard-duck, herring-gull, red-squirrel, hedgehog |
| NW-V1 new | london-plane-pollarded, small-leaved-lime, apple-tree, japanese-maple |
| NW-V2 new | bamboo-clump, ivy-on-trellis, agave, hosta-clump |
| NW-F new (furniture) | three-seat-sofa, dining-table, bookshelf, bar-stool, picnic-table, folding-deck-chair, single-bed |
| NW-O new (objects) | patio-heater, charcoal-grill, stepladder, shopping-trolley, drinking-fountain, retractable-bollard, bicycle-stand |
| NW-VE new (vehicles) | motor-scooter, electric-kick-scooter, delivery-van, tram, city-bicycle |
| NW-MA new (machinery) | concrete-mixer-truck, scissor-lift, wheel-loader, diesel-generator, heat-pump-outdoor-unit |
| NW-C new (components) | wheelchair-ramp, panel-radiator, external-venetian-blind, sectional-garage-door, spiral-stair, masonry-wall-with-lintel, curtain-wall-bay |

Before: 48 assets. After: 107 (59 new). Report per asset: representation decisions summary, unavailable scales with reasons, check output,
preview files looked at, remaining doubts — into `T/content-v2-<worker-id>.md`.
