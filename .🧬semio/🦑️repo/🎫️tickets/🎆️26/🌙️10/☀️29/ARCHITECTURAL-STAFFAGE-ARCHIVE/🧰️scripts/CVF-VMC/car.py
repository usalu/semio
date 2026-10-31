import sys
from build import *

REL = "🚜️vehicles/city-car"
f = lambda v: ("%.3f" % v).rstrip("0").rstrip(".")


def W(a):
    ws = world(a)
    return lambda bone, x, y: ap(ws[bone], (x, y))


KEYS = [
    opkey("engine-load", "Engine load", "Motorlast", [(0, "Engine off", "Motor aus"), (0.2, "Idle", "Leerlauf"), (0.5, "Part load (cruising)", "Teillast (Reisefahrt)"), (1, "Full load", "Volllast")]),
    opkey("rolling", "Rolling speed", "Rollgeschwindigkeit", [(0, "Standstill", "Stillstand"), (0.5, "About 25 km/h", "Etwa 25 km/h"), (1, "About 50 km/h", "Etwa 50 km/h")]),
    opkey("braking", "Braking", "Bremsen", [(0, "Not braking", "Nicht gebremst"), (0.3, "Light braking", "Leichte Bremsung"), (1, "Full braking from 50 km/h", "Vollbremsung aus 50 km/h")]),
]

steel = lambda: layer("sheet", "Steel sheet 0.8 mm", "Stahlblech 0,8 mm", "metal", "paint", 0.0008, 0.0008 / 50, 3100, source="literature", ref="incropera-2011")
paintl = lambda: layer("paint", "Paint film", "Lackschicht", "polymer", "paint", 0.00008, 0.00008 / 0.3, 150, source="authored-estimate")
REGIONS = [
    region("body-panels", "Painted side and end panels", "Lackierte Seiten- und Endbleche", "surface", "passive", "paint", 8.2, 1.2, "vertical-plate", 0.5, 6000, [steel(), paintl()], ref="ashrae-2021"),
    region("roof", "Roof panel", "Dachblech", "surface", "passive", "paint", 1.5, 1.3, "horizontal-plate-up", 0.95, 6000, [steel(), paintl()], ref="ashrae-2021"),
    region("bonnet", "Bonnet, warmed from below", "Motorhaube, von unten erwärmt", "surface", "operating", "paint", 1.2, 0.9, "horizontal-plate-up", 0.9, 6500, [steel(), paintl()], ("engine-load", 200), ref="heywood-2018"),
    region("tailgate", "Tailgate panel", "Heckklappenblech", "surface", "passive", "paint", 0.9, 0.8, "vertical-plate", 0.6, 6000, [steel(), paintl()], ref="ashrae-2021"),
    region("glazing", "Glazing (windscreen, side and rear glass)", "Verglasung (Windschutz-, Seiten- und Heckscheibe)", "glazing", "passive", "glass", 3.2, 0.9, "vertical-plate", 0.6, 7500,
           [layer("glass", "Laminated or toughened glass 4 mm", "Verbund- oder Einscheibensicherheitsglas 4 mm", "glass", "glass", 0.004, 0.004 / 1.0, 7500, source="literature", ref="incropera-2011")], ref="ashrae-2021"),
    region("engine-bay", "Engine and manifold surfaces", "Motor- und Krümmeroberflächen", "engine", "operating", "engine-metal", 1.1, 0.5, "cylinder", 0.1, 9000, [], ("engine-load", 1100), ref="heywood-2018"),
    region("exhaust-system", "Exhaust pipe and silencer", "Abgasrohr und Schalldämpfer", "exhaust", "operating", "exhaust-steel", 0.45, 0.08, "cylinder", 0.2, 8500,
           [layer("pipe", "Steel pipe 1.5 mm", "Stahlrohr 1,5 mm", "metal", "exhaust-steel", 0.0015, 0.0015 / 45, 8500, source="authored-estimate")], ("engine-load", 2300), ref="heywood-2018"),
    region("tyres", "Tyre treads and sidewalls", "Reifenlaufflächen und -flanken", "tyre", "operating", "rubber", 1.8, 0.6, "cylinder", 0.4, 34000,
           [layer("tread", "Rubber tread and carcass", "Gummilauffläche und Karkasse", "polymer", "rubber", 0.01, 0.01 / 0.2, 34000, source="authored-estimate", ref="wong-2008")], ("rolling", 480), ref="wong-2008"),
    region("brakes", "Brake discs and calipers", "Bremsscheiben und Bremssättel", "surface", "operating", "cast-iron", 0.22, 0.28, "vertical-plate", 0.2, 31000, [], ("braking", 36000), ref="limpert-1999"),
]

