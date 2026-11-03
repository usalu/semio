import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

# World = rig rest pose, root on the ground, arm to +x. The arm works in the plane at lateral LA = -0.25 m (right of the centre
# line, the cab sits on the left). Boom, stick, bucket and all cylinders are extruded solids of their bone-local side-view
# profiles (the same outlines as the elevation), so every pose and clip instant reads in the front view (height range) and in the
# plan (forward range) with the full bone transform.
LA = -0.25

BOOM = "M 0,-0.5 C 0.6,-0.72 1.6,-0.6 2.6,-0.28 C 3.6,0.04 4.8,-0.14 5.7,-0.3 L 5.7,0.3 C 4.8,0.34 3.9,0.75 2.9,0.98 C 2.1,1.1 1.1,0.95 0,0.5 Z"
STICK = "M 0,-0.38 C 0.9,-0.44 2.0,-0.32 2.9,-0.2 L 2.9,0.2 C 2.0,0.3 0.9,0.46 0,0.42 Z"
BUCKET_FULL = "M 1.35,0 L 1.3,0.05 C 1.05,0.62 0.35,0.78 -0.08,0.34 L -0.12,0.06 L 0.02,-0.12 C 0.4,-0.1 0.95,-0.1 1.35,0 Z"
BUCKET_TEETH = "M 1.35,0 L 1.3,0.05 C 1.05,0.62 0.35,0.78 -0.08,0.34 L -0.12,0.06 L 0.02,-0.12 C 0.4,-0.1 0.95,-0.1 1.35,0 L 1.5,-0.03 L 1.28,0.07 Z"


def arm(d):
    """Boom, stick, bucket and cylinders; identical profiles in front and plan, only the extrusion reads differently."""
    s = []
    bucket = BUCKET_FULL if d <= 100 else BUCKET_TEETH
    s += [X(path("bucket-body", "bucket", "body", 3.0, 0.35, bucket, region="attachment", material="wear-steel"), LA - 0.5, LA + 0.5)]
    if d in (50, 100):
        s += [X(poly("bucket-tooth-%d" % i, "bucket", "detail", 3.05, 0.2, [(1.3, 0.07), (1.5, -0.03), (1.28, -0.09)], region="attachment", material="wear-steel"), LA - 0.47 + i * 0.2, LA - 0.35 + i * 0.2) for i in range(5)]
    s += [X(path("stick-body", "stick", "body", 2.9, 0.55, STICK, region="attachment", material="paint-steel"), LA - 0.24, LA + 0.24)]
    s += [X(path("boom-body", "boom", "body", 2.8, 0.55, BOOM, regions=["attachment", "hydraulics"] if d == 500 else None, region=None if d == 500 else "attachment", material="paint-steel"), LA - 0.3, LA + 0.3)]
    if d <= 200:
        for n, o in (("left", 0.5), ("right", -0.5)):
            s += [X(capsule("boom-cyl-barrel-%s" % n, "boom-cyl", "detail", 3.5, 0.4, 0, 0, 1.45, 0, 0.1, 0.1, region="hydraulics", material="paint-steel"), LA + o - 0.1, LA + o + 0.1)]
            s += [X(capsule("boom-rod-%s" % n, "boom-rod", "detail", 3.55, 0.8, 0, 0, 1.65, 0, 0.05 if d >= 200 else 0.035, 0.05 if d >= 200 else 0.035, region="hydraulics", material="chrome"), LA + o - 0.05, LA + o + 0.05)]
        s += [X(capsule("stick-cyl-barrel", "stick-cyl", "detail", 3.5, 0.4, 0, 0, 1.6, 0, 0.09, 0.09, region="hydraulics", material="paint-steel"), LA - 0.09, LA + 0.09),
              X(capsule("stick-rod", "stick-rod", "detail", 3.55, 0.8, 0, 0, 2.35, 0, 0.045, 0.045, region="hydraulics", material="chrome"), LA - 0.045, LA + 0.045),
              X(capsule("bucket-cyl-barrel", "bucket-cyl", "detail", 3.5, 0.4, 0, 0, 0.85, 0, 0.07, 0.07, region="hydraulics", material="paint-steel"), LA - 0.07, LA + 0.07)]
    if d <= 100:
        s += [X(capsule("bucket-rod", "bucket-rod", "detail", 3.55, 0.8, 0, 0, 0.95, 0, 0.03, 0.03, region="hydraulics", material="chrome"), LA - 0.03, LA + 0.03)]
    return s


