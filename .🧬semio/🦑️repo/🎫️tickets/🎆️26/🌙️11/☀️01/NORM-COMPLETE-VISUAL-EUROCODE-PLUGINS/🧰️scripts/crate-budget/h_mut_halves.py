"""🪚 Crate-budget H: every mutation leaf crate above 24 files (not en1994, frozen) is halved with `split_mounts.split` into `<crate>-more`. `h_mut_halves.py <scan.txt>`"""
import os, re, sys, traceback
sys.path.insert(0, os.path.dirname(__file__))
from g_lib import *
import split_mounts
for line in open(sys.argv[1], encoding="utf-8"):
    p = line.split()
    if len(p) < 6 or "-mut-" not in p[4] or int(p[2]) <= 24 or "en1994" in p[4] or p[4].endswith("-more"):
        continue
    d = p[5]
    art = re.search(r"🗿️artifacts/([^/]+)/", d).group(1)
    old_dir = re.search(r"🧬️mutations/([^/]+)/", d).group(1)
    root = rd(os.path.join(SEMIO, d, "../../🦀️.rs"))
    mounts = split_mounts.MOUNT.findall(root)
    mods = [m for _, m in mounts][len(mounts) // 2:]
    slug = p[4].split("-mut-")[1] + "-more"
    try:
        split_mounts.split(art, old_dir, old_dir + "-more", slug, mods, "more mutation leaves")
    except Exception:
        print("FAILED", p[4])
        traceback.print_exc()
