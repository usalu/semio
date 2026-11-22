#!/usr/bin/env python3
"""🎺 The authoritative author script of `used-marching-snare-and-bass-drum`: writes `🔣️.json` next to it from the numbers of `author_parts.py`, the coarse scales of `author_scales.py`, the drawings of `author_drawing.py` and the texts, thermal model, rig, poses and clips stated here.

Run: `python3 author-marching-drums.py` (prints the derived masses and extents).

@see ../../brief.md — the binding brief of the wave
@see ../../reports/used-marching-snare-and-bass-drum.md — reference table and decisions
"""
import json
import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from author_drawing import draw_scale
from author_geometry import Ring, Tube, r6
from author_parts import BASS_LIFT, BONES, FRAMES, SNARE_LIFT, STICK_LIFT, STICK_RAISED, STICK_STRIKE, XS, YB, parts_50
from author_scales import parts_100, parts_200, parts_500

HERE = os.path.dirname(os.path.abspath(__file__))
SCALES = {50: parts_50, 100: parts_100, 200: parts_200, 500: parts_500}
FEATURE = {50: 0.0075, 100: 0.015, 200: 0.03, 500: 0.075}


def T(en, de):
    """🌍 A bilingual text, English first."""
    return {"en": en, "de": de}


MATERIALS = [
    ("steel-coated", "Powder-coated steel tube, chipped", "Pulverbeschichtetes Stahlrohr, abgeplatzt", 0.9, 0.6, 7850, 50, 460, "literature", "incropera-2018"),
    ("rubber", "Rubber caps and sleeves, hardened", "Gummikappen und -hülsen, verhärtet", 0.93, 0.9, 1150, 0.16, 1700, "literature", "incropera-2018"),
    ("birch-lacquer", "Birch plywood shell, white lacquer crazed (aged: emissivity as new)", "Birkensperrholzkessel, weißer Lack mit Haarrissen (gealtert: Emissionsgrad wie neu)", 0.9, 0.35, 680, 0.17, 1600, "literature", "iso-10456-2007"),
    ("calfskin", "Calfskin head, stained (aged rawhide)", "Kalbfell, fleckig (gealterte Rohhaut)", 0.95, 0.5, 1000, 0.17, 1500, "authored-estimate", "rossing-2000"),
    ("beech-lacquer", "Lacquered beech counter hoop", "Lackierter Buchen-Spannreifen", 0.9, 0.5, 720, 0.18, 1600, "literature", "iso-10456-2007"),
    ("zinc-nickel", "Nickel-plated zinc die-cast lugs, tarnished (aged 0.15, new 0.05)", "Vernickelte Zinkdruckguss-Böckchen, angelaufen (gealtert 0,15, neu 0,05)", 0.15, 0.4, 6600, 110, 420, "authored-estimate", "incropera-2018"),
    ("steel-nickel", "Nickel-plated steel claws, rods and T-handles, tarnished (aged 0.15, new 0.05)", "Vernickelte Stahlkrallen, Spannschrauben und Flügel, angelaufen (gealtert 0,15, neu 0,05)", 0.15, 0.4, 7850, 50, 460, "authored-estimate", "incropera-2018"),
    ("aluminium", "Black anodised aluminium, worn bare at the edges (aged 0.7, new 0.8)", "Schwarz eloxiertes Aluminium, an den Kanten blank gerieben (gealtert 0,7, neu 0,8)", 0.7, 0.75, 2700, 200, 900, "authored-estimate", "incropera-2018"),
    ("foam-pad", "Polyethylene foam in nylon cover, compressed", "Polyethylenschaum in Nylonbezug, zusammengedrückt", 0.9, 0.8, 90, 0.045, 1500, "authored-estimate", "iso-10456-2007"),
    ("birch-ply", "Six-ply birch shell with pearl wrap", "Sechslagiger Birkenkessel mit Perlmuttfolie", 0.9, 0.4, 680, 0.17, 1600, "literature", "iso-10456-2007"),
    ("pvc-wrap", "Celluloid-type pearl wrap, yellowed and faded", "Perlmuttfolie (Zelluloid-Art), vergilbt und verblasst", 0.9, 0.4, 1380, 0.19, 1000, "authored-estimate", "incropera-2018"),
    ("pet-coated", "Coated polyester head film, coating worn at the beating spot", "Beschichtete Polyesterfolie, Beschichtung am Schlagpunkt abgespielt", 0.9, 0.3, 1390, 0.15, 1170, "literature", "incropera-2018"),
    ("chrome-steel", "Chrome-plated steel, pitted (aged 0.12, polished new 0.07)", "Verchromter Stahl, mit Lochfraß (gealtert 0,12, poliert neu 0,07)", 0.12, 0.4, 7850, 50, 460, "authored-estimate", "incropera-2018"),
    ("chrome-brass", "Chrome-plated brass tube lugs (aged 0.12)", "Verchromte Messing-Rohrböckchen (gealtert 0,12)", 0.12, 0.4, 8500, 110, 380, "authored-estimate", "incropera-2018"),
    ("steel-wire", "Snare wires, tarnished carbon steel", "Teppichspiralen, angelaufener Kohlenstoffstahl", 0.3, 0.6, 7850, 50, 460, "authored-estimate", "incropera-2018"),
    ("hickory", "Hickory sticks, lacquer worn at the grip", "Hickorystöcke, Lack am Griff abgegriffen", 0.9, 0.5, 830, 0.16, 1600, "literature", "fpl-2010"),
]
DENSITY = {m[0]: m[5] for m in MATERIALS}

REFERENCES = [
    ("hornbostel-sachs-1914", "von Hornbostel, E. M., Sachs, C. (1914). Systematik der Musikinstrumente. Ein Versuch. Zeitschrift für Ethnologie 46, 553-590 (membranophones 2; struck drums 21; double-skin cylindrical drums 211.212; snare drum with snares 211.212.11 with snare added, bass drum 211.212.1)."),
    ("blades-1992", "Blades, J. (1992). Percussion Instruments and their History, revised ed., Bold Strummer, Westport (side and military drums, the long and the bass drum, rod tension, counter hoops and snares)."),
    ("beck-2007", "Beck, J. H. (ed.) (2007). Encyclopedia of Percussion, 2nd ed., Routledge, New York (marching percussion: snare drums of 14 x 12 in, bass drums of 22 to 28 in, carriers and slings)."),
    ("rossing-2000", "Rossing, T. D. (2000). Science of Percussion Instruments, World Scientific, Singapore (membrane tension of drum heads, head and shell modes, calfskin and polyester heads)."),
    ("din-33402-2", "DIN 33402-2:2020-12. Ergonomie - Körpermaße des Menschen - Teil 2: Werte (standing shoulder height about 1.45 m and waist height about 1.03 m of adults, used for the carrier heights)."),
    ("iso-10456-2007", "EN ISO 10456:2007. Building materials and products - Hygrothermal properties - Tabulated design values (plywood, hardwood, polyethylene foam)."),
    ("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities of polished, plated and oxidised metals, thermal properties of metals and polymers)."),
    ("fpl-2010", "Forest Products Laboratory (2010). Wood Handbook - Wood as an Engineering Material, General Technical Report FPL-GTR-190, USDA (hickory 830 kg/m3 at 12 % moisture, beech, birch)."),
    ("catalogue-marching", "Catalogue-typical data of second-hand marching percussion of the 1970s to 1990s (14 x 12 in rod-tension snare drums with ten tube lugs a side, 24 x 14 in bass drums with wooden counter hoops and eight T-rods a side, aluminium tube shoulder carriers of 1.5 to 2.5 kg, 16 in hickory sticks of 45 g), as dealers list them; authored estimates, no brand."),
]


def bones_json(masses):
    """🦴 The rig with parent-local placements derived from the world rest frames."""
    out = []
    for bid, parent, f, length, joint, title in BONES:
        b = {"id": bid, "parent": parent, "title": T(*title)}
        if parent is None:
            b.update({"x": r6(f.o[0]), "y": r6(f.o[1]), "angleDeg": r6(f.deg)})
        else:
            pf = FRAMES[parent]
            loc = pf.local(f.o)
            b.update({"x": loc[0], "y": loc[1]})
            if abs(loc[2]) > 1e-9:
                b["z"] = loc[2]
            b["angleDeg"] = r6(f.deg - pf.deg)
        b["lengthM"] = length
        j = dict(joint)
        if bid == "sticks":
            pf = FRAMES[parent]
            d = pf.local_dir((STICK_LIFT[0], STICK_LIFT[1], 0.0))
            j["axisDeg"] = round(math.degrees(math.atan2(d[1], d[0])), 4)
        b["joint"] = {k: v for k, v in j.items() if v is not None}
        if bid in masses:
            kg, com = masses[bid]
            loc = f.local(com)
            b["mass"] = {"kg": round(kg, 3), "comX": loc[0], "comY": loc[1], "distribution": "point", "source": "authored-estimate", "reference": "catalogue-marching"}
        if bid in SECTIONS:
            b["section"] = SECTIONS[bid]
        out.append(b)
    return out


SECTIONS = {
    "stand": {"areaM2": 8.72e-05, "secondMomentM4": 3.93e-09, "extremeFibreM": 0.01, "material": "steel-coated", "title": T("Steel tube 20 x 1.5 mm", "Stahlrohr 20 x 1,5 mm")},
    "bass-carrier": {"areaM2": 2.513e-04, "secondMomentM4": 1.343e-08, "extremeFibreM": 0.011, "material": "aluminium", "title": T("Two aluminium tubes 22 x 2 mm", "Zwei Aluminiumrohre 22 x 2 mm")},
    "snare-carrier": {"areaM2": 2.513e-04, "secondMomentM4": 1.343e-08, "extremeFibreM": 0.011, "material": "aluminium", "title": T("Two aluminium tubes 22 x 2 mm", "Zwei Aluminiumrohre 22 x 2 mm")},
    "stick-r": {"areaM2": 1.767e-04, "secondMomentM4": 2.485e-09, "extremeFibreM": 0.0075, "material": "hickory", "title": T("Hickory rod 15 mm", "Hickorystab 15 mm")},
    "stick-l": {"areaM2": 1.767e-04, "secondMomentM4": 2.485e-09, "extremeFibreM": 0.0075, "material": "hickory", "title": T("Hickory rod 15 mm", "Hickorystab 15 mm")},
}


