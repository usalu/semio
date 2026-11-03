"""📊 Attribute-level byte shares of an SVG frame."""
import re, sys
from collections import Counter
s = open(sys.argv[1], encoding="utf-8").read()
c = Counter()
for m in re.finditer(r'\s([\w:-]+)="([^"]*)"', s):
    c[m.group(1)] += len(m.group(0))
for m in re.finditer(r"<title>(.*?)</title>", s, re.S): c["<title>"] += len(m.group(0))
print(len(s), c.most_common(10))
d = "".join(m.group(1) for m in re.finditer(r' d="([^"]*)"', s))
print("d chars", len(d), "numbers", len(re.findall(r"-?\d+\.?\d*", d)))
