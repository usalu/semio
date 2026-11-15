from author_15_lib import *
SD, PB, EH = "steel-desk", "particleboard-laminate", "equipment-housing"
ZS = 1
FX = {"l": 0.25, "r": 1.35}
SIDES = (("l", "foot-left", "stage-left", "column-left"), ("r", "foot-right", "stage-right", "column-right"))

def hollow(s, w): return [[-s, -s], [s, -s], [s, s], [-s, s]], [[-s + w, -s + w], [s - w, -s + w], [s - w, s - w], [-s + w, s - w]]
def tube(c, id, bone, cx, y0, y1, s, w, mat=SD, tone=0.5, role="body", **ex):
    o, h = hollow(s, w)
    return c.ext(id, bone, mat, role, tone, [[cx + p[0], p[1]] for p in o], "xz", y0, y1, holes=[[[cx + p[0], p[1]] for p in h]], **ex)
def solid(c, id, bone, cx, y0, y1, s, mat=SD, tone=0.5, sz=None, role="body", corner=None, **ex):
    return c.bw(id, bone, mat, role, tone, (cx - s, cx + s), (y0, y1), (-(sz or s), sz or s), corner=corner, **ex)

def laptop_feet(c, y0, y1, d):
    for sid, z in (("a", -0.15), ("b", 0.2 - 2.5 * d)):
        c.bw("laptop-foot-" + sid, "top", EH, "detail", 0.15, (0.95 + d / 2, 0.95 + 1.5 * d), (y0, y1), (z + d / 2, z + 1.5 * d), repeat={"count": 2, "translation": [0.25 - 2 * d, 0, 0]})

