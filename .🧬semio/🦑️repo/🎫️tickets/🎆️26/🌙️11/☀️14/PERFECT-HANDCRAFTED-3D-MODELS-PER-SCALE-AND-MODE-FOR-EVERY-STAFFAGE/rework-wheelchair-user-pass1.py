"""🦽 wheelchair-user, author pass 1 (second round): hand-stated corrections on top of `rework-wheelchair-user-rig.py` (anatomical girdles) and `rework-wheelchair-user-coarse.py`.

Every number is chosen by hand. Left parts are stated, right parts are their mirror images.
- load path: the sitter bears on the cushion and the slings, so cushion, seat sling and back sling (1:500: seat and back block) are structural members; castor forks, axle plates and axle receivers are members wherever the body has them;
- sacrum with its alae (104 mm across the base) so that the sacroiliac joints lie in bone; coccyx at 1:50;
- finger bones start at the heads of their metacarpals; acromion at 1:100 ties the scapula to the clavicle;
- 1:200 sternum as a typified manubrium and body that reaches the sternoclavicular joints; talus dome on the tarsal block at 1:200 and 1:500 so the tibia stands on the foot;
- chair: the rear cross tube reaches the axle plates, the footplate is a 15.5 mm moulded plate lying on the footrest loop, at 1:500 the back post runs down to the axle receiver (post and axle strut drawn as one tube).

Usage: python3 rework-wheelchair-user-pass1.py             cand/ -> wu-p1/🔣️.json
       python3 rework-wheelchair-user-pass1.py --publish   wu-p1/ -> catalog members, volumes source, volumes-insert
"""
import copy
import json
import os
import subprocess
import sys

T = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(T, *([".."] * 7)))
CATALOG = os.path.join(REPO, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "👤️humans", "wheelchair-user", "🔣️.json")
SOURCE = os.path.join(T, "volumes", "humans", "wheelchair-user.json")
BASE = os.path.join(T, "🗑️generated", "rework-wheelchair-user", "cand", "🔣️.json")
OUT = os.path.join(T, "🗑️generated", "rework-wheelchair-user", "wu-p1", "🔣️.json")
ALONG = [0.5, 0.5, 0.5, 0.5]
SACRUM = {
    "50": [([0.098, 0.04, 0], 0.017, 0.052), ([0.06, 0.062, 0], 0.014, 0.045), ([0.02, 0.078, 0], 0.011, 0.028), ([-0.01, 0.084, 0], 0.008, 0.014), ([-0.035, 0.078, 0], 0.005, 0.007)],
    "100": [([0.098, 0.04, 0], 0.017, 0.052), ([0.04, 0.07, 0], 0.013, 0.038), ([-0.01, 0.084, 0], 0.008, 0.014)],
    "200": [([0.098, 0.04, 0], 0.0152, 0.052), ([0.04, 0.07, 0], 0.0152, 0.038), ([0, 0.082, 0], 0.0152, 0.0152)],
}
STERNUM_200 = [([0.262, -0.112, 0], 0.0152, 0.028), ([0.2, -0.135, 0], 0.0152, 0.02), ([0.115, -0.1456, 0], 0.0152, 0.0152)]
TARSALS = {
    "200": [([-0.05, -0.05, 0], 0.022, 0.02), ([0.0, -0.03, 0], 0.026, 0.03), ([0.06, -0.045, 0], 0.03, 0.018), ([0.138, -0.062, 0], 0.034, 0.0152)],
    "500": [([-0.055, -0.04, 0], 0.038, 0.038), ([0.0, -0.03, 0], 0.038, 0.04), ([0.138, -0.05, 0], 0.038, 0.038)],
}
SEAT_MEMBERS = {"50": ["seat-sling", "seat-cushion", "back-sling"], "100": ["seat-cushion", "back-sling"], "200": ["seat-cushion", "back-sling"], "500": ["seat-block", "back-block"]}
FRAME_MEMBERS = ["castor-fork-l", "castor-fork-r", "axle-plate-l", "axle-plate-r", "axle-receiver-l", "axle-receiver-r", "axle-strut-l", "axle-strut-r"]


def mirrored(solid):
    out = copy.deepcopy(solid)
    out["id"] = out["id"][:-2] + "-r"
    for key in ("bone", "member"):
        if out.get(key, "").endswith("-l"):
            out[key] = out[key][:-2] + "-r"
    for section in out.get("sections", []):
        section["at"] = [section["at"][0], section["at"][1], -section["at"][2]]
    return out


def find(volume, name, ident):
    return next((s for s in volume[name] if s["id"] == ident), None)


