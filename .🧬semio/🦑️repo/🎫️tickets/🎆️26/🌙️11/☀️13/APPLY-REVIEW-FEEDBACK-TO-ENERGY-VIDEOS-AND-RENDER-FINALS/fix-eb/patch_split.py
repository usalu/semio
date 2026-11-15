import sys
from pathlib import Path

BROKEN = 'for part in text.replace("\n", " · ").split(" · "):'
FIXED = 'for part in text.replace("\\n", " · ").split(" · "):'

for name in sys.argv[1:]:
    path = Path(name)
    source = path.read_text(encoding="utf-8")
    count = source.count(BROKEN)
    path.write_text(source.replace(BROKEN, FIXED), encoding="utf-8", newline="\n")
    print(ascii(name)[-30:], count)
