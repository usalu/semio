import sys
from body import *

H, M, WF = 1.72, 70.0, 1.0
h = Human(H, M, WF)
s = h.s
A_BODY = dubois_m2(H, M)
fr = Frame("fwd", 1.0, 1.0)
up, down, fwd = h.up, h.down, h.fwd
SHAFT_TOP, HANDLE_END, RIB = 0.78, -0.12, 0.56
RHO, CD, AREA_REF = 1.25, 0.5, 0.55
def wind_force(v, area=AREA_REF, cd=CD): return 0.5 * RHO * cd * area * v * v

rv = lambda lo, hi: {"type": "revolute", "minDeg": lo, "maxDeg": hi}
def lit(kg, comX, comY=0.0, src="authored-estimate", ref=None):
    m = {"kg": kg, "comX": comX, "comY": comY, "distribution": "point", "source": src}
    if ref: m["reference"] = ref
    return m
umb_bones = [
    h.bone("umbrella", "hand-r", "Umbrella shaft with handle", "Schirmstock mit Griff", 0.075 * s, 0, 0, SHAFT_TOP, rv(-170, 170), lit(0.22, 0.3)),
    h.bone("rib-front", "umbrella", "Front ribs and canopy half", "Vordere Streben und Schirmhälfte", SHAFT_TOP, 0, -114, RIB, rv(-70, 20), lit(0.12, 0.3)),
    h.bone("rib-back", "umbrella", "Rear ribs and canopy half", "Hintere Streben und Schirmhälfte", SHAFT_TOP, 0, 114, RIB, rv(-20, 70), lit(0.12, 0.3)),
]
bones = h.bones() + umb_bones
anchors = h.anchors() + [
    {"id": "canopy-centre", "bone": "umbrella", "x": SHAFT_TOP, "y": 0, "kind": "load", "title": T("Canopy centre of pressure", "Druckmittelpunkt des Schirms")},
    {"id": "lumbar", "bone": "torso", "x": 0, "y": 0, "kind": "load", "title": T("Lumbar joint L5/S1", "Lendengelenk L5/S1")},
]
rig = Rig(bones, anchors)
FEET = ["heel-l", "contact-l", "heel-r", "contact-r"]

# ---------------------------------------------------------------- geometry
elev = h.standard(arm_sleeve="long", legs="long", shoe="boot", tones={"shirt": 0.62, "trousers": 0.25, "shoes": 0.12, "skin": 0.84}, hair="short", hair_tone=0.30)
byid = {x["id"]: x for x in elev}
for x in elev:
    if x["id"].startswith(("upper-arm", "forearm")) and not x["id"].startswith("upper-arm-skin"):
        x["region"], x["material"], x["tone"] = "coat", "rain-shell", 0.62
