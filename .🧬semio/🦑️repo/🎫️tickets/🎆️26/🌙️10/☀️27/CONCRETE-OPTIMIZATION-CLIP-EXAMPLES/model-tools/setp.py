import re, sys
from pathlib import Path
p = Path("C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/🧮️model/🔣️.json")
s = p.read_text(encoding="utf-8")
for arg in sys.argv[1:]:
    key, value = arg.split("=")
    s, n = re.subn(r'("%s": \{"value": )[-0-9.]+' % key, r"\g<1>" + value, s, count=1)
    assert n == 1, key
p.write_text(s, encoding="utf-8")
