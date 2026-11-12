from body import *
from decisions import *
A = Asset("person-with-umbrella")
names = list(A.draft)
FOLD = {k for k in names if any(t in k for t in ("fold", "crease", "nostril", "jaw-line", "lower-lip", "sole", "elbow", "shoulder-1", "shoulder-2", "rib-line", "rim-tip"))}
UMB = {k for k, s in A.draft.items() if s["bone"] in ("umbrella", "rib-front", "rib-back")}
SK = FOLD | UMB | {"eye", "brow", "ear", "mouth-line", "cuff-l", "cuff-r", "sleeve-cuff-r", "trouser-hem-l", "trouser-hem-r", "coat-hem-l", "coat-hem-r", "coat-belt", "coat-front-seam", "shirt-neck", "thumb-l", "thumb-r",
                  "fingers-l-0", "fingers-l-1", "fingers-l-2", "fingers-r-0", "fingers-r-1", "fingers-r-2", "torso"}
humanize(A, skip=SK, bones=set(b for b in A.bones if is_human_bone(b)))
tj = loft_from(A.draft["torso"], rxfn=front_halfwidth(A, "torso", "torso"), amax=2.6, exp=2.5)
A.add(tj, like="torso")
hd = get(A, "head")
for sd in "lr":
    thumb(A, f"thumb-{sd}", f"hand-{sd}", glove=f"hand-{sd}")
    refinger(A, [f"fingers-{sd}-{i}" for i in range(3)], f"hand-{sd}")
    band(A, f"cuff-{sd}", f"forearm-{sd}", 0.232, 0.246, 0.003, "rain-shell", 0.3, "arm-clothing", like="cuff")
    band(A, f"trouser-hem-{sd}", f"shank-{sd}", 0.37, 0.382, 0.003, "cotton-clothing", 0.2, "leg-clothing", like="trouser-hem")
    band(A, f"coat-hem-{sd}", f"thigh-{sd}", 0.307, 0.319, 0.003, "rain-shell", 0.3, "coat", like="coat-hem")
band(A, "coat-belt", "torso", 0.11, 0.124, 0.004, "rain-shell", 0.3, "coat", like="coat-belt")
A.add(capsule("coat-front-seam", "torso", [-0.15, surface_v(tj, -0.15, 0.0) + 0.002, 0.0], [0.44, surface_v(tj, 0.44, 0.0) + 0.002, 0.0], 0.004, material="rain-shell", role="detail", tone=0.2, region="coat"), like="coat-front-seam", maxden=50)
A.add(capsule("neck", "torso", [0.43, 0.0, 0.0], [0.5, 0.0, 0.0], 0.055, material="skin", role="body", tone=0.84, region="head-skin"), like="neck")
eyes(A, u=0.174, w=0.035, rx=0.006, ry=0.007, rz=0.008, material="skin", tone=0.1, region="head-skin")
for sd, z in (("l", 1), ("r", -1)):
    vs = surface_v(hd, 0.19, z * 0.035) + 0.002
    A.add(capsule(f"brow-{sd}", "head", [0.19, vs, z * 0.012], [0.193, vs + 0.003, z * 0.055], 0.004, material="skin", role="detail", tone=0.2, region="head-skin"), like="brow", maxden=50)
    A.add(ellipsoid(f"ear-{sd}", "head", [0.024, 0.012, 0.012], [0.154, 0.019, z * 0.092], material="skin", role="detail", tone=0.7, region="head-skin"), like="ear")