EMITTERS = [emitter("tailpipe", "exhaust", "Exhaust jet", "Abgasstrahl", "tailpipe", 190, 0.045, "engine-load", 0.08, 280, 0.075, ref="heywood-2018")]

DEFAULTS = {"body-panels": (1, 0.5), "roof": (1, 1), "bonnet": (1, 1), "tailgate": (1, 0.5), "glazing": (1, 0.5), "engine-bay": (0.2, 0), "exhaust-system": (0.6, 0), "tyres": (1, 0.15), "brakes": (0.5, 0)}
CONTACTS = {"tyres": (0.07, 30)}


def P(eng, roll, brake, em=None, regs=None, grounded=True):
    return {"operating": {"engine-load": eng, "rolling": roll, "braking": brake}, "emitters": {"tailpipe": (1 if eng > 0 else 0) if em is None else em}, "regions": regs or {}, "grounded": grounded}


POSES = {
    "parked": P(0, 0, 0),
    "driving": P(0.5, 0.8, 0),
    "hatch-open": P(0, 0, 0, regs={"tailgate": {"exposedFraction": 1, "sunlitFraction": 0.9}}),
    "bonnet-open": P(0, 0, 0, regs={"engine-bay": {"exposedFraction": 1, "sunlitFraction": 0.7}, "bonnet": {"exposedFraction": 1, "sunlitFraction": 0.3}}),
    "braking-nose-down": P(0.15, 0.4, 1),
    "lane-start": P(0.2, 0, 0),
    "lane-cruise-entry": P(0.6, 0.8, 0),
    "lane-exit": P(0.5, 0.8, 0),
    "crawl-in": P(0.2, 0.2, 0.3),
}


def fix_clips(clips):
    for c in clips:
        if c["id"] == "drive-by":
            for t in c["tracks"]:
                if t["target"] == "emitter:tailpipe":
                    t["ease"] = "step"
                    t["keys"] = [{"t": 0, "v": 0}, {"t": 0.3, "v": 1}]
            c["events"][0]["target"] = "tailpipe"
        if c["id"] == "park-and-open-hatch":
            for t in c["tracks"]:
                if t["target"] == "emitter:tailpipe":
                    t["ease"] = "step"
                    t["keys"] = [{"t": 0, "v": 1}, {"t": 4.4, "v": 0}]
                if t["target"] == "operating:engine-load":
                    t["keys"] = [{"t": 0, "v": 0.2}, {"t": 4.4, "v": 0.2}, {"t": 4.7, "v": 0}, {"t": 9, "v": 0}]
            c["tracks"].append({"target": "region:tailgate:sunlitFraction", "ease": "smooth", "keys": [{"t": 6.5, "v": 0.5}, {"t": 7.4, "v": 0.9}]})


def hatch_panel(w, thick, regionid="tailgate", pid="tailgate-panel", z=2.3, tone=0.62):
    inner = lambda y: y + thick
    d = "M -0.05,-0.009 C 0.199,-0.054 0.489,-0.031 0.747,-0.013 L 0.80,0.03 L 0.74,%s C 0.465,%s 0.196,%s -0.05,%s Z" % (f(inner(-0.015)), f(inner(-0.035)), f(inner(-0.045)), f(inner(-0.03)))
    return path(pid, "hatch", "body", z, d, tone, True, region=regionid, material="paint")


def bonnet_panel(bottom, pid="bonnet-panel", z=2.3, tone=0.62):
    d = "M 0.94,0.033 C 0.652,0.051 0.248,0.031 -0.05,0.02 L -0.05,%s L 0.9,%s C 0.95,%s 0.965,0.0 0.94,0.033 Z" % (f(bottom), f(bottom), f(bottom + 0.015))
    return path(pid, "bonnet", "body", z, d, tone, True, region="bonnet", material="paint")


