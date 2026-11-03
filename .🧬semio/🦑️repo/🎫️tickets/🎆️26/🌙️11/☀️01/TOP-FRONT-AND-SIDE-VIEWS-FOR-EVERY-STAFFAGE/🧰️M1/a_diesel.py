import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

# World = rig rest pose, root on the ground. Front = seen from the discharge-hood end (+x end), plan = from above, left = +lateral.
# Container 6.06 x 2.44 x 2.59 m; roof silencer axis 0.9 m to the right (-lateral) of the centre line, stack on that axis.
# The rain cap and the outlet flap are extruded solids: their side-view profile (bone-local, as in the elevation) spans the lateral
# width and follows the full bone transform, so the plates stand up in the front view and shorten in the plan when they open.
SL = -0.9
TF = {50: 0.06, 100: 0.06, 200: 0.07, 500: 0.1}
TC = {50: 0.1, 100: 0.1, 200: 0.07, 500: 0.1}


def outlet_flap(d):
    t = TF[d]
    return X(poly("outlet-flap", "outlet-flap", "body", 2.6, 0.3, [(0, -t), (0.63, -t), (0.63, 0), (0, 0)], region="outlet-flap", material="galvanised-steel"), -0.85, 0.85)


def rain_cap(d):
    t = TC[d]
    return X(poly("rain-cap", "cap-flap", "body", 2.7, 0.3, [(0, -t), (0.3, -t), (0.3, 0), (0, 0)], region="rain-cap", material="galvanised-steel"), SL - 0.15, SL + 0.15)


def front(d):
    s = []
    s += [outlet_flap(d)]
    s += [rect("outlet-mouth", "outlet-hood", "detail", 2.55, 0.15, -0.85, 2.65, 1.7, 0.55, region="discharge-hood", material="container-steel")] if d <= 200 else []
    s += [rect("discharge-hood", "outlet-hood", "body", 2.5, 0.4, -0.9, 2.59, 1.8, 0.66, region="discharge-hood", material="container-steel")]
    s += [rain_cap(d)]
    s += [rect("exhaust-stack", "stack", "body", 2.6, 0.2, SL - 0.1, 3.14, 0.2, 0.33, region="exhaust-stack", material="exhaust-steel")]
    if d == 500:
        s += [ellipse("silencer", "silencer", "body", 2.5, 0.35, SL, 2.865, 0.275, 0.275, region="silencer", material="exhaust-steel")]
        s += [rect("container-body", "container", "body", 2.0, 0.5, -1.22, 0, 2.44, 2.59, regions=["enclosure-walls", "roof", "inlet-louvres", "skid-tank"], material="container-steel")]
        return s
    s += [ellipse("silencer", "silencer", "body", 2.5, 0.35, SL, 2.865, 0.275, 0.275, region="silencer", material="exhaust-steel")]
    if d in (50, 100):
        s += [rect("end-door", "container", "detail", 2.2, 0.5, -0.5, 0.45, 1.0, 1.7, region="enclosure-walls", material="container-steel", filled=False),
              rect("corner-casting-left", "container", "detail", 1.95, 0.2, 0.97, 0, 0.25, 0.2, region="skid-tank", material="container-steel"),
              rect("corner-casting-right", "container", "detail", 1.95, 0.2, -1.22, 0, 0.25, 0.2, region="skid-tank", material="container-steel")]
    s += [rect("roof-band", "container", "body", 2.1, 0.4, -1.22, 2.49, 2.44, 0.1, region="roof", material="container-steel"),
          rect("wall", "container", "body", 2.0, 0.55, -1.22, 0.2, 2.44, 2.29, region="enclosure-walls", material="container-steel"),
          rect("skid-tank", "container", "body", 1.9, 0.3, -1.22, 0, 2.44, 0.2, region="skid-tank", material="container-steel")]
    if d == 200:
        s += [rect("end-louvre", "container", "detail", 2.2, 0.25, -0.9, 0.55, 1.8, 1.5, region="inlet-louvres", material="aluminium")] if False else []
    return s


def plan(d):
    s = []
    s += [outlet_flap(d), rain_cap(d)]
    rs = 0.1 if d == 500 else 0.1
    s += [ellipse("exhaust-stack", "stack", "body", 2.65, 0.2, -1.2, SL, rs, rs, region="exhaust-stack", material="exhaust-steel")]
    s += [rect("discharge-hood", "outlet-hood", "body", 2.5, 0.4, 1.9, -0.9, 1.13, 1.8, region="discharge-hood", material="container-steel")]
    if d == 50:
        s += [poly("hood-ridge", "outlet-hood", "detail", 2.55, 0.3, [(2.2, -0.9), (2.2, 0.9)], closed=False, region="discharge-hood", material="container-steel")]
    s += [rect("silencer", "silencer", "body", 2.5, 0.35, -2.0, SL - 0.275, 2.2, 0.55, rr=0.275, region="silencer", material="exhaust-steel")]
    if d in (50, 100):
        s += [rect("silencer-clamp-%d" % i, "silencer", "detail", 2.55, 0.2, x, SL - 0.285, 0.12, 0.57, region="silencer", material="exhaust-steel") for i, x in enumerate((-1.6, -0.5), 1)]
        s += [rect("roof-edge", "container", "detail", 2.1, 0.4, -2.93, -1.12, 5.86, 2.24, region="roof", material="container-steel", filled=False)]
    if d == 500:
        s += [rect("container-body", "container", "body", 2.0, 0.5, -3.03, -1.22, 6.06, 2.44, regions=["enclosure-walls", "roof", "inlet-louvres", "skid-tank"], material="container-steel")]
    else:
        s += [rect("roof", "container", "body", 2.0, 0.45, -3.03, -1.22, 6.06, 2.44, regions=["roof", "enclosure-walls", "skid-tank"] if d == 200 else ["roof"], material="container-steel")]
    if d in (50, 100):
        s += [rect("corner-casting-%s" % n, "container", "detail", 2.05, 0.2, x, y, 0.25, 0.25, region="skid-tank", material="container-steel") for n, x, y in (("1", -3.03, -1.22), ("2", 2.78, -1.22), ("3", -3.03, 0.97), ("4", 2.78, 0.97))]
        s += [rect("inlet-louvre-hint", "container", "detail", 2.05, 0.25, 0, 0, 0.01, 0.01, region="inlet-louvres", material="aluminium")] if False else []
    return s


