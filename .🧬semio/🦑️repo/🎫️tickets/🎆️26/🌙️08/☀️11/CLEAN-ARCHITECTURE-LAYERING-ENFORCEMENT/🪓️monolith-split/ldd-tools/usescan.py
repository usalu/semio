"""🔍 Classifies every stdio import of the layout Rust crates as real use or dead import (read-only)."""
import re, subprocess, sys, os, collections
R = "/Users/ueli/Documents/semio"
B = "✏️s/🔌️plugins/📏️layout"
out = subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", f":(glob){B}/**/*.rs"], cwd=R, capture_output=True, text=True).stdout.split("\n")
rx = re.compile(r"semio_s_artifact_stdio_[a-z0-9_]+")
rows = []
for f in out:
    if not f: continue
    p = os.path.join(R, f)
    try: t = open(p, encoding="utf8").read()
    except Exception: continue
    if not rx.search(t): continue
    lines = t.split("\n")
    names = {}
    for i, l in enumerate(lines):
        if re.match(r"\s*(#\[cfg\(test\)\]\s*)?use\s", l) and "semio_s_artifact_stdio" in l:
            m = re.search(r"::\{([^}]*)\}\s*;?|::([A-Za-z0-9_]+)(?:\s+as\s+([A-Za-z0-9_]+))?\s*;", l)
            if m:
                if m.group(1) is not None:
                    for n in m.group(1).split(","):
                        n = n.strip()
                        if n: names[n.split(" as ")[-1].strip()] = i
                else:
                    names[(m.group(3) or m.group(2))] = i
            else:
                m2 = re.search(r"\{(.*)\}", l)
    body_by_name = {}
    for n, i in names.items():
        c = sum(len(re.findall(r"\b%s\b" % re.escape(n), l)) for j, l in enumerate(lines) if j != i)
        body_by_name[n] = c
    inline = [l.strip()[:120] for l in lines if "semio_s_artifact_stdio" in l and not re.match(r"\s*(#\[cfg\(test\)\]\s*)?use\s", l)]
    rows.append((f, body_by_name, len(inline)))
for f, d, inl in rows:
    short = f.replace(B + "/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/", "")
    dead = [n for n, c in d.items() if c == 0]
    live = {n: c for n, c in d.items() if c > 0}
    print(f"{short}\n   dead={dead}\n   live={live} inline_uses={inl}")
