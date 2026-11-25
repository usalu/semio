# Python call sites: chrome verification of every missing call site

Driver: `python-call-sites.py` (one scene per site, each subclasses the clip's own scene class or calls its own module helper with sample content, static, 1280x720). Run: `python-call-sites-run.sh <Scene>` (`lockf` lock, `timeout 300`, `-ql -s -r 1280,720`, language en). Pngs: `evidence/python/site-*.png` (all < 40 KB, shrunk with `python-call-sites-shrink.py`). Bright yellow strips sit under the left part of each chip so glass-over-content shows. BIM representations stubs `quantities` and the model setup (the geometry query takes > 5 min); the rest is real.

## Per site (all looked at)
| Site | Result |
| --- | --- |
| BIM objects `_chip`, `_storey_chip`, `class_box`, `class_tree` | OK: square glass chips, strip visible through, header chip + one inner edge, readable |
| BIM representations info table `_card` | OK; title squeezed to 2.8 wide so it no longer touches "stays the same" (was 3.1) |
| BIM `guid_chip` | Fixed: label row is now a top glass chip (was a frame without chip) |
| BIM `property_table` with title, `model_card`, consistency `card` | Fixed: heading rule replaced by a top glass chip of the shared owner (`frame_box top_depth`); `panel(..., top_depth)` new |
| BIM exchange plates (`_backed`, `_ring`, `step_panel`) | OK (opaque text backing plate is not chrome) |
| Capsule `_light_card`, `_params_card`, `_card`, `_table` | Fixed: table units row ("m", "°") sat on the chip's inner edge; header chip deepened to `1.4 * pitch + 0.1`. Others OK |
| Nakagin `_chip` | OK |
| Trees `_plate`, `_chips` | OK (plate is an opaque text backing, chips glass, dark appearance on dark) |
| Casting `_tag`, `_order`, `_chips` | Fixed: chips on the light Grasshopper viewport were dark glass with dark text; `badge(..., appearance)` is new, casting chips, Grasshopper anatomy `_tag` and capsule "limit reached" pass `light` |
| Casting `_habits` cards | Fixed: heading was inside a plain rectangle; now a light frame with the heading and number in a top glass chip |
| Excel references `_card` / `_formula_card` caption, Name Manager | Fixed: caption/heading in a top glass chip (`_card(top_depth)`); Name Manager rows moved below its chip |
| Excel energy `_chain`, `_annual_tiles`, `_chip` | OK |
| Heating fundamentals `_spectrum` panel | Fixed: was a plain filled rectangle with the heading inside; now the shared frame with the heading as top chip |
| Primary-energy inset | Now `_upstream_inset(ground)` (helper), square frame without fill, caption below (derrick feet stay inside) |
| Cooling "Smart load control" | Fixed: rounded-corner-0 rectangle with loose title text; now `_control_panel` (shared frame, title chip), sliders moved 0.08 lower; bottom edge at y about -2.0, inside the safe zone |

## Optimization_Extrema
The orange "≈ 220.5 CHF/a" bill sat 0.28 under a text pair with the same pitch; it now sits at `rows_y[0] - .42`, a full line below `θ* = 360° · 882 kWh/a`. Real clip still `evidence/python/optimization-extrema.png` confirms.

## Primary energy staffage
`shared-heat-pump-heating-sweep` named clip `heating-sweep`, which does not exist on `heat-pump-outdoor-unit` (clips: `heating-modulation` = heating with modulating load, loop; `defrost-sequence`). Renamed cast and clip to `shared-heat-pump-heating-modulation` / `heating-modulation` in the cast manifest and all consumers (primary-energy clip + `🔣️scale.json`, system-losses, roadmap). A fast static sweep over all 98 casts (asset exists, clip exists, pose exists, against the catalog `🔣️.json` files) found that one only. New test `Catalog.test_every_cast_names_an_asset_clip_and_pose_of_the_catalog` in `🧍️staffage/🧪️tests/🐍️.py`.

## Tests
- Staffage `Catalog` 1 OK; visuals `scenes` 15 of 15 OK (was 14); semio-style 14 OK; capsule, grasshopper clip and excel clip tests 158 OK; cooling internal-gains 3 OK; BIM `chapter-source` 23 of 23 OK (was 21).
- `test_occupants_clear_open_window_swing`: real layout issue. `_room_occupants` assumed a body half-width of `0.2 * height / 2`, but the drawn figure (breath plume included) reaches 0.19 to one side and 0.63 to the other. It now measures the drawn extent of the first and last occupant (mirror alternates) and clears the windows by the gap on the drawn extent. Needs a real look at `Beat1_Lueftungsbedarf` (people are now spread wider); no video render done.
- `chapter-source` errors were harness drift, not source bugs: (1) the stub scene lacked `_reveal_then` (added to the stub, returns the reveal fade); (2) the real `load_tests` now also uses `ROOT_TEST_MODULES`, `_colocated_tests`, so the test executes those too, gives fake modules `__name__` and `__module__`, and counts every module through `importlib` (class modules are no longer loaded by `loadTestsFromName`).
- Failing, not caused here: primary-energy `🧪️tests/🐍️scale.py` 4 failures: catalog heights changed (person 1.72 vs 1.69, boiler 0.82 vs 0.72, pumpjack 6.878 vs 5.57, boiler bracket 0.74 vs 0.64) by the staffage lane; the vectors in `🔣️scale.json` need that lane's decision. The heat-pump vector passes.

## Open
BIM objects clip full retirement test (> 15 min) and real BIM/Grasshopper/Excel clip stills still not rendered; the driver proves the call sites, not full clips. Ticket not closed.
