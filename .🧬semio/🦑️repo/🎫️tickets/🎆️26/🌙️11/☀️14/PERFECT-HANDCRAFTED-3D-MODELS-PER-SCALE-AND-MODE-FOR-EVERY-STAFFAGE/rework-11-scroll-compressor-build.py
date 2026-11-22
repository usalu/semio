"""🌀 Rework 11: the hermetic scroll compressor with its real mechanism, every scale stated by hand.

Usage: python3 rework-11-scroll-compressor-build.py [--publish]
Frame of bone `shell`: u ahead (terminal box), v up, w left; the window looks out to -w. Stack along v: sump and lower bearing 0.045..0.07, motor 0.06..0.19, main frame 0.175..0.2445 (thrust face), orbiting scroll plate 0.2445..0.2565, wraps 0.2565..0.2765, fixed scroll plate 0.2765..0.2915, shell top 0.3175.
Bone `crank` stands on the shaft axis from v 0.06 (u of the bone = up, v of the bone = -ahead); bone `orbiter` sits on the eccentric, 6 mm off the axis, and is counter-turned so that it orbits without turning.
Scroll: involute of base radius 3.34 mm, pitch 21 mm, wall 4.5 mm, orbit radius 21/2 - 4.5 = 6 mm, 2.6 turns, wrap height 20 mm.
"""
import math
import sys

from rework_11_lib import *

SLUG = "scroll-compressor"
ST, CU, PA, AL, RU, GL, FE = "steel-shell", "copper", "pa-plastic", "cast-aluminium", "epdm-rubber", "borosilicate-glass", "electrical-steel"
X, Y, Z = [1, 0, 0], [0, 1, 0], [0, 0, 1]
OLD = pristine(SLUG)
A_BASE, WALL, ECC = 0.00334, 0.0045, 0.006
Y0 = 0.2445


def involute(step_deg, phi0=2.5, phi1=19.0):
    """The wrap as a closed polygon (ahead, left) about the scroll centre: outer flank out, inner flank back."""
    n = int(round((phi1 - phi0) / math.radians(step_deg)))
    def point(phi, s):
        return (A_BASE * math.cos(phi) + (A_BASE * phi + s) * math.sin(phi), A_BASE * math.sin(phi) - (A_BASE * phi + s) * math.cos(phi))
    outer = [point(phi0 + (phi1 - phi0) * k / n, WALL / 2) for k in range(n + 1)]
    inner = [point(phi0 + (phi1 - phi0) * k / n, -WALL / 2) for k in range(n, -1, -1)]
    return outer + inner


def scroll_set(step):
    """Fixed and orbiting scroll, Oldham ring and main frame at true size (kept as they are at 1:100)."""
    wrap = involute(step)
    return [
        rev("main-frame", "shell", FE, 0.3, [[0.175, 0.0135], [0.175, 0.03], [0.2, 0.04], [0.232, 0.113], [0.2445, 0.113], [0.2445, 0.03], [0.21, 0.03], [0.21, 0.0135], [0.175, 0.0135]], Y, [0, 0, 0]),
        rev("oldham-ring", "shell", AL, 0.55, [[0.2445, 0.082], [0.2445, 0.09], [0.252, 0.09], [0.252, 0.082], [0.2445, 0.082]], Y, [0, 0, 0], role="detail"),
        rev("fixed-scroll-plate", "shell", FE, 0.35, [[0.2765, 0.006], [0.2765, 0.105], [0.2915, 0.105], [0.2915, 0.006], [0.2765, 0.006]], Y, [0, 0, 0]),
        rev("fixed-scroll-wall", "shell", FE, 0.35, [[0.2445, 0.092], [0.2445, 0.105], [0.2765, 0.105], [0.2765, 0.092], [0.2445, 0.092]], Y, [0, 0, 0]),
        ext("fixed-scroll-wrap", "shell", FE, 0.4, [[z, x] for x, z in wrap], "zx", 0.2565, 0.2765),
        rev("discharge-valve", "shell", ST, 0.3, [[0.2915, 0], [0.2915, 0.014], [0.299, 0.014], [0.299, 0]], Y, [0, 0, 0], role="detail"),
        rev("orbiting-scroll-plate", "orbiter", AL, 0.65, [[0, 0], [0, 0.075], [0.012, 0.075], [0.012, 0]], X, [0, 0, 0]),
        rev("orbiting-scroll-hub", "orbiter", AL, 0.65, [[-0.027, 0.0125], [-0.027, 0.02], [0, 0.02], [0, 0.0125], [-0.027, 0.0125]], X, [0, 0, 0]),
        ext("orbiting-scroll-wrap", "orbiter", AL, 0.7, [[x, -z] for x, z in wrap][::-1], "yz", 0.012, 0.032),
    ]


