# Previously

GoldenLayout window borders were still missing on the bottom/right edges and dashed borders were not desired.

# Plan

Render GoldenLayout window borders as an inset 1px stroke so all edges render reliably, revert window borders to solid style, and update dev docs to match the final mechanism.

# Changes

GoldenLayout stacks now render a continuous solid outline via inset box-shadow to avoid edge clipping on bottom/right. Canvas windows reverted from dashed to solid `border-window`. Documentation updated.
