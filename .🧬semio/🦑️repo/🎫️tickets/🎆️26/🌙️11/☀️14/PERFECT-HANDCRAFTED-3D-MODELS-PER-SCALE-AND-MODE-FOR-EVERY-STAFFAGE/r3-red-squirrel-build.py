"""🏗 Round 3 red squirrel: assembles the candidate asset document (corrected rig, anchors, poses, clips, dimensions, 2D drawings, volumes) from the handcrafted modules and writes it to 🗑️generated/r3-red-squirrel/🔣️.json with its volumes source volumes/adhoc/r3-red-squirrel.json; `--apply` writes the root members into the catalog file (re-read right before, only the named members replaced, formatting kept) and the source volumes/animals/red-squirrel.json.

Usage: python3 r3-red-squirrel-build.py [--apply]
"""
import copy
import glob
import json
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import r3_red_squirrel_rig as rig
import r3_red_squirrel_volumes as volumes
import r3_red_squirrel_draw as draw
import r3_red_squirrel_text as text

T = os.path.dirname(os.path.abspath(__file__))
CATALOG = glob.glob("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/red-squirrel/🔣️.json")[0]
MANIFEST = os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(CATALOG))), "🔣️.json")
OUT = os.path.join(T, "🗑️generated", "r3-red-squirrel")
ROOT_MEMBERS = ["summary", "provenance", "dimensions", "rig", "anchors", "materials", "poses", "clips", "representations"]


def r(x, n=5):
    return round(x, n)


def new_rig(asset):
    old = {b["id"]: b for b in asset["rig"]["bones"]}
    bones = []
    for b in rig.BONES:
        o = old[b["id"]]
        n = {"id": b["id"], "parent": b["parent"], "title": o["title"], "x": b["x"], "y": b["y"]}
        if "z" in b:
            n["z"] = b["z"]
        n.update({"angleDeg": b["angleDeg"], "lengthM": b["lengthM"], "joint": o["joint"]})
        mass = dict(o["mass"])
        mass["comX"] = r(b["lengthM"] / 2) if b["id"] != "head" else 0.024
        n["mass"] = mass
        bones.append(n)
    return {"kind": asset["rig"]["kind"], "root": asset["rig"]["root"], "bones": bones}


def new_anchors(asset):
    out = []
    for a in asset["anchors"]:
        key = a["id"][:-2] if a["id"][-2:] in ("-n", "-f") else a["id"]
        spec = rig.ANCHORS[key]
        n = dict(a)
        for k in ("x", "y", "footprintLengthM", "footprintWidthM"):
            if k in spec:
                n[k] = spec[k]
        out.append(n)
    return out


def new_poses(asset):
    solved = rig.poses()
    out = []
    listed = list(asset["poses"])
    for pid, (after, title) in text.NEW_POSES.items():
        if not any(p["id"] == pid for p in listed):
            at = next(i for i, p in enumerate(listed) if p["id"] == after)
            listed.insert(at + 1, dict(copy.deepcopy(listed[at]), id=pid, title=title))
    for p in listed:
        off, root, ground, contacts = solved[p["id"]]
        n = dict(p)
        bones = {}
        for b in rig.BONES:
            o = off[b["id"]]
            e = {"angleDeg": r(o["angleDeg"], 2)}
            if o.get("abductionDeg"):
                e["abductionDeg"] = o["abductionDeg"]
            bones[b["id"]] = e
        n["bones"] = bones
        y = rig.root_y(p["id"], off, root, ground, contacts)
        if p["id"].startswith("climb-"):
            y += {"a": 0.0, "b": 0.024, "c": 0.048}[p["id"][-1]]
        n["root"] = {"x": 0, "y": r(y, 4)}
        n["ground"] = ground
        n["contacts"] = contacts
        if p["id"] in text.POSE_DESCRIPTIONS:
            n["description"] = text.POSE_DESCRIPTIONS[p["id"]]
        out.append(n)
    return out


def new_clips(asset):
    out = []
    for c in asset["clips"]:
        n = copy.deepcopy(c)
        if c["id"] in text.CLIP_PATCH:
            n.update(copy.deepcopy(text.CLIP_PATCH[c["id"]]))
        if c["id"] == "bound":
            for phase in n["phases"]:
                phase["toS"] = 0.24 if phase["id"] == "hind-stance" else phase["toS"]
                phase["fromS"] = 0.24 if phase["id"] == "air-b" else phase["fromS"]
            for event in n["events"]:
                event["t"] = 0.24 if event["kind"] == "contact-off" and event["target"] == "ground-foot-n" else event["t"]
        out.append(n)
    return out


