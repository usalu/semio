"""🛁️ Authoritative chisel of the asset `used-washbasin-vanity`: rig, anchors, materials, thermal regions, structure, poses, clips, drawings, volumes and texts of the used 60 cm washbasin with vanity unit and mirror cabinet on its depot pallet, and the writer of `🔣️.json` next to this file.

Run: `python3 author-used-washbasin-vanity.py` (writes `🔣️.json`, prints the masses and centres it derived).

@see ./author_geometry.py — every hand-chosen dimension and the part lists per scale
@see ./author_drawing.py — the 2D shapes
@see ../../brief.md — the brief of the wave
"""
import json
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from author_geometry import (BOWL_X, HANDLE_Y, LEVER_LEN, ORIGIN, PT, TAP_X, XB0, XB1, XF1, XH1, XL1, XM0, XS0, XS1, XV0, XV1, YB0, YD1, YL1, YM1, YV1, ZBI, ZBO, ZM, ZS, ZV, ZVI, loc, masses, parts_of, r6, solid_of)
from author_drawing import auto_shapes, details, ellipse, poly, rect, sh

G = 9.80665


def T(en, de):
    """🗣️ One bilingual text, English first."""
    return {"en": en, "de": de}


PARTS = {s: parts_of(s) for s in (50, 100, 200)}
MASS = masses(PARTS[50])
TOTAL_KG = sum(m["kg"] for m in MASS.values())
W = {b: MASS[b]["kg"] * G for b in MASS}


def local_com(b):
    """🎯️ The centre of mass of a bone in its rest frame."""
    ox, oy = ORIGIN[b]
    return (MASS[b]["x"] - ox, MASS[b]["y"] - oy, MASS[b]["z"])


def ref(id, of, **kw):
    """🔗️ A thermal or structural reference to a body solid of the same scale."""
    s = {"id": id, "of": of}
    s.update(kw)
    return s


def jbox(id, bone, x0, x1, y0, y1, z0, z1, material, rep=None):
    """🔩️ A joint solid (fitting, runner, bolt) from world rest bounds."""
    ox, oy = ORIGIN[bone]
    s = {"id": id, "kind": "box", "bone": bone, "material": material, "role": "detail", "tone": 0.3, "size": [r6(x1 - x0), r6(y1 - y0), r6(z1 - z0)], "transform": {"translation": [r6((x0 + x1) / 2 - ox), r6((y0 + y1) / 2 - oy), r6((z0 + z1) / 2)]}, "member": bone, "part": "joint"}
    if rep:
        s["repeat"] = {"count": rep[0], "translation": [r6(c) for c in rep[1]]}
    return s


MASS_MATERIAL = {"pallet": "pallet-spruce", "mirror": "melamine-chipboard", "vanity": "melamine-chipboard", "drawer": "mdf-gloss", "basin": "vitreous-china", "lever": "chrome-brass"}


def mass_markers(sfx, r):
    """⚫️ One mass marker per bone at its centre of mass."""
    out = []
    for b in ("pallet", "mirror", "vanity", "drawer", "basin", "lever"):
        u, v, w = local_com(b)
        rr = {0.012: 0.005, 0.016: 0.0075, 0.03: 0.015}[r] if b == "lever" else r
        out.append({"id": f"mass-{b}{sfx}", "kind": "ellipsoid", "bone": b, "material": MASS_MATERIAL[b], "role": "detail", "tone": 0.3, "radii": [rr, rr, rr], "transform": {"translation": [r6(u), r6(v), r6(w)]}, "member": b, "part": "mass"})
    return out


def joints(scale):
    """🔧️ The fittings that join the members: drawer runners, vanity hangers, basin wall bolts, tap fixing, lever cartridge and mirror hangers."""
    sfx = "" if scale == 50 else f"-{scale}"
    t = {50: 0.008, 100: 0.015, 200: 0.03}[scale]
    out = [
        jbox("j-vanity-hangers" + sfx, "vanity", XV0 + 0.003, XV0 + 0.003 + t, YV1 - 0.06, YV1 - 0.016, ZVI - 0.03, ZVI, "zinc-steel", (2, (0, 0, -(2 * ZVI - 0.03)))),
        jbox("j-mirror-hangers" + sfx, "mirror", XM0 + 0.003, XM0 + 0.003 + t, YM1 - 0.07, YM1 - 0.016, 0.284 - 0.03, 0.284, "zinc-steel", (2, (0, 0, -(2 * 0.284 - 0.03)))),
        jbox("j-basin-bolts" + sfx, "basin", XS0, XS0 + max(t, 0.015), PT + 0.51, PT + 0.51 + max(t, 0.02), 0.19, 0.19 + max(t, 0.02), "zinc-steel", (2, (0, 0, -(0.38 + max(t, 0.02))))),
        jbox("j-tap-fixing" + sfx, "basin", TAP_X - max(t, 0.012) / 2, TAP_X + max(t, 0.012) / 2, YD1 - 0.012 - max(t, 0.03), YD1 - 0.012, -max(t, 0.012) / 2, max(t, 0.012) / 2, "chrome-brass"),
    ]
    if scale < 200:
        out.append(jbox("j-runners" + sfx, "drawer", XB0 + 0.02, XB1 - 0.02, YB0, YB0 + max(t, 0.012), ZBO, 0.274, "zinc-steel", (2, (0, 0, -(ZBO + 0.274)))))
    else:
        out.append(jbox("j-front-fixing-200", "drawer", XV1 - 0.011, XV1 + 0.019, PT + 0.03, PT + 0.06, -0.2, 0.2, "zinc-steel"))
    c = max(t, 0.01)
    lx, ly = ORIGIN["lever"]
    out.append(jbox("j-lever-cartridge" + sfx, "lever", lx + 0.005, lx + 0.005 + c, ly - c + 0.002, ly + 0.002, -c / 2, c / 2, "chrome-brass"))
    return out



def body(scale):
    """🧱️ The body list of a scale."""
    return [solid_of(p) for p in PARTS[scale]]


def thermal(scale):
    """🌡️ Every body solid in its region, plus the hidden water stream of the running tap."""
    out = [ref("th-" + p["id"], p["id"], region=p["region"]) for p in PARTS[scale]]
    ox, oy = ORIGIN["basin"]
    r = {50: 0.004, 100: 0.0075, 200: 0.015}[scale]
    mx, my = ORIGIN["mirror"]
    q = {50: 0.004, 100: 0.0075, 200: 0.015}[scale]
    out.append({"id": f"th-light-plume-{scale}", "kind": "ellipsoid", "bone": "mirror", "material": "opal-acrylic", "role": "detail", "tone": 0.4, "radii": [max(0.012, q), max(0.012, q), 0.2], "transform": {"translation": [r6(XH1 + 0.02 - mx), r6(YM1 + 0.02 - my), 0.0]}, "region": "light-bar", "tags": ["light-plume"]})
    out.append({"id": f"th-basin-mist-{scale}", "kind": "ellipsoid", "bone": "basin", "material": "water", "role": "detail", "tone": 0.4, "radii": [0.03, max(q, 0.01), 0.03], "transform": {"translation": [r6(0.06 - ox), r6(PT + 0.47 - oy), 0.0]}, "region": "water", "tags": ["basin-mist"]})
    out.append({"id": f"th-water-stream-{scale}", "kind": "capsule", "bone": "basin", "material": "water", "role": "detail", "tone": 0.5, "p0": [r6(0.034 - ox), r6(PT + 0.611 - r - oy), 0.0], "p1": [r6(0.05 - ox), r6(PT + 0.43 - oy), 0.0], "r0": r, "r1": r, "region": "water"})
    return out


def structure(scale):
    """🏗️ Members referencing the body, the fittings as joints and one mass marker per bone."""
    out = [ref("m-" + p["id"], p["id"], member=p["bone"], part="member") for p in PARTS[scale]]
    out += joints(scale)
    out += mass_markers("" if scale == 50 else f"-{scale}", {50: 0.012, 100: 0.016, 200: 0.03}[scale])
    return out


HATCHES_50 = [
    {"id": "grain", "title": T("Spruce grain along the pallet boards", "Fichtenmaserung längs der Palettenbretter"), "pattern": "lines", "angleDeg": 0, "spacingMm": 0.8, "penMm": 0.13},
    {"id": "mirror", "title": T("Mirror glass reflection hatch", "Spiegelglas-Reflexschraffur"), "pattern": "lines", "angleDeg": 45, "spacingMm": 1.6, "penMm": 0.13},
]


def layered(body, marks):
    """🧅️ Body shapes with each mark painted just above the nearest body shape of its region and bone, so nearer parts still cover it."""
    out = list(body)
    for k, m in enumerate(marks):
        own = [b["z"] for b in body if b.get("region") == m.get("region") and b["bone"] == m["bone"]]
        out.append(dict(m, z=round(max(own) + 0.01 + 0.001 * k, 3)))
    return out


def rep(scale):
    """🖋️ The authored drawing of a scale: automatic body shapes from the part list plus the hand-placed traces of use."""
    hat = scale == 50
    el, fr, pl = (auto_shapes(PARTS[scale], v, hat) for v in ("elevation", "front", "plan"))
    de, dfr, dpl = details(scale)
    geom = {"elevation": layered(el, de), "front": layered(fr, dfr), "plan": layered(pl, dpl)}
    pens = {50: {"body": 0.5, "detail": 0.25, "texture": 0.13, "glazing": None, "ground": 0.7}, 100: {"body": 0.35, "detail": 0.18, "texture": 0.13, "glazing": None, "ground": 0.5}, 200: {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}}[scale]
    band = {50: (20, 75), 100: (75, 150), 200: (150, 350)}[scale]
    return {"status": "authored", "denominator": scale, "validRange": {"minDenominator": band[0], "maxDenominator": band[1]}, "pens": pens, "hatches": HATCHES_50 if hat else [], "geometry": geom, "decisions": DECISIONS_2D[scale]}