def front(d):
    s = arm(d)
    if d <= 100:
        s += [rect("boom-foot", "upper", "body", 3.0, 0.4, LA - 0.45, 1.22, 0.9, 0.56, region="house", material="paint-steel")]
    s += [rect("cab-glass", "upper", "glazing", 6.0, 0.85, 0.2, 1.58, 0.95, 1.29, region="cab-glazing", material="glass")]
    if d == 50:
        s += [poly("cab-pillar", "upper", "detail", 6.1, 0.3, [(0.675, 1.58), (0.675, 2.87)], closed=False, material="paint-steel"),
              poly("cab-sill", "upper", "detail", 6.1, 0.3, [(0.1, 1.55), (1.25, 1.55)], closed=False, material="paint-steel")]
    if d <= 200:
        s += [rect("stack", "upper", "detail", 2.4, 0.3, -0.955, 2.25, 0.11, 0.55, region="exhaust-stack", material="exhaust-steel")]
    if d == 50:
        s += [rect("stack-cap", "upper", "detail", 2.45, 0.2, -0.98, 2.78, 0.16, 0.04, material="exhaust-steel"),
              poly("hood-rail", "upper", "detail", 2.3, 0.3, [(-1.15, 2.25), (-1.15, 2.5), (-0.2, 2.5), (-0.2, 2.25)], closed=False, material="paint-steel")]
    if d == 500:
        s += [rect("cab-block", "upper", "body", 2.1, 0.5, 0.1, 1.0, 1.15, 1.97, regions=["house", "engine-compartment", "exhaust-stack", "counterweight"], material="paint-steel"),
              rect("house", "upper", "body", 2.0, 0.5, -1.25, 1.0, 1.35, 1.25, regions=["house", "engine-compartment", "exhaust-stack", "counterweight"], material="paint-steel")]
    else:
        s += [rect("cab-block", "upper", "body", 2.1, 0.62 if d <= 100 else 0.55, 0.1, 1.0, 1.15, 1.97, region="house", material="paint-steel"),
              rect("house", "upper", "body", 2.0, 0.62 if d <= 100 else 0.55, -1.25, 1.0, 1.35, 1.25, region="house", material="paint-steel")]
    for n, lo in (("left", 0.895), ("right", -1.495)):
        s += [rect("track-belt-%s" % n, "undercarriage", "body", 1.0, 0.25, lo, 0, 0.6, 0.86, rr=0.12 if d <= 100 else None, region="tracks", material="track-steel")]
    s += [rect("track-frame", "undercarriage", "detail", 1.5, 0.5, -0.9, 0.25, 1.8, 0.61, region="tracks", material="paint-steel")] if d <= 200 else [rect("track-frame", "undercarriage", "body", 1.5, 0.3, -0.9, 0.25, 1.8, 0.61, region="tracks", material="track-steel")]
    if d <= 100:
        s += [rect("slew-ring", "undercarriage", "detail", 2.6, 0.3, -0.8, 0.86, 1.6, 0.14, material="paint-steel")]
    if d == 50:
        s += [poly("grouser-%s-%d" % (n, i), "undercarriage", "texture", 1.1, 0.4, [(lo, y), (lo + 0.6, y)], closed=False, material="track-steel") for n, lo in (("left", 0.895), ("right", -1.495)) for i, y in enumerate((0.15, 0.3, 0.45, 0.6, 0.72), 1)]
    return s


