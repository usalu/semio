"""🏗️ Builds round 3 of `machinery/tower-crane`: the `volumes` source of all four scales (assemblies from `r3-tower-crane-parts.py`, the mast, tower head and winch house carried over from round 2 where they were already right) and the candidate asset document with the root fixes (2D shapes of the changed assemblies at every scale and viewpoint, dimensions, masses, symbol, layer order, region areas).

Usage: python3 r3-tower-crane-build.py            writes T/volumes/machinery/tower-crane.json and 🗑️generated/r3-tower-crane/🔣️.json
@see ./r3-tower-crane-parts.py
@see ./r3-machinery-tower-crane.md
"""
import copy
import json
import os
from importlib import util

HERE = os.path.dirname(os.path.abspath(__file__))
_s = util.spec_from_file_location("parts", os.path.join(HERE, "r3-tower-crane-parts.py"))
Q = util.module_from_spec(_s)
_s.loader.exec_module(Q)
K = Q.K
CATALOG = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json"
SOURCE = os.path.join(HERE, "volumes/machinery/tower-crane.json")
ROUND2 = os.path.join(HERE, "🗑️generated/r3-tower-crane/round2-volumes.json")
CANDIDATE = os.path.join(HERE, "🗑️generated/r3-tower-crane/🔣️.json")

AREAS = json.load(open(os.path.join(HERE, "r3-tower-crane-areas.json"))) if os.path.exists(os.path.join(HERE, "r3-tower-crane-areas.json")) else {}
REPLACED_BONES = {"base", "jib", "counter-jib", "counterweight", "trolley", "hoist-block", "skip"} | {f"rope-{n}" for n in range(1, 9)}
REPLACED_SLEWING = ("slew-ring", "slew-platform", "slew-motor", "cab", "pendant")


def kept_solid(s, sc):
    if s.get("bone") == "mast":
        return sc == 500 or s["id"].startswith("mast-ladder")
    if s.get("bone") == "winch":
        return True
    return s.get("bone") == "slewing" and (s["id"].startswith("tower-head-") or s["id"] == "head-sheave")


def kept_shape(s, sc):
    if s["id"] == "winch-skids":
        return False
    if s["bone"] == "hoist-block" and s["id"] == "hook" and sc != 500:
        return True
    if s["bone"] == "mast":
        return sc == 500 or s["kind"] == "rect"
    if s["bone"] in REPLACED_BONES:
        return False
    if s["bone"] == "slewing" and s["id"].startswith(REPLACED_SLEWING):
        return False
    return True


DECISIONS = {int(k): v for k, v in json.load(open(os.path.join(HERE, "r3-tower-crane-decisions.json"), encoding="utf-8")).items() if k.isdigit()}


def slewing_members(part, body, sc):
    """🗼 The load path of the slewing unit: every tower-head bar of the body as a member (a repeat family as one explicit member per copy, so each drawn bar has its member) and the three pendant ropes as tension members (B1, D3)."""
    out = []
    for s in body:
        if s.get("bone") != "slewing" or not s["id"].startswith("tower-head-"):
            continue
        rep = s.get("repeat")
        if rep is None:
            out.append({"id": "member-" + s["id"], "of": s["id"], "member": "slewing", "part": "member"})
            continue
        for k in range(rep["count"]):
            c = copy.deepcopy(s)
            c.pop("repeat")
            c.pop("texture", None)
            c["origin"] = [K.r(c["origin"][i] + k * rep.get("translation", [0, 0, 0])[i]) for i in range(3)]
            c["id"] = f"member-{s['id']}-{k + 1}"
            c.update({"member": "slewing", "part": "member"})
            out.append(c)
    for s in part.body:
        if s["id"].startswith("pendant"):
            out.append({"id": "member-" + s["id"], "of": s["id"], "member": "slewing", "part": "member"})
    return out