def restate(solid, rows, rotation=ALONG, **extra):
    solid["sections"] = [{"at": at, "rx": rx, "ry": ry, **({"rotation": rotation} if rotation else {}), **extra} for at, rx, ry in rows]
    solid.pop("slender", None)


def structure(asset):
    for scale, volume in asset["volumes"].items():
        body = {s["id"] for s in volume["body"]}
        ids = {s["id"] for s in volume["structure"]}
        for ident in SEAT_MEMBERS[scale] + FRAME_MEMBERS:
            if ident in body and f"st-{ident}" not in ids:
                volume["structure"].append({"id": f"st-{ident}", "of": ident, "member": "frame", "part": "member"})
        if scale in SACRUM:
            restate(find(volume, "structure", "sacrum"), SACRUM[scale])
        if scale in TARSALS:
            for side in ("l", "r"):
                solid = find(volume, "structure", f"tarsals-{side}")
                restate(solid, TARSALS[scale], rotation=[0, -0.70711, 0, 0.70711], exponent=2)
        if scale == "200":
            restate(find(volume, "structure", "sternum"), STERNUM_200, exponent=3)
        if scale == "50":
            find(volume, "structure", "spinous-4")["p0"] = [0.1, -0.012, 0]
        if scale in ("50", "100"):
            for side in ("l", "r"):
                for ray in "1234":
                    meta, phal = find(volume, "structure", f"metacarpal-{ray}-{side}"), find(volume, "structure", f"phalanges-{ray}-{side}")
                    if meta and phal and phal["sections"][0]["at"] != meta["sections"][-1]["at"]:
                        phal["sections"].insert(0, {**copy.deepcopy(phal["sections"][0]), "at": list(meta["sections"][-1]["at"])})
        if scale == "100" and "acromion-l" not in ids:
            fine = asset["volumes"]["50"]["structure"]
            for side in ("l", "r"):
                acromion = copy.deepcopy(next(s for s in fine if s["id"] == f"acromion-{side}"))
                for section in acromion["sections"]:
                    section["rx"] = section["ry"] = 0.008
                at = next(i for i, s in enumerate(volume["structure"]) if s["id"] == f"scapula-{side}") + 1
                volume["structure"].insert(at, acromion)


def chair(asset):
    fine = asset["volumes"]["50"]
    cross = find(fine, "body", "rear-cross")
    cross["p0"][2], cross["p1"][2] = -0.2195, 0.2195
    find(fine, "body", "footplate")["from"] = -0.2165
    coarse = asset["volumes"]["500"]
    for side, z in (("l", 0.215), ("r", -0.215)):
        post = find(coarse, "body", f"back-post-{side}")
        if post["sections"][0]["at"][1] > 0.05:
            post["sections"].insert(0, {"at": [-0.03, 0.02, z], "rx": 0.0135, "ry": 0.0135})


def publish():
    kit = {"__file__": os.path.join(T, "rework-04-publish.py")}
    exec(open(os.path.join(T, "rework-04-publish.py"), encoding="utf-8").read().split("\ndef main")[0], kit)
    candidate = json.load(open(OUT, encoding="utf-8"))["asset"]
    text = open(CATALOG, encoding="utf-8").read()
    current = json.loads(text)["asset"]
    if list(candidate.keys()) != list(current.keys()):
        raise SystemExit("member order differs")
    changed = [k for k in current if k != "volumes" and candidate[k] != current[k]]
    for name in changed:
        a, b = kit["member_range"](text, name)
        text = text[:a] + kit["dump"](candidate[name]) + text[b:]
    check = json.loads(text)["asset"]
    for k in current:
        if k != "volumes" and check[k] != candidate[k]:
            raise SystemExit(f"member {k} not written as intended")
    open(CATALOG, "w", encoding="utf-8").write(text)
    open(SOURCE, "w", encoding="utf-8").write(json.dumps({"volumes": candidate["volumes"]}, indent=1, ensure_ascii=False) + "\n")
    subprocess.run([sys.executable, os.path.join(T, "volumes-insert.py"), SOURCE, CATALOG], check=True)
    print("published members:", ", ".join(changed), "+ volumes")


def main():
    if "--publish" in sys.argv:
        return publish()
    document = json.load(open(BASE, encoding="utf-8"))
    asset = document["asset"]
    structure(asset)
    chair(asset)
    extra = os.path.join(T, "rework-wheelchair-user-pass1b.py")
    if os.path.exists(extra):
        scope = {"__file__": extra}
        exec(open(extra, encoding="utf-8").read(), scope)
        scope["apply"](asset)
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    open(OUT, "w", encoding="utf-8").write(json.dumps(document, indent=1, ensure_ascii=False) + "\n")
    print("written", OUT)


main()