def part_mass(p):
    """⚖️ The mass of a part from its solids, its material density and its section factor (hollow tubes, sheet profiles)."""
    n = p.repeat[2] if p.repeat else 1
    vol = sum(prim.volume() for prim in p.prims) * n
    return vol * DENSITY[p.material] * p.density_factor


def masses_of(parts):
    """⚖️ The mass and centre of mass of every bone from the 1:50 parts (repeats about the drum axis keep their centre on the axis)."""
    acc = {}
    for p in parts:
        m = part_mass(p)
        if p.repeat:
            axis, c, n, step = p.repeat
            prim = p.prims[0]
            pc = prim.centroid()
            com = (c[0], pc[1], c[2]) if axis == "y" else (c[0], c[1], pc[2])
        else:
            vols = [prim.volume() for prim in p.prims]
            cs = [prim.centroid() for prim in p.prims]
            tv = sum(vols)
            com = tuple(sum(v * c[i] for v, c in zip(vols, cs)) / tv for i in range(3))
        kg, mc = acc.get(p.bone, (0.0, [0.0, 0.0, 0.0]))
        acc[p.bone] = (kg + m, [mc[i] + m * com[i] for i in range(3)])
    return {b: (kg, tuple(v / kg for v in mc)) for b, (kg, mc) in acc.items()}


def extents(parts):
    """📏 The rest-pose bounding box of the parts."""
    pts = []
    for p in parts:
        for prim in p.prims:
            if isinstance(prim, Ring):
                for a in (prim.a0, prim.a1):
                    for t in range(0, 360, 5):
                        pts.append(prim.at(a, prim.r_out, math.radians(t)))
            else:
                pts.extend(prim.points3())
        if p.repeat:
            from author_drawing import copies

            for cp in copies(p, p.prims[0]):
                pts.extend(cp.points3())
    xs, ys, zs = [q[0] for q in pts], [q[1] for q in pts], [q[2] for q in pts]
    return (min(xs), max(xs)), (min(ys), max(ys)), (min(zs), max(zs))


def solid_of(p, k, prim, scale):
    """🧊 The kernel solid of one primitive of a part at a scale."""
    f = FRAMES[p.bone]
    if isinstance(prim, Tube):
        s = prim.loft_solid(f)
    else:
        s = prim.solid(f)
    sid = p.id if len(p.prims) == 1 else f"{p.id}-{k}"
    out = {"id": sid, "bone": p.bone, "material": p.material, "role": p.role, "tone": p.tone}
    for flag in ("slender", "sheet", "envelope", "texture"):
        if p.flags.get(flag):
            out[flag] = True
    out.update(s)
    if p.repeat:
        axis, c, n, step = p.repeat
        ax = f.local_dir((0, 1, 0) if axis == "y" else (0, 0, 1))
        out["repeat"] = {"count": n, "rotationAxis": ax, "rotationRad": r6(math.radians(step)), "origin": f.local(c)}
    return out


def marker_at(bone, com):
    """⚫️ Where the mass ball of a bone is drawn: at its centre of mass, except for a carrier, whose centre of mass lies in the air between its bars, so its ball sits on the bar line at that height (the bone mass itself keeps the true centre)."""
    if not bone.endswith("-carrier"):
        return com
    from author_parts import BASS_JS, BASS_XP, SNARE_JS, SNARE_XP, j_path

    path = j_path(SNARE_XP, 0.29, 0.40, SNARE_JS) if bone.startswith("snare") else j_path(BASS_XP, 0.545, 0.655, BASS_JS)
    for (x0, y0), (x1, y1) in zip(path, path[1:]):
        if min(y0, y1) <= com[1] <= max(y0, y1) and abs(y1 - y0) > 1e-9:
            return (x0 + (x1 - x0) * (com[1] - y0) / (y1 - y0), com[1], 0.0)
    return com


THERMAL_ONLY = {
    200: [
        ("bass", Ring("z", (0.0, YB, 0.0), -0.158, 0.158, 0.297, 0.305), "bass-shell", "birch-lacquer", {}),
        ("bass", Ring("z", (0.0, YB, 0.0), 0.158, 0.200, 0.3065, 0.3245), "bass-hoops", "beech-lacquer", {}),
        ("bass", Ring("z", (0.0, YB, 0.0), -0.200, -0.158, 0.3065, 0.3245), "bass-hoops", "beech-lacquer", {}),
        ("snare", Ring("y", (XS, 0.0, 0.0), 0.030, 0.299, 0.172, 0.178), "snare-wrap", "pvc-wrap", {}),
        ("snare", Ring("y", (XS, 0.0, 0.0), 0.0, 0.030, 0.1795, 0.1905), "snare-hardware", "chrome-steel", {}),
        ("snare", Ring("y", (XS, 0.0, 0.0), 0.299, 0.329, 0.1795, 0.1905), "snare-hardware", "chrome-steel", {}),
    ],
    500: [
        ("bass", Ring("z", (0.0, YB, 0.0), -0.141, 0.141, 0.297, 0.305), "bass-shell", "birch-lacquer", {}),
        ("bass", Ring("z", (0.0, YB, 0.0), 0.141, 0.216, 0.3065, 0.3245), "bass-hoops", "beech-lacquer", {}),
        ("bass", Ring("z", (0.0, YB, 0.0), -0.216, -0.141, 0.3065, 0.3245), "bass-hoops", "beech-lacquer", {}),
        ("bass", Ring("z", (0.0, YB, 0.0), 0.141, 0.216, 0.3245, 0.342), "bass-hardware", "steel-nickel", {}),
        ("bass", Ring("z", (0.0, YB, 0.0), -0.216, -0.141, 0.3245, 0.342), "bass-hardware", "steel-nickel", {}),
        ("snare", Ring("y", (XS, 0.0, 0.0), 0.075, 0.254, 0.172, 0.178), "snare-wrap", "pvc-wrap", {}),
        ("snare", Ring("y", (XS, 0.0, 0.0), 0.0, 0.075, 0.1795, 0.1905), "snare-hardware", "chrome-steel", {}),
        ("snare", Ring("y", (XS, 0.0, 0.0), 0.254, 0.329, 0.1795, 0.1905), "snare-hardware", "chrome-steel", {}),
    ],
}


def own_solid(sid, bone, prim, material, role, tone, flags):
    """🧊 A thermal- or structure-only solid."""
    s = {"id": sid, "bone": bone, "material": material, "role": role, "tone": tone}
    for flag, v in flags.items():
        if v:
            s[flag] = True
    s.update(prim.solid(FRAMES[bone]))
    return s


def volume_of(scale, parts, masses):
    """🔧 The authored volume of one scale: body solids of every part, the thermal list as references (merged turned bodies get own region shells), the structure list as the members, joints and centres of mass."""
    body, thermal, structure = [], [], []
    for p in parts:
        for k, prim in enumerate(p.prims):
            s = solid_of(p, k, prim, scale)
            body.append(s)
            if not p.flags.get("solidbody"):
                t = {"id": f"th-{s['id']}", "of": s["id"], "region": p.region}
                if p.region in ("bass-heads", "snare-heads") and scale in (200, 500):
                    t["id"] = f"th-{s['id']}-sheet"
                thermal.append(t)
            if p.struct and not p.flags.get("solidbody"):
                structure.append({"id": f"st-{s['id']}", "of": s["id"], "member": p.bone, "part": p.struct})
    for i, (bone, prim, region, material, flags) in enumerate(THERMAL_ONLY.get(scale, [])):
        thermal.append(dict(own_solid(f"th-{region}-{i}", bone, prim, material, "body", 0.5, flags), region=region))
        if region in ("bass-shell", "bass-hoops", "snare-wrap") or (region == "snare-hardware" and not flags):
            structure.append(dict(own_solid(f"st-{region}-{i}", bone, prim, material, "body", 0.5, flags), member=bone, part="member"))
    r = {50: 0.015, 100: 0.02, 200: 0.03, 500: 0.05}[scale]
    for bone, (kg, com) in masses.items():
        if bone in ("snare-strainer",):
            continue
        rr = r
        com = marker_at(bone, com)
        loc = FRAMES[bone].local(com)
        structure.append({"id": f"mass-{bone}", "kind": "ellipsoid", "bone": bone, "material": "steel-coated", "role": "detail", "tone": 0.15, "radii": [rr, rr, rr], "transform": {"translation": loc}, "member": bone, "part": "mass"})
    return {"status": "authored", "denominator": scale, "body": body, "thermal": thermal, "structure": structure, "decisions": VOLUME_DECISIONS[scale]}


