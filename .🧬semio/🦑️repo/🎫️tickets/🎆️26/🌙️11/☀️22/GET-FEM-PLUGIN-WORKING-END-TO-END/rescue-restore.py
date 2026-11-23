import os, sys, importlib.util, subprocess
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("rh", os.path.join(here, "rescue-hunks.py")); rh = importlib.util.module_from_spec(spec); spec.loader.exec_module(rh)
spec2 = importlib.util.spec_from_file_location("r2", os.path.join(here, "rescue-resolve2.py"))
B = os.path.join(here, "git-semio-rescue-backup"); os.makedirs(B, exist_ok=True)
rows = []
for i, (d, p, hs) in sorted(rh.load().items()):
    restored = 0
    for h in hs:
        kd = rh.kind(h)
        if kd in ("ws", "R"): continue
        if [x for x in h["new"] if x.strip() and x not in h["base"] and x not in h["old"]]: restored += 1
    if not restored: rows.append((i, "old-resolved", p)); continue
    fn = os.path.join(r"C:\semio", p)
    orig = open(d + r"\new", "rb").read()
    open(fn, "wb").write(orig)
    base = open(d + r"\base", "rb").read().replace(b"\r\n", b"\n"); old = open(d + r"\old", "rb").read().replace(b"\r\n", b"\n")
    open(d + r"\b.lf", "wb").write(base); open(d + r"\o.lf", "wb").write(old)
    patch = subprocess.run(["git", "diff", "--no-index", "-U2", d + r"\b.lf", d + r"\o.lf"], capture_output=True).stdout
    open(os.path.join(B, f"open-old-change-{i}.patch"), "wb").write(patch)
    rows.append((i, "kept-C-semio", p))
open(os.path.join(here, "🗑️generated", "rescue-decisions.tsv"), "w", encoding="utf-8").write("\n".join(f"{a}\t{b}\t{c}" for a, b, c in rows))
print(sum(1 for r in rows if r[1] == "kept-C-semio"), "kept;", sum(1 for r in rows if r[1] == "old-resolved"), "old")