elev = [x for x in elev if x["id"] != "torso"]
elev += [
    h.sh("torso", "torso", "body", 0, 3.2, 0.62, g_path(up, table_cmds(TORSO, -0.16, 0.46, grow=0.012), True), "coat", "rain-shell"),
    h.sh("coat-skirt-r", "thigh-r", "body", 0, 2.4, 0.62, g_path(down, table_cmds(THIGH, -0.14, 0.32, grow=0.028), True), "coat", "rain-shell"),
    h.sh("coat-skirt-l", "thigh-l", "body", 0, 1.4, 0.58, g_path(down, table_cmds(THIGH, -0.14, 0.32, grow=0.028), True), "coat", "rain-shell"),
    h.sh("coat-hem-r", "thigh-r", "detail", 2, 2.45, 0.3, g_poly(down, [(0.125, 0.32), (-0.135, 0.32)], False), "coat", "rain-shell", False),
    h.sh("coat-hem-l", "thigh-l", "detail", 2, 1.45, 0.3, g_poly(down, [(0.125, 0.32), (-0.135, 0.32)], False), "coat", "rain-shell", False),
    h.sh("coat-belt", "torso", "detail", 2, 3.3, 0.3, g_poly(up, [(0.13, 0.12), (-0.125, 0.12)], False), "coat", "rain-shell", False),
    h.sh("coat-front-seam", "torso", "detail", 2, 3.3, 0.3, g_poly(up, [(0.126, -0.16), (0.128, 0.30), (0.075, 0.46)], False), "coat", "rain-shell", False),
    h.sh("coat-hood", "head", "body", 1, 4.5, 0.55, g_path(up, [("M", -0.02, 0.06), ("C", -0.07, 0.04, -0.12, 0.07, -0.13, 0.13), ("C", -0.12, 0.18, -0.09, 0.19, -0.06, 0.15), ("C", -0.03, 0.13, 0.02, 0.10, 0.04, 0.06), ("Z",)], True), "coat", "rain-shell"),
    h.sh("coat-collar", "head", "body", 1, 4.6, 0.58, g_path(up, [("M", 0.06, -0.01), ("C", 0.07, 0.05, 0.05, 0.075, 0.02, 0.078), ("L", -0.06, 0.07), ("C", -0.085, 0.05, -0.08, 0.0, -0.075, -0.01), ("Z",)], True), "coat", "rain-shell"),
    h.sh("rain-spots-coat", "torso", "texture", 3, 3.3, 0.9, g_poly(up, [(0.04, 0.35), (0.04, 0.34), (-0.05, 0.28), (-0.05, 0.27), (0.07, 0.20), (0.07, 0.19), (-0.07, 0.12)], False), "coat", "rain-shell", False),
]
for x in elev:
    if x["id"].startswith(("hand-")): x["region"] = "hand-skin"
elev += [
    h.sh("shaft", "umbrella", "body", 0, 6.1, 0.2, g_capsule(fr, HANDLE_END, 0, SHAFT_TOP + 0.08, 0, 0.0075, 0.0075), "umbrella-frame", "umbrella-metal"),
    h.sh("handle", "umbrella", "body", 0, 6.15, 0.18, g_path(fr, [("M", 0.03, 0.017), ("L", HANDLE_END + 0.03, 0.017), ("C", HANDLE_END - 0.04, 0.017, HANDLE_END - 0.06, -0.06, HANDLE_END, -0.07), ("C", HANDLE_END + 0.04, -0.075, HANDLE_END + 0.04, -0.04, HANDLE_END + 0.02, -0.03), ("L", HANDLE_END + 0.02, -0.017), ("L", 0.03, -0.017), ("Z",)], True), "umbrella-frame", "umbrella-handle"),
    h.sh("ferrule", "umbrella", "detail", 1, 6.2, 0.1, g_capsule(fr, SHAFT_TOP + 0.04, 0, SHAFT_TOP + 0.10, 0, 0.012, 0.004), "umbrella-frame", "umbrella-metal"),
    h.sh("hub", "umbrella", "detail", 1, 6.16, 0.3, g_ellipse(fr, SHAFT_TOP, 0, 0.025, 0.025), "umbrella-frame", "umbrella-metal", False),
    h.sh("canopy-front", "rib-front", "body", 0, 6.0, 0.30, g_path(fr, [("M", 0, 0), ("C", 0.16, 0.115, 0.42, 0.135, RIB, 0.01), ("C", 0.42, 0.04, 0.18, 0.03, 0, 0), ("Z",)], True), "canopy", "umbrella-fabric"),
    h.sh("canopy-back", "rib-back", "body", 0, 6.0, 0.30, g_path(fr, [("M", 0, 0), ("C", 0.16, -0.115, 0.42, -0.135, RIB, -0.01), ("C", 0.42, -0.04, 0.18, -0.03, 0, 0), ("Z",)], True), "canopy", "umbrella-fabric"),
    h.sh("rib-line-front", "rib-front", "detail", 1, 6.05, 0.1, g_poly(fr, [(0.0, 0.0), (RIB, 0.0)], False), "umbrella-frame", "umbrella-metal", False),
    h.sh("rib-line-back", "rib-back", "detail", 1, 6.05, 0.1, g_poly(fr, [(0.0, 0.0), (RIB, 0.0)], False), "umbrella-frame", "umbrella-metal", False),
    h.sh("panel-seam-front", "rib-front", "texture", 2, 6.02, 0.2, g_poly(fr, [(0.1, 0.07), (0.30, 0.10), (0.50, 0.04)], False), "canopy", "umbrella-fabric", False),
    h.sh("panel-seam-back", "rib-back", "texture", 2, 6.02, 0.2, g_poly(fr, [(0.1, -0.07), (0.30, -0.10), (0.50, -0.04)], False), "canopy", "umbrella-fabric", False),
    h.sh("rim-tip-front", "rib-front", "detail", 3, 6.06, 0.1, g_ellipse(fr, RIB, 0.004, 0.011, 0.011), "umbrella-frame", "umbrella-metal"),
    h.sh("rim-tip-back", "rib-back", "detail", 3, 6.06, 0.1, g_ellipse(fr, RIB, -0.004, 0.011, 0.011), "umbrella-frame", "umbrella-metal"),
    h.sh("furl-strap", "rib-front", "detail", 3, 6.07, 0.5, g_poly(fr, [(0.30, -0.03), (0.30, 0.10)], False), "canopy", "umbrella-fabric", False),
]