VOLUME_DECISIONS = {
    50: [
        T("Body 1:50 (feature 7.5 mm): every part at its real size. Bass drum: the 8 mm lacquered birch shell of 610 x 356 mm, two calfskin heads of 0.5 mm (declared sheets), two beech counter hoops of 18 x 42 mm, and on each side eight die-cast lugs, eight 8 mm tension rods outside the hoop, eight claws over the hoop edge and eight T-handles, as repeats about the drum axis; the glued beech cleat over the crack of the right hoop and the cast carrier mount. Snare drum: the wrapped 6 mm birch shell of 356 x 305 mm, the batter and snare-side heads of 0.25 mm (sheets), the triple-flanged hoops as stepped rings with their flange lip, ten 14 mm tube lugs, twenty ears, the strainer with its throw-off lever, the butt, the 1 mm band of snare wires under the bottom head and the carrier mount. Each carrier: the 3 mm vest plate (sheet), the 12 mm belly pad, the mount bar and two J-bars of 22 mm tube as capsule chains with their foam slings. Sticks: tapered hickory rods with their beads. Cradle: 20 mm tube feet with rubber caps, uprights, cross tubes and four arms with rubber sleeves that touch the bass shell at 40 degrees from its bottom.", "Körper 1:50 (kleinstes Detail 7,5 mm): jedes Teil in seiner wirklichen Größe. Große Trommel: der 8 mm starke lackierte Birkenkessel von 610 x 356 mm, zwei Kalbfelle von 0,5 mm (als Blatt erklärt), zwei Buchen-Spannreifen von 18 x 42 mm und auf jeder Seite acht Druckgussböckchen, acht 8-mm-Spannschrauben außen am Reifen, acht Krallen über der Reifenkante und acht Flügelgriffe, als Wiederholungen um die Trommelachse; die aufgeleimte Buchenlasche über dem Riss des rechten Reifens und die gegossene Trägeraufnahme. Kleine Trommel: der folierte 6-mm-Birkenkessel von 356 x 305 mm, Schlag- und Resonanzfell von 0,25 mm (Blätter), die dreifach gebördelten Reifen als gestufte Ringe mit Bördellippe, zehn 14-mm-Rohrböckchen, zwanzig Ösen, die Abhebung mit Hebel, das Gegenstück, das 1 mm starke Band des Teppichs unter dem unteren Fell und die Trägeraufnahme. Jedes Tragegestell: die 3 mm starke Bauchplatte (Blatt), das 12-mm-Bauchpolster, der Haltearm und zwei J-Bügel aus 22-mm-Rohr als Kapselketten mit ihren Schaumpolstern. Stöcke: verjüngte Hickorystäbe mit Kopf. Bock: Füße aus 20-mm-Rohr mit Gummikappen, Stützen, Querrohre und vier Arme mit Gummihülsen, die den Kessel 40 Grad neben seinem tiefsten Punkt berühren."),
        T("Thermal 1:50: every body solid is referenced into its region: the cradle tubes, caps and sleeves (stand-frame), the bass shell, the calfskin heads, the beech hoops with the cleat, the plated tension hardware with the mount, the aluminium of both carriers, the foam of pads and slings, the wrapped snare shell, the polyester heads, the chromed snare hardware with the snare wires, and the sticks. Shell and heads close each drum, so the inside of the shells counts for no outer area; the 1.5 mm clearance between a head collar and its hoop is closed below a quarter feature.", "Thermik 1:50: jeder Körper ist seiner Region zugeordnet: Rohre, Kappen und Hülsen des Bocks (stand-frame), der Kessel der großen Trommel, die Kalbfelle, die Buchenreifen mit der Lasche, die vernickelten Spannteile mit der Aufnahme, das Aluminium beider Gestelle, der Schaum von Polstern und Schlingen, der folierte Kessel der kleinen Trommel, die Polyesterfelle, die verchromten Teile der kleinen Trommel mit dem Teppich und die Stöcke. Kessel und Felle schließen jede Trommel, daher zählt das Kesselinnere nicht als Außenfläche; der Spalt von 1,5 mm zwischen Fellkragen und Reifen ist unter einem Viertel Detail geschlossen."),
        T("Structure 1:50: the load path in the stored pose. The cradle tubes are members of the stand bone, its rubber caps carry it on the floor and its four sleeves are the joints on which the bass shell rests; the bass shell and its hoops are the members of the drum, the mount and the mount bar join the carrier plate and the two J-bars to it; the snare stands on its bottom hoop, its shell and hoops are members, its mount and T-bar join its carrier; the sticks lie across the top hoop. A ball at the centre of mass of every bone carries its mass.", "Tragwerk 1:50: der Lastpfad in der Lagerstellung. Die Rohre des Bocks sind Stäbe des Bockknochens, seine Gummikappen tragen ihn am Boden und seine vier Hülsen sind die Gelenke, auf denen der Kessel der großen Trommel liegt; Kessel und Reifen sind die Stäbe der Trommel, Aufnahme und Haltearm verbinden Bauchplatte und J-Bügel mit ihr; die kleine Trommel steht auf ihrem unteren Reifen, Kessel und Reifen sind Stäbe, Aufnahme und T-Arm verbinden ihr Gestell; die Stöcke liegen quer auf dem oberen Reifen. Eine Kugel im Schwerpunkt jedes Knochens trägt seine Masse."),
    ],
    100: [
        T("Body 1:100 (feature 15 mm): a new abstraction of the same drums. The rubber end caps are merged into 22 mm feet; on the bass drum the lug, the rod and the claw with its T-handle of every tension point are two blocks of 20 and 18 mm, the hoop cleat (9.5 mm) is left out; the snare hoops lose their flange lip, the 8.5 mm ears are merged into Ø15 tube lugs that run from hoop to hoop, the strainer lever is merged into the strainer and the snare wires are left out (hidden under the head). Plate and belly pad of each carrier are one 15 mm block, mounts and mount bars 15 mm; the bars keep their 22 mm tube and the slings their 44 mm; each stick is one tapered rod at its true section, declared slender.", "Körper 1:100 (kleinstes Detail 15 mm): eine neue Abstraktion derselben Trommeln. Die Gummikappen gehen in 22 mm starken Füßen auf; an der großen Trommel sind Böckchen, Spannschraube und Kralle mit Flügel jedes Spannpunkts zwei Blöcke von 20 und 18 mm, die Reifenlasche (9,5 mm) entfällt; die Reifen der kleinen Trommel verlieren ihre Bördellippe, die 8,5-mm-Ösen gehen in Rohrböckchen Ø15 auf, die von Reifen zu Reifen reichen, der Abhebehebel geht in der Abhebung auf und der Teppich entfällt (unter dem Fell verborgen). Bauchplatte und Polster jedes Gestells sind ein 15-mm-Block, Aufnahmen und Haltearme 15 mm; die Bügel behalten ihr 22-mm-Rohr und die Schlingen ihre 44 mm; jeder Stock ist ein verjüngter Stab in wahrem Querschnitt, als schlank erklärt."),
        T("Thermal 1:100: the body solids of the scale are referenced into the same eleven regions; the merged blocks carry the region of the part they stand for (tension blocks: bass hardware, plate blocks: carrier frames).", "Thermik 1:100: die Körper dieses Maßstabs sind denselben elf Regionen zugeordnet; die zusammengefassten Blöcke tragen die Region des Teils, für das sie stehen (Spannblöcke: Beschläge der großen Trommel, Plattenblöcke: Gestelle)."),
        T("Structure 1:100: the same load path with the solids of the scale: cradle tubes and sleeves, shells and hoops, mounts and mount bars as joints, plate blocks and bars of the carriers, the sticks; masses at the centres of mass.", "Tragwerk 1:100: derselbe Lastpfad mit den Körpern dieses Maßstabs: Rohre und Hülsen des Bocks, Kessel und Reifen, Aufnahmen und Haltearme als Gelenke, Plattenblöcke und Bügel der Gestelle, die Stöcke; Massen in den Schwerpunkten."),
    ],
    200: [
        T("Body 1:200 (feature 30 mm): each drum is one turned body with the steps of its hoops (bass hoops 42 mm wide at r 0.3245 m, snare hoops generalised to 30 mm bands at r 0.1905 m); the 14 mm tube lugs, the strainer and the butt of the snare are below the feature and left out, the sixteen tension points of the bass drum are eight 30 mm blocks a side. The cradle keeps its feet as slender 22 mm tubes and its arms as one slender V-shaped tube a side at the 29 mm of the sleeves; uprights and cross tubes are 30 mm. Mount and mount bar are one 30 mm block per drum, plate and pad one 30 mm block per carrier; the J-bars are slender lofts at their true 22 mm, the slings four 44 mm capsules each; the sticks stay slender at their true section.", "Körper 1:200 (kleinstes Detail 30 mm): jede Trommel ist ein gedrehter Körper mit den Stufen ihrer Reifen (Reifen der großen Trommel 42 mm breit bei r 0,3245 m, Reifen der kleinen Trommel zu 30-mm-Bändern bei r 0,1905 m verallgemeinert); die 14-mm-Rohrböckchen, Abhebung und Gegenstück der kleinen Trommel liegen unter dem Detail und entfallen, die sechzehn Spannpunkte der großen Trommel sind acht 30-mm-Blöcke je Seite. Der Bock behält seine Füße als schlanke 22-mm-Rohre und seine Arme als ein schlankes V-Rohr je Seite in den 29 mm der Hülsen; Stützen und Querrohre sind 30 mm. Aufnahme und Haltearm sind ein 30-mm-Block je Trommel, Platte und Polster ein 30-mm-Block je Gestell; die J-Bügel sind schlanke Lofts in wahren 22 mm, die Schlingen je vier 44-mm-Kapseln; die Stöcke bleiben schlank in wahrem Querschnitt."),
        T("Thermal 1:200: the merged drum bodies are split again into their region shells: the bass shell ring, the two hoop rings and the two calfskin sheets, the snare shell ring, the two hoop bands and the two head sheets; the other regions reference the body solids of the scale.", "Thermik 1:200: die zusammengefassten Trommelkörper sind wieder in ihre Regionsschalen geteilt: Kesselring, zwei Reifenringe und zwei Kalbfelle der großen Trommel, Kesselring, zwei Reifenbänder und zwei Felle der kleinen Trommel; die übrigen Regionen verweisen auf die Körper dieses Maßstabs."),
        T("Structure 1:200: shell rings and hoop rings are the members of the drums, the cradle tubes and the V-arms that touch the shell carry them, mounts are joints, the carriers are their plate blocks and slender bars, the sticks slender rods; masses at the centres of mass.", "Tragwerk 1:200: Kesselringe und Reifenringe sind die Stäbe der Trommeln, die Rohre des Bocks und die V-Arme, die den Kessel berühren, tragen sie, Aufnahmen sind Gelenke, die Gestelle sind Plattenblöcke und schlanke Bügel, die Stöcke schlanke Stäbe; Massen in den Schwerpunkten."),
    ],
    500: [
        T("Body 1:500 (feature 75 mm): each drum is one turned body with 75 mm hoop bands; the tension hardware is merged into the hoop bands. The tubular cradle is typified as its envelope saddle (declared envelope), whose top follows the bass shell between the two sleeve contacts. The vest plates and belly pads are declared sheets of 3 and 10 mm, the mounts 75 mm blocks bridging plate and shell, the J-bars and the sticks slender at their true section; the slings are merged into the bars.", "Körper 1:500 (kleinstes Detail 75 mm): jede Trommel ist ein gedrehter Körper mit 75-mm-Reifenbändern; die Spannteile gehen in den Reifenbändern auf. Der Rohrbock ist als Hüllsattel typisiert (als Hülle erklärt), dessen Oberkante dem Kessel zwischen den beiden Hülsenauflagern folgt. Bauchplatten und Polster sind erklärte Blätter von 3 und 10 mm, die Aufnahmen 75-mm-Blöcke zwischen Platte und Kessel, J-Bügel und Stöcke schlank in wahrem Querschnitt; die Schlingen gehen in den Bügeln auf."),
        T("Thermal 1:500: the drum bodies are split into shell rings, 75 mm hoop rings, head sheets and, for the bass drum, a 75 mm ring of its tension hardware outside the hoops; the pads are the belly pad sheets, the other regions reference the body solids of the scale.", "Thermik 1:500: die Trommelkörper sind in Kesselringe, 75-mm-Reifenringe, Fellblätter und, bei der großen Trommel, einen 75-mm-Ring ihrer Spannteile außen an den Reifen geteilt; die Polster sind die Polsterblätter, die übrigen Regionen verweisen auf die Körper dieses Maßstabs."),
        T("Structure 1:500: shell and hoop rings of the drums on the envelope saddle of the cradle, mount blocks as joints, plates and slender bars of the carriers, slender sticks; masses at the centres of mass.", "Tragwerk 1:500: Kessel- und Reifenringe der Trommeln auf dem Hüllsattel des Bocks, Aufnahmeblöcke als Gelenke, Platten und schlanke Bügel der Gestelle, schlanke Stöcke; Massen in den Schwerpunkten."),
    ],
}


