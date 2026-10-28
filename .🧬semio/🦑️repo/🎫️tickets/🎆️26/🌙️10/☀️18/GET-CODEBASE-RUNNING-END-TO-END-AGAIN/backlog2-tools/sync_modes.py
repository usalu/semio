"""🔐 [backlog2] copies the modes of the files of root A onto the same files of root B (mirror hygiene after a mode experiment). usage: python3 -I sync_modes.py <a> <b>"""
import os, sys
a, b = sys.argv[1:3]; n = 0
for sub in ["🧰️framework/🛍️products/🦑️repo", "🧰️framework/🔨️modules", "🧰️framework/📦️packages", ".github", ".devcontainer"]:
    for dp, dn, fn in os.walk(f"{b}/{sub}"):
        dn[:] = [d for d in dn if d not in ("node_modules", ".cache") and not os.path.islink(os.path.join(dp, d))]
        for f in fn:
            p = os.path.join(dp, f); q = a + "/" + os.path.relpath(p, b)
            if os.path.exists(q) and not os.path.islink(p) and os.stat(p).st_mode != os.stat(q).st_mode: os.chmod(p, os.stat(q).st_mode & 0o7777); n += 1
print("modes synced", n)
