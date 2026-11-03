# r2 ws: wall-strip scenes after the radiator lost its wall

Scripts: `🧰️ws/ws_render.sh` (demand-series variant of `r2_render.sh`), `🧰️ws/ws_crop.py`. Generated output deleted. No source edits were needed.

## Scenes checked (ql, 15 fps, de, one at a time, frames via PyAV, 8x crops)
- `regulations` Beat4_Gradtagszahl (`_stage(with_radiator=True)`, `⚖️regulations/🐍️.py:421-429`): the strip abuts the radiator with no gap or overlap, same cream cut material as the house section walls, 0.15 m thick and as high as the cast. The radiator foot (supply pipe) reaches the slab; the body stands above it. Heat rings and the rising flow show while heating (about 40 percent of the beat); off in the "Rechenkonvention" step (about 48 percent), as the story requires. The house is small (storey about 1.1 u), so the strip reads as a thicker white post beside the thin radiator at 480p.
- `demand-power` Beat2_Normheizlast (`_heated_house`, `⚡️demand-power/🐍️.py:253-256`): both floors show the strip against the radiator, standing on the slab, same material as the walls; radiator rings and loss arrows present in "nur Verluste zaehlen" (about 50 percent) and the red loss arrows in the "Raum fuer Raum" step (about 40 percent). No gap or overlap on either floor.
- system-losses Beat2 was rendered by ap; recap was fixed separately by rm (front-view cast, no strip), so both were skipped.

## Notes
- First demand-power render died with `ValueError: unsupported path command 'l'` while loading `shared-person-winter` (staffage parser, `🧍️staffage/🐍️.py:384`), i.e. a foreign, transient edit by another agent; the immediate re-run succeeded. Not related to the radiator.
- Radiator mount height: the cast is a standing radiator whose body starts 0.11 m above the slab with the supply pipe to the floor; the strip is the full 0.71 m cast height, so the strip bottom sits on the slab. The strip is a free-standing stub near the window wall (same footprint as the old asset wall), unchanged.
- Tests not re-run: no files edited; ap's `staffage_only` pass (50 tests) covers the current tree.
