import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "tealight"
doc = load(A)
ALU, PARA, WICK, FL = "aluminium-cup", "paraffin", "wick-cotton", "flame"
FLAME_Y, TONGUE_Y = 0.0185, 0.0265


def side(sx, pts):
    return " ".join(f"C {r(sx*a)} {r(FLAME_Y + b if False else b)} {r(sx*c)} {r(d)} {r(sx*e)} {r(f)}" for a, b, c, d, e, f in pts)


def teardrop(y0, a, h, tip, mid):
    ca, cb = 0.0072 / 0.0072, 0.0
    (p1x, p1y, p2x, p2y, p3x, p3y), (q1x, q1y, q2x, q2y, q3x, q3y) = tip, mid
    right = f"M 0 {r(y0)} C {r(a*p1x)} {r(y0+h*p1y)} {r(a*p2x)} {r(y0+h*p2y)} {r(a*p3x)} {r(y0+h*p3y)} C {r(a*q1x)} {r(y0+h*q1y)} {r(a*q2x)} {r(y0+h*q2y)} 0 {r(y0+h)}"
    left = f" C {r(-a*q2x)} {r(y0+h*q2y)} {r(-a*q1x)} {r(y0+h*q1y)} {r(-a*p3x)} {r(y0+h*p3y)} C {r(-a*p2x)} {r(y0+h*p2y)} {r(-a*p1x)} {r(y0+h*p1y)} 0 {r(y0)} Z"
    return right + left


def flame(y0, a, h):
    return teardrop(y0, a, h, (1.0, 0.19, 1.0, 0.64, 0.2778, 0.893), (0.1389, 0.95, 0.0556, 0.979, 0.0, 1.0))


def tongue(y0):
    return teardrop(y0, 0.0019, 0.014, (0.8421, 0.143, 1.0, 0.5, 0.3684, 0.857), (0.2105, 0.95, 0.1053, 0.986, 0.0, 1.0))


def build(scale):
    f, p = [], []
    if scale == 200:
        f += [
            path(flame(FLAME_Y, 0.0105, 0.022), id="flame", bone="flame-body", role="body", z=3.3, tone=0.9, region="flame", material=FL),
            poly([(-0.0165, 0), (0.0165, 0), (0.0215, 0.016), (-0.0215, 0.016)], id="cup", bone="cup", role="body", z=3.5, tone=0.5, regions=["cup", "wax", "wick"], material=ALU),
        ]
        p += [
            ellipse(0, 0, 0.0215, 0.0215, id="cup", bone="cup", role="body", z=3.5, tone=0.5, region="cup", material=ALU),
            ellipse(0, 0, 0.0185, 0.0185, id="wax-top", bone="wick", role="body", z=3.55, tone=0.85, regions=["wax", "wick"], material=PARA),
            ellipse(0, 0, 0.0105, 0.0105, id="flame", bone="flame-body", role="body", z=3.6, tone=0.9, region="flame", material=FL),
        ]
    elif scale == 100:
        f += [
            path(tongue(TONGUE_Y), id="tongue", bone="flame-tip", role="body", z=3.2, tone=0.97, region="flame", material=FL),
            path(flame(FLAME_Y, 0.0072, 0.014), id="flame", bone="flame-body", role="body", z=3.3, tone=0.9, region="flame", material=FL),
            poly([(-0.0155, 0), (0.0155, 0), (0.0195, 0.016), (-0.0195, 0.016)], id="cup", bone="cup", role="body", z=3.5, tone=0.5, regions=["cup", "wax", "wick"], material=ALU),
        ]
        p += [
            ellipse(0, 0, 0.0195, 0.0195, id="cup", bone="cup", role="body", z=3.5, tone=0.5, region="cup", material=ALU),
            ellipse(0, 0, 0.0168, 0.0168, id="wax-top", bone="wick", role="body", z=3.55, tone=0.85, regions=["wax", "wick"], material=PARA),
            ellipse(0, 0, 0.0072, 0.0072, id="flame", bone="flame-body", role="body", z=3.6, tone=0.9, region="flame", material=FL),
        ]
    else:
        f += [
            path(tongue(TONGUE_Y), id="tongue", bone="flame-tip", role="body", z=3.2, tone=0.97, region="flame", material=FL),
            path(flame(FLAME_Y, 0.0058, 0.014), id="flame", bone="flame-body", role="body", z=3.3, tone=0.9, region="flame", material=FL),
            poly([(0, 0.0145), (0, 0.0198)], closed=False, id="wick", bone="wick", role="detail", z=3.4, tone=0.1, filled=False, region="wick", material=WICK),
            poly([(-0.019, 0.016), (-0.0155, 0), (0.0155, 0), (0.019, 0.016)], id="cup", bone="cup", role="body", z=3.5, tone=0.5, region="cup", material=ALU),
            poly([(-0.0172, 0.0152), (0, 0.0158), (0.0172, 0.0152)], closed=False, id="wax-surface", bone="wax", role="detail", z=3.6, tone=0.85, filled=False, region="wax", material=PARA),
        ]
        p += [
            ellipse(0, 0, 0.019, 0.019, id="cup", bone="cup", role="body", z=3.5, tone=0.5, region="cup", material=ALU),
            ellipse(0, 0, 0.0172, 0.0172, id="wax-surface", bone="wick", role="detail", z=3.6, tone=0.85, region="wax", material=PARA),
            ellipse(0, 0, 0.0058, 0.0058, id="flame", bone="flame-body", role="body", z=3.7, tone=0.9, region="flame", material=FL),
        ]
    return f, p


