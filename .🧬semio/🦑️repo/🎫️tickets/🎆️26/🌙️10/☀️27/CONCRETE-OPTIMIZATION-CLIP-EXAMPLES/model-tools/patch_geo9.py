from pathlib import Path
clip = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
p = clip / "🧮️model/🐍️.py"
s = p.read_text(encoding="utf-8")
old = 'sky = 1 - daylight["sky_angle_loss"] * overhang / (overhang + daylight["overhang_half_depth"])'
assert old in s
s = s.replace(old, 'sky = 1 - daylight["sky_angle_loss"] * overhang ** 2 / (overhang ** 2 + daylight["overhang_half_depth"] ** 2)')
p.write_text(s, encoding="utf-8", newline="\n")
p = clip / "🧪️tests/🐍️.py"
s = p.read_text(encoding="utf-8")
old = 'angle = daylight["sky_angle"] * (1 - daylight["sky_angle_loss"] * depth / (depth + daylight["overhang_half_depth"]))'
assert old in s
s = s.replace(old, 'angle = daylight["sky_angle"] * (1 - daylight["sky_angle_loss"] * depth ** 2 / (depth ** 2 + daylight["overhang_half_depth"] ** 2))')
p.write_text(s, encoding="utf-8", newline="\n")
