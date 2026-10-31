"""japanese-maple: small multi-stem Acer palmatum with layered horizontal tiers."""
import math
from lib import *
from common import *
from reps import *
from gen import *
from sp_plane import Sp, common_refs, tree_thermal_assumptions, tree_structure_assumptions
from sp_lime import on


def maple_leaf(cx, cy, R, rot=0.0):
    pts = []
    for i in range(14):
        a = (90 + 360 / 14 * i) * D2R
        r = R if i % 2 == 0 else R * 0.42
        pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return rot_pts(pts, cx, cy, rot)


def build():
    S = Sp()
    S.id = "japanese-maple"
    S.folder = "japanese-maple"
    S.title = T("Japanese Maple, Multi-Stem", "Fächerahorn, mehrstämmig")
    S.summary = T("A mature small Japanese maple (Acer palmatum) of three slender stems whose near-horizontal branches carry the foliage in layered, flat tiers; the crown is wider than it is high.",
                  "Ein ausgewachsener kleiner Fächerahorn (Acer palmatum) aus drei schlanken Stämmen, deren fast waagerechte Äste das Laub in geschichteten, flachen Etagen tragen; die Krone ist breiter als hoch.")
    S.tags = ["tree", "deciduous", "small-tree", "multi-stem", "layered-tiers", "garden", "wind", "snow", "transpiration"]
    S.depth_m = 5.2
    S.footprint = 1.2
    S.density = 900
    S.root_mass_factor = 1.3
    S.default_scale = 100
    S.default_pose = "summer-calm"
    S.breeze_pose = "summer-breeze"
    S.breeze_v10 = 5.5
    S.gust_peak = 14.0
    S.trunk_bone = "stem-c"
    S.twig_factor = {"leaf-sun": 1.4, "leaf-shade": 1.4}
    S.cd = {}
    S.bark_extra = {}
    r = Rig()
    S.rig = r
    W = "maple-wood"
    r.add("root-plate", None, (0, 0), 90, 0.25, T("Root collar", "Wurzelhals"), 0.55, 0.40, limit=0, material=W).region = "stem-bark"
    r.add("stem-c", "root-plate", (0.0, 0.25), 88, 2.7, T("Central stem", "Mittelstamm"), 0.17, 0.09, limit=5, material=W).region = "stem-bark"
    r.add("stem-w", "root-plate", (-0.03, 0.2), 112, 2.4, T("West stem", "Weststamm"), 0.15, 0.08, limit=6, material=W).region = "stem-bark"
    r.add("stem-e", "root-plate", (0.04, 0.2), 66, 2.6, T("East stem", "Oststamm"), 0.14, 0.08, limit=6, material=W).region = "stem-bark"
    r.add("stem-c-top", "stem-c", r.tip("stem-c"), 90, 1.0, T("Central stem, top", "Mittelstamm, Spitze"), 0.09, 0.04, limit=8, material=W).region = "stem-bark"
    TIERS = [("t1w", "stem-w", 1.3, 172, 1.9, "Tier 1 west branch", "Etage 1, Ast West"), ("t1e", "stem-e", 1.5, 8, 1.9, "Tier 1 east branch", "Etage 1, Ast Ost"),
             ("t2w", "stem-w", 2.0, 170, 1.7, "Tier 2 west branch", "Etage 2, Ast West"), ("t2e", "stem-e", 2.2, 10, 1.6, "Tier 2 east branch", "Etage 2, Ast Ost"),
             ("t3w", "stem-c", 2.5, 165, 1.3, "Tier 3 west branch", "Etage 3, Ast West"), ("t3e", "stem-c", 2.6, 15, 1.3, "Tier 3 east branch", "Etage 3, Ast Ost")]
    for id, par, s, ang, ln, te, td in TIERS:
        r.add(id, par, on(r, par, s), ang, ln, T(te, td), 0.06, 0.03, limit=14, material=W).region = "limb-bark"
    S.levels = {"stem-c": "stem", "stem-w": "stem", "stem-e": "stem", "stem-c-top": "stemtop"}
    for id, *_ in TIERS:
        S.levels[id] = "tier"
    S.lean_ref = {"stem": 0.4, "stemtop": 0.8, "tier": 1.6, "fol": 3.5}
    S.droop_levels = {"stem": 0.2, "tier": 3.0, "fol": 4.0, "stemtop": 0.5}
    S.sway_amp = {"stem": 0.5, "stemtop": 0.55, "tier": 0.6, "fol": 0.7}
    S.sway_lag = 0.5
    S.sway_period = 1.8
    S.gust_tau = {"stem": 0.45, "stemtop": 0.35, "tier": 0.3, "fol": 0.2}
    S.sway_desc_en = "A deterministic %s s sway cycle about the breeze lean (5.5 m/s at 10 m; estimated first mode of the 4.5 m multi-stem tree about 0.55 Hz). The long thin tier branches bob with a phase lag behind the stems; the flat foliage pads swing about 3 degrees."
    S.sway_desc_de = "Ein deterministischer Schwingungszyklus von %s s um die Neigung bei Brise (5,5 m/s in 10 m Höhe; geschätzte erste Eigenform des 4,5 m hohen mehrstämmigen Baums etwa 0,55 Hz). Die langen dünnen Etagenäste wippen phasenverzögert hinter den Stämmen; die flachen Laubpolster schwingen etwa 3 Grad."
    S.gust_desc_en = "A single gust over 8 s from %.1f to %.1f m/s at 10 m: the wind-load tracks rise with the reconfigured drag law and the tier branches lean downwind with a level-wise lag, the stems last."
    S.gust_desc_de = "Eine einzelne Böe über 8 s von %.1f auf %.1f m/s in 10 m Höhe: Die Windlast-Spuren steigen mit dem umgeformten Widerstandsgesetz, die Etagenäste neigen sich ebenenweise verzögert in Windrichtung, die Stämme zuletzt."
    S.masses = [
        Mass("fol-core-low", "stem-c", (0.0, 1.95), 0.9, 0.7, 0, "leaf-shade", "shade", T("shaded lower core", "beschatteter Kern unten")),
        Mass("fol-core-up", "stem-c", (0.05, 3.05), 0.9, 0.5, 0, "leaf-shade", "shade", T("shaded upper core", "beschatteter Kern oben")),
        Mass("fol-t1w", "t1w", (-1.45, 1.65), 1.2, 0.34, 2, "leaf-sun", "sun", T("tier 1 west foliage pad", "Laubpolster Etage 1 West")),
        Mass("fol-t1e", "t1e", (1.5, 1.7), 1.25, 0.34, -2, "leaf-sun", "sun", T("tier 1 east foliage pad", "Laubpolster Etage 1 Ost")),
        Mass("fol-t2w", "t2w", (-1.45, 2.45), 1.15, 0.34, 3, "leaf-sun", "sun", T("tier 2 west foliage pad", "Laubpolster Etage 2 West")),
        Mass("fol-t2e", "t2e", (1.5, 2.55), 1.1, 0.34, -3, "leaf-sun", "sun", T("tier 2 east foliage pad", "Laubpolster Etage 2 Ost")),
        Mass("fol-t3w", "t3w", (-0.6, 3.2), 0.95, 0.34, 4, "leaf-sun", "sun", T("tier 3 west foliage pad", "Laubpolster Etage 3 West")),
        Mass("fol-t3e", "t3e", (0.65, 3.3), 0.95, 0.34, -4, "leaf-sun", "sun", T("tier 3 east foliage pad", "Laubpolster Etage 3 Ost")),
        Mass("fol-top", "stem-c-top", (0.05, 3.9), 1.0, 0.55, 0, "leaf-sun", "sun", T("crown top foliage", "Laubmasse Kronenspitze")),
    ]
    proj = math.pi / 4 * 5.6 * S.depth_m
    S.leaf_area = {"leaf-sun": 2.2 * proj, "leaf-shade": 1.6 * proj}
    S.mat_of = {"stem-bark": "maple-bark", "limb-bark": "maple-bark-young", "leaf-sun": "maple-leaf-sun", "leaf-shade": "maple-leaf-shade"}
    S.tone = {"wood": 0.4, "sun": 0.6, "shade": 0.45}
    S.tone_line = 0.2
    S.sun = {"stem-bark": 0.3, "limb-bark": 0.35, "leaf-sun": 0.85, "leaf-shade": 0.08}
    S.regions = [
        {"id": "stem-bark", "title": T("Smooth grey-brown bark of the three stems", "Glatte graubraune Rinde der drei Stämme"), "kind": "bark", "model": "passive", "material": "maple-bark", "len": 0.14, "geom": "cylinder", "sky": 0.5, "hc": 30000, "ref": "wood-handbook-2021"},
        {"id": "limb-bark", "title": T("Bark of the thin tier branches", "Rinde der dünnen Etagenäste"), "kind": "bark", "model": "passive", "material": "maple-bark-young", "len": 0.05, "geom": "cylinder", "sky": 0.6, "hc": 15000, "ref": "wood-handbook-2021"},
        {"id": "leaf-sun", "title": T("Sun leaves of the outer tiers", "Sonnenblätter der äußeren Etagen"), "kind": "foliage", "model": "leaf", "material": "maple-leaf-sun", "area": S.leaf_area["leaf-sun"], "len": 0.07, "geom": "horizontal-plate-up", "sky": 0.75, "hc": 800, "gs": 0.0030, "lai": 2.2, "ref": "jones-2014"},
        {"id": "leaf-shade", "title": T("Shade leaves of the inner tiers", "Schattenblätter der inneren Etagen"), "kind": "foliage", "model": "leaf", "material": "maple-leaf-shade", "area": S.leaf_area["leaf-shade"], "len": 0.08, "geom": "horizontal-plate-up", "sky": 0.3, "hc": 600, "gs": 0.0018, "lai": 1.6, "ref": "jones-2014"},
    ]
    sunm = [m for m in S.masses if m.kind == "sun"]
    S.emitters = [
        {"id": "transpiration-sun", "aid": "tp-sun", "region": "leaf-sun", "strength": 1.0, "diam": 1.4, "title": T("Transpiration of the sun leaves", "Transpiration der Sonnenblätter"), "anchors": [(m.id, (m.c[0], m.c[1] + 0.1)) for m in sunm]},
        {"id": "transpiration-shade", "aid": "tp-shade", "region": "leaf-shade", "strength": 0.7, "diam": 1.1, "title": T("Transpiration of the shade leaves", "Transpiration der Schattenblätter"),
         "anchors": [("fol-core-low", (-0.4, 1.8)), ("fol-core-low", (0.4, 2.1)), ("fol-core-low", (0.0, 2.35)), ("fol-core-up", (-0.3, 2.9)), ("fol-core-up", (0.3, 3.15))]},
    ]
    S.materials = [
        {"id": "maple-bark", "title": T("Maple stem bark, smooth with lenticels", "Ahornstammrinde, glatt mit Lentizellen"), "emissivity": 0.95, "solarAbsorptivity": 0.65, "densityKgM3": 600, "conductivityWmK": 0.12, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "maple-bark-young", "title": T("Bark of thin branches", "Rinde dünner Äste"), "emissivity": 0.95, "solarAbsorptivity": 0.6, "densityKgM3": 650, "conductivityWmK": 0.13, "source": "authored-estimate", "reference": "wood-handbook-2021"},
        {"id": "maple-leaf-sun", "title": T("Maple sun leaves (green)", "Ahorn-Sonnenblätter (grün)"), "emissivity": 0.97, "solarAbsorptivity": 0.5, "source": "literature", "reference": "jones-2014"},
        {"id": "maple-leaf-shade", "title": T("Maple shade leaves (effective)", "Ahorn-Schattenblätter (wirksam)"), "emissivity": 0.97, "solarAbsorptivity": 0.25, "source": "authored-estimate", "reference": "jones-2014"},
        {"id": W, "title": T("Green maple wood", "Frisches Ahornholz"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 900, "conductivityWmK": 0.16, "elasticModulusPa": 7500000000, "strengthPa": 45000000, "source": "authored-estimate", "reference": "wood-handbook-2021"},
    ]
    S.references = common_refs()
    S.notes = T("Rig and drawings are authored by hand for a 35-year-old garden specimen: three stems from one root collar, three tiers of near-horizontal branches at about 1.7, 2.5 and 3.3 m and a leader tip to 4.4 m; the crown is 5.6 m wide. Sizes are typical for mature Acer palmatum specimens in gardens, wood properties, masses and conductances are authored estimates. Autumn colour is not geometric; the autumn pose changes the thermal binding only.",
                 "Rig und Zeichnungen sind von Hand für ein 35-jähriges Gartenexemplar entworfen: drei Stämme aus einem Wurzelhals, drei Etagen fast waagerechter Äste bei etwa 1,7, 2,5 und 3,3 m und eine Wipfelspitze bis 4,4 m; die Krone ist 5,6 m breit. Die Größen sind für ausgewachsene Gartenexemplare von Acer palmatum typisch, Holzeigenschaften, Massen und Leitfähigkeiten sind Autorenschätzungen. Herbstfärbung ist nicht geometrisch; die Herbstpose ändert nur die thermische Bindung.")
    S.environment = {"airTempC": 22, "meanRadiantTempC": 26, "relativeHumidity": 0.55, "windMS": 1.0, "windDirectionDeg": 0, "solarWM2": 650, "pressurePa": 101325, "diffuseFraction": 0.2}
    S.thermal_assumptions = tree_thermal_assumptions("", "")
    S.structure_assumptions = tree_structure_assumptions("maple", "Ahorn")
    S.poses = [
        {"id": "summer-calm", "title": T("Calm summer day", "Windstiller Sommertag"), "desc": T("Light air of 1.5 m/s at 10 m: the flat tiers stand at rest, the upper pads are lit, the lower tiers sit in the shade of the upper ones; transpiration at 80 % of the daytime maximum.", "Leiser Zug von 1,5 m/s in 10 m Höhe: Die flachen Etagen stehen in Ruhe, die oberen Polster sind beleuchtet, die unteren liegen im Schatten der oberen; Transpiration bei 80 % des Tagesmaximums."), "v10": 1.5, "sun": 1.0, "emit": 0.8},
        {"id": "summer-breeze", "title": T("Summer breeze", "Sommerbrise"), "desc": T("Moderate breeze of 5.5 m/s at 10 m (Beaufort 4) in an open garden: the tier branches lean downwind; drag by the reconfigured law with cd 0.35.", "Mäßige Brise von 5,5 m/s in 10 m Höhe (Beaufort 4) in einem offenen Garten: Die Etagenäste neigen sich in Windrichtung; Widerstand nach dem umgeformten Gesetz mit cd 0,35."), "v10": 5.5, "sun": 1.0, "emit": 0.95},
        {"id": "autumn-colour", "title": T("Autumn colour, stomata closing", "Herbstfärbung, Stomata schließen"), "desc": T("Dry, sunny autumn day: the leaves have turned red and the stomata close, transpiration at 25 %; the red leaves absorb a little more of the sun, which is not modelled by a pose, so only the lower transpiration and the shorter day (sun 70 %) are bound.", "Trockener, sonniger Herbsttag: Die Blätter sind rot gefärbt und die Stomata schließen sich, Transpiration bei 25 %; die roten Blätter absorbieren etwas mehr Sonne, was eine Pose nicht abbilden kann, daher sind nur die geringere Transpiration und der kürzere Tag (Sonne 70 %) gebunden."), "v10": 2.0, "sun": 0.7, "emit": 0.25},
        {"id": "autumn-snow", "title": T("Early snow on the tiers", "Früher Schnee auf den Etagen"), "desc": T("Wet early snow of 12 kg per m2 of foliage plan area (order of magnitude from EN 1991-1-3; the flat pads catch more snow than a rounded crown) bends the long thin tier branches down; calm, no sun, stomata closed.", "Nasser früher Schnee von 12 kg je m2 Laubgrundfläche (Größenordnung nach EN 1991-1-3; die flachen Polster fangen mehr Schnee als eine runde Krone) biegt die langen dünnen Etagenäste nach unten; windstill, keine Sonne, Stomata geschlossen."), "v10": 1.0, "sun": 0.05, "emit": 0.05, "wet": 1.0, "droop": 1.0, "extra": snow_extra(12.0)},
        {"id": "storm", "title": T("Storm with rain", "Sturm mit Regen"), "desc": T("Beaufort 9, 22 m/s at 10 m with rain in an exposed garden: strong downwind lean, drag with the reconfiguration reduction (cd 0.35, (v/8)^-0.6 above 8 m/s), wet bark and leaves, overcast sky.", "Beaufort 9, 22 m/s in 10 m Höhe mit Regen in einem exponierten Garten: starke Neigung in Windrichtung, Widerstand mit der Umformungsminderung (cd 0,35, (v/8)^-0,6 über 8 m/s), nasse Rinde und Blätter, bedeckter Himmel."), "v10": 22.0, "sun": 0.15, "emit": 0.5, "wet": 0.6},
    ]
    S.make_reps = make_reps
    S.make_symbols = make_symbols
    return S


def stem_shapes(S, sc):
    r = S.rig
    out = []
    n = {"200": 5, "100": 6, "50": 8}[sc]
    rp = r.bones["root-plate"]
    wobs = {"200": [1.0, 1.1, 1.0], "100": [1.0, 1.15, 1.05, 0.98], "50": [1.0, 1.2, 1.1, 1.0, 0.96]}[sc]
    out.append(tapered(S, rp, sc, n, [1.0, 0.9, 0.85, 0.8, 0.8, 0.8, 0.8, 0.8][:n], 1.0, id="root-collar"))
    for i, id in enumerate(["stem-c", "stem-w", "stem-e"]):
        b = r.bones[id]
        out.append(tapered(S, b, sc, n, [wobs[(i + k) % len(wobs)] for k in range(n)], 1.2 + 0.01 * i, id="wood-" + id, s1=[1.3, 1.65, 1.6][i]))
    if sc == "50":
        for i, (id, s, off) in enumerate([("stem-c", 0.5, 0.03), ("stem-c", 1.1, -0.03), ("stem-w", 0.6, 0.0), ("stem-w", 1.2, 0.03), ("stem-e", 0.7, -0.03), ("stem-e", 1.4, 0.0)]):
            b = r.bones[id]
            c = b.to_world((s, off))
            c2 = b.to_world((s + 0.07, off))
            out.append(Shape("lenticel-%d" % (i + 1), b, "texture", cr_open([c, c2]), False, 0.15, 1.5))
    if sc == "100":
        for i, (id, s, off) in enumerate([("stem-c", 0.6, 0.0), ("stem-w", 0.9, 0.0)]):
            b = r.bones[id]
            out.append(Shape("bark-line-%d" % (i + 1), b, "texture", cr_open(wavy(b, s, s + 0.5, off, 0.01, 1.0, n=5)), False, 0.15, 1.5))
    return out


def extras(S, sc):
    r = S.rig
    out = []
    for i, id in enumerate(["t1w", "t1e", "t2w", "t2e", "t3w", "t3e"]):
        b = r.bones[id]
        if sc == "100":
            out.append(centre_stroke(S, b, sc, "detail", 8 + 0.01 * i, n=4, id="tier-" + id))
        if sc == "50":
            pts = limb_outline(b, stations=7, wob=WOBS[i % 3])
            out.append(Shape("tier-" + id, b, "texture", cr_closed(pts), False, 0.2, 8 + 0.01 * i, region="limb-bark"))
    if sc == "200":
        pass
    return out


def make_reps(S):
    C = dict(
        stem_bones=["root-plate", "stem-c", "stem-w", "stem-e", "stem-c-top"], no_wood={"200": ("t1w", "t1e", "t2w", "t2e", "t3w", "t3e"), "100": ("t1w", "t1e", "t2w", "t2e", "t3w", "t3e"), "50": ("t1w", "t1e", "t2w", "t2e", "t3w", "t3e")},
        min_wood_mm={"200": 0.5, "100": 0.45, "50": 0.4}, min_stroke_wood_mm={"100": 99.0, "50": 99.0},
        mass={
            "200": dict(lobe=1.0, amp=0.1, vary=[1.0, 0.75, 1.15, 0.85, 1.05], under=0.35, pointed=0.7, n=5, rx_s=0.97, ry_s=0.97, phase=0.25, dz=0.2),
            "100": dict(lobe=0.65, amp=0.14, vary=[1.0, 0.7, 1.15, 0.8, 1.0, 0.9], under=0.3, pointed=0.6, n=6, rx_s=0.95, ry_s=0.95, phase=0.5, dz=0.3),
            "50": dict(lobe=0.4, amp=0.16, vary=[1.0, 0.65, 1.15, 0.8, 1.05, 0.7, 1.1], under=0.25, pointed=0.55, n=7, rx_s=0.95, ry_s=0.95, phase=0.3, dz=0.4),
        },
        mass_regions={"200": {k: ["leaf-sun", "limb-bark"] for k in ("fol-t1w", "fol-t1e", "fol-t2w", "fol-t2e", "fol-t3w", "fol-t3e")}}, glyph=maple_leaf, glyph_kinds=("sun",), glyph_R=0.045, stems=stem_shapes, extras=extras, shapes500=shapes500, words=WORDS,
    )
    return make_reps_generic(S, C)


CROWN500 = [(0, 1.45), (1.0, 1.33), (2.2, 1.3), (2.8, 1.5), (2.9, 1.75), (2.75, 2.0), (2.25, 2.08), (2.05, 2.17), (2.5, 2.32), (2.65, 2.55), (2.55, 2.8), (2.05, 2.88), (1.85, 2.97), (1.88, 3.2), (1.7, 3.48), (1.3, 3.58), (1.0, 3.7), (0.95, 4.0), (0.6, 4.4), (0, 4.5), (-0.6, 4.4), (-0.95, 4.0), (-1.0, 3.7), (-1.35, 3.58), (-1.75, 3.48), (-1.9, 3.2), (-1.85, 2.97), (-2.1, 2.88), (-2.55, 2.8), (-2.65, 2.55), (-2.5, 2.32), (-2.1, 2.17), (-2.3, 2.08), (-2.8, 2.0), (-2.9, 1.75), (-2.75, 1.5), (-2.2, 1.3), (-1.0, 1.33)]


def shapes500(S):
    r = S.rig
    c = r.bones["stem-c"]
    st = [(-0.25, 0.3), (-0.15, 0.24), (0.0, 0.2), (0.4, 0.17), (1.0, 0.155), (1.6, 0.15)]
    stem = Shape("stems", c, "body", cr_closed(stem_pts(c, st, [0.0, 0.004])), True, 0.4, 1, region="stem-bark", material="maple-bark")
    crown = Shape("crown", c, "body", cr_closed([(x * 0.945, y) for x, y in CROWN500]), True, 0.55, 3, regions=["leaf-sun", "leaf-shade", "limb-bark"], material="maple-leaf-sun")
    return [stem, crown]


WORDS = {
    "s500_sil": ("The Japanese maple is composed as one closed, wider-than-high crown whose outline steps in three notched tiers and a rounded top, above a slender fused stem group; the notches are 0.3 mm deep slits between the tiers at 1 m = 2 mm.", "Der Fächerahorn ist als eine geschlossene, breitere als hohe Krone komponiert, deren Umriss in drei gekerbten Etagen und einer gerundeten Spitze abgestuft ist, über einer schlanken verschmolzenen Stammgruppe; die Kerben sind bei 1 m = 2 mm 0,3 mm tiefe Schlitze zwischen den Etagen."),
    "s500_ret": ("Retained: the tiered outline with its notches and the stem group as one tapered form.", "Beibehalten: der gestufte Umriss mit seinen Kerben und die Stammgruppe als eine verjüngte Form."),
    "s500_mer": ("Merged into the crown path: seven sun foliage pads, both shade cores and all tier branches; the three stems are fused into one form (the real stems are 0.3 mm wide with gaps below the minimum).", "Im Kronenpfad vereint: sieben Sonnenlaubpolster, beide Schattenkerne und alle Etagenäste; die drei Stämme sind zu einer Form verschmolzen (die wahren Stämme sind 0,3 mm breit mit Abständen unter dem Minimum)."),
    "s500_omi": ("Omitted: single branches, leaves, bark lines and the gaps between the three stems; no internal edge.", "Weggelassen: einzelne Äste, Blätter, Rindenlinien und die Lücken zwischen den drei Stämmen; keine innere Kante."),
    "s500_thin": ("Tier branches of 0.03 to 0.06 m are not drawn; they are merged into the filled crown, so the outline stays within the real outline tolerance of the 1:50 drawing.", "Etagenäste von 0,03 bis 0,06 m werden nicht gezeichnet; sie sind in die gefüllte Krone eingeschmolzen, so dass der Umriss innerhalb der realen Umrisstoleranz der 1:50-Zeichnung bleibt."),
    "s500_ovl": ("Overlays map onto: stem-bark (fused stems) and, merged in the crown path, leaf-sun, leaf-shade and limb-bark; merged shapes show the area-weighted value of their regions, labelled as merged.", "Überlagerungen bilden sich ab auf: stem-bark (verschmolzene Stämme) sowie, im Kronenpfad vereint, leaf-sun, leaf-shade und limb-bark; vereinte Formen zeigen den flächengewichteten Wert ihrer Regionen, als vereint gekennzeichnet."),
    "s200_sil": ("At 1:200 (1 m is 5 mm) the maple is separated into its main masses: the root collar and three diverging stems, and nine flat foliage pads (seven sun pads in three tiers and a top, two shade cores) as scalloped, horizontally elongated outlines with a clear white gap between the tiers.", "Bei 1:200 (1 m sind 5 mm) wird der Ahorn in seine Hauptmassen aufgeteilt: Wurzelhals und drei auseinanderlaufende Stämme sowie neun flache Laubpolster (sieben Sonnenpolster in drei Etagen und eine Spitze, zwei Schattenkerne) als gewellte, waagerecht gestreckte Umrisse mit deutlichem weißem Zwischenraum zwischen den Etagen."),
    "s200_ret": ("Retained: the three stems and the root collar as filled tapered outlines; every foliage pad.", "Beibehalten: die drei Stämme und der Wurzelhals als gefüllte verjüngte Umrisse; jedes Laubpolster."),
    "s200_mer": ("Merged into the pad outlines: leaf clusters, single leaves and the tier branches (they lie inside the pads).", "In den Polsterumrissen vereint: Blattbüschel, einzelne Blätter und die Etagenäste (sie liegen innerhalb der Polster)."),
    "s200_omi": ("Omitted: the tier branches (0.15 to 0.3 mm), twigs, lenticels and leaf marks and all texture strokes.", "Weggelassen: die Etagenäste (0,15 bis 0,3 mm), Zweige, Lentizellen und Blattzeichen sowie alle Texturstriche."),
    "s200_thin": ("Tier branches would be 0.15 to 0.3 mm wide and are not drawn as filled outlines; the stems keep their real width (0.45 to 0.85 mm).", "Etagenäste wären 0,15 bis 0,3 mm breit und werden nicht als gefüllte Umrisse gezeichnet; die Stämme behalten ihre reale Breite (0,45 bis 0,85 mm)."),
    "edge200": ("none; the drawing has no internal strokes.", "keine; die Zeichnung hat keine inneren Striche."),
    "s200_ovl": ("Every thermal region keeps a shape: stems and root collar (stem-bark), pads and cores; limb-bark lies inside the pads and is drawn from 1:100.", "Jede thermische Region behält eine Form: Stämme und Wurzelhals (stem-bark), Polster und Kerne; limb-bark liegt innerhalb der Polster und wird ab 1:100 gezeichnet."),
    "s100_sil": ("At 1:100 (1 m is 10 mm) the pads get finer lobes and the layered structure appears: the branches of the six tiers are drawn as strokes through the pads, with two leaf-cluster arcs per pad, bark lines on the stems and a clear gap between tiers.", "Bei 1:100 (1 m sind 10 mm) erhalten die Polster feinere Lappen und die Schichtstruktur erscheint: die Äste der sechs Etagenansätze werden als Striche durch die Polster gezeichnet, mit zwei Blattbüschelbögen je Polster, Rindenlinien an den Stämmen und deutlichem Zwischenraum zwischen den Etagen."),
    "s100_ret": ("Retained: nine foliage pads, three stems with root collar and leader tip, six tier branches as centre strokes.", "Beibehalten: neun Laubpolster, drei Stämme mit Wurzelhals und Wipfelspitze, sechs Etagenäste als Mittelstriche."),
    "s100_mer": ("Single leaves and twig ends are merged into lobed pad outlines; bark relief into two lines.", "Einzelne Blätter und Zweigenden sind in gelappte Polsterumrisse eingeschmolzen; Rindenrelief in zwei Linien."),
    "s100_omi": ("Omitted: individual leaves, lenticels and fine twigs; the fine 1:50 leaf marks are replaced by two arcs per pad.", "Weggelassen: einzelne Blätter, Lentizellen und feine Zweige; die feinen 1:50-Blattzeichen sind durch zwei Bögen je Polster ersetzt."),
    "s100_thin": ("The tier branches (0.3 to 0.6 mm) are drawn as single 0.13 mm centre strokes and carry the limb-bark region as open strokes; stems keep their real width.", "Die Etagenäste (0,3 bis 0,6 mm) werden als einzelne 0,13-mm-Mittelstriche gezeichnet und tragen die Region limb-bark als offene Striche; Stämme behalten ihre reale Breite."),
    "s100_ovl": ("Every region keeps its shapes; the shade hatch is a pure drawing hatch and does not stand for temperature.", "Jede Region behält ihre Formen; die Schattenschraffur ist eine reine Zeichenschraffur und steht nicht für Temperatur."),
    "s50_sil": ("At 1:50 (1 m is 20 mm) the maple is drawn in full: three stems with lenticel marks, the six tier branches as outlines seen through the foliage, finely lobed flat pads with leaf-cluster arcs and seven-lobed palmate leaf marks at the real leaf size.", "Bei 1:50 (1 m sind 20 mm) wird der Ahorn vollständig gezeichnet: drei Stämme mit Lentizellenzeichen, die sechs Etagenäste als durch das Laub gesehene Umrisse, fein gelappte flache Polster mit Blattbüschelbögen und siebenlappigen Blattzeichen in wahrer Blattgröße."),
    "s50_ret": ("Retained: every woody member of the rig and every foliage pad; the leaf marks show the few largest leaves per pad (about 0.09 m).", "Beibehalten: jedes Holzglied des Rigs und jedes Laubpolster; die Blattzeichen zeigen die wenigen größten Blätter je Polster (etwa 0,09 m)."),
    "s50_mer": ("Only the plug overlaps at the bone joints close the seams between tapered woody parts.", "Nur die Überlappungen an den Knochengelenken schließen die Nähte zwischen verjüngten Holzteilen."),
    "s50_omi": ("Omitted: the thousand single leaves (1.8 mm on paper) and buds.", "Weggelassen: die tausend einzelnen Blätter (1,8 mm auf dem Papier) und Knospen."),
    "s50_thin": ("The tier branches (0.6 to 1.2 mm) are drawn as unfilled outlines at their real width; lenticels, arcs and leaf marks are 0.13 to 0.18 mm strokes, all above the pen minimum.", "Die Etagenäste (0,6 bis 1,2 mm) werden in realer Breite als ungefüllte Umrisse gezeichnet; Lentizellen, Bögen und Blattzeichen sind 0,13- bis 0,18-mm-Striche, alle über dem Stiftminimum."),
    "s50_ovl": ("Each thermal region has its own shapes (the open tier outlines carry limb-bark); lenticels, arcs and leaf marks are strokes without a region and follow their bone.", "Jede thermische Region hat eigene Formen (die offenen Etagenumrisse tragen limb-bark); Lentizellen, Bögen und Blattzeichen sind Striche ohne Region und folgen ihrem Knochen."),
}


def make_symbols(S, bounds):
    root = S.rig.bones["root-plate"]
    W, D = 5.6, S.depth_m
    radii = [1.0, 0.97, 0.99, 0.96, 1.0, 0.98, 0.97, 0.99, 1.0, 0.96, 0.98, 0.97]
    crown = envelope(0, 0, W / 2, D / 2, radii, k=11, amp=0.05, phase=0.4, n=4)
    plan = [Shape("crown-extent", root, "body", cr_closed(crown), False, 0.5, 1)]
    for i, (x, y) in enumerate([(0.0, 0.0), (-0.12, 0.08), (0.14, 0.05)]):
        plan.append(Shape("stem-%d" % (i + 1), root, "body", cr_closed(ring_pts(x, y, 0.08, n=10)), True, 0.3, 3 + i * 0.1))
    for i in range(6):
        a = (20 + 60 * i) * D2R
        plan.append(Shape("tier-%d" % (i + 1), root, "detail", cr_open([(0.2 * math.cos(a), 0.2 * math.sin(a)), (1.2 * math.cos(a), 1.2 * math.sin(a)), (2.2 * math.cos(a + 0.1), 2.2 * math.sin(a + 0.1))]), False, 0.3, 2))
    nominal = [Shape("sign-circle", root, "body", cr_closed(ring_pts(0, 0, 1.4, n=12)), False, 0.5, 1), Shape("sign-dot", root, "body", cr_closed(ring_pts(0, 0, 0.35, n=8)), True, 0.3, 2)]
    for s_ in plan + nominal:
        s_.frame = IDENT
    return [
        {"id": "plan-crown", "title": T("Plan symbol with true crown extent", "Grundrisssymbol mit wahrer Kronenausdehnung"), "description": T("A conventional plan sign for garden plans: the true plan outline of the crown, the three stem sections and six branch directions. It is a separate symbol and is never used silently for a pictorial scale.", "Ein konventionelles Grundrisszeichen für Gartenpläne: der wahre Grundrissumriss der Krone, die drei Stammquerschnitte und sechs Astrichtungen. Es ist ein eigenes Symbol und wird nie stillschweigend für einen Bildmaßstab verwendet."),
         "semantics": "true-extent", "units": "model-m", "denominators": [500, 200, 100], "pens": PENS["100"], "hatches": [], "geometry": {"elevation": [], "plan": [s.json() for s in plan]},
         "dimensionalNote": T("The outline is the true plan extent of the crown in model metres (%.1f m by %.1f m); the three filled circles are the stem sections of about 0.16 m, true to scale but placed schematically near the origin; the six strokes only indicate branch directions." % (W, D), "Der Umriss ist die wahre Kronenausdehnung im Grundriss in Modellmetern (%.1f m mal %.1f m); die drei gefüllten Kreise sind die Stammquerschnitte von etwa 0,16 m, maßstäblich, aber schematisch nahe dem Ursprung angeordnet; die sechs Striche deuten nur Astrichtungen an." % (W, D))},
        {"id": "tree-sign", "title": T("Nominal tree sign with stem centre", "Nominelles Baumzeichen mit Stammmitte"), "description": T("The conventional circle-and-dot tree sign of small-scale plans: the dot marks the stem group, the circle is a fixed paper size.", "Das übliche Kreis-und-Punkt-Baumzeichen kleinmaßstäblicher Pläne: Der Punkt markiert die Stammgruppe, der Kreis hat eine feste Papiergröße."),
         "semantics": "nominal", "units": "paper-mm", "denominators": [500, 200], "pens": PENS["500"], "hatches": [], "geometry": {"elevation": [], "plan": [s.json() for s in nominal]},
         "dimensionalNote": T("A fixed 2.8 mm circle with a 0.7 mm centre dot in paper millimetres; its size carries no dimension. Use the true-extent plan symbol where the crown width matters.", "Ein fester Kreis von 2,8 mm mit 0,7 mm Mittelpunkt in Papiermillimetern; seine Größe trägt kein Maß. Wo die Kronenbreite zählt, ist das Grundrisssymbol mit wahrer Ausdehnung zu verwenden.")},
    ]