def reps(a, old):
    w = W(a)
    R = {}

    def shell_path(under, thick_h, band, A=0.73):
        bt = w("bonnet", 0.9, under - band)
        bh = w("bonnet", 0.0, under - band)
        h_in = lambda x, y: w("hatch", x, y + thick_h)
        top = h_in(-0.012, -0.009)
        bot = h_in(0.78, -0.012)
        s = ("M -1.9,0.3 L -1.58,0.3 C -1.58,0.6 -1.42,0.73 -1.2,0.73 C -0.98,0.73 -0.82,0.6 -0.82,0.3 L -0.78,0.2 L 0.83,0.2 L 0.87,0.3 C 0.87,0.6 1.03,0.73 1.25,0.73 C 1.47,0.73 1.63,0.6 1.63,0.3 L 1.9,0.3 "
                "C 1.945,0.32 1.955,0.4 1.95,0.48 L 1.95,0.62 C 1.95,0.68 1.93,0.7 1.91,%s L %s,%s L %s,%s L %s,0.99 C 0.78,1.1 0.5,1.33 0.25,1.46 C 0.1,1.5 -0.4,1.5 -0.9,1.495 C -1.2,1.49 -1.33,1.47 -1.45,1.4 L %s,%s L %s,%s L -1.95,0.76 L -1.95,0.45 C -1.95,0.36 -1.94,0.31 -1.9,0.3 Z") % (
            f(bt[1] + 0.0), f(bt[0]), f(bt[1]), f(bh[0]), f(bh[1]), f(bh[0]), f(top[0]), f(top[1]), f(bot[0]), f(bot[1]))
        return s.replace('0.73', f(A))

    # ---------------- 1:500 ----------------
    bt = w("bonnet", 0.9, -0.11)
    bh = w("bonnet", 0.0, -0.11)
    ht = w("hatch", -0.012, 0.13)
    hb = w("hatch", 0.78, 0.13)
    sh500 = ("M -1.9,0.3 L -1.35,0.3 L -1.35,0.2 L 1.1,0.2 L 1.1,0.3 L 1.9,0.3 C 1.945,0.32 1.955,0.4 1.95,0.48 L 1.95,0.62 C 1.95,0.68 1.93,0.7 1.9,%s L %s,%s L %s,%s L %s,0.99 C 0.78,1.1 0.5,1.33 0.25,1.46 C 0.1,1.5 -0.4,1.5 -0.9,1.495 C -1.2,1.49 -1.33,1.47 -1.45,1.4 L %s,%s L %s,%s L -1.95,0.76 L -1.95,0.45 C -1.95,0.36 -1.94,0.31 -1.9,0.3 Z") % (
        f(bt[1]), f(bt[0]), f(bt[1]), f(bh[0]), f(bh[1]), f(bh[0]), f(ht[0]), f(ht[1]), f(hb[0]), f(hb[1]))
    wheel = lambda sfx, bone, regs, extra=(): [ell("tyre-" + sfx, bone, "body", 4, 0, 0, 0.3, tone=0.1, regions=regs, material="rubber")] + list(extra)
    e500 = [
        path("body-shell", "chassis", "body", 2, sh500, 0.62, True, regions=["body-panels", "roof", "engine-bay", "exhaust-system"], material="paint"),
        path("greenhouse", "chassis", "glazing", 6, "M 0.74,1.04 L 0.33,1.38 C 0.28,1.41 0.2,1.42 0.1,1.42 L -1.2,1.4 C -1.3,1.32 -1.34,1.16 -1.33,1.04 Z", 0.85, True, region="glazing", material="glass"),
        bonnet_panel(-0.11, "bonnet-panel"),
        hatch_panel(0, 0.14),
    ] + wheel("f", "wheel-f", ["tyres", "brakes"]) + wheel("r", "wheel-r", ["tyres", "brakes"])
    R["500"] = rep(500, *RANGES[500], PRINT_PENS[500], [], e500, decisions(
        tx("One composed body silhouette (hatchback profile) with two solid wheel discs; the wheel arches are closed so each tyre reads as one dark disc against the body.", "Eine zusammengesetzte Karosseriesilhouette (Fließheckprofil) mit zwei vollen Radscheiben; die Radkästen sind geschlossen, sodass jeder Reifen als dunkle Scheibe vor dem Aufbau steht."),
        tx("Body, one glazing band, bonnet and tailgate panels (so both still open on their hinge bones) and both tyres.", "Aufbau, ein Verglasungsband, Motorhauben- und Heckklappenblech (beide öffnen weiter an ihren Gelenken) und beide Reifen."),
        tx("Side glass, windscreen and B-pillar are one band (pillar 0.12 m < 0.3 mm). Rim, disc and caliper merge into the tyre disc; engine bay and exhaust merge into the body fill (area-weighted overlay).", "Seitenscheiben, Frontscheibe und B-Säule bilden ein Band (Säule 0,12 m < 0,3 mm). Felge, Scheibe und Sattel gehen in der Reifenscheibe auf; Motorraum und Abgasanlage in der Aufbaufläche (flächengewichtete Überlagerung)."),
        tx("Door gaps, handles, mirror, lamps, bumper lines, seats, tread and every wheel detail; exhaust pipe (0.06 m = 0.12 mm).", "Türfugen, Griffe, Spiegel, Leuchten, Stoßfängerlinien, Sitze, Profil und alle Raddetails; Abgasrohr (0,06 m = 0,12 mm)."),
        tx("Bonnet and tailgate are drawn 0.14 m thick (real 0.05 m) so the hinged panels stay a printable 0.3 mm; the body is cut back by the same amount, so the closed outline equals the real one.", "Motorhaube und Heckklappe sind 0,14 m dick gezeichnet (real 0,05 m), damit die schwenkbaren Bleche druckbare 0,3 mm bleiben; der Aufbau ist um denselben Betrag zurückgenommen, die geschlossene Kontur entspricht der realen."),
        tx("None.", "Keine."),
        tx("Body mid-tone, glazing light, tyres almost black; one pen of 0.15 mm for all outlines and a 0.13 mm line for the glazing edge.", "Aufbau mittlerer Ton, Verglasung hell, Reifen fast schwarz; ein Stift von 0,15 mm für alle Konturen und 0,13 mm für die Verglasungskante."),
        tx("Thermal overlays colour the merged body by the area-weighted mean of panels, roof, engine bay and exhaust; bonnet, tailgate, glazing and tyres (with brakes) are separate.", "Thermische Überlagerungen färben den Aufbau mit dem flächengewichteten Mittel aus Blechen, Dach, Motorraum und Abgasanlage; Haube, Heckklappe, Verglasung und Reifen (mit Bremsen) sind getrennt.")))

    # ---------------- 1:200 and 1:100 common ----------------
    def body_set(under, thick_h, band, roofband=True):
        sh = shell_path(under, thick_h, band)
        bt2 = [w("bonnet", 0.0, under), w("bonnet", 0.9, under), w("bonnet", 0.9, under - band), w("bonnet", 0.0, under - band)]
        return sh, bt2

    sh200, band_pts = body_set(-0.075, 0.12, 0.07)
    roof_band = "M 0.25,1.46 C 0.1,1.5 -0.4,1.5 -0.9,1.495 C -1.2,1.49 -1.33,1.47 -1.45,1.4 L -1.4,1.34 C -1.25,1.41 -1.2,1.42 -0.9,1.425 C -0.4,1.43 0.1,1.43 0.2,1.39 Z"
    glass_f = "M 0.72,1.05 L 0.345,1.405 C 0.28,1.41 0.2,1.42 0.1,1.42 L -0.02,1.42 L -0.02,1.05 Z"
    glass_r = "M -0.14,1.05 L -0.14,1.42 C -0.6,1.425 -1.0,1.425 -1.22,1.405 C -1.31,1.32 -1.34,1.16 -1.33,1.05 Z"
    well = lambda sfx, cx, A=0.73: path("well-" + sfx, "chassis", "body", 1, ("M %s,0.3 C %s,0.6 %s,0.73 %s,0.73 C %s,0.73 %s,0.6 %s,0.3 Z" % (f(cx - 0.38), f(cx - 0.38), f(cx - 0.22), f(cx), f(cx + 0.22), f(cx + 0.38), f(cx + 0.38))).replace("0.73", f(A)), 0.04, True, material="plastic-trim")
    bay = poly("engine-bay-top", "chassis", "detail", 2.2, band_pts[0:0] + [band_pts[1], band_pts[0], band_pts[3], band_pts[2]], True, 0.12, True, region="engine-bay", material="engine-metal")
    exhaust200 = [
        cap("exhaust-pipe", "chassis", "detail", 0.8, 0.35, 0.16, -0.9, 0.16, 0.035, tone=0.35, region="exhaust-system", material="exhaust-steel"),
        cap("silencer", "chassis", "detail", 0.8, -1.0, 0.2, -1.75, 0.2, 0.08, tone=0.4, region="exhaust-system", material="exhaust-steel"),
    ]
    wheel200 = lambda sfx, bone: [
        ell("tyre-" + sfx, bone, "body", 4, 0, 0, 0.3, tone=0.1, region="tyres", material="rubber"),
        ell("disc-" + sfx, bone, "detail", 4.4, 0, 0, 0.19, tone=0.5, region="brakes", material="cast-iron"),
        ell("hub-" + sfx, bone, "detail", 5.2, 0, 0, 0.055, tone=0.72, material="alloy"),
    ]
    e200 = [
        path("body-shell", "chassis", "body", 2, sh200, 0.62, True, region="body-panels", material="paint"),
        path("roof-band", "chassis", "detail", 2.1, roof_band, 0.5, True, region="roof", material="paint"),
        well("front", 1.25), well("rear", -1.2), bay,
        path("side-glass-front", "chassis", "glazing", 6, glass_f, 0.85, True, region="glazing", material="glass"),
        path("side-glass-rear", "chassis", "glazing", 6, glass_r, 0.85, True, region="glazing", material="glass"),
        path("door-gap-b", "chassis", "detail", 7, "M -0.08,1.05 L -0.08,0.5", filled=False),
        path("door-front-edge", "chassis", "detail", 7, "M 0.72,1.05 C 0.72,0.8 0.72,0.62 0.73,0.5", filled=False),
        bonnet_panel(-0.075), hatch_panel(0, 0.12),
    ] + exhaust200 + wheel200("f", "wheel-f") + wheel200("r", "wheel-r")
    R["200"] = rep(200, *RANGES[200], PRINT_PENS[200], [], e200, decisions(
        tx("Hatchback profile with open wheel arches, a lighter roof band, two glass panes and the dark engine-bay slot under the bonnet; each tyre keeps 0.08 m clearance (0.4 mm) to its arch.", "Fließheckprofil mit offenen Radkästen, hellerem Dachband, zwei Scheiben und dem dunklen Motorraumspalt unter der Haube; jeder Reifen hält 0,08 m (0,4 mm) Abstand zum Radlauf."),
        tx("Body, roof band, front and rear side glass (separated by the B-pillar, 0.6 mm), bonnet and tailgate panels, exhaust pipe and silencer, tyres with brake discs and hubs, two door gaps.", "Aufbau, Dachband, vordere und hintere Seitenscheibe (durch die B-Säule 0,6 mm getrennt), Motorhauben- und Heckklappenblech, Abgasrohr und Schalldämpfer, Reifen mit Bremsscheiben und Naben, zwei Türfugen."),
        tx("Rim barrel and disc are one lighter disc (brakes region); tail pipe merges into the silencer; tailgate glass merges into the tailgate panel.", "Felgenbett und Scheibe sind eine hellere Scheibe (Region Bremsen); Endrohr geht im Schalldämpfer auf; Heckscheibe im Heckklappenblech."),
        tx("Spokes, calipers, mirror, handles, lamps, belt crease, seats, steering wheel, tread (all below 0.1 m).", "Speichen, Sättel, Spiegel, Griffe, Leuchten, Zierlinie, Sitze, Lenkrad, Profil (alle unter 0,1 m)."),
        tx("Panels are 0.10 to 0.12 m thick (real 0.05 m) to stay at 0.5 mm; the pipe is 0.07 m (0.35 mm) and is kept because it carries the exhaust region.", "Bleche sind 0,10 bis 0,12 m dick gezeichnet (real 0,05 m), um 0,5 mm zu halten; das Rohr hat 0,07 m (0,35 mm) und bleibt, weil es die Region Abgasanlage trägt."),
        tx("None.", "Keine."),
        tx("Body mid-tone, roof lighter, glass lightest, wheel wells and tyres near black; outlines 0.18 mm, door gaps 0.13 mm.", "Aufbau mittlerer Ton, Dach heller, Glas am hellsten, Radkästen und Reifen fast schwarz; Konturen 0,18 mm, Türfugen 0,13 mm."),
        tx("Every thermal region has its own shape: panels, roof band, bonnet, tailgate, glass, engine-bay slot, exhaust, tyres, brake discs; the engine bay shows when the bonnet is raised.", "Jede thermische Region hat eine eigene Form: Bleche, Dachband, Haube, Heckklappe, Glas, Motorraumspalt, Abgasanlage, Reifen, Bremsscheiben; der Motorraum zeigt sich bei geöffneter Haube.")))

    sh100, band100 = body_set(-0.07, 0.11, 0.07)
    sh100 = shell_path(-0.07, 0.11, 0.07, 0.69)
    spokes = lambda sfx, bone: [cap("spoke-%s-%d" % (sfx, i + 1), bone, "detail", 5, x0, y0, x1, y1, 0.03, 0.025, tone=0.7, material="alloy") for i, (x0, y0, x1, y1) in enumerate(
        [(0.0, 0.05, 0.0, 0.165), (-0.0476, 0.0155, -0.1569, 0.051), (-0.0294, -0.0405, -0.097, -0.1335), (0.0294, -0.0405, 0.097, -0.1335), (0.0476, 0.0155, 0.1569, 0.051)])]
    wheel100 = lambda sfx, bone: [
        ell("tyre-" + sfx, bone, "body", 4, 0, 0, 0.3, tone=0.1, region="tyres", material="rubber"),
        ell("rim-barrel-" + sfx, bone, "detail", 4.2, 0, 0, 0.19, tone=0.55, material="alloy"),
        ell("disc-" + sfx, bone, "detail", 4.4, 0, 0, 0.15, tone=0.42, region="brakes", material="cast-iron"),
    ] + spokes(sfx, bone) + [ell("hub-" + sfx, bone, "detail", 5.2, 0, 0, 0.05, tone=0.7, material="alloy")]
    e100 = [
        path("body-shell", "chassis", "body", 2, sh100, 0.62, True, region="body-panels", material="paint"),
        path("roof-band", "chassis", "detail", 2.1, roof_band, 0.5, True, region="roof", material="paint"),
        well("front", 1.25, 0.69), well("rear", -1.2, 0.69),
        poly("engine-bay-top", "chassis", "detail", 2.2, [band100[1], band100[0], band100[3], band100[2]], True, 0.12, True, region="engine-bay", material="engine-metal"),
        path("side-glass-front", "chassis", "glazing", 6, glass_f, 0.85, True, region="glazing", material="glass"),
        path("side-glass-rear", "chassis", "glazing", 6, glass_r, 0.85, True, region="glazing", material="glass"),
        path("windscreen-edge", "chassis", "glazing", 6.1, "M 0.9,1.01 C 0.74,1.12 0.52,1.31 0.31,1.4", 0.85, False, region="glazing", material="glass"),
        path("door-gap-b", "chassis", "detail", 7, "M -0.08,1.05 L -0.08,0.24", filled=False),
        path("door-front-edge", "chassis", "detail", 7, "M 0.72,1.0 C 0.72,0.7 0.72,0.45 0.74,0.24", filled=False),
        path("door-gap-rear", "chassis", "detail", 7, "M -1.34,1.05 L -1.4,0.64", filled=False),
        path("headlamp", "chassis", "glazing", 7, "M 1.72,0.78 C 1.8,0.8 1.9,0.77 1.95,0.72 L 1.94,0.67 C 1.86,0.69 1.78,0.71 1.7,0.73 Z", 0.85, True, material="glass"),
        rect("tail-lamp", "chassis", "detail", 7, -1.95, 0.5, 0.06, 0.13, tone=0.3, material="plastic-trim"),
        ell("mirror", "chassis", "detail", 7, 0.80, 1.13, 0.07, 0.045, tone=0.3, rot=-10, material="plastic-trim"),
        bonnet_panel(-0.07), hatch_panel(0, 0.11),
        path("hatch-glass", "hatch", "glazing", 6, "M 0.03,-0.03 L 0.45,-0.015 L 0.45,0.05 L 0.03,0.055 Z", 0.85, True, region="glazing", material="glass"),
        cap("exhaust-pipe", "chassis", "detail", 0.8, 0.35, 0.16, -0.9, 0.16, 0.03, tone=0.35, region="exhaust-system", material="exhaust-steel"),
        cap("silencer", "chassis", "detail", 0.8, -1.0, 0.14, -1.7, 0.14, 0.07, tone=0.4, region="exhaust-system", material="exhaust-steel"),
        cap("tail-pipe", "chassis", "detail", 0.8, -1.7, 0.16, -1.945, 0.25, 0.03, 0.025, tone=0.45, region="exhaust-system", material="exhaust-steel"),
    ] + wheel100("f", "wheel-f") + wheel100("r", "wheel-r")
    R["100"] = rep(100, *RANGES[100], pens(0.18, 0.13, None, 0.1, 0.35), [], e100, decisions(
        tx("Hatchback profile with arches and a gap of 0.08 m (0.8 mm) around each tyre, separate roof band, glazing in three parts, bonnet and tailgate as thickened panels.", "Fließheckprofil mit Radläufen und 0,08 m (0,8 mm) Abstand um jeden Reifen, eigenes Dachband, Verglasung in drei Teilen, Haube und Heckklappe als verstärkte Bleche."),
        tx("Door gaps (front, B, rear), headlamp, tail lamp, mirror, exhaust pipe, silencer and tail pipe, tyres with rim, disc, five spokes and hub, engine-bay slot.", "Türfugen (vorn, B, hinten), Scheinwerfer, Rückleuchte, Spiegel, Abgasrohr, Schalldämpfer und Endrohr, Reifen mit Felge, Scheibe, fünf Speichen und Nabe, Motorraumspalt."),
        tx("Sill, belt crease and bumper lines merge into the body outline; the caliper merges into the disc.", "Schweller, Zierlinie und Stoßfängerlinien gehen in der Aufbaukontur auf; der Sattel in der Scheibe."),
        tx("Door handles, fuel flap, grille lines, seats, steering wheel, sidewall and tread (below 0.04 m = 0.4 mm).", "Türgriffe, Tankklappe, Kühlergrilllinien, Sitze, Lenkrad, Flanke und Profil (unter 0,04 m = 0,4 mm)."),
        tx("Spokes are 0.06 m wide (0.6 mm) and drawn at real width; panels 0.11 m (real 0.05 m).", "Speichen sind 0,06 m breit (0,6 mm) in Realbreite gezeichnet; Bleche 0,11 m (real 0,05 m)."),
        tx("None; material is shown by tone only.", "Keine; das Material zeigt sich nur über den Ton."),
        tx("Body mid-tone, roof lighter, alloy lightest, tyres near black; outlines 0.25 mm, detail 0.18 mm.", "Aufbau mittlerer Ton, Dach heller, Leichtmetall am hellsten, Reifen fast schwarz; Konturen 0,25 mm, Details 0,18 mm."),
        tx("Regions map one to one onto shapes; the rotating wheel shows spokes, so rolling is legible in clips.", "Regionen bilden sich eins zu eins auf Formen ab; das rollende Rad zeigt Speichen, sodass das Rollen in Clips lesbar ist.")))

    e50 = convert_v1_shapes(old)
    for s in e50:
        if s["id"] in ("side-glass-front", "side-glass-rear", "hatch-glass"):
            s["hatch"] = "glass-sheen"
        if s["id"].startswith(("spoke-", "hub-", "rim-barrel-")):
            s["region"] = "brakes"
        if s["id"] == "exhaust-pipe":
            s["y0"] = s["y1"] = 0.12
            s["x1"] = -1.1
        if s["id"] == "silencer":
            s["y0"] = s["y1"] = 0.13
            s["x0"] = -1.1
        if s["id"] == "tail-pipe":
            s["y0"], s["y1"] = 0.15, 0.25
        if s["id"] == "fuel-flap":
            s["cy"] = 0.84
        if s["id"] == "steering-wheel":
            s["cx"], s["cy"] = 0.56, 1.08
        if s["id"] == "belt-crease":
            s["d"] = "M 1.64,0.9 C 1.2,0.93 0.9,0.97 0.4,0.98 C -0.4,0.99 -1.0,0.99 -1.62,0.97"
        if s["id"] == "hatch-glass":
            s["d"] = "M 0.03,-0.0 L 0.15,-0.03 L 0.33,-0.032 L 0.49,-0.005 L 0.455,0.036 L 0.296,0.01 L 0.13,0.01 L 0.02,0.04 Z"
    R["50"] = rep(50, *RANGES[50], pens(0.25, 0.18, 0.1, 0.13, 0.5), [hatch("glass-sheen", "Glass sheen", "Glasglanz", "lines", 55, 1.6, 0.13)], e50, decisions(
        tx("Full side elevation as authored in the first version: wheel arches, roof, windscreen, door gaps, handles, mirror, lamps, bumpers, exhaust.", "Vollständige Seitenansicht wie in der ersten Fassung erarbeitet: Radkästen, Dach, Windschutzscheibe, Türfugen, Griffe, Spiegel, Leuchten, Stoßfänger, Abgasanlage."),
        tx("Everything: seats and steering wheel behind the glass, five-spoke wheels with discs and calipers, tread marks, panel gaps.", "Alles: Sitze und Lenkrad hinter dem Glas, Fünfspeichenräder mit Scheiben und Sätteln, Profilstriche, Blechfugen."),
        tx("Nothing merged except the fuel flap and bumper lamp, drawn as outlines.", "Nichts zusammengefasst außer Tankklappe und Heckleuchte, als Umrisse gezeichnet."),
        tx("Windscreen wipers, door seals and badges (not modelled).", "Scheibenwischer, Türdichtungen und Embleme (nicht modelliert)."),
        tx("Thinnest elements: handle 0.024 m = 0.48 mm, tread tick 0.016 m = 0.32 mm at the texture pen 0.13 mm.", "Dünnste Elemente: Griff 0,024 m = 0,48 mm, Profilstrich 0,016 m = 0,32 mm mit Texturstift 0,13 mm."),
        tx("Glazing carries a sheen hatch (55 degrees, 1.6 mm).", "Verglasung trägt eine Glanzschraffur (55 Grad, 1,6 mm)."),
        tx("Body mid-tone, glass light, tyres and wheel wells near black, alloy light; three line weights 0.35 / 0.25 / 0.13 mm.", "Aufbau mittlerer Ton, Glas hell, Reifen und Radkästen fast schwarz, Leichtmetall hell; drei Linienstärken 0,35 / 0,25 / 0,13 mm."),
        tx("All regions drawn at their real extent, including calipers and exhaust silencer.", "Alle Regionen in ihrer realen Ausdehnung gezeichnet, einschließlich Sättel und Schalldämpfer.")))
    return R