DECISIONS_2D = {
    50: {
        "silhouette": T("The stored set at 24 x 18 mm: the EUR pallet with every deck board, cross board, block and bottom board, the mirror cabinet standing on it with sides, light housing and quarter-round diffuser, the 8 mm mirror doors, and in front of it the vanity unit with its 19 mm drawer front, the bar handle on two posts, and the basin with its 70 mm ceramic edge and the single-lever tap with spout and lever. Front view: drawer front, basin edge, tap, the mirror doors with their joint and the light bar above the basin. Plan: basin deck with the bowl opening, the pop-up waste and the tap, the light bar and the pallet boards.", "Das gelagerte Set mit 24 x 18 mm: die EUR-Palette mit jedem Deckbrett, Querbrett, Klotz und Bodenbrett, der darauf stehende Spiegelschrank mit Seiten, Leuchtengehäuse und viertelrundem Diffusor, die 8-mm-Spiegeltüren und davor der Unterschrank mit 19-mm-Schubladenfront, der Bügelgriff auf zwei Stützen und das Becken mit seiner 70-mm-Keramikkante und der Einhebelarmatur mit Auslauf und Hebel. Vorderansicht: Schubladenfront, Beckenkante, Armatur, die Spiegeltüren mit ihrer Fuge und die Leuchte über dem Becken. Grundriss: Beckendeck mit Beckenöffnung, Ablaufventil und Armatur, die Leuchte und die Palettenbretter."),
        "retained": T("The traces of use: limescale crust around the tap base and the waste, a soap mark on the deck, the chip in the front edge of the basin, the swollen lower edges of the vanity side and the drawer front, the chipped lacquer at the lower drawer corner, scratches, the two filled holes of the original 128 mm handle beside the replacement bar, the desilvered lower door corners, limescale splashes and a child's sticker on the mirror, a sticker on the cabinet side and the mains flex taped to the back.", "Die Gebrauchsspuren: Kalkkruste um den Armaturenfuß und den Ablauf, ein Seifenrand auf dem Deck, die Abplatzung an der Vorderkante des Beckens, die aufgequollenen Unterkanten von Unterschrankseite und Schubladenfront, der abgeplatzte Lack an der unteren Schubladenecke, Kratzer, die zwei Löcher des ursprünglichen 128-mm-Griffs neben dem Ersatzbügel, die erblindeten unteren Türecken, Kalkspritzer und ein Kinderaufkleber auf dem Spiegel, ein Aufkleber an der Schrankseite und die an die Rückwand geklebte Netzleitung."),
        "merged": T("Left and right sides, doors and pallet rows coincide in the side view; the bowl, the drawer box and the glass shelves lie inside and are drawn under the outer parts.", "Linke und rechte Seiten, Türen und Palettenreihen fallen in der Seitenansicht zusammen; Becken, Schubladenkasten und Glasböden liegen innen und sind unter den äußeren Teilen gezeichnet."),
        "omitted": T("Hinges, shelf pins, screws, the overflow slot (6 mm), the flexible tap hoses inside the vanity and the nails of the pallet (all under 7.5 mm).", "Scharniere, Bodenträger, Schrauben, der Überlaufschlitz (6 mm), die flexiblen Armaturenschläuche im Unterschrank und die Nägel der Palette (alle unter 7,5 mm)."),
        "thinElements": T("The 3 mm hardboard backs and the 6 mm glass shelves keep their gauge as sheets; mirror doors 8 mm, drawer bottom 8 mm, bowl wall 12 mm.", "Die 3-mm-Hartfaserrückwände und die 6-mm-Glasböden behalten ihre Stärke als Platten; Spiegeltüren 8 mm, Schubladenboden 8 mm, Beckenwand 12 mm."),
        "hatching": T("Spruce grain 0.8 mm on the pallet bottom and deck boards, a 45 degree reflection hatch of 1.6 mm on the mirror doors, all with the 0.13 mm pen.", "Fichtenmaserung 0,8 mm auf den Boden- und Deckbrettern der Palette, eine 45-Grad-Reflexschraffur von 1,6 mm auf den Spiegeltüren, alle mit dem 0,13-mm-Stift."),
        "styles": T("Basin deck 0.92, drawer front 0.9, opal diffuser 0.9 (yellowed), cabinet sides 0.82, vanity sides 0.8, mirror doors 0.68, tap 0.55, pallet deck 0.55, blocks 0.4, backs 0.35, pop-up waste 0.5.", "Beckendeck 0,92, Schubladenfront 0,9, Opaldiffusor 0,9 (vergilbt), Schrankseiten 0,82, Unterschrankseiten 0,8, Spiegeltüren 0,68, Armatur 0,55, Palettendeck 0,55, Klötze 0,4, Rückwände 0,35, Ablaufventil 0,5."),
        "overlays": T("One shape per part and copy, each naming its region; the traces of use are strokes on the region they mark.", "Eine Form je Teil und Kopie, jede mit ihrer Region; die Gebrauchsspuren sind Striche auf der Region, die sie zeichnen."),
    },
    100: {
        "silhouette": T("The stored set at 12 x 9 mm: pallet boards and blocks, mirror cabinet as carcass frame with the doors as one sheet and the light bar as one profile, vanity carcass as one U profile, drawer front with a block handle, basin edge, bowl and tap.", "Das gelagerte Set mit 12 x 9 mm: Palettenbretter und -klötze, Spiegelschrank als Korpusrahmen mit den Türen als eine Platte und der Leuchte als ein Profil, Unterschrankkorpus als ein U-Profil, Schubladenfront mit Blockgriff, Beckenkante, Becken und Armatur."),
        "retained": T("The door joint, the drawer reveal, the bowl opening and the pop-up waste in plan, the chipped lower drawer corner.", "Die Türfuge, die Schubladenfuge, Beckenöffnung und Ablaufventil im Grundriss, die abgeplatzte untere Schubladenecke."),
        "merged": T("Mirror sides, top and bottom in one frame; both doors in one sheet; housing and diffuser in one profile; vanity sides and bottom in one U; handle bar and posts in one block.", "Spiegelschrankseiten, -boden und -deckel in einem Rahmen; beide Türen in einer Platte; Gehäuse und Diffusor in einem Profil; Unterschrankseiten und -boden in einem U; Griffbügel und Stützen in einem Block."),
        "omitted": T("Glass shelves, grain and reflection hatches, scratches, stickers, limescale and the filled handle holes (all under 0.15 mm).", "Glasböden, Maserungs- und Reflexschraffur, Kratzer, Aufkleber, Kalk und die alten Grifflöcher (alle unter 0,15 mm)."),
        "thinElements": T("Hardboard backs 3 mm and the drawer bottom as sheets; mirror doors 8 mm as one glass sheet; boards from 15 mm.", "Hartfaserrückwände 3 mm und der Schubladenboden als Platten; Spiegeltüren 8 mm als eine Glasplatte; Platten ab 15 mm."),
        "hatching": T("None.", "Keine."),
        "styles": T("As 1:50: basin 0.92, drawer 0.9, diffuser 0.85, carcasses 0.8 to 0.82, doors 0.68, tap 0.55, pallet 0.4 to 0.55.", "Wie 1:50: Becken 0,92, Schublade 0,9, Diffusor 0,85, Korpusse 0,8 bis 0,82, Türen 0,68, Armatur 0,55, Palette 0,4 bis 0,55."),
        "overlays": T("One shape per part and copy with its region.", "Eine Form je Teil und Kopie mit ihrer Region."),
    },
    200: {
        "silhouette": T("The stored set at 6 x 4.5 mm: pallet deck plate on nine feet, mirror cabinet block with its door sheet and light bar, vanity U carcass, drawer front, basin edge, bowl and a tap of the 30 mm feature.", "Das gelagerte Set mit 6 x 4,5 mm: Palettendeckplatte auf neun Füßen, Spiegelschrankblock mit Türplatte und Leuchte, U-Korpus des Unterschranks, Schubladenfront, Beckenkante, Becken und eine Armatur im 30-mm-Merkmal."),
        "retained": T("The fork openings of the pallet, the light bar over the cabinet, the drawer front proud of the carcass, the bowl opening in plan.", "Die Gabelöffnungen der Palette, die Leuchte über dem Schrank, die vor dem Korpus stehende Schubladenfront, die Beckenöffnung im Grundriss."),
        "merged": T("Pallet boards into one deck plate and nine feet; mirror carcass into one frame of 30 mm; housing and diffuser into one bar; drawer box left inside the closed carcass; handle, spout and lever thickened to the feature or merged.", "Palettenbretter zu einer Deckplatte und neun Füßen; Spiegelschrankkorpus zu einem Rahmen von 30 mm; Gehäuse und Diffusor zu einer Leiste; Schubladenkasten im geschlossenen Korpus weggelassen; Griff, Auslauf und Hebel auf das Merkmal verdickt oder zusammengefasst."),
        "omitted": T("Handle, door joint, reveal, pop-up waste and every trace of use (under 0.15 mm at 1:200).", "Griff, Türfuge, Fuge, Ablaufventil und alle Gebrauchsspuren (unter 0,15 mm bei 1:200)."),
        "thinElements": T("Mirror doors and the 3 mm back remain sheets; everything else from 30 mm.", "Spiegeltüren und die 3-mm-Rückwand bleiben Platten; alles andere ab 30 mm."),
        "hatching": T("None.", "Keine."),
        "styles": T("Basin 0.92, drawer 0.9, carcasses 0.8, doors 0.68, tap 0.55, pallet 0.4 to 0.52.", "Becken 0,92, Schublade 0,9, Korpusse 0,8, Türen 0,68, Armatur 0,55, Palette 0,4 bis 0,52."),
        "overlays": T("One shape per part and copy with its region.", "Eine Form je Teil und Kopie mit ihrer Region."),
    },
}


