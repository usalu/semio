# Round 2, visual check of merged casts (agent rr)

Scripts in `T/🧰️rr/` (`rr_render.sh` one scene, ql, 15 fps, jobs 1; `rr_strip.py` zoomed crops; `rr_montage.py`). Generated output deleted.

| Scene | Casts covered | Verdict | Fix |
|---|---|---|---|
| ventilation volume `Beat2_Luftwechselrate` | `ventilation-volume-cook/-hood/-operating-table` (now `office-neutral`), `shared-person-pathlines` (operating-table nurse), office worker, students | OK. Cook and hood read warm and fit a kitchen; operating table cool-neutral with the standing person at the same height as the table; office and classroom glyphs unchanged, plumes animate, nothing clipped | none |
| ventilation natural `Beat2_StossOderKipp` | `ventilation-natural-radiator-on/-off` (`office-neutral`, scale 20) | OK. The running radiator is orange to pink with fins and heat ripples, the off one is dim; heat plume animates | none |
| ventilation mechanical `Beat3_Anlagenvergleich` | `shared-person-pathlines` occupant (was `indoor-heated-winter`), mvhr | OK. The occupant fills about 0.7 of the room height in all three cards, same as the other ventilation rooms; pathline plume differs between frames. Winter story is carried by the 0 degC/20 degC air labels, not the occupant | none |
| heating-comparison `Beat1_WinterGains` | `shared-person-winter`, `shared-lamp-winter`, desk, laptop | OK. Winter person warm orange standing at 0.7 of a storey; lamps glow | none |
| heating-comparison `Beat2_SummerOverheat` | `shared-person-fanning`, `shared-lamp-summer` | OK. Summer person fans (arm moves between frames), same scale as the winter one, clearly different from the winter scene; red-tinted room | none |
| optimization experiments `Beat3_FactorialExperiments` | `shared-lamp-*`, occupant and window legend at 0.5 u/m | OK. Room occupants identical across the four sim cards and the big room; legend row (cloud, occupant, window) legible, no overlap with the dashed spread box, no clipping | none |
| physics fundamentals vacuum | `physics-fundamentals-vacuum` (`office-neutral`) | Not seen: the rendered `Beat1_Leistung` (11 min) shows the power scale and annual curve at the sampled instants, not the device. Standalone device, a 21 vs 22 degC ambient change is only a faint tint change. Left unverified | none |

Not rendered (time): summer-strategy, cooling internal-gains (`shared-desk`, `shared-lamp-summer`), certificate/practice/recap (`shared-person-winter`, `shared-person-neutral`), bim reviewer (`shared-person-relaxed-line`), demand device casts.

No merge needed a split. `staffage_only` guard untouched (no source edits).

Notes:
- The staffage TS exporter failed twice mid-run (`LANGUAGES is not defined`, `base0 is not defined`) from other agents' half-written edits; the plain rerun succeeded each time.
- Each render took 4 to 15 minutes under fleet load; the `SEMIO_TEACHING_LANGUAGE=en` env had no effect on the volume and natural clips (German captions), which is irrelevant for staffage.
