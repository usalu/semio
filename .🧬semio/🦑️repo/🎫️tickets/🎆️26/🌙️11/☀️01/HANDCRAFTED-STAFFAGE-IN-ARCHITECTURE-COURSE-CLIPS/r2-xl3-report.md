# r2-xl3: padding, run merging, persisted ordering cache for the Nakagin tower

Verdict: the ordering cache is done and verified. The padding is already scale-consistent and cannot be tightened exactly. Run merging gains almost nothing. `test_native_lifecycle` for `📋️program` still does not meet 3 GB (killed at the 5 GB guard).

## 1. Padding (no absolute-unit bug)

- `coverage_padding = _raster_footprint(scale, stroke)` = (half stroke + 2 pixels) / scale (`🏢️nakagin/🎨️visuals/🐍️.py`, `_raster_footprint`, `_tower_template`). It is physical, expressed in projected model units of the scale-1 template. The only absolute epsilons are `1e-10` / `1e-7` in `_convex_difference`, `_single_cover`, `_opaque_visible` (orders of magnitude below any face), not scale related.
- At `TOWER_FIT` (scale 0.0752, 854x480) one pixel spans 0.197 model units, so the padding is 0.227 model units against capsule faces of 0.1 to 0.5. A 4 storey probe (`🧰️xl3/xl3_count.py`): scale 1 keeps 8041 faces (padding 0.017), scale 0.0752 keeps 17 642 (padding 0.227). The 3x retention is therefore physical, not a bug: a face may only be dropped when its padded footprint is covered.
- Tried the tightest provable alternative: replacing the padded bounding box by the convex hull of the polygon grown by the padding (`_dilated`). It removed 6 percent of shapes (29 768 vs 31 736, 2 storeys) but the quality-l frame comparison (`🧰️xl3/xl3_frames.py`, 854x480, real Cairo) gave 37 differing pixels, max delta 48, out of 5004 covered pixels. Not identical, so it was reverted completely (no `_dilated` left). The bbox footprint stays; two antialias pixels cannot be reduced either without the same risk.

## 2. Run merging (measured, not applied)

4 storeys at fit scale, after the exact ordering (`🧰️xl3/xl3_runs.py`): 75 658 shapes from 17 413 retained faces (about 4.3 shapes per face: 2.6 fragments plus 30 610 boundary-line children).

| merge rule | resulting shapes |
|---|---|
| none | 75 658 |
| consecutive identical paint (fill, stroke, width, children flag) | 73 138 (3 percent) |
| identical paint and pixel-disjoint padded boxes (the only variant that keeps Cairo antialiasing identical) | 75 219 |

Adjacent faces in painter order alternate in tone, so runs are short, and merged subpaths in one Cairo path would change seam antialiasing and break the stroke/fill interleave. A merge would save 3 percent and cost recolour bookkeeping, so it was not implemented.

## 3. Where the memory goes (`🧰️xl3/xl3_rss.py`)

A shape costs about 3.2 KB python overhead plus 2.1 KB points, 0.5 KB `world_vertices`, 0.3 KB `world_facets`: about 7.5 KB. 75 658 shapes: template plus clone 923 MB, clone alone 693 MB. The full tower has about 693 481 order rows (634 533 fragments, see below), so one live drawing is more than 5 GB and `template.copy()` doubles it. Lifecycle also calls `tower_axon` again for each paint state (`📋️program/🐍️.py:303`, `:308` builds a whole clone only to read one capsule centre). The 3 GB target needs a structural change (fewer shapes per face, lean shape class, or capsule-only paint states), not a culling tweak.

## 4. Persisted ordering cache (done)

`🏢️nakagin/🎨️visuals/🐍️.py`: `_section_order` checks memory, then disk, then computes and stores. Functions `_order_directory`, `_source_digest`, `_order_path`, `_order_rows`, `_order_recipes`, `_store_order`, `_load_order`.

- Directory: `$SEMIO_TEACHING_CACHE_ROOT` or `.🧬semio/🎓️teaching/⚡️cache`, subfolder `nakagin-orders` (same root rule as the staffage cache).
- File name: blake2b of the existing geometry/view digest (all item vertices, normals, tiers, view basis and towards, which also covers padding because it changes the retained items) plus the digest of every file of `source_files()` (this module and the fragment module). The file records key and source digest and is accepted only if both match, so a stale entry cannot be read; corrupt or foreign files rebuild.
- Writes: temp file in the same folder then `os.replace`; failure removes the temp file and leaves no entry.
- Test: `🏢️nakagin/🧪️tests/💾️order-cache/🐍️.py` with JSON case `🏢️nakagin/🧪️cases/💾️order-cache/🔣️.json` (three interpenetrating faces that fragment): cold build persists, warm build never calls `ordered_fragments`, changed geometry/view creates new entries, stale source digest / foreign key / corrupt JSON rebuild, failed `os.replace` leaves nothing. 4 tests OK.
- Measured: the full tower order (693 481 rows, 203 MB JSON) took about 3500 s to compute; loading it takes 14 s (`🧰️xl3/xl3_load.py`). Any edit to the visuals or fragment source invalidates it by design.

## 5. Verification

| check | result |
|---|---|
| 28 occlusion tests plus 4 order-cache tests | `Ran 32 tests in 245 s, OK` |
| `🏢️nakagin/🧪️tests/🎨️visuals` | `Ran 22 tests in 5718 s, OK` |
| `test_native_lifecycle` `📋️program` en, 5 GB guard (`🧰️xl2/xl2_lifecycle.py`) | killed at 4665 s, 5.1 GB: tower items 245 s, ordering finished about 3500 s (persisted), then shape creation passed 5 GB. Before (r2-xl2): killed 3240 s at 5.2 GB. |

## Open

- Memory target not met: a single full-tower drawing is above 5 GB at about 7.5 KB per shape and 4.3 shapes per face. Options for the owner: a lean points-only shape class for the template, dropping `world_*` attributes on the cached tower, avoiding `template.copy()` for the one-time use, and replacing `📋️program` line 308 by a capsule-centre lookup.
- With a warm order cache (and unchanged sources) the 50 minute ordering is skipped; rerun once after the final edits to populate it.

## Files

- Edited: `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py` (ordering cache only).
- Added: `🏢️nakagin/🧪️tests/💾️order-cache/🐍️.py`, `🏢️nakagin/🧪️cases/💾️order-cache/🔣️.json`.
- Scripts: `🧰️xl3/xl3_count.py`, `xl3_runs.py`, `xl3_rss.py`, `xl3_frames.py`, `xl3_mem.py`, `xl3_load.py`. Generated logs and the order cache deleted.
