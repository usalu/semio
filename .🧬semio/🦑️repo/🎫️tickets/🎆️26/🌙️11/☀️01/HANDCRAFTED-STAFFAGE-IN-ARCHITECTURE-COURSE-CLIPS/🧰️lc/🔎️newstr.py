import re
import subprocess

ROOT = r"C:\git\semio"
files = subprocess.run(["git", "-c", "core.quotepath=false", "diff", "--name-only"], cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout.split("\n")
lit = re.compile(r'(?<![\w])(?:[rf]{0,2})"((?:[^"\\]|\\.)+)"')
for f in files:
    if "energy/" not in f or not f.endswith(".py") or "tests" in f or "authoring" in f or "clip/" not in f:
        continue
    d = subprocess.run(["git", "-c", "core.quotepath=false", "diff", "-U0", "--", f], cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout
    old, new, ln = set(), [], 0
    for l in d.split("\n"):
        m = re.match(r"@@ -\S+ \+(\d+)", l)
        if m:
            ln = int(m.group(1))
            continue
        if l.startswith("-") and not l.startswith("---"):
            old.update(x for x in lit.findall(l))
        elif l.startswith("+") and not l.startswith("+++"):
            body = l[1:].strip()
            if not body.startswith(('"', "#", "'", "(\"", "f\"")) and '"""' not in body:
                for s in lit.findall(body):
                    new.append((ln, s, body))
            ln += 1
    out = [(n, s) for n, s, b in new if s not in old and re.search(r"[A-Za-zÄÖÜäöüß]{3}", s) and not re.fullmatch(r"[a-z0-9_\-:.\[\]]+", s) and "heating-" not in s and "cooling-" not in s and "shared-" not in s and "ventilation-" not in s]
    if out:
        print("###", f.split("energy/")[1])
        for n, s in out:
            print(f"  {n}: {s[:110]}")
