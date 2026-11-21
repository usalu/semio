"""🏗️ Round 3 storage crate stack: every solid of every scale and list, stated by hand from the crate geometry of the kit, written to volumes/objects/storage-crate-stack.json."""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(HERE, "r3-storage-crate-stack-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)

N = 5
MAT = "hdpe-crate"
OUT = os.path.join(HERE, "volumes", "objects", "storage-crate-stack.json")
ARCS = {"50": 6, "100": 4, "200": 3, "500": 2}


def text(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def tones(k):
    """🎨 Body, rim and foot tones of crate k, alternating like the drawings."""
    return (0.45, 0.55, 0.35) if k % 2 else (0.55, 0.65, 0.45)


def ext(sid, k, profile, u0, u1, tone, holes=None, taper=True, role="body", fix=None):
    """🧱 A profile on the (v, w) plane pushed up the crate from u0 to u1, drafted with the wall taper (scaled about its centroid) unless taper is off."""
    s = {"id": sid, "kind": "extrude", "bone": f"crate-{k}", "material": MAT, "role": role, "tone": tone, "profile": profile}
    if holes:
        s["holes"] = holes
    s.update({"plane": "yz", "from": round(u0, 4), "to": round(u1, 4)})
    if taper:
        s["taperTo"] = [round(K.f(u0), 6), round(K.f(u1), 6)]
    if fix:
        s["transform"] = {"translation": fix}
    return s


def drafted_piece(sid, k, profile, u0, u1, tone):
    """🫳 A band piece off the axis: the taper scales about the piece's own centroid, so a translation by half the drift brings it back onto the draft of the wall (residual +-0.8 mm)."""
    cv, cw = K.centroid(profile)
    m = 1 - (K.f(u0) + K.f(u1)) / 2
    return ext(sid, k, profile, u0, u1, tone, fix=[0, round(-m * cv, 6), round(-m * cw, 6)])


def box(sid, k, size, at, tone, role="body", **extra):
    """📦 A box centred at `at` (u, v, w)."""
    s = {"id": sid, "kind": "box", "bone": f"crate-{k}", "material": MAT, "role": role, "tone": tone, "size": [round(x, 5) for x in size], "transform": {"translation": [round(x, 5) for x in at]}}
    s.update(extra)
    return s


def ring(hv, hw, r, ihv, ihw, ir, n):
    """⭕ Outer and inner rounded rectangle of a rim ring."""
    return K.outline(hv, hw, r, n=n), [K.outline(ihv, ihw, ir, n=n)]


def ledges(k, p, tone, u0):
    """🪜 The three stacking ledges at the top inside the crate: one in each end wall (+v and -v, over w0..w1) and one in the -w long wall (over v0..v1), each from the wall face inwards to its bearing edge and down to the roof of the channel below it."""
    w0, w1, _ = p["p1"]
    v0, v1, _ = p["p3"]
    o1 = K.HV * K.f(u0) - 0.0005
    o3 = K.HW * K.f(u0) - 0.0005
    e1, e3 = p["edge1"], p["edge3"]
    h = K.PITCH - u0
    uc = u0 + h / 2
    return [
        box(f"ledge-a-{k}", k, [h, o1 - e1, w1 - w0], [uc, (o1 + e1) / 2, (w0 + w1) / 2], tone),
        box(f"ledge-b-{k}", k, [h, o1 - e1, w1 - w0], [uc, -(o1 + e1) / 2, (w0 + w1) / 2], tone),
        box(f"ledge-c-{k}", k, [h, v1 - v0, o3 - e3], [uc, (v0 + v1) / 2, -(o3 + e3) / 2], tone),
    ]


FLANGE = (K.FLANGE_HOLE["hv"], K.FLANGE_HOLE["hw"], K.FLANGE_HOLE["r"])


def walls(k, p, n, u0, u1, tone):
    """🧱 The drafted wall from u0 to u1 in three heights: below the hand holes, the band split by the two hand holes into its +w and -w halves, above the hand holes."""
    wall = lambda inset: K.wall_outline(p, inset, n=n)
    return [
        ext(f"wall-{k}", k, wall(0), u0, K.HAND["u0"], tone, holes=[wall(K.T)]),
        drafted_piece(f"band-a-{k}", k, K.band_piece(p, 1, n=n), K.HAND["u0"], K.HAND["u1"], tone),
        drafted_piece(f"band-b-{k}", k, K.band_piece(p, -1, n=n), K.HAND["u0"], K.HAND["u1"], tone),
        ext(f"wall-top-{k}", k, wall(0), K.HAND["u1"], u1, tone, holes=[wall(K.T)]),
    ]


def body50(k):
    """🧺 1:50: foot ring, floor, wall in three heights (the middle one split by the two hand holes), U-rim of skirt and flange, three stacking ledges, two floor ribs."""
    p, n = K.FINE, ARCS["50"]
    b, r, f = tones(k)
    wall = lambda inset: K.wall_outline(p, inset, n=n)
    sk_o, sk_i = ring(0.30, 0.20, 0.03, 0.297, 0.197, 0.027, n)
    fl_o, fl_i = ring(0.30, 0.20, 0.03, *FLANGE, n)
    return [
        ext(f"foot-{k}", k, wall(0.005), 0, 0.014, f, holes=[wall(0.009)], taper=False),
        ext(f"floor-{k}", k, wall(K.T - 0.0008), K.U0, 0.020, b),
        *walls(k, p, n, K.U0, 0.295, b),
        ext(f"skirt-{k}", k, sk_o, 0.262, K.PITCH, r, holes=sk_i, taper=False),
        ext(f"flange-{k}", k, fl_o, 0.292, K.PITCH, r, holes=fl_i, taper=False),
        *ledges(k, p, r, p["ledge_u"]),
        box(f"floor-rib-v-{k}", k, [0.006, 0.46, 0.006], [0.023, 0, 0], round(b - 0.1, 2), role="detail", texture=True),
        box(f"floor-rib-w-{k}", k, [0.006, 0.006, 0.26], [0.023, 0, 0], round(b - 0.1, 2), role="detail", texture=True),
    ]


def body_reduced(k, scale):
    """🧺 The reinforced floor and three nesting channels, with scale-specific corner faceting and rim sections."""
    p, n = K.FINE, ARCS[scale]
    b, r, f = tones(k)
    wall = lambda inset: K.wall_outline(p, inset, n=n)
    foot = ext(f"foot-{k}", k, wall(0.005), 0, 0.014, f, holes=[wall(0.009)], taper=False)
    floor = ext(f"floor-{k}", k, wall(K.T - 0.0008), K.U0, 0.020, b)
    shell = walls(k, p, n, K.U0, 0.295, b) if scale != "500" else [ext(f"wall-{k}", k, wall(0), K.U0, 0.295, b, holes=[wall(K.T)])]
    sk_o, sk_i = ring(0.30, 0.20, 0.03, 0.297, 0.197, 0.027, n)
    fl_o, fl_i = ring(0.30, 0.20, 0.03, *FLANGE, n)
    skirt_id = "skirt" if scale == "100" else "collar"
    skirt = ext(f"{skirt_id}-{k}", k, sk_o, 0.262, K.PITCH, r, holes=sk_i, taper=False)
    flange = ext(f"flange-{k}", k, fl_o, 0.292, K.PITCH, r, holes=fl_i, taper=False)
    return [foot, floor, *shell, skirt, flange, *ledges(k, p, r, p["ledge_u"])]


def th(sid, k, profile, u0, u1, taper=True):
    """🌡️ A thermal envelope solid of crate k with its horizontal faces as section faces (ruling in area-decisions.md: the region is the outer wall surface of 0.6 m2)."""
    s = ext(sid, k, profile, u0, u1, 0.5, taper=taper)
    s["sectionFaces"] = ["-u", "+u"]
    s["region"] = f"crate-{k}"
    return s


def thermal(scale, k):
    """🌡️ Envelope of the outer wall surface of crate k at the scale: wall, rim band and (1:50) foot ring."""
    n = ARCS[scale]
    p = K.FINE
    rim_o = K.outline(0.30, 0.20, 0.03, n=n)
    return [th(f"th-envelope-{k}", k, K.wall_outline(p, 0, n=n), K.U0, 0.295),
            th(f"th-rim-edge-{k}", k, rim_o, 0.262, K.PITCH, taper=False),
            th(f"th-foot-edge-{k}", k, K.wall_outline(p, 0.005, n=n), 0, 0.014, taper=False)]


def st(solid, part, k, **extra):
    """🏗️ Marks a solid of the structure list as a member, joint, support or mass of crate k."""
    solid["member"] = f"crate-{k}"
    solid["part"] = part
    solid.update(extra)
    return solid


def of(sid, ref, k, part="member"):
    """🔗 A structural reference to the body solid that is itself the member."""
    return {"id": sid, "of": ref, "member": f"crate-{k}", "part": part}


def joints(k, h, foot_v, foot_w):
    """🔩 The joint of crate k with the crate below: three contact pads under its foot where it bears on the three ledges of the lower crate, which is turned by half a turn (so the pads lie at the ledge places mirrored through the axis)."""
    if k == 1:
        return []
    p = K.FINE
    w0, w1, _ = p["p1"]
    v0, v1, _ = p["p3"]
    return [
        st(box(f"joint-a-{k}", k, [h, h, w1 - w0], [h / 2, -(foot_v - h / 2), -(w0 + w1) / 2], 0.7), "joint", k),
        st(box(f"joint-b-{k}", k, [h, h, w1 - w0], [h / 2, foot_v - h / 2, -(w0 + w1) / 2], 0.7), "joint", k),
        st(box(f"joint-c-{k}", k, [h, v1 - v0, h], [h / 2, -(v0 + v1) / 2, foot_w - h / 2], 0.7), "joint", k),
    ]


def structure(scale, k):
    """🏗️ The load path of crate k: drafted wall (a closed box section), floor plate, foot or floor, rim ring beam, three stacking ledges, three bearing pads under the foot, the support on the floor, the moulding mass."""
    n = ARCS[scale]
    b, r, _ = tones(k)
    out = []
    if scale == "50":
        p = K.FINE
        wall = lambda inset: K.wall_outline(p, inset, n=n)
        fl_o, fl_i = ring(0.30, 0.20, 0.03, *FLANGE, n)
        out += [
            st(ext(f"member-wall-{k}", k, wall(0), K.U0, 0.295, 0.5, holes=[wall(K.T)]), "member", k),
            of(f"member-floor-{k}", f"floor-{k}", k),
            of(f"member-foot-{k}", f"foot-{k}", k),
            of(f"member-rim-{k}", f"skirt-{k}", k),
            of(f"member-flange-{k}", f"flange-{k}", k),
            of(f"member-ledge-a-{k}", f"ledge-a-{k}", k), of(f"member-ledge-b-{k}", f"ledge-b-{k}", k), of(f"member-ledge-c-{k}", f"ledge-c-{k}", k),
        ]
        out += joints(k, 0.004, 0.26, 0.16)
        if k == 1:
            out.append(st(ext("support-base-contact", 1, wall(0.005), 0, 0.004, 0.1, holes=[wall(0.009)], taper=False), "support", 1, anchor="base-contact"))
        out.append(st({"id": f"mass-{k}", "kind": "ellipsoid", "bone": f"crate-{k}", "material": MAT, "role": "body", "tone": 0.5, "radii": [0.03, 0.03, 0.03], "transform": {"translation": [K.COM_U, 0, 0]}}, "mass", k))
        return out
    p = K.FINE
    wall = lambda inset: K.wall_outline(p, inset, n=n)
    if scale == "500":
        out.append(of(f"member-wall-{k}", f"wall-{k}", k))
    else:
        out.append(st(ext(f"member-wall-{k}", k, wall(0), K.U0, 0.295, 0.5, holes=[wall(K.T)]), "member", k))
    rim_id = "skirt" if scale == "100" else "collar"
    out += [of(f"member-floor-{k}", f"floor-{k}", k), of(f"member-foot-{k}", f"foot-{k}", k),
            of(f"member-rim-{k}", f"{rim_id}-{k}", k), of(f"member-flange-{k}", f"flange-{k}", k)]
    out += [of(f"member-ledge-{side}-{k}", f"ledge-{side}-{k}", k) for side in ("a", "b", "c")]
    out += joints(k, 0.004, 0.26, 0.16)
    if k == 1:
        out.append(st(ext("support-base-contact", 1, wall(0.005), 0, 0.004, 0.1, holes=[wall(0.009)], taper=False), "support", 1, anchor="base-contact"))
    radius = 0.04 if scale == "500" else 0.03
    out.append(st({"id": f"mass-{k}", "kind": "ellipsoid", "bone": f"crate-{k}", "material": MAT, "role": "body", "tone": 0.5, "radii": [radius] * 3, "transform": {"translation": [K.COM_U, 0, 0]}}, "mass", k))
    return out


DECISIONS = {
    "50": [
        text("Body: five rotating stack-nest crates, each hand-built as the moulding is: a 14 mm foot ring (4 mm wall, 0.52 x 0.32 m), an 8 mm floor, the 3 mm wall drafted from 0.53 x 0.33 m at the floor to 0.59 x 0.37 m at the flange (uniform taper 1.10), split at the hand-hole band into two pieces that leave true openings of 120 x 35 mm in both end walls, the outward U-rim of a 38 mm skirt and an 8 mm flange (0.60 x 0.40 m, corner radius 30 mm), three stacking ledges at the top inside (two in the end walls, one in the far long wall) and, under each ledge, a channel stepped into the wall from the foot up to the ledge (40 mm deep in the ends, 32 mm in the side). Turned by half a turn the next crate stands on the ledges; in the same orientation the ledges pass through its channels and it nests with 0.6 to 4 mm play. The floor ribs are texture (6 mm, below the feature).",
             "Körper: fünf Drehstapelbehälter, jeder so gebaut wie das Spritzgussteil: ein 14 mm hoher Fußring (4 mm Wand, 0,52 x 0,32 m), ein 8 mm Boden, die 3 mm Wand mit Formschräge von 0,53 x 0,33 m am Boden auf 0,59 x 0,37 m am Flansch (gleichmäßige Konizität 1,10), im Griffband in zwei Teile geteilt, die in beiden Stirnwänden echte Öffnungen von 120 x 35 mm lassen, der nach außen offene U-Rand aus 38 mm Schürze und 8 mm Flansch (0,60 x 0,40 m, Eckradius 30 mm), drei Stapelleisten oben innen (zwei in den Stirnwänden, eine in der hinteren Längswand) und unter jeder Leiste eine Nut, die vom Fuß bis zur Leiste in die Wand gestuft ist (40 mm tief in den Stirnseiten, 32 mm in der Längsseite). Um eine halbe Drehung gedreht steht die nächste Kiste auf den Leisten; in gleicher Ausrichtung gleiten die Leisten durch ihre Nuten und sie stellt sich mit 0,6 bis 4 mm Spiel hinein. Die Bodenrippen sind Textur (6 mm, unter der Mindestbreite)."),
        text("Thermal: the declared 0.6 m2 per crate is the outer wall area (2 x (0.6 + 0.4) x 0.3 m), so each region is the outer envelope of its crate (drafted wall with its channels, rim band, foot ring) with the horizontal faces as section faces, as the area reviewer ruled (area-decisions.md, sectionFaces table); the inside of the open crate and its floor are not counted. The honest outer area of the open moulding (both faces of wall and floor, C29/C31) is about 1.6 m2 per crate and is listed for the area reviewer in area-review.md, not applied (factor above 1.5).",
             "Thermik: die angegebenen 0,6 m2 je Kiste sind die äußere Wandfläche (2 x (0,6 + 0,4) x 0,3 m), daher ist jede Region die Außenhülle ihrer Kiste (Wand mit Formschräge und Nuten, Randband, Fußring) mit den waagrechten Flächen als Schnittflächen, wie die Flächenprüfung entschieden hat (area-decisions.md, Tabelle sectionFaces); Innenseite und Boden der offenen Kiste zählen nicht. Die ehrliche Außenfläche des offenen Formteils (beide Seiten von Wand und Boden, C29/C31) beträgt etwa 1,6 m2 je Kiste und ist für die Flächenprüfung in area-review.md eingetragen, nicht angewendet (Faktor über 1,5)."),
        text("Structure: each crate is a monocoque: the drafted wall is a closed box section (member), the floor plate carries the goods to the wall, the foot ring carries the crate, the rim is a ring beam, the three stacking ledges carry the crate above; the joint of each upper crate is three 4 mm bearing pads under its foot where it stands on the ledges of the crate below (turned by half a turn); the support is the foot ring of the lowest crate on the floor; one 2.15 kg mass at the moulding's centre 0.13 m above the foot.",
             "Tragwerk: jede Kiste ist eine Schale: die Wand mit Formschräge ist ein geschlossener Kastenquerschnitt (Stab), die Bodenplatte gibt die Ware an die Wand, der Fußring trägt die Kiste, der Rand ist ein Ringbalken, die drei Stapelleisten tragen die obere Kiste; das Gelenk jeder oberen Kiste sind drei 4-mm-Auflagerflächen unter ihrem Fuß, wo sie auf den Leisten der um eine halbe Drehung gedrehten unteren Kiste steht; das Auflager ist der Fußring der untersten Kiste auf dem Boden; eine Masse von 2,15 kg im Schwerpunkt des Formteils 0,13 m über dem Fuß."),
    ],
    **{d: [
        text("Each tub has an 8 mm floor on a 14 mm foot ring, a 3 mm drafted wall, a 38 mm skirt and an 8 mm top flange. Three 68 mm tall bearing ledges clear the three moulded channels in the nested position. " + detail,
             "Jede Kiste hat einen 8-mm-Boden auf einem 14-mm-Fußring, eine 3 mm starke Wand mit Formschräge, eine 38-mm-Schürze und einen 8-mm-Flansch. Drei 68 mm hohe Auflager passen beim Einstellen in die drei eingeformten Nuten. " + de),
        text("The exterior-wall exchange area follows the tapered shell, moulded lip and foot.", "Die Wärmeübertragung der Außenwand folgt der konischen Wand, dem Rand und dem Fuß."),
        text("Each upper foot bears on three separate seats. The bottom ring transmits their combined loads into the supporting floor.", "Jeder obere Fuß steht auf drei einzelnen Auflagern. Der unterste Ring leitet die Gesamtlast in den Boden ein.")
    ] for d, detail, de in [
        ("100", "The wall retains the two 120 × 35 mm hand openings and four facets per rounded corner.", "Die Wand enthält beide Grifföffnungen von 120 × 35 mm. Vier Facetten bilden jede gerundete Ecke."),
        ("200", "The three-facet corners and hand openings remain visible.", "Die Ecken haben drei Facetten. Beide Grifföffnungen bleiben sichtbar."),
        ("500", "Two facets describe each corner. The end walls read as continuous faces at this scale.", "Zwei Facetten beschreiben jede Ecke. Die Stirnwände erscheinen in diesem Maßstab als geschlossene Flächen.")
    ]}
}


def volume(scale):
    """📐 One authored scale: body, thermal, structure and decisions of the five crates."""
    body = []
    for k in range(1, N + 1):
        body += body50(k) if scale == "50" else body_reduced(k, scale)
    return {"status": "authored", "denominator": int(scale), "body": body,
            "thermal": [s for k in range(1, N + 1) for s in thermal(scale, k)],
            "structure": [s for k in range(1, N + 1) for s in structure(scale, k)],
            "decisions": DECISIONS[scale]}


def main():
    """🚀 Writes the volumes source of the asset."""
    doc = {"volumes": {s: volume(s) for s in ("50", "100", "200", "500")}}
    with open(OUT, "w") as fh:
        json.dump(doc, fh, ensure_ascii=False, indent=1)
        fh.write("\n")
    print(OUT, {s: (len(v["body"]), len(v["thermal"]), len(v["structure"])) for s, v in doc["volumes"].items()})


if __name__ == "__main__":
    main()
