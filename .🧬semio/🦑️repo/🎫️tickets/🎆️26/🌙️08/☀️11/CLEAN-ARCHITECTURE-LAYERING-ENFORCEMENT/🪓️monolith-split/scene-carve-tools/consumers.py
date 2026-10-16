"""🧑‍🔧 Consumer API check: every name that a Rust consumer takes from the ui-scene facade must exist in the facade's explicit re-exports, math aggregator or pack module."""
import re, sys, collections
from lib import *
from model import *
REPO_SCENE = REPO + "/" + SCENE_REL
def api_of_tree():
    root = api_names(rd(REPO_SCENE + "/📦️packages/🦀️rust/🦀️.rs"))
    math = api_names(rd(REPO_SCENE + "/📐️math/🦀️.rs"))
    packn = api_names(rd(REPO_SCENE + "/📦️pack/🦀️.rs"))
    return root, math, packn
root, math, packn = api_of_tree()
root_names = set(root) | {"math", "pack"}
files = [l.strip() for l in open(SP + "/consumers.txt", encoding="utf-8") if l.strip()]
bad = collections.Counter()
seen = 0
for f in files:
    p = REPO + "/" + f
    try:
        t = rd(p)
    except Exception:
        continue
    for m in re.finditer(r"(?<![\w:])(?:semio_framework_ui_scene|ui_scene)::", t):
        i = m.end()
        if i < len(t) and t[i] == "{":
            depth, j = 0, i
            while j < len(t):
                depth += (t[j] == "{") - (t[j] == "}")
                j += 1
                if depth == 0:
                    break
            tree = "FACADE::" + t[i:j]
        else:
            m2 = re.match(r"\w+(?:::\w+)*(?:::\{)?", t[i:])
            if not m2:
                continue
            j = i + m2.end()
            if t[j - 1] == "{":
                depth, k = 1, j
                while k < len(t) and depth:
                    depth += (t[k] == "{") - (t[k] == "}")
                    k += 1
                j = k
            tree = "FACADE::" + t[i:j]
        try:
            names = flatten(re.sub(r"\s+", " ", tree).replace("FACADE::{", "{")) if tree.startswith("FACADE::{") else flatten(tree)
        except Exception as e:
            continue
        for n, full in names:
            segs = full.replace("FACADE::", "").split("::")
            seen += 1
            head = segs[0]
            if head == "self" or head == "*":
                continue
            if head not in root_names:
                bad[("root", head, f)] += 1
            elif head == "math" and len(segs) > 1 and segs[1] not in math and segs[1] not in ("*",):
                bad[("math", segs[1], f)] += 1
            elif head == "pack" and len(segs) > 1 and segs[1] not in packn and segs[1] != "*":
                bad[("pack", segs[1], f)] += 1
print("paths checked", seen, "files", len(files), "unresolved", len(bad))
for k, v in sorted(bad.items()): print(k, v)
