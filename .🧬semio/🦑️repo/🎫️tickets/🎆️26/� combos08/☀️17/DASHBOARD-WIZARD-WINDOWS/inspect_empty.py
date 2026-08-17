#!/usr/bin/env python3
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
tui = next(root.glob("**/🖱️ui/⌨️tui/🦀️component.rs"))
win = next(root.glob("**/Window/⌨️component.rs"))
wiz = next(root.glob("**/Wizard/⌨️component.rs"))
dash = next(p for p in root.glob("**/terminal-dashboard/🦀️component.rs") if "daemon" not in str(p))
# also try emoji path
cands = list(root.glob("**/🎛️terminal-dashboard/🦀️component.rs"))
if cands:
    dash = cands[0]

print("TUI", tui)
print("WIN", win)
print("WIZ", wiz)
print("DASH", dash)

text = tui.read_text()
start = text.find("    pub fn solve(scene: &mut Scene, viewport: Rect)")
print("==== SOLVE ====")
print(text[start : start + 2800])
print("==== CONSTRAINT ====")
start = text.find("pub struct Constraint")
print(text[start : start + 900])
print("==== shell window create ====")
start = text.find("for measure in solve_window_layout")
print(text[start : start + 600])
print("==== WIN paint ====")
print(win.read_text()[:2500])
print("==== WIZ paint ====")
print(wiz.read_text()[:2000])
