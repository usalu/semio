"""Normalised pass/fail set of bun test logs (without timings). usage: passfail.py <label>  -> writes orv-<label>-passfail.txt from orv-<label>-reader-N.log"""
import glob, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
label = sys.argv[1]
out = []
for p in sorted(glob.glob(f"{F}/🗑️generated/orv-{label}-reader-*.log")):
    n = re.search(r"reader-(\d+)\.log", p).group(1)
    for l in read(p).split("\n"):
        m = re.match(r"^\((pass|fail)\) (.*?)( \[[0-9.]+m?s\])?$", l)
        if m: out.append(f"{n}\t{m.group(1)}\t{m.group(2)}")
        m = re.match(r"^\s*(\d+) (pass|fail)$", l)
        if m: out.append(f"{n}\ttotal\t{m.group(2)}={m.group(1)}")
write(f"{F}/🗑️generated/orv-{label}-passfail.txt", "\n".join(sorted(out)) + "\n")
print(label, len(out), "tests;", sum(1 for o in out if "\tfail\t" in o), "fail")
