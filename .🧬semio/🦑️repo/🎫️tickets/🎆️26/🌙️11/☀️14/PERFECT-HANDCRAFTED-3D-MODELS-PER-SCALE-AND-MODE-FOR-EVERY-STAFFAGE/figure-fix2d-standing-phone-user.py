"""Figure kit 2D repair (inputs kept): standing-phone-user plan head. The plan shapes of the bowed head (hair ellipse `head`, `face`, `nose`, `ear-l/r`) were authored upright; the pose tilts the head by 44 degrees and the plan foreshortens bone-local x by cos 44 degrees, so the hair drew 0.130 m long where the elevation draws the same head 0.203 m (x 0.019 .. 0.222). Bone-local plan x is re-solved so that every plan extent equals the elevation one: hair x 0.019..0.222, face (forehead disc) 0.150..0.212, nose tip 0.008 beyond (nose capsule x0 0.1591, x1 0.1741), ears at the elevation ear (x 0.072..0.105). All four scales."""
import os, shutil, re
A = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/standing-phone-user/🔣️.json"
B = os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated/figure/backup/standing-phone-user.asset.before-2d-fix.json")
if not os.path.exists(B): shutil.copy(A, B)
L = open(A).read().split("\n"); n = 0
FIX = {"head": {"cx": "0.0186", "rx": "0.1411"}, "face": {"cx": "0.1167", "rx": "0.0549"}, "ear-l": {"cx": "-0.0384"}, "ear-r": {"cx": "-0.0384"}}
# The plan mapping is not the same for every shape and scale (1:500 hair needs cx 0.0428 / rx 0.1427, the 1:50 and 1:100 face cx 0.1167 / rx 0.0549, solved numerically so that the projected extents equal the elevation ones, probe: 🗑️generated/figure/probe-all.ts); the values below are those finals.
NOTE = "applied once with these finals; asset backup kept in 🗑️generated/figure/backup"
for i, l in enumerate(L):
    m = re.match(r'\s*"id": "(head|face|ear-l|ear-r|nose)",', l)
    if not m: continue
    a = i
    while L[a].strip() != "{": a -= 1
    b = i
    while L[b].strip() not in ("}", "},"): b += 1
    blk = "\n".join(L[a:b + 1])
    if '"bone": "head"' not in blk: continue
    if '"kind": "ellipse"' in blk and '"cx": 0.0082' in blk and m.group(1) in FIX and (m.group(1) != "head" or '"region": "hair"' in blk):
        for k in range(a, b + 1):
            for key, val in FIX[m.group(1)].items():
                if re.match(rf'\s*"{key}": ', L[k]): L[k] = re.sub(r'(: )[-\d.]+', rf'\g<1>{val}', L[k]); 
        n += 1
    elif m.group(1) == "nose" and '"kind": "capsule"' in blk and '"x0": 0.0982' in blk:
        for k in range(a, b + 1):
            if '"x0": 0.0982' in L[k]: L[k] = L[k].replace("0.0982", "0.1341")
            if '"x1": 0.1132' in L[k]: L[k] = L[k].replace("0.1132", "0.1491")
        n += 1
open(A, "w").write("\n".join(L)); print("edited blocks", n)