# ---------------------------------------------------------------- poses by inverse kinematics and moment balance
def pose_off(theta_deg, torso_extra, grip_rel, umbrella_world, ribs, hold_hand_deg=60.0):
    """Offsets for a standing person whose whole body is rotated by theta about the hips (feet kept flat), holding the grip at grip_rel from the shoulder."""
    base = merge_offsets({f"foot-{sd}": {"angleDeg": -theta_deg} for sd in "lr"}, {"torso": {"angleDeg": torso_extra}, "head": {"angleDeg": -torso_extra}})
    y = root_y(rig, base, FEET, root_angle=theta_deg)
    w = rig.solve(base, (0, y, theta_deg))
    shd = rig.origin(w, "upper-arm-r")
    a = math.radians(rig.angle(w, "torso") - 90)
    grip = (shd[0] + grip_rel[0] * math.cos(a) - grip_rel[1] * math.sin(a), shd[1] + grip_rel[0] * math.sin(a) + grip_rel[1] * math.cos(a))
    phi = hold_hand_deg
    arms = {}
    for _ in range(3):
        wr = (grip[0] - 0.075 * s * math.cos(math.radians(phi)), grip[1] - 0.075 * s * math.sin(math.radians(phi)))
        for sd in "lr": arms.update(arm_offsets(rig, w, sd, wr, phi, bend=-1))
    out = merge_offsets(base, arms)
    w2 = rig.solve(out, (0, y, theta_deg))
    hw = rig.angle(w2, "hand-r")
    out["umbrella"] = {"angleDeg": r(norm(umbrella_world - hw), 2)}
    out["rib-front"] = {"angleDeg": ribs[0]}
    out["rib-back"] = {"angleDeg": ribs[1]}
    return out, y

def moment_residual(theta, torso_extra, v, tilt_deg):
    off, y = pose_off(theta, torso_extra, GRIP_REL, 90 - tilt_deg, (0, 0))
    w = rig.solve(off, (0, y, theta))
    cx, cy, m = com_x(rig, off, (0, y, theta))
    cop = 0.5 * (rig.anchor(w, "heel-r")[0] + rig.anchor(w, "contact-r")[0])
    p = rig.anchor(w, "canopy-centre")
    F = wind_force(v)
    return m * 9.80665 * (cx - cop) - F * p[1] + 0.0

