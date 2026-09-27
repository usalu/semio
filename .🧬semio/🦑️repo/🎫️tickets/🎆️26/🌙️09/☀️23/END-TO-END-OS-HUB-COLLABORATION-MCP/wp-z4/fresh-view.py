#!/usr/bin/env python3
"""🧊️ Z4: materializes a fresh-clone VIEW of the working tree (tracked + untracked-not-ignored files, tickets and
`.tmp*` excluded) as APFS clones (copy-on-write, never hard links) so probes can run the repository's own validators
against exactly what a clone would contain. usage: fresh-view.py <repo root> <destination (must not exist)>"""
import ctypes, os, subprocess, sys

repo, dest = os.path.abspath(sys.argv[1]), os.path.abspath(sys.argv[2])
if os.path.exists(dest): sys.exit(f"destination exists: {dest}")
libc = ctypes.CDLL("libc.dylib", use_errno=True)
libc.clonefile.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_uint32]
out = subprocess.run(["git", "-C", repo, "ls-files", "-co", "--exclude-standard", "-z"], check=True, capture_output=True).stdout
paths = [p for p in out.decode("utf-8").split("\0") if p and not p.startswith(".tmp") and not p.startswith(".🧬semio/🦑️repo/🎫️tickets/")]
copied = links = missing = 0
for rel in paths:
    src, dst = os.path.join(repo, rel), os.path.join(dest, rel)
    if not os.path.lexists(src): missing += 1; continue
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    if os.path.islink(src): os.symlink(os.readlink(src), dst); links += 1; continue
    if os.path.isdir(src): continue
    if libc.clonefile(src.encode(), dst.encode(), 0) != 0: sys.exit(f"clonefile failed ({ctypes.get_errno()}): {rel}")
    copied += 1
print(f"view={dest} files={copied} symlinks={links} listed-but-missing={missing}")
