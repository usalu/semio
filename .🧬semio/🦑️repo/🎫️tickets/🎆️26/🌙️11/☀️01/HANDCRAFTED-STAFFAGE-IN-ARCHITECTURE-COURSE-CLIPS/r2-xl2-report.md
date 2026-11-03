# r2-xl2: build the Nakagin tower once per geometry, recolour in place

## Change (`🏢️nakagin/🎨️visuals/🐍️.py`, excel `📋️program`)

- `_TOWERS` is keyed by geometry only (view, `lit is None`, included slots, capital/podium/core/ground, towers, storeys, segments) and bounded by `_TOWER_CACHE = 2` (LRU: lit and unlit). Tint, types, base and the lit selection left the key.
- The template is always drawn in neutral paint. `tower_axon` clones it and `_tint_capsules` recolours the capsule groups in place (`_slot_look` is the single colour rule, also used by `_tower_items`; `_recolour` rewrites fill/stroke from the `paint` tuple `(material, tone, hairline, cover)` that `_assembly_items` and `_paint_ordered` now attach to each shape; split fragments keep stroke width 0 and their boundary line child is recoloured or removed when a capsule is dimmed).
- New `tower_capsules(keys, color, ...)`: copies only the requested capsules out of the cached template and recolours them (colour sources from capsules, not from a tinted whole building). `📋️program` `silhouettes` uses it; `coloured()` and its unbounded `_coloured` dict are gone. `📍️references` needs no change (it only calls `tower_axon(lit=, tint=)`).

## Verification

- Vector equivalence (`🧰️xl2/xl2_equiv.py`, tower 0, storeys 0-1, 10 418 shapes): fresh `_tower_drawing` with the colour arguments vs template clone recoloured, comparing points, fill rgba, stroke rgba and stroke width of every shape. tint OK, types OK, lit OK, lit+tint OK, lit+tint+types OK, plain OK, `tower_capsules` vs fresh flat tint OK (0 failures, 361 s). The first attempt caught two real bugs (fragment stroke widths, child line fill), fixed.
- Occlusion suite `🏢️nakagin/🧪️tests/👁️occlusion/🐍️.py`: `Ran 28 tests in 245 s, OK`.
- `🏢️nakagin/🧪️tests/🎨️visuals/🐍️.py` was started but did not finish inside 10 min (builds whole towers); not verified.
- Not done: the `Beat3_Program` frame digest comparison. The "before" frame needs the first whole-building drawing, which never completes (below), so the before/after pixel check was replaced by the shape-level comparison above, which is stricter (exact vector data).

## Lifecycle result (not met)

| | before (r2-xl) | after |
|---|---|---|
| `test_native_lifecycle` `📋️program` en | never finished; killed at 6 GB / 2426 s in 7 runs | not finished: first template build ordered after about 50 min, then RSS rose from 2.7 GB to 5.2 GB during shape creation and my 5 GB guard killed it at 3240 s |
| peak | 2.66 GB at 40 min, more later | 5.2 GB (killed) |

Reason: at the real `TOWER_FIT` scale the coverage padding is large, so about 165 000 items survive (the same build at scale 1 keeps 58 000 and finishes the exact ordering in 11 min, measured with the progress hook). One finished whole-tower drawing is therefore more than 5 GB, because a manim `VMobject` costs about 25 KB. Building once instead of per colour removes the multiplication (before: one build per tint, per capsule-type colour, per language), but a single template alone cannot meet 3 GB or finish in a test.

## Open (needs an owner decision)

1. A lighter shape type or a points-only template for the 165 000 faces (the per-shape manim cost dominates memory), or fewer retained faces (for example a coarser `coverage_padding`, which changes pixels).
2. A persisted, versioned cache of `_OCCLUSION_ORDERS` recipes would make the 50 min ordering a one-time cost; not added, because stale entries would be silent.
3. Scripts: `🧰️xl2/xl2_equiv.py`, `xl2_progress.py`, `xl2_lifecycle.py` (5 GB guard), `xl2_debug.py`. Generated output deleted.
