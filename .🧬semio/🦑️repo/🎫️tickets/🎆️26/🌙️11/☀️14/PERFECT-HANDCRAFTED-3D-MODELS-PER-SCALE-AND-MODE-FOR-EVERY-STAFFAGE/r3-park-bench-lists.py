"""🌡 Round 3, furniture/park-bench: the masses of the modelled parts, the thermal and the structural list of every scale and the bilingual decisions.

Thermal: every body surface is the surface of its region (seat slats, back slats, the steel of the end frames with armrests, pads and bolt heads), one layer each at 1:50. Structure: the load path only: the end frames (the casting itself, `of` its web and rims or plate) as the members that carry seat and back to the floor, the seat slats as the beams spanning 1.56 m between the frames with 0.12 m overhangs, the back slats as the beams on the back posts; the carriage bolts that join every slat to each frame as the joints; the foot pads as the supports of the two contact anchors; one mass per bone at its centre.
"""
import importlib.util
import math
import os

from shapely.geometry import Polygon

HERE = os.path.dirname(os.path.abspath(__file__))


def _load(name):
    spec = importlib.util.spec_from_file_location("r3_park_bench_" + name, os.path.join(HERE, f"r3-park-bench-{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


K = _load("kit")
B = _load("body")

BOLT_KG = 0.04
ANCHORS = (("l", "end-left", "contact-left"), ("r", "end-right", "contact-right"))


def superellipse_area(a, b, n):
    """📐 Area of |x/a|^n + |y/b|^n <= 1."""
    return 4 * a * b * math.gamma(1 + 1 / n) ** 2 / math.gamma(1 + 2 / n)


def masses():
    """⚖ Mass (kg) and world centre (x, y, z) of every bone from the 1:50 model: steel 7850, oak 700 kg/m3; carriage bolts M8 with nut and washer 0.04 kg each, counted with the slats they hold."""
    prof = K.frame_profile(True)
    core, rims = K.i_section(prof)
    parts = [(core.area * K.WEB, core.centroid.coords[0])]
    parts += [(r.area * K.FLANGE, r.centroid.coords[0]) for r in rims]
    arm_len = K.arm_reach() - K.ARM_FRONT
    parts.append((superellipse_area(0.0175, 0.025, 2.6) * arm_len, (K.ARM_FRONT + arm_len / 2, (K.ARM_Y0 + K.ARM_Y1) / 2)))
    pad = K.PAD_H * K.PAD_L * K.PAD_X
    parts += [(pad, (K.FRONT_Z, K.PAD_H / 2)), (pad, (K.REAR_FOOT_Z, K.PAD_H / 2))]
    vol = sum(v for v, _ in parts)
    fz = sum(v * c[0] for v, c in parts) / vol
    fy = sum(v * c[1] for v, c in parts) / vol
    frame = (vol * K.RHO[K.STEEL], fz, fy)
    out = {"end-left": (frame[0], K.FRAME_X[0], fy, fz), "end-right": (frame[0], K.FRAME_X[1], fy, fz)}
    for bone, sections, bolts in (("seat", K.seat_slats(), 2 * K.SEAT_N), ("backrest", K.back_slats(), 2 * len(K.BACK_S))):
        polys = [Polygon(s) for s in sections]
        area = sum(p.area for p in polys)
        cz = sum(p.area * p.centroid.x for p in polys) / area
        cy = sum(p.area * p.centroid.y for p in polys) / area
        out[bone] = (area * K.LENGTH * K.RHO[K.OAK] + bolts * BOLT_KG, K.LENGTH / 2, cy, cz)
    return {k: (round(v[0], 1), v[1], v[2], v[3]) for k, v in out.items()}


def region(sid):
    """🏷 The thermal region and layer of a body solid."""
    if sid.startswith(("seat-slat", "seat-board", "seat-slab")):
        return "seat-timber", "oak-slat"
    if sid.startswith(("back-slat", "back-board", "back-slab")):
        return "back-timber", "oak-slat"
    return "steel-frame", "cast-style-steel"


def thermal(scale, body):
    """🌡 The thermal list of a scale: a reference per body solid."""
    out = []
    for solid in body:
        name, layer = region(solid["id"])
        entry = {"id": "th-" + solid["id"], "of": solid["id"], "region": name}
        if scale == "50":
            entry["layer"] = layer
        out.append(entry)
    return out


def of(sid, member, part="member"):
    """🔗 A structural reference to a body solid that is the member itself."""
    return {"id": "m-" + sid, "of": sid, "member": member, "part": part}


def own(solid, member, part, **more):
    """🧩 An own structural solid."""
    solid.update({"member": member, "part": part}, **more)
    return solid


def pin(sid, bone, origin, axis, radius, length, tone=0.25, head=True):
    """🔩 A carriage bolt as a joint: the domed head (2.4 radii) standing on the face at `origin`, the shank of `radius` along the unit (z, y) `axis` through slat and flange for `length`, and the nut (1.6 radii) under the flange, so the joint shows on both sides of the parts it clamps."""
    h, nut = 2.4 * radius, 1.6 * radius
    profile = [[-0.6 * h, 0.6 * h], [-0.45 * h, h], [0, h], [0, radius], [length, radius], [length, nut], [length + nut, nut]] if head else [[0, radius], [length, radius]]
    out = K.base(sid, "revolve", bone, K.STEEL, tone, "detail")
    out.update({"profile": [[K.r6(a), K.r6(b)] for a, b in profile], "axis": [0, K.r6(axis[1]), K.r6(axis[0])], "origin": [K.r6(v) for v in origin]})
    return out


def seat_joints(radius, length, sy, merged=False, head=True):
    """🔩 The bolts of the seat slats into each frame's rail flange (one per slat and frame; `merged`: one per frame through the board's middle)."""
    out = []
    down = (-K.N[0], -K.N[1])
    step = K.SEAT_W + K.SEAT_GAP
    if merged:
        z, y = K.seat_point((K.SEAT_S[0][0] + K.SEAT_S[-1][1]) / 2, K.SEAT_T)
    else:
        z, y = K.seat_point((K.SEAT_S[0][0] + K.SEAT_S[0][1]) / 2, K.SEAT_T)
    for p, x in zip(("l", "r"), K.FRAME_X):
        j = pin(f"bolt-seat-{p}", "seat", (x, y - sy, z), down, radius, length, head=head)
        if not merged:
            j["repeat"] = {"count": K.SEAT_N, "translation": [0, K.r6(K.D[1] * step), K.r6(K.D[0] * step)]}
        out.append(own(j, "seat", "joint"))
    return out


def back_joints(radius, length, by, bands, head=True):
    """🔩 The bolts of the back slats into each post (one per slat band and frame)."""
    out = []
    back = (-K.M[0], -K.M[1])
    for k, (t0, t1) in enumerate(bands):
        z, y = K.add(K.post_front((t0 + t1) / 2), K.M, K.BACK_T)
        j = pin(f"bolt-back-{k + 1}", "backrest", (K.FRAME_X[0], y - by, z), back, radius, length, head=head)
        j["repeat"] = {"count": 2, "translation": [K.r6(K.FRAME_X[1] - K.FRAME_X[0]), 0, 0]}
        out.append(own(j, "backrest", "joint"))
    return out


def supports(height, length, width):
    """🟢 The contact pads under the front and rear foot of each frame, one solid per anchor."""
    out = []
    for p, bone, anchor in ANCHORS:
        pad = B.pads("support-" + anchor, bone, height, length, width, radius=0, tone=0.2, role="ground")
        out.append(own(pad, bone, "support", anchor=anchor))
    return out


def lumps(r, r_frame):
    """⚫ The mass of every bone at its centre; the end frames' centre lies in the frame plane among the seat slats, so their marker (`r_frame`) stays within the seat it shows through."""
    out = []
    sy, by = K.seat_y(), K.back_y()
    for bone, (kg, x, y, z) in masses().items():
        if bone.startswith("end"):
            ox, oy = (K.FRAME_X[0] if bone == "end-left" else K.FRAME_X[1]), 0.0
            centre = [y, -(x - ox), z]
        else:
            centre = [x, y - (sy if bone == "seat" else by), z]
        e = K.base("mass-" + bone, "ellipsoid", bone, K.STEEL if bone.startswith("end") else K.OAK, 0.3, "detail")
        rr = r_frame if bone.startswith("end") else r
        e["radii"] = [rr, rr, rr]
        e["transform"] = {"translation": [K.r6(v) for v in centre]}
        out.append(own(e, bone, "mass"))
    return out


def structure(scale, body):
    """🏗 The structural list of a scale."""
    out = []
    sy, by = K.seat_y(), K.back_y()
    for solid in body:
        sid = solid["id"]
        if sid.startswith(("l-web", "l-rim", "l-frame")):
            out.append(of(sid, "end-left"))
        elif sid.startswith(("r-web", "r-rim", "r-frame")):
            out.append(of(sid, "end-right"))
        elif sid.startswith("seat-") and "bolt" not in sid:
            out.append(of(sid, "seat"))
        elif sid.startswith("back-") and "bolt" not in sid:
            out.append(of(sid, "backrest"))
    if scale == "50":
        out += seat_joints(0.004, K.SEAT_T + K.RIM + 0.008, sy)
        out += back_joints(0.004, K.BACK_T + K.RIM + 0.008, by, K.BACK_S)
        out += supports(0.0075, K.PAD_L, K.PAD_X)
        out += lumps(0.022, 0.02)
    elif scale == "100":
        out += seat_joints(0.0075, K.SEAT_T + 0.02, sy)
        out += back_joints(0.0075, K.BACK_T + 0.02, by, K.BACK_S)
        out += supports(0.015, K.PAD_L, K.PAD_X)
        out += lumps(0.035, 0.025)
    elif scale == "200":
        out += seat_joints(0.015, K.SEAT_T + 0.03, sy, merged=True)
        out += back_joints(0.015, K.BACK_T + 0.03, by, [K.BACK_S[0], (K.BACK_S[1][0], K.BACK_S[2][1])])
        out += supports(0.03, 0.07, 0.07)
        out += lumps(0.04, 0.04)
    else:
        out += seat_joints(0.0375, 0.075, sy, merged=True, head=False)
        out += back_joints(0.0375, 0.075, by, [(K.BACK_S[0][0], K.BACK_S[2][1])], head=False)
        out += supports(0.075, 0.075, 0.075)
        out += lumps(0.055, 0.055)
    return out


def T(en, de):
    """🌐 A bilingual text."""
    return {"en": en, "de": de}


DECISIONS = {
    "50": [
        T("Body, authored for 7.5 mm: two one-piece cast end frames of I-section (40 mm flange rims around the outline and around both openings, 10 mm web), each with front leg and arm post in one, the seat rail raked 5 degrees, the back post raked 8 degrees, the rear leg kicked back for stability, a lower brace, the arm band under a 50 x 35 mm rounded armrest with its knuckle, and a 70 x 70 x 16 mm pad under each foot; four 100 x 45 mm oak seat slats with 10 mm drainage gaps and three 80 x 35 mm oak back slats, all with 6 mm eased edges, overhanging the frames by 0.12 m; the armrest meets the post in the 90 mm gap between the lumbar slat and the upper slats; the domed heads of the M8 carriage bolts (one per slat and frame) as texture.",
          "Körper, für 7,5 mm gebaut: zwei einteilige gegossene Endgestelle im I-Querschnitt (40-mm-Randflansch um den Umriss und beide Öffnungen, 10-mm-Steg), je mit Vorderbein und Armlehnenstütze in einem Stück, um 5 Grad geneigter Sitzzarge, um 8 Grad geneigtem Rückenpfosten, zur Standsicherheit nach hinten ausgestelltem Hinterbein, unterer Strebe, Armband unter einer gerundeten Armlehne 50 x 35 mm mit Knauf und einer Fußplatte 70 x 70 x 16 mm unter jedem Fuß; vier Eichen-Sitzlatten 100 x 45 mm mit 10-mm-Wasserfugen und drei Eichen-Rückenlatten 80 x 35 mm, alle mit 6 mm gebrochenen Kanten, 0,12 m über die Gestelle auskragend; die Armlehne trifft den Pfosten in der 90-mm-Fuge zwischen Lendenlatte und oberen Latten; die gewölbten Köpfe der M8-Schlossschrauben (eine je Latte und Gestell) als Textur."),
        T("Thermal: every region is the surface of its body solids with its one layer (45 mm and 35 mm oak slats, 10 mm cast steel); no hidden regions, no emitters. Structure: the end frames (web and rims of the casting) carry seat and back to the floor; the seat slats span 1.56 m between the frames, the back slats lie on the back posts; the M8 carriage bolts of every slat at each frame are the joints; the two foot pads of each frame are the support of its contact anchor; each bone carries its mass at its centre.",
          "Thermik: jede Region ist die Oberfläche ihrer Körper mit ihrer einen Schicht (Eichenlatten 45 mm und 35 mm, Stahlguss 10 mm); keine verdeckten Regionen, keine Emitter. Tragwerk: die Endgestelle (Steg und Flansche des Gusses) tragen Sitz und Lehne zum Boden; die Sitzlatten spannen 1,56 m zwischen den Gestellen, die Rückenlatten liegen auf den Rückenpfosten; die M8-Schlossschrauben jeder Latte an jedem Gestell sind die Gelenke; die zwei Fußplatten jedes Gestells sind das Auflager seines Kontaktankers; jeder Knochen trägt seine Masse im Schwerpunkt."),
    ],
    "100": [
        T("Body, authored anew for 15 mm: each end frame as one 40 mm cast plate with the same outline and both openings (the 15 mm recess of the web is below the feature), the armrest as a straight rounded bar (the knuckle merged), the foot pads kept; four square-edged seat slats with their 10 mm gaps (on paper the 0.15 mm print minimum) and three square-edged back slats; the bolt heads are omitted.",
          "Körper, für 15 mm neu gebaut: jedes Endgestell als eine 40-mm-Gussplatte mit gleichem Umriss und beiden Öffnungen (die 15-mm-Vertiefung des Stegs liegt unter dem Merkmal), die Armlehne als gerader gerundeter Stab (der Knauf verschmolzen), die Fußplatten bleiben; vier scharfkantige Sitzlatten mit ihren 10-mm-Fugen (auf Papier das Druckminimum 0,15 mm) und drei scharfkantige Rückenlatten; die Schraubenköpfe entfallen."),
        T("Thermal: one surface per region. Structure: frame plates, seat and back slats as members; one 15 mm bolt per slat and frame as the joints; 15 mm contact pads under both feet of each frame; masses at the centres.",
          "Thermik: eine Oberfläche je Region. Tragwerk: Gestellplatten, Sitz- und Rückenlatten als Stäbe; je Latte und Gestell eine 15-mm-Schraube als Gelenk; 15-mm-Kontaktplatten unter beiden Füßen jedes Gestells; Massen in den Schwerpunkten."),
    ],
    "200": [
        T("Body, authored anew for 30 mm: each end frame a 40 mm plate whose members are at least 30 mm, the legs reaching the floor with their feet (the 16 mm pads merged), the armrest merged into the arm band; the seat as one raked 45 mm board (the 10 mm gaps closed); the back as the lumbar slat and one board for the two upper slats (their 30 mm gap closed); the 90 mm gap where the armrest meets the post stays open.",
          "Körper, für 30 mm neu gebaut: jedes Endgestell eine 40-mm-Platte mit Gliedern von mindestens 30 mm, die Beine mit ihren Füßen bis zum Boden (die 16-mm-Platten verschmolzen), die Armlehne im Armband aufgegangen; der Sitz als ein geneigtes 45-mm-Brett (die 10-mm-Fugen geschlossen); die Lehne als Lendenlatte und ein Brett aus den zwei oberen Latten (ihre 30-mm-Fuge geschlossen); die 90-mm-Fuge, in der die Armlehne den Pfosten trifft, bleibt offen."),
        T("Thermal: one surface per region. Structure: plates and boards as members; one 30 mm bolt per board and frame as the joints; 30 mm contact pads under the feet; masses at the centres.",
          "Thermik: eine Oberfläche je Region. Tragwerk: Platten und Bretter als Stäbe; je Brett und Gestell eine 30-mm-Schraube als Gelenk; 30-mm-Kontaktplatten unter den Füßen; Massen in den Schwerpunkten."),
    ],
    "500": [
        T("Body, authored anew for 75 mm: each end frame a 75 mm plate of 75 mm members (front column up to the armrest, seat rail, rear leg rising into the back, arm band; the brace and the slender legs merged); the seat a flat 75 mm slab (the 38 mm rake is below the feature); the back one 75 mm slab of the posts and the slats together (36 + 35 mm), raked 8 degrees from the seat to the top. The model is minimal: two frames, a seat and a back; nothing smaller can be left out without losing the bench.",
          "Körper, für 75 mm neu gebaut: jedes Endgestell eine 75-mm-Platte aus 75-mm-Gliedern (vordere Stütze bis zur Armlehne, Sitzzarge, in die Lehne aufsteigendes Hinterbein, Armband; Strebe und schlanke Beine verschmolzen); der Sitz eine flache 75-mm-Platte (die 38-mm-Neigung liegt unter dem Merkmal); die Lehne eine 75-mm-Platte aus Pfosten und Latten zusammen (36 + 35 mm), um 8 Grad geneigt vom Sitz bis oben. Das Modell ist minimal: zwei Gestelle, ein Sitz und eine Lehne; kleiner geht es nicht, ohne die Bank zu verlieren."),
        T("Thermal: one surface per region. Structure: frames, seat slab and back slab as members; one 75 mm joint where the seat and where the back cross each frame; 75 mm contact pads; masses at the centres.",
          "Thermik: eine Oberfläche je Region. Tragwerk: Gestelle, Sitz- und Lehnenplatte als Stäbe; je ein 75-mm-Gelenk, wo Sitz und Lehne jedes Gestell kreuzen; 75-mm-Kontaktplatten; Massen in den Schwerpunkten."),
    ],
}


def decisions(scale):
    """💬 The bilingual decisions of a scale."""
    return list(DECISIONS[scale])


if __name__ == "__main__":
    for k, v in masses().items():
        print(k, v)
    print("total", round(sum(v[0] for v in masses().values()), 1))
