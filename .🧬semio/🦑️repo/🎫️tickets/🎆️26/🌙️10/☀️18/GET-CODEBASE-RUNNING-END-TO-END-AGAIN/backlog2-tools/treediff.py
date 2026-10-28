"""🔎 [backlog2] content diff of two roots over the lane subtrees (symlinks skipped): lists files only in one root or differing in bytes or mode. usage: python3 -I treediff.py <a> <b> [--modes]"""
import os, stat, sys
a, b = sys.argv[1:3]; modes = "--modes" in sys.argv
subs = ["🧰️framework/🛍️products/🦑️repo", "🧰️framework/🔨️modules", "🧰️framework/📦️packages", ".github", ".devcontainer", "✏️s"]
def walk(root):
    out = {}
    for sub in subs:
        for dp, dn, fn in os.walk(f"{root}/{sub}"):
            dn[:] = [d for d in dn if d not in ("node_modules", ".cache") and not os.path.islink(os.path.join(dp, d))]
            for f in fn:
                p = os.path.join(dp, f)
                if not os.path.islink(p): out[os.path.relpath(p, root)] = p
    return out
A, B = walk(a), walk(b); n = 0
for rel in sorted(set(A) | set(B)):
    if rel not in B: print("ONLY-A", rel[-120:]); n += 1
    elif rel not in A: print("ONLY-B", rel[-120:]); n += 1
    elif open(A[rel], "rb").read() != open(B[rel], "rb").read(): print("BYTES", rel[-120:]); n += 1
    elif modes and stat.S_IMODE(os.stat(A[rel]).st_mode) != stat.S_IMODE(os.stat(B[rel]).st_mode): print("MODE", oct(stat.S_IMODE(os.stat(A[rel]).st_mode)), oct(stat.S_IMODE(os.stat(B[rel]).st_mode)), rel[-100:]); n += 1
print(len(A), len(B), "files;", n, "differences"); sys.exit(1 if n else 0)
