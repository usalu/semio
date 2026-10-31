"""Generic v1 -> v2 assembly for CV-VM assets. Each asset script supplies a spec dict; this module applies it and runs self-checks."""
import json, os, sys
from lib import *
from tcommon import *

HERE = os.path.dirname(os.path.abspath(__file__)) + "/"

REF_INCROPERA = {"id": "incropera-2011", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley. Ch. 7 and 9 (external and natural convection), Appendix A (thermophysical properties of steel, glass, rubber)."}
REF_ISO13732 = {"id": "iso-13732-1", "citation": "ISO 13732-1:2006, Ergonomics of the thermal environment: Methods for the assessment of human responses to contact with surfaces, Part 1: Hot surfaces (surface temperatures of machinery)."}

PRINT_PENS = {
    500: pens(0.15, None, None, 0.13, 0.25),
    200: pens(0.18, 0.13, None, 0.13, 0.25),
    100: pens(0.25, 0.18, None, 0.13, 0.35),
    50: pens(0.35, 0.25, 0.13, 0.18, 0.5),
}
RANGES = {500: (350, 1000), 200: (150, 350), 100: (75, 150), 50: (20, 75)}


def add_refs(a, refs):
    have = {r["id"] for r in a["provenance"]["references"]}
    for r in refs:
        if r["id"] not in have:
            a["provenance"]["references"].append(r)


def assemble(name, rel, spec):
    doc = json.load(open(HERE + "v1/%s.json" % name))
    a = doc["asset"]
    old_geo = a.pop("geometry")["elevation"]
    th = a["thermal"]
    a["thermal"] = {
        "environment": environment(th["environment"], spec["sky"], spec["ground"], spec.get("diffuse", 0.2)),
        "operating": spec["operating"],
        "regions": spec["regions"],
        "emitters": spec["emitters"],
        "passengers": [],
        "assumptions": spec["assumptions"],
    }
    add_refs(a, [REF_INCROPERA] + spec.get("refs", []))
    if "materials" in spec:
        a["materials"] = spec["materials"](a["materials"])
    for p in a["poses"]:
        ps = spec["poses"][p["id"]]
        p["thermal"] = binding(spec["defaults"], ps.get("regions"), ps["operating"], ps["emitters"], spec.get("contacts") if ps.get("grounded", True) else None)
    spec["fix_clips"](a["clips"])
    a["representations"] = {k: v for k, v in spec["reps"](a, old_geo).items()}
    a["symbols"] = spec.get("symbols", [])
    a["provenance"]["notes"] = spec["notes"]
    a["summary"] = spec.get("summary", a["summary"])
    a["tags"] = spec.get("tags", a["tags"])
    if "category" in spec:
        a["category"] = spec["category"]
    a["completion"] = "authored"
    order = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "presentation", "exports", "completion"]
    a = {k: a[k] for k in order}
    out = {"$schema": doc["$schema"], "version": "semio.staffage/v2", "asset": a}
    self_check(a)
    save(rel, out)
    return a


def self_check(a):
    ids = {r["id"] for r in a["thermal"]["regions"]}
    keys = {k["id"] for k in a["thermal"]["operating"]}
    bones = {b["id"] for b in a["rig"]["bones"]}
    dims = a["dimensions"]
    p0 = [p for p in a["poses"] if p["id"] == a["presentation"]["defaultPose"]][0]
    problems = []
    for p in a["poses"]:
        t = p["thermal"]
        if set(t["regions"]) != ids:
            problems.append("pose %s regions mismatch %s" % (p["id"], set(t["regions"]) ^ ids))
        if set(t["operating"]) != keys:
            problems.append("pose %s operating mismatch" % p["id"])
        if set(t["emitters"]) != {e["id"] for e in a["thermal"]["emitters"]}:
            problems.append("pose %s emitters mismatch" % p["id"])
    for r in a["thermal"]["regions"]:
        if r.get("operating") and r["operating"]["key"] not in keys:
            problems.append("region %s key" % r["id"])
        if r["material"] not in {m["id"] for m in a["materials"]}:
            problems.append("region %s material %s" % (r["id"], r["material"]))
    for den, rp in a["representations"].items():
        if rp["status"] != "authored":
            continue
        shapes = rp["geometry"]["elevation"]
        seen = set()
        sid = set()
        for s in shapes:
            if s["id"] in sid:
                problems.append("%s dup shape %s" % (den, s["id"]))
            sid.add(s["id"])
            if s["bone"] not in bones:
                problems.append("%s shape %s bone %s" % (den, s["id"], s["bone"]))
            for r in ([s["region"]] if s.get("region") else []) + s.get("regions", []):
                if r not in ids:
                    problems.append("%s shape %s region %s" % (den, s["id"], r))
                seen.add(r)
            if s.get("hatch") and s["hatch"] not in {h["id"] for h in rp["hatches"]}:
                problems.append("%s shape %s hatch" % (den, s["id"]))
            if rp["pens"].get(s["role"]) is None and not s.get("penMm"):
                problems.append("%s shape %s role %s has no pen" % (den, s["id"], s["role"]))
        miss = ids - seen
        if miss:
            problems.append("%s regions not drawn: %s" % (den, sorted(miss)))
        bb = bbox(a, shapes, p0)
        w, h = bb[2] - bb[0], bb[3] - bb[1]
        print("  %s bbox w %.3f (%.3f) h %.3f (%.3f) x %.2f..%.2f ymin %.3f shapes %d" % (den, w, dims["widthM"], h, dims["heightM"], bb[0], bb[2], bb[1], len(shapes)))
        for lab, v, ref in (("w", w, dims["widthM"]), ("h", h, dims["heightM"])):
            if abs(v - ref) > max(0.02 * ref, 0.015):
                problems.append("%s bbox %s %.3f vs %.3f" % (den, lab, v, ref))
    for p in problems:
        print("  PROBLEM", p)
    return problems


def previews(a, tag, poses=None, dens=(500, 200, 100, 50), print_ppm=24, zoom=(4,), view=None):
    for den in dens:
        rp = a["representations"][str(den)]
        if rp["status"] != "authored":
            continue
        for pid in poses or [a["presentation"]["defaultPose"]]:
            pose = [p for p in a["poses"] if p["id"] == pid][0]
            for z in (1,) + tuple(zoom):
                f = OUT + "%s-%s-%d-%s-x%d.png" % (tag, pid, den, "print", z)
                render(a, rp, pose, den, f, ppm=print_ppm * z, view=view)
                print("  wrote", f)
