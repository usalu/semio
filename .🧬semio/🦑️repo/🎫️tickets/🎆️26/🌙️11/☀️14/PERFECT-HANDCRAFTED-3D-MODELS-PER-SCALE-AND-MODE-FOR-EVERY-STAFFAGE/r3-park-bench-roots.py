"""🌱 Round 3, furniture/park-bench: the root corrections outside `volumes` and the 2D: rig (end frames as vertical columns from the foot to the post top, seat and backrest on the beam lines of their slats, the floor reference without length, masses from the model), anchors in the new end-frame frames, total mass, thermal layers and areas of the modelled slats, references and notes.
"""
import importlib.util
import os

from shapely.geometry import Polygon

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_park_bench_" + name, os.path.join(HERE, f"r3-park-bench-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")
L = _load("lists")


def T(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


def slat_area(sections):
    """📐 The outer area of oak slats 1.80 m long: perimeter times length plus both end faces."""
    return sum(Polygon(s).length * K.LENGTH + 2 * Polygon(s).area for s in sections)


def rig(old):
    """🦴 The corrected rig."""
    m = L.masses()
    sy, by = K.seat_y(), K.back_y()
    bones = []
    for bone in old["bones"]:
        bone = dict(bone)
        bid = bone["id"]
        if bid == "ground":
            bone["lengthM"] = 0
        elif bid in ("end-left", "end-right"):
            kg, x, y, z = m[bid]
            bone.update(x=x, y=0, angleDeg=90, lengthM=round(K.post_front(K.POST_TOP)[1], 2))
            bone["mass"] = dict(bone["mass"], kg=kg, comX=round(y, 3), comY=0)
        elif bid == "seat":
            kg, x, y, z = m[bid]
            bone["y"] = sy
            bone["mass"] = dict(bone["mass"], kg=kg, comX=0.9, comY=round(y - sy, 3) + 0.0)
        elif bid == "backrest":
            kg, x, y, z = m[bid]
            bone["y"] = by
            bone["mass"] = dict(bone["mass"], kg=kg, comX=0.9, comY=round(y - by, 3) + 0.0)
        bones.append(bone)
    return dict(old, bones=bones)


def anchors(old):
    """⚓ The foot contacts at the joint of each end frame (the floor under the frame plane); footprint: the 70 mm pad width along the bench by the span of the front and the rear pad."""
    out = []
    for a in old:
        a = dict(a)
        if a["id"] in ("contact-left", "contact-right"):
            a.update(x=0, y=0, footprintLengthM=K.PAD_X, footprintWidthM=round(K.REAR_FOOT_Z - K.FRONT_Z + K.PAD_L, 3))
        out.append(a)
    return out


STEEL_AREA = 1.22


def thermal(th):
    """🌡 Layers at the modelled thicknesses and the areas of the modelled parts (slats: perimeter times length plus ends; steel: the outer area of the 1:50 casting, pads, armrests and bolt heads as `volumes-check.ts --product` measures it, 1.216 m2); all within a factor 1.5 of the old values (C16)."""
    seat = round(slat_area(K.seat_slats()), 2)
    back = round(slat_area(K.back_slats()), 2)
    regions = []
    for r in th["regions"]:
        r = dict(r)
        if r["id"] == "seat-timber":
            r.update(areaM2=seat, characteristicLengthM=0.43)
            r["layers"] = [dict(r["layers"][0], thicknessM=K.SEAT_T, resistanceM2KW=0.25)]
        elif r["id"] == "back-timber":
            r.update(areaM2=back, characteristicLengthM=0.36)
            r["layers"] = [dict(r["layers"][0], thicknessM=K.BACK_T, resistanceM2KW=0.194444)]
        elif r["id"] == "steel-frame":
            r.update(areaM2=STEEL_AREA, characteristicLengthM=0.04)
        regions.append(r)
    assumptions = list(th["assumptions"])
    assumptions[1] = T("Areal heat capacity is rho*cp*min(thickness, sqrt(alpha*3600 s)): the part of the section that takes part within one hour (oak slats 45 and 35 mm: 24 mm; cast steel wall 10 mm: 10 mm). Conductivity, cp and density after ISO 10456.",
                       "Die flächenbezogene Wärmekapazität ist rho*cp*min(Dicke, sqrt(alpha*3600 s)): der Teil des Querschnitts, der binnen einer Stunde teilnimmt (Eichenlatten 45 und 35 mm: 24 mm; Stahlgusswand 10 mm: 10 mm). Leitfähigkeit, cp und Dichte nach ISO 10456.")
    assumptions[3] = T("Heat flow into the ground through the four 70 x 70 mm foot pads (0.4 percent of the area) is neglected.",
                       "Der Wärmestrom über die vier Fußplatten 70 x 70 mm (0,4 Prozent der Fläche) in den Boden wird vernachlässigt.")
    return dict(th, regions=regions, assumptions=assumptions)


REFERENCES = [
    {"id": "bs-8300-1-2018", "citation": "BS 8300-1:2018 Design of an accessible and inclusive built environment, Part 1: External environment, Code of practice (public seating: seat height about 450-480 mm, backrest at least 300 mm above the seat, armrests about 200 mm above the seat)."},
    {"id": "inclusive-mobility-2021", "citation": "Department for Transport (2021). Inclusive Mobility: A Guide to Best Practice on Access to Pedestrian and Transport Infrastructure, section 13 Seating (seat height 470-480 mm, seat depth, armrests and backrests of public benches)."},
    {"id": "cast-bench-catalogues", "citation": "Street-furniture catalogues of cast-iron and timber benches, e.g. CIS Street Furniture Type 512 Eastgate (1800 x 570 mm, 490 mm high to the seat), Kronemag traditional bench 700 (1800 mm long, seat 440 mm); armrested 1.8 m public benches of 0.57-0.70 m depth and 0.80-0.90 m height with 40-50 mm hardwood slats bolted to cast end frames by carriage bolts."},
]


def provenance(p):
    """📚 References and notes."""
    have = {r["id"] for r in p["references"]}
    refs = list(p["references"]) + [r for r in REFERENCES if r["id"] not in have]
    notes = T("Front elevation along the bench length. Construction after cast-iron park benches of street-furniture catalogues and the seating dimensions of BS 8300-1 and Inclusive Mobility: seat 0.45 m high at the front and 0.43 m deep, raked 5 degrees; back slats to 0.85 m on posts raked 8 degrees (103 degrees between seat and back); armrest 0.18 m above the seat front; two one-piece cast end frames of I-section 0.12 m in from the slat ends, four 100 x 45 mm oak seat slats and three 80 x 35 mm oak back slats held by M8 carriage bolts. Masses from the modelled volumes (steel 7850, oak 700 kg/m3); occupant masses are authored estimates (75 kg adult). v3: rig, 2D and 3D rebuilt on this construction.",
              "Vorderansicht entlang der Banklänge. Konstruktion nach gusseisernen Parkbänken aus Stadtmöblierungskatalogen und den Sitzmaßen von BS 8300-1 und Inclusive Mobility: Sitz vorne 0,45 m hoch und 0,43 m tief, um 5 Grad geneigt; Rückenlatten bis 0,85 m auf um 8 Grad geneigten Pfosten (103 Grad zwischen Sitz und Lehne); Armlehne 0,18 m über der Sitzvorderkante; zwei einteilige Endgestelle im I-Querschnitt 0,12 m von den Lattenenden, vier Eichen-Sitzlatten 100 x 45 mm und drei Eichen-Rückenlatten 80 x 35 mm mit M8-Schlossschrauben befestigt. Massen aus den modellierten Volumen (Stahl 7850, Eiche 700 kg/m3); Personenmassen sind Schätzungen (75 kg Erwachsene). v3: Rig, 2D und 3D auf dieser Konstruktion neu aufgebaut.")
    return dict(p, references=refs, notes=notes)


def structure(st):
    """🏗 The structural assumptions."""
    a = list(st["assumptions"])
    a[0] = T("The bench is a rigid body on two foot contacts; each cast end frame stands on a front and a rear foot pad and carries the seat slats on its raked rail and the back slats on its back post.",
             "Die Bank ist ein starrer Körper auf zwei Fußkontakten; jedes gegossene Endgestell steht auf einer vorderen und einer hinteren Fußplatte und trägt die Sitzlatten auf seiner geneigten Zarge und die Rückenlatten auf seinem Rückenpfosten.")
    a[2] = T("Slat gaps, eased edges and bolt heads are geometric detail of the 1:50 and 1:100 drawings only; the structural model is unchanged by the drawing scale.",
             "Lattenfugen, gebrochene Kanten und Schraubenköpfe sind nur Detail der Zeichnungen 1:50 und 1:100; das Tragmodell hängt nicht vom Zeichnungsmaßstab ab.")
    return dict(st, assumptions=a)


def apply(asset):
    """🖊 Applies every root correction to the asset in memory."""
    asset["rig"] = rig(asset["rig"])
    asset["anchors"] = anchors(asset["anchors"])
    asset["dimensions"] = dict(asset["dimensions"], massKg=round(sum(v[0] for v in L.masses().values()), 1))
    asset["thermal"] = thermal(asset["thermal"])
    asset["provenance"] = provenance(asset["provenance"])
    asset["structure"] = structure(asset["structure"])


if __name__ == "__main__":
    print("seat area", slat_area(K.seat_slats()), "back area", slat_area(K.back_slats()))
