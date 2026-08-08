#!/usr/bin/env python3
from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
app3d = next(root.glob("**/🌀️procedural/🎛️apps/*/🦀️component.rs"))
# Prefer the 3d app root (not wasm/config nested) — list candidates
candidates = sorted(root.glob("**/🌀️procedural/🎛️apps/*/🦀️component.rs"))
for c in candidates:
    print("CAND", c)
app3d = [c for c in candidates if "3d" in c.as_posix()][0]
print("APP3D", app3d)
text = app3d.read_text()
for needle in ["fn handle(", "impl DocumentApp", "type Snapshot", "fn initial_snapshot", "DraftView", "EngineHandles"]:
    i = text.find(needle)
    print("\n====", needle, i, "====")
    if i >= 0:
        print(text[i : i + 800])

# lowpoly handle
for p in root.glob("**/💠️lowpoly/🎛️apps/**/🦀️component.rs"):
    t = p.read_text(errors="ignore")
    if "fn handle(" in t and "DraftView" in t and "DocumentApp for" in t:
        print("\nLOWPOLY", p)
        i = t.find("fn handle(")
        print(t[i : i + 700])
        break

gumball = next(root.glob("**/🌀️procedural/**/🧭️gumball/🦀️component.rs"))
t = gumball.read_text()
print("\nGUMBALL len", len(t))
for needle in ["fn apply", "commit_fixture", "host_operations", "gumball_xform", "TranslateSelection"]:
    i = t.find(needle)
    print(needle, i)
    if i >= 0:
        print(t[max(0, i - 80) : i + 500])
        print("---")
