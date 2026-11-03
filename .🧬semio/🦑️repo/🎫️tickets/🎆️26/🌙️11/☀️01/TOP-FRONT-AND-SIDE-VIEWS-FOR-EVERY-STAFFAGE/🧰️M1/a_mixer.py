import sys, os, math
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

# World = rig rest pose, root on the ground, cab to +x. Front x = lateral (left +x); plan x = forward, plan y = lateral (left +y).
# Drum: axis from (1.9, 2.1375) rising to the rear at 14 deg; radius profile as in the side view.
# Front: the drum shows as a tilted-circle capsule above the cab; the chute and its extension stretch between pivot and tip (followTip)
# and therefore appear above the drum when raised. Plan: the chute pair are capsules that stretch along their bones (followTip);
# the spiral flights ride on the drum-spiral bone and slide along the drum axis.
DX, DY = 1.9, 2.1375
C14, S14 = math.cos(math.radians(14)), math.sin(math.radians(14))
YA = DY
YB = DY + 3 * S14
PROFILE_FULL = [(0, 0), (0.05, 0.3681), (0.14, 0.5999), (0.28, 0.8119), (0.45, 0.9699), (0.65, 1.0759), (0.9, 1.12), (3, 1.12), (5.4, 0.5)]
PROFILE_COARSE = [(0, 0), (0.14, 0.5999), (0.45, 0.9699), (0.9, 1.12), (3, 1.12), (5.4, 0.5)]
WHEELS = [("1", 3.55), ("2", 1.95), ("3", -1.7), ("4", -3.05)]


def px(s):
    return DX - 0.9703 * s


def drum_solid(id, profile, z, **kw):
    """The drum as a solid of revolution about its own tilted axis (the drum bone): half profile (distance along the axis, radius)."""
    shape = poly(id, "drum", "body", z, 0.3, list(profile) + [(profile[-1][0], 0), (0, 0)], **kw)
    shape["solid"] = {"kind": "revolved", "axisBone": "drum", "axisX": DX, "axisZ": 0, "mirror": True}
    return shape


def flight(k, s0, w=0.08):
    sc = lambda t: s0 - 0.85 * t
    return [(px(sc(0.5) - w), 0.5), (px(sc(-0.5) - w), -0.5), (px(sc(-0.5) + w), -0.5), (px(sc(0.5) + w), 0.5)]


CHUTE_TIP = (-3.9 + 2.4 * math.cos(math.radians(40)), 2.2 + 2.4 * math.sin(math.radians(40)))
EXT_TIP = (CHUTE_TIP[0] + 1.3 * math.cos(math.radians(220)), CHUTE_TIP[1] + 1.3 * math.sin(math.radians(220)))


def chute(z_ext, z_main):
    return [X(poly("chute-ext-folded", "chute-ext", "body", z_ext, 0.6, [(0, 0), (1.3, 0), (1.3, 0.22), (0, 0.22)], region="chute", material="galvanised-steel"), -0.3, 0.3),
            X(poly("chute-main", "chute", "body", z_main, 0.5, [(0, -0.14), (2.4, -0.1), (2.4, 0.14), (0, 0.14)], region="chute", material="galvanised-steel"), -0.275, 0.275)]