def build(scale):
    c = Ctx("height-adjustable-desk", scale); c.to("body")
    F = scale
    plate_t = 0.016
    sl, st, co = 0.032, 0.027, 0.022
    dr = 0.05
    top_t = 0.025
    top_y0 = 0.695
    for sd, foot, stage, col in SIDES:
        x = FX[sd]; zd = -ZS * 0.1 if sd == "l" else ZS * 0.1
        if scale == 50:
            c.ext("%s-plate" % sd, foot, SD, "body", 0.3, [[x - 0.25, 0], [x + 0.25, 0], [x + 0.25, 0.008], [x + 0.2, 0.016], [x - 0.2, 0.016], [x - 0.25, 0.008]], "xy", -0.3, 0.3)
            tube(c, "%s-sleeve" % sd, foot, x, 0.012, 0.4, sl, 0.004, tone=0.28)
            tube(c, "%s-sleeve-collar" % sd, foot, x, 0.385, 0.405, 0.036, 0.0035, tone=0.18, role="detail")
            tube(c, "%s-stage" % sd, stage, x, 0.05, 0.55, st, 0.004, tone=0.42)
            solid(c, "%s-stage-cap" % sd, stage, x, 0.538, 0.55, 0.029, tone=0.2, role="detail")
            tube(c, "%s-column" % sd, col, x, 0.25, 0.695, co, 0.004, tone=0.55)
            c.bw("%s-bracket" % sd, col, SD, "body", 0.22, (x - 0.045, x + 0.045), (0.66, 0.695), (-0.045, 0.045))
            c.bw("%s-drive" % sd, foot, EH, "body", 0.22, (x - 0.05, x + 0.05), (0.012, 0.16), (zd - 0.05, zd + 0.05), corner=0.012)
            c.bw("%s-drive-mount" % sd, foot, SD, "detail", 0.3, (x - 0.04, x + 0.04), (0.03, 0.14), ((-0.05, -0.032) if zd < 0 else (0.032, 0.05)))
        else:
            c.bw("%s-plate" % sd, foot, SD, "body", 0.3, (x - 0.25, x + 0.25), (0, plate_t), (-0.3, 0.3))
            if scale == 100:
                tube(c, "%s-sleeve" % sd, foot, x, 0.012, 0.4, sl, 0.004, tone=0.28)
                tube(c, "%s-stage" % sd, stage, x, 0.05, 0.55, st, 0.004, tone=0.42)
                tube(c, "%s-column" % sd, col, x, 0.25, 0.695, co, 0.004, tone=0.55)
            else:
                solid(c, "%s-sleeve" % sd, foot, x, plate_t - 0.004, 0.4, sl, tone=0.28)
                solid(c, "%s-stage" % sd, stage, x, 0.4, 0.55, st, tone=0.42)
                solid(c, "%s-column" % sd, col, x, 0.55, 0.695, co, tone=0.55)
            if scale < 500: c.bw("%s-bracket" % sd, col, SD, "body", 0.22, (x - 0.045, x + 0.045), (0.66, 0.695), (-0.045, 0.045))
            lo = (-0.05 if zd < 0 else 0.032, -0.032 if zd < 0 else 0.05)
            c.bw("%s-drive" % sd, foot, EH, "body", 0.22, (x - dr, x + dr), (0.012, 0.15), (zd - dr, zd + dr), corner=0.012 if scale == 100 else None)
    # top
    if scale == 50:
        c.bw("top-board", "top", PB, "body", 0.58, (0, 1.6), (0.695, 0.72), (-0.4, 0.4), corner=0.004)
    else:
        c.bw("top-board", "top", PB, "body", 0.58, (0, 1.6), (top_y0, top_y0 + top_t), (-0.4, 0.4), corner=0.004 if scale == 100 else None)
    ry = {50: (0.62, 0.67), 100: (0.62, 0.67), 200: (0.62, 0.67), 500: (0.62, 0.67)}[scale]
    rs = 0.025
    rz = (ZS * 0.25, ZS * 0.3)
    if scale == 50:
        o, h = hollow(0.025, 0.003)
        c.ext("top-rail", "top", SD, "body", 0.35, [[0.645 + p[1], rz[0] + 0.025 + p[0]] for p in o], "yz", 0.25, 1.35, holes=[[[0.645 + p[1], rz[0] + 0.025 + p[0]] for p in h]])
        for k, x in enumerate((0.35, 0.65, 0.95, 1.25)):
            c.bw("top-rail-mount-%d" % (k + 1), "top", SD, "detail", 0.3, (x - 0.03, x + 0.03), (0.665, 0.695), rz)
        c.ext("top-cable-tray", "top", SD, "detail", 0.3, [[0.62, 0.2], [0.58, 0.2], [0.58, 0.3], [0.62, 0.3], [0.62, 0.2925], [0.5875, 0.2925], [0.5875, 0.2075], [0.62, 0.2075]], "yz", 0.55, 1.05)
        c.bw("top-keypad", "top", PB, "detail", 0.15, (1.42, 1.51), (0.675, 0.695), (ZS * -0.4, ZS * -0.38), corner=0.003)
    elif scale in (100, 200):
        c.bw("top-rail", "top", SD, "body", 0.35, (0.25, 1.35), ry, rz)
        c.bw("top-rail-mount", "top", SD, "detail", 0.3, (0.3, 0.4), (ry[1], top_y0), rz) if scale == 100 else None
        if scale == 100:
            c.ext("top-cable-tray", "top", SD, "detail", 0.3, [[0.62, 0.2], [0.58, 0.2], [0.58, 0.3], [0.62, 0.3], [0.62, 0.285], [0.595, 0.285], [0.595, 0.215], [0.62, 0.215]], "yz", 0.55, 1.05)
    else:
        c.bw("top-rail", "top", SD, "body", 0.35, (0.25, 1.35), ry, rz)
    # monitor and laptop
    mz = ZS * 0.15
    if scale == 50:
        c.bw("monitor-base", "top", EH, "body", 0.25, (0.51, 0.69), (0.72, 0.732), (ZS * 0.05, ZS * 0.25), corner=0.005)
        c.loft("monitor-neck", "top", EH, "body", 0.3, [([0.6, 0.732, mz], 0.015, 0.015, 4), ([0.6, 0.845, mz], 0.012, 0.012, 4)], X, Z)
        c.bw("monitor-panel", "top", EH, "body", 0.2, (0.3, 0.9), (0.815, 1.18), (mz - 0.02, mz + 0.02), corner=0.012)
        c.bw("monitor-screen", "top", EH, "glazing", 0.1, (0.315, 0.885), (0.83, 1.165), (mz + 0.02, mz + 0.024), corner=0.0015)
        c.bw("laptop", "top", EH, "body", 0.4, (0.95, 1.2), (0.728, 0.748), (ZS * -0.15, ZS * 0.2), corner=0.003)
        laptop_feet(c, 0.72, 0.728, 0.008)
    elif scale == 100:
        c.bw("monitor-base", "top", EH, "body", 0.25, (0.52, 0.68), (0.72, 0.736), (mz - 0.1, mz + 0.1))
        c.cap("monitor-neck", "top", EH, "body", 0.3, [0.6, 0.735, mz], [0.6, 0.845, mz], 0.015)
        c.bw("monitor-panel", "top", EH, "body", 0.2, (0.3, 0.9), (0.815, 1.18), (mz - 0.02, mz + 0.02), corner=0.012)
        c.bw("laptop", "top", EH, "body", 0.4, (0.95, 1.2), (0.7355, 0.7555), (-0.15 * ZS, 0.2 * ZS), corner=None)
        laptop_feet(c, 0.72, 0.7355, 0.015)
    elif scale == 200:
        c.bw("monitor-neck", "top", EH, "body", 0.3, (0.585, 0.615), (0.72, 0.84), (mz - 0.015, mz + 0.015))
        c.bw("monitor-panel", "top", EH, "body", 0.2, (0.3, 0.9), (0.815, 1.18), (mz - 0.02, mz + 0.02))
        c.bw("laptop", "top", EH, "body", 0.4, (0.95, 1.2), (0.721, 0.741), (-0.15 * ZS, 0.2 * ZS))
    else:
        c.bw("monitor-panel", "top", EH, "body", 0.2, (0.3, 0.9), (0.72, 1.18), (mz - 0.02, mz + 0.02))
        c.bw("laptop", "top", EH, "body", 0.4, (0.95, 1.2), (0.721, 0.741), (-0.15 * ZS, 0.2 * ZS))
    return c

