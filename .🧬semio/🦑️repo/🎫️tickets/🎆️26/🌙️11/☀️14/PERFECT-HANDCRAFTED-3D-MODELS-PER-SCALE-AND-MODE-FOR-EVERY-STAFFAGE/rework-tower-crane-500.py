"""🔗 Tower crane 1:500, second round pass 1: the parts are joined and modelled for 75 mm.

Hand-set changes of the 1:500 lists only:
- hoist rope: one fall on the axis in all eight pieces (pieces 1 to 4 were two falls 0.4 m apart, 5 to 8 one fall, 186 mm beside them);
- trolley: 75 mm side plates, plain 280 mm wheel discs (no flange profile, no 58 mm axles), sheave block 0.36 m high so the rope ends in it;
- jib tip: a 75 mm axle carries the tip sheave between the two tip posts;
- skip: the bail legs stand 0.71 m off the axis and bite into the 0.68 m shell (no lugs at this scale);
- structure: the hook and the winch skids are members (hook block to bail, winch frame to the counter-jib chords).
"""
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "machinery", "tower-crane.json")
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json"


def main():
    src = json.load(open(SOURCE, encoding="utf-8"))
    v = src["volumes"]["500"]
    body = []
    for s in v["body"]:
        sid = s["id"]
        m = re.match(r"^rope-([1-4])-(a|b)$", sid)
        if m:
            if m.group(2) == "a":
                s.update(id=f"rope-{m.group(1)}", origin=[0, 0, 0])
                body.append(s)
            continue
        if sid.endswith("-axle") and sid.startswith("trolley-wheel-"):
            continue
        if sid in ("trolley-plate-r", "trolley-plate-l"):
            side = -1 if sid.endswith("-r") else 1
            s.update(size=[1.4, 0.64, 0.075], transform={"translation": [0, 0.1, side * 0.6925]})
        if re.match(r"^trolley-wheel-[rl]-\d$", sid):
            s["profile"] = [[0, 0.14], [0.086, 0.14]]
        if sid == "trolley-sheave-block":
            s.update(size=[1, 0.36, 0.112], transform={"translation": [0, -0.4, 0]})
        if sid in ("bail-leg-r", "bail-leg-l"):
            s["origin"][2] = -0.71 if sid.endswith("-r") else 0.71
        if sid == "bail-bar":
            s.update(origin=[0.05, 0.33, -0.7475], profile=[[0, 0.0375], [1.495, 0.0375]])
        body.append(s)
        if sid == "jib-tip-sheave":
            body.append({"id": "jib-tip-axle", "kind": "revolve", "bone": "jib", "material": "steel-s355", "role": "body", "tone": 0.5, "profile": [[0, 0.0375], [0.66, 0.0375]], "axis": [0, 0, 1], "origin": [55, 0, -0.33]})
    v["body"] = body
    ids = {s["id"] for s in body}
    for lst in ("thermal", "structure"):
        out = []
        for s in v[lst]:
            of = s.get("of")
            m = re.match(r"^rope-([1-4])-(a|b)$", of or "")
            if m:
                if m.group(2) == "a":
                    s.update(id=s["id"][:-2], of=f"rope-{m.group(1)}")
                    out.append(s)
                continue
            if of and of not in ids:
                print("dropped", lst, s["id"])
                continue
            out.append(s)
        v[lst] = out
    v["thermal"].append({"id": "th-jib-tip-axle", "of": "jib-tip-axle", "region": "lattice"})
    v["structure"] += [
        {"id": "member-jib-tip-axle", "of": "jib-tip-axle", "member": "jib", "part": "member"},
        {"id": "member-hook", "of": "hook", "member": "hoist-block", "part": "member"},
        {"id": "member-winch-skid-r", "of": "winch-skid-r", "member": "winch", "part": "member"},
        {"id": "member-winch-skid-l", "of": "winch-skid-l", "member": "winch", "part": "member"},
    ]
    json.dump(src, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    doc = json.load(open(CATALOG, encoding="utf-8"))
    for view, shapes in doc["asset"]["representations"]["500"]["geometry"].items():
        for s in shapes:
            if s.get("bone", "").startswith("rope-"):
                print(view, json.dumps({k: x for k, x in s.items() if k != "title"})[:300])


if __name__ == "__main__":
    main()