T = {
    200: (
        ("Front view: the cup as one 0.4 mm trapezoid and the flame as one teardrop above it; the flame bone lifts it out of the cup. Plan view: the cup disc and the flame disc in its middle; the wax top disc sits on the wick bone, so the sunken flame of the unlit pose is painted behind it and disappears.",
         "Vorderansicht: das Schälchen als ein 0,4 mm breites Trapez und die Flamme als ein Tropfen darüber; der Flammenhub hebt sie aus dem Schälchen. Draufsicht: die Schälchenscheibe und die Flammenscheibe in ihrer Mitte; die Wachsscheibe hängt am Dochtknochen, die versenkte Flamme der gelöschten Pose wird deshalb dahinter gemalt und verschwindet."),
        ("Front and plan view: cup (merged with wax and wick in the front view) and flame.",
         "Vorder- und Draufsicht: Schälchen (mit Wachs und Docht verschmolzen) und Flamme."),
        ("Front and plan view: tongue, wax line and wick are not drawn.",
         "Vorder- und Draufsicht: Zungenspitze, Wachslinie und Docht sind nicht gezeichnet."),
    ),
    100: (
        ("Front view: the cup trapezoid, the flame and its tongue as two teardrops; the tongue rides on its own bone, so it flickers independently. Plan view: the cup disc, the wax top disc and the flame disc inside it (the sunken flame of the unlit pose is hidden behind the wax); the tongue sits above the flame and is hidden.",
         "Vorderansicht: das Schälchentrapez, die Flamme und ihre Zunge als zwei Tropfen; die Zunge hängt an einem eigenen Knochen und flackert unabhängig. Draufsicht: die Schälchenscheibe, die Wachsscheibe und darin die Flammenscheibe (die versenkte Flamme der gelöschten Pose liegt hinter dem Wachs verdeckt); die Zunge liegt über der Flamme und ist verdeckt."),
        ("Front and plan view: cup (merged), flame; front view: tongue.",
         "Vorder- und Draufsicht: Schälchen (verschmolzen), Flamme; Vorderansicht: Zunge."),
        ("Front and plan view: wax line and wick are not drawn.",
         "Vorder- und Draufsicht: Wachslinie und Docht sind nicht gezeichnet."),
    ),
    50: (
        ("Front view: the conical cup, the wax surface line, the wick, the flame and its tongue; flame and tongue lift and flicker with their bones. Plan view: the cup rim disc, the wax surface disc and the flame disc in the middle (hidden behind the wax disc when the flame is sunk in the unlit pose); the wick and the tongue are hidden below and above the flame.",
         "Vorderansicht: das konische Schälchen, die Wachsoberfläche, der Docht, die Flamme und ihre Zunge; Flamme und Zunge heben sich und flackern mit ihren Knochen. Draufsicht: die Randscheibe des Schälchens, die Wachsoberfläche und die Flammenscheibe in der Mitte (bei versenkter Flamme der gelöschten Pose hinter der Wachsscheibe verdeckt); Docht und Zunge liegen unter und über der Flamme verdeckt."),
        ("Front view: wick, wax line, tongue. Plan view: wax disc.",
         "Vorderansicht: Docht, Wachslinie, Zunge. Draufsicht: Wachsscheibe."),
        ("Plan view: the wick and the tongue are hidden by the flame; the sustainer tab is hidden below the wax.",
         "Draufsicht: Docht und Zunge sind von der Flamme verdeckt; der Dochtfuß liegt verdeckt unter dem Wachs."),
    ),
}

for scale in (200, 100, 50):
    f, p = build(scale)
    s, ret, omi = T[scale]
    install(doc, scale, f, p, {"silhouette": s, "retained": ret, "omitted": omi})
finish(doc)
save(A, doc)
print("[DEBUG] tealight written")