A.add(ellipsoid("nose", "head", [0.015, 0.009, 0.008], [0.127, surface_v(hd, 0.127, 0.0) + 0.003, 0.0], material="skin", role="detail", tone=0.8, region="head-skin"), like="nose", maxden=50)
vm = surface_v(hd, 0.11, 0.0) + 0.001
A.add(capsule("mouth", "head", [0.11, vm, -0.017], [0.11, vm, 0.017], 0.003, material="skin", role="detail", tone=0.3, region="head-skin"), like="mouth-line", maxden=50)
deltoids(A, ["upper-arm-l", "upper-arm-r"])
U = "umbrella"
UM = dict(material="umbrella-metal", role="body", region="umbrella-frame")
A.add(capsule("shaft", U, [-0.1, 0.0, 0.0], [0.84, 0.0, 0.0], 0.0105, tone=0.2, **UM), like="shaft")
tube_path(A, "handle", U, [(0.03, 0.0), (-0.09, 0.0), (-0.13, -0.012), (-0.145, -0.04), (-0.125, -0.06), (-0.1, -0.055)], 0.0165, "umbrella-handle", 0.18, "umbrella-frame", like="handle")
A.add(capsule("ferrule", U, [0.8, 0.0, 0.0], [0.884, 0.0, 0.0], 0.008, 0.004, tone=0.1, **{**UM, "role": "detail"}), like="ferrule", maxden=100)
A.add(capsule("hub", U, [0.756, 0.0, 0.0], [0.806, 0.0, 0.0], 0.0225, ls=0.55, tone=0.3, **{**UM, "role": "detail"}), like="hub", maxden=100)
th = math.radians(114)
ROTSGN = -1.0
for name, bone, sgn, sweep0 in (("canopy-front", "rib-front", 1, 0), ("canopy-back", "rib-back", -1, 0)):
    ring = A.draft[name]["profile"]
    ax = (math.cos(th), sgn * math.sin(th))
    n = (math.sin(th), sgn * -math.cos(th)) if False else (0.9135, 0.4067 * sgn)
    prof = []
    for (u, v) in ring:
        a = u * ax[0] + v * ax[1]
        r = max(0.0004, u * n[0] + v * n[1])
        if not prof or abs(a - prof[-1][0]) + abs(r - prof[-1][1]) > 5e-4:
            prof.append([a, r])
    imax = max(range(len(prof)), key=lambda i: prof[i][1])
    lo = imax
    while lo > 0 and prof[lo - 1][1] >= 0.03:
        lo -= 1
    hi = imax
    while hi < len(prof) - 1 and prof[hi + 1][1] >= 0.03:
        hi += 1
    prof = clean_polyline(prof[lo : hi + 1])
    for tol in (0.0005, 0.001, 0.002, 0.003):
        cand = dp(prof, tol)
        if simple(cand):
            prof = cand
            break
    print(name, len(prof), "simple" if simple(prof) else "NOT SIMPLE")
    s = revolve(name, bone, prof, axis=(ax[0], ax[1], 0.0), material="umbrella-fabric", role="body", tone=0.3, region="canopy")
    s["sweepRad"] = math.pi
    h = math.sqrt(0.5)
    s["transform"] = {"rotation": [ax[0] * h * ROTSGN, ax[1] * h * ROTSGN, 0.0, h]}
    A.add(s, like=name)
DEC = BODY + [
    d("The umbrella canopy is two half-revolves about the shaft axis, one on each hinged rib bone: the rib outline (a thin curved shell section, drawn as a filled sliver) is converted to an axial/radial profile about the shaft direction in the rib frame, so the opened dome has the drawn 0.52 m radius, hangs 0.23 m below the hub and folds with the rib bones when the umbrella is furled; shaft, hook handle (tube path), hub and ferrule are tubes on the shaft bone; the rib lines drawn as texture are not modelled. The 2D side view draws the canopy only as a thin sliver along each rib, whereas a dome's side silhouette is the filled cone (measured elevation IoU about 0.77 at rest): the physically right dome is kept.",
      "Das Schirmdach besteht aus zwei Halb-Rotationskörpern um die Schaftachse, je einer an einem gelenkigen Speichenknochen: der Speichenumriss (ein dünner gekrümmter Schalenschnitt, als gefüllter Streifen gezeichnet) wird im Speichenrahmen in ein Axial-/Radialprofil um die Schaftrichtung umgerechnet, sodass die geöffnete Kuppel den gezeichneten Radius von 0,52 m hat, 0,23 m unter der Nabe hängt und mit den Speichenknochen zusammenklappt, wenn der Schirm geschlossen wird; Schaft, Hakengriff (Rohrzug), Nabe und Zwinge sind Röhren am Schaftknochen; die als Textur gezeichneten Speichenlinien sind nicht modelliert. Die 2D-Seitenansicht zeichnet das Dach nur als dünnen Streifen entlang jeder Speiche, die Seitensilhouette einer Kuppel ist jedoch der gefüllte Kegel (gemessene Seiten-IoU etwa 0,77 in Ruhe): die physikalisch richtige Kuppel bleibt erhalten."),
    d("The coat is wider than the plan trunk: its half width follows the front outline (0.254 m) and the plan shoulder ellipse (0.278 m); hems and the belt are thin rings, the front seam a tube on the surface; the ears of the draft sat on the head axis and are replaced by the two lateral ears (0.092 m).",
      "Der Mantel ist breiter als der Rumpfkreis der Draufsicht: seine Halbbreite folgt dem Vorderansichtsumriss (0,254 m) und der Schulterellipse der Draufsicht (0,278 m); Säume und Gürtel sind dünne Ringe, die Frontnaht eine Röhre auf der Oberfläche; die Ohren des Entwurfs sassen auf der Kopfachse und sind durch die zwei seitlichen Ohren (0,092 m) ersetzt."),
]
A.write(DEC)