def drive(scale):
    """Stator, windings, rotor, crankshaft with eccentric and counterweights, lower bearing."""
    weight = 0.0075 if scale == "50" else 0.015
    return [
        rev("stator", "shell", FE, 0.25, [[0.085, 0.058], [0.085, 0.113 if scale == "50" else 0.1055], [0.165, 0.113 if scale == "50" else 0.1055], [0.165, 0.058], [0.085, 0.058]], Y, [0, 0, 0]),
        rev("winding-lower", "shell", CU, 0.6, [[0.06, 0.06], [0.06, 0.095], [0.085, 0.095], [0.085, 0.06], [0.06, 0.06]], Y, [0, 0, 0]),
        rev("winding-upper", "shell", CU, 0.6, [[0.165, 0.06], [0.165, 0.095], [0.19, 0.095], [0.19, 0.06], [0.165, 0.06]], Y, [0, 0, 0]),
        rev("lower-bearing", "shell", FE, 0.3, [[0.045, 0.0135], [0.045, 0.028], [0.07, 0.028], [0.07, 0.0135], [0.045, 0.0135]], Y, [0, 0, 0]),
        box("lower-bearing-bridge", "shell", ST, 0.3, [0.215, 0.015, 0.03], [0, 0.0525, 0], repeat=rep(2, rotation=1.5708, axis=Y, origin=[0, 0, 0])),
        rev("rotor", "crank", FE, 0.45, [[0.025, 0.0125], [0.025, 0.054], [0.105, 0.054], [0.105, 0.0125], [0.025, 0.0125]], X, [0, 0, 0]),
        rev("crankshaft", "crank", ST, 0.5, [[-0.02, 0], [-0.02, 0.0125], [0.15, 0.0125], [0.15, 0]], X, [0, 0, 0]),
        rev("eccentric-pin", "crank", ST, 0.5, [[0.15, 0], [0.15, 0.0115], [0.182, 0.0115], [0.182, 0]], X, [0, 0, ECC]),
        box("counterweight-upper", "crank", ST, 0.35, [weight + 0.0025, 0.07, 0.03], [0.105 + (weight + 0.0025) / 2, 0, -0.03], role="detail", corner=0.003),
        box("counterweight-lower", "crank", ST, 0.35, [weight + 0.0025, 0.06, 0.025], [0.025 - (weight + 0.0025) / 2, 0, 0.03], role="detail", corner=0.003),
    ]


def hollow_shell(t, band):
    """The welded shell as a hollow vessel of wall `t`; with `band` the wall is opened behind the window (56 x 56 mm)."""
    inner = 0.1205 - t
    if not band:
        return [rev("shell", "shell", ST, 0.35, [[0.0295, 0], [0.0295, 0.1205], [0.3175, 0.1205], [0.3175, inner], [0.0295 + t, inner], [0.0295 + t, 0]], Y, [0, 0, 0])]
    gap = 2 * math.asin(0.028 / (inner + t / 2))
    return [
        rev("shell", "shell", ST, 0.35, [[0.0295, 0], [0.0295, 0.1185], [0.0315, 0.1205], [0.232, 0.1205], [0.232, inner], [0.0295 + t, inner], [0.0295 + t, 0]], Y, [0, 0, 0]),
        rev("shell-window-band", "shell", ST, 0.35, [[0.232, inner], [0.232, 0.1205], [0.288, 0.1205], [0.288, inner], [0.232, inner]], Y, [0, 0, 0], sweepRad=r(2 * math.pi - gap), transform={"rotation": quat(Y, BAND_TURN + math.degrees(gap) / 2)}),
        rev("shell-top-ring", "shell", ST, 0.35, [[0.288, inner], [0.288, 0.1205], [0.3175, 0.1205], [0.3175, inner], [0.288, inner]], Y, [0, 0, 0]),
    ]


