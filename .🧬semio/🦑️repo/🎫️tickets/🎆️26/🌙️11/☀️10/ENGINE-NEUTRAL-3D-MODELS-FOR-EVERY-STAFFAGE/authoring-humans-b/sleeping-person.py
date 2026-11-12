from lib import Builder, R
b = Builder("sleeping-person")
O = b.organic
O("pillow", "bed-surface", "pillow", front=["pillow"], plan=["pillow"], n=9, exp=3.5, ends=0.85)
for sid, bone, ids in (("duvet-chest", "torso", ["duvet-chest"]), ("duvet-hips", "pelvis", ["duvet-hips"]), ("duvet-thighs", "thigh-r", ["duvet-thighs"]), ("duvet-shins", "shank-r", ["duvet-shins"])):
    O(sid, bone, sid, front=ids, plan=ids, n=7, exp=2.5, ends=0.8)
for sid in ("duvet-skirt-top", "duvet-skirt-legs"):
    b.extrude(sid, "bed-surface", sid, (-0.2, 0.2), tol=0.002)
O("head", "head", "head", front=["head"], plan=["head"], n=8, exp=2.2)
O("hair", "head", "hair", front=["hair"], plan=["hair"], n=6, exp=2.2)
O("neck", "neck", "neck", front=["neck"], plan=["neck"], n=4, exp=2.1)
for s, sg in (("l", 1), ("r", -1)):
    b.blob(f"eye-{s}", "head", "eye", lat=(0.02, 0.045) if sg > 0 else (-0.045, -0.02), maxden_ids=["eye"])
    O(f"sleeve-{s}", f"arm-{s}", "sleeve-r", front=[f"sleeve-{s}"], plan=[f"sleeve-{s}"], n=6, exp=2.2, mt=b.meta("sleeve-r"), lat=(0.1, 0.2) if sg > 0 else (-0.2, -0.1))
    O(f"forearm-{s}", f"forearm-{s}", "forearm-r", n=6, exp=2.2, lat=(0.11, 0.19) if sg > 0 else (-0.19, -0.11), mt=b.meta("forearm-r"))
    O(f"hand-{s}", f"hand-{s}", "hand-r", n=5, exp=2.1, lat=(0.12, 0.18) if sg > 0 else (-0.18, -0.12), mt=b.meta("hand-r"))
    O(f"foot-rear-{s}", f"foot-{s}", f"foot-rear-{'l' if sg > 0 else 'r'}", front=[f"foot-{s}"], plan=[f"foot-{s}"], n=6, exp=2.4)
    O(f"foot-toe-{s}", f"toe-{s}", f"foot-toe-{s}", front=[f"foot-{s}"], plan=[f"foot-{s}"], n=4, exp=2.4)
    O(f"shin-bare-{s}", f"shank-{s}", "shin-bare", lat=(0.04, 0.14) if sg > 0 else (-0.14, -0.04), n=6, exp=2.3, mt=b.meta("shin-bare"))
b.blob("thumb-r", "hand-r", "thumb-r", lat=(-0.145, -0.125))
b.blob("thumb-l", "hand-l", "thumb-r", lat=(0.125, 0.145), mt=b.meta("thumb-r"))
MT = b.meta("sleeve-r", role="body", tone=0.4)
b.loft("body-torso", "torso", [{"at": [0.0, 0.0, 0.0], "rx": 0.15, "ry": 0.09, "exponent": 2.4}, {"at": [0.22, 0.0, 0.0], "rx": 0.17, "ry": 0.1, "exponent": 2.4}, {"at": [0.44, 0.0, 0.0], "rx": 0.16, "ry": 0.085, "exponent": 2.3}], mt=MT, maxden_ids=False)
b.ell("body-pelvis", "pelvis", (1.03, 0.575, 0), (0.085, 0.09, 0.16), mt=MT, maxden_ids=False)
for s, sg in (("l", 1), ("r", -1)):
    b.capsule(f"body-thigh-{s}", f"thigh-{s}", (1.05, 0.575, sg * 0.09), (1.465, 0.564, sg * 0.09), 0.08, 0.06, mt=MT, maxden_ids=False)
    b.capsule(f"body-shank-{s}", f"shank-{s}", (1.465, 0.564, sg * 0.09), (1.87, 0.536, sg * 0.09), 0.055, 0.04, mt=MT, maxden_ids=False)
b.dec("The sleeper lies on the back along the bed; the duvet is four lofts (chest, hips, thighs, shins) bound to the bones under them, so it follows every pose of the sleeper, plus two thick blocks (extrusions of the side outline, 0.40 m wide) that the plan draws as the bulk below the mounds. The pillow is a rounded loft 0.60 m long and 0.40 m wide. Because the 2D drawings show only the visible parts, the body under the duvet is modelled as a plain torso loft, a pelvis ellipsoid and thigh and shank capsules (invented from the rig lengths; they are hidden at rest and tied to the duvet bones).",
      "Die schlafende Person liegt auf dem Rücken entlang des Bettes; das Federbett besteht aus vier Lofts (Brust, Hüfte, Oberschenkel, Unterschenkel), gebunden an die Knochen darunter, sodass es jeder Haltung der Person folgt, dazu zwei dicke Blöcke (Extrusionen der Seitenkontur, 0,40 m breit), die der Grundriss als Masse unter den Hügeln zeichnet. Das Kissen ist ein gerundeter Loft, 0,60 m lang und 0,40 m breit. Da die 2D-Zeichnungen nur die sichtbaren Teile zeigen, ist der Körper unter dem Federbett als schlichter Rumpf-Loft, Becken-Ellipsoid sowie Oberschenkel- und Unterschenkelkapseln modelliert (aus den Rig-Längen erfunden; im Ruhezustand verdeckt und an die Federbettknochen gebunden).")
b.dec("Head, neck, hair, the visible arms (the left arm is the mirror of the drawn right arm: the front and plan views draw it, the side view hides it), the bare shins and the feet pointing up are lofts measured on the side outline with the width from the plan or front; the eyes are small ellipsoids that exist at 1:50 only, the ear (drawn 0.11 m long in the side view, an outlier) and the duvet fold lines are not modelled.",
      "Kopf, Hals, Haar, die sichtbaren Arme (der linke Arm ist die Spiegelung des gezeichneten rechten: Vorderansicht und Grundriss zeichnen ihn, die Seitenansicht verdeckt ihn), die nackten Schienbeine und die nach oben zeigenden Füsse sind Lofts, die an der Seitenkontur gemessen sind, die Breite aus Grundriss oder Vorderansicht; die Augen sind kleine Ellipsoide, die es nur im Massstab 1:50 gibt; das Ohr (in der Seitenansicht 0,11 m lang gezeichnet, ein Ausreisser) und die Falten des Federbetts werden nicht modelliert.")
b.save()