def front(d):
    s = []
    # chute pair: behind the drum, visible above it when raised
    s += chute(1.95, 1.9)
    s += [drum_solid("drum", PROFILE_FULL if d <= 100 else PROFILE_COARSE, 2.0, region="drum", material="drum-paint") if d < 500 else drum_solid("drum-body", PROFILE_COARSE, 2.0, regions=["drum", "drum-drive"], material="drum-paint")]
    if d in (50, 100):
        s += [ellipse("drum-seam", "drum", "detail", 2.1, 0.3, 0, YB, 1.12, 1.0867, region="drum", material="drum-paint", filled=False)]
    # stack behind the cab, peeking over the roof
    s += [rect("exhaust-stack", "chassis", "body", 2.2, 0.2, -1.2, 1.3, 0.2, 2.05, region="exhaust-stack", material="exhaust-steel")]
    if d in (50, 100):
        s += [rect("stack-cap", "chassis", "detail", 2.3, 0.25, -1.22, 3.35, 0.24, 0.1, region="exhaust-stack", material="exhaust-steel")]
    # cab front and bumper
    if d == 500:
        s += [rect("cab-glass", "chassis", "glazing", 3.5, 0.85, -1.0, 2.46, 2.0, 0.6, region="cab-glazing", material="glass")]
        s += [rect("truck-body", "chassis", "body", 3.0, 0.45, -1.2, 0.42, 2.4, 2.88, regions=["cab", "chassis", "engine-bay"], material="paint-steel")]
    else:
        s += [rect("windscreen", "chassis", "glazing", 3.5, 0.85, -1.05, 2.5 if d >= 200 else 2.52, 2.1, 0.6, region="cab-glazing", material="glass")]
        s += [rect("front-grille", "chassis", "detail", 3.2, 0.2, -0.7, 1.3, 1.4, 0.65, region="engine-bay", material="paint-steel")]
        if d == 50:
            s += [rect("head-lamp-left", "chassis", "detail", 3.3, 0.9, 0.7, 1.2, 0.4, 0.3, region="cab-glazing", material="glass"),
                  rect("head-lamp-right", "chassis", "detail", 3.3, 0.9, -1.1, 1.2, 0.4, 0.3, region="cab-glazing", material="glass")]
        if d in (50, 100):
            s += [poly("mirror-arm-left", "chassis", "detail", 3.3, 0.2, [(1.2, 2.7), (1.35, 2.7), (1.35, 2.95), (1.2, 2.95)], region="cab", material="paint-steel"),
                  poly("mirror-arm-right", "chassis", "detail", 3.3, 0.2, [(-1.35, 2.7), (-1.2, 2.7), (-1.2, 2.95), (-1.35, 2.95)], region="cab", material="paint-steel")]
        s += [rect("cab", "chassis", "body", 3.0, 0.5, -1.2, 0.9, 2.4, 2.4, rr=0.12 if d <= 100 else None, region="cab", material="paint-steel"),
              rect("bumper-frame", "chassis", "body", 2.9, 0.4, -1.15, 0.42, 2.3, 0.48, region="chassis", material="paint-steel")]
    # tyres of the front axle (the others sit behind them)
    t_regions = ["tyres", "brakes"] if d == 500 else ["tyres"]
    for n, lo in (("left", 0.955), ("right", -1.275)):
        s += [rect("tyre-1-%s" % n, "wheel-1", "body", 4.0, 0.12 if d != 500 else 0.15, lo, 0, 0.32, 1.04, regions=t_regions, material="rubber", hatch="tread" if d == 50 else None)]
    return s


def plan(d):
    s = []
    # chute pair (capsules stretching along their bones)
    s += chute(4.0, 3.9)
    # hopper and drum
    s += [poly("hopper", "drum", "body", 3.8, 0.55, [(-3.14, -0.55), (-3.14, 0.55), (-3.9, 0.35), (-4.02, 0.0), (-3.9, -0.35)], region="hopper", material="galvanised-steel")]
    if d in (50, 100):
        s += [poly("drum-seam", "drum", "detail", 3.7, 0.3, [(px(3), -1.12), (px(3), 1.12)], closed=False, region="drum", material="drum-paint")]
    if d <= 200:
        s += [poly("flight-%d" % (k + 1), "drum-spiral", "detail", 3.4, 0.75, flight(k, 0.7 + 0.85 * k), region="drum", material="drum-paint", filled=False) for k in range(5)]
    if d == 500:
        s += [drum_solid("drum-body", PROFILE_COARSE, 3.0, regions=["drum", "drum-drive"], material="drum-paint")]
    else:
        s += [drum_solid("drum-belly", PROFILE_FULL if d <= 100 else PROFILE_COARSE, 3.0, region="drum", material="drum-paint")]
    # stack, cab, windscreen
    sr = 0.1
    s += [ellipse("exhaust-stack", "chassis", "body", 2.4, 0.2, 2.32, -1.1, sr, sr, region="exhaust-stack", material="exhaust-steel")]
    if d in (50, 100):
        s += [ellipse("stack-cap", "chassis", "detail", 2.45, 0.25, 2.32, -1.1, 0.12, 0.12, region="exhaust-stack", material="exhaust-steel", filled=False)]
    if d == 500:
        s += [rect("cab-glass", "chassis", "glazing", 2.6, 0.85, 4.14, -1.05, 0.2, 2.1, region="cab-glazing", material="glass")]
        s += [poly("truck-body", "chassis", "body", 2.0, 0.45, [(-4.5, -0.45), (2.4, -0.45), (2.4, -1.2), (4.5, -1.2), (4.5, 1.2), (2.4, 1.2), (2.4, 0.45), (-4.5, 0.45)], regions=["cab", "chassis", "engine-bay"], material="paint-steel")]
    else:
        s += [rect("windscreen", "chassis", "glazing", 2.6, 0.85, 4.14, -1.05, 0.16, 2.1, region="cab-glazing", material="glass")]
        if d in (50, 100):
            s += [poly("mirror-arm-left", "chassis", "detail", 2.5, 0.2, [(4.3, 1.2), (4.5, 1.2), (4.5, 1.35), (4.3, 1.35)], region="cab", material="paint-steel"),
                  poly("mirror-arm-right", "chassis", "detail", 2.5, 0.2, [(4.3, -1.35), (4.5, -1.35), (4.5, -1.2), (4.3, -1.2)], region="cab", material="paint-steel")]
        s += [rect("cab", "chassis", "body", 2.0, 0.5, 2.4, -1.2, 2.1, 2.4, rr=0.12 if d <= 100 else None, region="cab", material="paint-steel"),
              rect("frame", "chassis", "body", 1.4, 0.4, -4.5, -0.45, 8.8, 0.9, region="chassis", material="paint-steel")]
    # wheels
    for n, x in WHEELS:
        for side, lo in (("left", 0.955), ("right", -1.275)):
            s += [rect("tyre-%s-%s" % (n, side), "wheel-%s" % n, "body", 1.5, 0.12 if d != 500 else 0.15, x - 0.52, lo, 1.04, 0.32, regions=["tyres", "brakes"] if d == 500 else ["tyres"], material="rubber", hatch="tread" if d == 50 else None)]
    return s