def rep500():
    """🚫️ 1:500 is not drawn: the stored set is under 3 mm of paper."""
    return {"status": "unavailable", "denominator": 500, "cause": "below-printable-feature",
            "reason": T("The stored set is 1.2 x 0.8 x 0.9 m = 2.4 x 1.6 x 1.8 mm on paper at 1:500, under 3 mm: the basin edge (70 mm = 0.14 mm), the drawer front (19 mm), the light bar (60 mm = 0.12 mm) and the pallet boards (22 mm) would merge into one anonymous block.", "Das gelagerte Set misst 1,2 x 0,8 x 0,9 m = 2,4 x 1,6 x 1,8 mm auf dem Papier bei 1:500, unter 3 mm: Beckenkante (70 mm = 0,14 mm), Schubladenfront (19 mm), Leuchte (60 mm = 0,12 mm) und Palettenbretter (22 mm) würden zu einem namenlosen Block verschmelzen."),
            "limitingFeature": T("Basin edge and light bar, 60-70 mm (0.12-0.14 mm on paper)", "Beckenkante und Leuchte, 60-70 mm (0,12-0,14 mm auf dem Papier)"),
            "limitingSizeM": 0.06, "alternatives": [{"kind": "scale", "denominator": 200}]}


def vol(den, decisions):
    """🧊️ One authored scale volume."""
    return {"status": "authored", "denominator": den, "body": body(den), "thermal": thermal(den), "structure": structure(den), "decisions": decisions}


VOLUMES = {
    "50": vol(50, [
        T("Body for the 7.5 mm feature: the EUR pallet board by board (three bottom boards, nine blocks, three cross boards, five deck boards); the mirror cabinet with 16 mm melamine sides, top and bottom, 3 mm back (sheet), two 6 mm glass shelves (sheets), two 8 mm mirror doors with their 1.5 mm joint, the light bar as one 2.5 mm hollow profile around a single lamp chamber (aluminium housing with the opal diffuser clipped to it, modelled in the opal material) closed by 8 mm ABS end caps; the vanity unit with 16 mm sides and bottom, a vertical front rail and a back rail, the 3 mm back with its pipe cut-out; the drawer with 19 mm MDF front, 12 mm box sides and back, 8 mm bottom and the bar handle on two posts; the basin as a 12 mm deck with the bowl opening, a 15 mm apron ring and the bowl as a turned shell of 12 mm closed at the bottom; the mixer tap as a turned body, a capsule spout and the lever on its own bone.", "Körper für das 7,5-mm-Merkmal: die EUR-Palette Brett für Brett (drei Bodenbretter, neun Klötze, drei Querbretter, fünf Deckbretter); der Spiegelschrank mit 16-mm-Melaminseiten, -deckel und -boden, 3-mm-Rückwand (Platte), zwei 6-mm-Glasböden (Platten), zwei 8-mm-Spiegeltüren mit 1,5-mm-Fuge, der Leuchte als ein 2,5-mm-Hohlprofil um eine einzige Lampenkammer (Aluminiumgehäuse mit aufgeklipstem Opaldiffusor, im Opalmaterial modelliert), geschlossen von 8-mm-ABS-Endkappen; der Unterschrank mit 16-mm-Seiten und -Boden, senkrechter Frontblende und Rückleiste, der 3-mm-Rückwand mit Rohrausschnitt; die Schublade mit 19-mm-MDF-Front, 12-mm-Kastenseiten und -rückwand, 8-mm-Boden und dem Bügelgriff auf zwei Stützen; das Becken als 12-mm-Deck mit Beckenöffnung, 15-mm-Schürze und das Becken als gedrehte 12-mm-Schale mit geschlossenem Boden; die Mischbatterie als gedrehter Körper, Kapselauslauf und Hebel auf eigenem Knochen."),
        T("Traces of use are texture and detail strokes of the drawings (limescale, chips, swelling, scratches, stickers, desilvering, the old handle holes): all are under 7.5 mm deep and do not change the solids.", "Gebrauchsspuren sind Textur- und Detailstriche der Zeichnungen (Kalk, Abplatzungen, Quellungen, Kratzer, Aufkleber, Erblindung, alte Grifflöcher): alle sind weniger als 7,5 mm tief und ändern die Körper nicht."),
        T("Thermal: every body solid in its region; the hidden water region is a thin stream from the aerator into the bowl, used only when the tap runs. Structure: every board, shell and fitting as a member of its bone, hangers, wall bolts, runners, the tap fixing and the lever cartridge as joints, one mass marker per bone; no permanent support, the pose contacts carry it (pallet on the floor, hands, wall hangers).", "Thermik: jeder Körper in seiner Region; die verdeckte Region Wasser ist ein dünner Strahl vom Strahlregler ins Becken, nur bei laufendem Hahn genutzt. Tragwerk: jede Platte, Schale und jedes Beschlagteil als Stab seines Knochens, Aufhänger, Wandschrauben, Auszüge, Armaturenbefestigung und Hebelkartusche als Gelenke, eine Massenmarke je Knochen; kein dauerndes Auflager, die Kontakte der Posen tragen (Palette auf dem Boden, Hände, Wandaufhänger)."),
    ]),
    "100": vol(100, [
        T("Body for the 15 mm feature: pallet boards and blocks as at 1:50 (22 mm and more); mirror carcass as one hollow front frame of 16 mm with the back sheet, both doors as one 8 mm glass sheet, housing and diffuser as one profile; vanity sides and bottom as one U profile with the back sheet and the two rails; drawer front, box sides and back thickened from 12 to 15 mm, the 8 mm bottom as a sheet, the handle as one 15 mm block; basin deck 15 mm, apron 15 mm, bowl shell 15 mm; tap body and spout as at 1:50, the lever 15 mm thick.", "Körper für das 15-mm-Merkmal: Palettenbretter und -klötze wie bei 1:50 (22 mm und mehr); Spiegelschrankkorpus als ein hohler Frontrahmen von 16 mm mit Rückwandplatte, beide Türen als eine 8-mm-Glasplatte, Gehäuse und Diffusor als ein Profil; Unterschrankseiten und -boden als ein U-Profil mit Rückwandplatte und den zwei Blenden; Schubladenfront, Kastenseiten und -rückwand von 12 auf 15 mm verdickt, der 8-mm-Boden als Platte, der Griff als ein 15-mm-Block; Beckendeck 15 mm, Schürze 15 mm, Beckenschale 15 mm; Armaturenkörper und Auslauf wie bei 1:50, der Hebel 15 mm dick."),
        T("Merged against 1:50: the glass shelves (inside the closed cabinet, their loads stay on the cabinet bone), the door joint, the handle posts.", "Gegenüber 1:50 zusammengefasst: die Glasböden (im geschlossenen Schrank, ihre Lasten bleiben am Schrankknochen), die Türfuge, die Griffstützen."),
        T("Thermal and structure reference the body; fittings as 15 mm joints, masses 32 mm.", "Thermik und Tragwerk verweisen auf den Körper; Beschläge als 15-mm-Gelenke, Massen 32 mm."),
    ]),
    "200": vol(200, [
        T("Body for the 30 mm feature: pallet as a 44 mm deck plate on nine 100 mm feet (bottom boards merged into the feet); mirror carcass as one 30 mm frame with the door sheet in front, the light bar as one 60 mm block; vanity as a 30 mm U carcass with its back sheet; drawer front 30 mm, box left out inside the closed carcass; basin deck, apron and bowl at 30 mm; tap body, a 30 mm spout and a 30 mm lever.", "Körper für das 30-mm-Merkmal: Palette als 44-mm-Deckplatte auf neun 100-mm-Füßen (Bodenbretter in die Füße gezogen); Spiegelschrankkorpus als ein 30-mm-Rahmen mit der Türplatte davor, die Leuchte als ein 60-mm-Block; Unterschrank als 30-mm-U-Korpus mit Rückwandplatte; Schubladenfront 30 mm, Kasten im geschlossenen Korpus weggelassen; Beckendeck, Schürze und Becken mit 30 mm; Armaturenkörper, ein 30-mm-Auslauf und ein 30-mm-Hebel."),
        T("Thermal: the drawer front is the drawer region; structure: the same parts as members, fittings 30 mm, masses 60 mm.", "Thermik: die Schubladenfront ist die Region Schublade; Tragwerk: dieselben Teile als Stäbe, Beschläge 30 mm, Massen 60 mm."),
    ]),
    "500": {"status": "unavailable", "denominator": 500},
}


def mat(id, en, de, eps, alpha, rho, k, c, E=None, f=None, source="literature", reference="iso-10456-2007"):
    """🧱️ One material."""
    m = {"id": id, "title": T(en, de), "emissivity": eps, "solarAbsorptivity": alpha, "densityKgM3": rho, "conductivityWmK": k, "specificHeatJKgK": c}
    if E is not None:
        m["elasticModulusPa"] = E
    if f is not None:
        m["strengthPa"] = f
    m.update({"source": source, "reference": reference})
    return m