REGIONS = [
    ("stand-frame", "Storage cradle: coated tubes, rubber caps and sleeves", "Lagerbock: beschichtete Rohre, Gummikappen und -hülsen", "steel-coated", 0.20, 0.02, "cylinder", "metal", 0.0015, 1.0e-05, 5400, "Steel tube 1.5 mm", "Stahlrohr 1,5 mm"),
    ("bass-shell", "Bass drum shell, lacquered birch", "Kessel der großen Trommel, lackierte Birke", "birch-lacquer", 0.60, 0.61, "cylinder", "timber", 0.008, 0.047, 8700, "Birch plywood 8 mm", "Birkensperrholz 8 mm"),
    ("bass-heads", "Bass drum heads, calfskin", "Felle der großen Trommel, Kalbfell", "calfskin", 0.60, 0.61, "vertical-plate", "shell", 0.0005, 0.003, 750, "Calfskin 0.5 mm", "Kalbfell 0,5 mm"),
    ("bass-hoops", "Bass drum counter hoops, lacquered beech", "Spannreifen der großen Trommel, lackierte Buche", "beech-lacquer", 0.34, 0.042, "cylinder", "timber", 0.018, 0.1, 20700, "Beech 18 mm", "Buche 18 mm"),
    ("bass-hardware", "Bass drum lugs, rods, claws, T-handles and mount, nickel-plated", "Böckchen, Spannschrauben, Krallen, Flügel und Aufnahme der großen Trommel, vernickelt", "steel-nickel", 0.20, 0.02, "cylinder", "metal", 0.003, 6.0e-05, 10800, "Plated steel and die-cast 3 mm", "Vernickelter Stahl und Druckguss 3 mm"),
    ("carrier-frames", "Carrier plates, mount bars and J-bars, aluminium", "Bauchplatten, Haltearme und J-Bügel der Gestelle, Aluminium", "aluminium", 0.40, 0.022, "cylinder", "metal", 0.002, 1.0e-05, 4900, "Aluminium 2 mm", "Aluminium 2 mm"),
    ("carrier-pads", "Belly pads and shoulder slings, foam in nylon", "Bauchpolster und Schulterschlingen, Schaum in Nylon", "foam-pad", 0.25, 0.044, "cylinder", "insulation", 0.012, 0.27, 1600, "PE foam 12 mm", "PE-Schaum 12 mm"),
    ("snare-wrap", "Snare drum shell, faded pearl wrap on birch", "Kessel der kleinen Trommel, verblasste Perlmuttfolie auf Birke", "pvc-wrap", 0.31, 0.305, "cylinder", "polymer", 0.0004, 0.002, 550, "Wrap 0.4 mm", "Folie 0,4 mm"),
    ("snare-heads", "Snare drum heads, coated polyester", "Felle der kleinen Trommel, beschichtetes Polyester", "pet-coated", 0.20, 0.357, "horizontal-plate-up", "polymer", 0.00025, 0.0017, 400, "Polyester film 0.25 mm", "Polyesterfolie 0,25 mm"),
    ("snare-hardware", "Snare drum hoops, tube lugs, ears, strainer and snares, chrome-plated", "Reifen, Rohrböckchen, Ösen, Abhebung und Teppich der kleinen Trommel, verchromt", "chrome-steel", 0.30, 0.024, "cylinder", "metal", 0.0023, 5.0e-05, 8300, "Chromed steel 2.3 mm", "Verchromter Stahl 2,3 mm"),
    ("sticks", "Hickory sticks", "Hickorystöcke", "hickory", 0.033, 0.015, "cylinder", "timber", 0.0075, 0.047, 10000, "Hickory 7.5 mm to the axis", "Hickory 7,5 mm bis zur Achse"),
]


def regions_json(areas):
    """🌡️ The passive thermal regions with one surface layer each; both instruments state their nameplate as passive 0 W."""
    out = []
    for rid, en, de, mat, area, lc, geom, kind, th, res, cap, len_, lde in REGIONS:
        r = {"id": rid, "title": T(en, de), "kind": "surface", "model": "passive", "material": mat, "areaM2": areas.get(rid, area), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": 0.4, "layers": [{"id": f"{rid}-skin", "title": T(len_, lde), "kind": kind, "material": mat, "thicknessM": th, "resistanceM2KW": res, "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "incropera-2018"}], "heatCapacityJM2K": cap, "source": "authored-estimate", "reference": "incropera-2018"}
        if rid in ("bass-shell", "snare-wrap"):
            r["ratedPowerW"] = 0
            r["ratedPowerBasis"] = "passive"
        out.append(r)
    return out


def anchor(aid, bone, world, kind, en, de, **extra):
    """📍 An anchor at a world rest point, stated in the frame of its bone."""
    loc = FRAMES[bone].local(world)
    a = {"id": aid, "bone": bone, "x": loc[0], "y": loc[1]}
    if abs(loc[2]) > 1e-9:
        a["z"] = loc[2]
    a.update({"kind": kind, "title": T(en, de)})
    a.update(extra)
    return a


