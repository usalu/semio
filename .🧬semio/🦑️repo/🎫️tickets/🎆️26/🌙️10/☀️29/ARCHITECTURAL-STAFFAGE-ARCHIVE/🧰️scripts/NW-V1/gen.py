"""Generic per-scale drawing driver for lime, apple and maple: every number (crown outline points, stations, lobes, arcs, glyph sizes) comes from the species configuration C."""
import math
from lib import *
from common import *
from reps import *

WOBS = [[1.0, 0.95, 1.04, 0.97, 1.02, 0.96, 1.0], [1.0, 1.05, 0.96, 1.02, 0.95, 1.03, 0.98], [0.98, 1.0, 0.94, 1.03, 1.0, 0.95, 1.04]]
ARC_SETS = [[(0.62, 0.62, 200, 330, 0), (0.35, 0.4, 30, 150, 0)], [(0.6, 0.6, 20, 130, 0), (0.38, 0.4, 210, 320, 0)], [(0.6, 0.62, 250, 350, 0), (0.38, 0.4, 100, 200, 0)]]
ARC_SETS50 = [[(0.7, 0.7, 190, 330, 0), (0.5, 0.5, 20, 150, 0), (0.28, 0.3, 230, 340, 0)], [(0.7, 0.7, 10, 130, 0), (0.5, 0.52, 200, 320, 0), (0.28, 0.3, 60, 170, 0)], [(0.68, 0.7, 240, 350, 0), (0.48, 0.5, 90, 200, 0), (0.28, 0.3, 300, 380, 0)]]


def wood_shapes(S, sc, C):
    out = []
    den = int(sc)
    skip = set(C["stem_bones"])
    for i, id in enumerate(S.rig.order):
        b = S.rig.bones[id]
        if b.kind != "wood" or id in skip or id in C.get("no_wood", {}).get(sc, ()):
            continue
        mean = (b.d0 + b.d1) / 2 * 1000 / den
        if mean >= C["min_wood_mm"][sc] and b.d1 * 1000 / den >= 0.22:
            wob = WOBS[i % 3]
            n = {"200": 4, "100": 5, "50": 7}[sc]
            wb = C.get("wob_override", {}).get(id)
            out.append(tapered(S, b, sc, n, wb or wob, 2 + 0.01 * i, id=("wood-" + id)))
        elif sc in ("100", "50") and (b.d0 * 1000 / den >= C["min_stroke_wood_mm"][sc] or id in C.get("always_stroke", ())):
            out.append(centre_stroke(S, b, sc, "detail" if sc == "100" else "texture", 2 + 0.01 * i, bend=C.get("bend", 0.0), id="twig-" + id))
    return out


def mass_set(S, sc, C):
    out = []
    P = C["mass"][sc]
    for i, m in enumerate(S.masses):
        hatch = "shade-lines" if (m.kind == "shade" and sc in ("100", "50")) else None
        lobe = P["lobe"] * (P.get("epi_lobe", 1.0) if m.kind == "epi" else 1.0)
        shp, _ = mass_shape(S, m, sc, lobe=lobe, amp=P["amp"], vary=P["vary"], z=3 + P.get("dz", 0.4) * i, phase=P.get("phase", 0.3) * i + 0.1, under=P.get("under", 0.1), pointed=P.get("pointed", 0.6), n=P.get("n", 6), hatch=hatch, rx_s=P.get("rx_s", 1.0) * (P.get("core_s", 1.0) if m.kind == "shade" else 1.0), ry_s=P.get("ry_s", 1.0), regions=C.get("mass_regions", {}).get(sc, {}).get(m.id))
        out.append(shp)
        if sc == "100":
            out += mass_arcs(S, m, sc, ARC_SETS[i % 3], "detail", 3.2 + P.get("dz", 0.4) * i)
        if sc == "50":
            out += mass_arcs(S, m, sc, ARC_SETS50[i % 3], "texture", 3.2 + P.get("dz", 0.4) * i)
            if C.get("glyph") and m.kind in C["glyph_kinds"]:
                R = C["glyph_R"]
                for j in range(2 + i % 3):
                    fr = [0.72, 0.74, 0.7, 0.74][j]
                    ang = ([200, 330, 100, 20][j] + 25 * i) * D2R
                    cx = m.c[0] + m.rx * P.get("rx_s", 1.0) * fr * 0.9 * math.cos(ang)
                    cy = m.c[1] + m.ry * P.get("ry_s", 1.0) * fr * 0.9 * math.sin(ang)
                    out.append(Shape("%s-leaf%d" % (m.id, j + 1), S.rig.bones[m.id], "detail", cr_closed(C["glyph"](cx, cy, R, 15 * (i + j) - 40)), False, 0.2, 3.3 + P.get("dz", 0.4) * i))
    return out