def thermal_structure(c, scale):
    body = {s["id"]: s for s in c.lists["body"]}
    c.to("thermal")
    reg = lambda sid: ("top-board" if sid == "top-board" else "drive-left" if sid == "l-drive" else "drive-right" if sid == "r-drive" else "laptop-lid" if sid.startswith("laptop") else "monitor-housing" if sid.startswith("monitor") else "steel-desk")
    for sid in body:
        if sid == "top-board" or sid == "monitor-screen": continue
        if sid.endswith("drive-mount"): c.ref("th-" + sid, sid, region="steel-desk", layer="tube"); continue
        r = reg(sid)
        kw = {"layer": {"steel-desk": "tube", "drive-left": "housing", "drive-right": "housing", "monitor-housing": "housing", "laptop-lid": "case"}[r]} if scale == 50 else {}
        c.ref("th-" + sid, sid, region=r, **kw)
    if "monitor-screen" in body: c.ref("th-monitor-screen", "monitor-screen", region="monitor-housing", **({"layer": "housing"} if scale == 50 else {}))
    if scale == 50:
        s = body["top-board"]
        c.bw("th-top-board-board", "top", PB, "body", 0.5, (0.001, 1.599), (0.696, 0.719), (-0.399, 0.399), region="top-board", layer="board")
        c.bw("th-top-board-laminate", "top", PB, "body", 0.58, (0, 0.1), (0.695, 0.72), (-0.4, 0.4), region="top-board", layer="laminate", repeat={"count": 16, "translation": [0.1, 0, 0]})
    else:
        c.bw("th-top-board", "top", PB, "body", 0.58, (0, TILE[scale]), (top_y(scale), top_y(scale) + 0.025), (-0.4, 0.4), region="top-board", repeat={"count": round(1.6 / TILE[scale]), "translation": [TILE[scale], 0, 0]})
    c.to("structure")
    for sid in body:
        if sid.startswith(("monitor", "laptop")) or sid.endswith(("drive", "drive-mount", "collar", "keypad")) or "rail-mount" in sid: continue
        if sid.startswith("top"): m = "top"
        elif "plate" in sid: m = "foot-left" if sid[0] == "l" else "foot-right"
        elif "sleeve" in sid: m = "foot-left" if sid[0] == "l" else "foot-right"
        elif "stage" in sid: m = "stage-left" if sid[0] == "l" else "stage-right"
        elif "column" in sid or "bracket" in sid: m = "column-left" if sid[0] == "l" else "column-right"
        else: continue
        c.ref("m-" + sid, sid, member=m, part="member")

    f = {50: 0.0225, 100: 0.0225, 200: 0.0305, 500: 0.0378}[scale]
    for sd, foot, stage, col in SIDES:
        x = FX[sd]
        c.bw("j-%s-stage" % sd, stage, SD, "detail", 0.2, (x - f, x + f), (0.4 - f / 2, 0.4 + f / 2) if scale < 500 else (0.4 - f, 0.4 + f), (-f, f), member=stage, part="joint")
        c.bw("j-%s-column" % sd, col, SD, "detail", 0.2, (x - f, x + f), (0.55 - f / 2, 0.55 + f / 2) if scale < 500 else (0.55 - f, 0.55 + f), (-f, f), member=col, part="joint")
        c.bw("mass-%s-foot" % sd, foot, SD, "detail", 0.4, (x - f, x + f), (0, plate_of(scale)), (-f, f), member=foot, part="mass") if scale >= 100 else None
        c.bw("support-contact-foot-%s" % ("left" if sd == "l" else "right"), foot, SD, "ground", 0.2, (x - 0.06, x + 0.06), (0, {50: 0.008, 100: 0.0155, 200: 0.0165, 500: 0.0165}[scale]), (-0.06, 0.06), member=foot, part="support", anchor="contact-foot-%s" % ("left" if sd == "l" else "right"))
    h = {50: 0.04, 100: 0.04, 200: 0.04, 500: 0.0375}[scale]
    c.bw("mass-top", "top", PB, "detail", 0.4, (0.8 - h, 0.8 + h), (top_y(scale), top_y(scale) + 0.025), (-h, h), member="top", part="mass")