MATERIALS = [
    mat("vitreous-china", "Vitreous china with white glaze, dulled by limescale", "Sanitärporzellan mit weißer Glasur, von Kalk stumpf", 0.92, 0.3, 2350, 1.3, 840, 70e9, 40e6, "literature", "en-14688-2015"),
    mat("chrome-brass", "Chrome-plated brass, dulled and spotted with limescale", "Verchromtes Messing, stumpf und kalkfleckig", 0.15, 0.45, 8500, 110, 380, 100e9, 250e6, "authored-estimate", "incropera-2018"),
    mat("melamine-chipboard", "Chipboard P2 with white melamine facing, yellowed", "Spanplatte P2 mit weißer Melaminbeschichtung, vergilbt", 0.9, 0.35, 650, 0.13, 1700, 1.6e9, 11e6, "literature", "en-312-2010"),
    mat("mdf-gloss", "MDF with high-gloss white lacquer, scratched", "MDF mit weißem Hochglanzlack, zerkratzt", 0.88, 0.3, 750, 0.14, 1700, 2.2e9, 20e6, "literature", "en-622-5-2009"),
    mat("hardboard", "Hardboard HB 3 mm, white-coated face", "Hartfaserplatte HB 3 mm, weiß beschichtet", 0.9, 0.4, 950, 0.18, 1700, 4.0e9, 30e6, "literature", "en-622-2-2004"),
    mat("aluminium-anodised", "Anodised aluminium, dulled", "Eloxiertes Aluminium, stumpf", 0.75, 0.45, 2700, 160, 900, 70e9, 160e6, "literature", "incropera-2018"),
    mat("mirror-glass", "Silvered float glass mirror 4 + 4 mm, desilvered at the edges", "Silberspiegel aus Floatglas 4 + 4 mm, an den Kanten erblindet", 0.88, 0.12, 2500, 1.0, 750, 70e9, 45e6, "literature", "en-1036-1-2007"),
    mat("float-glass", "Float glass shelf 6 mm", "Floatglas-Boden 6 mm", 0.9, 0.1, 2500, 1.0, 750, 70e9, 45e6, "literature", "en-572-1-2012"),
    mat("opal-acrylic", "Opal acrylic diffuser, yellowed", "Opaler Acryldiffusor, vergilbt", 0.9, 0.45, 1190, 0.19, 1470, 3.2e9, 70e6, "authored-estimate", "incropera-2018"),
    mat("abs-endcap", "ABS end caps of the light bar, grey", "ABS-Endkappen der Leuchte, grau", 0.9, 0.5, 1050, 0.17, 1400, 2.3e9, 40e6, "authored-estimate", "incropera-2018"),
    mat("pallet-spruce", "Spruce pallet timber, weathered grey", "Fichtenholz der Palette, grau verwittert", 0.9, 0.6, 470, 0.13, 1600, 11e9, 24e6, "literature", "en-338-2016"),
    mat("zinc-steel", "Zinc-plated steel fittings", "Verzinkte Stahlbeschläge", 0.3, 0.5, 7850, 50, 470, 210e9, 235e6, "literature", "incropera-2018"),
    mat("water", "Tap water at 38 degC", "Leitungswasser von 38 degC", 0.96, 0.05, 993, 0.63, 4180, None, None, "literature", "incropera-2018"),
]


def layer(id, en, de, kind, material, t, k, rho, c):
    """🧶️ One single layer with resistance and areal heat capacity rho c min(t, sqrt(alpha 3600 s))."""
    depth = min(t, math.sqrt(k / (rho * c) * 3600))
    return {"id": id, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": round(t / k, 6), "heatCapacityJM2K": round(rho * c * depth), "source": "literature", "reference": "iso-10456-2007"}


AREA = {"pallet": 2.907, "mirror-carcass": 0.76, "mirror-doors": 0.464, "light-bar": 0.194, "vanity-carcass": 1.066, "drawer": 1.181, "basin": 0.583, "tap": 0.027, "water": 0.02}


def region(id, en, de, kind, model, material, geometry, L, sky, lay, extra=None):
    """🔥️ One thermal region."""
    r = {"id": id, "title": T(en, de), "kind": kind, "model": model, "material": material, "areaM2": AREA[id], "characteristicLengthM": L, "geometry": geometry, "skyViewFactor": sky, "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": "authored-estimate", "reference": "iso-10456-2007"}
    if extra:
        r.update(extra)
    return r


REGIONS = [
    region("basin", "Vitreous-china basin", "Porzellanbecken", "surface", "passive", "vitreous-china", "horizontal-plate-up", 0.48, 0.3, layer("china", "Vitreous china 12 mm", "Sanitärporzellan 12 mm", "masonry", "vitreous-china", 0.012, 1.3, 2350, 840), {"ratedPowerW": 0, "ratedPowerBasis": "passive", "reference": "en-14688-2015"}),
    region("tap", "Single-lever mixer tap", "Einhebelmischer", "surface", "passive", "chrome-brass", "cylinder", 0.045, 0.3, layer("brass", "Brass body 3 mm", "Messingkörper 3 mm", "metal", "chrome-brass", 0.003, 110, 8500, 380), {"reference": "incropera-2018"}),
    region("vanity-carcass", "Melamine vanity carcass", "Melamin-Unterschrankkorpus", "surface", "passive", "melamine-chipboard", "vertical-plate", 0.5, 0.2, layer("v-board", "Chipboard 16 mm", "Spanplatte 16 mm", "timber", "melamine-chipboard", 0.016, 0.13, 650, 1700)),
    region("drawer", "High-gloss drawer with box and handle", "Hochglanzschublade mit Kasten und Griff", "surface", "passive", "mdf-gloss", "vertical-plate", 0.494, 0.2, layer("d-board", "MDF 19 mm", "MDF 19 mm", "timber", "mdf-gloss", 0.019, 0.14, 750, 1700)),
    region("mirror-carcass", "Melamine mirror cabinet carcass and shelves", "Melamin-Spiegelschrankkorpus und Böden", "surface", "passive", "melamine-chipboard", "vertical-plate", 0.7, 0.2, layer("m-board", "Chipboard 16 mm", "Spanplatte 16 mm", "timber", "melamine-chipboard", 0.016, 0.13, 650, 1700)),
    region("mirror-doors", "Mirror doors", "Spiegeltüren", "surface", "passive", "mirror-glass", "vertical-plate", 0.7, 0.2, layer("mirror", "Mirror glass 4 + 4 mm", "Spiegelglas 4 + 4 mm", "glass", "mirror-glass", 0.008, 1.0, 2500, 750), {"reference": "en-1036-1-2007"}),
    region("light-bar", "Light bar with opal diffuser", "Leuchte mit Opaldiffusor", "lamp", "operating", "opal-acrylic", "cylinder", 0.06, 0.4, layer("opal", "Opal acrylic 3 mm", "Opalacryl 3 mm", "polymer", "opal-acrylic", 0.003, 0.19, 1190, 1470), {"operating": {"key": "light", "heatW": 45}, "ratedPowerW": 50, "ratedPowerBasis": "electrical-input", "reference": "iec-60598-1-2020"}),
    region("pallet", "EUR pallet", "EUR-Palette", "surface", "passive", "pallet-spruce", "horizontal-plate-up", 0.8, 0.5, layer("spruce", "Spruce 22 mm", "Fichte 22 mm", "timber", "pallet-spruce", 0.022, 0.13, 470, 1600), {"reference": "en-13698-1-2003"}),
    region("water", "Running tap water", "Laufendes Leitungswasser", "water", "operating", "water", "cylinder", 0.008, 0.3, layer("stream", "Water stream 4 mm (radius)", "Wasserstrahl 4 mm (Radius)", "polymer", "water", 0.004, 0.63, 993, 4180), {"areaBasis": "exchange", "operating": {"key": "water", "heatW": 80}, "ratedPowerW": 0, "ratedPowerBasis": "passive", "reference": "incropera-2018", "hidden": T("The stream is water in free fall without a fixed outline: it is not drawn in any view but carried by the vent of the transport view, by the wetted basin and by the warmed tap.", "Der Strahl ist frei fallendes Wasser ohne feste Kontur: er wird in keiner Ansicht gezeichnet, sondern von der Lüftungsquelle der Transportansicht, vom benetzten Becken und von der erwärmten Armatur getragen.")}),
]
REGION_IDS = [r["id"] for r in REGIONS]


def A(id, bone, x, y, z, kind, en, de, **kw):
    """📍️ An anchor from a world rest point."""
    u, v = loc(bone, x, y)
    a = {"id": id, "bone": bone, "x": u, "y": v, "z": r6(z), "kind": kind, "title": T(en, de)}
    a.update(kw)
    return a


ANCHORS = [
    *[A(f"pallet-floor-{k}", "pallet", 0.0, 0.0, z, "contact", f"Pallet bottom board on the floor, {en}", f"Bodenbrett der Palette auf dem Boden, {de}", support="contact", normalDeg=90, footprintLengthM=1.2, footprintWidthM=0.1 if k != "centre" else 0.145, frictionCoefficient=0.5) for k, z, en, de in (("right", -0.35, "right", "rechts"), ("centre", 0.0, "centre", "Mitte"), ("left", 0.35, "left", "links"))],
    A("grip-left", "vanity", 0.06, PT, 0.282, "support", "Left carrier's hands under the vanity side", "Hände des linken Trägers unter der Unterschrankseite", support="fixed"),
    A("grip-right", "vanity", 0.06, PT, -0.282, "support", "Right carrier's hands under the vanity side", "Hände des rechten Trägers unter der Unterschrankseite", support="fixed"),
    A("vanity-hanger-left", "vanity", XV0, YV1 - 0.03, 0.25, "support", "Left vanity hanger on its wall hook", "Linker Unterschrankaufhänger am Wandhaken", support="pin"),
    A("vanity-hanger-right", "vanity", XV0, YV1 - 0.03, -0.25, "support", "Right vanity hanger on its wall hook", "Rechter Unterschrankaufhänger am Wandhaken", support="pin"),
    A("basin-bolt-left", "basin", XS0, PT + 0.53, 0.2, "support", "Left basin wall bolt", "Linke Wandschraube des Beckens", support="pin"),
    A("basin-bolt-right", "basin", XS0, PT + 0.53, -0.2, "support", "Right basin wall bolt", "Rechte Wandschraube des Beckens", support="pin"),
    A("mirror-hanger-left", "mirror", XM0, YM1 - 0.04, 0.25, "support", "Left mirror cabinet hanger", "Linker Spiegelschrankaufhänger", support="pin"),
    A("mirror-hanger-right", "mirror", XM0, YM1 - 0.04, -0.25, "support", "Right mirror cabinet hanger", "Rechter Spiegelschrankaufhänger", support="pin"),
    A("toiletries", "mirror", -0.33, PT + 0.246, 0.0, "load", "Toiletries on the lower glass shelf", "Toilettenartikel auf dem unteren Glasboden"),
    A("soap", "basin", -0.10, YD1, 0.2, "load", "Soap dish and cup on the deck", "Seifenschale und Becher auf dem Deck"),
    A("drawer-contents", "drawer", 0.08, YB0 + 0.008, 0.0, "load", "Contents of the drawer", "Inhalt der Schublade"),
    A("hands", "basin", XS1 - 0.02, YD1, 0.0, "load", "Hands leaning on the front edge", "Auf die Vorderkante gestützte Hände"),
    A("water-in-bowl", "basin", BOWL_X, PT + 0.40, 0.0, "load", "Water standing in the bowl", "Wasser im Becken"),
    A("handle-pull", "drawer", XF1 + 0.03, HANDLE_Y, 0.0, "load", "Hand pulling the handle", "Hand am Griff"),
    A("light-top", "mirror", XH1, YL1 + 0.01, 0.0, "emitter", "Warm air over the light bar", "Warme Luft über der Leuchte"),
    A("aerator", "basin", 0.034, PT + 0.611, 0.0, "emitter", "Aerator of the spout", "Strahlregler des Auslaufs"),
]


def bone(id, parent, en, de, x, y, length, joint, mass_of=None, ref="en-1991-1-1"):
    """🦴️ One bone of the mechanical rig."""
    b = {"id": id, "parent": parent, "title": T(en, de), "x": r6(x), "y": r6(y), "angleDeg": 0, "lengthM": length, "joint": joint}
    if mass_of:
        u, v, _ = local_com(mass_of)
        b["mass"] = {"kg": round(MASS[mass_of]["kg"], 5), "comX": round(u, 5), "comY": round(v, 5), "distribution": "point", "source": "authored-estimate", "reference": ref}
    return b


FIXED = {"type": "fixed"}
BONES = [
    bone("ground", None, "Depot floor reference", "Bodenbezug im Lager", 0, 0, 0, FIXED),
    bone("pallet", "ground", "EUR pallet (stays behind)", "EUR-Palette (bleibt zurück)", -0.6, 0, 1.2, FIXED, "pallet", "en-13698-1-2003"),
    bone("mirror-travel", "ground", "Travel of the mirror cabinet along the floor", "Weg des Spiegelschranks über den Boden", 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.0}),
    bone("mirror-lift", "mirror-travel", "Lift of the mirror cabinet", "Hub des Spiegelschranks", 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -0.05, "maxM": 1.2}),
    bone("mirror", "mirror-lift", "Mirror cabinet with doors and light bar", "Spiegelschrank mit Türen und Leuchte", XM0, PT, 0.16, FIXED, "mirror", "en-312-2010"),
    bone("vanity-travel", "ground", "Travel of the vanity unit along the floor", "Weg des Unterschranks über den Boden", 0, 0, 0, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 2.0}),
    bone("vanity-lift", "vanity-travel", "Lift of the vanity unit", "Hub des Unterschranks", 0, 0, 0, {"type": "prismatic", "axisDeg": 90, "minM": -0.05, "maxM": 0.8}),
    bone("vanity", "vanity-lift", "Vanity carcass (origin at the rear lower edge)", "Unterschrankkorpus (Ursprung an der hinteren Unterkante)", XV0, PT, 0.44, FIXED, "vanity", "en-312-2010"),
    bone("drawer", "vanity", "Drawer on ball-bearing runners", "Schublade auf Kugelauszügen", ORIGIN["drawer"][0] - XV0, ORIGIN["drawer"][1] - PT, 0.419, {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 0.35}, "drawer", "en-622-5-2009"),
    bone("basin", "vanity", "Basin with mixer tap, set on the vanity", "Becken mit Mischbatterie, auf dem Unterschrank", 0, 0.5, 0.48, FIXED, "basin", "en-14688-2015"),
    bone("lever", "basin", "Mixer lever (lifts to open)", "Mischhebel (hebt zum Öffnen)", ORIGIN["lever"][0] - XS0, ORIGIN["lever"][1] - (PT + 0.5), LEVER_LEN, {"type": "revolute", "minDeg": -2, "maxDeg": 25}, "lever", "en-817-2008"),
]


