"""🎹 Authoritative author script of the used upright piano: every hand-chosen number of the asset lives here (geometry in `author_parts.py`, texts in `author_texts.py`) and the script writes `🔣️.json` next to it.

Run: `python3 author-piano.py` (prints the mass and centre-of-mass table of the report with the `[DEBUG] ` prefix).

See also ../../brief.md and ../../reports/used-upright-piano.md.
"""
import json
import os

from author_geom import T, r6, clean
import author_parts as P
import author_texts as X

HERE = os.path.dirname(os.path.abspath(__file__))
SCALES = ("50", "100", "200", "500")

BODY = {s: P.body(s) for s in SCALES}


def masses():
    """⚖️ Bone masses and centres of mass (bone-local u, v) from the 1:50 solids, section x density."""
    dens = {m["id"]: m["densityKgM3"] for m in P.MATERIALS}
    out = {}
    for s in BODY["50"]:
        m = s["_vol"] * dens[s["material"]]
        e = out.setdefault(s["bone"], [0.0, 0.0, 0.0])
        e[0] += m
        e[1] += m * s["_c"][0]
        e[2] += m * s["_c"][1]
    return {b: (v[0], v[1] / v[0], v[2] / v[0]) for b, v in out.items()}


MASS = masses()
TOTAL = sum(v[0] for v in MASS.values())
PIANO_BONES = ("case", "lid", "fallboard", "pedal-soft", "pedal-mute", "pedal-sustain")
PIANO_KG = sum(MASS[b][0] for b in PIANO_BONES)


def piano_com():
    """🎯 Centre of mass of the piano alone in the case frame (x from the back, y from the castor contact)."""
    off = {"case": (0, 0), "lid": (0, P.LID_Y0), "fallboard": P.FALL_HINGE, "pedal-soft": P.PEDAL_PIVOT, "pedal-mute": P.PEDAL_PIVOT, "pedal-sustain": P.PEDAL_PIVOT}
    m = sum(MASS[b][0] for b in PIANO_BONES)
    x = sum(MASS[b][0] * (MASS[b][1] + off[b][0]) for b in PIANO_BONES) / m
    y = sum(MASS[b][0] * (MASS[b][2] + off[b][1]) for b in PIANO_BONES) / m
    return x, y


COM_X, COM_Y = piano_com()
TIP_FORCE = PIANO_KG * P.G * (P.CASTOR_X[1] - COM_X) / P.H


def mass_entry(bone, ref):
    """🏋️ The mass record of a bone."""
    kg, cx, cy = MASS[bone]
    return {"kg": r6(kg), "comX": r6(cx), "comY": r6(cy), "distribution": "point", "source": "authored-estimate", "reference": ref}


def bones():
    """🦴 The rig with masses from the 1:50 solids."""
    out = []
    for b in P.BONES:
        b = dict(b)
        if b["id"] in MASS:
            b["mass"] = mass_entry(b["id"], P.MASS_REF[b["id"]])
        out.append(b)
    return out


def ref(sid, of, **kw):
    """🔗 A reference solid of the thermal or structure list."""
    s = {"id": sid, "of": of}
    s.update(kw)
    return s


def thermal_list(scale):
    """🔥 Thermal list: references of the exposed body solids with their region, own closing solids where the body has openings that are closed in reality, own solids of the hidden keyboard."""
    out = []
    for s in BODY[scale]:
        reg = P.region_of(s["id"])
        if reg:
            out.append(ref(f"th-{s['id']}", s["id"], region=reg))
    out += P.thermal_extra(scale)
    return out