NOTES = tx(
    "Wheels roll consistently: the pose roots and wheel angles of the clips satisfy theta = -x / r with r = 0.30 m. The side doors hinge about vertical axes and are drawn as panel lines, not as elevation joints; the tailgate and bonnet are true transverse-axis hinges. Thermal: sheet-steel body regions with 6 kJ/(m2 K); engine surfaces, exhaust, tyres and brake discs are heated by heat release (W) at operating level 1, calibrated from typical steady surface rises with total film coefficients of 11 to 22 W/(m2 K), so warm-up is a transient with minutes-scale time constants. Dimensions, masses and loads are authored typical values of the vehicle class (authored-estimate).",
    "Die Räder rollen konsistent: Wurzellage und Radwinkel der Clips erfüllen theta = -x / r mit r = 0,30 m. Die Seitentüren schwenken um vertikale Achsen und sind als Fugenlinien gezeichnet, nicht als Gelenke der Ansicht; Heckklappe und Motorhaube sind echte Scharniere um die Querachse. Thermik: Karosserieregionen aus Stahlblech mit 6 kJ/(m2 K); Motoroberflächen, Abgasanlage, Reifen und Bremsscheiben werden durch Wärmefreisetzung (W) bei Betriebsstufe 1 beheizt, kalibriert aus typischen stationären Oberflächenerwärmungen mit Gesamtübergangskoeffizienten von 11 bis 22 W/(m2 K); das Warmlaufen ist daher ein Übergang mit Zeitkonstanten im Minutenbereich. Abmessungen, Massen und Lasten sind selbst gesetzte typische Werte der Fahrzeugklasse (authored-estimate).")

