"""Jogger 1:100: kit coarsen of the finished 1:50 with hand repairs, thermal, structure."""
from author_33_lib import *
slug = "jogger"
print(coarsen(slug, 50, 100, 0.995, drop=("sleeve-r", "shorts-stripe-r", "headband", "shoulder-l", "shoulder-r")))
d = load(slug); b = d["volumes"]["100"]["body"]
h = by(b, "shirt-hem"); h["sections"].insert(0, sec(-0.115, 0.118, 0.085, 2.2)); h["sections"].insert(0, sec(-0.123, 0.112, 0.08, 2.2)); save(slug, d)
print(thermal(slug, 100, "torso-clothing=shirt-hem+shoulder-l+shoulder-r")[:100])
person_structure(slug, 100)
print(bun("author-33-joints.ts", f"humans/{slug}", "--scale", "100", "--write")[-100:])
print(check(slug, 100))
