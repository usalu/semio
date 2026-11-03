# Asset agent A1 report

Builders (scratch, deterministic): `🧰️a1/worker.mjs`, `🧰️a1/sleeper.mjs` (helpers `lib.mjs`, preview tools `render.sh`, `sheet.mjs`, `preview.sh`, `allviews.sh`). Re-running them rewrites the two catalog files.

## seated-office-worker (`S/🗂️catalog/👤️humans/seated-office-worker`)
- Man, 1.78 m, 76 kg, seated-reader skeleton scaled by 1/0.944; shirt, suit trousers, leather shoes, short hair, collar are new.
- The office-swivel-chair is embedded as a sub-assembly (bones/shapes/regions prefixed `chair-`, backrest tilts). Desk fragment (top 0.72 m, end panel) and open laptop are part of the figure, so a single cast is self-contained (casts hold one asset; the catalog desk already carries a monitor, which would clash).
- Poses: `typing` (default), `reading-screen`, `leaning-back` (hip shift solved so the back touches the backrest; hands behind the head).
- Clips: `typing` (4.8 s loop, alternating finger lifts, head dips), `settle-back` (7 s).
- Thermal: 1.2 / 1.1 / 1.0 met; office clothing about 0.57 clo (vest 0.06, shirt 0.25, briefs 0.04, trousers 0.25, socks 0.03, shoes 0.03); DuBois 1.937 m2; basal 82 W; laptop is an `operating` region (key `laptop-load`, 30 W, fan emitter 1.2 L/s +10 K); indoor reference 22 degrees C. Dimensions 1.411 x 1.291 m.

## sleeping-person (`S/🗂️catalog/👤️humans/sleeping-person`)
- Woman, 1.70 m, 64 kg, supine chain of the reader skeleton; head, hand, feet are the reader's drawn forms, duvet (4 mound pieces plus hanging skirt) and pillow are new.
- Bed fit: root `bed-surface` at y = 0.46 m (single-bed mattress top in its `asleep` pose), head end at x = 0, hip joint at x = 1.05. No bed geometry in the figure: draw a `single-bed` cast with the same origin under it.
- Poses: `supine` (default), `side`, `curled`. Clips: `breathing` (4.8 s loop), `turn-over` (8 s).
- Thermal: 0.8 met (0.95 during turning); pyjama top 0.12 + shorts 0.06 clo under duvet about 1.6 clo (Lin and Deng 2008, authored estimate); mattress contact 29 degrees C (22 to 34 % of area); basal 66 W; reference air 17 degrees C. Dimensions 0.45 x 1.9 m.
- Known limitation: the elevation keeps the profile convention for both lying poses (the side pose is shown as the knee ridge, not a view of the back); the larger mattress contact lives in the thermal binding.

## Verification
- Catalog check (`🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id> --samples 5`): both pass for all four scales, office-laser suitable. Manifest rows added after `seated-reader` entry (one Edit).
- Vitest (catalog-physics, seven-modes, asset-validation, schema-conformance, clip-sampling, respiration): 491 of 492 passed; the one failure (`category-missing`, manifest consistency) was a foreign concurrent edit (`appliances` category was not yet in the manifest when it ran; present now).
- Previews looked at (chromium raster, rsvg-convert is not installed): every pose in dark-solid, dark-line and thermal at 1:20. Worker: arms read well on the keys, chair and desk consistent with the other humans; the hands-behind-head hand reads slightly like a ponytail in thermal. Sleeper: breathing head profile on the pillow reads clearly; side/curled duvet pieces show seams at the joints. light-line, light-solid, forces and moments were not rendered separately (the check and seven-modes test evaluate them).
