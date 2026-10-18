"""📑 OHH late-inner-doc scan (read only): reports `.rs` files whose `//!` inner doc comment follows an item or a `use` (rustc E0753). usage: innerdoc.py <pathspec>..."""
import subprocess, sys, re
files = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "ls-files", "-z", "--", *sys.argv[1:]], capture_output=True).stdout.split(b"\0")
bad = 0
for f in files:
    f = f.decode()
    if not f.endswith(".rs"):
        continue
    try:
        lines = open(f, encoding="utf-8").read().split("\n")
    except Exception:
        continue
    seen_item = False
    for i, l in enumerate(lines[:400]):
        s = l.strip()
        if not s or s.startswith("//") and not s.startswith("//!"):
            continue
        if s.startswith("//!") or s.startswith("#![") :
            if seen_item and s.startswith("//!"):
                print(f"{f}:{i+1}")
                bad += 1
                break
            continue
        if s.startswith("#[") or s.startswith("/*"):
            seen_item = True
            continue
        seen_item = True
print("total", bad)