def binding(light=0.0, water=0.0):
    """🌬️ The thermal binding of every region of a pose."""
    regs = {r: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGION_IDS}
    regs["pallet"]["contact"] = {"fraction": 0.3, "tempC": 16}
    regs["water"]["wettedness"] = water
    return {"regions": regs, "emitters": {"light-plume": light, "basin-mist": water}, "operating": {"light": light, "water": water}}


def imposed(id, en, de, b, fy):
    """⬆️ The floor reaction of a part that rests on its own, as an upward load at its centre."""
    u, v, _ = local_com(b)
    return {"id": id, "kind": "imposed", "title": T(en, de), "bone": b, "x": round(u, 5), "y": round(v, 5), "fxN": 0, "fyN": round(fy, 3), "source": "literature", "reference": "en-1991-1-1"}


PALLET_REACTION = imposed("floor-under-pallet", "Floor reaction under the pallet, which stays behind on its own", "Bodenreaktion unter der Palette, die für sich zurückbleibt", "pallet", W["pallet"])
MIRROR_REACTION = imposed("pallet-under-mirror", "Pallet reaction under the mirror cabinet, which stays on the pallet", "Palettenreaktion unter dem Spiegelschrank, der auf der Palette bleibt", "mirror", W["mirror"])


def load(id, kind, en, de, anchor, kg=None, fx=0.0, fy=None, refn="en-1991-1-1"):
    """⬇️ A load at an anchor."""
    return {"id": id, "kind": kind, "title": T(en, de), "anchor": anchor, "fxN": round(fx, 3), "fyN": round(-kg * G if fy is None else fy, 3), "source": "authored-estimate", "reference": refn}


IN_USE_LOADS = [
    load("toiletries-load", "equipment", "Toiletries, 2.0 kg", "Toilettenartikel, 2,0 kg", "toiletries", 2.0, refn="en-16122-2012"),
    load("soap-load", "equipment", "Soap dish and tooth mug, 0.6 kg", "Seifenschale und Zahnputzbecher, 0,6 kg", "soap", 0.6, refn="en-14688-2015"),
    load("drawer-load", "equipment", "Drawer contents, 2.5 kg", "Schubladeninhalt, 2,5 kg", "drawer-contents", 2.5, refn="en-16122-2012"),
    load("hands-load", "live", "Hands leaning on the front edge, 150 N", "Auf die Vorderkante gestützte Hände, 150 N", "hands", fy=-150.0, refn="en-14688-2015"),
]
HANG = ["vanity-hanger-left", "vanity-hanger-right", "basin-bolt-left", "basin-bolt-right", "mirror-hanger-left", "mirror-hanger-right"]
FLOOR = ["pallet-floor-right", "pallet-floor-centre", "pallet-floor-left"]
GRIPS = ["grip-left", "grip-right"]
IN_USE = {"mirror-travel": {"dx": 1.5}, "mirror-lift": {"dy": 1.056}, "vanity-travel": {"dx": 1.26}, "vanity-lift": {"dy": 0.136}}
CARRY_DY = 0.72 - PT


def pose(id, en, de, den, dde, bones, contacts, th, loads, key=False):
    """🧍️ One pose."""
    p = {"id": id}
    if key:
        p["key"] = True
    p.update({"title": T(en, de), "description": T(den, dde), "bones": bones, "ground": "authored", "contacts": contacts, "thermal": th, "loads": loads})
    return p


POSES = [
    pose("stored", "Stored on the pallet", "Auf der Palette gelagert", "Emptied, cleaned and taken out at a flat clearance: the vanity unit with the basin set on it and the mirror cabinet behind it, mirror doors facing the vanity, stand on a EUR pallet in the reuse depot; the siphon is removed, the pop-up waste and the tap stay on the basin.", "Bei einer Haushaltsauflösung geleert, gereinigt und ausgebaut: der Unterschrank mit aufgesetztem Becken und dahinter der Spiegelschrank, Spiegeltüren zum Unterschrank, stehen auf einer EUR-Palette im Wiederverwendungslager; der Siphon ist entfernt, Ablaufgarnitur und Armatur bleiben am Becken.", {}, FLOOR, binding(), []),
    pose("lift-off-key", "Lifting off the pallet", "Abheben von der Palette", "Two carriers have taken the vanity under its sides and lift it with the basin clear of the pallet.", "Zwei Träger haben den Unterschrank unter den Seiten gefasst und heben ihn mit dem Becken von der Palette.", {"vanity-lift": {"dy": 0.03}}, GRIPS, binding(), [PALLET_REACTION, MIRROR_REACTION], key=True),
    pose("carried", "Carried by two people", "Von zwei Personen getragen", f"The vanity unit with the basin taped onto it is carried level by two people, one at each side, hands under the side panels at {0.72:.2f} m; the pallet and the mirror cabinet stay behind.", f"Der Unterschrank mit dem aufgeklebten Becken wird von zwei Personen waagerecht getragen, je eine an jeder Seite, die Hände unter den Seitenwänden in {0.72:.2f} m Höhe; Palette und Spiegelschrank bleiben zurück.", {"vanity-travel": {"dx": 1.0}, "vanity-lift": {"dy": round(CARRY_DY, 4)}}, GRIPS, binding(), [PALLET_REACTION, MIRROR_REACTION]),
    pose("in-use", "Reinstalled in a bathroom", "Wieder im Bad montiert", "Hung on a bathroom wall again: basin rim at 0.85 m, vanity on two hangers and the basin on two wall bolts, the mirror cabinet with its underside at 1.20 m; the light is on, toiletries on the shelf, a soap dish on the deck.", "Wieder an eine Badwand gehängt: Beckenrand in 0,85 m, der Unterschrank an zwei Aufhängern und das Becken an zwei Wandschrauben, der Spiegelschrank mit der Unterkante in 1,20 m; das Licht ist an, Toilettenartikel auf dem Boden, eine Seifenschale auf dem Deck.", IN_USE, HANG, binding(1.0), [PALLET_REACTION] + IN_USE_LOADS),
    pose("in-use-dark", "Reinstalled, light off", "Montiert, Licht aus", "Reinstalled as in use with the light bar switched off.", "Wie in Gebrauch montiert, die Leuchte ausgeschaltet.", IN_USE, HANG, binding(0.0), [PALLET_REACTION] + IN_USE_LOADS),
    pose("washing", "Washing hands", "Händewaschen", "The lever is lifted 20 degrees and warm water runs from the aerator into the bowl, about 5 L/min at 38 degC; the light is on.", "Der Hebel ist um 20 Grad angehoben und warmes Wasser läuft aus dem Strahlregler ins Becken, etwa 5 L/min bei 38 degC; das Licht ist an.", {**IN_USE, "lever": {"angleDeg": 20}}, HANG, binding(1.0, 1.0), [PALLET_REACTION] + IN_USE_LOADS + [load("water-load", "live", "Water in the bowl, 0.5 kg", "Wasser im Becken, 0,5 kg", "water-in-bowl", 0.5, refn="en-14688-2015")]),
    pose("drawer-open", "Drawer pulled out", "Schublade gezogen", "Reinstalled with the drawer pulled 0.30 m out on its runners by the handle.", "Montiert, die Schublade am Griff 0,30 m auf ihren Auszügen herausgezogen.", {**IN_USE, "drawer": {"dx": 0.3}}, HANG, binding(1.0), [PALLET_REACTION] + IN_USE_LOADS + [load("pull", "live", "Pull at the handle, 15 N", "Zug am Griff, 15 N", "handle-pull", fx=15.0, fy=0.0, refn="en-16122-2012")]),
]