BAND_TURN = 90
DOME_OUTER = {
    "50": None,
    "100": [[0, 0.1245], [0.02, 0.1245], [0.033, 0.122], [0.04414, 0.11485], [0.05182, 0.10407], [0.06434, 0.07059], [0.07196, 0.03566], [0.07451, 0]],
    "200": [[0, 0.1245], [0.02, 0.1245], [0.03891, 0.11895], [0.05182, 0.10407], [0.06878, 0.05326], [0.07451, 0]],
    "500": [[0, 0.1245], [0.018, 0.1245], [0.032, 0.1205], [0.044, 0.1115], [0.055, 0.096], [0.064, 0.072], [0.071, 0.04], [0.0745, 0]],
}
DOME_INNER = {
    "50": [[0.067, 0], [0.0645, 0.0345], [0.0575, 0.067], [0.046, 0.098], [0.0395, 0.108], [0.03, 0.1145], [0.02, 0.117], [0, 0.117]],
    "100": [[0.0595, 0], [0.057, 0.034], [0.05, 0.065], [0.039, 0.094], [0.03, 0.104], [0.02, 0.1095], [0, 0.1095]],
    "200": [[0.0445, 0], [0.04, 0.045], [0.028, 0.082], [0.02, 0.0945], [0, 0.0945]],
}


def dome(scale):
    """The torispherical top cap as a hollow shell (the discharge plenum)."""
    outer = DOME_OUTER[scale] or find(OLD["volumes"]["50"]["body"], "top-cap")["profile"][1:]
    if scale == "500":
        return [rev("top-cap", "dome", ST, 0.35, [[0, 0]] + outer, Y, [0, 0, 0])]
    return [rev("top-cap", "dome", ST, 0.35, outer + DOME_INNER[scale] + [outer[0]], Y, [0, 0, 0])]


def body50():
    b = hollow_shell(0.0075, True) + dome("50")
    b += kept(OLD, "50", "body", "discharge-boss", "discharge-tube", "suction-boss", "suction-tube", "terminal-cover", "terminal-gland", "window-boss", "window-bezel", "sight-glass", "grommets", "base-plate", "mount-sleeves", "mount-bolts")
    return b + scroll_set(20) + drive("50")


def body100():
    b = hollow_shell(0.015, True) + dome("100")
    b += kept(OLD, "100", "body", "discharge-stub", "suction-stub", "terminal-cover", "window-ring", "grommets", "base-plate", "mount-sleeves", "mount-bolts")
    b += kept(OLD, "50", "body", "sight-glass")
    return b + scroll_set(30) + drive("100")


def body200():
    b = hollow_shell(0.03, False) + dome("200")
    b += kept(OLD, "200", "body", "discharge-stub", "suction-stub", "terminal", "grommets", "base-plate", "mount-sleeves", "mount-bolts")
    b += [
        rev("window", "shell", ST, 0.35, [[0.1, 0.04], [0.1, 0.07], [0.1455, 0.07], [0.1455, 0.04], [0.1, 0.04]], [0, 0, -1], [0, 0.26, 0]),
        rev("sight-glass", "shell", GL, 0.85, [[0.1425, 0], [0.1425, 0.04], [0.1455, 0.04], [0.1455, 0]], [0, 0, -1], [0, 0.26, 0], role="glazing"),
        rev("stator", "shell", FE, 0.25, [[0.07, 0.058], [0.07, 0.0905], [0.18, 0.0905], [0.18, 0.058], [0.07, 0.058]], Y, [0, 0, 0]),
        rev("main-frame", "shell", FE, 0.3, [[0.2145, 0.03], [0.2145, 0.0905], [0.2445, 0.0905], [0.2445, 0.03], [0.2145, 0.03]], Y, [0, 0, 0]),
        rev("fixed-scroll-plate", "shell", FE, 0.35, [[0.2795, 0], [0.2795, 0.0905], [0.3095, 0.0905], [0.3095, 0]], Y, [0, 0, 0]),
        rev("rotor", "crank", FE, 0.45, [[-0.0005, 0], [-0.0005, 0.015], [0.025, 0.015], [0.025, 0.054], [0.105, 0.054], [0.105, 0.015], [0.182, 0.015], [0.182, 0]], X, [0, 0, 0]),
        rev("orbiting-scroll", "orbiter", AL, 0.65, [[0, 0], [0, 0.075], [0.035, 0.075], [0.035, 0]], X, [0, 0, 0]),
    ]
    return b


def body500():
    full = {x["id"]: x for x in scroll_set(30) + drive("50")}
    b = hollow_shell(0.0075, False) + dome("500")
    b += kept(OLD, "500", "body", "discharge-stub", "suction-stub", "terminal", "grommets", "base-plate", "mount-sleeves", "mount-bolts")
    b += kept(OLD, "50", "body", "sight-glass")
    b += [full[id] for id in ("main-frame", "fixed-scroll-plate", "orbiting-scroll-plate", "stator", "rotor")]
    return b


