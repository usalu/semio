# CV-H3 content report (jogger, person-with-umbrella, leaning-person, tour-guide)

Files: `S/🗂️catalog/👤️humans/{jogger,person-with-umbrella,leaning-person,tour-guide}/🔣️.json` (all four rewritten as `semio.staffage/v2`).
Scratch (inputs, kept): `T/🧰️scripts/CV-H3/` = `v1/` (copies of the v1 files, the raw material), `lib.py` (FK, SVG preview, bounds), `kit.py`/`body.py` (station tables
-> smooth paths, hand-entered numbers), `thermal.py`/`common.py` (layer/region/pattern helpers), `<asset>_build.py` (one per asset; all content is entered there),
`selfcheck.py` (JSON-schema validator for `🧬️schema/🔣️.json` plus semantic checks), `dims.py`, `view.py`. Evidence images: `T/🗑️generated/CV-H3/` (20 PNG).

## Status of the checks

- `T/core-v2-status.md` does not exist and the check CLI still reads v1 category documents, so the official check/preview could not be run. Instead:
  `selfcheck.py` validates every file against the v2 JSON schema (all four **OK, 0 errors**) and checks: every region bound by a shape at every scale (merged `regions` allowed),
  every pose binds every region and non-breath emitter and a known breathing pattern, metabolic shares and skin fractions sum to 1, emitter anchors and regions exist,
  clip tracks/keys resolve, references resolve, materials resolve, unique shape ids, validRange contains its denominator.
- `dims.py` evaluates the default pose through my own FK and Bezier bounds for every scale and every pose for ground penetration (worst: 4 mm, jogger stretch-hamstring, inherited from v1).
- Pens are my own assumption (500: 0.15; 200: 0.18/0.13; 100: 0.2/0.13; 50: 0.25/0.18/0.13); `🖨️print-profiles/🔣️.json` did not exist yet, so min stroke/gap/feature/hatch
  were not measured. They are single constants in `common.py` (`PENS_ALL`) and will need one re-run if the default profile demands more.

## Common decisions

- Rig, anchors, masses, poses, clips and `structure` of v1 are kept (bone names unchanged). Added anchors `nostrils` and `crown` on the head bone; breath emitters `nose-breath`
  (-150 deg, 9.5 mm) and `mouth-breath` (-100 deg, 12.4 mm), `plume` emitter at `crown`. Chest/abdomen motion on `torso` (1.2 deg, 4 mm) and `pelvis` (0.5 deg, 3 mm) per 0.5 L.
- Thermal: `model: body` regions with layered clothing (innermost first: air gap, fabric, shell; ISO 9920 clo x 0.155; vapour resistance = R/(0.38 x 0.0165)), skin fractions and
  metabolic shares sum to 1; shoes became a body region (sock, air, upper). `BodyPhysiology` with DuBois area, Mifflin-St Jeor basal metabolism (male, 30 a), Gagge set points and
  blood-flow limits. All pose bindings explicit (activity, breathing pattern, every region, plume strength; wettedness, ground contact), clips carry `breathing` step tracks.
- Provenance tags per `references-physiology.md`: skin areas (DuBois, `literature`); everything else (opening areas, jet angles, patterns, Gagge limits, exhaled state, clothing layers,
  heat capacities) is `authored-estimate`, the reference kept as "guided by" (gupta-2010, hoeppe-1981, gagge-1971, astrand-rodahl-2003, bramble-carrier-1983, winkworth-1995,
  mifflin-1990, incropera-2011, iso-9920-2007). The wall layers use `literature` with `din-4108-4` (brick 0.8, plaster 0.87 W/(m K)).
- Representations (all four scales `authored`, no `unavailable`): 1:50 = v1 drawing (face, hair, hands, shoes, seams, accessories) on new smooth round-capped limb bodies (v1 limbs had
  square ends that opened at knees/elbows); 1:100, 1:200 are new composed forms from hand-entered station tables (no simplification of 1:50); 1:500 = black silhouette of 13
  articulated overlapping forms (head+trunk+pelvis as one path, per side upper arm, forearm+hand, thigh, shank, shoe, toe) so every clip still animates. Toe bones are used at every
  scale (v1 toe-off penetration fixed). validRange 350-1000 / 150-350 / 75-150 / 20-75. Decisions (en+de) per scale and asset in the files.
- Dimensions corrected to the evaluated default-pose bounds: jogger heightM 1.82 -> 1.77 (forward-leaning run pose), umbrella 2.10 -> 2.09, leaning 1.72 x 0.60 -> 1.73 x 0.53 (wall
  and courses are `ground` role and do not enter the bounds). All scales within 1.3 % (500) / 0.5 % (others).

## jogger

- Respiration: `run` 41.67 bpm x 1.4 L (58 L/min, matches 8 met), I 0.46 / E 0.54, no pauses, **mouth**, 4:1 coupled to 2.8 Hz steps (one breath per two strides); `cooldown` 26 bpm x 1.1 L
  nose-and-mouth; `rest` 16 bpm x 0.7 L nose. Exhaled 34 C, 95 %. Flight phases: ground contact binding absent (flight poses), stance 0.06-0.26 of the shoe area at 22 C.
  Clip `stretch`: breathing step track cooldown -> rest, `emitter:plume` 0.6 -> 1, activityMet 4 -> 2.
