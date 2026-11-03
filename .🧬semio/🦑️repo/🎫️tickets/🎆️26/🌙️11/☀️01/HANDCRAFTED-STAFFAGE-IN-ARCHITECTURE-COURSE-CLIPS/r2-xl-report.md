# r2-xl: why `test_native_lifecycle` of the excel presentation suite never finishes

Verdict: not staffage. The test stalls inside the Nakagin tower drawing (exact painter's ordering of the whole building), reached from the first `tower_axon` call of `Beat3_Program.present_program`. It is unverified, still: it cannot complete in any sensible time or memory with the current tower design.

## 1. Staffage contribution: none

- `test_native_lifecycle` (`📊️excel/🎬️clip/🧪️tests/🎭️presentation/🐍️.py:235`) calls `scene.construct()` of every discovered scene, in en and de, with `play`/`wait` replaced. `📋️program` `construct()` -> `present()` -> `present_program()` (`📋️program/🐍️.py:140`). The first statements are the workbook, then `self._axon = self.tower_axon(**TOWER_FIT, types=False)` at line 151.
- All stack dumps (faulthandler every 90 s, and a 0.25 s main-thread sampler) show only `_tower_axon -> _tower_drawing -> _tower_items / _compose -> _section_order` in `🏢️nakagin/🎨️visuals/🐍️.py`. `plan()`, `occupant_spots`, the staffage loader, parser cache and exporter are never reached before the stall (the first `plan()` call is in a later step). Staffage RSS is irrelevant: the parser LRU is bounded (256 frames) and the occupant cast is 75 frames.

## 2. Profile (harness `🧰️xl/xl_lifecycle.py`: RSS sampler thread, kill above 6 GB, stack sampler, timing wrappers)

Scene `Beat3_Program`, language en, `📋️program` only:

| Phase | Wall time | RSS |
|---|---|---|
| import and workbook | about 10 s | 130 MB |
| `_tower_items`: 140 capsules, each 2368 faces, `_opaque_visible` (18 distinct signatures, rest cache hits) | 240 s | about 1.0 GB |
| `_compose` -> `_section_order` (exact occlusion order over about 165 000 retained faces) | still running after 40 more minutes | 1.4 GB, then +0.5 to 1.8 MB/s, 2.7 GB at 40 min |

The first drawing never finished in any of 7 runs (longest 2426 s, 2.66 GB). The earlier 25 min / 3.4 GB kill is the same state. Nothing leaked; this is the first build of a single drawing.

Why it cannot finish in the test, even if the first build did:

- Each of the 140 capsules is 2368 faces; about 1180 are retained, so the tower is about 165 000 `VMobject` shapes, roughly 1.5 GB per finished drawing (`_paint_ordered`, `_compose`).
- `NakaginDrawing.tower_axon` keys `_TOWERS` (8 entries) on `tint`/`lit`/`where`. `📋️program` calls it with a different tint for every paint state and, in `coloured()` (line 202), once per capsule-type colour, and stores each in an unbounded scene dict `_coloured`. The ordering (`_OCCLUSION_ORDERS`) and visibility caches are colour-free, so later builds skip the sort, but every variant still creates 165 000 shapes, so memory grows by about 1.5 GB per variant. The 6 GB limit is crossed after about 3 variants, before the first `plan()` is reached.
- The test then repeats this for en and de, and the `📍️references` scene (`TOWER_FIT` at its line 28) does the same.

The hog is therefore the design of the clip (one full-building drawing per colour), not a leak: a render of `Beat3_Program` has the same cost. `r2-rx` already killed a real render at 35 min / 2.4 GB for this reason.

## 3. Fix applied (algorithmic, behaviour-preserving)

`🏢️nakagin/🎨️visuals/🐍️.py` `_section_order`, overlap sweep: the active set was a Python list rebuilt with a comprehension for every one of the 165 000 items and then converted with `np.asarray`, so the sweep cost O(n x active) in interpreted code. It now keeps preallocated NumPy arrays (index, x upper, y lower, y upper), compacts them every 256 items and selects candidates with one vectorised mask. Insertion order of candidates is unchanged, so the edge sets are identical.

Result: the hot line moved from the list comprehension to the pair loop (`_section_order` 1150/1151 and `np.linalg.norm` of the plane comparison); first-build wall time stayed in the same range because the remaining cost is the number of overlapping candidate pairs (about quadratic in faces) and the Python per-pair loop, which is inherent to the exact algorithm. Verification: `🏢️nakagin/🧪️tests/👁️occlusion/🐍️.py`: `Ran 28 tests in 255 s, OK`.

## 4. Still open (needs an owner decision, not a clear bug)

1. `📋️program` and `📍️references` should not draw the 140-capsule building once per tint. Options: draw one tower and recolour capsule groups in place (fill update) instead of `tower_axon(tint=...)`; build the colour sources from one capsule per type (`_morph.silhouette` already hulls them) at their slot place; or add a lightweight tower mode (fewer segments, `lod` facets merged) for the excel clips. Any of these also bounds memory under 6 GB.
2. Until then `test_native_lifecycle` for `📋️program` and `📍️references` cannot be completed; the other 19 tests of the suite pass (see `r2-rx-report.md`). A test-only stub of `_section_order`/`_opaque_visible` would not help, because the 165 000 shapes per variant remain.
3. `NakaginDrawing._TOWERS` holds up to 8 whole-building drawings (about 12 GB if full). Lower it, or key it on geometry only and recolour on clone, together with item 1.

## Files

- Edited: `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py` (`_section_order` sweep).
- Scripts: `🧰️xl/xl_lifecycle.py` (usage: `m9`-style runner `🧰️rx/rx_py.sh 🧰️xl/xl_lifecycle.py <clip folder> <language>`). Generated logs deleted.
- Note: my processes were killed externally twice (other agents); each of the 7 runs was restarted.