REGION = {
    "shell": "shell", "shell-window-band": "shell", "shell-top-ring": "shell", "window-boss": "shell", "window-bezel": "shell", "window-ring": "shell", "window": "shell", "top-cap": "dome",
    "discharge-boss": "discharge", "discharge-tube": "discharge", "discharge-stub": "discharge", "suction-boss": "suction", "suction-tube": "suction", "suction-stub": "suction",
    "terminal-cover": "terminal", "terminal-gland": "terminal", "terminal": "terminal", "grommets": "feet", "base-plate": "feet", "mount-sleeves": "feet", "mount-bolts": "feet", "sight-glass": "sight-glass",
    "main-frame": "scroll", "oldham-ring": "scroll", "fixed-scroll-plate": "scroll", "fixed-scroll-wall": "scroll", "fixed-scroll-wrap": "scroll", "discharge-valve": "scroll",
    "orbiting-scroll-plate": "scroll", "orbiting-scroll-hub": "scroll", "orbiting-scroll-wrap": "scroll", "orbiting-scroll": "scroll",
    "stator": "motor", "winding-lower": "motor", "winding-upper": "motor", "lower-bearing": "motor", "lower-bearing-bridge": "motor", "rotor": "motor", "crankshaft": "motor", "eccentric-pin": "motor",
    "counterweight-upper": "motor", "counterweight-lower": "motor",
}
HIDDEN = ("motor", "scroll")


def thermal(scale, body):
    out = []
    for s in body:
        region = REGION[s["id"]]
        if region in HIDDEN:
            own = dict(s)
            own["id"] = "th-" + s["id"]
            out.append(th(None, region, own=own))
        else:
            out.append(th(s["id"], region))
    out += kept(OLD, scale, "thermal", "th-dome-plume-outlet")
    return out


def structure(scale, body):
    ids = {s["id"] for s in body}
    f = FEATURE[scale]
    out = [st("shell-vessel", "shell", "member", of="shell")]
    for id, name in (("shell-window-band", "shell-band"), ("shell-top-ring", "shell-ring"), ("main-frame", "bearing-frame"), ("fixed-scroll-plate", "fixed-scroll"), ("fixed-scroll-wall", "fixed-scroll-skirt"), ("lower-bearing-bridge", "lower-bridge")):
        if id in ids:
            out.append(st(name, "shell", "member", of=id))
    if "lower-bearing" in ids:
        out.append(st("joint-lower-bearing", "shell", "joint", of="lower-bearing"))
    if "stator" in ids:
        out.append(st("mass-motor", "shell", "mass", of="stator"))
    else:
        out.append(st("mass-motor", "shell", "mass", own=ell("", "shell", FE, 0.2, [0.0375, 0.0375, 0.0375], [0, 0.067, 0])))
    out += [
        st("top-cap-vessel", "dome", "member", of="top-cap"),
        st("joint-lap-weld", "dome", "joint", own=rev("", "dome", ST, 0.3, torus(0, 0.1245, max(0.004, f / 2), 6), Y, [0, 0, 0])),
        st("terminal-fence", "terminal", "member", of="terminal-cover" if "terminal-cover" in ids else "terminal"),
        st("crank-rotor", "crank", "member", of="rotor"),
    ]
    if "crankshaft" in ids:
        out += [st("crank-shaft", "crank", "member", of="crankshaft"), st("joint-eccentric", "crank", "joint", of="eccentric-pin")]
    else:
        out.append(st("joint-main-bearing", "crank", "joint", own=rev("", "crank", ST, 0.3, torus(0.13, max(0.02, f / 2 + 0.008), max(0.006, f / 2), 6), X, [0, 0, 0])))
    if "orbiting-scroll-plate" in ids:
        out.append(st("orbiting-scroll", "orbiter", "member", of="orbiting-scroll-plate"))
        if "orbiting-scroll-hub" in ids:
            out += [st("orbiting-wrap", "orbiter", "member", of="orbiting-scroll-wrap"), st("joint-drive-bearing", "orbiter", "joint", of="orbiting-scroll-hub")]
    else:
        out.append(st("orbiting-scroll-member", "orbiter", "member", of="orbiting-scroll"))
        out.append(st("joint-drive-bearing", "orbiter", "joint", own=rev("", "orbiter", AL, 0.3, torus(-f / 2 if scale == "200" else 0.02, max(0.02, f / 2 + 0.005), max(0.006, f / 2), 6), X, [0, 0, 0])))
    out += [
        st("contact-grommets", "shell", "support", of="grommets", anchor="base-contact"),
        st("base-plate-member", "shell", "member", of="base-plate"),
        st("mount-sleeve-joints", "shell", "joint", of="mount-sleeves"),
        st("mount-bolt-joints", "shell", "joint", of="mount-bolts"),
    ]
    return out


