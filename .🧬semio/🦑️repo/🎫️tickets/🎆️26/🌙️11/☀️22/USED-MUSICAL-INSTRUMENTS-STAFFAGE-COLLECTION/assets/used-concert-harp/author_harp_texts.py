"""📜️ Texts, physics and metadata of the used concert harp: materials, references, bone titles, masses and sections, anchors,
poses, clips, the thermal and structural model, the hand-drawn traces of use and the decisions of every scale.

@see author-harp.py
"""

import math

from author_harp_geometry import (
    BASE_X0, BASE_X1, BASE_Y1, BASE_Z, CASTOR_X, CASTOR_Z, COL_X, DECK_Y0, FEET_X, FEET_Z, PEDAL_NOTCH_DEG, PEDALS, SB_LEN,
    T_DECK, TILT_DEG, TROLLEY_X0, WHEEL_R, r6, sb_dp, sb_hw, sb_point, strings,
)

TENSION_N = 8000.0
ROLL_M = 1.2
TROLLEY_PARK = 0.95
G = 9.80665
NATURAL = r6(PEDAL_NOTCH_DEG)
SHARP = r6(2 * PEDAL_NOTCH_DEG)


def t(en, de):
    """📝️ One bilingual text."""
    return {"en": en, "de": de}


def dez(x):
    """🇩🇪️ A number with a decimal comma."""
    return str(x).replace(".", ",")