def structure_list(scale):
    """🏗️ Structure list: the boards, beams, frame and blocks as members of their bones, the cheek blocks as joints of keybed and sides, mass markers at the computed centres of mass."""
    r = {"50": 0.0045, "100": 0.009, "200": 0.018, "500": 0.045}[scale]
    out = []
    for s in BODY[scale]:
        sid = s["id"]
        if not P.structural(sid):
            continue
        part = "joint" if sid.startswith("cheek") else "member"
        out.append(ref(f"st-{sid}", sid, member=s["bone"], part=part))
    out += P.structure_extra(scale)
    for b in MASS:
        kg, cx, cy = MASS[b]
        z = P.BONE_Z.get(b, 0.0)
        out.append({"id": f"mass-{b}", "kind": "ellipsoid", "bone": b, "material": P.MASS_MATERIAL[b], "role": "detail", "tone": 0.3, "radii": [r, r, r], "transform": {"translation": [r6(cx), r6(cy), 0.0]}, "member": b, "part": "mass"})
    return out


def volume(scale):
    """🧊 One authored volume."""
    return {"status": "authored", "denominator": int(scale), "body": clean(BODY[scale]), "thermal": clean(thermal_list(scale)), "structure": clean(structure_list(scale)), "decisions": X.VOL_DECISIONS[scale]}


def representation(scale):
    """🖨️ One authored representation."""
    lo, hi = P.RANGES[scale]
    return {"status": "authored", "denominator": int(scale), "validRange": {"minDenominator": lo, "maxDenominator": hi}, "pens": P.PENS[scale], "hatches": [], "geometry": {"elevation": P.elevation(scale), "front": P.front(scale), "plan": P.plan(scale)}, "decisions": X.REP_DECISIONS[scale]}


AREA_FILE = os.path.join(HERE, "areas.json")
AREAS = dict(P.AREAS)
if os.path.exists(AREA_FILE):
    AREAS.update(json.load(open(AREA_FILE)))

FACTS = {"piano": PIANO_KG, "total": TOTAL, "comx": COM_X, "comy": COM_Y, "tip": TIP_FORCE, "mass": MASS}

ASSET = {
    "id": "used-upright-piano",
    "category": "objects",
    "title": T("Used Upright Piano", "Gebrauchtes Klavier"),
    "summary": X.SUMMARY,
    "tags": ["used", "household", "second-hand", "living", "piano", "upright-piano", "musical-instrument", "keyboard", "walnut-veneer", "chipboard", "cast-iron", "piano-dolly", "ratchet-strap", "bench", "interior"],
    "provenance": {"author": "Semio staffage content (used household objects)", "method": "handcrafted", "created": "2026-11-22", "references": P.REFERENCES, "notes": X.PROVENANCE_NOTES},
    "dimensions": {"heightM": r6(P.DIM_H), "widthM": r6(P.DIM_W), "depthM": r6(P.DIM_D), "massKg": round(TOTAL, 3)},
    "materials": P.MATERIALS,
    "rig": {"kind": "mechanical", "root": "floor", "bones": bones()},
    "anchors": P.ANCHORS,
    "representations": {s: representation(s) for s in ("500", "200", "100", "50")},
    "symbols": [],
    "poses": X.poses(FACTS),
    "clips": X.clips(),
    "thermal": X.thermal(P.regions(AREAS)),
    "structure": X.structure(FACTS),
    "volumes": {s: volume(s) for s in SCALES},
    "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": ASSET}

if __name__ == "__main__":
    with open(os.path.join(HERE, "🔣️.json"), "w", encoding="utf-8") as f:
        json.dump(DOC, f, ensure_ascii=False, indent=1)
        f.write("\n")
    for b, (kg, cx, cy) in MASS.items():
        print(f"[DEBUG] mass {b}: {kg:.3f} kg at ({cx:.3f}, {cy:.3f})")
    print(f"[DEBUG] piano {PIANO_KG:.2f} kg, com x {COM_X:.3f} y {COM_Y:.3f}, tip force {TIP_FORCE:.0f} N; asset {TOTAL:.2f} kg")
    print("[DEBUG]", {s: (len(BODY[s]), len(thermal_list(s)), len(structure_list(s))) for s in SCALES})
