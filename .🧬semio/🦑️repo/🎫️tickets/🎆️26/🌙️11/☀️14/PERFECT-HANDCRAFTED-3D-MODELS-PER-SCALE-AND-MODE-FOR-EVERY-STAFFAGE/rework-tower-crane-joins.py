"""🔗 Tower crane 1:50, 1:100, 1:200, second round pass 1: every part is carried by a real connection.

- jib tip sheave on a 60 mm axle between the two tip posts;
- the two rope falls start at the tangent points of the trolley sheaves (0.10 m higher) and end in the groove of the block sheave
  (0.20 m lower; 1:200: in the block body);
- 1:50 and 1:100: a 60 mm pin carries the block sheave between the two side plates;
- 1:50: the mast ladder is held by 25 mm brackets on the ties of the mast face, every 2 m;
- 1:200: the bail legs stand 0.70 m off the axis and bite into the skip shell (no lugs at this scale);
- structure: hook block plates, crosshead, pin, hook, skip shell, lugs and winch skids are members, so the load path is closed
  from the skip to the foundation.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
SOURCE = os.path.join(HERE, "volumes", "machinery", "tower-crane.json")
STEEL = {"material": "steel-s355", "role": "body", "tone": 0.5}


def main():
    src = json.load(open(SOURCE, encoding="utf-8"))
    for sc in ("50", "100", "200"):
        v = src["volumes"][sc]
        if any(s["id"] == "jib-tip-axle" for s in v["body"]):
            continue
        new = [{"id": "jib-tip-axle", "kind": "revolve", "bone": "jib", **STEEL, "profile": [[0, 0.03], [0.66, 0.03]], "axis": [0, 0, 1], "origin": [55, 0, -0.33]}]
        for s in v["body"]:
            if s["id"] in ("rope-1-a", "rope-1-b"):
                s["origin"][1] = 0.1
                s["profile"] = [[0, 0.007], [5.85, 0.007]]
            if s["id"] in ("rope-8-a", "rope-8-b") and sc != "200":
                s["profile"] = [[0, 0.007], [5.95, 0.007]]
            if sc == "200" and s["id"] in ("bail-leg-r", "bail-leg-l"):
                s["origin"][2] = -0.7 if s["id"].endswith("-r") else 0.7
            if sc == "200" and s["id"] == "bail-bar":
                s.update(origin=[0.05, 0.36, -0.72], profile=[[0, 0.025], [1.44, 0.025]])
        if sc != "200":
            new.append({"id": "block-sheave-pin", "kind": "revolve", "bone": "hoist-block", "material": "skip-steel", "role": "detail", "tone": 0.3, "profile": [[0, 0.03], [0.18, 0.03]], "axis": [0, 0, 1], "origin": [0, -0.2, -0.09]})
        if sc == "50":
            for side, z in (("r", -0.2), ("l", 0.2)):
                new.append({"id": f"mast-ladder-bracket-{side}", "kind": "revolve", "bone": "mast", "material": "steel-s355", "role": "body", "tone": 0.3, "texture": True, "profile": [[0, 0.0125], [0.37, 0.0125]], "axis": [0, 1, 0], "origin": [2, -0.92, z], "repeat": {"count": 19, "translation": [2, 0, 0]}})
        v["body"] += new
        ids = {s["id"] for s in v["body"]}
        region = {s.get("of"): s.get("region") for s in v["thermal"]}
        for s in new:
            reg = "lattice" if s["bone"] in ("jib", "mast") else region.get("block-sheave") or region.get("block-crosshead") or "skip"
            v["thermal"].append({"id": "th-" + s["id"], "of": s["id"], "region": reg})
        have = {s.get("of") for s in v["structure"]}
        for sid, member in (("jib-tip-axle", "jib"), ("block-plate-r", "hoist-block"), ("block-plate-l", "hoist-block"), ("block-sheave-pin", "hoist-block"), ("block-crosshead", "hoist-block"), ("hook", "hoist-block"), ("skip-body", "skip"), ("skip-lug-r", "skip"), ("skip-lug-l", "skip"), ("winch-skid-r", "winch"), ("winch-skid-l", "winch")):
            if sid in ids and sid not in have:
                v["structure"].append({"id": "member-" + sid, "of": sid, "member": member, "part": "member"})
    json.dump(src, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
