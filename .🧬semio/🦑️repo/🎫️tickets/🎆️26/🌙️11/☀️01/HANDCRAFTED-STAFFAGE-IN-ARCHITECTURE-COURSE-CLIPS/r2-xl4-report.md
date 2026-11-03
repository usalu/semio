# r2-xl4: peak memory of the Nakagin tower drawing (`test_native_lifecycle`, `📋️program`)

Verdict: the 3 GB target is not reached. One whole-building drawing at the program-clip fit now peaks at 3.18 GB (was above 6.9 GB, killed at the 6 GB guard), frames pixel-identical after every step. The actual lifecycle test is a different, heavier case (480x270 config, about 1.2 to 1.5 million faces, several live towers and 140-capsule sets), and it still crosses 8 GB. The causes are structural and listed under "Why 3 GB is out of reach".

## 1. Measurements (`🧰️xl4/xl4_peak.py full`, program fit, 1080p config, 791 321 shapes, warm ordering cache, working set from `GetProcessMemoryInfo`)

| step (cumulative) | peak | after template | note |
|---|---|---|---|
| baseline (xl3 state) | above 6.86 GB (guard kill during `template.copy()`) | 4.4 GB | template build 4.9 GB |
| 1 lean faces (`_Facet`) | 5.51 GB | 3.61 GB | face 3.19 KB -> 1.30 KB (bare quad) |
| 2 orders over 50 000 rows not kept in memory, lazy `_Restored` items, streaming bundles in `_paint_ordered` | 4.90 GB | 2.84 GB | fragment items no longer all alive at once |
| 3 `_Facet.__deepcopy__` shares read-only world data | 4.12 GB | 2.84 GB | clone cost 1.7 GB -> 1.2 GB |
| 4 template faces share read-only colour arrays (`_share_style`) | 3.68 GB | 2.84 GB | clone 0.77 GB |
| 5 template frozen to a temporary file, clones thawed (`_Frozen`) | 3.18 GB | 0.90 GB after freeze | freeze peak 3.02 GB, thaw +2.18 GB, 56 s |

Pixel identity (`🧰️xl4/xl4_frames.py`, Cairo, 1920x1080, seven variants: plain, tint, types, lit, other fit and scale, repeated call, `tower_capsules`; all pixels of the frame compared against frames stored before the first edit): `RESULT IDENTICAL total differing=0` after steps 1, 2, 3, 4 (twice) and 5 (forced with `XL4_FROZEN=0`, also after the compact recipes below).

The same script measured 2 storeys, 26 857 shapes: peak 688 MB -> 438 MB.

## 2. Lifecycle test itself (`🧰️xl4/xl4_lifecycle.py`, guard 8 GB)

`test_native_lifecycle` runs at 480x270. The coverage padding is two pixels, so it is four times the 1080p padding and far fewer faces are culled: the ordering file has 203 MB (1080p: 129 MB), the thawed drawing needs 4.1 GB alone (about 1.2 to 1.5 million faces).

| run | result |
|---|---|
| xl3 final (5 GB guard) | killed during first shape creation, 5.1 GB |
| xl4 lean + lazy items, memory template (`life1`) | ordering (cold, one time) 3.9 GB, template 4.3 GB, first clone 5.5 GB, killed at 8.2 GB when the scene built its second live tower |
| xl4 frozen (`life2`, warm ordering) | freeze peak 4.5 GB, first thaw 5.2 GB, killed at 8.3 GB in the loop that copies 140 capsules (99 `tower_capsules` calls) |

## 3. Why 3 GB is out of reach with one VMobject per face

- Floor of one live drawing: points 1.4 KB per face (the material-coverage shapes hold every polygon of a capsule again) plus about 1.3 KB of object overhead, so 2.2 GB at 791 k faces and 4.1 GB in the lifecycle config. float32 points would change Cairo's 24.8 rounding and break pixel identity; merging runs was ruled out in xl3.
- The clip holds several of these at once: `Beat3_Program.tower()` through `Live` (old and new during a rebuild), `tower_keep` (line 745) next to `tower()` and, in `silhouettes` and the scatter morph, sets of 140 full capsule copies (each capsule is about 10 000 faces).
- Post-order single-cover culling (`🧰️xl4/xl4_cull.py`, padded bounding box inside one later convex polygon) hides only 11.3 percent of the shapes, so option (c) gains almost nothing on top of the pre-order culling. Option (d) was measured in xl3 (3 percent). Option (a) is done: no deepcopy doubling, the template is a file.
- To reach 3 GB one needs either at least four times fewer faces or bytes per live tower, or a clip that never holds more than one tower plus lightweight silhouettes. Candidates for the owner: draw `silhouettes` and the morph sources from hulls instead of 10 000-face capsules; recolour one persistent tower instead of rebuilding it (`Live`); a coarser coverage rule at low resolution is not pixel-identical.

