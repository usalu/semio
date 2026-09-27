#!/usr/bin/env python3
"""🧭️ S19: per norm family, compare the editor's `examples()` roster with the ids `setActiveExample` resolves."""
import pathlib, re
root = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
for fam in sorted(p for p in root.iterdir() if p.is_dir()):
    editor = fam / "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"
    handler = fam / "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"
    ex_text = editor.read_text() if editor.exists() else ""
    m = re.search(r"fn examples\(\)[^{]*\{(.*?)\n    \}", ex_text, re.S)
    roster = re.findall(r"crate::(?:[a-z0-9_]+::)*?([a-z0-9_]+)::source\(\)", m.group(1)) if m else []
    h = handler.read_text() if handler.exists() else ""
    handled = re.findall(r"crate::(?:[a-z0-9_]+::)*?([a-z0-9_]+)::ID", h)
    missing = [r for r in roster if r not in handled]
    print(f"{fam.name}: roster={len(roster)} handled={len(handled)} missing={missing}{'' if handler.exists() else ' (no handler file)'}")