def clean(sh):
    for x in sh:
        pass
    return sh


D = {
    "50": ("Front view from the discharge-hood end: skid, wall with an end door outline and corner castings, roof band, the hood with its dark mouth, the outlet flap as a plate that shortens towards the hinge as it swings out, the silencer peeking out beside the hood, the stack and the rain cap plate that rises when the cap opens. Plan: roof with its edge outline and four castings, silencer with two clamps and the stack circle, hood with its ridge, the rain cap and the outlet flap as plates (extruded side-view profiles) that shorten towards their hinges as they open.",
           "Vorderansicht vom Ausblasende: Rahmen, Wand mit Stirntür-Umriss und Eckbeschlägen, Dachband, die Haube mit dunkler Mündung, die Ausblasklappe als Platte, die beim Ausschwenken zum Scharnier hin kürzer wird, der Schalldämpfer, der neben der Haube hervorschaut, das Rohr und die Regenkappenplatte, die sich beim Öffnen aufstellt. Grundriss: Dach mit Randumriss und vier Beschlägen, Schalldämpfer mit zwei Schellen und dem Rohrkreis, Haube mit Firstlinie, die Regenkappe und die Ausblasklappe als Platten (extrudierte Seitenansichtsprofile), die beim Öffnen zum Scharnier hin kürzer werden."),
    "100": ("Front view: skid, wall with end door outline and castings, roof band, hood with mouth, outlet flap plate, silencer, stack and rain cap plate. Plan: roof with edge outline and castings, silencer with clamps, stack circle, hood, the rain cap and outlet flap plates.",
            "Vorderansicht: Rahmen, Wand mit Stirntür-Umriss und Beschlägen, Dachband, Haube mit Mündung, Ausblasklappenplatte, Schalldämpfer, Rohr und Regenkappenplatte. Grundriss: Dach mit Randumriss und Beschlägen, Schalldämpfer mit Schellen, Rohrkreis, Haube, Regenkappenbalken und der Rahmen der Ausblasklappe."),
    "200": ("Front view: base, wall, roof band, hood with mouth, outlet flap plate, silencer, stack and rain cap plate. Plan: roof block, silencer, stack circle, hood, the rain cap and outlet flap plates.",
            "Vorderansicht: Sockel, Wand, Dachband, Haube mit Mündung, Ausblasklappenplatte, Schalldämpfer, Rohr und Regenkappenplatte. Grundriss: Dachblock, Schalldämpfer, Rohrkreis, Haube, die Platten von Regenkappe und Ausblasklappe."),
    "500": ("Front view: one container block, the silencer disc beside the hood, the hood, the stack and the two moving flap plates. Plan: one container block, silencer, stack circle, hood, the rain cap and outlet flap plates.",
            "Vorderansicht: ein Containerblock, die Schalldämpferscheibe neben der Haube, die Haube, das Rohr und die zwei beweglichen Klappenplatten. Grundriss: ein Containerblock, Schalldämpfer, Rohrkreis, Haube, die Platten von Regenkappe und Ausblasklappe."),
}
OM = {
    "50": ("The inlet louvre fields sit on the long sides and are edge-on in the front view and plan; the front rain cap is drawn 0.1 m thick at every scale.",
           "Die Zuluft-Lamellenfelder liegen an den Längsseiten und stehen in Vorderansicht und Grundriss auf der Kante; die Regenkappe ist in der Vorderansicht in jedem Maßstab 0,1 m dick gezeichnet."),
    "100": ("Louvre fields (edge-on), clamps in the front view.", "Lamellenfelder (auf der Kante), Schellen in der Vorderansicht."),
    "200": ("Door outline, castings and clamps.", "Türumriss, Beschläge und Schellen."),
    "500": ("In both views walls, louvres, base tank, castings and clamps merge into the container block.", "In beiden Ansichten gehen Wände, Lamellen, Sockeltank, Beschläge und Schellen im Containerblock auf."),
}
plan_ = {str(d): {"front": front(d), "plan": plan(d), "dec": {"retained": D[str(d)], "omitted": OM[str(d)]}} for d in (50, 100, 200, 500)}
apply("diesel-generator", plan_)
