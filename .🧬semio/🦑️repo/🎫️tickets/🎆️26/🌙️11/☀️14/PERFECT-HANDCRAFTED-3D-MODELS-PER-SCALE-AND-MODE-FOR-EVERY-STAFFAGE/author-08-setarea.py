import re, sys
from author_08_lib import *
def set_area(slug, vals):
    path = asset_file("humans", slug)
    lines = open(path).read().split("\n")
    t0 = next(i for i, l in enumerate(lines) if re.match(r'^\s*"thermal": \{', l))
    start = next(i for i in range(t0, len(lines)) if lines[i].strip() == '"regions": [')
    for rid, n in vals.items():
        i = next(i for i in range(start, len(lines)) if lines[i].strip() == f'"id": "{rid}",')
        j = next(j for j in range(i, i + 40) if lines[j].strip().startswith('"areaM2":'))
        lines[j] = re.sub(r'"areaM2": *[0-9.eE+-]+', f'"areaM2": {vals[rid]:g}', lines[j])
    open(path, "w").write("\n".join(lines))
R = {
 "person-cooking": {"face-skin": 0.1173, "hob": 0.3, "pot": 0.12, "spoon": 0.012},
 "person-with-rollator": {"face-skin": 0.0733, "hair": 0.0163, "beret": 0.0326},
 "person-with-umbrella": {"hair": 0.073, "torso-clothing": 0.037, "shoes": 0.128, "canopy": 0.87},
 "person-with-white-cane": {"sunglasses": 0.006, "cane-tip": 0.003},
}
for s, v in R.items():
    set_area(s, v)
    print("reverted", s, v)