def scaled(s, sc):
    """📏 Joint, mass and support markers of the structure sized for the scale: grown by half a smallest feature so that each scale states its own (B3, B5)."""
    if s.get("part") not in ("joint", "mass", "support") or "of" in s:
        return s
    f = K.FEAT[sc] / 2
    s = copy.deepcopy(s)
    if s["kind"] == "ellipsoid":
        s["radii"] = [K.r(x + f) for x in s["radii"]]
    elif s["kind"] == "box":
        s["size"] = [K.r(x + f) for x in s["size"]]
    elif s["kind"] == "revolve":
        s["profile"] = [[q[0], K.r(q[1] + f)] for q in s["profile"]]
    elif s["kind"] == "extrude":
        s["to"] = K.r(s["to"] + f)
    return s


def build():
    if not os.path.exists(ROUND2):
        os.makedirs(os.path.dirname(ROUND2), exist_ok=True)
        json.dump(json.load(open(CATALOG, encoding="utf-8"))["asset"]["volumes"], open(ROUND2, "w"), ensure_ascii=False)
    old = json.load(open(ROUND2, encoding="utf-8"))
    vols, views = {}, {}
    for sc in K.DEN:
        o = old[str(sc)]
        keep_ids = {s["id"] for s in o["body"] if kept_solid(s, sc)}
        body = [s for s in o["body"] if s["id"] in keep_ids]
        thermal = [s for s in o["thermal"] if (s.get("of") in keep_ids) or ("of" not in s and kept_solid(s, sc))]
        structure = [s for s in o["structure"] if (s.get("of") in keep_ids) or ("of" not in s and (s.get("bone") == "winch" or (s.get("bone") == "mast" and sc == 500)) and s["id"] not in ("mass-winch",)) or s["id"] == "mass-winch" or (s.get("member") == "slewing" and s.get("of", "").startswith("tower-head"))]
        hook = next(s for s in o["body"] if s["id"] == "hook")
        part = Q.Part()
        for p in ((Q.mast(sc),) if sc != 500 else ()) + (Q.base(sc), Q.slewing(sc), Q.cab(sc), Q.jib(sc), Q.counter_jib(sc), Q.pendants(sc), Q.counterweight(sc), Q.trolley(sc), Q.block(sc, hook), Q.skip(sc), Q.ropes(sc), Q.winch_skids(sc)):
            part.add(p)
        body += part.body
        thermal = [s for s in thermal if not (s.get("region") == "winch-house" and s.get("of", "").startswith("winch-house-"))] + Q.winch_house_thermal(sc)
        thermal += part.thermal
        if sc == 500:
            envelopes = {s["id"] for s in body if s.get("envelope")}
            thermal = [s for s in thermal if s.get("region") != "lattice" or s.get("of") in envelopes]
        structure = [s for s in structure if s.get("member") != "slewing" or not (s["id"].startswith("member-tower-head") or s["id"].startswith("member-slewing-extra"))]
        structure += slewing_members(part, body, sc)
        part.structure = [scaled(s, sc) for s in part.structure]
        if sc == 500:
            for v in part.views.values():
                for s in v:
                    if s.get("role") == "detail" and s["kind"] != "member":
                        s["role"] = "body"
        seen = set()
        structure = [s for s in structure + part.structure if not (s["id"] in seen or seen.add(s["id"]))]
        vols[str(sc)] = {"status": "authored", "denominator": sc, "body": body, "thermal": thermal, "structure": structure, "decisions": DECISIONS[sc]}
        views[sc] = part.views
    json.dump({"volumes": vols}, open(SOURCE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    return vols, views


def roots(vols, views):
    doc = json.load(open(CATALOG, encoding="utf-8"))
    a = doc["asset"]
    a["dimensions"]["depthM"] = 7.0
    bones = {b["id"]: b for b in a["rig"]["bones"]}
    bones["base"]["mass"].update({"comY": 1.05})
    bones["base"]["title"] = {"en": "Cross-base with ballast", "de": "Kreuzrahmen mit Ballast"}
    for r_ in a["thermal"]["regions"]:
        if r_["id"] == "cab-glazing":
            r_["layers"] = [next(l for l in r_["layers"] if l["id"] == i) for i in ("pane-in", "gap", "pane-out")]
        if r_["id"] in AREAS:
            r_["areaM2"] = AREAS[r_["id"]]
        if r_["id"] == "ballast":
            r_["title"] = {"en": "Concrete ballast blocks", "de": "Betonballastblöcke"}
        if r_["id"] == "pendants":
            r_["characteristicLengthM"] = 0.016
        if r_["id"] == "rope":
            r_["characteristicLengthM"] = 0.014
    for an in a["anchors"]:
        if an["id"] == "base-fixed":
            an["title"] = {"en": "Cross-base on its four pads, fully fixed", "de": "Kreuzrahmen auf seinen vier Platten, voll eingespannt"}
    extra = json.load(open(os.path.join(HERE, "r3-tower-crane-decisions.json"), encoding="utf-8"))["2d"]
    for sc in K.DEN:
        dec = a["representations"][str(sc)]["decisions"]
        for key, text in extra.items():
            for lang in ("en", "de"):
                if text[lang].strip() not in dec[key][lang]:
                    dec[key][lang] = dec[key][lang].rstrip() + text[lang]
        dec.update(json.load(open(os.path.join(HERE, "r3-tower-crane-decisions.json"), encoding="utf-8"))["2d-set"][str(sc)])
        g = a["representations"][str(sc)]["geometry"]
        for v in ("elevation", "front", "plan"):
            g[v] = [s for s in g[v] if kept_shape(s, sc)] + views[sc][v]
    sym = a["symbols"][0]["geometry"]
    fix = {
        "elevation-ballast": {"x": -3.5, "y": 0, "w": 7, "h": 1.5},
        "front-ballast": {"x": -3.5, "y": 0, "w": 7, "h": 1.5},
        "plan-ballast": {"x": -3.5, "y": -3.5, "w": 7, "h": 7},
        "elevation-counterweight": {"x": -15.7, "y": 41.5, "w": 1.6, "h": 2.22},
        "plan-counterweight": {"x": -15.7, "y": -1.25, "w": 1.6, "h": 2.5},
        "front-counterweight": {"x": -1.25, "y": 41.5, "w": 2.5, "h": 2.22},
        "elevation-cab": {"x": 1.45, "y": 41.2, "w": 1.85, "h": 2.1},
        "front-cab": {"x": -2.25, "y": 41.2, "w": 1.4, "h": 2.1},
        "plan-cab": {"x": 1.45, "y": -2.25, "w": 1.85, "h": 1.4},
        "front-jib": {"x": -0.66, "y": 43.59, "w": 1.32, "h": 1.62},
        "plan-jib": {"x": 0.56, "y": -0.66, "w": 54.44, "h": 1.32},
        "elevation-jib": {"x": 0.56, "y": 43.59, "w": 54.44, "h": 1.62},
        "elevation-counter-jib": {"x": -16.5, "y": 43.7, "w": 15.715, "h": 1.4},
        "plan-counter-jib": {"x": -16.5, "y": -0.7, "w": 15.715, "h": 1.4},
        "elevation-trolley": {"x": 49.3, "y": 43.1, "w": 1.4, "h": 1.07},
        "front-trolley": {"x": -0.73, "y": 43.1, "w": 1.46, "h": 1.07},
        "plan-trolley": {"x": 49.3, "y": -0.73, "w": 1.4, "h": 1.46},
    }
    for v, shapes in sym.items():
        for s in shapes:
            if s["id"] in fix:
                s.update(fix[s["id"]])
            if s["id"] == "elevation-pendant-fore":
                s["points"] = [[0, 48.6], [40.55, 45.21]]
            if s["id"] == "elevation-pendant-back":
                s["points"] = [[0, 48.6], [-16.47, 45.09]]
    a["volumes"] = vols
    os.makedirs(os.path.dirname(CANDIDATE), exist_ok=True)
    json.dump(doc, open(CANDIDATE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    return doc


if __name__ == "__main__":
    v, w = build()
    roots(v, w)
    for sc in K.DEN:
        x = v[str(sc)]
        print(sc, {k: len(x[k]) for k in ("body", "thermal", "structure")}, {k: len(w[sc][k]) for k in w[sc]})
