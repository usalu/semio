# Native Batch Cache Retention Review

## Finding

Read-only review finds no project-owned public cleanup or memory-budget interface for rich Nakagin drawing templates. The batch currently calls `gc.collect()` after each runpy pair, but the imported renderer retains up to eight complete building template trees in private `_TOWERS`. `tower_axon` clones each retained `NakaginDrawing3D` using `template.copy()`. The cached template remains strongly reachable independently of the returned scene or clone; collecting a finished Scene cannot release it. A count limit of eight is not a measured byte budget. Contracts run first and can populate these templates before any native pair starts. No heavy process or memory experiment was launched for this review.

## Exact existing cache controls

- `_TOWERS`: private dict, insertion-order eviction when a new style arrives at eight entries. Keys include view, selected/tinted/included slots, types, capital/podium/core/ground flags, towers, storeys, tessellation and base colour. There is no public release/clear/budget method.
- `_TEXTS`: private unbounded text-template dict, copied when used; no public cleanup. It is useful for warm fonts/text and is not the same category as a whole building tree.
- `_OCCLUSION_ORDERS`: private 24-entry OrderedDict of integer index tuples, not rich drawings.
- `_OPAQUE_VISIBILITY`: private 64-entry OrderedDict of integer index tuples.
- `capsule_mesh` (512), `_capsule_parts` (256), `_core_parts` (64), `_capital_parts` (32), `_podium_parts` (1), `_bridge_parts` (1): functools LRU functions retaining geometry/mesh parts. The standard wrapper exposes `.cache_clear()`, but private per-function clearing is not an application cache lifecycle interface and would also discard useful pure geometry.
- `_blend`: 8,192-entry colour-value LRU.

A scoped search across architecture Python sources found only test calls to individual `.cache_clear()` methods; it found no public rich-drawing release/budget implementation. Manim disk cache settings do not bound these Python template references.

## Safe harness decision before native GO

Do not mutate `_TOWERS` or selectively reload modules from a ticket harness. With the present public interfaces, the existing fresh-process `validate-scenes.py` per scene/language is the established isolation boundary. A bounded compromise is one subprocess per scene containing its two language replays, with the full source/runtime/config baseline enforced by the coordinator before and after each child. Run presentation contracts in a separate child. Geometry/order/font memory caches remain warm within that child; on-disk text/font outputs remain reusable across children. This does not pretend to preserve all in-memory geometry across process exit. The all-scenes single-process batch has no current proof that its retained rich trees stay below admission, so I do not recommend GO for that arrangement merely because `gc.collect()` runs.

If retaining pure geometry/order/fonts across the entire batch is required, a project-owned explicit rich-template lifecycle/budget API is needed first, with portable cache-retention/release tests; that is a separate production change and was not implemented during this read-only review.
