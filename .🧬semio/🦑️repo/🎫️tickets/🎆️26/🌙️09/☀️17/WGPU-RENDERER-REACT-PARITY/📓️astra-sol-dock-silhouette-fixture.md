# Dock Silhouette Fixture Repair

Native145 (`renderer-native145-compact-ownership-ax/run.log`) failed `dock_stack_content_fills_full_bounds_through_one_silhouette_clip` because the test selected the first full-bounds solid in the authored draw list. Dock chrome now contains an earlier unclipped full-bounds border/background layer and the intended body fill later under the two-piece silhouette clip. The selection therefore observed `clip: None` even though `render_stack` had authored the clipped fill through `begin_silhouette_clip`.

The fixture now selects the full-bounds solid whose owning layer has a clip, then retains the exact two-piece assertion and the central cutout assertion. Production clip admission, overlap rejection, and Dock paint code are unchanged. Native rerun remains root-owned.
# Native148 physical clip diagnosis

Native148 showed that every produced layer had `clip: None`. The intended content fill existed at `[10,20,600,400]`, but `begin_silhouette_clip` rejected the clip before attaching it. The logical body begins at `y=48.8` and the top chip ends at the same `y=48.8`; conservative independent `floor`/`ceil` conversion assigned physical row 48 to both pieces, and the post-conversion overlap guard treated that rounding overlap as an authored overlap.

The clip owner now rejects true overlap in logical coordinates, then partitions the conservative physical scissors into a disjoint union and charges the resulting retained piece count. The existing strict overlapping-alpha refusal remains in place. The focused Dock law and the new lower-level fractional-adjacency law await the next root native gate.