GRIP_REL = (0.30, -0.22)
def solve_theta(v, tilt_deg, torso_extra):
    a, b = -25.0, 8.0
    fa, fb = moment_residual(a, torso_extra, v, tilt_deg), moment_residual(b, torso_extra, v, tilt_deg)
    for _ in range(50):
        m = (a + b) / 2
        fm = moment_residual(m, torso_extra, v, tilt_deg)
        if fa * fm <= 0: b, fb = m, fm
        else: a, fa = m, fm
    return round((a + b) / 2, 2)

def wind_load(v, tilt_deg, fid="wind"):
    F = wind_force(v)
    return {"id": fid, "kind": "wind", "title": T("Wind on the canopy", "Wind auf dem Schirm"), "anchor": "canopy-centre", "fxN": r(-F, 2), "fyN": r(0.35 * F, 2),
            "source": "literature", "reference": "en-1991-1-4"}

# state parameters: wind speed (m/s), umbrella tilt into the wind (deg), extra trunk lean
STATES = {"still": (3.0, 0.0, 0.0), "light": (8.0, 18.0, -3.0), "tilted": (12.0, 32.0, -5.0), "heavy": (16.0, 40.0, -7.0)}
POSES = {}
for name, (v, tilt, tl) in STATES.items():
    th = solve_theta(v, tilt, tl) if v > 3.5 else 0.0
    off, y = pose_off(th, tl, GRIP_REL, 90 - tilt, (0, 0))
    POSES[name] = (off, th, v, tilt)

# furled umbrella carried like a cane at the right hip
fo, fy = pose_off(0.0, 0.0, (0.06, -0.58), -80.0, (-66, 66), hold_hand_deg=-70.0)
POSES["furled"] = (fo, 0.0, 0.0, 0.0)

def pz(id, en, de, den, dde, key_, met, br, loadv=None, wet=0.25, key=False):
    off, th, v, tilt = POSES[key_]
    loads = [wind_load(loadv, tilt)] if loadv else []
    root = {"angleDeg": th} if th else None
    return pose(id, en, de, den, dde, off, FEET, met, {"breath": br}, loads=loads, root=root, regions={"coat": {"wettedness": wet}, "canopy": {"wettedness": wet}}, key=key)

poses = [
    pz("umbrella-up-still", "Umbrella up, standing", "Schirm aufgespannt, stehend", "Standing upright in light rain, umbrella held vertically above the head at chest height; a light drizzle breeze of 3 m/s loads the canopy.", "Aufrecht stehend im leichten Regen, der Schirm senkrecht über dem Kopf in Brusthöhe gehalten; ein leichter Nieselwind von 3 m/s belastet den Schirm.", "still", 1.3, 0.5, 3.0, wet=0.2),
    pz("umbrella-tilted-to-wind", "Leaning into the gust", "In die Böe gelehnt", "Whole body leaning 5 degrees into a 12 m/s wind with the umbrella tilted 32 degrees towards it; the lean puts the centre of mass ahead of the feet so that the wind moment is balanced.", "Der ganze Körper lehnt sich in einen Wind von 12 m/s, der Schirm ist 32 Grad zum Wind geneigt; die Vorlage schiebt den Schwerpunkt vor die Füße, sodass das Windmoment ausgeglichen wird.", "tilted", 1.9, 0.8, 12.0, wet=0.4),
    pz("umbrella-furled", "Umbrella furled", "Schirm geschlossen", "Rain has stopped: the umbrella is closed and carried tip-down like a cane in the right hand.", "Der Regen hat aufgehört: der Schirm ist geschlossen und wird wie ein Stock mit der Spitze nach unten in der rechten Hand getragen.", "furled", 1.2, 0.5, None, wet=0.1),
    pz("gust-light", "Gust building up", "Anschwellende Böe", "Light gust of 8 m/s: slight lean and the umbrella starts to tilt.", "Leichte Böe von 8 m/s: leichte Vorlage, der Schirm beginnt sich zu neigen.", "light", 1.6, 0.7, 8.0, wet=0.3, key=True),
    pz("gust-heavy", "Strong gust", "Starke Böe", "Strong gust of 16 m/s: steep lean and a strongly tilted umbrella.", "Starke Böe von 16 m/s: starke Vorlage und ein stark geneigter Schirm.", "heavy", 2.2, 0.9, 16.0, wet=0.45, key=True),
]
# the wind load enters the gust clip through the pose load blended and multiplied by the track