DECISIONS = {
    "50": [
        text("The welded shell is a hollow vessel of 241 mm with a 7.5 mm wall and a real 56 mm opening behind the bolted sight glass; the torispherical top cap is hollow too and forms the discharge plenum. Stubs, terminal box, window boss and bezel, base plate, grommets, sleeves and bolts stay as modelled.", "Das geschweißte Gehäuse ist ein hohles Gefäß von 241 mm mit 7,5 mm Wand und einer echten 56-mm-Öffnung hinter dem verschraubten Schauglas; die Korbbogenkappe ist ebenfalls hohl und bildet den Druckraum. Stutzen, Klemmenkasten, Fensterbund und Rahmen, Grundplatte, Tüllen, Hülsen und Schrauben bleiben wie modelliert."),
        text("The scroll set lies horizontal under the cap as in the real machine: the fixed scroll (plate with discharge port and reed valve, skirt, involute wrap) bolted on the main frame, the orbiting scroll (plate, drive hub, wrap turned by 180 degrees) on the thrust face inside the Oldham ring. Both wraps are the same involute of 3.34 mm base radius, 21 mm pitch, 4.5 mm wall and 20 mm height; the orbit radius of 6 mm is half the pitch minus the wall, so the flanks touch and the crescent pockets close.", "Der Spiralsatz liegt wie in der echten Maschine waagrecht unter der Kappe: die feste Spirale (Platte mit Auslassbohrung und Zungenventil, Mantel, Evolventenwand) auf dem Hauptlagerschild verschraubt, die orbitierende Spirale (Platte, Antriebsnabe, um 180 Grad gedrehte Wand) auf der Anlauffläche im Oldham-Ring. Beide Wände sind dieselbe Evolvente mit 3,34 mm Grundkreisradius, 21 mm Teilung, 4,5 mm Wand und 20 mm Höhe; der Orbitradius von 6 mm ist die halbe Teilung minus die Wand, sodass die Flanken anliegen und die Sichelkammern schließen."),
        text("The drive is complete: laminated stator pressed into the shell with its two copper end windings, rotor on the 25 mm crankshaft, eccentric pin 6 mm off the axis in the drive hub, two counterweights, main bearing in the frame and lower bearing on a cross bridge above the oil sump. The crankshaft turns on the bone crank; the orbiting scroll rides on the bone orbiter on the eccentric and is turned back by the same angle, so it orbits without rotating as the Oldham coupling enforces. The Oldham ring itself is drawn at rest.", "Der Antrieb ist vollständig: geblechter Stator im Gehäuse eingepresst mit zwei Kupfer-Wickelköpfen, Läufer auf der 25-mm-Kurbelwelle, Exzenterzapfen 6 mm außermittig in der Antriebsnabe, zwei Ausgleichsgewichte, Hauptlager im Lagerschild und unteres Lager auf einer Kreuzbrücke über dem Ölsumpf. Die Kurbelwelle dreht auf dem Knochen crank; die orbitierende Spirale sitzt auf dem Knochen orbiter am Exzenter und wird um denselben Winkel zurückgedreht, sodass sie orbitiert ohne zu rotieren, wie es die Oldham-Kupplung erzwingt. Der Oldham-Ring selbst ist in Ruhe dargestellt."),
        text("Thermal: shell, dome, stubs, terminal and feet as before; the glass is its own glazing region; the hidden regions scroll (scroll set and frame, the hottest metal) and motor (stator, windings, rotor, shaft) are solids of their own inside. Structure: the shell carries stator, frame and fixed scroll; the crankshaft hangs in the two bearings, the orbiting scroll on the eccentric and the thrust face; four grommets carry the base plate.", "Thermisch: Gehäuse, Kappe, Stutzen, Klemmenkasten und Füße wie zuvor; das Glas ist eine eigene Verglasungsregion; die verdeckten Regionen scroll (Spiralsatz und Lagerschild, das heißeste Metall) und motor (Stator, Wicklungen, Läufer, Welle) sind eigene Körper im Inneren. Tragwerk: Das Gehäuse trägt Stator, Lagerschild und feste Spirale; die Kurbelwelle hängt in den zwei Lagern, die orbitierende Spirale am Exzenter und auf der Anlauffläche; vier Tüllen tragen die Grundplatte."),
    ],
    "100": [
        text("Modelled for 15 mm: shell and cap hollow with a 15 mm wall and the window opening, glass 15 mm, stator, windings, rotor, crankshaft, eccentric, 17.5 mm counterweights, bearings and bridge.", "Für 15 mm modelliert: Gehäuse und Kappe hohl mit 15 mm Wand und der Fensteröffnung, Glas 15 mm, Stator, Wicklungen, Läufer, Kurbelwelle, Exzenter, 17,5-mm-Ausgleichsgewichte, Lager und Brücke."),
        text("The scroll set keeps the exact dimensions of 1:50 (plates, skirt, hub, the two involute wraps with 30 degree steps), because the meshing of the wraps is the subject; orbit and counter-turn as at 1:50.", "Der Spiralsatz behält die genauen Maße von 1:50 (Platten, Mantel, Nabe, die zwei Evolventenwände in 30-Grad-Schritten), weil das Ineinandergreifen der Wände das Thema ist; Orbit und Rückdrehung wie bei 1:50."),
        text("Thermal regions and structure as at 1:50.", "Thermische Regionen und Tragwerk wie bei 1:50."),
    ],
    "200": [
        text("Modelled for 30 mm: hollow shell and cap with a 30 mm wall, the window as a closed boss, stator ring, rotor with a 30 mm shaft on the crank bone, main frame and fixed scroll as 30 mm discs, the orbiting scroll as one 150 mm disc on the orbiter bone.", "Für 30 mm modelliert: hohles Gehäuse und Kappe mit 30 mm Wand, das Fenster als geschlossener Bund, Statorring, Läufer mit 30-mm-Welle auf dem Knochen crank, Lagerschild und feste Spirale als 30-mm-Scheiben, die orbitierende Spirale als eine 150-mm-Scheibe auf dem Knochen orbiter."),
        text("Thermal regions and structure follow 1:50 with the parts that remain.", "Thermische Regionen und Tragwerk folgen 1:50 mit den verbleibenden Teilen."),
    ],
    "500": [
        text("The whole compressor is 0.86 mm tall at 1:500 and no drawing exists; a 75 mm wall would leave no room for the machine, so the shell, the glass and the five largest inner parts (stator, rotor on the crank bone, main frame, fixed scroll plate, orbiting scroll plate on the orbiter bone) keep their exact 1:50 dimensions under a solid cap; stubs, terminal and feet as before.", "Der ganze Verdichter ist bei 1:500 0,86 mm hoch und es gibt keine Zeichnung; eine 75-mm-Wand ließe der Maschine keinen Raum, deshalb behalten Gehäuse, Glas und die fünf größten Innenteile (Stator, Läufer auf dem Knochen crank, Lagerschild, feste Spiralplatte, orbitierende Spiralplatte auf dem Knochen orbiter) ihre genauen 1:50-Maße unter einer massiven Kappe; Stutzen, Klemmenkasten und Füße wie zuvor."),
        text("Thermal regions and structure follow 1:50 with the parts that remain.", "Thermische Regionen und Tragwerk folgen 1:50 mit den verbleibenden Teilen."),
    ],
}

