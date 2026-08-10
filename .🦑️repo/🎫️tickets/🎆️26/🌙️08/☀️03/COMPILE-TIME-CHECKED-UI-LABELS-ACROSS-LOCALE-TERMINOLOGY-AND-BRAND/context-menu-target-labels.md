# Context Menu Target Labels

## Requirement

Every context menu title must identify the element under the pointer. For example, a right-click on a vortex must open a menu titled `Vortex Menu`, while a right-click on the 3D background must use `Scene Menu`.

## Finding

The React renderer currently stores only coordinates and resolved rows after a surface menu request. Each host supplies one fixed surface title (`Scene actions`, `Board actions`, and similar), even though the request already carries the freshly hit-tested target domain in `surface.hits[0].domain`.

The shared `openSurfaceContextMenu` funnel is the correct single source of truth. It can resolve a typed translation key from the most-specific hit domain, then fall back to the surface kind when the click is on the background. Returning that key with the rows prevents individual hosts from drifting and preserves locale-aware labels.

## Covered Target Domains

`architecture`, `attraction`, `block`, `edge`, `entry`, `feature`, `group`, `handle`, `layer`, `node`, `object`, `part`, `path`, `pixel`, `position`, `reference`, `route`, `row`, `slider`, and `vortex`.

## Covered Background Surfaces

Canvas, scene, node graph, editor, table, paint, map, board, ink, history, block list, diff, event feed, file system, and workspace fallback menus.
