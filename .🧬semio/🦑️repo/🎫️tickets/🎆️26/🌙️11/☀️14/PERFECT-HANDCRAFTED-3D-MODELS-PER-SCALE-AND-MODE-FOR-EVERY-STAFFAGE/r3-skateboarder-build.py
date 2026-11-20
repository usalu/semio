"""🏗️ Round 3 assembler of humans/skateboarder: merges the corrected roots (rig, anchors, poses), the hand-authored volumes of every scale and (after `r3-skateboarder-draw2d.py`) the redrawn 2D into one candidate asset document `🗑️generated/r3-skateboarder/🔣️.json`.

Usage: python3 r3-skateboarder-build.py [--draft 50 100 ...] [--old]   (--draft: body lists only with placeholder thermal and structure, for looking at the form; --old: keep the volumes as they were, for looking at the rig)
"""
import copy, importlib.util, json, os, sys

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated", "r3-skateboarder")


def mod(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(T, f"r3-skateboarder-{name}.py"))
    m = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(m)
    return m


def strip(solids):
    return [{k: v for k, v in s.items() if not k.startswith("_")} for s in solids]


def draft_scale(den, body):
    thermal = [{"id": f"th-{s['id']}", "of": s["id"], "region": s["_region"]} for s in body]
    structure = [{"id": f"st-{s['id']}", "of": s["id"], "member": s["bone"], "part": "member"} for s in body[:1]]
    return {"status": "authored", "denominator": den, "body": strip(body), "thermal": thermal, "structure": structure, "decisions": [{"en": "draft", "de": "Entwurf"}]}


def main():
    old = json.load(open(os.path.join(OUT, "asset.before.json")))
    root = json.load(open(os.path.join(OUT, "root.json")))
    doc = copy.deepcopy(old)
    a = doc["asset"]
    a["rig"], a["anchors"], a["poses"] = root["rig"], root["anchors"], root["poses"]
    if "--old" in sys.argv:
        pass
    elif "--draft" in sys.argv:
        scales = [int(x) for x in sys.argv[sys.argv.index("--draft") + 1:] if x.isdigit()] or [50]
        for den in scales:
            a["volumes"][str(den)] = draft_scale(den, mod("body50").build() if den == 50 else getattr(mod("coarse"), f"body{den}")())
    else:
        a["volumes"] = mod("volumes").build()
    json.dump(doc, open(os.path.join(OUT, "🔣️.json"), "w"), ensure_ascii=False, indent=1)
    print("candidate", {k: (len(v.get("body", [])), len(v.get("thermal", [])), len(v.get("structure", []))) for k, v in a["volumes"].items()})


if __name__ == "__main__":
    main()