def candidate():
    doc = json.load(open(CATALOG, encoding="utf-8"))
    a = doc["asset"]
    if text.SUMMARY:
        a["summary"] = text.SUMMARY
    a["dimensions"] = dict(a["dimensions"], **text.DIMENSIONS)
    if not any(x["id"] == text.REFERENCE["id"] for x in a["provenance"]["references"]):
        a["provenance"] = dict(a["provenance"], references=a["provenance"]["references"] + [text.REFERENCE])
    if not any(x["id"] == "bone" for x in a["materials"]):
        a["materials"] = a["materials"] + [text.BONE]
    a["rig"] = new_rig(a)
    a["anchors"] = new_anchors(a)
    a["poses"] = new_poses(a)
    a["clips"] = new_clips(a)
    vols = volumes.volumes()
    a["representations"] = draw.representations(a["representations"], vols["_solids"])
    del vols["_solids"]
    a["volumes"] = vols
    return doc


def member_end(s, start):
    depth, i, quoted = 0, start, False
    while i < len(s):
        ch = s[i]
        if quoted:
            if ch == "\\":
                i += 1
            elif ch == '"':
                quoted = False
        elif ch == '"':
            quoted = True
        elif ch in "{[":
            depth += 1
        elif ch in "}]":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    raise SystemExit("unbalanced file")


def replace_member(s, indent, key, value):
    found = re.search(r"\n" + indent + r'"' + key + r'"\s*:\s*', s)
    if not found:
        raise SystemExit(f"member {key} not found")
    end = member_end(s, found.end())
    body = json.dumps(value, ensure_ascii=False, indent=2).replace("\n", "\n" + indent)
    return s[: found.end()] + body + s[end:]


def apply(doc):
    s = open(CATALOG, encoding="utf-8").read()
    before = json.loads(s)
    for key in ROOT_MEMBERS:
        s = replace_member(s, "    ", key, doc["asset"][key])
    after = json.loads(s)
    for key in before["asset"]:
        if key not in ROOT_MEMBERS and before["asset"][key] != after["asset"][key]:
            raise SystemExit(f"member {key} changed unexpectedly")
    if list(before["asset"]) != list(after["asset"]):
        raise SystemExit("member order changed")
    open(CATALOG, "w", encoding="utf-8").write(s)
    m = open(MANIFEST, encoding="utf-8").read()
    at = m.index('"id": "red-squirrel"')
    end = m.index('"poses"', at)
    entry = m[at:end]
    entry = re.sub(r'("heightM":\s*)[0-9.]+', lambda g: g.group(1) + json.dumps(doc["asset"]["dimensions"]["heightM"]), entry)
    entry = re.sub(r'("widthM":\s*)[0-9.]+', lambda g: g.group(1) + json.dumps(doc["asset"]["dimensions"]["widthM"]), entry)
    fresh = open(MANIFEST, encoding="utf-8").read()
    if fresh != m:
        raise SystemExit("manifest changed while editing; run again")
    m = m[:at] + entry + m[end:]
    d_at = m.index('"distinction"', at)
    d_end = member_end(m, m.index("{", d_at))
    m = m[: m.index("{", d_at)] + json.dumps(text.SUMMARY, ensure_ascii=False, indent=2).replace("\n", "\n" + manifest_indent(m, d_at)) + m[d_end:]
    json.loads(m)
    open(MANIFEST, "w", encoding="utf-8").write(m)


def manifest_indent(m, at):
    line = m.rfind("\n", 0, at) + 1
    return m[line:at]


def main():
    doc = candidate()
    os.makedirs(OUT, exist_ok=True)
    json.dump(doc, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False)
    source = {"volumes": doc["asset"]["volumes"]}
    os.makedirs(os.path.join(T, "volumes", "adhoc"), exist_ok=True)
    json.dump(source, open(os.path.join(T, "volumes", "adhoc", "r3-red-squirrel.json"), "w", encoding="utf-8"), ensure_ascii=False)
    counts = {k: (len(v.get("body", [])), len(v.get("thermal", [])), len(v.get("structure", []))) for k, v in source["volumes"].items()}
    print("candidate", os.path.join(OUT, "🔣️.json"), counts)
    if "--apply" in sys.argv:
        json.dump(source, open(os.path.join(T, "volumes", "animals", "red-squirrel.json"), "w", encoding="utf-8"), ensure_ascii=False)
        apply(doc)
        print("applied root members", ROOT_MEMBERS, "to", CATALOG)


if __name__ == "__main__":
    main()
