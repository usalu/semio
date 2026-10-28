"""🔐 [backlog2] stage-tool mode law on a mirror: apply the whole queue, list the files that existed before, revert, chmod those to 0755, apply again; every replaced file must still be 0755 and every new file 0666 minus the umask. usage: python3 -I modes_roundtrip.py <mirror-name>   (uses backlog2-tools/order.txt and land.py)"""
import os, stat, subprocess, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__))); T = C + "/backlog-tools"
home = os.path.join(os.environ["TMPDIR"], "sm"); root = f"{home}/{sys.argv[1]}"
ids = open(C + "/backlog2-tools/order.txt").read().split()
um = os.umask(0); os.umask(um)
subs = ["🧰️framework/🛍️products/🦑️repo", "🧰️framework/🔨️modules", "🧰️framework/📦️packages", ".github", ".devcontainer"]
def snap():
    out = {}
    for sub in subs:
        for dp, dn, fn in os.walk(f"{root}/{sub}"):
            dn[:] = [d for d in dn if d not in ("node_modules", ".cache") and not os.path.islink(os.path.join(dp, d))]
            for f in fn:
                p = os.path.join(dp, f)
                if not os.path.islink(p): out[os.path.relpath(p, root)] = open(p, "rb").read()
    return out
def land(i, cmd): return subprocess.run(["python3", "-I", f"{T}/land.py", i, cmd, "--root", root], capture_output=True, text=True).returncode
before = snap()
for i in ids:
    if land(i, "apply"): print("APPLY-FAIL", i); sys.exit(2)
after = snap(); existing = [r for r in after if r in before and after[r] != before[r]]; fresh = [r for r in after if r not in before]
for i in reversed(ids):
    if land(i, "revert"): print("REVERT-FAIL", i)
for r in existing: os.chmod(f"{root}/{r}", 0o755)
for i in ids:
    if land(i, "apply"): print("APPLY2-FAIL", i); sys.exit(2)
bad = [(r, oct(stat.S_IMODE(os.stat(f"{root}/{r}").st_mode))) for r in existing if stat.S_IMODE(os.stat(f"{root}/{r}").st_mode) != 0o755]
badnew = [(r, oct(stat.S_IMODE(os.stat(f"{root}/{r}").st_mode))) for r in fresh if stat.S_IMODE(os.stat(f"{root}/{r}").st_mode) != 0o666 & ~um]
print(f"replaced files {len(existing)}, new files {len(fresh)}; mode violations: replaced {len(bad)}, new {len(badnew)}")
for r, m in (bad + badnew)[:40]: print(" ", m, r[-110:])
sys.exit(1 if bad or badnew else 0)
