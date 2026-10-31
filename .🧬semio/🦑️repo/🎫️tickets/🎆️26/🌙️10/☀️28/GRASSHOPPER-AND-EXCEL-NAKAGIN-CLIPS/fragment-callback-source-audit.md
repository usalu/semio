# Fragment Draw Callback Source Audit

Read-only source trace of the current amended boundary a8f21ceb08fdc68ea3dd4adbc59eb5773e48baea1e0c9f332d93187d8dae8f7e. No source/test/case change and no rendering or runtime validation was performed.

No concrete reachable view-only callback gap was found.

- Shared V `_fragment_item` lines209-238 calls current.draw(view, polygons=polygons) at220. Its own wrapper218 accepts override; nested fragments retain the contract.
- Shared V production `_assembly_items` callback496 accepts polygons and uses that argument for projection499 and world vertices497/501. All physical assembly consumers inherit this callback.
- Main occlusion test synthetic callbacks172,221,284,343,412,432,444,546 and686 accept polygons. Recolor wrappers584-585 and744-745 accept and forward polygons to their captured original callback. The latter reaches `_compose`754; the former reaches `_section_order`593-597. Both are safe for actual fragment/BSP entry.
- Fragment test callback definitions210,300 and327 also accept polygons.
- Production V ground callback1585 accepts only view, but its Item1589 is tier -1 and normal None; `_section_order`1019 excludes negative tiers from surface ordering, so it cannot enter `_fragment_item`. It needs no override.
- Bounded source search through the GH and Excel clip Python trees found no Item construction, draw replacement, `_assembly_items`, or `_section_order` call. Their scene consumers use shared V drawing methods. GH ghosts test58 has a different capsule-render spy signature (cap, plane, view, base, *, layout), not an Item callback.

Locations above are relative to `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py` and its main `🧪️tests/👁️occlusion/🐍️.py`, except where named. Runtime43/Visual20/native18 and eight-gallery acceptance remain pending and owned separately.