def plan(d):
    s = arm(d)
    if d <= 100:
        s += [rect("boom-foot", "upper", "body", 3.0, 0.4, 0.78, LA - 0.4, 0.52, 0.8, region="house", material="paint-steel")]
    s += [rect("cab-glass", "upper", "glazing", 6.0, 0.85, 1.2, 0.2, 0.26 if d <= 200 else 0.3, 0.95, region="cab-glazing", material="glass")]
    if d < 500:
        s += [rect("cab-roof", "upper", "body", 2.3, 0.55 if d > 100 else 0.6, 0.12, 0.1, 1.38, 1.1, rr=0.1 if d <= 100 else None, region="house", material="paint-steel")]
    if d <= 200:
        s += [ellipse("stack", "upper", "detail", 2.4, 0.3, -1.2, -0.9, 0.055, 0.055, region="exhaust-stack", material="exhaust-steel"),
              rect("hood-grille", "upper", "detail", 2.2, 0.2, -1.55, -0.9, 1.0, 1.8, region="engine-compartment", material="engine-metal")]
    if d == 50:
        s += [poly("hood-louvre-%d" % i, "upper", "texture", 2.3, 0.3, [(x, -0.85), (x, 0.85)], closed=False, material="engine-metal") for i, x in enumerate((-1.45, -1.3, -1.15, -1.0, -0.85, -0.7), 1)]
    if d == 500:
        s += [rect("house-counterweight", "upper", "body", 2.0, 0.5, -3.0, -1.25, 4.6, 2.5, regions=["house", "engine-compartment", "exhaust-stack", "counterweight"], material="paint-steel"),
              rect("cab-roof", "upper", "body", 2.3, 0.55, 0.12, 0.1, 1.38, 1.1, regions=["house"], material="paint-steel")]
    else:
        s += [rect("house", "upper", "body", 2.0, 0.62 if d <= 100 else 0.55, -1.75, -1.25, 3.35, 2.5, rr=0.15 if d <= 100 else None, region="house", material="paint-steel"),
              rect("counterweight-body", "counterweight", "body", 2.0, 0.35, -3.0, -1.2, 1.3, 2.4, rr=0.4 if d <= 100 else None, region="counterweight", material="cast-iron")]
    for n, lo in (("left", 0.895), ("right", -1.495)):
        s += [rect("track-belt-%s" % n, "undercarriage", "body", 1.0, 0.25, -2.08, lo, 4.16, 0.6, rr=0.3 if d <= 100 else None, region="tracks", material="track-steel")]
        if d <= 100:
            s += [rect("track-frame-%s" % n, "undercarriage", "detail", 1.5, 0.5, -1.7, lo + 0.1, 3.4, 0.4, region="tracks", material="paint-steel")]
    if d == 50:
        s += [poly("grouser-%s-%d" % (n, i), "undercarriage", "texture", 1.1, 0.4, [(x, lo), (x, lo + 0.6)], closed=False, material="track-steel") for n, lo in (("left", 0.895), ("right", -1.495)) for i, x in enumerate([-1.9 + 0.2 * k for k in range(20)], 1)]
    return s


def clean(sh):
    for x in sh:
        for k in ("r", "regions", "region"):
            if k in x and x[k] is None:
                del x[k]
    return sh


