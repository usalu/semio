from pathlib import Path

base = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugin")
puzzle = [x for x in base.iterdir() if "puzzle" in x.name][0]
d3 = [x for x in (puzzle / "🎛️app").iterdir() if "3d" in x.name][0]
p = d3 / "🔨️module" / "🖱️ui" / "⚡️implementation" / "🦀️rust" / "📦️lib.rs"
t = p.read_text()

old = """        let fixture = puzzle3d_fixture_with_fill_display_memo(
            self.render_fixture(&doc.projection.0),
            &*self.precompute.borrow(),
            runtime_for_window.fill_count,
            self.precompute.borrow_mut().fill_available_count(),
            &self.fill_display_memo,
        );"""

new = """        let fill_available = self.precompute.borrow().fill_available_count();
        let fixture = puzzle3d_fixture_with_fill_display_memo(
            self.render_fixture(&doc.projection.0),
            &*self.precompute.borrow(),
            runtime_for_window.fill_count,
            fill_available,
            &self.fill_display_memo,
        );"""

if old not in t:
    raise SystemExit("pattern not found")
p.write_text(t.replace(old, new, 1))
print("fixed render fill_available overlap")

# scan for other same-line / same-call overlapping patterns: borrow() then borrow_mut on precompute in nearby lines
lines = p.read_text().splitlines()
for i, line in enumerate(lines):
    if "precompute.borrow()" in line and "precompute.borrow_mut()" in line:
        print(f"same-line {i+1}:{line[:160]}")
# window of 8 lines with both
for i in range(len(lines)):
    window = "\n".join(lines[i : i + 8])
    if "precompute.borrow()" in window and "precompute.borrow_mut()" in window:
        # only report if they're in a call that might overlap (not sequential statements that drop)
        if "puzzle3d_fixture_with_fill_display" in window or ("," in window and window.count("precompute") >= 2):
            print(f"window@{i+1}:")
            for j in range(i, min(i + 8, len(lines))):
                if "precompute" in lines[j]:
                    print(f"  {j+1}:{lines[j][:140]}")
