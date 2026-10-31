# Phase 2 — Audit, Contracts and Plan (coordinator: Opus 5.5)

S = `🎓️teaching/🏛️architecture/🧍️staffage`, T = this ticket folder. Inventory before: [inventory-before.md](inventory-before.md) (48 assets).

## Audit: underlying deficiencies

1. **Scale is a visibility filter.** One geometry set per asset; `Shape.lod` 0–3 hides shapes by `lodFor(denominator)`
   (`S/🔨️modules/📐️units`, `🎨️depiction/🧩️shapes` `prepare`), a global ISO pen table per role, a 0.4 mm detail cull. No per-scale
   silhouette, merging, gap treatment, hatching or decisions; nothing can be unavailable; custom scales snap silently.
2. **No print-suitability model.** No min stroke / gap / feature check; thin parts (fingers, spokes, glazing bars) print as noise at 1:500.
3. **Thermal colours are authored, not derived.** `core-resistance` regions carry an authored `coreTempC` per region (hands 35 °C, head
   36.8 °C) independent of scenario; `operating` is `T_air + riseK·level`; activity is an ad-hoc `0.15 K/met` bump; no skin-under-clothing
   distinction, no layers, no physiology, no thermoregulation, no heat capacity (instant response), no sun/shade split, no sky/ground radiant
   split. Quantities `radiant-flux` vs `net-flux` are ambiguous; ranges are auto per figure (not comparable).
4. **Plumes are frozen.** `streamlines` are one centreline from the *current* emitter pose under the *current* wind; parcels are placed on
   that same line by age, so turning the head swings the whole plume; breath is an on/off duty window with constant velocity
   (no flow profile, no tidal volume, no pauses, no chest motion); the same jet/puff law is reused for every category; transpiration is a
   point jet; no condensation; streamlines and pathlines are not distinguished.
5. **Clip boundaries reset** time (wrap) for everything; there is no absolute timeline for transport or root advance across loops.

## Decisions (binding)

- Schema `semio.staffage/v2` (`S/🧬️schema/🟦️.ts` — coordinator; JSON twin `🔣️.json` — core worker). No v1 support.
- **8 categories**: humans, animals, vegetation, furniture, objects, vehicles, machinery, components (furniture and machinery split out).
- **One file per asset**: `S/🗂️catalog/<emoji><category>/<asset-id>/🔣️.json` = `AssetDocument { $schema, version, asset }`. One owner per file.
- `Asset.representations` = `{ "500", "200", "100", "50" }` each `authored` (own pens, hatches, geometry, decisions, validRange) or
  `unavailable` (reason, limiting feature + real size, ≥ 1 alternative). `Asset.symbols` = separately identified conventional symbols with
  explicit dimensional semantics. `Shape.lod` and `Asset.geometry` are gone.
- Print profiles `S/🗂️catalog/🖨️print-profiles/🔣️.json`; suitability is **measured** on evaluated paper geometry (strokes, gaps between
  separate outlines, filled feature widths, hatch spacing) over named poses; failing ⇒ resolution `unsuitable` with violations + alternatives.
- Custom scale ⇒ the authored representation whose `validRange` contains it (labelled *derived*), else `unavailable`. Paper size always
  follows the requested scale (1.80 m = 3.6/9/18/36 mm). Validator checks every authored representation's default-pose bounding box against
  `dimensions` (±2 % or ±15 mm real).
- Thermal: explicit quantities (see `THERMAL_QUANTITIES` in `🔗️contracts`), layered regions, `BodyPhysiology` with Gagge-type two-node
  thermoregulation, operating heat release in W, conditioned building elements, leaf C3/C4/CAM, sun/shade + sky/ground radiation, first-order
  transient with region heat capacity. Shared documented comparison ranges per quantity; `auto` labelled as normalised.
- Respiration: `BreathingPattern` (rate, tidal volume, 4 phase fractions, route), half-sine flow (∫ = V_T), continuous phase integrated over
  pattern history, chest/abdomen motion bones, nostril/mouth emitters on the head.
- Transport `S/🔨️modules/💨️transport`: world-space Lagrangian parcels on a global fixed step grid (pure function of absolute time; play =
  scrub), entrainment, buoyancy, ambient wind timeline, pathlines vs instantaneous streamlines, condensation layer, category-specific sources.
- Physical scenario never depends on scale: evaluation/thermal/transport/structure ignore the representation (tests prove invariance).