def clean(sh):
    for x in sh:
        for k in ("r", "regions", "region", "hatch"):
            if k in x and x[k] is None:
                del x[k]
    return sh


D = {
    "50": ("Front view from the cab: the front tyres with tread hatch, bumper frame, cab with rounded roof, grille, windscreen, headlamps and mirror arms, the stack with its cap peeking over the roof and above it the drum as a solid of revolution about its own tilted axis with its seam ring; behind the drum the chute and its extension, extruded solids of their side-view profiles that rise above the drum when the chute is raised. Plan: frame and the four tyre pairs, cab with windscreen and mirror arms, stack with cap ring, the drum outline (a solid of revolution seen from above) with its seam, five spiral flights that slide along the drum axis, the hopper, and the chute and its extension as extruded solids, so a swung chute reaches out behind the truck.",
           "Vorderansicht vom Fahrerhaus aus: die Vorderreifen mit Profilschraffur, Stoßfängerrahmen, Fahrerhaus mit gerundetem Dach, Kühlergrill, Frontscheibe, Scheinwerfer und Spiegelarme, das Auspuffrohr mit Kappe über dem Dach und darüber die Trommel als Rotationskörper um ihre eigene geneigte Achse mit ihrem Nahtring; hinter der Trommel die Rinne und ihre Verlängerung, extrudierte Körper ihrer Seitenansichtsprofile, die über die Trommel ragen, wenn die Rinne aufgerichtet ist. Grundriss: Rahmen und die vier Reifenpaare, Fahrerhaus mit Frontscheibe und Spiegelarmen, Auspuffrohr mit Kappenring, der Trommelumriss mit seiner Naht, fünf Wendel, die entlang der Trommelachse gleiten, der Fülltrichter sowie die Rinne und ihre Verlängerung als extrudierte Körper, sodass eine ausgeschwenkte Rinne hinter dem Fahrzeug ausgreift."),
    "100": ("Front view: front tyres, bumper frame, cab with grille, windscreen and mirror arms, stack with cap, the revolved drum with seam ring and the chute pair behind it. Plan: frame, eight tyres, cab with windscreen and mirror arms, stack, drum outline with seam, five flights, hopper and the stretching chute pair.",
            "Vorderansicht: Vorderreifen, Stoßfängerrahmen, Fahrerhaus mit Grill, Frontscheibe und Spiegelarmen, Auspuffrohr mit Kappe, die Rotationstrommel mit Nahtring und dahinter das Rinnenpaar. Grundriss: Rahmen, acht Reifen, Fahrerhaus mit Frontscheibe und Spiegelarmen, Auspuffrohr, Trommelumriss mit Naht, fünf Wendel, Fülltrichter und das sich streckende Rinnenpaar."),
    "200": ("Front view: the front tyres, bumper band, cab with grille panel and windscreen, the stack over the roof, the revolved drum and the chute pair behind it. Plan: frame band, eight tyres, cab with glass strip, stack, drum outline with five flight marks, hopper and the stretching chute pair.",
            "Vorderansicht: die Vorderreifen, Stoßfängerband, Fahrerhaus mit Grilltafel und Frontscheibe, das Auspuffrohr über dem Dach, die Rotationstrommel und dahinter das Rinnenpaar. Grundriss: Rahmenband, acht Reifen, Fahrerhaus mit Glasstreifen, Auspuffrohr, Trommelumriss mit fünf Wendelmarken, Fülltrichter und das sich streckende Rinnenpaar."),
    "500": ("Front view: one body block with a glass band, the front tyres as black bars, the revolved drum above the cab, the stack and the chute bars behind the drum. Plan: one composed truck outline (frame and cab), eight tyre bars, the drum outline as one form, a hopper wedge and the chute pair as stretching bars, the stack as a dot.",
            "Vorderansicht: ein Aufbaublock mit Glasband, die Vorderreifen als schwarze Balken, die Rotationstrommel über dem Fahrerhaus, das Auspuffrohr und die Rinnenbalken hinter der Trommel. Grundriss: ein zusammengesetzter Fahrzeugumriss (Rahmen und Fahrerhaus), acht Reifenbalken, der Trommelumriss als eine Form, ein Fülltrichterkeil und das Rinnenpaar als sich streckende Balken, das Auspuffrohr als Punkt."),
}
OM = {
    "50": ("The other axles, the brake drums, the fuel and air tanks, the pedestal and the drive housing are hidden behind the tyres, cab or drum; the chute is hidden behind the drum until it is raised above it.",
           "Die übrigen Achsen, die Bremstrommeln, Kraftstoff- und Lufttank, der Bock und das Antriebsgehäuse sind hinter Reifen, Fahrerhaus oder Trommel verdeckt; die Rinne bleibt hinter der Trommel verdeckt, bis sie darüber hinausragt."),
    "100": ("Hidden behind tyres, cab or drum: other axles, tanks, pedestal and drive housing; lamps are not drawn.", "Hinter Reifen, Fahrerhaus oder Trommel verdeckt: übrige Achsen, Tanks, Bock und Antriebsgehäuse; Scheinwerfer sind nicht gezeichnet."),
    "200": ("Mirror arms, lamps, steps and tanks in both views.", "Spiegelarme, Scheinwerfer, Tritte und Tanks in beiden Ansichten."),
    "500": ("Flights, seams, door, mirrors, lamps and rims; in the front view everything behind the cab and drum except the stack and the raised chute.", "Wendel, Nähte, Tür, Spiegel, Scheinwerfer und Felgen; in der Vorderansicht alles hinter Fahrerhaus und Trommel außer Auspuffrohr und aufgerichteter Rinne."),
}
plan_ = {str(d): {"front": clean(front(d)), "plan": clean(plan(d)), "dec": {"retained": D[str(d)], "omitted": OM[str(d)]}} for d in (50, 100, 200, 500)}
apply("concrete-mixer-truck", plan_)

