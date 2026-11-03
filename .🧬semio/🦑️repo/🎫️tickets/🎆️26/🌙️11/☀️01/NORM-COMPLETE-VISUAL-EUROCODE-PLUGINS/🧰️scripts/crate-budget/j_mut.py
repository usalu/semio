"""🪚 Crate-budget J: moves the second half of the kind mounts of one mutation leaf crate into a sibling (`art old_dir new_dir slug`) with split_mounts."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from g_lib import *
import split_mounts
art, old_dir, new_dir, slug = sys.argv[1:5]
M = NORM + "/🗿️artifacts/" + art + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
mounts = split_mounts.MOUNT.findall(rd(M + "/" + old_dir + "/🦀️.rs"))
split_mounts.split(art, old_dir, new_dir, slug, [m for _, m in mounts][len(mounts) // 2:], "more mutation leaves")
