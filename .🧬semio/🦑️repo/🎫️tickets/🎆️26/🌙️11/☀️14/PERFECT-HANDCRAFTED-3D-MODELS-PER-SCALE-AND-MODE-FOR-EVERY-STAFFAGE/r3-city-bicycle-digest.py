"""🧾 Prints the rule lines of a volumes-check output without the per-item information fields, numbers cut to four decimals.

Usage: python3 r3-city-bicycle-digest.py <check output txt>"""
import re
import sys

for line in open(sys.argv[1], encoding="utf8").read().split("\n"):
    if line.startswith(("ok", "FAIL", "vehicles")):
        line = re.sub(r" info:(exposed|exposure)\w*:[\w-]+=[\d.e-]+", "", line)
        line = re.sub(r" info:extentDifferenceM:[\w-]+:\w=[\d.e-]+", "", line)
        line = re.sub(r" slenderRatio:[\w-]+=[\d.]+>=20", "", line)
        line = re.sub(r"(\d\.\d{4})\d+", r"\1", line)
        print(line[:2600])