- Layers: shirt 0.09 clo, shorts 0.05 clo, sock 0.04 clo + shoe; wettedness 0.2-0.3 while running. Symbol `runner-direction-mark` (true-position, 500/200).
- 1:500 toe wedge separate so the foot does not sink in midstance; 1:100 adds headband and armband; 1:50 hair redrawn (v1 hair read as a flap).

## person-with-umbrella

- Coat = shirt 0.25 clo + 12 mm air + vapour-tight shell 0.45 clo (vapour 30 m2 Pa/W, estimate); wet coat/canopy wettedness 0.1-0.45; canopy (0.87 m2, 600 J/m2K) and frame passive regions;
  furled pose binds canopy exposedFraction 0.35 and clip `close-umbrella` tracks it. Rain env 8 C/95 %. Patterns `rest`, `walk`, `brace` (mouth added only in the heavy gust);
  clip track breathing 0 -> 1 -> 2 -> 1 -> 0. Wind loads of v1 untouched (poses, `load:wind` track).
- Umbrella per scale: 500 two canopy lenses + one 0.15 mm shaft line (7.5 cm real, declared); 200 lenses + shaft/handle strokes 0.13 + two rib struts; 100 capsule shaft, hub, handle,
  ferrule, ribs, rim tips, seams; 50 all v1 parts + `gores` hatch (0.6 mm, 75 deg, 0.1 mm pen). Canopy halves stay bound to `rib-front/rib-back`, so furling/tilting works at every scale.

## leaning-person

- Wall = `conditioned` element (15 mm plaster + 365 mm brick, interior 20 C, film 0.13, 150 kJ/m2K). `RegionBinding.contact`: torso-clothing against the wall 0.08 (0.12 with the seat
  in arms-folded) at 17.5 C, hand on the phone 0.12 at 27-29 C, head on the phone 0.03 at 30 C in `phone-call`, shoes on 15 C pavement 0.5; none for the wall at `pushing-off`
  and `standing-away` (contact conduction ends, event at 1.4 s). Phone = `operating` region (1.5 W, key `phone`: off / screen on 0.3 / call 1), tracked in the clip.
- Speech breathing `phone-talk` 12 bpm x 0.75 L, I 0.10 / E 0.78, mouth (winkworth-1995, estimate); clip `phone-call-idle` switches rest -> talk at 3 s (matches the `talk` phase).
- Wall as line (`ground` role) at all scales, courses ticks at 100/50 (ground role, 0.13 mm).

## tour-guide

- Layered: shirt 0.25 clo + air + softshell 0.3 clo (new material `softshell`), trousers 0.24 clo; passive flag cloth, aluminium pole (new region `pole-metal`) and pointer; sun 250 W/m2
  (diffuse 0.35), sunlitFraction 0.5 on the body (0.3 when turned), 1.0 on flag, pole and pointer.
- Talking: `talk` 11 bpm x 0.9 L, I 0.10 / E 0.80 (a 4.4 s controlled expiration), mouth; `talk-loud` 10 bpm x 1.3 L (waving, calling); `rest` nose. `gesture-point` inspires 0.8 s before
  pointing, `turn-and-wave` switches rest -> talk -> loud and tracks `region:torso-clothing:sunlitFraction` while turning.
- Accessories per scale: 500 pole/pointer as single lines (10/7.5 cm real, declared) with a flag lens; 200 strokes 0.13; 100 true-width capsules, finial, emblem; 50 v1 detail.

## Images looked at (T/🗑️generated/CV-H3/)

For each asset the default pose at 1:50 (25 px/mm), 1:100 (30), 1:200 (50), 1:500 (120) plus jogger flight-l (100), umbrella gust-heavy (200), leaning pushing-off (100), guide waving (100);
all read at their scale; the 1:500 figures read as people with umbrella/pole/wall; limbs stay closed in running poses. (Hatches are not drawn by my previewer.)

## Remaining doubts

1. Official check/preview CLIs and the default print profile were not available: min stroke/gap/feature/hatch spacing unverified; hatch `gores` not rendered.
2. Respiration `motion` values (per tidal volume, `dy` on a revolute bone) are my reading of the schema doc; the engine may scale them differently.
3. 1:500 forearm+hand and head/trunk/pelvis are rigid merges taken from the default pose (head turns of 15 deg are invisible at 0.2 mm).
4. Jogger summary/decisions mention the standing height 1.82 m while `dimensions.heightM` is the run-pose bound 1.77 m; manifest rows (heightM, widthM, representations, symbols,
   analytics) need syncing by the coordinator (leaning 1.73 x 0.53, umbrella 2.09, jogger 1.77).
5. Thermal contact temperatures (wall 17.5 C, phone 27-30 C) and heat capacities are estimates tagged `authored-estimate`.
