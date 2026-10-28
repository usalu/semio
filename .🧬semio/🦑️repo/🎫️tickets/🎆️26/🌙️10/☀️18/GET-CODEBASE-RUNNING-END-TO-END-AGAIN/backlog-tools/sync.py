"""🔄 [backlog] brings a mirror up to the CURRENT live tree with ordinary reads and writes (no clonefile of live files: a clone marks every live source ItemCloned and the change journal then reports the whole tree as modified). Copies changed/new tracked+untracked-not-ignored files of the mirrored subtrees, removes files that vanished live. Never writes live. usage: sync.py <mirror> [--dry]"""
import os, subprocess, sys, shutil
home = os.path.join(os.environ.get("TMPDIR", "/tmp"), "sm")
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
mir = f"{home}/{sys.argv[1]}"; dry = "--dry" in sys.argv
subs = ["🧰️framework/🛍️products/🦑️repo", "🧰️framework/🔨️modules", "🧰️framework/📦️packages", ".github", ".devcontainer"]
out = subprocess.run(["git", "ls-files", "-co", "--exclude-standard", "-z", "--", *subs], cwd=LIVE, capture_output=True).stdout.split(b"\0")
live = {p.decode() for p in out if p}
n = {"copied": 0, "same": 0, "removed": 0, "skipped": 0}
def stat(p):
    try: st = os.lstat(p); return (st.st_size, st.st_mtime_ns)
    except OSError: return None
for rel in sorted(live):
    a, b = f"{LIVE}/{rel}", f"{mir}/{rel}"
    sa = stat(a)
    if sa is None or not os.path.isfile(a) or os.path.islink(a): n["skipped"] += 1; continue
    if sa == stat(b): n["same"] += 1; continue
    if os.path.exists(b) and os.path.getsize(a) == os.path.getsize(b) and open(a, "rb").read() == open(b, "rb").read(): n["same"] += 1; continue
    n["copied"] += 1
    if dry: continue
    os.makedirs(os.path.dirname(b), exist_ok=True)
    if os.path.lexists(b): os.remove(b)
    with open(a, "rb") as src, open(b, "wb") as dst:
        shutil.copyfileobj(src, dst, 1 << 20)
    os.chmod(b, os.stat(a).st_mode & 0o7777); os.utime(b, ns=(sa[1], sa[1]))
for sub in subs:
    for dp, dn, fn in os.walk(f"{mir}/{sub}"):
        dn[:] = [d for d in dn if d not in ("node_modules", ".cache")]
        for f in fn:
            rel = os.path.relpath(os.path.join(dp, f), mir)
            if rel not in live and os.path.isfile(f"{LIVE}/{rel}") is False:
                n["removed"] += 1
                if not dry: os.remove(os.path.join(dp, f))
print(n)
