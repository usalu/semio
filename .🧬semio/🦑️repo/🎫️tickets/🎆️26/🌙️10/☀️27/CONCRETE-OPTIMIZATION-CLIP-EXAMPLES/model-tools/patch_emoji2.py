import re
from pathlib import Path
clip = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip")
p = clip / "🧮️model/🐍️.py"
s = p.read_text(encoding="utf-8")
s, n = re.subn(r'(def _pattern_path\(\):\n    """)🧭', r"\1🪀", s); assert n == 1
p.write_text(s, encoding="utf-8")
p = clip / "🧪️tests/🐍️.py"
s = p.read_text(encoding="utf-8")
old = "    def utilization(ratio, exponent):\n"
assert s.count(old) == 1
s = s.replace(old, old + '        """🎛️ Vectorized utilization factor with the ratio-one limit."""\n')
p.write_text(s, encoding="utf-8")
