# Dashboard Navigation Fix

## Root cause
Mouse reporting (`1002h`) delivered Move events; `Tui::dispatch` focused whatever cell was under the cursor. Moving over the empty **Terminal** pane stole focus from the catalog **Table**, so ↑/↓/Enter never reached the table (they became terminal passthrough no-ops).

## Fixes
1. **Engine**: focus changes only on `MouseKind::Down` (test: `engine_mouse_move_does_not_steal_focus`).
2. **Dashboard**: table-first input — arrows / `hjkl` / Enter always route to the focused window’s catalog table unless terminal-input mode is on (`Ctrl-Space` then `t` after a launch, Esc leaves).
3. **Tab** cycles `dev`/`build` tables only (never empty terminals).
4. Full repaint after input; window chrome `focused` border synced.

## How to use
1. Rebuild/run: `bun …/📜️script.ts run` (or relaunch the VS Code dashboard entry).
2. `j`/`k` or arrows move; `h`/`l` collapse/expand; **Enter** on a leaf launches; **Tab** switches windows; `x` stops; `q` quits.