# ---------------------------------------------------------------- thermal
ENV = {"airTempC": 8, "meanRadiantTempC": 8, "relativeHumidity": 0.95, "windMS": 4.0, "windDirectionDeg": 180, "solarWM2": 0, "pressurePa": 101325}
mats = std_materials() + [
    material("rain-shell", "Coated polyester raincoat", "Beschichteter Polyester-Regenmantel", 0.90, "literature", "iso-9920-2007", 0.5),
    material("umbrella-fabric", "Nylon pongee canopy", "Nylon-Pongee-Bespannung", 0.90, "authored-estimate", None, 0.6, 1140, 0.25),
    material("umbrella-metal", "Steel and aluminium frame", "Stahl- und Aluminiumgestänge", 0.25, "literature", "en-1991-1-1", 0.5, 7850, 50),
    material("umbrella-handle", "Wooden handle", "Holzgriff", 0.90, "authored-estimate", None, 0.6, 700, 0.15),
]
def reg(*a, **k): return region(*a, **k)
regions = [
    reg("head-skin", "Face and neck skin", "Haut von Gesicht und Hals", "skin", "skin", 0.03 * A_BODY, 0.15, "core-resistance", 36.8, 0.05, 0.06, "literature", "dubois-1916", wet=0.06),
    reg("hair", "Hair", "Haar", "hair", "hair", 0.03 * A_BODY, 0.15, "core-resistance", 36.8, 0.10, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("coat", "Raincoat with sleeves and hood", "Regenmantel mit Ärmeln und Kapuze", "clothing", "rain-shell", 0.62 * A_BODY, 0.30, "core-resistance", 36.8, 0.21, 0.62, "literature", "rain-wetting"),
    reg("torso-clothing", "Shirt visible at the neck", "Am Hals sichtbares Hemd", "clothing", "cotton-clothing", 0.02 * A_BODY, 0.05, "core-resistance", 36.8, 0.15, 0.0, "literature", "iso-9920-2007"),
    reg("arm-clothing", "Cuffs", "Ärmelabschlüsse", "clothing", "cotton-clothing", 0.01 * A_BODY, 0.05, "core-resistance", 36.8, 0.15, 0.0, "literature", "iso-9920-2007"),
    reg("hand-skin", "Hands", "Hände", "skin", "skin", 0.05 * A_BODY, 0.07, "core-resistance", 36.8, 0.05, 0.03, "literature", "dubois-1916", wet=0.06),
    reg("leg-clothing", "Trousers below the coat", "Hose unter dem Mantel", "clothing", "cotton-clothing", 0.20 * A_BODY, 0.12, "core-resistance", 36.8, 0.16, 0.29, "literature", "iso-9920-2007"),
    reg("shoes", "Waterproof boots", "Wasserdichte Stiefel", "surface", "leather-shoe", 0.07 * A_BODY, 0.12, "passive", None, None, 0.0, "authored-estimate", "iso-9920-2007"),
    reg("canopy", "Umbrella canopy", "Schirmbespannung", "surface", "umbrella-fabric", 1.0, 0.55, "passive", None, None, None, "authored-estimate", "hoerner-1965"),
    reg("umbrella-frame", "Shaft, ribs and handle", "Stock, Streben und Griff", "surface", "umbrella-metal", 0.08, 0.02, "passive", None, None, None, "authored-estimate", "hoerner-1965"),
]
emitters = [emitter("breath", "mouth", 4.0, vel=1.6, diam=0.03, temp=34)]

# the lower arm sleeves use the coat region: area budget moved there; arm-clothing and torso-clothing stay as small visible parts
for x in elev:
    if x["id"] in ("shirt-sleeve",): pass
elev += [h.sh("shirt-neck", "head", "detail", 3, 4.7, 0.8, g_poly(up, [(0.05, 0.005), (-0.05, 0.005)], False), "torso-clothing", "cotton-clothing", False),
         h.sh("sleeve-cuff-r", "forearm-r", "detail", 2, 6.1, 0.8, g_poly(down, [(0.03, 0.24), (-0.027, 0.24)], False), "arm-clothing", "cotton-clothing", False)]

clips = [
    clip("rain-gust-sway", "Gust in the rain", "Böe im Regen", "A gust rises from 3 to 16 m/s and fades: the person leans into the wind, the umbrella tilts and the wind load on the canopy follows the gust track while the lean keeps the moment about the feet balanced.", "Eine Böe steigt von 3 auf 16 m/s und klingt ab: die Person lehnt sich in den Wind, der Schirm neigt sich und die Windlast am Schirm folgt der Böenspur, während die Vorlage das Moment um die Füße im Gleichgewicht hält.",
         8.0, False, "none",
         [key(0, "umbrella-up-still"), key(1.5, "umbrella-up-still"), key(2.7, "gust-light"), key(3.8, "umbrella-tilted-to-wind"), key(4.8, "gust-heavy"), key(6.2, "umbrella-tilted-to-wind"), key(8.0, "umbrella-up-still")],
         [span(a, 0, 8.0) for a in FEET],
         [phase("calm", 0, 1.5, "Light drizzle", "Leichter Nieselregen"), phase("gust", 1.5, 6.2, "Gust", "Böe"), phase("fade", 6.2, 8.0, "Gust fades", "Böe klingt ab")],
         [ev(1.5, "motion", "Gust arrives", "Böe trifft ein"), ev(4.8, "motion", "Gust peak, 16 m/s", "Böenspitze, 16 m/s"), ev(6.2, "motion", "Gust passes", "Böe zieht ab"), ev(2.0, "inhale", "Breath against the wind", "Atemzug gegen den Wind")],
         [track("load:wind", [(0, 1.0), (1.5, 1.0), (2.7, 1.1), (3.8, 1.0), (4.3, 1.25), (4.8, 1.0), (6.2, 0.8), (8.0, 1.0)]), track("activityMet", [(0, 1.3), (3.8, 1.9), (4.8, 2.2), (8.0, 1.3)]), track("emitter:breath", [(0, 0.4), (3.8, 0.8), (4.8, 1.0), (8.0, 0.5)])]),
    clip("close-umbrella", "Closing the umbrella", "Schirm schließen", "The rain stops: the person straightens up, lowers the umbrella and folds the ribs along the shaft until it hangs furled from the hand.", "Der Regen hört auf: die Person richtet sich auf, senkt den Schirm und klappt die Streben an den Stock, bis er geschlossen an der Hand hängt.",
         4.0, False, "none",
         [key(0, "umbrella-tilted-to-wind"), key(1.2, "umbrella-up-still"), key(2.4, "umbrella-up-still", bones={"rib-front": {"angleDeg": -30}, "rib-back": {"angleDeg": 30}}), key(4.0, "umbrella-furled")],
         [span(a, 0, 4.0) for a in FEET],
         [phase("straighten", 0, 1.2, "Straighten up", "Aufrichten"), phase("fold", 1.2, 2.4, "Fold the ribs", "Streben einklappen"), phase("lower", 2.4, 4.0, "Lower the umbrella", "Schirm senken")],
         [ev(1.2, "load-off", "Wind load vanishes", "Windlast entfällt", "canopy-centre"), ev(2.4, "motion", "Canopy closed", "Schirm geschlossen")],
         [track("load:wind", [(0, 1.0), (1.2, 0.3), (2.4, 0.0), (4.0, 0.0)]), track("activityMet", [(0, 1.9), (4.0, 1.2)]), track("emitter:breath", [(0, 0.7), (4.0, 0.4)])]),
]

def build():
    return assemble("person-with-umbrella", T("Person with Umbrella", "Person mit Regenschirm"),
        T("A person in a knee-length raincoat holds an open umbrella: shaft and ribs are bones, and the wind on the canopy is a wind load whose track rises in a gust while the body leans into it and the umbrella tilts.",
          "Eine Person im knielangen Regenmantel hält einen aufgespannten Schirm: Stock und Streben sind Knochen, und der Wind auf dem Schirm ist eine Windlast, deren Spur in einer Böe ansteigt, während sich der Körper in den Wind lehnt und der Schirm neigt."),
        ["human", "rain", "umbrella", "wind", "raincoat", "gust", "lean"], ["de-leva-1996", "dubois-1916", "iso-9920-2007", "iso-8996-2021", "ashrae-2021", "en-1991-1-4", "hoerner-1965", "rain-wetting", "en-1991-1-1"],
        T("The lean angle for each wind speed is solved so that the moment of weight and wind about the middle of the feet vanishes; arms by inverse kinematics; the umbrella bone counter-rotates the hand so the shaft keeps the authored tilt.", "Der Vorlagewinkel jeder Windgeschwindigkeit wird so gelöst, dass das Moment aus Gewicht und Wind um die Fußmitte verschwindet; Arme per Inverskinematik; der Schirmknochen gleicht die Handdrehung aus, sodass der Stock die geplante Neigung behält."),
        {"heightM": 2.1, "widthM": 1.05, "depthM": 1.0, "massKg": 70.46}, mats, bones, anchors, elev, poses, clips, regions, emitters,
        [T("Rainy reference case: 8 degrees C, 95 % humidity, 4 m/s wind from the front; coated raincoat over light clothing about 1.35 clo (ISO 9920), wet coat surface (wettedness 0.2 to 0.45) cools by evaporation.", "Regen-Referenzfall: 8 Grad C, 95 % Feuchte, 4 m/s Wind von vorn; beschichteter Regenmantel über leichter Kleidung etwa 1,35 clo (ISO 9920), nasse Mantelfläche (Benetzung 0,2 bis 0,45) kühlt durch Verdunstung."),
         T("Metabolic rate 1.3 met standing, 1.9 to 2.2 met while leaning against a gust (ISO 8996 class 1 to 2).", "Stoffwechselrate 1,3 met im Stand, 1,9 bis 2,2 met beim Lehnen gegen eine Böe (ISO 8996 Klasse 1 bis 2).")],
        [T("Wind load F = 0.5 rho cd A v^2 at the canopy centre with rho 1.25 kg/m3 (EN 1991-1-4), cd 0.5 for a convex dome facing the flow (Hoerner), A 0.55 m2 projected at 32 degrees tilt; 35 % of F is added as uplift. The pose load is the 12 m/s value, the track scales it.", "Windlast F = 0,5 rho cd A v^2 im Schirmmittelpunkt mit rho 1,25 kg/m3 (EN 1991-1-4), cd 0,5 für eine dem Strom zugewandte konvexe Kuppel (Hoerner), A 0,55 m2 projiziert bei 32 Grad Neigung; 35 % von F kommen als Auftrieb hinzu. Die Posenlast ist der Wert bei 12 m/s, die Spur skaliert ihn."),
         T("Equal-stiffness support model: the lean puts the centre of mass ahead of the cop so the heel contacts may lift; both feet carry the horizontal wind force by friction.", "Gleichsteifigkeits-Auflagermodell: die Vorlage bringt den Schwerpunkt vor den Druckpunkt, sodass die Fersenkontakte abheben können; beide Füße tragen die horizontale Windkraft durch Reibung.")],
        "umbrella-up-still", "rain-gust-sway", base_met=1.3, env=ENV)

if __name__ == "__main__":
    json.dump(build(), open("out/person-with-umbrella.json", "w"), ensure_ascii=False, indent=1)
    print("written", {k: (v[1], v[2], v[3]) for k, v in POSES.items()})
