# Root cause

`fill_count` (plus fill distribution weights and overlap budget) lived in `Puzzle3dWindowOptions`, which is keyed by window *instance* id so split top/perspective panes can keep independent grid/LOD/sun prefs.

`setFillCount` only `save_window`s the active pane. The other pane kept a stale `fill_count`, so:

1. `revealCutoffs["puzzle3d-fill"]` differed per pane
2. `puzzle3d_fixture_with_fill_display(applied_count=fill_count, …)` built different instance tails per pane
3. Live `worldRevealCutoffStore` reconciliation fought between the two committed cutoffs

# Fix

Keep fill count / object+vortex kind weights / overlap budget on the flat `Puzzle3dRuntime` only. `load_window` / `save_window` no longer touch them.
