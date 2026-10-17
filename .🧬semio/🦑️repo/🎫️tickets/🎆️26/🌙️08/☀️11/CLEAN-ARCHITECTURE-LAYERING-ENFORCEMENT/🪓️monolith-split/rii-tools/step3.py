"""📋 rii step 3: registers the new crate `semio-framework-os-renderer-wgpu-thread-isolation`: workspace member, owner row, six launch routes in both launch files (cloned from the test-support routes so each file keeps its own layout), Cargo.lock, taxonomy member. usage: step3.py [--dry]"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, HERE + "/../rtb-tools")
os.environ["DRY"] = "1" if "--dry" in sys.argv else "0"
import register as RG
from lib import read, REPO
CRATE = "semio-framework-os-renderer-wgpu-thread-isolation"
PROJECT = "@semio-tech/os-renderer-wgpu-thread-isolation-rs"
DIR = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🔒️thread-isolation"
PKG = DIR + "/📦️packages/🦀️rust"
FOLDER = "🔒️thread-isolation"
print("member", RG.add_member(PKG))
print("owner", RG.add_owner(PROJECT, PKG, PKG + "/📋️project.json"))

def order_values(text):
    return {float(x) for x in re.findall(r'"order":\s*([0-9.]+)', text)}

def routes(f):
    text = RG.rd(f)
    if f'"name": "🦀️ {CRATE} 🏗️build 🛠️prototype"' in text:
        print("routes present", f)
        return
    used = order_values(RG.rd(".vscode/launch.json")) | order_values(RG.rd(".vscode/🧩️launch.seed.jsonc"))
    tmpl = []
    for m in re.finditer(r'    \{\n      "name": "🦀️ semio-framework-os-renderer-wgpu-test-support [^\n]*\n(?:(?!    \},?\n).*\n)*?    \},\n', text):
        tmpl.append(m.group(0))
    assert len(tmpl) == 6, len(tmpl)
    start = {"900.": 24580, "922.": 5380, "903.": 59380}
    nxt = {}
    for pfx, n0 in start.items():
        picked = []
        n = n0
        while len(picked) < 2:
            v = float(f"{pfx}{n:05d}")
            if v not in used:
                picked.append(f"{pfx}{n:05d}".rstrip("0") if False else str(float(f"{pfx}{n:05d}")))
                used.add(v)
            n += 1
        nxt[pfx] = picked
    out = ""
    for blk in tmpl:
        b = blk.replace("semio-framework-os-renderer-wgpu-test-support", CRATE).replace("@semio-tech/os-renderer-wgpu-test-support-rs", PROJECT)
        group = re.search(r'"group": "(4_[a-z]+)"', b).group(1)
        pfx = {"4_build": "900.", "4_gate": "922.", "4_test": "903."}[group]
        prof = "release" if "--configuration=release" in b else "prototype"
        idx = 1 if prof == "release" else 0
        b = re.sub(r'("order":\s*)[0-9.]+', lambda m: m.group(1) + nxt[pfx][idx], b)
        out += b
    anchor = f'      "name": "🦀️ semio-framework-os-renderer-wgpu-frame-tests-1 🧪️test 🚀️release",'
    assert text.count(anchor) == 1
    i = text.index(anchor)
    j = text.index("\n    },\n", i) + len("\n    },\n")
    RG.wr(f, text[:j] + out + text[j:])
    print("routes added", f, nxt)

for f in RG.LAUNCH:
    routes(f)

tax = RG.rd(RG.TAX)
if f'"{FOLDER}"' in tax:
    print("taxonomy present")
else:
    anchor = '        "🍇️frame-tests-1"\n      ],\n      "source": "registry"\n    },\n    "members-of-hub-inference": {'
    assert tax.count(anchor) == 1
    out = tax.replace(anchor, f'        "🍇️frame-tests-1",\n        "{FOLDER}"\n      ],\n      "source": "registry"\n    }},\n    "members-of-hub-inference": {{')
    json.loads(out)
    RG.wr(RG.TAX, out)
    print("taxonomy added")