SX = XS - 0.249 - 0.111
BX = -0.344 - 0.111
ANCHORS = [
    anchor("foot-front-right", "stand", (0.274, 0.0, -0.08), "contact", "Front right rubber cap of the cradle", "Vordere rechte Gummikappe des Bocks", support="contact", footprintLengthM=0.032, footprintWidthM=0.022, frictionCoefficient=0.6),
    anchor("foot-front-left", "stand", (0.274, 0.0, 0.08), "contact", "Front left rubber cap of the cradle", "Vordere linke Gummikappe des Bocks", support="contact", footprintLengthM=0.032, footprintWidthM=0.022, frictionCoefficient=0.6),
    anchor("foot-rear-right", "stand", (-0.274, 0.0, -0.08), "contact", "Rear right rubber cap of the cradle", "Hintere rechte Gummikappe des Bocks", support="contact", footprintLengthM=0.032, footprintWidthM=0.022, frictionCoefficient=0.6),
    anchor("foot-rear-left", "stand", (-0.274, 0.0, 0.08), "contact", "Rear left rubber cap of the cradle", "Hintere linke Gummikappe des Bocks", support="contact", footprintLengthM=0.032, footprintWidthM=0.022, frictionCoefficient=0.6),
    anchor("snare-floor", "snare", (XS, 0.0, 0.0), "contact", "Bottom hoop of the snare drum on the floor", "Unterer Reifen der kleinen Trommel auf dem Boden", support="contact", footprintLengthM=0.38, footprintWidthM=0.38, frictionCoefficient=0.4),
    anchor("snare-grip", "snare-carrier", (XS - 0.2615, 0.40, 0.0), "support", "Hand grip at the top edge of the snare carrier plate", "Handgriff an der Oberkante der Bauchplatte der kleinen Trommel", support="pin"),
    anchor("bass-grip", "bass-carrier", (-0.3565, 0.655, 0.0), "support", "Hand grip at the top edge of the bass carrier plate", "Handgriff an der Oberkante der Bauchplatte der großen Trommel", support="pin"),
    anchor("snare-shoulder-right", "snare-carrier", (SX, 0.812, -0.095), "support", "Right shoulder under the sling of the snare carrier", "Rechte Schulter unter der Schlinge des Gestells der kleinen Trommel", support="contact", footprintLengthM=0.08, footprintWidthM=0.05, frictionCoefficient=0.5),
    anchor("snare-shoulder-left", "snare-carrier", (SX, 0.812, 0.095), "support", "Left shoulder under the sling of the snare carrier", "Linke Schulter unter der Schlinge des Gestells der kleinen Trommel", support="contact", footprintLengthM=0.08, footprintWidthM=0.05, frictionCoefficient=0.5),
    anchor("snare-belly", "snare-carrier", (XS - 0.275, 0.29, 0.0), "support", "Belly of the snare drummer against the pad", "Bauch des Trommlers am Polster der kleinen Trommel", support="contact", normalDeg=0, footprintLengthM=0.20, footprintWidthM=0.22, frictionCoefficient=0.5),
    anchor("bass-shoulder-right", "bass-carrier", (BX, YB + 0.472, -0.095), "support", "Right shoulder under the sling of the bass carrier", "Rechte Schulter unter der Schlinge des Gestells der großen Trommel", support="contact", footprintLengthM=0.08, footprintWidthM=0.05, frictionCoefficient=0.5),
    anchor("bass-shoulder-left", "bass-carrier", (BX, YB + 0.472, 0.095), "support", "Left shoulder under the sling of the bass carrier", "Linke Schulter unter der Schlinge des Gestells der großen Trommel", support="contact", footprintLengthM=0.08, footprintWidthM=0.05, frictionCoefficient=0.5),
    anchor("bass-belly", "bass-carrier", (-0.370, 0.545, 0.0), "support", "Belly of the bass drummer against the pad", "Bauch des Trommlers am Polster der großen Trommel", support="contact", normalDeg=0, footprintLengthM=0.20, footprintWidthM=0.22, frictionCoefficient=0.5),
    anchor("snare-strike", "snare", (XS + 0.03, 0.31725, 0.0), "load", "Beating spot of the snare batter head", "Schlagpunkt des Schlagfells der kleinen Trommel"),
    anchor("bass-strike", "bass", (0.02, YB + 0.03, -0.1785), "load", "Beater spot on the right calfskin head", "Schlagpunkt auf dem rechten Kalbfell"),
]

FEET = ["foot-front-right", "foot-front-left", "foot-rear-right", "foot-rear-left"]
WORN = ["snare-shoulder-right", "snare-shoulder-left", "snare-belly", "bass-shoulder-right", "bass-shoulder-left", "bass-belly"]
GRIPS = ["snare-grip", "bass-grip"]


def axis_offset(bid, s):
    """📐 A travel of s along the stated axis of a prismatic bone, at full precision so it stays exactly on its axis."""
    j = next(b for b in bones_json({}) if b["id"] == bid)["joint"]
    a = math.radians(j["axisDeg"])
    return {"dx": s * math.cos(a), "dy": s * math.sin(a)}


def lift_offsets(bob=0.0):
    """🏋️ The prismatic offsets that put both drums on the carriers (bob lowers or raises them by a marching step)."""
    return {"bass-lift": {"dy": r6(BASS_LIFT + bob)}, "snare-lift": axis_offset("snare-lift", math.hypot(*SNARE_LIFT) + bob), "sticks": axis_offset("sticks", math.hypot(*STICK_LIFT)), "snare-strainer": {"angleDeg": 180}}


def binding(worn, sticks_held):
    """🌬️ The thermal binding of a pose: indoor depot or rehearsal room without sun, every region exposed; floor contact of the cradle and, worn, the drummers' bodies on pads and frames and the hands on the sticks."""
    regions = {r[0]: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGIONS}
    regions["stand-frame"] = {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.02, "tempC": 17}}
    if worn:
        regions["carrier-pads"] = {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.45, "tempC": 33}}
        regions["carrier-frames"] = {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.05, "tempC": 32}}
    else:
        regions["snare-hardware"] = {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.02, "tempC": 17}}
    if sticks_held:
        regions["sticks"] = {"exposedFraction": 1, "sunlitFraction": 0, "contact": {"fraction": 0.3, "tempC": 33}}
    return {"regions": regions, "emitters": {}}


STRIKES = [
    {"id": "stick-strike", "kind": "live", "title": T("Stick stroke on the batter head (peak 45 N)", "Stockschlag auf das Schlagfell (Spitze 45 N)"), "anchor": "snare-strike", "fxN": 0, "fyN": -45, "source": "authored-estimate", "reference": "rossing-2000"},
    {"id": "bass-strike", "kind": "live", "title": T("Beater stroke on the right calfskin head (peak 35 N)", "Schlägelschlag auf das rechte Kalbfell (Spitze 35 N)"), "anchor": "bass-strike", "fxN": 0, "fyN": 0, "fzN": 35, "source": "authored-estimate", "reference": "rossing-2000"},
]


def pose(pid, en, de, den, dde, bones, contacts, worn, held, loads=(), key=False):
    """📥️ One pose with its bindings."""
    p = {"id": pid, "title": T(en, de), "description": T(den, dde)}
    if key:
        p["key"] = True
    p.update({"bones": bones, "ground": "auto", "contacts": contacts, "thermal": binding(worn, held), "loads": list(loads)})
    return p


def poses_json():
    """🧍 Stored on the cradle, carried on the carriers, played on the march, and the key poses of the clips."""
    worn = lift_offsets()
    carried = dict(worn, **{"stick-r": {"angleDeg": -30}, "stick-l": {"angleDeg": -30}})
    played = dict(worn, **{"stick-r": {"angleDeg": STICK_STRIKE}, "stick-l": {"angleDeg": STICK_RAISED}})
    low = dict(lift_offsets(-0.012), **{"stick-r": {"angleDeg": STICK_STRIKE}, "stick-l": {"angleDeg": STICK_RAISED}})
    high = dict(lift_offsets(0.012), **{"stick-r": {"angleDeg": STICK_RAISED}, "stick-l": {"angleDeg": STICK_STRIKE}})
    return [
        pose("stored", "Stored on the cradle", "Gelagert auf dem Bock", "As the band depot keeps them: the bass drum lies on its tubular cradle with heads to the sides, its carrier still mounted; the snare stands on its bottom hoop in front, snares thrown off, its carrier leaning with the back of its shoulder bars against the bass shell, the sticks laid across its top hoop.", "Wie das Lager des Musikvereins sie aufbewahrt: die große Trommel liegt mit den Fellen zur Seite auf ihrem Rohrbock, das Tragegestell noch montiert; davor steht die kleine Trommel auf ihrem unteren Reifen, der Teppich abgehoben, ihr Gestell lehnt mit dem Rücken der Schulterbügel am Kessel der großen Trommel, die Stöcke liegen quer auf dem oberen Reifen.", {}, FEET + ["snare-floor"], False, False),
        pose("carried", "Carried on the carriers", "Getragen an den Gestellen", "Both drums are worn on their shoulder carriers by two drummers in file, standing at the line-up: the bass drum's axis at 1.0 m, the snare's top hoop at belt height; snares on, the sticks held at rest; the cradle stays behind on the floor.", "Beide Trommeln hängen an ihren Schultergestellen an zwei Trommlern hintereinander, beim Antreten: die Achse der großen Trommel auf 1,0 m, der obere Reifen der kleinen Trommel auf Gürtelhöhe; Teppich an, Stöcke in Ruhe gehalten; der Bock bleibt am Boden zurück.", carried, FEET + WORN, True, True),
        pose("played", "Played on the march", "Auf dem Marsch gespielt", "The same carriers while marching and playing: the right stick strikes the batter head at 45 N while the left is raised to 22 degrees below level, the beater strikes the right calfskin head at 35 N; the shoulders and bellies take the weight and the strokes.", "Dieselben Gestelle beim Marschieren und Spielen: der rechte Stock schlägt mit 45 N auf das Schlagfell, während der linke auf 22 Grad unter der Waagrechten angehoben ist, der Schlägel trifft das rechte Kalbfell mit 35 N; Schultern und Bäuche nehmen Gewicht und Schläge auf.", played, FEET + WORN, True, True, STRIKES),
        pose("stored-snares-on", "Stored, snares on", "Gelagert, Teppich an", "Key pose of the clips: still stored, the throw-off lever turned up so the snares lie against the bottom head.", "Zwischenstellung der Clips: noch gelagert, der Abhebehebel nach oben gelegt, sodass der Teppich am unteren Fell anliegt.", {"snare-strainer": {"angleDeg": 180}}, FEET + ["snare-floor"], False, False, key=True),
        pose("march-low-right", "Footfall, right stroke", "Auftritt, rechter Schlag", "Key pose of the march: at the footfall both drums dip 12 mm on the carriers while the right stick strikes.", "Zwischenstellung des Marschs: beim Auftritt sinken beide Trommeln 12 mm in den Gestellen, während der rechte Stock schlägt.", low, FEET + WORN, True, True, STRIKES, key=True),
        pose("march-high-left", "Mid-step, left stroke", "Schrittmitte, linker Schlag", "Key pose of the march: in mid-step both drums rise 12 mm while the left stick strikes and the right is raised.", "Zwischenstellung des Marschs: in der Schrittmitte heben sich beide Trommeln um 12 mm, während der linke Stock schlägt und der rechte angehoben ist.", high, FEET + WORN, True, True, STRIKES, key=True),
    ]