def dec(sc, w):
    """Decision texts from the species words; every sentence names the asset's own elements."""
    en = lambda k: w[k][0]
    de = lambda k: w[k][1]
    mm = {"500": 2, "200": 5, "100": 10, "50": 20}[sc]
    pens = {"500": ("0.15", "0.25"), "200": ("0.18", "0.25"), "100": ("0.2", "0.3"), "50": ("0.25", "0.35")}[sc]
    d = {}
    key = {"500": "s500", "200": "s200", "100": "s100", "50": "s50"}[sc]
    d["silhouette"] = (w[key + "_sil"][0], w[key + "_sil"][1])
    d["retained"] = (w[key + "_ret"][0], w[key + "_ret"][1])
    d["merged"] = (w[key + "_mer"][0], w[key + "_mer"][1])
    d["omitted"] = (w[key + "_omi"][0], w[key + "_omi"][1])
    d["thinElements"] = (w[key + "_thin"][0], w[key + "_thin"][1])
    d["hatching"] = {
        "500": ("No hatching; the pen is 0.15 mm for outlines and 0.25 mm for the ground line.", "Keine Schraffur; der Stift ist 0,15 mm für Umrisse und 0,25 mm für die Bodenlinie."),
        "200": ("No hatching. Internal edges: " + w["edge200"][0], "Keine Schraffur. Innere Kanten: " + w["edge200"][1]),
        "100": ("Restrained: only the shaded inner foliage carries a thin 45 degree line hatch (0.6 mm spacing, above the profile minimum); no other hatching.", "Zurückhaltend: nur das beschattete Innenlaub trägt eine feine 45-Grad-Schraffur (0,6 mm Abstand, über dem Profilminimum); sonst keine Schraffur."),
        "50": ("A thin 45 degree line hatch (0.8 mm spacing) marks the shaded inner foliage; bark and leaves are drawn with strokes.", "Eine feine 45-Grad-Schraffur (0,8 mm Abstand) kennzeichnet das beschattete Innenlaub; Rinde und Blätter werden mit Strichen gezeichnet."),
    }[sc]
    d["styles"] = {
        "500": ("Light line: closed outlines only. Dark solid: the crown reads as one dark mass, the stem lighter.", "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: die Krone liest sich als eine dunkle Masse, der Stamm heller."),
        "200": ("Light line: outlines 0.18 mm, one 0.13 mm detail pen. Dark solid: sun and shade foliage in two tones, wood lighter.", "Helle Linie: Umrisse 0,18 mm, ein Detailstift 0,13 mm. Dunkle Fläche: Sonnen- und Schattenlaub in zwei Tönen, Holz heller."),
        "100": ("Light line: 0.2 mm outlines, 0.13 mm detail, 0.1 mm texture. Dark solid: tonal masses, detail strokes stay visible on them.", "Helle Linie: Umrisse 0,2 mm, Detail 0,13 mm, Textur 0,1 mm. Dunkle Fläche: getönte Massen, Detailstriche bleiben darauf sichtbar."),
        "50": ("Light line: 0.25 mm outlines, 0.18 mm detail, 0.13 mm texture. Dark solid: tonal fills separate bark, sun foliage and shade foliage.", "Helle Linie: Umrisse 0,25 mm, Detail 0,18 mm, Textur 0,13 mm. Dunkle Fläche: getönte Füllungen trennen Rinde, Sonnenlaub und Schattenlaub."),
    }[sc]
    d["overlays"] = (w[key + "_ovl"][0], w[key + "_ovl"][1])
    return d


def make_reps_generic(S, C):
    S.shapes = {}
    reps = {}
    S.shapes["500"] = C["shapes500"](S)
    for sc in ["200", "100", "50"]:
        sh = []
        sh += C["stems"](S, sc)
        sh += wood_shapes(S, sc, C)
        sh += mass_set(S, sc, C)
        sh += C["extras"](S, sc)
        S.shapes[sc] = sh
    for sc in ["500", "200", "100", "50"]:
        reps[sc] = rep_json(S, sc, S.shapes[sc], dec(sc, C["words"]))
    return reps, world_bbox(S.shapes["50"])