def step(target, keys):
    """📶️ A step track."""
    return {"target": target, "ease": "step", "keys": [{"t": t, "v": v} for t, v in keys]}


CLIPS = [
    {"id": "carry", "title": T("Carry the vanity off the pallet", "Unterschrank von der Palette tragen"), "description": T("Two carriers take the vanity under its sides, lift it with the basin off the pallet and carry it away level; the pallet and the mirror cabinet stay in the depot.", "Zwei Träger fassen den Unterschrank unter den Seiten, heben ihn mit dem Becken von der Palette und tragen ihn waagerecht fort; Palette und Spiegelschrank bleiben im Lager."), "durationS": 3.0, "fps": 30, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 0.6, "pose": "lift-off-key", "ease": "smooth"}, {"t": 2.4, "pose": "carried", "ease": "smooth"}, {"t": 3.0, "pose": "carried", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 3.0} for a in FLOOR] + [{"anchor": a, "fromS": 0, "toS": 3.0} for a in GRIPS],
     "phases": [{"id": "grip", "fromS": 0, "toS": 0.6, "title": T("Take hold and lift", "Fassen und anheben")}, {"id": "walk", "fromS": 0.6, "toS": 2.4, "title": T("Lift to hip height and walk", "Auf Hüfthöhe heben und gehen")}, {"id": "carry", "fromS": 2.4, "toS": 3.0, "title": T("Carry", "Tragen")}],
     "events": [{"t": 0.3, "kind": "contact-off", "title": T("Vanity leaves the pallet", "Unterschrank verlässt die Palette")}, {"t": 2.4, "kind": "motion", "title": T("Carried level at hip height", "Waagerecht auf Hüfthöhe getragen")}], "tracks": []},
    {"id": "wash-hands", "title": T("Switch on the light and wash hands", "Licht an und Hände waschen"), "description": T("Reinstalled: the light bar is switched on, the lever is lifted, warm water runs for 4.5 s, the lever is pushed down again; the light stays on.", "Montiert: die Leuchte wird eingeschaltet, der Hebel angehoben, warmes Wasser läuft 4,5 s, der Hebel wird wieder heruntergedrückt; das Licht bleibt an."), "durationS": 8.0, "fps": 25, "loop": False, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-use-dark", "ease": "smooth"}, {"t": 0.5, "pose": "in-use", "ease": "smooth"}, {"t": 1.5, "pose": "washing", "ease": "smooth"}, {"t": 6.0, "pose": "washing", "ease": "smooth"}, {"t": 7.0, "pose": "in-use", "ease": "smooth"}, {"t": 8.0, "pose": "in-use", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 8.0} for a in HANG],
     "phases": [{"id": "light", "fromS": 0, "toS": 0.5, "title": T("Switch on", "Einschalten")}, {"id": "open", "fromS": 0.5, "toS": 1.5, "title": T("Lift the lever", "Hebel anheben")}, {"id": "wash", "fromS": 1.5, "toS": 6.0, "title": T("Water runs", "Wasser läuft")}, {"id": "close", "fromS": 6.0, "toS": 7.0, "title": T("Push the lever down", "Hebel herunterdrücken")}, {"id": "after", "fromS": 7.0, "toS": 8.0, "title": T("Light stays on", "Licht bleibt an")}],
     "events": [{"t": 0.5, "kind": "operating", "title": T("Light switched on", "Licht eingeschaltet"), "target": "light"}, {"t": 1.5, "kind": "operating", "title": T("Water running", "Wasser läuft"), "target": "water"}, {"t": 6.0, "kind": "shutdown", "title": T("Water off", "Wasser aus"), "target": "water"}],
     "tracks": [step("operating:light", [(0, 0), (0.5, 1)]), step("emitter:light-plume", [(0, 0), (0.5, 1)]), step("operating:water", [(0, 0), (1.5, 1), (6.0, 0)]), step("emitter:basin-mist", [(0, 0), (1.5, 1), (6.0, 0)])]},
    {"id": "drawer-pull", "title": T("Pull the drawer and push it back", "Schublade ziehen und zurückschieben"), "description": T("Reinstalled: the drawer is pulled 0.30 m out by its handle, held and pushed back.", "Montiert: die Schublade wird am Griff 0,30 m herausgezogen, gehalten und zurückgeschoben."), "durationS": 3.0, "fps": 30, "loop": True, "rootMotion": "none",
     "keys": [{"t": 0, "pose": "in-use", "ease": "smooth"}, {"t": 1.0, "pose": "drawer-open", "ease": "smooth"}, {"t": 2.0, "pose": "drawer-open", "ease": "smooth"}, {"t": 3.0, "pose": "in-use", "ease": "smooth"}],
     "contacts": [{"anchor": a, "fromS": 0, "toS": 3.0} for a in HANG],
     "phases": [{"id": "pull", "fromS": 0, "toS": 1.0, "title": T("Pull", "Ziehen")}, {"id": "hold", "fromS": 1.0, "toS": 2.0, "title": T("Open", "Offen")}, {"id": "push", "fromS": 2.0, "toS": 3.0, "title": T("Push back", "Zurückschieben")}],
     "events": [{"t": 1.0, "kind": "motion", "title": T("Drawer out", "Schublade offen")}, {"t": 3.0, "kind": "motion", "title": T("Drawer shut", "Schublade zu")}], "tracks": []},
]

SYMBOL = {"id": "used-washbasin-vanity-symbol", "title": T("Washbasin set on its pallet, true extent", "Waschtischset auf seiner Palette, wahre Ausdehnung"), "description": T("The stored set in all three views: EUR pallet 1.2 x 0.8 m, mirror cabinet 0.6 x 0.17 x 0.76 m and vanity with basin 0.6 x 0.48 x 0.57 m standing on it.", "Das gelagerte Set in allen drei Ansichten: EUR-Palette 1,2 x 0,8 m, Spiegelschrank 0,6 x 0,17 x 0,76 m und Unterschrank mit Becken 0,6 x 0,48 x 0,57 m darauf."),
          "semantics": "true-extent", "units": "model-m", "denominators": [200, 100, 50], "pens": {"body": 0.25, "detail": 0.13, "texture": None, "glazing": None, "ground": 0.35}, "hatches": [],
          "geometry": {
              "elevation": [sh("pallet", "ground", "body", 1, True, 0.45, rect(-0.6, 0, 0.6, PT)), sh("mirror", "ground", "body", 2, True, 0.8, poly([(XM0, PT), (-0.244, PT), (-0.244, YM1), (XL1, YM1), (XL1, YL1), (XM0, YL1)])), sh("vanity", "ground", "body", 3, True, 0.85, poly([(XV0, PT), (XF1, PT), (XF1, PT + 0.5), (XS1, PT + 0.5), (XS1, YD1), (XS0, YD1)])), sh("tap", "ground", "detail", 4, True, 0.5, rect(TAP_X - 0.0225, YD1, TAP_X + 0.0225, YD1 + 0.09))],
              "front": [sh("pallet", "ground", "body", 1, True, 0.45, rect(-0.4, 0, 0.4, PT)), sh("mirror", "ground", "body", 2, True, 0.7, rect(-ZM, PT, ZM, YL1)), sh("vanity", "ground", "body", 3, True, 0.85, poly([(-ZV, PT), (ZV, PT), (ZV, PT + 0.5), (ZS, PT + 0.5), (ZS, YD1), (-ZS, YD1), (-ZS, PT + 0.5), (-ZV, PT + 0.5)])), sh("drawer-line", "ground", "detail", 4, False, 0.3, rect(-0.284, PT + 0.003, 0.284, PT + 0.497))],
              "plan": [sh("pallet", "ground", "body", 1, True, 0.45, rect(-0.6, -0.4, 0.6, 0.4)), sh("mirror", "ground", "body", 2, True, 0.7, rect(XM0, -ZM, XL1, ZM)), sh("basin", "ground", "body", 3, True, 0.9, rect(XS0, -ZS, XS1, ZS)), sh("bowl", "ground", "detail", 4, True, 0.6, ellipse(BOWL_X, 0.0, 0.15, 0.21))],
          },
          "dimensionalNote": T("Model metres at real size in the frame of the asset (x ahead to the fronts, lateral positive to the left, y up): the side view is seen from the right, the front view from ahead, the top view from above; all three show the stored pose on the pallet.", "Modellmeter in wahrer Größe im Koordinatensystem des Objekts (x nach vorn zu den Fronten, seitlich positiv nach links, y nach oben): die Seitenansicht blickt von rechts, die Vorderansicht von vorn, die Draufsicht von oben; alle drei zeigen die gelagerte Pose auf der Palette.")}