def clips_json():
    """🎞️ Carry (stored to carried), march (strokes and gait bob, looping) and the throw-off of the snares."""
    carry = {"id": "carry", "title": T("Carry", "Tragen"), "description": T("The snares are thrown on, then both drums are lifted by the top edges of their carrier plates off the cradle and the floor and hung onto the shoulders of two drummers; the cradle stays behind.", "Der Teppich wird angelegt, dann werden beide Trommeln an den Oberkanten ihrer Bauchplatten vom Bock und vom Boden gehoben und zwei Trommlern auf die Schultern gehängt; der Bock bleibt zurück."), "durationS": 5, "fps": 10, "loop": False, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.6, "pose": "stored-snares-on", "ease": "smooth"}, {"t": 3.6, "pose": "carried", "ease": "smooth"}, {"t": 5, "pose": "carried", "ease": "smooth"}],
             "contacts": [{"anchor": a, "fromS": 0, "toS": 5} for a in FEET] + [{"anchor": "snare-floor", "fromS": 0, "toS": 0.9}] + [{"anchor": a, "fromS": 0.9, "toS": 3.4} for a in GRIPS] + [{"anchor": a, "fromS": 3.4, "toS": 5} for a in WORN],
             "phases": [{"id": "snares-on", "fromS": 0, "toS": 0.6, "title": T("Snares on", "Teppich an")}, {"id": "lift", "fromS": 0.6, "toS": 3.6, "title": T("Lifting onto the shoulders", "Auf die Schultern heben")}, {"id": "worn", "fromS": 3.6, "toS": 5, "title": T("Worn", "Getragen")}],
             "events": [{"t": 0.9, "kind": "contact-off", "title": T("Snare lifted off the floor", "Kleine Trommel vom Boden gehoben"), "target": "snare-floor"}, {"t": 0.9, "kind": "contact-on", "title": T("Hands on the plate edges", "Hände an den Plattenkanten"), "target": "snare-grip"}, {"t": 3.4, "kind": "contact-on", "title": T("Slings on the shoulders", "Schlingen auf den Schultern"), "target": "snare-shoulder-right"}, {"t": 3.4, "kind": "contact-off", "title": T("Hands let go", "Hände lassen los"), "target": "bass-grip"}],
             "tracks": []}
    stroke_keys = []
    for i in range(4):
        t0 = 0.25 * i
        stroke_keys += [{"t": round(t0, 3), "v": 1}, {"t": round(t0 + 0.06, 3), "v": 0}, {"t": round(t0 + 0.19, 3), "v": 0}]
    stroke_keys.append({"t": 1.0, "v": 1})
    bass_keys = [{"t": 0, "v": 1}, {"t": 0.06, "v": 0}, {"t": 0.44, "v": 0}, {"t": 0.5, "v": 1}, {"t": 0.56, "v": 0}, {"t": 0.94, "v": 0}, {"t": 1.0, "v": 1}]
    march = {"id": "march", "title": T("March", "Marsch"), "description": T("One cycle of two steps at 120 steps a minute: the drums dip 12 mm at each footfall and rise in mid-step, the sticks alternate eighth-note strokes at 45 N and the beater strikes the calfskin on every step.", "Ein Zyklus von zwei Schritten bei 120 Schritten in der Minute: die Trommeln sinken bei jedem Auftritt um 12 mm und heben sich in der Schrittmitte, die Stöcke schlagen abwechselnd Achtel mit 45 N und der Schlägel trifft das Kalbfell auf jeden Schritt."), "durationS": 1.0, "fps": 25, "loop": True, "rootMotion": "none",
             "keys": [{"t": 0, "pose": "march-low-right", "ease": "smooth"}, {"t": 0.25, "pose": "march-high-left", "ease": "smooth"}, {"t": 0.5, "pose": "march-low-right", "ease": "smooth"}, {"t": 0.75, "pose": "march-high-left", "ease": "smooth"}, {"t": 1.0, "pose": "march-low-right", "ease": "smooth"}],
             "contacts": [{"anchor": a, "fromS": 0, "toS": 1.0} for a in FEET + WORN],
             "phases": [{"id": "right-step", "fromS": 0, "toS": 0.5, "title": T("Right step", "Rechter Schritt")}, {"id": "left-step", "fromS": 0.5, "toS": 1.0, "title": T("Left step", "Linker Schritt")}],
             "events": [{"t": 0, "kind": "load-on", "title": T("Right stroke and beater", "Rechter Schlag und Schlägel"), "target": "stick-strike"}, {"t": 0.25, "kind": "load-on", "title": T("Left stroke", "Linker Schlag"), "target": "stick-strike"}, {"t": 0.5, "kind": "load-on", "title": T("Right stroke and beater", "Rechter Schlag und Schlägel"), "target": "bass-strike"}, {"t": 0.75, "kind": "load-on", "title": T("Left stroke", "Linker Schlag"), "target": "stick-strike"}],
             "tracks": [{"target": "load:stick-strike", "ease": "linear", "keys": stroke_keys}, {"target": "load:bass-strike", "ease": "linear", "keys": bass_keys}]}
    snares = {"id": "snares-on", "title": T("Snares on", "Teppich anlegen"), "description": T("The throw-off lever of the strainer is turned up through 180 degrees and the snare wires come to lie against the bottom head.", "Der Abhebehebel wird um 180 Grad nach oben gelegt und der Teppich legt sich an das untere Fell."), "durationS": 1.2, "fps": 10, "loop": False, "rootMotion": "none",
              "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 1.0, "pose": "stored-snares-on", "ease": "smooth"}, {"t": 1.2, "pose": "stored-snares-on", "ease": "smooth"}],
              "contacts": [{"anchor": a, "fromS": 0, "toS": 1.2} for a in FEET + ["snare-floor"]],
              "phases": [{"id": "throw", "fromS": 0, "toS": 1.0, "title": T("Lever turned", "Hebel umgelegt")}, {"id": "on", "fromS": 1.0, "toS": 1.2, "title": T("Snares on", "Teppich an")}],
              "events": [{"t": 1.0, "kind": "motion", "title": T("Snares against the head", "Teppich am Fell"), "target": "snare-strainer"}], "tracks": []}
    return [carry, march, snares]