SWING = 30.0


def swivel():
    doc, indent, _ = load("concrete-mixer-truck")
    a = doc["asset"]
    bones = a["rig"]["bones"]
    if any(b["id"] == "chute-swivel" for b in bones):
        raise SystemExit("already swivelled")
    i = next(k for k, b in enumerate(bones) if b["id"] == "chute")
    chute = bones[i]
    pivot = {"id": "chute-swivel", "parent": chute["parent"], "title": {"en": "Chute swivel pedestal", "de": "Rutschen-Drehbock"}, "x": chute["x"], "y": chute["y"], "angleDeg": 90, "lengthM": 0.3, "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0}}
    chute["parent"], chute["x"], chute["y"], chute["angleDeg"] = "chute-swivel", 0, 0, chute["angleDeg"] - 90
    bones.insert(i, pivot)
    for p in a["poses"]:
        c = (p.get("bones") or {}).get("chute")
        if c is not None:
            p["bones"]["chute-swivel"] = {"twistDeg": round(SWING * c["angleDeg"] / 165, 3)}
    for clip in a["clips"]:
        for k in clip.get("keys", []):
            c = (k.get("bones") or {}).get("chute")
            if c is not None:
                k["bones"]["chute-swivel"] = {"twistDeg": round(SWING * max(0, min(165, c["angleDeg"])) / 165, 3)}
    save("concrete-mixer-truck", doc, indent)
    print("swivelled")


swivel()