ASSUMPTIONS = [
    tx("Heat release at level 1 is the part of the waste heat that leaves through the visible surface: engine bay 1.1 kW (about +70 K), exhaust system 2.3 kW (about +230 K), bonnet 0.2 kW, tyres 0.48 kW (about 50 km/h), brake discs 36 kW (full stop from 50 km/h, 96 kJ in 2.7 s). Heat capacities are the thermally active shell masses on a minutes timescale: engine surfaces 9 kJ/(m2 K), exhaust 8.5 kJ/(m2 K), tyres 34 kJ/(m2 K), discs 31 kJ/(m2 K).",
       "Die Wärmefreisetzung bei Stufe 1 ist der Teil der Abwärme, der durch die sichtbare Oberfläche austritt: Motorraum 1,1 kW (etwa +70 K), Abgasanlage 2,3 kW (etwa +230 K), Haube 0,2 kW, Reifen 0,48 kW (etwa 50 km/h), Bremsscheiben 36 kW (Vollbremsung aus 50 km/h, 96 kJ in 2,7 s). Wärmekapazitäten sind die thermisch wirksamen Schalenmassen im Minutenbereich: Motoroberflächen 9 kJ/(m2 K), Abgasanlage 8,5 kJ/(m2 K), Reifen 34 kJ/(m2 K), Scheiben 31 kJ/(m2 K)."),
    tx("Tailpipe at level 1 (full load, about 4000 rpm of a 1.4 L petrol engine): exhaust mass flow rho_a * eta_v * V * N / 2 = 1.19 * 0.85 * 1.4e-3 * 33 = 0.047 kg/s plus fuel, gas at about 300 C (rho 0.6 kg/m3), hence 0.08 m3/s through a 45 mm pipe; the gas carries about 0.075 kg of water vapour per kg (combustion water, Heywood Ch. 4). Flow, temperature rise and vapour scale linearly with the engine load level; this overestimates idle exhaust temperature.",
       "Endrohr bei Stufe 1 (Volllast, etwa 4000 U/min eines 1,4-L-Ottomotors): Abgasmassenstrom rho_a * eta_v * V * N / 2 = 1,19 * 0,85 * 1,4e-3 * 33 = 0,047 kg/s plus Kraftstoff, Gas mit etwa 300 C (rho 0,6 kg/m3), also 0,08 m3/s durch ein 45-mm-Rohr; das Gas führt etwa 0,075 kg Wasserdampf je kg (Verbrennungswasser, Heywood Kap. 4). Volumenstrom, Temperaturerhöhung und Dampf skalieren linear mit der Motorlast; der Leerlaufabgas-Temperatur wird dadurch überschätzt."),
    tx("Sun: the elevation shows one flank, so side panels and glass are half sunlit; roof and bonnet fully; ground contact of the tyres (7 % of the tread area, four patches of 0.085 x 0.37 m) is on asphalt at 30 C. Closed bonnet exposes 20 % of the engine surfaces to air, an open bonnet all of it.",
       "Sonne: die Ansicht zeigt eine Flanke, daher sind Seitenbleche und Glas zur Hälfte besonnt, Dach und Haube ganz; der Bodenkontakt der Reifen (7 % der Lauffläche, vier Aufstandsflächen von 0,085 x 0,37 m) liegt auf Asphalt mit 30 C. Die geschlossene Haube setzt 20 % der Motoroberflächen der Luft aus, die offene alle."),
]

SPEC = dict(sky=8, ground=28, operating=KEYS, regions=REGIONS, emitters=EMITTERS, assumptions=ASSUMPTIONS, defaults=DEFAULTS, contacts=CONTACTS, poses=POSES, fix_clips=fix_clips, reps=reps, notes=NOTES)

if __name__ == "__main__":
    a = assemble("city-car", REL, SPEC)
    if len(sys.argv) > 1:
        previews(a, "car", poses=sys.argv[2:] or None, dens=[int(x) for x in sys.argv[1].split(",")])