## Ownership (one owner per path)

| Path | Owner |
|---|---|
| `S/🧬️schema/🟦️.ts`, `S/🔨️modules/🔗️contracts/🟦️.ts`, manifest `S/🗂️catalog/🔣️.json` | coordinator |
| `🧬️schema/🔣️.json`, `📐️units`, `🦴️rig`, `🎞️animation`, `🧮️evaluation`, `🗂️catalog` (+check, files), `🖨️representation` (new), `🎨️depiction` (except `🌡️thermal-view`), `📤️export/🖼️svg`, `🖨️preview`, print-profiles file, proof-sheet tooling | **core worker** |
| `🌡️thermal`, `🎨️depiction/🌡️thermal-view`, environments file, thermal tests/fixtures/oracles | **thermal worker** |
| `💨️transport` (new), transport tests/fixtures/oracles, breathing chest motion hook (function consumed by `🧮️evaluation`) | **transport worker** |
| `📤️export/*` except svg | **export worker** (wave B) |
| `🎯️targets/⚛️react/**`, `🌐️site/**` | **website worker** (wave B) |
| `🗂️catalog/<cat>/<asset>/🔣️.json` | the content worker listed in [authoring-v2.md](authoring-v2.md) |

Cross-owner interface changes go through the coordinator (message it; do not edit another owner's files).

## Waves

- A (now): core, thermal, transport (sonnet ×3); content conversion + new assets start in parallel against the TS twin (sonnet, ≤ 8 live).
- B: website + exports (sonnet ×2) once core/thermal/transport APIs land; remaining content workers.
- C: verification (haiku ×N audits), coverage matrix, proof sheets, motion review, final gates.

## Coordinator rulings (append-only)

1. Symbol shapes are static: evaluated in the asset's model frame (identity transform, no bone or pose transform); `bone` must be the
   rig root. `true-extent` = model metres; `true-position`/`nominal` = paper mm around the model origin. A symbol may offer only
   `plan` geometry (elevation may be an empty array) when the convention is plan-only.
2. Open (unfilled) stroke shapes may carry `region`; the thermal view paints such strokes in the region colour at the stroke pen.
3. Converted assets may base the 1:50 drawing on the v1 full-detail geometry, but it must be reviewed at 1:50 print size and its pens,
   hatches and decisions authored; smaller scales must be redrawn.
4. Schema addition: `ThermalModel.passengers: Passenger[]` (required, usually `[]`) for second living bodies (toddler in stroller or on
   hip, rider); passenger regions set `body.subject`; poses bind `thermal.passengers[id] = { activityMet, breathing }`. Thermal and
   transport solve each passenger like a body; its breath emitters come from its own `respiration`. No `operating` stand-ins for bodies.
5. Layer `resistanceM2KW`, vapour resistance and heat capacity are referred to the region's outer `areaM2` (intrinsic I_cl per skin
   area enters as I_cl·f_cl). `body.skinFraction` carries the skin area the region covers.
6. `ThermalRegion.hidden?: Text` declares a region not visible in elevation (canopy underside, seat underside); such regions are exempt
   from "drawn at every scale". All others must be drawn.
7. Bun is available only via `repo tools exec bun -- …` (not on PATH).

## Restart 2026-10-29 15:30 (coordinator session 2)

State found: all 48 assets rewritten as v2 by CV-* workers **without** the official check (their own validators only); 0 new assets;
engine modules for representation, thermal v2 (no `thermal-v2.md` yet), transport (report done) exist; check CLI had a syntax error
(fixed: escaped backticks in `🗂️catalog/🟦️.ts:245`). No previous workers alive. Machine capacity is shared: bun runs reserve 2 GB and
can queue for minutes — run checks per asset (`--asset`), never catalog-wide from content workers.

Wave 2 (now): core (finish M1+M2, make check/preview authoritative, proof-sheet + coverage-matrix tooling, transport plug-in in `depict`),
thermal (verify, finish, document `thermal-v2.md`, third-party oracle), transport (integration, determinism tests, exports of clips),
NW-H1, NW-H2, NW-H3, NW-A1, NW-A2, NW-V1, NW-V2, NW-F (new assets). Wave 3: NW-O, NW-VE, NW-MA, NW-C, CV fix-ups from the official
check, website + exports. Wave 4: audits, coverage matrix, proof sheets, motion review.
