from pathlib import Path
p = Path("C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-OPTIMIZATION-CLIP-EXAMPLES/exp-r2-savings-block.txt")
t = p.read_text(encoding="utf-8")
a = t.index("        label = _mn.VGroup(")
b = t.index("\n", a)
t = t[:a] + '        label = _mn.VGroup(_room.text(_room.parts(-saving, "", 1, german), (0, 0, 0), size=15, color=SHADED), _room.text("kWh/a", (0, 0, 0), size=14, color=SHADED)).arrange(DOWN, buff=.04)' + t[b:]
p.write_text(t, encoding="utf-8")
