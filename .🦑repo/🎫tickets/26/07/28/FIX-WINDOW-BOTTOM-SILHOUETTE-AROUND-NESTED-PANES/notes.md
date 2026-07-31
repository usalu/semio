# Fix Window Bottom Silhouette Around Nested Panes

## Bug

Mode-dock / window silhouettes queried **all** descendant `[data-window-silhouette-chip]` nodes. Bottom-anchored {@link Pane} chrome (projection) stamps `data-dock="bottom"` on its own nested `[data-window-silhouette]` stack, so the enclosing window outline wrongly notched around those panes.

Intended: projection (and other panes) overlay the window like window options; the window bottom stays a rectangular silhouette edge.

## Fix

`measureWindowSilhouetteMetrics` only counts chips / gaps / caps whose nearest `[data-window-silhouette]` ancestor is the measured stack (`windowSilhouetteOwnsElement`). Nested pane/panel silhouettes keep their own outline independently.

## Test

`measureWindowSilhouetteMetrics ignores nested pane silhouette chips so the window bottom stays rectangular`
