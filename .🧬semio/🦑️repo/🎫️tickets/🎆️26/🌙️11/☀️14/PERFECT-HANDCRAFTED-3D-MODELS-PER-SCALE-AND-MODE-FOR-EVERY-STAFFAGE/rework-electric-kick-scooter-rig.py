"""🧭 Steering limits of `vehicles/electric-kick-scooter`: the fork turns about the raked steering axis (its own u) by 45 degrees to either side."""
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
pattern = re.compile(r'("id": "fork",.*?"joint": \{\s*"type": "revolute",\s*"minDeg": 0,\s*"maxDeg": 0)(\s*\})', re.S)
match = pattern.search(text)
if match and "twistMinDeg" not in match.group(0):
    indent = re.search(r'\n(\s*)"maxDeg"', match.group(1)).group(1)
    text = text[: match.end(1)] + f',\n{indent}"twistMinDeg": -45,\n{indent}"twistMaxDeg": 45' + text[match.end(1):]
    path.write_text(text, encoding="utf-8")
    print("[DEBUG] steering limits written")
else:
    print("[DEBUG] nothing to do", bool(match))