MATERIALS = [
    {"id": "spruce-varnished", "title": t("Spruce soundboard, aged spirit varnish (yellowed)", "Fichtendecke, gealterter Spirituslack (vergilbt)"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 450, "conductivityWmK": 0.12, "specificHeatJKgK": 1600, "elasticModulusPa": 10.0e9, "strengthPa": 65e6, "source": "literature", "reference": "wood-handbook"},
    {"id": "maple-lacquer", "title": t("Maple and beech laminate, nitro lacquer worn at the contact points", "Ahorn- und Buchenschichtholz, Nitrolack, an den Griffstellen abgegriffen"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 650, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "elasticModulusPa": 12.6e9, "strengthPa": 100e6, "source": "literature", "reference": "wood-handbook"},
    {"id": "gold-leaf-worn", "title": t("Gold leaf on gesso over carved maple, rubbed through to the red bole", "Blattgold auf Kreidegrund über geschnitztem Ahorn, bis auf den roten Poliment durchgerieben"), "emissivity": 0.35, "solarAbsorptivity": 0.45, "densityKgM3": 650, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "source": "authored-estimate", "reference": "ashrae-2021"},
    {"id": "brass-tarnished", "title": t("Brass action plates, tarnished with dark patches", "Messing-Mechanikplatten, angelaufen mit dunklen Flecken"), "emissivity": 0.3, "solarAbsorptivity": 0.6, "densityKgM3": 8500, "conductivityWmK": 110, "specificHeatJKgK": 380, "elasticModulusPa": 100e9, "strengthPa": 300e6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "steel-nickel", "title": t("Nickel-plated steel pedal bars, plating worn bright at the toes", "Vernickelte Stahlpedale, Vernickelung an den Spitzen blank getreten"), "emissivity": 0.2, "solarAbsorptivity": 0.45, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 200e9, "strengthPa": 350e6, "source": "literature", "reference": "ashrae-2021"},
    {"id": "pedal-rubber", "title": t("Rubber pedal caps, worn smooth", "Gummikappen der Pedale, glattgetreten"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 1200, "conductivityWmK": 0.16, "specificHeatJKgK": 1500, "source": "literature", "reference": "en-iso-10456"},
    {"id": "rubber-black", "title": t("Black rubber feet", "Schwarze Gummifüße"), "emissivity": 0.92, "solarAbsorptivity": 0.9, "densityKgM3": 1200, "conductivityWmK": 0.16, "specificHeatJKgK": 1500, "source": "literature", "reference": "en-iso-10456"},
    {"id": "wound-string", "title": t("Copper-wound bass strings on a silk and steel core", "Kupferumsponnene Basssaiten auf Seiden- und Stahlkern"), "emissivity": 0.4, "solarAbsorptivity": 0.6, "densityKgM3": 6500, "conductivityWmK": 50, "specificHeatJKgK": 420, "elasticModulusPa": 60e9, "source": "authored-estimate", "reference": "rossing-2010"},
    {"id": "gut-string", "title": t("Gut strings (C red, F black), slightly frayed", "Darmsaiten (C rot, F schwarz), leicht ausgefranst"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 1300, "conductivityWmK": 0.2, "specificHeatJKgK": 1500, "elasticModulusPa": 5e9, "source": "literature", "reference": "rossing-2010"},
    {"id": "nylon-string", "title": t("Nylon treble strings", "Nylon-Diskantsaiten"), "emissivity": 0.9, "solarAbsorptivity": 0.4, "densityKgM3": 1140, "conductivityWmK": 0.25, "specificHeatJKgK": 1700, "elasticModulusPa": 4e9, "source": "literature", "reference": "rossing-2010"},
    {"id": "padded-nylon", "title": t("Padded transport cover: nylon shell, 12 mm foam, quilted, faded and taped", "Gepolsterte Transporthülle: Nylonhülle, 12 mm Schaum, gesteppt, ausgeblichen und geflickt"), "emissivity": 0.88, "solarAbsorptivity": 0.75, "densityKgM3": 60, "conductivityWmK": 0.04, "specificHeatJKgK": 1400, "source": "authored-estimate", "reference": "en-iso-10456"},
    {"id": "plywood-birch", "title": t("Birch plywood 18 mm, edges scuffed", "Birkensperrholz 18 mm, Kanten bestoßen"), "emissivity": 0.9, "solarAbsorptivity": 0.5, "densityKgM3": 680, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 9e9, "strengthPa": 40e6, "source": "literature", "reference": "en-iso-10456"},
    {"id": "carpet-felt", "title": t("Needle-felt carpet on the deck, worn under the feet", "Nadelfilz auf dem Deck, unter den Füßen abgetreten"), "emissivity": 0.9, "solarAbsorptivity": 0.8, "densityKgM3": 200, "conductivityWmK": 0.06, "specificHeatJKgK": 1300, "source": "literature", "reference": "en-iso-10456"},
    {"id": "pine-batten", "title": t("Pine rim battens 20 x 20 mm", "Kiefernleisten 20 x 20 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.55, "densityKgM3": 450, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11e9, "source": "literature", "reference": "wood-handbook"},
    {"id": "polyurethane-grey", "title": t("Grey polyurethane castor wheels 50 mm", "Graue Polyurethan-Lenkrollen 50 mm"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": 1200, "conductivityWmK": 0.25, "specificHeatJKgK": 1800, "source": "literature", "reference": "en-12530"},
    {"id": "steel-zinc", "title": t("Zinc-plated pressed-steel castor forks", "Verzinkte Lenkrollengabeln aus Stahlblech"), "emissivity": 0.25, "solarAbsorptivity": 0.6, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210e9, "source": "literature", "reference": "ashrae-2021"},
]

REFERENCES = [
    {"id": "hornbostel-sachs", "citation": "E. M. von Hornbostel, C. Sachs: Systematik der Musikinstrumente, Zeitschrift für Ethnologie 46 (1914) 553-590; revision by the MIMO Consortium (2011): class 322.222, frame harp with pedal tuning action."},
    {"id": "rensch-2007", "citation": "R. Rensch: Harps and Harpists, revised edition, Indiana University Press, Bloomington 2007 (the double-action pedal harp after Erard 1810: seven pedals in three notches, flat, natural, sharp; column with the pedal rods, neck with the disc action, soundbox and base)."},
    {"id": "rossing-2010", "citation": "T. D. Rossing (ed.): The Science of String Instruments, Springer, New York 2010, chapter on the harp (concert harp of 47 strings C1 to G7, gut, nylon and wound strings, total string tension of the order of 8 to 10 kN, soundboard and soundbox)."},
    {"id": "concert-harp-catalogues", "citation": "Catalogue data of concert grand pedal harps of the class (several makers, no single product): height 1.83 to 1.88 m, 47 strings, 7 pedals, mass 36 to 40 kg, soundboard about 0.45 m wide at the bass end, base about 0.6 x 0.43 m (authored-estimate)."},
    {"id": "wood-handbook", "citation": "Forest Products Laboratory: Wood Handbook, Wood as an Engineering Material, General Technical Report FPL-GTR-282, Madison 2021 (densities, moduli and conductivities of spruce, maple, birch and pine)."},
    {"id": "ashrae-2021", "citation": "ASHRAE Handbook Fundamentals (2021), chapter 4, Table 5 emissivities and solar absorptances (oxidised brass about 0.3 against 0.03 to 0.05 polished, nickel plating 0.1 to 0.2, gold leaf 0.02 to 0.05 polished; worn gilding with the bole showing is taken as 0.35, lacquered wood 0.9)."},
    {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (design conductivity and heat capacity of wood-based panels, rubber, foams and textiles)."},
    {"id": "en-12530", "citation": "EN 12530:1999 Castors and wheels, castors and wheels for manually propelled general purpose applications (50 mm swivel castors, rolling resistance)."},
    {"id": "din-1356-1", "citation": "DIN 1356-1:1995 Building drawings, types, contents and basic principles of presentation (scales, line groups)."},
]

BONE_META = {
    "floor": (t("Depot floor (frame of the harp, trolley and cover)", "Lagerboden (Bezug von Harfe, Rollbrett und Hülle)"), None, None),
    "trolley": (t("Harp trolley: carpeted plywood deck with rim battens, stays behind when the harp is played", "Harfenrollbrett: Sperrholzdeck mit Teppich und Randleisten, bleibt beim Spielen zurück"), (4.91, 0.34, 0.012, "point", "authored-estimate", "wood-handbook"), {"areaM2": 0.009, "secondMomentM4": 2.43e-07, "extremeFibreM": 0.009, "material": "plywood-birch", "title": t("Birch plywood deck 500 x 18 mm", "Birkensperrholzdeck 500 x 18 mm")}),
    "castors-rear": (t("Rear castor pair (wheels spin when rolled)", "Hinteres Lenkrollenpaar (Räder drehen beim Rollen)"), (0.64, 0.0, 0.0, "point", "authored-estimate", "en-12530"), None),
    "castors-front": (t("Front castor pair (wheels spin when rolled)", "Vorderes Lenkrollenpaar (Räder drehen beim Rollen)"), (0.64, 0.0, 0.0, "point", "authored-estimate", "en-12530"), None),
    "harp-lift": (t("Lift of the harp from the trolley deck down to the floor (massless)", "Absenken der Harfe vom Rollbrett auf den Boden (masselos)"), None, None),
    "harp": (t("Base of the harp with the pedal box, pivot of the tilt at the rear edge of the rear feet", "Fuß der Harfe mit Pedalkasten, Kippachse an der Hinterkante der hinteren Füße"), (8.0, 0.265, 0.075, "point", "authored-estimate", "concert-harp-catalogues"), {"areaM2": 0.01515, "secondMomentM4": 2.742e-05, "extremeFibreM": 0.0525, "material": "maple-lacquer", "title": t("Hollow wooden base 430 x 105 mm, walls 15 mm", "Hohler Holzfuß 430 x 105 mm, Wände 15 mm")}),
    "column": (t("Column with the seven pedal rods, gilded foot and crown", "Säule mit den sieben Pedalstangen, vergoldetem Fuß und Kapitell"), (5.0, 0.84, 0.0, "point", "authored-estimate", "concert-harp-catalogues"), {"areaM2": 0.003365, "secondMomentM4": 1.279e-06, "extremeFibreM": 0.036, "material": "maple-lacquer", "title": t("Hollow maple column 72 mm, bore 30 mm", "Hohle Ahornsäule 72 mm, Bohrung 30 mm")}),
    "neck": (t("Neck with the brass disc action and the tuning pins", "Hals mit der Messing-Scheibenmechanik und den Stimmwirbeln"), (9.5, 0.45, 0.05, "point", "authored-estimate", "concert-harp-catalogues"), {"areaM2": 0.00672, "secondMomentM4": 1.0976e-05, "extremeFibreM": 0.07, "material": "maple-lacquer", "title": t("Laminated maple neck 48 x 140 mm", "Verleimter Ahornhals 48 x 140 mm")}),
    "soundbox": (t("Soundbox: spruce soundboard on a ribbed maple back", "Resonanzkörper: Fichtendecke auf gerippter Ahornschale"), (10.9, 0.62, 0.075, "point", "authored-estimate", "concert-harp-catalogues"), {"areaM2": 0.00697, "secondMomentM4": 4.25e-05, "extremeFibreM": 0.131, "material": "maple-lacquer", "title": t("Half-round shell r 200 mm x 6 mm with an 8 mm soundboard", "Halbrunde Schale r 200 mm x 6 mm mit 8-mm-Decke")}),
    "strings": (t("The 47 strings C1 to G7 from the soundboard to the neck", "Die 47 Saiten C1 bis G7 von der Decke zum Hals"), (0.85, 0.55, 0.33, "point", "authored-estimate", "rossing-2010"), {"areaM2": 7.2e-05, "secondMomentM4": 1e-12, "extremeFibreM": 0.001, "material": "gut-string", "title": t("Sum of the string sections, about 72 mm2", "Summe der Saitenquerschnitte, etwa 72 mm²")}),
    "cover-turn": (t("Pivot of the cover, lays it on its left side on the floor (massless)", "Drehpunkt der Hülle, legt sie auf ihre linke Seite auf den Boden (masselos)"), None, None),
    "cover": (t("Padded transport cover, stays behind when the harp is played", "Gepolsterte Transporthülle, bleibt beim Spielen zurück"), (2.6, None, None, "point", "authored-estimate", "concert-harp-catalogues"), None),
}
PEDAL_NAMES = {"d": ("D", "D"), "c": ("C", "C"), "b": ("B", "H"), "e": ("E", "E"), "f": ("F", "F"), "g": ("G", "G"), "a": ("A", "A")}
for _n, _z in PEDALS:
    _en, _de = PEDAL_NAMES[_n]
    _side = ("left", "links") if _z > 0 else ("right", "rechts")
    BONE_META[f"pedal-{_n}"] = (t(f"{_en} pedal ({_side[0]} foot), three notches flat, natural, sharp", f"{_de}-Pedal ({_side[1]}er Fuß), drei Kerben b, Auflösung, Kreuz"), (0.25, 0.2, 0.0, "point", "authored-estimate", "rensch-2007"), {"areaM2": 0.000192, "secondMomentM4": 4.1e-09, "extremeFibreM": 0.008, "material": "steel-nickel", "title": t("Steel bar 12 x 16 mm", "Stahlstab 12 x 16 mm")})


def rig_bones(bones, mass_at):
    """🦴️ The rig bones with titles, masses and sections."""
    out = []
    for b in bones:
        title, mass, section = BONE_META[b["id"]]
        nb = {"id": b["id"], "parent": b["parent"], "title": title}
        nb.update({k: v for k, v in b.items() if k not in ("id", "parent")})
        if mass:
            kg, cx, cy, dist, src, ref = mass
            if b["id"] in mass_at:
                cx, cy = mass_at[b["id"]]
            nb["mass"] = {"kg": kg, "comX": r6(cx), "comY": r6(cy), "distribution": dist, "source": src, "reference": ref}
        if section:
            nb["section"] = section
        out.append(nb)
    return out


def anchors(rig, cover_hw_u):
    """📍️ Contacts, supports and load points."""
    a = []

    def add(id_, bone, x, y, kind, en, de, z=None, **kw):
        d = {"id": id_, "bone": bone, "x": r6(x), "y": r6(y)}
        if z is not None:
            d["z"] = r6(z)
        d.update({"kind": kind, "title": t(en, de)})
        d.update(kw)
        a.append(d)

    for cid, cx in (("rear", CASTOR_X[0]), ("front", CASTOR_X[1])):
        for side, zc, en, de in (("l", CASTOR_Z, "left", "links"), ("r", -CASTOR_Z, "right", "rechts")):
            add(f"castor-{cid}-{side}", "trolley", cx - TROLLEY_X0, -DECK_Y0, "contact", f"{cid.capitalize()} {en} castor on the floor", f"{'Hintere' if cid == 'rear' else 'Vordere'} Lenkrolle {de} auf dem Boden", zc, support="contact", footprintLengthM=0.012, footprintWidthM=0.02, frictionCoefficient=0.3)
    for fx, fn, fen, fde in ((0.0, "rear", "Rear", "Hinterer"), (FEET_X[1], "front", "Front", "Vorderer"), ):
        for side, zc, en, de in (("l", FEET_Z, "left", "linker"), ("r", -FEET_Z, "right", "rechter")):
            add(f"foot-{fn}-{side}", "harp", fx, 0.0, "contact", f"{fen} {en} foot of the harp", f"{fde} {de} Fuß der Harfe", zc, support="contact", footprintLengthM=0.04, footprintWidthM=0.04, frictionCoefficient=0.6)
    add("shoulder", "soundbox", 1.35, sb_dp(1.35), "contact", "Harpist's right hand, then right shoulder, at the top of the soundbox", "Rechte Hand, dann rechte Schulter der Harfenistin oben am Resonanzkörper", support="contact", normalDeg=50, footprintLengthM=0.1, footprintWidthM=0.08, frictionCoefficient=0.5)
    (ox, oy), ang, zb = rig.f["cover"]
    u = 0.45 * [b for b in rig.bones if b["id"] == "cover"][0]["lengthM"]
    add("cover-panel", "cover", u, 0.0, "contact", "Left panel of the cover lying on the floor", "Linke Seitenwand der Hülle auf dem Boden", cover_hw_u(u) - zb, support="contact", normalDeg=90, footprintLengthM=0.6, footprintWidthM=0.4, frictionCoefficient=0.5)
    gx, gy = rig.local("neck", (-0.28, 1.70 + T_DECK))
    add("push-grip", "neck", gx, gy, "load", "Mover's hand on the neck through the cover", "Hand des Trägers am Hals durch die Hülle")
    for nm, (hx, hy), en, de in (("pluck-bass", (0.12, 1.05), "Left hand plucking in the bass", "Linke Hand zupft im Bass"), ("pluck-treble", (-0.2, 1.42), "Right hand plucking in the treble", "Rechte Hand zupft im Diskant")):
        q = rig.local("strings", (hx, hy + T_DECK))
        add(nm, "strings", q[0], q[1], "load", en, de)
    for nm in ("d", "e"):
        add(f"pedal-{nm}-toe", f"pedal-{nm}", 0.29, 0.0, "load", f"Toe of the {PEDAL_NAMES[nm][0]} pedal", f"Spitze des {PEDAL_NAMES[nm][1]}-Pedals")
    return a


REGION_IDS = ["soundboard", "soundbox", "column", "gilding", "neck", "action", "strings", "base", "pedals", "cover", "trolley", "castors"]


def bind(state):
    """🌬️️ Region bindings of one pose: stored under the cover, or uncovered beside the lying cover, or played."""
    under = state in ("stored", "carried")
    out = {}
    for r in REGION_IDS:
        if r == "cover":
            b = {"exposedFraction": 1.0, "sunlitFraction": 0.0} if under else {"exposedFraction": 1.0, "sunlitFraction": 0.0, "contact": {"fraction": 0.3, "tempC": 18}}
        elif r in ("trolley", "castors"):
            b = {"exposedFraction": 0.9 if (under and r == "trolley") else 1.0, "sunlitFraction": 0.0}
        elif r in ("base", "pedals"):
            b = {"exposedFraction": 1.0, "sunlitFraction": 0.0}
        else:
            b = {"exposedFraction": 0.3 if under else 1.0, "sunlitFraction": 0.0}
        if state == "played" and r == "soundbox":
            b["contact"] = {"fraction": 0.04, "tempC": 33}
        if state == "played" and r == "strings":
            b["contact"] = {"fraction": 0.02, "tempC": 33}
        if state in ("upright", "played") and r == "base":
            b["contact"] = {"fraction": 0.02, "tempC": 18}
        out[r] = b
    return out


def tension(angle):
    """🎼️ The string tension as a self-equilibrated pair along the tilted strings: the neck pulled towards the soundboard, the soundboard towards the neck."""
    a = math.radians(angle)
    dx, dy = math.sin(a), -math.cos(a)
    wn = TENSION_N / 0.8
    ws = TENSION_N / (1.30)
    return [
        {"id": "strings-on-neck", "kind": "imposed", "title": t("Pull of the 47 strings on the neck (about 8 kN)", "Zug der 47 Saiten am Hals (etwa 8 kN)"), "fxN": 0, "fyN": 0, "distributed": {"bone": "neck", "fromM": 0.05, "toM": 0.85, "wxNPerM": r6(wn * dx), "wyNPerM": r6(wn * dy)}, "source": "authored-estimate", "reference": "rossing-2010"},
        {"id": "strings-on-soundboard", "kind": "imposed", "title": t("Pull of the 47 strings on the soundboard (about 8 kN)", "Zug der 47 Saiten an der Decke (etwa 8 kN)"), "fxN": 0, "fyN": 0, "distributed": {"bone": "soundbox", "fromM": 0.1, "toM": 1.4, "wxNPerM": r6(-ws * dx), "wyNPerM": r6(-ws * dy)}, "source": "authored-estimate", "reference": "rossing-2010"},
    ]


CASTORS = ["castor-rear-l", "castor-rear-r", "castor-front-l", "castor-front-r"]
FEET = ["foot-rear-l", "foot-rear-r", "foot-front-l", "foot-front-r"]


def poses(cover_twist):
    """🧍️ Stored, carried, upright and played."""
    wheel = r6(-math.degrees(ROLL_M / WHEEL_R))
    off = {"harp-lift": {"dy": r6(-T_DECK)}, "trolley": {"dx": TROLLEY_PARK}, "cover-turn": {"twistDeg": cover_twist}}
    played = dict(off, harp={"angleDeg": TILT_DEG}, **{f"pedal-{n}": {"angleDeg": NATURAL} for n, _ in PEDALS})
    return [
        {"id": "stored", "title": t("Stored on its trolley under the cover", "Auf dem Rollbrett unter der Hülle gelagert"), "description": t("In the dealer's depot the harp stands on its four rubber feet inside the rim of a carpeted castor trolley, under its faded and taped padded cover; only the base, the seven pedals in the flat notch and the trolley show.", "Im Lager des Händlers steht die Harfe auf ihren vier Gummifüßen innerhalb der Randleisten eines teppichbelegten Rollbretts, unter ihrer ausgeblichenen und geflickten gepolsterten Hülle; nur Fuß, die sieben Pedale in der b-Kerbe und das Rollbrett sind zu sehen."), "bones": {}, "ground": "authored", "contacts": CASTORS, "thermal": {"regions": bind("stored"), "emitters": {}}, "loads": tension(0.0)},
        {"id": "carried", "title": t("Rolled on the trolley", "Auf dem Rollbrett gerollt"), "description": t(f"One person steadies the covered harp at the neck and pushes it {dez(ROLL_M)} m across the depot floor on the trolley; the castor wheels turn {dez(round(abs(wheel) / 360, 1))} times.", f"Eine Person hält die verhüllte Harfe am Hals und schiebt sie auf dem Rollbrett {dez(ROLL_M)} m über den Lagerboden; die Lenkrollen drehen sich {dez(round(abs(wheel) / 360, 1))}-mal."), "bones": {"castors-rear": {"angleDeg": wheel}, "castors-front": {"angleDeg": wheel}}, "root": {"x": ROLL_M}, "ground": "authored", "contacts": CASTORS, "thermal": {"regions": bind("carried"), "emitters": {}}, "loads": tension(0.0) + [{"id": "push", "kind": "live", "title": t("Push of the mover's hand on the neck", "Schub der Hand am Hals"), "anchor": "push-grip", "fxN": 35, "fyN": 0, "source": "authored-estimate", "reference": "en-12530"}]},
        {"id": "upright", "title": t("Uncovered and upright on the floor", "Enthüllt und aufrecht auf dem Boden"), "description": t(f"The cover lies on its left side on the floor beside the harp, the trolley is pushed {dez(TROLLEY_PARK)} m forward, and the harp stands on its four feet on the floor, pedals in the flat notch, ready to be tilted.", f"Die Hülle liegt auf ihrer linken Seite neben der Harfe auf dem Boden, das Rollbrett ist {dez(TROLLEY_PARK)} m nach vorn geschoben, und die Harfe steht auf ihren vier Füßen auf dem Boden, Pedale in der b-Kerbe, bereit zum Kippen."), "bones": off, "ground": "authored", "contacts": CASTORS + FEET + ["cover-panel"], "thermal": {"regions": bind("upright"), "emitters": {}}, "loads": tension(0.0)},
        {"id": "played", "title": t("Tilted onto the shoulder and played", "Auf die Schulter gekippt und gespielt"), "description": t(f"Tilted back {dez(TILT_DEG)} degrees about the rear feet onto the harpist's right shoulder, the front feet off the floor, all seven pedals in the natural notch; both hands pluck and both feet rest on the D and E pedals.", f"Um {dez(TILT_DEG)} Grad über die hinteren Füße auf die rechte Schulter der Harfenistin gekippt, die vorderen Füße vom Boden abgehoben, alle sieben Pedale in der Auflösungskerbe; beide Hände zupfen, beide Füße ruhen auf dem D- und dem E-Pedal."), "bones": played, "ground": "authored", "contacts": CASTORS + ["foot-rear-l", "foot-rear-r", "shoulder", "cover-panel"], "thermal": {"regions": bind("played"), "emitters": {}}, "loads": tension(TILT_DEG) + [
            {"id": "pluck-left", "kind": "live", "title": t("Left hand plucking in the bass", "Linke Hand zupft im Bass"), "anchor": "pluck-bass", "fxN": -4, "fyN": 0, "source": "authored-estimate", "reference": "rossing-2010"},
            {"id": "pluck-right", "kind": "live", "title": t("Right hand plucking in the treble", "Rechte Hand zupft im Diskant"), "anchor": "pluck-treble", "fxN": -3, "fyN": 0, "source": "authored-estimate", "reference": "rossing-2010"},
            {"id": "foot-on-d", "kind": "live", "title": t("Left foot resting on the D pedal", "Linker Fuß ruht auf dem D-Pedal"), "anchor": "pedal-d-toe", "fxN": 0, "fyN": -15, "source": "authored-estimate", "reference": "rensch-2007"},
            {"id": "foot-on-e", "kind": "live", "title": t("Right foot resting on the E pedal", "Rechter Fuß ruht auf dem E-Pedal"), "anchor": "pedal-e-toe", "fxN": 0, "fyN": -15, "source": "authored-estimate", "reference": "rensch-2007"},
        ]},
    ]


def key_poses(base):
    """🔑️ The in-between key of the pedal clip: the D pedal held in the sharp notch by the left foot."""
    played = [p for p in base if p["id"] == "played"][0]
    loads = [x for x in played["loads"] if x["id"] != "foot-on-d"] + [{"id": "pedal-push", "kind": "live", "title": t("Left foot holding the D pedal down in sharp against spring and strings", "Linker Fuß hält das D-Pedal gegen Feder und Saiten im Kreuz"), "anchor": "pedal-d-toe", "fxN": 0, "fyN": -45, "source": "authored-estimate", "reference": "rensch-2007"}]
    bones = dict(played["bones"])
    bones["pedal-d"] = {"angleDeg": SHARP}
    return [{"id": "pedal-d-sharp", "title": t("D pedal held in sharp", "D-Pedal im Kreuz gehalten"), "description": t("Key of the pedal clip: played as before, the D pedal pushed one notch down into sharp.", "Schlüssel des Pedalclips: gespielt wie zuvor, das D-Pedal eine Kerbe tiefer ins Kreuz gedrückt."), "key": True, "bones": bones, "ground": "authored", "contacts": played["contacts"], "thermal": played["thermal"], "loads": loads}]


def clips():
    """🎞️ Carry on the trolley, tilt onto the shoulder, and the pedal change."""
    span = lambda ids, a, b: [{"anchor": x, "fromS": a, "toS": b} for x in ids]
    return [
        {"id": "carry", "title": t("Roll the covered harp on its trolley", "Die verhüllte Harfe auf dem Rollbrett rollen"), "description": t(f"From the stored place the mover steadies the neck and pushes the trolley {dez(ROLL_M)} m in eight seconds; the castor wheels roll without slip.", f"Vom Lagerplatz aus hält der Träger den Hals und schiebt das Rollbrett in acht Sekunden {dez(ROLL_M)} m weit; die Lenkrollen rollen ohne Schlupf."), "durationS": 8, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 8, "pose": "carried", "ease": "smooth"}],
         "contacts": span(CASTORS, 0, 8), "phases": [{"id": "start", "fromS": 0, "toS": 2, "title": t("Start pushing", "Anschieben")}, {"id": "roll", "fromS": 2, "toS": 6, "title": t("Rolling", "Rollen")}, {"id": "stop", "fromS": 6, "toS": 8, "title": t("Stop", "Anhalten")}],
         "events": [{"t": 0, "kind": "load-on", "title": t("Hand on the neck", "Hand am Hals"), "target": "push"}, {"t": 8, "kind": "load-off", "title": t("Arrived", "Angekommen"), "target": "push"}],
         "tracks": []},
        {"id": "tilt-to-shoulder", "title": t("Tilt back onto the shoulder", "Auf die Schulter zurückkippen"), "description": t(f"The harpist takes the top of the soundbox with her right hand and tilts the harp {dez(TILT_DEG)} degrees back about its rear feet onto her right shoulder; the front feet lift off and the pedals are set from flat to natural.", f"Die Harfenistin fasst den Resonanzkörper oben mit der rechten Hand und kippt die Harfe um {dez(TILT_DEG)} Grad über die hinteren Füße auf ihre rechte Schulter; die vorderen Füße heben ab, und die Pedale werden von b auf Auflösung gestellt."), "durationS": 4, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "upright", "ease": "smooth"}, {"t": 3.2, "pose": "played", "ease": "smooth"}, {"t": 4, "pose": "played", "ease": "smooth"}],
         "contacts": span(CASTORS + ["cover-panel", "foot-rear-l", "foot-rear-r"], 0, 4) + span(["foot-front-l", "foot-front-r"], 0, 0.5) + span(["shoulder"], 0.3, 4),
         "phases": [{"id": "grip", "fromS": 0, "toS": 0.3, "title": t("Hand at the soundbox top", "Hand oben am Resonanzkörper")}, {"id": "tilt", "fromS": 0.3, "toS": 3.2, "title": t("Tilting back", "Zurückkippen")}, {"id": "rest", "fromS": 3.2, "toS": 4, "title": t("Resting on the shoulder", "Auf der Schulter")}],
         "events": [{"t": 0.5, "kind": "contact-off", "title": t("Front feet leave the floor", "Vordere Füße verlassen den Boden"), "target": "foot-front-l"}, {"t": 3.2, "kind": "contact-on", "title": t("Soundbox rests on the shoulder", "Resonanzkörper liegt auf der Schulter"), "target": "shoulder"}],
         "tracks": []},
        {"id": "pedal-change", "title": t("D pedal from natural to sharp and back", "D-Pedal von Auflösung ins Kreuz und zurück"), "description": t("Playing, the left foot pushes the D pedal down one notch into sharp (the discs turn and shorten every D string), holds it for a phrase and lets it back up into natural.", "Beim Spielen drückt der linke Fuß das D-Pedal eine Kerbe tiefer ins Kreuz (die Scheiben drehen und verkürzen jede D-Saite), hält es eine Phrase lang und lässt es zurück in die Auflösung."), "durationS": 4, "fps": 25, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "played", "ease": "smooth"}, {"t": 0.6, "pose": "pedal-d-sharp", "ease": "smooth"}, {"t": 2.4, "pose": "pedal-d-sharp", "ease": "smooth"}, {"t": 3.0, "pose": "played", "ease": "smooth"}, {"t": 4, "pose": "played", "ease": "smooth"}],
         "contacts": span(CASTORS + ["cover-panel", "foot-rear-l", "foot-rear-r", "shoulder"], 0, 4),
         "phases": [{"id": "press", "fromS": 0, "toS": 0.6, "title": t("Press into sharp", "Ins Kreuz drücken")}, {"id": "hold", "fromS": 0.6, "toS": 2.4, "title": t("Hold", "Halten")}, {"id": "release", "fromS": 2.4, "toS": 3.0, "title": t("Back into natural", "Zurück in die Auflösung")}, {"id": "play", "fromS": 3.0, "toS": 4, "title": t("Playing on", "Weiterspielen")}],
         "events": [{"t": 0.6, "kind": "motion", "title": t("D pedal in the sharp notch", "D-Pedal in der Kreuzkerbe"), "target": "pedal-d"}, {"t": 3.0, "kind": "motion", "title": t("D pedal back in natural", "D-Pedal zurück in der Auflösung"), "target": "pedal-d"}],
         "tracks": []},
    ]


REGIONS = [
    ("soundboard", t("Spruce soundboard and centre strip, varnish yellowed and crazed", "Fichtendecke und Saitenleiste, Lack vergilbt und craqueliert"), "surface", "spruce-varnished", 0.3, "vertical-plate", 0.3, 5760),
    ("soundbox", t("Ribbed maple back and foot of the soundbox, lacquer worn at the shoulder", "Gerippte Ahornschale und Fuß des Resonanzkörpers, Lack an der Schulter abgegriffen"), "surface", "maple-lacquer", 0.4, "vertical-plate", 0.3, 6240),
    ("column", t("Column shaft, lacquer rubbed at the hand height", "Säulenschaft, Lack in Handhöhe abgegriffen"), "surface", "maple-lacquer", 0.072, "cylinder", 0.4, 17700),
    ("gilding", t("Gilded column foot and crown, worn to the bole", "Vergoldeter Säulenfuß und Kapitell, bis aufs Poliment abgerieben"), "surface", "gold-leaf-worn", 0.15, "cylinder", 0.45, 31200),
    ("neck", t("Maple neck", "Ahornhals"), "surface", "maple-lacquer", 0.14, "vertical-plate", 0.4, 25000),
    ("action", t("Brass action plates with the discs, tarnished", "Messing-Mechanikplatten mit den Scheiben, angelaufen"), "surface", "brass-tarnished", 0.08, "vertical-plate", 0.4, 9690),
    ("strings", t("The 47 strings", "Die 47 Saiten"), "surface", "gut-string", 0.0015, "cylinder", 0.4, 730),
    ("base", t("Base with the pedal box and rubber feet, corners scuffed", "Fuß mit Pedalkasten und Gummifüßen, Ecken bestoßen"), "surface", "maple-lacquer", 0.105, "vertical-plate", 0.2, 15600),
    ("pedals", t("Seven pedals, plating worn at the toes", "Sieben Pedale, Vernickelung an den Spitzen abgetreten"), "surface", "steel-nickel", 0.016, "cylinder", 0.2, 14444),
    ("cover", t("Padded transport cover", "Gepolsterte Transporthülle"), "surface", "padded-nylon", 0.5, "vertical-plate", 0.4, 1008),
    ("trolley", t("Trolley deck, carpet and rim battens", "Rollbrettdeck, Teppich und Randleisten"), "surface", "plywood-birch", 0.5, "horizontal-plate-up", 0.2, 19584),
    ("castors", t("Four castors", "Vier Lenkrollen"), "surface", "polyurethane-grey", 0.05, "cylinder", 0.1, 27000),
]


def thermal(areas):
    """🌡️️ The passive thermal model: indoor depot, no power."""
    regions = []
    for rid, title, kind, mat, L, geom, sky, hc in REGIONS:
        r = {"id": rid, "title": title, "kind": kind, "model": "passive", "material": mat, "areaM2": areas[rid], "characteristicLengthM": L, "geometry": geom, "skyViewFactor": sky, "layers": [], "heatCapacityJM2K": hc, "source": "authored-estimate", "reference": "en-iso-10456" if rid in ("cover", "trolley", "castors") else "wood-handbook"}
        if rid == "cover":
            r["layers"] = [{"id": "quilted-padding", "title": t("Nylon shell on 12 mm foam, quilted", "Nylonhülle auf 12 mm Schaum, gesteppt"), "kind": "insulation", "material": "padded-nylon", "resistanceM2KW": 0.3, "heatCapacityJM2K": 1008, "source": "authored-estimate", "reference": "en-iso-10456"}]
        if rid == "soundboard":
            r["ratedPowerW"] = 0
            r["ratedPowerBasis"] = "passive"
        regions.append(r)
    return {
        "environment": {"airTempC": 19, "meanRadiantTempC": 19, "skyTempC": 19, "groundTempC": 18, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
        "passengers": [], "operating": [], "regions": regions, "emitters": [],
        "assumptions": [
            t("An acoustic instrument draws no power: every region is passive, 0 W, stated on the soundboard region; the trolley and the cover are passive as well.", "Ein akustisches Instrument nimmt keine Leistung auf: alle Regionen sind passiv, 0 W, an der Deckenregion angegeben; Rollbrett und Hülle sind ebenfalls passiv."),
            t("Reference case: a heated depot of a used-instrument dealer at 19 degC and 45 % relative humidity, no sun, still air; under the cover the harp exchanges heat with the trapped air only (30 % of its surface counted as exposed).", "Referenzfall: ein beheiztes Lager eines Gebrauchtinstrumentenhandels bei 19 °C und 45 % relativer Feuchte, keine Sonne, ruhende Luft; unter der Hülle tauscht die Harfe nur mit der eingeschlossenen Luft Wärme aus (30 % ihrer Oberfläche als frei gezählt)."),
            t("Aged surfaces: the yellowed spirit varnish and the worn nitro lacquer stay near emissivity 0.9; the gilding rubbed through to the red bole reads 0.35 instead of 0.02 to 0.05 for intact gold leaf; the tarnished brass plates 0.3 instead of 0.03 to 0.05 when polished; the pedals' nickel plating 0.2 where it is worn bright.", "Gealterte Oberflächen: der vergilbte Spirituslack und der abgegriffene Nitrolack bleiben nahe Emissionsgrad 0,9; die bis aufs rote Poliment abgeriebene Vergoldung liegt bei 0,35 statt 0,02 bis 0,05 für intaktes Blattgold; die angelaufenen Messingplatten bei 0,3 statt 0,03 bis 0,05 poliert; die Vernickelung der Pedale bei 0,2, wo sie blank getreten ist."),
            t("Played, the harpist's shoulder and knees touch about 4 % of the soundbox back and the fingertips about 2 % of the strings at 33 degC; a plucked instrument has no breath or exhaust, so the asset has no emitter.", "Beim Spielen berühren Schulter und Knie der Harfenistin etwa 4 % der Schale und die Fingerkuppen etwa 2 % der Saiten bei 33 °C; ein Zupfinstrument hat weder Atem noch Abluft, daher hat das Asset keinen Emitter."),
            t("The padded cover is one layer of nylon over 12 mm foam (R about 0.3 m2K/W); lying on the floor, 30 % of it touches the 18 degC floor.", "Die gepolsterte Hülle ist eine Schicht Nylon über 12 mm Schaum (R etwa 0,3 m²K/W); auf dem Boden liegend berühren 30 % den 18 °C warmen Boden."),
        ],
    }


STRUCTURE = {"gravityMS2": G, "dynamics": "quasi-static", "supports": CASTORS, "assumptions": [
    t("Stored and carried, the whole stack of harp, cover and trolley is one rigid body on the four castors (EN 12530 general-purpose castors, rolling resistance taken by friction 0.3); the harp stands on its four rubber feet inside the rim battens of the deck, the cover hangs on the crown, the neck and the soundbox top.", "Gelagert und gerollt ist der ganze Stapel aus Harfe, Hülle und Rollbrett ein starrer Körper auf den vier Lenkrollen (Lenkrollen nach EN 12530, Rollwiderstand über Reibung 0,3); die Harfe steht auf ihren vier Gummifüßen innerhalb der Randleisten des Decks, die Hülle hängt an Kapitell, Hals und Resonanzkörperkopf."),
    t(f"Upright and played the trolley and the cover stay behind as separate bodies on their own contacts; the archive solves one rigid assembly per pose, so their 8.2 kg are shared over all active contacts. Played, the harp rests on the rear edge of its rear feet and on the harpist's right shoulder at the top of the soundbox (restraint at 50 degrees), tilted {dez(TILT_DEG)} degrees.", f"Aufrecht und gespielt bleiben Rollbrett und Hülle als eigene Körper auf ihren Kontakten zurück; das Archiv löst je Pose eine starre Baugruppe, ihre 8,2 kg verteilen sich daher auf alle aktiven Kontakte. Gespielt ruht die Harfe auf der Hinterkante ihrer hinteren Füße und auf der rechten Schulter der Harfenistin oben am Resonanzkörper (Lagerung unter 50 Grad), um {dez(TILT_DEG)} Grad gekippt."),
    t("The 47 strings pull the neck towards the soundboard with about 8 kN in total (order of magnitude after Rossing 2010), applied as two equal and opposite distributed loads along the neck and the soundboard: they close through neck, column, base and soundbox, which is the frame of the instrument (the column in compression, neck and soundbox in bending), and change no reaction.", "Die 47 Saiten ziehen den Hals mit insgesamt etwa 8 kN zur Decke (Größenordnung nach Rossing 2010), angesetzt als zwei gleich große, entgegengesetzte Streckenlasten längs Hals und Decke: sie schließen sich über Hals, Säule, Fuß und Resonanzkörper, den Rahmen des Instruments (Säule auf Druck, Hals und Resonanzkörper auf Biegung), und ändern keine Auflagerkraft."),
    t("Masses: base with the pedal box 8.0 kg, seven pedals 0.25 kg each, column with the pedal rods 5.0 kg, neck with the brass action 9.5 kg, soundbox 10.9 kg, strings 0.85 kg (harp 36.0 kg); trolley deck 4.91 kg and four castors 1.28 kg; padded cover 2.6 kg; total 44.79 kg.", "Massen: Fuß mit Pedalkasten 8,0 kg, sieben Pedale je 0,25 kg, Säule mit den Pedalstangen 5,0 kg, Hals mit der Messingmechanik 9,5 kg, Resonanzkörper 10,9 kg, Saiten 0,85 kg (Harfe 36,0 kg); Rollbrettdeck 4,91 kg und vier Lenkrollen 1,28 kg; gepolsterte Hülle 2,6 kg; zusammen 44,79 kg."),
    t("Pedal forces: a foot resting on a pedal about 15 N, pushing one into the next notch against its spring and the string tension about 45 N at the toe (authored estimates); plucking about 3 to 4 N per hand.", "Pedalkräfte: ein auf dem Pedal ruhender Fuß etwa 15 N, das Drücken in die nächste Kerbe gegen Feder und Saitenzug etwa 45 N an der Spitze (Schätzwerte); Zupfen etwa 3 bis 4 N je Hand."),
], "clinical": False}


AREAS = {"soundboard": 0.898, "soundbox": 1.376, "column": 0.32, "gilding": 0.167, "neck": 0.4, "action": 0.228, "strings": 0.16, "base": 0.75, "pedals": 0.061, "cover": 4.2, "trolley": 0.9, "castors": 0.067}


def _s(sid, bid, role, z, pts, closed=False, filled=False, tone=0.5, region=None, pen=None):
    """✏️️ One hand-drawn trace shape."""
    s = {"id": sid, "bone": bid, "role": role, "z": z, "filled": filled, "tone": tone}
    if region:
        s["region"] = region
    if pen is not None:
        s["penMm"] = pen
    s.update({"kind": "poly", "points": [[r6(a), r6(b)] for a, b in pts], "closed": closed})
    return s


def details(scale, rig, harp, lw, cover_hw_u):
    """🩹️ The traces of use and the secondary lines of one scale, drawn by hand on the parts they belong to."""
    T = T_DECK
    loc = lambda bid, pts: [rig.local(bid, p) for p in pts]
    el, fr, pl = [], [], []
    if scale in (50, 100):
        el.append(_s("base-moulding-line", "harp", "detail", 2.1, loc("harp", [harp(BASE_X0, 0.115), harp(BASE_X1, 0.115)]), region="base"))
        el.append(_s("dealer-sticker", "harp", "texture", 2.2, loc("harp", [harp(0.30, 0.05), harp(0.37, 0.05), harp(0.37, 0.095), harp(0.30, 0.095)]), True, True, 0.92, "base"))
        el.append(_s("hand-wear-column", "column", "texture", 4.1, loc("column", [harp(COL_X - 0.03, 0.95), harp(COL_X + 0.03, 0.95), harp(COL_X + 0.028, 1.16), harp(COL_X - 0.028, 1.16)]), True, True, 0.75, "column"))
        el.append(_s("gilding-worn-crown", "column", "texture", 4.3, loc("column", [harp(COL_X - 0.05, 1.66), harp(COL_X + 0.01, 1.68), harp(COL_X + 0.04, 1.76), harp(COL_X - 0.02, 1.79), harp(COL_X - 0.06, 1.73)]), True, True, 0.45, "gilding"))
        sh = [sb_point(1.22, 0.7 * sb_dp(1.22)), sb_point(1.42, 0.7 * sb_dp(1.42)), sb_point(1.42, 0.96 * sb_dp(1.42)), sb_point(1.22, 0.96 * sb_dp(1.22))]
        el.append(_s("shoulder-wear", "soundbox", "texture", 3.05, loc("soundbox", [harp(*p) for p in sh]), True, True, 0.7, "soundbox"))
        fr.append(_s("soundboard-decal", "soundbox", "texture", 3.15, [(0.09, 0.62 + T), (0.12, 0.74 + T), (0.08, 0.86 + T), (0.1, 0.98 + T), (0.07, 1.1 + T)], region="soundboard", pen=0.1))
        fr.append(_s("soundboard-decal-r", "soundbox", "texture", 3.15, [(-0.09, 0.62 + T), (-0.12, 0.74 + T), (-0.08, 0.86 + T), (-0.1, 0.98 + T), (-0.07, 1.1 + T)], region="soundboard", pen=0.1))
        fr.append(_s("base-moulding-line", "harp", "detail", 2.1, [(-BASE_Z, 0.115 + T), (BASE_Z, 0.115 + T)], region="base"))
        pl.append(_s("base-moulding-line", "harp", "detail", 2.1, [(BASE_X0 + 0.008, -BASE_Z + 0.008), (BASE_X1 - 0.008, -BASE_Z + 0.008), (BASE_X1 - 0.008, BASE_Z - 0.008), (BASE_X0 + 0.008, BASE_Z - 0.008)], True, region="base"))
    if scale == 50:
        for k, v in enumerate((-0.018, 0.018)):
            el.append(_s(f"column-flute-{k}", "column", "texture", 4.05, [(0.14, v), (1.45, v)], region="column", pen=0.1))
            fr.append(_s(f"column-flute-{k}", "column", "texture", 4.05, [(v, 0.38 + T), (v, 1.58 + T)], region="column", pen=0.1))
        el.append(_s("crown-carving", "column", "texture", 4.35, loc("column", [harp(COL_X - 0.065, 1.71), harp(COL_X - 0.02, 1.735), harp(COL_X + 0.02, 1.735), harp(COL_X + 0.065, 1.71)]), region="gilding", pen=0.1))
        el.append(_s("foot-carving", "column", "texture", 4.35, loc("column", [harp(COL_X - 0.05, 0.19), harp(COL_X, 0.205), harp(COL_X + 0.05, 0.19)]), region="gilding", pen=0.1))
        for k, f in enumerate((0.55, 0.82)):
            el.append(_s(f"stave-seam-{k}", "soundbox", "texture", 3.02, [(0.06, f * sb_dp(0.06)), (1.42, f * sb_dp(1.42))], region="soundbox", pen=0.1))
        el.append(_s("rib-scratch", "soundbox", "texture", 3.03, [(0.62, 0.9 * sb_dp(0.62)), (0.7, 0.86 * sb_dp(0.7)), (0.76, 0.88 * sb_dp(0.76))], region="soundbox", pen=0.1))
        from author_harp_geometry import NECK_UNDER
        pins = [harp(x, y + 0.1) for x, y in NECK_UNDER if -0.36 <= x <= 0.42]
        el.append(_s("tuning-pin-row", "neck", "texture", 5.2, loc("neck", pins), region="neck", pen=0.1))
        discs = [harp(x, y + 0.03) for x, y in NECK_UNDER if -0.36 <= x <= 0.42]
        el.append(_s("disc-row", "neck", "texture", 5.2, loc("neck", discs), region="action", pen=0.1))
        el.append(_s("base-scuff", "harp", "texture", 2.2, loc("harp", [harp(0.50, 0.035), harp(0.53, 0.05), harp(0.545, 0.04)]), region="base", pen=0.1))
        el.append(_s("deck-scuff", "trolley", "texture", 1.1, loc("trolley", [(0.53, DECK_Y0 + 0.004), (0.56, DECK_Y0 + 0.012), (0.58, DECK_Y0 + 0.006)]), region="trolley", pen=0.1))
        fr.append(_s("crack-repair", "soundbox", "texture", 3.16, [(0.15, 0.48 + T), (0.152, 0.6 + T), (0.149, 0.71 + T)], region="soundboard", pen=0.1))
        pl.append(_s("pedal-slot-plate", "harp", "detail", 2.3, [(BASE_X0, -0.2), (BASE_X0 + 0.008, -0.2), (BASE_X0 + 0.008, 0.16), (BASE_X0, 0.16)], True, region="pedals"))
        pl.append(_s("carpet-worn", "trolley", "texture", 1.15, [(-0.058, 0.05), (-0.034, 0.05), (-0.034, 0.15), (-0.058, 0.15)], True, True, 0.45, "trolley"))
    return {"elevation": el, "front": fr, "plan": pl}


DECISIONS = {
    50: {
        "silhouette": t("Side elevation 22 x 39 mm: the padded cover drawn over the whole harp from the crown down to 0.29 m, a convex quilted bag that follows the column, the neck and the soundbox back; below it the 590 x 105 mm base with its moulding, the seven pedals standing 70 mm out at the back, the rubber feet, and the carpeted trolley on four 50 mm castors. The harp itself is drawn under the cover part by part (column with gilded foot and crown, neck with the brass action plates, soundbox back, soundboard and centre strip, 47 strings), so every other pose shows it. Front: the cover tapering from 0.55 m at the hem to 0.18 m at the crown over the base and trolley. Plan: the cover over the base, the pedals at the back and the trolley rim.", "Seitenansicht 22 x 39 mm: die gepolsterte Hülle über der ganzen Harfe vom Kapitell bis 0,29 m, ein konvexer gesteppter Sack, der Säule, Hals und Schale folgt; darunter der 590 x 105 mm große Fuß mit seinem Profil, die sieben Pedale 70 mm hinten vorstehend, die Gummifüße und das teppichbelegte Rollbrett auf vier 50-mm-Lenkrollen. Die Harfe selbst ist unter der Hülle Teil für Teil gezeichnet (Säule mit vergoldetem Fuß und Kapitell, Hals mit den Messing-Mechanikplatten, Schale, Decke und Saitenleiste, 47 Saiten), damit jede andere Pose sie zeigt. Vorderansicht: die Hülle verjüngt sich von 0,55 m am Saum auf 0,18 m am Kapitell über Fuß und Rollbrett. Draufsicht: die Hülle über dem Fuß, die Pedale hinten und der Rand des Rollbretts."),
        "retained": t("Every part at its true size: 72 mm column, 150 mm crown, 48 mm neck with 3 mm brass plates, 8 mm soundboard and 8 mm centre strip, the half-round back, the 12 x 16 mm pedals with their rubber toes, the 40 mm feet, the 18 mm deck with 5 mm carpet and 20 mm rim battens, the castor plates, yokes and wheels; traces of use: gilding rubbed to the bole on the crown, hand wear on the column, the lacquer worn at the shoulder contact, a scratch on the back, a soundboard crack repair, the faded soundboard decals, a dealer sticker on the base, scuffed base and deck corners, the worn carpet.", "Jedes Teil in wahrer Größe: Säule 72 mm, Kapitell 150 mm, Hals 48 mm mit 3-mm-Messingplatten, Decke 8 mm und Saitenleiste 8 mm, die halbrunde Schale, die Pedale 12 x 16 mm mit Gummispitzen, die 40-mm-Füße, das 18-mm-Deck mit 5 mm Teppich und 20-mm-Randleisten, Platten, Gabeln und Räder der Lenkrollen; Gebrauchsspuren: Vergoldung am Kapitell bis aufs Poliment abgerieben, Griffspuren an der Säule, Lack an der Schulterstelle abgegriffen, ein Kratzer in der Schale, eine Deckenrissreparatur, die verblassten Deckenmalereien, ein Händleraufkleber am Fuß, bestoßene Fuß- und Deckecken, der abgetretene Teppich."),
        "merged": t("The 94 action discs and the 47 tuning pins are each one row line on the neck; the 7 pedal rods inside the column are part of its mass, not drawn.", "Die 94 Mechanikscheiben und die 47 Stimmwirbel sind je eine Reihenlinie am Hals; die 7 Pedalstangen in der Säule gehören zu ihrer Masse und sind nicht gezeichnet."),
        "omitted": t("Bridge pins, eyelets, string knots, the screws of the pedal box and the zip of the cover (all below 7.5 mm).", "Stegstifte, Ösen, Saitenknoten, die Schrauben des Pedalkastens und der Reißverschluss der Hülle (alle unter 7,5 mm)."),
        "thinElements": t("The 47 strings (0.8 to 2.1 mm) are 0.1 mm strokes from the eyelets to the neck; the front draws the string plane edge on, the plan its trace on the soundboard; the brass plates and the carpet are declared sheets.", "Die 47 Saiten (0,8 bis 2,1 mm) sind 0,1-mm-Striche von den Ösen zum Hals; die Vorderansicht zeigt die Saitenebene von der Kante, die Draufsicht ihre Spur auf der Decke; Messingplatten und Teppich sind erklärte dünne Platten."),
        "hatching": t("The cover carries a 45 degree cross hatch of 1.6 mm for its quilting; no section hatch, nothing is cut.", "Die Hülle trägt für ihre Steppung eine Kreuzschraffur unter 45 Grad mit 1,6 mm Abstand; keine Schnittschraffur, nichts ist geschnitten."),
        "styles": t("Outlines 0.35 mm, moulding, pedal plate and strings-plane 0.18 mm, grain, fluting, carving, pins, discs and wear 0.1 mm.", "Konturen 0,35 mm, Profil, Pedalplatte und Saitenebene 0,18 mm, Maserung, Kanneluren, Schnitzerei, Wirbel, Scheiben und Spuren 0,1 mm."),
        "overlays": t("Each part is a cell of its thermal region: soundboard, soundbox, column, gilding, neck, action, strings, base, pedals, cover, trolley and castors.", "Jedes Teil ist eine Zelle seiner thermischen Region: Decke, Resonanzkörper, Säule, Vergoldung, Hals, Mechanik, Saiten, Fuß, Pedale, Hülle, Rollbrett und Lenkrollen."),
    },
    100: {
        "silhouette": t("The same covered harp at 11 x 19.5 mm: quilted cover, base, pedals, feet and trolley with rims and castors; under the cover the column with simplified gilded foot and crown, the neck and plates, the soundbox in two layers and the 14 C and F strings.", "Dieselbe verhüllte Harfe mit 11 x 19,5 mm: gesteppte Hülle, Fuß, Pedale, Füße und Rollbrett mit Randleisten und Lenkrollen; unter der Hülle die Säule mit vereinfachtem vergoldetem Fuß und Kapitell, Hals und Platten, der Resonanzkörper in zwei Schichten und die 14 C- und F-Saiten."),
        "retained": t("Gilding wear on the crown, hand wear on the column, shoulder wear, the dealer sticker and the faded decals.", "Abgeriebene Vergoldung am Kapitell, Griffspuren an der Säule, Schulterspur, Händleraufkleber und verblasste Malereien."),
        "merged": t("Carpet into the deck (23 mm), base moulding into the base, castor plate and yoke into one block, pedal toe into the bar (15 x 18 mm), centre strip into a 16 mm soundboard layer.", "Teppich ins Deck (23 mm), Profil in den Fuß, Lenkrollenplatte und Gabel in einen Block, Pedalspitze in den Stab (15 x 18 mm), Saitenleiste in eine 16-mm-Deckenschicht."),
        "omitted": t("Fluting, carving lines, pin and disc rows, scratches and scuffs; 33 of the 47 strings (the C and F strings are kept, the coloured strings a harpist finds by).", "Kanneluren, Schnitzlinien, Wirbel- und Scheibenreihen, Kratzer und Stoßstellen; 33 der 47 Saiten (die C- und F-Saiten bleiben, die farbigen Saiten, an denen sich die Harfenistin orientiert)."),
        "thinElements": t("Strings 0.1 mm strokes; brass plates as declared sheets.", "Saiten als 0,1-mm-Striche; Messingplatten als erklärte dünne Platten."),
        "hatching": t("Cover quilting as a 0.8 mm cross hatch.", "Steppung der Hülle als Kreuzschraffur mit 0,8 mm."),
        "styles": t("Outlines 0.25 mm, moulding 0.13 mm, wear and strings 0.1 mm.", "Konturen 0,25 mm, Profil 0,13 mm, Spuren und Saiten 0,1 mm."),
        "overlays": t("As at 1:50.", "Wie 1:50."),
    },
    200: {
        "silhouette": t("Covered harp 5.5 x 9.8 mm on its trolley: the cover outline, the base down to the deck, two pedal blocks, the 43 mm deck and the castors; under the cover column, crown, neck, soundbox and the 7 C strings.", "Verhüllte Harfe 5,5 x 9,8 mm auf ihrem Rollbrett: Umriss der Hülle, der Fuß bis aufs Deck, zwei Pedalblöcke, das 43-mm-Deck und die Lenkrollen; unter der Hülle Säule, Kapitell, Hals, Resonanzkörper und die 7 C-Saiten."),
        "retained": t("The 0.29 m hem height of the cover, the 70 mm pedal overhang, the trolley height of 111 mm and the castors.", "Die Saumhöhe der Hülle von 0,29 m, der Pedalüberstand von 70 mm, die Rollbretthöhe von 111 mm und die Lenkrollen."),
        "merged": t("Feet into the base, rims and carpet into the deck, the pedals of each foot into one block 92 and 132 mm wide (typified), castor yokes 30 mm.", "Füße in den Fuß, Randleisten und Teppich ins Deck, die Pedale jedes Fußes in einen Block von 92 und 132 mm Breite (typisiert), Lenkrollengabeln 30 mm."),
        "omitted": t("All traces of use and the quilting; 40 of 47 strings.", "Alle Gebrauchsspuren und die Steppung; 40 der 47 Saiten."),
        "thinElements": t("Strings 0.1 mm; brass plates as declared sheets; castor wheels widened to 30 mm.", "Saiten 0,1 mm; Messingplatten als erklärte dünne Platten; Lenkrollen auf 30 mm verbreitert."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outlines 0.18 mm, strings 0.1 mm.", "Konturen 0,18 mm, Saiten 0,1 mm."),
        "overlays": t("The pedal blocks are cells of the pedals region.", "Die Pedalblöcke sind Zellen der Pedalregion."),
    },
    500: {
        "silhouette": t("Covered harp 2.2 x 3.9 mm: the cover outline, the base with the pedal band, and the trolley as one block 0.68 x 0.11 m; under the cover the column as one turned part with its crown, the neck thickened to 76 mm and the soundbox in two layers.", "Verhüllte Harfe 2,2 x 3,9 mm: Umriss der Hülle, der Fuß mit dem Pedalband und das Rollbrett als ein Block 0,68 x 0,11 m; unter der Hülle die Säule als ein gedrehtes Teil mit Kapitell, der auf 76 mm verdickte Hals und der Resonanzkörper in zwei Schichten."),
        "retained": t("Overall height 1.95 m, the 1.1 m length of the cover over the 0.68 m trolley, the hem above the base.", "Gesamthöhe 1,95 m, die Länge der Hülle von 1,1 m über dem 0,68-m-Rollbrett, der Saum über dem Fuß."),
        "merged": t("Trolley and castors into one block; pedals into the base outline; crown and foot into the column; plates and strings into the neck drawing (its shape lists the neck, action and strings regions).", "Rollbrett und Lenkrollen in einen Block; Pedale in den Umriss des Fußes; Kapitell und Fuß in die Säule; Platten und Saiten in die Halszeichnung (ihre Form führt die Regionen Hals, Mechanik und Saiten)."),
        "omitted": t("Strings, feet, rims, wheels and every trace of use.", "Saiten, Füße, Randleisten, Räder und alle Gebrauchsspuren."),
        "thinElements": t("None below the 75 mm feature; the neck (48 mm) and the column (72 mm) are generalised to 76 mm.", "Keine unter der kleinsten Einzelheit von 75 mm; Hals (48 mm) und Säule (72 mm) sind auf 76 mm verallgemeinert."),
        "hatching": t("No hatch.", "Keine Schraffur."),
        "styles": t("Outline 0.13 mm.", "Kontur 0,13 mm."),
        "overlays": t("Merged shapes list their regions: trolley and castors, base and pedals, column and gilding, neck, action and strings.", "Zusammengefasste Formen führen ihre Regionen: Rollbrett und Lenkrollen, Fuß und Pedale, Säule und Vergoldung, Hals, Mechanik und Saiten."),
    },
}

VOLUME_DECISIONS = {
    50: [
        t("Body: the harp part by part at true size: hollow base 590 x 430 x 90 mm with a 15 mm inset moulding and four turned rubber feet of 40 mm; the column as three turned parts (gilded foot 116 mm, fluted maple shaft 72 to 68 mm, gilded crown 150 mm); the neck a prism of its side profile 48 mm thick with a 3 mm brass action plate on each side (declared sheets); the soundbox as a half-superellipse back tapering from 450 x 250 to 130 x 100 mm over 1.46 m, an 8 mm spruce soundboard and an 8 mm centre strip, its foot block down onto the base; the 47 strings as slender capsules of 0.8 to 2.1 mm; seven pedals of 12 x 16 mm with rubber toes on their own pivots; the cover a convex prism over the harp with planar side panels that taper with the soundbox; the trolley deck, the 5 mm carpet (declared sheet), four rim battens and four castors with plates, yokes and 50 mm wheels.", "Körper: die Harfe Teil für Teil in wahrer Größe: hohler Fuß 590 x 430 x 90 mm mit eingerücktem 15-mm-Profil und vier gedrehten Gummifüßen von 40 mm; die Säule als drei gedrehte Teile (vergoldeter Fuß 116 mm, kannelierter Ahornschaft 72 bis 68 mm, vergoldetes Kapitell 150 mm); der Hals ein Prisma seines Seitenprofils, 48 mm dick, mit einer 3-mm-Messing-Mechanikplatte auf jeder Seite (erklärte dünne Platten); der Resonanzkörper als halbe Superellipse, die sich über 1,46 m von 450 x 250 auf 130 x 100 mm verjüngt, eine 8-mm-Fichtendecke und eine 8-mm-Saitenleiste, sein Fußblock bis auf den Harfenfuß; die 47 Saiten als schlanke Kapseln von 0,8 bis 2,1 mm; sieben Pedale 12 x 16 mm mit Gummispitzen auf eigenen Drehpunkten; die Hülle ein konvexes Prisma über der Harfe mit ebenen Seitenwänden, die sich mit dem Resonanzkörper verjüngen; das Rollbrettdeck, der 5-mm-Teppich (erklärte dünne Platte), vier Randleisten und vier Lenkrollen mit Platten, Gabeln und 50-mm-Rädern."),
        t("Thermal: every region is the skin of its body solids by reference: soundboard (board and strip), soundbox (back and foot), column, gilding (foot and crown), neck, action (plates), strings, base (base, moulding, feet), pedals, cover, trolley (deck, carpet, rims) and castors.", "Thermisch: jede Region ist die Haut ihrer Körper als Verweis: Decke (Decke und Leiste), Resonanzkörper (Schale und Fuß), Säule, Vergoldung (Fuß und Kapitell), Hals, Mechanik (Platten), Saiten, Fuß (Fuß, Profil, Füße), Pedale, Hülle, Rollbrett (Deck, Teppich, Leisten) und Lenkrollen."),
        t("Structure: members are the deck, the base, the column shaft, the neck, the soundbox back, every second string (slender), the seven pedal bars and the cover; joints are the feet, the soundbox foot, the column foot, the castor yokes and wheels and blocks where the neck meets the column and the soundbox; supports are pads under the four castors; masses mark the centres of the trolley, base, column, neck, soundbox, strings and cover.", "Tragwerk: Stäbe sind Deck, Fuß, Säulenschaft, Hals, Schale, jede zweite Saite (schlank), die sieben Pedalstäbe und die Hülle; Gelenke sind die Füße, der Fuß des Resonanzkörpers, der Säulenfuß, die Gabeln und Räder der Lenkrollen und Blöcke, wo der Hals auf Säule und Resonanzkörper trifft; Auflager sind Klötze unter den vier Lenkrollen; Massen markieren die Schwerpunkte von Rollbrett, Fuß, Säule, Hals, Resonanzkörper, Saiten und Hülle."),
    ],
    100: [
        t("Body: as 1:50 with the carpet in a 23 mm deck, the moulding in the base, plate and yoke of each castor in one 18 mm block, pedals as 15 x 18 mm bars, a 16 mm soundboard layer that takes in the centre strip, simplified turned profiles of foot and crown, the cover on 20 outline points and the 14 C and F strings.", "Körper: wie 1:50 mit dem Teppich in einem 23-mm-Deck, dem Profil im Fuß, Platte und Gabel jeder Lenkrolle in einem 18-mm-Block, Pedalen als Stäbe 15 x 18 mm, einer 16-mm-Deckenschicht, die die Saitenleiste aufnimmt, vereinfachten Drehprofilen von Fuß und Kapitell, der Hülle auf 20 Umrisspunkten und den 14 C- und F-Saiten."),
        t("Thermal: the body by reference, plus the 33 strings the body leaves out as thermal-only slender capsules, so the strings keep their whole surface.", "Thermisch: der Körper als Verweis, dazu die 33 Saiten, die der Körper weglässt, als rein thermische schlanke Kapseln, damit die Saiten ihre ganze Oberfläche behalten."),
        t("Structure: as 1:50 with every fourth string.", "Tragwerk: wie 1:50 mit jeder vierten Saite."),
    ],
    200: [
        t("Body: base down to the deck (feet merged), the 7 pedals typified as two blocks per foot side, deck with rims and carpet as one 43 mm slab, castor yokes 30 mm and wheels 30 mm wide, a 32 mm soundboard layer, two-segment foot and crown, the cover on 14 points and the 7 C strings.", "Körper: Fuß bis aufs Deck (Füße zusammengefasst), die 7 Pedale typisiert als zwei Blöcke je Fußseite, Deck mit Leisten und Teppich als eine 43-mm-Platte, Lenkrollengabeln 30 mm und Räder 30 mm breit, eine 32-mm-Deckenschicht, zweiteiliger Fuß und Kapitell, die Hülle auf 14 Punkten und die 7 C-Saiten."),
        t("Thermal: the body by reference plus the other 40 strings as slender capsules.", "Thermisch: der Körper als Verweis und die übrigen 40 Saiten als schlanke Kapseln."),
        t("Structure: the pedals keep their own slender bars of true section on their pivots; the base sits on the deck through a joint plate; every seventh string.", "Tragwerk: die Pedale behalten auf ihren Drehpunkten eigene schlanke Stäbe in wahrem Querschnitt; der Fuß sitzt über eine Gelenkplatte auf dem Deck; jede siebte Saite."),
    ],
    500: [
        t("Body: trolley and castors as one block from the floor to the rim top, the base with the pedal band in one prism, the column as one turned part with crown and foot (76 mm shaft), the neck 76 mm thick, the soundbox as a 75 mm soundboard layer on the back, the cover on 10 points; strings, feet, rims and wheels are below the 75 mm feature and left out or merged.", "Körper: Rollbrett und Lenkrollen als ein Block vom Boden bis zur Leistenoberkante, der Fuß mit dem Pedalband in einem Prisma, die Säule als ein gedrehtes Teil mit Kapitell und Fuß (Schaft 76 mm), der Hals 76 mm dick, der Resonanzkörper als 75-mm-Deckenschicht auf der Schale, die Hülle auf 10 Punkten; Saiten, Füße, Leisten und Räder liegen unter der kleinsten Einzelheit von 75 mm und entfallen oder gehen auf."),
        t("Thermal: the body by reference plus the surfaces the body merges, each at the size floor: 16 strings as slender capsules, the two brass plates as sheets on the thickened neck, the gilded crown, a pedal block and four castor blocks inside the trolley block.", "Thermisch: der Körper als Verweis und die Oberflächen, die der Körper aufnimmt, jede in Mindestgröße: 16 Saiten als schlanke Kapseln, die beiden Messingplatten als dünne Platten am verdickten Hals, das vergoldete Kapitell, ein Pedalblock und vier Lenkrollenblöcke im Rollbrettblock."),
        t("Structure: the trolley block, base, column, neck, soundbox and cover as members, the pedals as slender bars, every fifteenth string, joint blocks under the castors, at the column foot and at both ends of the neck.", "Tragwerk: Rollbrettblock, Fuß, Säule, Hals, Resonanzkörper und Hülle als Stäbe, die Pedale als schlanke Stäbe, jede fünfzehnte Saite, Gelenkblöcke unter den Lenkrollen, am Säulenfuß und an beiden Enden des Halses."),
    ],
}


def asset(slug, bones, rig, reps, vols, cover_twist, cover_pivot, dims, mass_at, cover_hw_u):
    """🏺️ The whole asset."""
    base = poses(cover_twist)
    return {
        "id": slug,
        "category": "objects",
        "title": t("Used Pedal Harp", "Gebrauchte Konzertharfe"),
        "summary": t("A used concert pedal harp of 1.85 m and 36 kg with 47 strings C1 to G7 and seven pedals, its gilding rubbed through on the crown, the lacquer worn at the shoulder and the varnish yellowed, stored on a carpeted castor trolley under a faded, taped padded cover; rolled on the trolley, stood uncovered on the floor, tilted 18 degrees back onto the harpist's shoulder and played, with the D pedal moved from natural into sharp.", "Eine gebrauchte Konzert-Pedalharfe von 1,85 m und 36 kg mit 47 Saiten C1 bis G7 und sieben Pedalen, die Vergoldung am Kapitell durchgerieben, der Lack an der Schulter abgegriffen und die Decke vergilbt, gelagert auf einem teppichbelegten Rollbrett unter einer ausgeblichenen, geflickten gepolsterten Hülle; auf dem Rollbrett gerollt, enthüllt auf den Boden gestellt, um 18 Grad auf die Schulter der Harfenistin gekippt und gespielt, mit dem D-Pedal von der Auflösung ins Kreuz."),
        "tags": ["used", "instrument", "music", "second-hand", "strings", "harp", "pedal-harp", "concert-harp", "chordophone", "maple", "spruce", "gilded", "brass", "trolley", "cover"],
        "provenance": {"author": "Semio staffage instrument authors", "method": "handcrafted", "created": "2026-11-22", "references": REFERENCES, "notes": t("A concert grand pedal harp of the class (Hornbostel-Sachs 322.222): 1.85 m high on its base, 1.10 m from the neck scroll to the front of the base, base 590 x 430 mm, soundbox 1.46 m long tapering from 450 to 130 mm, 47 strings, seven double-action pedals, 36 kg; the harp trolley 680 x 500 mm on 50 mm castors and the padded cover are typical dealer equipment. Every value is catalogue-typical or from the literature cited (authored-estimate or literature), nothing is measured.", "Eine Konzert-Pedalharfe der Klasse (Hornbostel-Sachs 322.222): 1,85 m hoch auf ihrem Fuß, 1,10 m von der Halsschnecke bis zur Vorderkante des Fußes, Fuß 590 x 430 mm, Resonanzkörper 1,46 m lang, verjüngt von 450 auf 130 mm, 47 Saiten, sieben Doppelpedale, 36 kg; das Harfenrollbrett 680 x 500 mm auf 50-mm-Lenkrollen und die gepolsterte Hülle sind übliche Händlerausstattung. Jeder Wert ist katalogtypisch oder aus der genannten Literatur (authored-estimate oder literature), nichts ist gemessen.")},
        "dimensions": dict(dims, massKg=44.79),
        "materials": MATERIALS,
        "rig": {"kind": "mechanical", "root": "floor", "bones": rig_bones(bones, mass_at)},
        "anchors": anchors(rig, cover_hw_u),
        "representations": reps,
        "symbols": [],
        "poses": base + key_poses(base),
        "clips": clips(),
        "thermal": thermal(AREAS),
        "structure": STRUCTURE,
        "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "tilt-to-shoulder", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