REFERENCES = [
    {"id": "en-14688-2015", "citation": "EN 14688:2015 Sanitary appliances - Wash basins - Functional requirements and test methods (static load test of the fixed basin, 1.5 kN for one hour without failure; water-tightness and cleanability of the glaze)."},
    {"id": "en-31-2011", "citation": "EN 31:2011 Pedestal wash basins - Connecting dimensions (tap hole of 35 mm, waste outlet of 45 mm diameter, fixing hole positions)."},
    {"id": "en-817-2008", "citation": "EN 817:2008 Sanitary tapware - Mechanical mixing valves (PN 10) - General technical specifications (single-lever mixers, flow rate classes about 0.25 L/s at 3 bar)."},
    {"id": "din-18022-2007", "citation": "DIN 18022:2007-11 Küche, Bad und WC im Wohnungsbau - Planungsgrundlagen (washbasin 0.60 m wide, rim height 0.85-0.90 m above the floor, mirror and mirror cabinet above it)."},
    {"id": "vdi-6000-1-2008", "citation": "VDI 6000 Blatt 1:2008-02 Ausstattung von und mit Sanitärräumen - Wohnungen (washbasin rim height 0.85 m, mirror lower edge about 1.20 m)."},
    {"id": "en-16122-2012", "citation": "EN 16122:2012 Domestic and non-domestic storage furniture - Test methods for the determination of strength, durability and stability (loads on shelves and in drawers, extension elements pulled out)."},
    {"id": "en-14749-2016", "citation": "EN 14749:2016 Domestic and kitchen storage units and worktops - Safety requirements and test methods (wall-hung units: strength of the suspension, stability with extension elements open)."},
    {"id": "en-312-2010", "citation": "EN 312:2010 Particleboards - Specifications (type P2 for interior fitments in dry conditions: 11 N/mm2 bending strength, 1600 N/mm2 modulus for 13-20 mm boards)."},
    {"id": "en-622-5-2009", "citation": "EN 622-5:2009 Fibreboards - Specifications - Part 5: Requirements for dry process boards (MDF, about 700-800 kg/m3, bending strength 20 N/mm2 and modulus 2200 N/mm2 for 12-19 mm)."},
    {"id": "en-622-2-2004", "citation": "EN 622-2:2004 Fibreboards - Specifications - Part 2: Requirements for hardboards (HB, density at least 900 kg/m3)."},
    {"id": "en-338-2016", "citation": "EN 338:2016 Structural timber - Strength classes (C24 softwood: 24 N/mm2 bending, 11 kN/mm2 modulus, 420 kg/m3 characteristic and about 470 kg/m3 mean density)."},
    {"id": "en-13698-1-2003", "citation": "EN 13698-1:2003 Pallet production specification - Part 1: Construction specification for 800 mm x 1200 mm flat wooden pallets (deck boards 22 mm, blocks 78 mm high, 144 mm overall height, about 20-25 kg)."},
    {"id": "en-1036-1-2007", "citation": "EN 1036-1:2007 Glass in building - Mirrors from silver-coated float glass for internal use (edge corrosion, the black edge desilvering of bathroom mirrors)."},
    {"id": "en-572-1-2012", "citation": "EN 572-1:2012 Glass in building - Basic soda lime silicate glass products (density 2500 kg/m3, modulus 70 GPa)."},
    {"id": "iec-60598-1-2020", "citation": "IEC 60598-1:2020 Luminaires - Part 1: General requirements and tests (protection class and the rated power of mirror-cabinet luminaires with two E14 lamps of 25 W)."},
    {"id": "en-1991-1-1", "citation": "EN 1991-1-1:2002 Eurocode 1: Actions on structures - Part 1-1: General actions - Densities, self-weight, imposed loads for buildings (self-weight as volume x density)."},
    {"id": "iso-10456-2007", "citation": "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (particleboard 0.13 W/mK and 1700 J/kgK, fibreboard, ceramics 1.3 W/mK and 840 J/kgK, glass 1.0 W/mK and 750 J/kgK, softwood 0.13 W/mK and 1600 J/kgK)."},
    {"id": "incropera-2018", "citation": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivity of glazed ceramics 0.90-0.94, of glass 0.84-0.92, of dulled chrome 0.1-0.2, of anodised aluminium 0.7-0.8; water 4180 J/kgK)."},
    {"id": "din-33402-2-2020", "citation": "DIN 33402-2:2020-12 Ergonomie - Körpermaße des Menschen - Teil 2: Werte (grip height of the hanging arm 0.72-0.78 m, taken as 0.72 m for a two-person carry at hip height)."},
    {"id": "bathroom-1990s-type", "citation": "Type data of German bathroom furniture sets of 1990-2005 as DIY stores and sanitary wholesalers offered them: 60 cm vitreous-china furniture basin 600 x 480 mm with a 70 mm edge, 15-18 kg; wall-hung vanity unit 580 x 440 x 500 mm of white melamine chipboard with one high-gloss drawer, 15-20 kg; mirror cabinet 600 x 160 x 700 mm with two mirror doors, two glass shelves and a light bar with two E14 lamps, 15-20 kg; no single product, no brand."},
    {"id": "neufert-2019", "citation": "Neufert, E., Kister, J. (2019). Bauentwurfslehre, 42nd ed., Springer Vieweg (bathroom: washbasin 0.60 x 0.48 m, rim at 0.85 m, mirror cabinet above it)."},
]

NOTES_EN = (f"German furniture washbasin set of the 1990s to 2000s type, taken out at a flat clearance, emptied and cleaned. Basin 600 x 480 mm of vitreous china with a 70 mm edge (12 mm deck, 15 mm apron), an elliptical bowl 300 x 420 mm at the rim and 185 mm deep with its 45 mm pop-up waste left in place (siphon removed), and a chrome single-lever mixer on the tap platform. Vanity unit 580 x 440 x 500 mm of 16 mm white melamine chipboard P2, wall-hung on two hangers, with a 3 mm back and its pipe cut-out and one full-width drawer (19 mm high-gloss MDF front, chipboard box on ball-bearing runners, replaced aluminium bar handle). Mirror cabinet 600 x 160 x 700 mm with two 8 mm mirror doors, two 6 mm glass shelves and a 60 mm light bar with a yellowed opal diffuser for two E14 lamps of 25 W. Traces of use: limescale crust at the tap base and the waste, a soap mark, a chip in the basin edge, swollen chipboard edges, chipped lacquer and scratches on the drawer front with the two holes of the original handle, desilvered mirror corners, limescale splashes and stickers, the mains flex taped to the back; nothing that stops it working. Stored on a EUR pallet of 1200 x 800 x 144 mm that stays behind. Masses from the 1:50 volumes x density: pallet {MASS['pallet']['kg']:.1f} kg, mirror cabinet {MASS['mirror']['kg']:.1f} kg, vanity {MASS['vanity']['kg']:.1f} kg, drawer {MASS['drawer']['kg']:.1f} kg, basin with tap {MASS['basin']['kg'] + MASS['lever']['kg']:.1f} kg.")
NOTES_DE = (f"Deutsches Möbelwaschtisch-Set vom Typ der 1990er- bis 2000er-Jahre, bei einer Haushaltsauflösung ausgebaut, geleert und gereinigt. Becken 600 x 480 mm aus Sanitärporzellan mit 70-mm-Kante (12-mm-Deck, 15-mm-Schürze), ein elliptisches Becken 300 x 420 mm am Rand und 185 mm tief mit der verbliebenen 45-mm-Ablaufgarnitur (Siphon entfernt) und eine verchromte Einhebelmischbatterie auf der Hahnbank. Unterschrank 580 x 440 x 500 mm aus 16-mm-weißer Melamin-Spanplatte P2, an zwei Aufhängern wandhängend, mit 3-mm-Rückwand und Rohrausschnitt und einer Schublade über die ganze Breite (19-mm-Hochglanz-MDF-Front, Spanplattenkasten auf Kugelauszügen, ersetzter Aluminium-Bügelgriff). Spiegelschrank 600 x 160 x 700 mm mit zwei 8-mm-Spiegeltüren, zwei 6-mm-Glasböden und einer 60-mm-Leuchte mit vergilbtem Opaldiffusor für zwei E14-Lampen zu 25 W. Gebrauchsspuren: Kalkkruste am Armaturenfuß und am Ablauf, ein Seifenrand, eine Abplatzung an der Beckenkante, aufgequollene Spanplattenkanten, abgeplatzter Lack und Kratzer auf der Schubladenfront mit den zwei Löchern des ursprünglichen Griffs, erblindete Spiegelecken, Kalkspritzer und Aufkleber, die an die Rückwand geklebte Netzleitung; nichts, was den Gebrauch hindert. Gelagert auf einer EUR-Palette 1200 x 800 x 144 mm, die zurückbleibt. Massen aus den 1:50-Volumen x Dichte: Palette {MASS['pallet']['kg']:.1f} kg, Spiegelschrank {MASS['mirror']['kg']:.1f} kg, Unterschrank {MASS['vanity']['kg']:.1f} kg, Schublade {MASS['drawer']['kg']:.1f} kg, Becken mit Armatur {MASS['basin']['kg'] + MASS['lever']['kg']:.1f} kg.")