AREA = {"scroll": 0.363, "motor": 0.389, "sight-glass": 0.005, "suction": 0.0075, "terminal": 0.022}
MATERIAL = {"id": FE, "title": text("Cast iron and electrical steel", "Gusseisen und Elektroblech"), "emissivity": 0.6, "solarAbsorptivity": 0.7, "densityKgM3": 7500, "conductivityWmK": 45, "specificHeatJKgK": 470, "source": "literature", "reference": "incropera-2018"}


def hidden_region(id, en, de, material, key_heat, why_en, why_de):
    return {
        "id": id, "title": text(en, de), "kind": "engine", "model": "operating", "material": material, "areaM2": AREA[id], "characteristicLengthM": 0.15, "geometry": "cylinder", "skyViewFactor": 0,
        "layers": [{"id": id + "-skin", "title": text(en, de), "kind": "metal", "material": material, "thicknessM": 0.01, "resistanceM2KW": 0.0002, "heatCapacityJM2K": 35000, "source": "authored-estimate", "reference": "incropera-2018"}],
        "heatCapacityJM2K": 35000, "operating": {"key": "run", "heatW": key_heat}, "source": "authored-estimate", "reference": "incropera-2018", "hidden": text(why_en, why_de),
    }


def root(asset):
    old = OLD
    bones = [b for b in old["rig"]["bones"] if b["id"] != "rotor"]
    bones = json_copy(bones)
    find(bones, "shell")["mass"]["kg"] = 19.3
    bones += [
        {"id": "crank", "parent": "shell", "title": text("Crankshaft with motor rotor, eccentric pin and counterweights", "Kurbelwelle mit Läufer, Exzenterzapfen und Ausgleichsgewichten"), "x": 0, "y": 0.06, "angleDeg": 90, "lengthM": 0.1845,
         "joint": {"type": "revolute", "continuousTwist": True}, "mass": {"kg": 3.2, "comX": 0.07, "comY": 0, "distribution": "point", "source": "authored-estimate"}},
        {"id": "orbiter", "parent": "crank", "title": text("Orbiting scroll on the eccentric, held from turning by the Oldham ring", "Orbitierende Spirale am Exzenter, vom Oldham-Ring am Drehen gehindert"), "x": 0.1845, "y": 0, "z": ECC, "angleDeg": 0, "lengthM": 0.032,
         "joint": {"type": "revolute", "continuousTwist": True}, "mass": {"kg": 0.9, "comX": 0.01, "comY": 0, "distribution": "point", "source": "authored-estimate"}},
    ]
    asset["rig"]["bones"] = bones
    turn = {"off": 0, "part-load": 40, "full-load": 80}
    for pose in asset["poses"]:
        pose["bones"] = {"crank": {"twistDeg": turn[pose["id"]]}, "orbiter": {"twistDeg": -turn[pose["id"]]}}
        regions = pose["thermal"]["regions"]
        regions["motor"] = {"sunlitFraction": 0, "exposedFraction": 1}
        regions["sight-glass"] = {"sunlitFraction": 0, "exposedFraction": 1}
    for clip in asset["clips"]:
        for key in clip["keys"]:
            if "rotor" in key.get("bones", {}):
                angle = key["bones"].pop("rotor")["angleDeg"]
                key["bones"].update({"crank": {"twistDeg": angle}, "orbiter": {"twistDeg": -angle}})
    spin = find(asset["clips"], "spin")
    spin["keys"] = [{"t": 0.25 * k, "pose": "full-load", "ease": "linear", "bones": {"crank": {"twistDeg": 90 * k}, "orbiter": {"twistDeg": -90 * k}}} for k in range(13)]
    spin["title"] = text("Orbiting scroll", "Orbitierende Spirale")
    spin["description"] = text("The crankshaft turns three times in three seconds and carries the orbiting scroll round its 6 mm circle without turning it; in the window the orbiting wraps swing between the fixed ones (the real shaft turns 50 times per second, shown slowed).", "Die Kurbelwelle dreht sich in drei Sekunden dreimal und führt die orbitierende Spirale auf ihrem 6-mm-Kreis, ohne sie zu drehen; im Fenster pendeln die orbitierenden Wände zwischen den festen (die reale Welle dreht 50 Mal je Sekunde, verlangsamt gezeigt).")
    asset["materials"] = [m for m in asset["materials"] if m["id"] != FE] + [MATERIAL]
    regions = [x for x in json_copy(old["thermal"]["regions"]) if x["id"] != "scroll"]
    glass = json_copy(find(old["thermal"]["regions"], "terminal"))
    glass.update({"id": "sight-glass", "title": text("Sight glass", "Schauglas"), "kind": "glazing", "material": GL, "areaM2": AREA["sight-glass"], "characteristicLengthM": 0.08})
    glass["layers"][0].update({"id": "glass-pane", "title": text("Borosilicate glass 10 mm", "Borosilikatglas 10 mm"), "material": GL, "thicknessM": 0.01, "kind": glass["layers"][0]["kind"]})
    regions += [
        glass,
        hidden_region("scroll", "Scroll set and main frame", "Spiralsatz und Lagerschild", FE, 600 * AREA["scroll"], "The fixed and the orbiting scroll with the main frame under the top cap, where the gas is compressed and reaches 80 to 110 C; the drawings show only their edges through the sight glass, the 3D thermal model shows them when cut.", "Die feste und die orbitierende Spirale mit dem Lagerschild unter der Kappe, wo das Gas verdichtet wird und 80 bis 110 C erreicht; die Zeichnungen zeigen nur ihre Kanten durch das Schauglas, das thermische 3D-Modell zeigt sie im Schnitt."),
        hidden_region("motor", "Motor: stator, windings, rotor and crankshaft", "Motor: Stator, Wicklungen, Läufer und Kurbelwelle", FE, 400 * AREA["motor"], "The suction-gas-cooled motor inside the shell: stator, copper end windings, rotor and crankshaft, at 50 to 80 C under load; no drawing shows it.", "Der sauggasgekühlte Motor im Gehäuse: Stator, Kupfer-Wickelköpfe, Läufer und Kurbelwelle, unter Last bei 50 bis 80 C; keine Zeichnung zeigt ihn."),
    ]
    find(regions, "suction")["areaM2"] = AREA["suction"]
    find(regions, "terminal")["areaM2"] = AREA["terminal"]
    asset["thermal"]["regions"] = regions
    for scale in ("50", "100", "200"):
        rep_old = old["representations"][scale]
        geometry = json_copy(rep_old["geometry"])
        elevation = [s for s in geometry["elevation"] if s["bone"] != "rotor"]
        common = {"bone": "shell", "filled": True, "material": FE}
        if scale == "200":
            find(elevation, "sight-ring")["region"] = "shell"
            for view in ("front", "plan"):
                find(geometry[view], "sight-ring")["region"] = "shell"
            elevation += [
                dict(common, id="sight-glass", role="body", z=3.65, tone=0.85, region="sight-glass", material=GL, kind="ellipse", cx=0, cy=0.26, rx=0.04, ry=0.04),
                dict(common, id="scroll-set", role="body", z=3.7, tone=0.4, region="scroll", kind="rect", x=-0.028, y=0.232, w=0.056, h=0.056),
            ]
        else:
            elevation += [
                dict(common, id="sight-glass", role="glazing" if scale == "50" else "body", z=3.65, tone=0.85, region="sight-glass", material=GL, kind="ellipse", cx=0, cy=0.26, rx=0.0399, ry=0.0399),
                dict(common, id="window-opening", role="detail", z=3.7, tone=0.12, region="scroll", kind="rect", x=-0.028, y=0.232, w=0.056, h=0.056),
            ]
            if scale == "50":
              elevation += [
                dict(common, id="main-frame-face", role="detail", z=3.72, tone=0.3, region="scroll", kind="rect", x=-0.028, y=0.232, w=0.056, h=0.0125),
                dict(common, id="orbiting-plate-edge", role="detail", z=3.72, tone=0.65, region="scroll", material=AL, kind="rect", x=-0.028, y=0.2445, w=0.056, h=0.012),
                dict(common, id="fixed-plate-edge", role="detail", z=3.72, tone=0.35, region="scroll", kind="rect", x=-0.028, y=0.2765, w=0.056, h=0.0115),
            ]
            if scale == "50":
                elevation += [
                    dict(common, id="fixed-wraps", role="detail", z=3.74, filled=False, tone=0.2, region="scroll", kind="path", d="M -0.021 0.2565 L -0.021 0.2765 M 0 0.2565 L 0 0.2765 M 0.021 0.2565 L 0.021 0.2765", penMm=0.13),
                    dict(common, id="orbiting-wraps", bone="orbiter", role="detail", z=3.76, filled=False, tone=0.75, region="scroll", material=AL, kind="path", d="M 0.012 0.0105 L 0.032 0.0105 M 0.012 -0.0105 L 0.032 -0.0105", penMm=0.18),
                ]
            else:
                elevation.append(dict(common, id="orbiting-wraps", bone="orbiter", role="detail", z=3.76, tone=0.75, region="scroll", material=AL, kind="rect", x=0.012, y=-0.015, w=0.02, h=0.03))
        geometry["elevation"] = elevation
        asset["representations"][scale]["geometry"] = geometry


def json_copy(value):
    import json
    return json.loads(json.dumps(value))


def main():
    document, before = load_catalog(SLUG)
    asset = document["asset"]
    root(asset)
    volumes = {}
    for scale, make in (("50", body50), ("100", body100), ("200", body200), ("500", body500)):
        body = make()
        volumes[scale] = {"status": "authored", "denominator": int(scale), "body": body, "thermal": thermal(scale, body), "structure": structure(scale, body), "decisions": DECISIONS[scale]}
    save(SLUG, document, volumes, before, "--publish" in sys.argv)


main()
