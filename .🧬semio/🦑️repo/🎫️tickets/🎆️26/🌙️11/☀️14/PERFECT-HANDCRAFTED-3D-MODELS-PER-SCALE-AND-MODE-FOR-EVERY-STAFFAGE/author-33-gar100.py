"""Kneeling-gardener 1:100: kit coarsen of the finished 1:50, hand repairs, thermal, structure."""
from author_33_lib import *
slug = "kneeling-gardener"
print(coarsen(slug, 50, 100, 0.99))
d = load(slug); v = d["volumes"]["100"]; b = v["body"]
print([s["id"] for s in b])
save(slug, d)
print(thermal(slug, 100, "shirt-torso=shirt+shirt-hem,shirt-sleeves=upper-arm-l+upper-arm-r+shoulder-l+shoulder-r,forearms=forearm-l+forearm-r,gloves=glove-l+glove-r,clogs=clog-l+clog-r+clog-toe-l+clog-toe-r,hat=hat-crown+hat-brim+hat-band,face=head")[:1900])
d = load(slug)
drop = ("trug-weeds", "weed-stalk-0", "weed-stalk-1", "weed-stalk-2", "weed-stalk-3", "clog-strap-l", "clog-strap-r")
d["volumes"]["100"]["thermal"] = [t for t in d["volumes"]["100"]["thermal"] if t.get("of") not in drop]
save(slug, d)
person_structure(slug, 100)
print(bun("author-33-joints.ts", f"humans/{slug}", "--scale", "100", "--write")[-300:])
print(check(slug, 100))
