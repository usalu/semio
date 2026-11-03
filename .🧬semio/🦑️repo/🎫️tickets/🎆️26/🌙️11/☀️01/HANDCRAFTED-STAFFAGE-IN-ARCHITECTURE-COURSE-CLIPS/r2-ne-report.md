# R2 ne: visual verification of staffage outside `⚡️energy`

Renders: ql, 15 fps, de, speaker, one scene at a time (`🧰️ne/chain.sh`, `render.sh`, `still.sh`), 3 to 5 frames each via `r2_frames.py`. Paths relative to `A = 🎓️teaching/🏛️architecture/`. No file was edited; no fix was needed. Generated output deleted.

## Call sites and scene classes

| Site | Cast | Scene |
|---|---|---|
| `🎯️optimization/🎬️clip/🎨️visuals/🏠️room/🐍️.py:587,588,1338,1349` (occupant, radiator, pendant lamp off/on, cooler), used through `ClipScene.room_section` | thermal | every optimization scene: `Beat1_LocalGlobalExtrema`, `Beat2_GridRandomSampling`, `Beat3_FactorialExperiments`, `Beat4_NormalizedIndex`, `Beat5_ParetoFront`, `Beat6_EvolutionarySearch` |
| `⚖️index/🐍️.py:135` criterion pictogram (pendant lamp) | `Beat4_NormalizedIndex` |
| `📉️extrema/🐍️.py:440` icons (radiator, cooler, lamp) | `Beat1_LocalGlobalExtrema` |
| `🧪️experiments/🐍️.py:275,904-905` (occupant, lamps, scatter icons occupant and window) | `Beat3_FactorialExperiments` |
| `📊️excel/🎬️clip/📋️program/🐍️.py:376` plan occupant | `Beat3_Program` |
| `🏗️bim/🎬️clip/📊️information/🐍️.py:212` car, `🔌️exchange/🐍️.py:876` people, `🔗️consistency/🐍️.py:273-1105` people | `Beat1_DataToInformation`, `Beat5_Interfaces`, `Beat6_Consistency` |
| `🦗️grasshopper`, `🏢️nakagin`, others | no staffage calls (only `cast` as a Grasshopper type conversion) |

## Verdicts

| Scene | Verdict |
|---|---|
| `Beat1_LocalGlobalExtrema` (4 frames) | OK. Handcrafted thermal occupant (orange, ground-contact), three pendant lamps, rear-wall cooler (teal) in summer, radiator (red) in winter; icons in the cost formula are the same casts. Person about 55 percent of the 3 m section height, so about 1.7 m. No clipping, no label overlap. |
| `Beat3_FactorialExperiments` (3 frames) | OK. Four mini sections plus big section: occupant grounded, lamps lit/unlit by crossfade, cooler flush at the top of the right wall, scatter icons fine. The tilted window icon stays a thin sash (known, r3). |
| `Beat4_NormalizedIndex` (3 frames) | OK. Pendant pictogram in the criterion cards and the room section are handcrafted; no primitives. |
| `Beat2_GridRandomSampling` (3 frames) | Staffage OK (lamps, cooler, radiator in the two mini sections; occupant omitted at that scale by design). Unrelated layout observation in the early and mid frames: the result list below the mini sections ("Nicht simuliert" / "Beleuchtung", and the right-hand result blocks) overlaps. Likely crossfade of the run panels; not staffage. |
| `Beat5_ParetoFront` (3 frames) | Staffage OK (occupant, lamps, cooler, thermal style). Unrelated observation: doubled text in "Sonnenhöhe 66°", the Licht/Kühlung/Heizung rows and the bar totals in all three frames, probably the simultaneous old/new text of the live-value crossfade. Not staffage, not changed. |
| `Beat6_EvolutionarySearch` (3 frames) | OK. Small and mid-size sections show occupant, lamps and cooler/radiator at consistent scale; no clipping. |
| `Beat3_Program` (excel) | Full render blocked, not by staffage: stalls in the Nakagin tower drawing (`🏢️nakagin/🎨️visuals/🐍️.py` `_section_order`) with no frame after 25 min and 2.4 GB; I killed it (known from r2-rx and r2-xl). Verified instead with the rx probe scene `🧰️rx/rx_plan_probe.py` (still): plan occupants are present in the window-wall corner of all five plans, ink style, true size, inside the net floor, clear of fittings, facing into the room. OK. |
| `Beat6_Consistency`, `Beat1_DataToInformation` (bim) | Blocked, skipped: model build fails with `AttributeError: 'types.SimpleNamespace' object has no attribute '_surface_finishes'` in `🏗️bim/🎬️clip/🧱️model/🎨️finishes/🧱️layers/🐍️.py:47 _downward_shadows` (finish/roof reserve area, agent br). Casts verified statically in r3 (render-audit-r3). |
| `Beat5_Interfaces` (bim) | Blocked: model build did not finish in about 25 CPU minutes, I killed it; same finish-reserve area. Casts unverified in this pass (r3 had checked people in `consistency` and the car in `information`). |

## Open

1. BIM scenes need the finish-reserve fix (br) before the people and car can be rendered again; the `_surface_finishes` attribute error is new relative to the roof-reserve description and may be a mid-edit state of the `🎨️finishes` module.
2. Excel `Beat3_Program` full render still needs the tower-drawing design change from r2-xl.
3. The text overlaps in `Beat2_GridRandomSampling` and doubled values in `Beat5_ParetoFront` should be checked by the owner of those clips with stills at clip-section boundaries.
