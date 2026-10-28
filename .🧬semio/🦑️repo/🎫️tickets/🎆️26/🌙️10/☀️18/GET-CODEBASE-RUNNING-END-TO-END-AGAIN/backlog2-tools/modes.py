"""🔐 [backlog2] mode law over two roots: every file present in both with different bytes must have the same mode (a stage tool keeps the mode of the file it replaces), every file new in the second must be 0666 minus the umask (never the 0600 of mkstemp). usage: python3 -I modes.py <base-root> <after-root>   (walks the product, process, packages, .github, .devcontainer subtrees; exit 1 on a violation)"""
import os, stat, sys
base, after = sys.argv[1:3]
um = os.umask(0); os.umask(um)
subs = ["🧰️framework/🛍️products/🦑️repo", "🧰️framework/🔨️modules", "🧰️framework/📦️packages", ".github", ".devcontainer"]
bad = 0; n = 0
for sub in subs:
    for dp, dn, fn in os.walk(f"{after}/{sub}"):
        dn[:] = [d for d in dn if d not in ("node_modules", ".cache")]
        for f in fn:
            p = os.path.join(dp, f); rel = os.path.relpath(p, after); q = f"{base}/{rel}"
            if os.path.islink(p): continue
            m = stat.S_IMODE(os.lstat(p).st_mode)
            if not os.path.exists(q):
                n += 1
                if m != (0o666 & ~um) and not (m & 0o111 and m & 0o666 == 0o666 & ~um): print("NEW-MODE", oct(m), rel[-110:]); bad += 1
            else:
                try:
                    if os.path.getsize(p) == os.path.getsize(q) and open(p, "rb").read() == open(q, "rb").read(): continue
                except OSError: continue
                n += 1; mb = stat.S_IMODE(os.lstat(q).st_mode)
                if m != mb: print("CHANGED-MODE", oct(mb), "->", oct(m), rel[-110:]); bad += 1
print(f"checked {n} changed/new files, {bad} violations"); sys.exit(1 if bad else 0)