SET_KG = TOTAL_KG - MASS["pallet"]["kg"]
ASSET = {
    "id": "used-washbasin-vanity", "category": "objects",
    "title": T("Used Washbasin with Vanity Unit", "Gebrauchter Waschtisch mit Unterschrank"),
    "summary": T(f"A 60 cm vitreous-china washbasin with a single-lever mixer on a wall-hung white melamine vanity unit with one high-gloss drawer, and the matching mirror cabinet with two mirror doors and a light bar, {SET_KG:.0f} kg together; limescale at the tap and waste, a chipped basin edge, swollen chipboard edges, a replaced handle and desilvered mirror corners. Stored on a EUR pallet, carried by two people, reinstalled with the rim at 0.85 m, light on and water running.", f"Ein 60-cm-Waschtisch aus Sanitärporzellan mit Einhebelmischer auf einem wandhängenden weißen Melamin-Unterschrank mit einer Hochglanzschublade und der passende Spiegelschrank mit zwei Spiegeltüren und Leuchte, zusammen {SET_KG:.0f} kg; Kalk an Armatur und Ablauf, eine abgeplatzte Beckenkante, aufgequollene Spanplattenkanten, ein ersetzter Griff und erblindete Spiegelecken. Auf einer EUR-Palette gelagert, von zwei Personen getragen, wieder montiert mit dem Rand in 0,85 m, Licht an und laufendem Wasser."),
    "tags": ["used", "household", "second-hand", "bathroom", "washbasin", "vanity-unit", "mirror-cabinet", "vitreous-china", "ceramic", "melamine", "chipboard", "mirror", "tap", "pallet", "1990s", "interior"],
    "provenance": {"author": "Semio staffage content (used household objects wave)", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": T(NOTES_EN, NOTES_DE)},
    "dimensions": {"heightM": round(YL1, 4), "widthM": 1.2, "depthM": 0.8, "massKg": round(TOTAL_KG, 5)},
    "materials": MATERIALS,
    "rig": {"kind": "mechanical", "root": "ground", "bones": BONES},
    "anchors": ANCHORS,
    "representations": {"50": rep(50), "100": rep(100), "200": rep(200), "500": rep500()},
    "symbols": [SYMBOL],
    "poses": POSES,
    "clips": CLIPS,
    "thermal": {"environment": {"airTempC": 22, "meanRadiantTempC": 21, "relativeHumidity": 0.6, "windMS": 0.05, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0, "pressurePa": 101325},
                "operating": [{"id": "light", "title": T("Light bar", "Leuchte"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("On (2 x 25 W)", "An (2 x 25 W)")}]}, {"id": "water", "title": T("Warm water", "Warmwasser"), "levels": [{"v": 0, "title": T("Off", "Aus")}, {"v": 1, "title": T("Running (5 L/min at 38 degC)", "Läuft (5 L/min bei 38 degC)")}]}],
                "regions": REGIONS,
                "emitters": [{"id": "light-plume", "kind": "plume", "title": T("Warm air rising over the light bar", "Warme Luft über der Leuchte"), "anchor": "light-top", "directionDeg": 90, "diameterM": 0.1, "region": "light-bar", "source": "authored-estimate", "reference": "incropera-2018"},
                             {"id": "basin-mist", "kind": "vent", "title": T("Warm moist air over the running water", "Warme feuchte Luft über dem laufenden Wasser"), "anchor": "aerator", "directionDeg": -90, "diameterM": 0.02, "operatingKey": "water", "volumeFlowM3S": 0.002, "temperatureRiseK": 2, "vapourKgPerKg": 0.004, "source": "authored-estimate", "reference": "incropera-2018"}],
                "passengers": [],
                "assumptions": [
                    T("Vitreous china after ISO 10456 and EN 14688 (2350 kg/m3, 1.3 W/mK, 840 J/kgK); its white glaze emits 0.92 and absorbs 0.3 of the light, a little more than new because limescale dulls it (authored estimate).", "Sanitärporzellan nach ISO 10456 und EN 14688 (2350 kg/m3, 1,3 W/mK, 840 J/kgK); seine weiße Glasur strahlt 0,92 ab und absorbiert 0,3 des Lichts, etwas mehr als neu, weil Kalk sie stumpf macht (Schätzwert)."),
                    T("Aged surfaces: the yellowed melamine facing absorbs 0.35 instead of about 0.25 when new, the yellowed opal diffuser 0.45, the dulled and spotted chrome emits 0.15 instead of 0.07 polished (Incropera ranges, authored estimates).", "Gealterte Oberflächen: die vergilbte Melaminbeschichtung absorbiert 0,35 statt etwa 0,25 neu, der vergilbte Opaldiffusor 0,45, das stumpfe und fleckige Chrom strahlt 0,15 statt 0,07 poliert ab (Bereiche nach Incropera, Schätzwerte)."),
                    T("The light bar draws 50 W (two E14 lamps of 25 W, electrical input) and releases about 45 W as heat at the bar; the warm water is fed from the house and draws nothing (passive 0 W): about 80 W of heat reach the room from the stream and the wetted bowl while it runs.", "Die Leuchte nimmt 50 W auf (zwei E14-Lampen zu 25 W, elektrische Aufnahme) und gibt etwa 45 W als Wärme an der Leiste ab; das Warmwasser kommt aus dem Haus und nimmt nichts auf (passiv 0 W): etwa 80 W Wärme gelangen vom Strahl und vom benetzten Becken in den Raum, solange es läuft."),
                    T("Reference case: a bathroom at 22 degC and 60 % humidity without sun; in the depot the pallet lies with 0.3 of its surface on a 16 degC floor. The vanity interior is open to the air through the pipe cut-out of the back, so its inner faces and the bowl underside count as outer area.", "Referenzfall: ein Bad bei 22 degC und 60 % Feuchte ohne Sonne; im Lager liegt die Palette mit 0,3 ihrer Oberfläche auf einem 16 degC warmen Boden. Das Innere des Unterschranks ist über den Rohrausschnitt der Rückwand zur Luft offen, daher zählen seine Innenflächen und die Beckenunterseite als Außenfläche."),
                ]},
    "structure": {"gravityMS2": G, "dynamics": "quasi-static", "supports": [],
                  "assumptions": [
                      T("One rigid body on the contacts of each pose: stored, the three bottom boards of the pallet stand on the depot floor and carry everything; carried, the hands of the two carriers under the vanity sides are fixed supports (they hold the 16 mm side edges and take the tipping moment); in use, the vanity hangs on two hangers, the basin on two wall bolts and the mirror cabinet on two hangers, all pins.", "Ein starrer Körper auf den Kontakten jeder Pose: gelagert stehen die drei Bodenbretter der Palette auf dem Lagerboden und tragen alles; getragen sind die Hände der zwei Träger unter den Unterschrankseiten feste Auflager (sie halten die 16-mm-Seitenkanten und nehmen das Kippmoment auf); in Gebrauch hängt der Unterschrank an zwei Aufhängern, das Becken an zwei Wandschrauben und der Spiegelschrank an zwei Aufhängern, alle gelenkig."),
                      T(f"Parts that stay behind rest on their own: the floor reaction of the pallet ({W['pallet']:.0f} N) and, while the vanity is carried, the pallet reaction of the mirror cabinet ({W['mirror']:.0f} N) are entered as upward loads at their centres, so the hands carry the vanity and basin alone ({(MASS['vanity']['kg'] + MASS['drawer']['kg'] + MASS['basin']['kg'] + MASS['lever']['kg']) * G:.0f} N) and the wall fittings the set alone.", f"Zurückbleibende Teile liegen für sich: die Bodenreaktion der Palette ({W['pallet']:.0f} N) und, während der Unterschrank getragen wird, die Palettenreaktion des Spiegelschranks ({W['mirror']:.0f} N) sind als aufwärts gerichtete Lasten in ihren Schwerpunkten angesetzt, so tragen die Hände nur Unterschrank und Becken ({(MASS['vanity']['kg'] + MASS['drawer']['kg'] + MASS['basin']['kg'] + MASS['lever']['kg']) * G:.0f} N) und die Wandbeschläge nur das Set."),
                      T("Members: 16 mm chipboard P2 (EN 312), 19 mm MDF (EN 622-5), 3 mm hardboard (EN 622-2), vitreous china (bending strength about 40 N/mm2), mirror and float glass, spruce C24 pallet boards (EN 338). Masses are section x density of the 1:50 solids; the basin is taped onto the vanity for the carry.", "Stäbe: 16-mm-Spanplatte P2 (EN 312), 19-mm-MDF (EN 622-5), 3-mm-Hartfaser (EN 622-2), Sanitärporzellan (Biegefestigkeit etwa 40 N/mm2), Spiegel- und Floatglas, Palettenbretter aus Fichte C24 (EN 338). Massen sind Querschnitt x Dichte der 1:50-Körper; das Becken ist für das Tragen auf den Unterschrank geklebt."),
                      T("In use: toiletries 2.0 kg on the lower shelf, a soap dish and mug 0.6 kg, 2.5 kg in the drawer and 150 N of hands leaning on the front edge (authored estimates, far below the 1.5 kN static test of EN 14688 and the drawer and shelf test loads of EN 16122); with the drawer 0.30 m out and 15 N at the handle the wall-hung vanity stays on its hangers (EN 14749).", "In Gebrauch: Toilettenartikel 2,0 kg auf dem unteren Boden, Seifenschale und Becher 0,6 kg, 2,5 kg in der Schublade und 150 N von auf die Vorderkante gestützten Händen (Schätzwerte, weit unter der statischen Prüflast von 1,5 kN nach EN 14688 und den Schubladen- und Bodenprüflasten nach EN 16122); mit 0,30 m gezogener Schublade und 15 N am Griff bleibt der wandhängende Unterschrank an seinen Aufhängern (EN 14749)."),
                  ], "clinical": False},
    "volumes": VOLUMES,
    "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
    "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
    "completion": "authored",
}

ORDER = ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "volumes", "presentation", "exports", "completion"]
DOC = {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": {k: ASSET[k] for k in ORDER}}

if __name__ == "__main__":
    (HERE / "🔣️.json").write_text(json.dumps(DOC, ensure_ascii=False, indent=1) + "\n")
    for b, m in MASS.items():
        print(f"[DEBUG] {b}: {m['kg']:.3f} kg at ({m['x']:.4f}, {m['y']:.4f}, {m['z']:.4f})")
    print(f"[DEBUG] total {TOTAL_KG:.3f} kg, set without pallet {SET_KG:.3f} kg; solids per scale " + ", ".join(f"1:{s} {len(PARTS[s])}" for s in PARTS))