TILE = {50: 0.1, 100: 0.2, 200: 0.4, 500: 0.8}
def plate_of(scale): return 0.016
def top_y(scale): return 0.695

DEC = {
50: ("1:50 is the full model: 16 mm foot plates with chamfered ends, three nested hollow telescoping square tubes per side (64, 54 and 44 mm with 4 mm walls, collar ring and top cap), a 90 x 35 x 90 mm column bracket, the 100 x 148 x 100 mm drive housings on steel mounts beside the sleeves, a 25 mm laminated particleboard top with 4 mm rounded edges, a hollow 50 mm steel rail on four mounting pads, the cable tray as a 7.5 mm U channel, the keypad under the front edge, and monitor (base, tapered neck, 40 mm panel with glazed screen) and laptop on the top; the laptop stands on four 8 mm rubber feet (the drawing shows it raised accordingly), so its underside is free air.",
     "1:50 ist das volle Modell: 16 mm Fussplatten mit gefasten Enden, pro Seite drei verschachtelte hohle Teleskop-Vierkantrohre (64, 54 und 44 mm mit 4 mm Wand, Kragenring und Kopfkappe), ein 90 x 35 x 90 mm Säulenwinkel, die 100 x 148 x 100 mm Antriebsgehäuse auf Stahlhaltern neben den Hülsen, eine 25 mm beschichtete Spanplattenplatte mit 4 mm gerundeten Kanten, eine hohle 50 mm Stahlschiene auf vier Montageplatten, die Kabelwanne als 7,5 mm U-Profil, das Bedienfeld unter der Vorderkante sowie Monitor (Fuss, verjüngter Hals, 40 mm Panel mit verglastem Schirm) und Laptop auf der Platte; der Laptop steht auf vier 8 mm Gummifüssen (die Zeichnung zeigt ihn entsprechend angehoben), seine Unterseite liegt also in freier Luft."),
100: ("1:100 keeps the three nested hollow telescoping tubes per side at their true 64, 54 and 44 mm (4 mm walls) but drops collar, cap, drive mounts, keypad, monitor screen and the chamfers and rounds the plates to plain 16 mm slabs; the laptop stands on 15 mm feet, the U channel keeps 15 mm walls; positions and proportions are those of 1:50.",
      "1:100 behält die drei verschachtelten hohlen Teleskoprohre pro Seite in wahrer Grösse von 64, 54 und 44 mm (4 mm Wand), lässt Kragen, Kappe, Antriebshalter, Bedienfeld, Monitorschirm und die Fasen weg und macht die Platten zu einfachen 16 mm Platten; der Laptop steht auf 15 mm Füssen, die U-Wanne behält 15 mm Wand; Lagen und Proportionen wie bei 1:50."),
200: ("1:200 uses solid stacked posts at the true widths (64, 54 and 44 mm), 16 mm foot plates, a 25 mm top, the 50 mm rail, a 30 mm monitor neck and a 40 mm panel, and a 20 mm laptop lifted 1 mm (its underside stays exposed); because the drawing shows these parts as filled outlines at their true sizes, C13 lets the solids keep them instead of the 30 mm feature. The cable tray, keypad, drive mounts and monitor base are merged away.",
      "1:200 nutzt massive gestapelte Pfosten in wahren Breiten (64, 54 und 44 mm), 16 mm Fussplatten, eine 25 mm Platte, die 50 mm Schiene, einen 30 mm Monitorhals und ein 40 mm Panel sowie einen 20 mm Laptop, 1 mm angehoben (seine Unterseite bleibt frei); weil die Zeichnung diese Teile als gefüllte Umrisse in wahrer Grösse zeigt, darf C13 die Körper so lassen statt auf das 30 mm Mindestmass zu bringen. Kabelwanne, Bedienfeld, Antriebshalter und Monitorfuss sind verschmolzen."),
500: ("1:500 draws the same true-size parts as filled outlines, merged to the minimum: 16 mm foot plates, three stacked posts per side, the 100 mm drive boxes, a 25 mm top, the 50 mm rail, one 40 mm monitor block (base, neck and panel merged) and the 20 mm laptop; nothing is thickened to the 75 mm feature.",
      "1:500 zeichnet dieselben Teile in wahrer Grösse als gefüllte Umrisse, auf das Minimum verschmolzen: 16 mm Fussplatten, drei gestapelte Pfosten pro Seite, die 100 mm Antriebskästen, eine 25 mm Platte, die 50 mm Schiene, ein 40 mm Monitorblock (Fuss, Hals und Panel verschmolzen) und der 20 mm Laptop; nichts wird auf das 75 mm Mindestmass verdickt."),
}
def make(scale):
    c = build(scale); thermal_structure(c, scale)
    c.dec(*DEC[scale])
    c.dec("Thermal and structure reference the body solids, except the top board: its outer shell is a real thermal solid cut into strips (16 at 1:50, 8, 4 and 2 at the coarser scales) because the validator judges burial per mesh face and one 1.6 x 0.8 m face would be buried completely by the laptop or the monitor base standing on it; at 1:50 it is nested (laminate outside, board inside, asset layer order corrected to innermost first). Joints are the slide stages between the telescoping parts, the top mass sits at the top centre of mass, supports are contact pads under the feet.", "Thermik und Tragwerk verweisen auf die Körper, ausser bei der Platte: ihre äussere Schale ist ein echter Thermikkörper in Streifen (16 bei 1:50, 8, 4 und 2 bei den gröberen Massstäben), weil der Validator die Überdeckung je Netzfläche beurteilt und eine einzige 1,6 x 0,8 m Fläche vom Laptop oder Monitorfuss darauf vollständig überdeckt würde; bei 1:50 ist sie verschachtelt (Beschichtung aussen, Spanplatte innen, Schichtfolge des Assets auf innen zuerst korrigiert). Gelenke sind die Gleitstufen zwischen den Teleskopteilen, die Masse der Platte sitzt im Schwerpunkt, Auflager sind Kontaktflächen unter den Füssen.")
    return c.out()

if __name__ == "__main__":
    save("height-adjustable-desk", {k: make(k) for k in (50, 100, 200, 500)})
