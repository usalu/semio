"""🪚 Crate-budget I: halves mutation leaf crates given as `art old_dir` pairs via split_mounts."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from g_lib import *
import split_mounts
art, old_dir = sys.argv[1], sys.argv[2]
M = NORM + "/🗿️artifacts/" + art + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
root = rd(M + "/" + old_dir + "/🦀️.rs")
mounts = split_mounts.MOUNT.findall(root)
mods = [m for _, m in mounts][len(mounts) // 2:]
tom = rd(M + "/" + old_dir + "/📦️packages/🦀️rust/Cargo.toml")
slug = re.search(r'name = "[^"]+-mut-([^"]+)"', tom).group(1) + "-more"
split_mounts.split(art, old_dir, old_dir + "-more", slug, mods, "more mutation leaves")