REP = {
    50: dict(
        pens={"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5}, valid=(20, 75),
        silhouette=T("Side elevation from the right: the bass drum seen on its right calfskin head, a 0.67 m disc ringed by its beech hoop with eight claws and T-handles, lying on the tubular cradle whose sleeves touch it at 40 degrees from its bottom, its carrier behind it with the J-bars rising to 1.03 m; in front the snare, 0.38 x 0.33 m, on its bottom hoop with ten tube lugs, the strainer and its lever on the near side, the sticks across the top hoop, its carrier bars leaning against the bass shell. Front and plan show the two drums one behind the other, the bass drum as its 0.43 m deep shell between the hoops with the tension rods.", "Seitenansicht von rechts: die große Trommel auf ihr rechtes Kalbfell gesehen, eine 0,67-m-Scheibe im Buchenreifen mit acht Krallen und Flügeln, auf dem Rohrbock liegend, dessen Hülsen sie 40 Grad neben ihrem tiefsten Punkt berühren, dahinter ihr Gestell mit den J-Bügeln bis 1,03 m; davor die kleine Trommel, 0,38 x 0,33 m, auf ihrem unteren Reifen mit zehn Rohrböckchen, Abhebung und Hebel auf der nahen Seite, die Stöcke quer auf dem oberen Reifen, die Bügel ihres Gestells am Kessel der großen Trommel angelehnt. Vorderansicht und Draufsicht zeigen die Trommeln hintereinander, die große Trommel als ihren 0,43 m tiefen Kessel zwischen den Reifen mit den Spannschrauben."),
        retained=T("Every part: shells, heads, hoops with the snare flange lips, all lugs, ears, claws, rods and T-handles, strainer, butt and lever, mounts, plates, pads, J-bars and slings, both sticks, the cradle tubes with caps and sleeves.", "Jedes Teil: Kessel, Felle, Reifen mit den Bördellippen der kleinen Trommel, alle Böckchen, Ösen, Krallen, Spannschrauben und Flügel, Abhebung, Gegenstück und Hebel, Aufnahmen, Platten, Polster, J-Bügel und Schlingen, beide Stöcke, die Rohre des Bocks mit Kappen und Hülsen."),
        merged=T("Tension rod and ear of each snare lug are one block; the curved vest plates are drawn flat.", "Spannschraube und Öse jedes Böckchens der kleinen Trommel sind ein Block; die gewölbten Bauchplatten sind eben gezeichnet."),
        omitted=T("The snare wires are hidden under the bottom head and not drawn; the vent hole of the shells is left out.", "Der Teppich liegt verborgen unter dem unteren Fell und ist nicht gezeichnet; das Entlüftungsloch der Kessel entfällt."),
        thinElements=T("Heads (0.25 and 0.5 mm) and vest plates (3 mm) are drawn as faces where they are seen face on and are hidden behind the hoops and pads edge on.", "Felle (0,25 und 0,5 mm) und Bauchplatten (3 mm) sind als Flächen gezeichnet, wo man sie von vorn sieht, und verschwinden hinter Reifen und Polstern, wo man sie von der Kante sieht."),
        hatching=T("No hatches; the traces of use are texture strokes: the faded patch and lifted seam of the pearl wrap, the worn coating at the beating spot, the worn chrome of the rimshot zone, the beater spot and a stain on the calfskin, the crack under the hoop cleat, the old inventory sticker and the taped sling.", "Keine Schraffuren; die Gebrauchsspuren sind Texturstriche: die verblasste Stelle und die gelöste Naht der Perlmuttfolie, die abgespielte Beschichtung am Schlagpunkt, das abgeriebene Chrom der Randschlagzone, der Schlägelfleck und ein Fleck auf dem Kalbfell, der Riss unter der Reifenlasche, der alte Inventaraufkleber und die geklebte Schlinge."),
        styles=T("Body pen for shells, hoops, heads, carriers, stand and sticks; detail pen for lugs, claws, rods, ears, strainer, mounts, pads and slings; texture pen for the traces of use.", "Körperstift für Kessel, Reifen, Felle, Gestelle, Bock und Stöcke; Detailstift für Böckchen, Krallen, Spannschrauben, Ösen, Abhebung, Aufnahmen, Polster und Schlingen; Texturstift für die Gebrauchsspuren."),
        overlays=T("The sticks and the strainer lever are revolved and extruded solids, so they swing right in all three views in the carry, march and snares-on clips.", "Stöcke und Abhebehebel sind gedrehte und gezogene Körper, damit sie in den Clips Tragen, Marsch und Teppich anlegen in allen drei Ansichten richtig schwenken.")),
    100: dict(
        pens={"body": 0.25, "detail": 0.18, "texture": None, "glazing": None, "ground": 0.35}, valid=(75, 150),
        silhouette=T("The same arrangement at 1:100: the bass drum face with its hoop, the tension points as lug and claw blocks, the cradle tubes and sleeves, the carriers with bars and slings, the snare with its hoops, Ø15 tube lugs, strainer and the sticks.", "Dieselbe Anordnung bei 1:100: die Stirnseite der großen Trommel mit Reifen, die Spannpunkte als Böckchen- und Krallenblöcke, Rohre und Hülsen des Bocks, die Gestelle mit Bügeln und Schlingen, die kleine Trommel mit Reifen, Rohrböckchen Ø15, Abhebung und den Stöcken."),
        retained=T("Shells, heads, hoops, lugs, the tension points of the bass drum, strainer and butt, mounts, plate blocks, J-bars, slings, sticks, all cradle tubes and sleeves.", "Kessel, Felle, Reifen, Böckchen, die Spannpunkte der großen Trommel, Abhebung und Gegenstück, Aufnahmen, Plattenblöcke, J-Bügel, Schlingen, Stöcke, alle Rohre und Hülsen des Bocks."),
        merged=T("Lug, rod, claw and T-handle of a bass tension point are two blocks; plate and belly pad are one 15 mm block; the end caps belong to the feet; the snare ears belong to the tube lugs.", "Böckchen, Spannschraube, Kralle und Flügel eines Spannpunkts sind zwei Blöcke; Platte und Bauchpolster ein 15-mm-Block; die Endkappen gehören zu den Füßen; die Ösen der kleinen Trommel zu den Rohrböckchen."),
        omitted=T("The flange lips of the snare hoops, the hoop cleat of the bass drum, the strainer lever and the snare wires.", "Die Bördellippen der Reifen der kleinen Trommel, die Reifenlasche der großen Trommel, der Abhebehebel und der Teppich."),
        thinElements=T("Heads are faces seen face on; plates are part of their 15 mm blocks.", "Felle sind von vorn gesehene Flächen; die Platten gehören zu ihren 15-mm-Blöcken."),
        hatching=T("No hatches; the worn beating spot of the batter head and the beater spot of the calfskin are detail strokes.", "Keine Schraffuren; der abgespielte Schlagpunkt des Schlagfells und der Schlägelfleck des Kalbfells sind Detailstriche."),
        styles=T("Body pen for the drums, carriers, stand and sticks, detail pen for hardware, pads and slings.", "Körperstift für Trommeln, Gestelle, Bock und Stöcke, Detailstift für Beschläge, Polster und Schlingen."),
        overlays=T("Sticks as revolved solids that swing in the clips.", "Stöcke als gedrehte Körper, die in den Clips schwenken.")),
    200: dict(
        pens={"body": 0.18, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.25}, valid=(150, 350),
        silhouette=T("A drum pair 7.7 mm long on paper: the bass drum disc with its hoop ring and eight tension blocks on the cradle drawn as feet, posts, cross tubes and two V-arms, the carriers as a plate block with two J-bars and slings, the snare as a cylinder with two hoop bands and the sticks.", "Ein Trommelpaar, 7,7 mm lang auf dem Papier: die Scheibe der großen Trommel mit Reifenring und acht Spannblöcken auf dem Bock aus Füßen, Stützen, Querrohren und zwei V-Armen, die Gestelle als Plattenblock mit zwei J-Bügeln und Schlingen, die kleine Trommel als Zylinder mit zwei Reifenbändern und die Stöcke."),
        retained=T("Drum bodies with hoop bands, the bass tension points, cradle, mounts, plate blocks, J-bars, slings, sticks.", "Trommelkörper mit Reifenbändern, die Spannpunkte der großen Trommel, Bock, Aufnahmen, Plattenblöcke, J-Bügel, Schlingen, Stöcke."),
        merged=T("The tension points of the bass drum are eight 30 mm blocks a side, the snare hoops 30 mm bands; mount and mount bar one block; arms and sleeves of the cradle one V-tube a side.", "Die Spannpunkte der großen Trommel sind acht 30-mm-Blöcke je Seite, die Reifen der kleinen Trommel 30-mm-Bänder; Aufnahme und Haltearm ein Block; Arme und Hülsen des Bocks ein V-Rohr je Seite."),
        omitted=T("Tube lugs, strainer, butt and lever of the snare, the cleat, the end caps and all traces of use.", "Rohrböckchen, Abhebung, Gegenstück und Hebel der kleinen Trommel, die Lasche, die Endkappen und alle Gebrauchsspuren."),
        thinElements=T("Bars, feet, V-arms and sticks keep their true section and print with the body pen.", "Bügel, Füße, V-Arme und Stöcke behalten ihren wahren Querschnitt und drucken mit dem Körperstift."),
        hatching=T("No hatches.", "Keine Schraffuren."),
        styles=T("Body pen for drums, carriers, stand and sticks, detail pen for the tension blocks, mounts and slings.", "Körperstift für Trommeln, Gestelle, Bock und Stöcke, Detailstift für Spannblöcke, Aufnahmen und Schlingen."),
        overlays=T("Sticks as revolved solids that swing in the clips.", "Stöcke als gedrehte Körper, die in den Clips schwenken.")),
    500: dict(
        pens={"body": 0.14, "detail": None, "texture": None, "glazing": None, "ground": 0.18}, valid=(350, 1000),
        silhouette=T("A drum pair 3.1 mm long on paper: the bass drum disc on the saddle of its cradle, the snare cylinder in front, the carriers as their J-bar strokes and plates, the sticks as strokes.", "Ein Trommelpaar, 3,1 mm lang auf dem Papier: die Scheibe der großen Trommel auf dem Sattel ihres Bocks, davor der Zylinder der kleinen Trommel, die Gestelle als Strich ihrer J-Bügel und Platten, die Stöcke als Striche."),
        retained=T("Both drum bodies with 75 mm hoop bands, the cradle envelope, mount blocks, plates, J-bars and sticks.", "Beide Trommelkörper mit 75-mm-Reifenbändern, die Hülle des Bocks, Aufnahmeblöcke, Platten, J-Bügel und Stöcke."),
        merged=T("The cradle is its envelope saddle; the head faces carry the regions of hoops and hardware; the plates carry the pads.", "Der Bock ist sein Hüllsattel; die Fellflächen tragen die Regionen von Reifen und Beschlägen; die Platten tragen die Polster."),
        omitted=T("Tension hardware, slings, all small parts and the sticks in the front view, where they are seen end on.", "Spannteile, Schlingen, alle Kleinteile und die Stöcke in der Vorderansicht, wo man sie von der Spitze sieht."),
        thinElements=T("J-bars, the edge-on plates and the sticks are single body strokes of 0.14 mm.", "J-Bügel, die von der Kante gesehenen Platten und die Stöcke sind einzelne Körperstriche von 0,14 mm."),
        hatching=T("No hatches.", "Keine Schraffuren."),
        styles=T("One body pen.", "Ein Körperstift."),
        overlays=T("None.", "Keine.")),
}


def representation(scale, parts):
    """🖨️ The authored representation of one scale."""
    d = REP[scale]
    geom = draw_scale(scale, parts)
    keys = ("silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays")
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": d["valid"][0], "maxDenominator": d["valid"][1]}, "pens": d["pens"], "hatches": [], "geometry": geom, "decisions": {k: d[k] for k in keys}}


def build(areas=None):
    """🏗️ The whole asset document."""
    areas = areas or {}
    p50 = parts_50()
    masses = masses_of(p50)
    (x0, x1), (y0, y1), (z0, z1) = extents(p50)
    total = sum(kg for kg, _ in masses.values())
    parts = {s: f() for s, f in SCALES.items()}
    asset = {
        "id": "used-marching-snare-and-bass-drum",
        "category": "objects",
        "title": T("Used Marching Drums", "Gebrauchte Marschtrommeln"),
        "summary": T(SUMMARY_EN.format(total=f"{total:.1f}"), SUMMARY_DE.format(total=f"{total:.1f}".replace(".", ","))),
        "tags": ["used", "instrument", "music", "second-hand", "percussion", "marching", "snare-drum", "bass-drum", "membranophone", "calfskin", "carrier", "band"],
        "provenance": {"author": "Semio staffage content (used instruments wave)", "method": "handcrafted", "created": "2026-11-22", "references": [{"id": i, "citation": c} for i, c in REFERENCES], "notes": T(NOTES_EN.format(total=total), NOTES_DE.format(total=f"{total:.1f}".replace(".", ",")))},
        "dimensions": {"heightM": round(y1 - y0, 3), "widthM": round(x1 - x0, 3), "depthM": round(z1 - z0, 3), "massKg": round(total, 2)},
        "materials": [{"id": m[0], "title": T(m[1], m[2]), "emissivity": m[3], "solarAbsorptivity": m[4], "densityKgM3": m[5], "conductivityWmK": m[6], "specificHeatJKgK": m[7], "source": m[8], "reference": m[9]} for m in MATERIALS],
        "rig": {"kind": "mechanical", "root": "stand", "bones": bones_json(masses)},
        "volumes": {str(s): volume_of(s, parts[s], masses) for s in SCALES},
        "anchors": ANCHORS,
        "representations": {str(s): representation(s, parts[s]) for s in SCALES},
        "symbols": [],
        "poses": poses_json(),
        "clips": clips_json(),
        "thermal": {"environment": {"airTempC": 18, "meanRadiantTempC": 18, "relativeHumidity": 0.45, "windMS": 0.05, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325}, "passengers": [], "operating": [], "regions": regions_json(areas), "emitters": [], "assumptions": THERMAL_ASSUMPTIONS},
        "structure": {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": STRUCTURE_ASSUMPTIONS, "clinical": False},
        "presentation": {"defaultPose": "stored", "defaultClip": "march", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}, masses, ((x0, x1), (y0, y1), (z0, z1))


SUMMARY_EN = "A second-hand marching pair from a band depot: a 14 x 12 in rod-tension snare drum with faded pearl wrap and worn chrome hoops and a 24 x 14 in bass drum with calfskin heads, a cracked and cleated beech hoop and tarnished T-rods, each on its aluminium shoulder carrier with foam slings, {total} kg with the tubular cradle the bass drum is stored on; stored, carried and played on the march."
SUMMARY_DE = "Ein gebrauchtes Marschtrommelpaar aus dem Lager eines Musikvereins: eine kleine Trommel 14 x 12 Zoll mit Spannschrauben, verblasster Perlmuttfolie und abgeriebenen Chromreifen und eine große Trommel 24 x 14 Zoll mit Kalbfellen, einem gerissenen und gelaschten Buchenreifen und angelaufenen Flügelschrauben, jede an ihrem Aluminium-Schultergestell mit Schaumschlingen, {total} kg mit dem Rohrbock, auf dem die große Trommel lagert; gelagert, getragen und auf dem Marsch gespielt."
NOTES_EN = "A pair of used marching drums of the type a brass band or a drum corps stored in its depot from the 1970s to the 1990s, no brand: a 14 x 12 in (356 x 305 mm) snare drum of six-ply birch with a pearl wrap, triple-flanged chromed steel hoops, ten tube lugs a side and a parallel strainer, and a 24 x 14 in (610 x 356 mm) bass drum of eight-ply lacquered birch with calfskin heads, beech counter hoops and eight T-rods a side. Each hangs on an aluminium tube shoulder carrier (vest plate, belly pad, two J-bars of 22 x 2 mm with foam slings) sized for a standing shoulder height of 1.45 m and a waist of 1.0 m (DIN 33402-2). The bass drum is stored on a tubular steel cradle that stays behind when it is worn. Hornbostel-Sachs: both are double-skin cylindrical drums, 211.212.1; the snare has its snares on the bottom head. The pair weighs {total:.1f} kg with the cradle; every mass is section by density of the modelled parts. Traces of use: faded and lifted wrap, worn chrome at the rimshot zone, the coating of the batter head played off at the beating spot, a stained calfskin with its beater spot, a crack in the right bass hoop under a glued beech cleat, tarnished plating, an old inventory sticker and a taped sling; all playable."
NOTES_DE = "Ein Paar gebrauchter Marschtrommeln von der Art, wie ein Blasorchester oder ein Spielmannszug sie von den 1970er bis in die 1990er Jahre in seinem Lager aufbewahrte, ohne Marke: eine kleine Trommel 14 x 12 Zoll (356 x 305 mm) aus sechslagiger Birke mit Perlmuttfolie, dreifach gebördelten verchromten Stahlreifen, zehn Rohrböckchen je Seite und einer Parallelabhebung, und eine große Trommel 24 x 14 Zoll (610 x 356 mm) aus achtlagiger lackierter Birke mit Kalbfellen, Buchen-Spannreifen und acht Flügelschrauben je Seite. Jede hängt an einem Schultergestell aus Aluminiumrohr (Bauchplatte, Bauchpolster, zwei J-Bügel 22 x 2 mm mit Schaumschlingen) für eine Schulterhöhe im Stand von 1,45 m und eine Taille von 1,0 m (DIN 33402-2). Die große Trommel lagert auf einem Rohrbock aus Stahl, der zurückbleibt, wenn sie getragen wird. Hornbostel-Sachs: beide sind zweifellige Zylindertrommeln, 211.212.1; die kleine Trommel hat ihren Teppich am unteren Fell. Das Paar wiegt mit dem Bock {total} kg; jede Masse ist Querschnitt mal Dichte der modellierten Teile. Gebrauchsspuren: verblasste und gelöste Folie, abgeriebenes Chrom in der Randschlagzone, die am Schlagpunkt abgespielte Beschichtung des Schlagfells, ein fleckiges Kalbfell mit Schlägelfleck, ein Riss im rechten Reifen der großen Trommel unter einer aufgeleimten Buchenlasche, angelaufene Vernickelung, ein alter Inventaraufkleber und eine geklebte Schlinge; alles spielbar."

THERMAL_ASSUMPTIONS = [
    T("Acoustic instruments: no element draws power; both drums state their nameplate as passive 0 W. All regions are passive surfaces in an unheated band depot or rehearsal room at 18 degC without sun; worn, the pads touch the drummers' bodies (33 degC on 45 % of their area), the carrier frames 5 % and the stick grips 30 % (hands at 33 degC), which warms the plated metal and the hickory where they are held.", "Akustische Instrumente: kein Element nimmt Leistung auf; beide Trommeln geben ihr Typenschild als passiv 0 W an. Alle Regionen sind passive Oberflächen in einem unbeheizten Vereinslager oder Proberaum bei 18 °C ohne Sonne; getragen berühren die Polster die Körper der Trommler (33 °C auf 45 % ihrer Fläche), die Gestelle zu 5 % und die Stockgriffe zu 30 % (Hände bei 33 °C), was das Metall und das Hickory dort erwärmt, wo sie gehalten werden."),
    T("Aged surfaces: tarnished nickel and pitted chrome radiate more than new plating (emissivity 0.15 and 0.12 against 0.05 and 0.07, authored estimates after Incropera), worn anodising less than new (0.7 against 0.8); crazed lacquer and faded wrap keep the emissivity of new lacquer (0.9).", "Gealterte Oberflächen: angelaufenes Nickel und Chrom mit Lochfraß strahlen mehr als neue Beschichtung (Emissionsgrad 0,15 und 0,12 gegen 0,05 und 0,07, Schätzwerte nach Incropera), abgeriebene Eloxierung weniger als neue (0,7 gegen 0,8); rissiger Lack und verblasste Folie behalten den Emissionsgrad neuen Lacks (0,9)."),
]

STRUCTURE_ASSUMPTIONS = [
    T("The asset is solved as one instantaneously rigid body on the contacts of the pose. Stored: the four rubber caps of the cradle and the bottom hoop of the snare on the floor; the bass drum rests on the four rubber sleeves of the cradle arms. Carried and played: the slings on the shoulders and the belly pads of the two drummers (DIN 33402-2 heights); the cradle stays on the floor on its caps.", "Das Objekt wird als ein augenblicklich starrer Körper auf den Kontakten der Stellung gelöst. Gelagert: die vier Gummikappen des Bocks und der untere Reifen der kleinen Trommel auf dem Boden; die große Trommel liegt auf den vier Gummihülsen der Bockarme. Getragen und gespielt: die Schlingen auf den Schultern und die Bauchpolster der beiden Trommler (Höhen nach DIN 33402-2); der Bock bleibt auf seinen Kappen am Boden."),
    T("Head tension is internal: a marching snare batter at about 2 kN/m of membrane tension pulls about 2.3 kN round its hoop, a calfskin bass head about 1.5 kN; rods, hoops and the shell carry it as a self-equilibrated ring and it is no load case. Playing loads: 45 N peaks of the stick strokes on the batter head and 35 N of the beater on the right calfskin head (authored estimates of loud marching strokes after Rossing).", "Die Fellspannung ist innere Kraft: ein Marsch-Schlagfell mit etwa 2 kN/m Membranspannung zieht rund 2,3 kN um seinen Reifen, ein Kalbfell der großen Trommel rund 1,5 kN; Spannschrauben, Reifen und Kessel tragen sie als in sich ausgeglichenen Ring, sie ist kein Lastfall. Spiellasten: Spitzen von 45 N der Stockschläge auf dem Schlagfell und 35 N des Schlägels auf dem rechten Kalbfell (Schätzwerte lauter Marschschläge nach Rossing)."),
]


def main():
    """🚀 Writes the candidate and prints the derived numbers."""
    areas_file = os.path.join(HERE, "author-areas.json")
    areas = json.load(open(areas_file)) if os.path.exists(areas_file) else {}
    doc, masses, ext = build(areas)
    with open(os.path.join(HERE, "🔣️.json"), "w") as f:
        json.dump(doc, f, ensure_ascii=False, indent=1)
    for b, (kg, com) in sorted(masses.items()):
        print(f"[DEBUG] mass {b:16s} {kg:7.3f} kg  com {tuple(round(c, 3) for c in com)}")
    print("[DEBUG] extents x", [round(v, 4) for v in ext[0]], "y", [round(v, 4) for v in ext[1]], "z", [round(v, 4) for v in ext[2]])
    print("[DEBUG] dimensions", doc["asset"]["dimensions"])


if __name__ == "__main__":
    main()