## 4. Changes (`🏢️nakagin/🎨️visuals/🐍️.py` unless noted)

- `_Facet` (lean `VMobject` subclass: defaults on the class, attributes assigned one by one so faces share a key table), `_lean`, `_adopt`, `_default`, `_interned`; `_shape` and boundary-line children return lean faces.
- `_Facet.__deepcopy__`: copies points and colour arrays, shares the read-only world data (`_WORLD`); world arrays are frozen (`_frozen`).
- `_remember_order` / `_ORDER_MEMORY_ROWS`: orders above 50 000 rows stay on disk only. `_Restored`: items bound on demand. `_paint_ordered` accumulates bundles directly instead of keeping every item. `_order_recipes` stores arrays instead of nested tuples. `_fragment_item` shares one read-only normal per fragment.
- `_share_style`: shared read-only colour arrays in templates below `_FROZEN_SHAPES` (200 000).
- `_Frozen`, `_Saver`, `_Loader`: templates of at least 200 000 faces rest in an unnamed temporary file under `.🧬semio/🎓️teaching/⚡️cache/nakagin-shapes` (removed when the process ends or is killed); the group skeleton is a pickle with face numbers as persistent ids, faces are read back one by one (`thaw`, `capsule`), world data (`world_vertices`, `world_normal`, `world_depth`, `world_facets`) is read lazily through `_Facet.__getattr__`. `tower_axon` and `tower_capsules` use it transparently.
- `tower_capsules(keys, None, ...)` keeps the neutral paint; `📋️program/🐍️.py`: `capsule_center` and the two former `_axon.capsules[...]` uses call `tower_capsules`, the full `_axon` tower in `present_program` is gone (one live tower less).
- Tests: `🏢️nakagin/🧪️tests/❄️frozen/🐍️.py` with `🧪️cases/❄️frozen/🔣️.json` (lean face equals plain manim shape and pixels, copies independent, large orders stay on disk, frozen tower equals memory clone for every shape, world data and pixels, thaw independence), 5 tests.

## 5. Verification

| check | result |
|---|---|
| occlusion, order-cache, fragments, grouping, frozen (`m9_tests.sh`) | `Ran 52 tests in 297 s, OK` |
| Excel presentation: all `SceneContract` tests except `test_native_lifecycle`, `WorkbookToolkit` | `Ran 19 tests in 238 s, OK` |
| `🏢️nakagin/🧪️tests/🎨️visuals` | `Ran 22 tests in 5667 s, OK` (builds whole towers through the frozen path) |
| `test_native_lifecycle` `📋️program` | not met (section 2) |

## Open

- Memory target (section 3), needs a clip-level decision.
- The first ever lifecycle run still computes the ordering cold (3.9 GB, about 40 minutes) and writes `$cache/nakagin-orders`; later runs load it.
- `_Frozen` writes about 1.6 GB per process (88 s at 791 k faces) and a thaw takes 56 s; a persistent shape cache keyed on all rendering sources would remove both but needs a stale-entry guard wider than `source_files()`.

## Files

- Edited: `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py`, `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📋️program/🐍️.py`.
- Added: `🏢️nakagin/🧪️tests/❄️frozen/🐍️.py`, `🏢️nakagin/🧪️cases/❄️frozen/🔣️.json`.
- Scripts: `🧰️xl4/` (`xl4_common.py`, `xl4_py.sh`, `xl4_peak.py`, `xl4_frames.py`, `xl4_lifecycle.py`, `xl4_multi.py`, `xl4_cull.py`, `xl4_trace.py`, `xl4_budget.py`, `xl4_pipeline.py`, `xl4_anatomy.py`, `xl4_dict.py`, `xl4_reuse.py`, `xl4_free.py`, `xl4_proto.py`). Generated logs, frames and my ordering-cache entries are deleted.
