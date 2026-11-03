import re
import subprocess
import sys

ROOT = r"C:\git\semio"
E = "🎓️teaching/🏛️architecture/⚡️energy"
pat = re.compile(r"Text\(|body_text\(|_label\(|legend\(|math_label\(|math_text\(|math_readout\(|_readout\(|_m\(|\\text|title=|label=|watt_anchor\(|tag\w* =|lbl\w* =|MathTex|Tex\(")
files = subprocess.run(["git", "-c", "core.quotepath=false", "diff", "--name-only"], cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout.split("\n")
for f in files:
    if "energy/" not in f or not f.endswith(".py") or "tests" in f or "authoring" in f or "clip/" not in f:
        continue
    d = subprocess.run(["git", "-c", "core.quotepath=false", "diff", "-U0", "--", f], cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout
    ln = 0
    print("###", f.split("energy/")[1])
    for l in d.split("\n"):
        m = re.match(r"@@ -\S+ \+(\d+)", l)
        if m:
            ln = int(m.group(1))
            continue
        if l.startswith("+") and not l.startswith("+++"):
            if pat.search(l) and '"' in l:
                print(f"{ln}: {l[1:].strip()[:220]}")
            ln += 1