D = {
    "50": ("Front view with all moving parts: bucket with five teeth, stick and boom, the boom cylinder pair, the stick and bucket cylinders with chrome rods and the boom foot; every arm part and cylinder is an extruded solid of its side-view outline, so a lowered boom, a curled bucket or an extended rod changes its height range exactly. House with cab block, glass, pillar, hood rail and stack, both track belts with grousers and the centre beam. Plan: the same arm solids seen from above (forward range of each part), boom foot, house, cab roof with the windscreen strip, engine hood with louvres, stack circle, counterweight and the two tracks with grouser lines.",
           "Vorderansicht mit allen beweglichen Teilen: Löffel mit fünf Zähnen, Stiel und Ausleger, das Auslegerzylinderpaar, Stiel- und Löffelzylinder mit Chromkolbenstangen und der Auslegerfuß; jedes Arm- und Zylinderteil ist ein extrudierter Körper seines Seitenansichtsumrisses, sodass ein gesenkter Ausleger, ein eingerollter Löffel oder eine ausgefahrene Stange ihren Höhenbereich genau ändern. Oberwagen mit Kabinenblock, Glas, Pfosten, Haubenreling und Auspuffrohr, beide Laufbänder mit Stollen und der Mittelträger. Grundriss: dieselben Armkörper von oben (Vorwärtsbereich jedes Teils), Auslegerfuß, Oberwagen, Kabinendach mit Frontscheibenstreifen, Motorhaube mit Lamellen, Rohrkreis, Gegengewicht und die zwei Ketten mit Stollenlinien."),
    "100": ("Front view: bucket with five teeth, stick, boom, boom foot and all three cylinders with rods as extruded solids of their side-view outlines, house, cab block with glass, stack, both belts and the centre beam with slew ring. Plan: the same arm solids, boom foot, house, cab roof and windscreen strip, hood grille, stack circle, counterweight and both tracks with their frames.",
            "Vorderansicht: Löffel mit fünf Zähnen, Stiel, Ausleger, Auslegerfuß und alle drei Zylinder mit Kolbenstangen als extrudierte Körper ihrer Seitenansichtsumrisse, Oberwagen, Kabinenblock mit Glas, Auspuffrohr, beide Laufbänder und der Mittelträger mit Drehkranz. Grundriss: dieselben Armkörper, Auslegerfuß, Oberwagen, Kabinendach und Frontscheibenstreifen, Haubengitter, Rohrkreis, Gegengewicht und beide Ketten mit ihren Rahmen."),
    "200": ("Front view: bucket, stick and boom as massive extruded bars, the boom cylinder pair, stick cylinder with rod and the bucket barrel, house with cab block, glass and stack, two belts and the centre beam. Plan: the arm solids from above, cab roof, glass strip, hood grille, stack circle, house, counterweight and both belts.",
            "Vorderansicht: Löffel, Stiel und Ausleger als massive extrudierte Balken, das Auslegerzylinderpaar, Stielzylinder mit Kolbenstange und der Löffelzylinder, Oberwagen mit Kabinenblock, Glas und Auspuffrohr, zwei Laufbänder und der Mittelträger. Grundriss: die Armkörper von oben, Kabinendach, Glasstreifen, Haubengitter, Rohrkreis, Oberwagen, Gegengewicht und beide Laufbänder."),
    "500": ("Front view: two dark belts, one house block with cab glass, and boom, stick and bucket as three extruded bars; stowed it folds in front of the cab. Plan: two belts, one house and counterweight block, the cab roof with its glass strip and the three arm bars.",
            "Vorderansicht: zwei dunkle Laufbänder, ein Oberwagenblock mit Kabinenglas und Ausleger, Stiel und Löffel als drei extrudierte Balken; gestaut klappt er vor der Kabine zusammen. Grundriss: zwei Laufbänder, ein Block aus Oberwagen und Gegengewicht, das Kabinendach mit Glasstreifen und die drei Armbalken."),
}
OM = {
    "50": ("In the front view the engine compartment, counterweight, rollers, sprocket and idler are hidden behind the house and tracks; the arm is drawn on its own lateral line, right of the cab; weld lines are not drawn in the front and plan.",
           "In der Vorderansicht sind Motorraum, Gegengewicht, Rollen, Antriebs- und Leitrad hinter Oberwagen und Ketten verdeckt; der Arm liegt auf seiner eigenen Querlinie rechts der Kabine; Schweißnähte sind in Vorderansicht und Grundriss nicht gezeichnet."),
    "100": ("Engine compartment, counterweight and rollers are hidden in the front view; no grousers.", "Motorraum, Gegengewicht und Rollen sind in der Vorderansicht verdeckt; keine Stollen."),
    "200": ("Bucket rod, pins and bosses; counterweight and engine compartment are hidden in the front view.", "Löffelkolbenstange, Bolzen und Naben; Gegengewicht und Motorraum sind in der Vorderansicht verdeckt."),
    "500": ("Cylinders (as in the side view), stack, hood grille and pins in both views.", "Zylinder (wie in der Seitenansicht), Auspuffrohr, Haubengitter und Bolzen in beiden Ansichten."),
}
plan_ = {str(d): {"front": clean(front(d)), "plan": clean(plan(d)), "dec": {"retained": D[str(d)], "omitted": OM[str(d)]}} for d in (50, 100, 200, 500)}
apply("excavator", plan_)
