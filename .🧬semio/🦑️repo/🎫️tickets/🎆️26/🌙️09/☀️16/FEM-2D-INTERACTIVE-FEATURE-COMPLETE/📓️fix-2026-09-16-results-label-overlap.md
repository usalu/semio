# Results Window Reaction Label Overlap

## Problem

Static results drew every support reaction at the same screen offset `(sx + 8, sy + 14)` per node. Nodes with restrained `Tx` and `Ty` (and `Rz`) produced illegible stacked text; nearby supports with the same anchor offset collided as well.

## Approach

In `📊️results/🦀️.rs`:

1. Sort reactions by `node_id` then `Dof::index()` so stacking order is stable.
2. Per node, increment a stack index so the base Y steps by `REACTION_LABEL_LINE` (13 px).
3. Estimate each label's bounding box and search a small grid of nudges (±12 px horizontally, +13 px per row) until no intersection with labels already placed.

## Tests

- `reaction_labels_at_one_node_use_distinct_positions` — integration law on the demo snapshot.
- `place_reaction_label_stacks_then_avoids_overlap` — unit law on the placement helper.
