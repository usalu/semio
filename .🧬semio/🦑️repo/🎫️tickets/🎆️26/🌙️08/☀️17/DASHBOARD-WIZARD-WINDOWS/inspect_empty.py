#!/usr/bin/env python3
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
tui = next(root.glob("**/🖱️ui/⌨️tui/🦀️component.rs"))
# Window and Wizard under elements
ui = next(p for p in root.glob("**/🖱️ui") if p.is_dir() and "modules" in str(p))
elems = next(ui.glob("*elements*"))
print("elems", elems)
for child in sorted(elems.iterdir()):
    if "Window" in child.name or "Wizard" in child.name:
        print("elem", child.name, list(child.rglob("*.rs")))

win = next(p for p in elems.rglob("*.rs") if "Window" in str(p))
wiz = next(p for p in elems.rglob("*.rs") if "Wizard" in str(p))
dash = next(root.glob("**/🎛️terminal-dashboard/🦀️component.rs"))
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
print("==== WIN paint ====")
print(win.read_text()[:3000])
print("==== attach ====")
d = dash.read_text()
i = d.find("fn attach_wizard")
print(d[i:i+900])
i = d.find("fn add_wizard_window")
print(d[i:i+800])
