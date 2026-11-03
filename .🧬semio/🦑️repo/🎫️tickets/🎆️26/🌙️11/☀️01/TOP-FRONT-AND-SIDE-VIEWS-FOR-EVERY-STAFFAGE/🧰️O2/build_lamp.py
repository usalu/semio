import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "street-lamp"
doc = load(A)
CON, GALV, AAL, PC = "concrete", "galv-steel", "aluminium-anodised", "polycarbonate"
AY = 5.8
HX = 0.72


def pole_front(b0, b1, scale, **kw):
    return poly([(-b0, 0.12), (b0, 0.12), (b1, 6.0), (-b1, 6.0)], follow=True, id="pole", bone="pole", role="body", z=3, tone=0.4, region="pole-galv", material=GALV, **kw)


def pole_plan(r0, r1, **kw):
    return capsule(0.0, 0.0, 0.0, 0.0, r0, r1, follow=True, id="pole", bone="pole", role="body", z=3, tone=0.4, regions=["pole-galv"], material=GALV, **kw)


def build(scale):
    f, p = [], []
    if scale == 500:
        f += [
            rect(-0.15, 0.0, 0.3, 0.12, id="plinth", bone="foundation", role="body", z=1, tone=0.65, region="plinth-concrete", material=CON),
            pole_front(0.07, 0.05, scale),
            rect(-0.12, AY - 0.06, 0.24, 0.12, id="head", bone="head", role="body", z=4, tone=0.5, regions=["lamp-housing", "lamp-lens"], material=AAL),
        ]
        p += [
            rect(-0.15, -0.15, 0.3, 0.3, id="plinth", bone="foundation", role="body", z=1, tone=0.65, region="plinth-concrete", material=CON),
            pole_plan(0.07, 0.05),
            poly([(0.0, 0.0), (0.72, 0.0)], closed=False, id="arm", bone="arm", role="body", z=2, tone=0.5, filled=False, region="arm-alu", material=AAL, pen=0.18),
            rect(0.72, -0.12, 0.33, 0.24, rad=0.04, id="head", bone="head", role="body", z=4, tone=0.5, regions=["lamp-housing", "lamp-lens"], material=AAL),
        ]
    elif scale == 200:
        f += [
            rect(-0.15, 0.0, 0.3, 0.12, id="plinth", bone="foundation", role="body", z=1, tone=0.65, region="plinth-concrete", material=CON),
            pole_front(0.09, 0.045, scale),
            ellipse(0.0, AY, 0.035, 0.035, id="arm", bone="arm", role="body", z=2, tone=0.5, region="arm-alu", material=AAL),
            poly([(0.0, AY - 0.46), (0.0, AY - 0.012)], closed=False, id="arm-brace", bone="arm", role="detail", z=1.9, tone=0.45, filled=False, region="arm-alu", material=AAL),
            path(f"M -0.12 {AY} C -0.12 {AY+0.08} 0.12 {AY+0.08} 0.12 {AY} L 0.1 {AY-0.05} L -0.1 {AY-0.05} Z", id="head-housing", bone="head", role="body", z=4, tone=0.5, region="lamp-housing", material=AAL),
            poly([(-0.1, AY - 0.058), (0.1, AY - 0.058)], closed=False, id="lens", bone="head", role="glazing", z=5, tone=0.85, filled=False, region="lamp-lens", material=PC),
        ]
        p += [
            rect(-0.15, -0.15, 0.3, 0.3, id="plinth", bone="foundation", role="body", z=1, tone=0.65, region="plinth-concrete", material=CON),
            pole_plan(0.09, 0.045),
            poly([(0.0, 0.0), (0.82, 0.0)], closed=False, id="arm", bone="arm", role="body", z=2, tone=0.5, filled=False, region="arm-alu", material=AAL),
            path(f"M {HX} -0.1 C {HX} -0.13 {HX+0.33} -0.13 {HX+0.33} -0.1 L {HX+0.33} 0.1 C {HX+0.33} 0.13 {HX} 0.13 {HX} 0.1 Z", id="head-housing", bone="head", role="body", z=4, tone=0.5, region="lamp-housing", material=AAL),
            poly([(HX + 0.04, 0.0), (HX + 0.29, 0.0)], closed=False, id="lens", bone="head", role="glazing", z=5, tone=0.85, filled=False, region="lamp-lens", material=PC),
        ]
    elif scale == 100:
        f += [
            rect(-0.15, 0.0, 0.3, 0.12, id="plinth", bone="foundation", role="body", z=1, tone=0.65, region="plinth-concrete", material=CON),
            path("M -0.12 0.12 C -0.11 0.2 -0.085 0.24 -0.082 0.3 L 0.082 0.3 C 0.085 0.24 0.11 0.2 0.12 0.12 Z", id="base-cover", bone="foundation", role="body", z=2.5, tone=0.33, region="pole-galv", material=GALV),
            pole_front(0.085, 0.04, scale),
            rect(-0.055, 5.72, 0.11, 0.08, id="pole-collar", bone="pole", role="detail", z=3.5, tone=0.3, region="pole-galv", material=GALV),
            ellipse(0.0, AY, 0.034, 0.034, id="arm", bone="arm", role="body", z=2, tone=0.5, region="arm-alu", material=AAL),
            poly([(0.0, AY - 0.46), (0.0, AY - 0.012)], closed=False, id="arm-brace", bone="arm", role="detail", z=1.9, tone=0.45, filled=False, region="arm-alu", material=AAL),
            path(f"M -0.12 {AY-0.01} C -0.12 {AY+0.09} 0.12 {AY+0.09} 0.12 {AY-0.01} L 0.1 {AY-0.05} C 0.05 {AY-0.062} -0.05 {AY-0.062} -0.1 {AY-0.05} Z", id="head-housing", bone="head", role="body", z=4, tone=0.5, region="lamp-housing", material=AAL),
            path(f"M -0.1 {AY-0.05} C -0.05 {AY-0.088} 0.05 {AY-0.088} 0.1 {AY-0.05} L 0.1 {AY-0.046} C 0.05 {AY-0.076} -0.05 {AY-0.076} -0.1 {AY-0.046} Z", id="lens", bone="head", role="glazing", z=5, tone=0.85, region="lamp-lens", material=PC, pen=0.138),
        ]
        p += [
            rect(-0.15, -0.15, 0.3, 0.3, id="plinth", bone="foundation", role="body", z=1, tone=0.65, region="plinth-concrete", material=CON),
            ellipse(0.0, 0.0, 0.12, 0.12, id="base-cover", bone="foundation", role="body", z=2.5, tone=0.33, regions=["pole-galv"], material=GALV),
            pole_plan(0.085, 0.04),
            ellipse(0.0, 0.0, 0.055, 0.055, id="pole-collar", bone="arm", role="detail", z=3.5, tone=0.3, region="pole-galv", material=GALV),
            capsule(0.0, 0.0, 0.95, 0.0, 0.034, 0.01, follow=True, id="arm", bone="arm", role="body", z=2, tone=0.5, region="arm-alu", material=AAL),
            path(f"M {HX} -0.07 C {HX+0.03} -0.13 {HX+0.25} -0.13 {HX+0.3} -0.08 C {HX+0.34} -0.04 {HX+0.34} 0.04 {HX+0.3} 0.08 C {HX+0.25} 0.13 {HX+0.03} 0.13 {HX} 0.07 Z", id="head-housing", bone="head", role="body", z=4, tone=0.5, region="lamp-housing", material=AAL),
            poly([(HX + 0.04, -0.09), (HX + 0.04, 0.09)], closed=False, id="head-rim", bone="head", role="detail", z=4.2, tone=0.3, filled=False, region="lamp-housing", material=AAL),
        ]
    else:
        f += [
            rect(-0.15, 0.0, 0.3, 0.1, id="plinth", bone="foundation", role="body", z=1, tone=0.65, region="plinth-concrete", material=CON, pen=0.35),
            poly([(-0.15, 0.05), (0.15, 0.05)], closed=False, id="plinth-joint", bone="foundation", role="detail", z=1.1, tone=0.4, filled=False, region="plinth-concrete", material=CON, pen=0.13),
            rect(-0.15, 0.1, 0.3, 0.02, rad=0.002, id="base-plate", bone="foundation", role="body", z=2, tone=0.35, region="pole-galv", material=GALV),
            rect(-0.132, 0.12, 0.024, 0.014, id="base-nut-0", bone="foundation", role="detail", z=2.1, tone=0.2, region="pole-galv", material=GALV, pen=0.13),
            rect(0.108, 0.12, 0.024, 0.014, id="base-nut-1", bone="foundation", role="detail", z=2.1, tone=0.2, region="pole-galv", material=GALV, pen=0.13),
            path("M -0.12 0.13 C -0.11 0.20 -0.085 0.24 -0.082 0.30 L 0.082 0.30 C 0.085 0.24 0.11 0.20 0.12 0.13 Z", id="base-cover", bone="foundation", role="body", z=2.5, tone=0.33, region="pole-galv", material=GALV, pen=0.35),
            pole_front(0.085, 0.04, scale, pen=0.35),
            poly([(0.055, 0.4), (0.035, 5.6)], closed=False, id="pole-highlight", bone="pole", role="texture", z=3.6, tone=0.3, filled=False, region="pole-galv", material=GALV, pen=0.13),
            poly([(-0.065, 2.22), (0.065, 2.22)], closed=False, id="pole-seam-0", bone="pole", role="texture", z=3.6, tone=0.3, filled=False, region="pole-galv", material=GALV, pen=0.13),
            poly([(-0.06, 4.02), (0.06, 4.02)], closed=False, id="pole-seam-1", bone="pole", role="texture", z=3.6, tone=0.3, filled=False, region="pole-galv", material=GALV, pen=0.13),
            rect(-0.055, 5.72, 0.11, 0.08, rad=0.01, id="pole-collar", bone="pole", role="detail", z=3.5, tone=0.3, region="pole-galv", material=GALV),
            path("M -0.042 5.98 C -0.03 6.02 0.03 6.02 0.042 5.98 Z", id="pole-cap", bone="pole", role="body", z=3.2, tone=0.3, region="pole-galv", material=GALV),
            ellipse(0.0, AY, 0.034, 0.034, id="arm", bone="arm", role="body", z=2, tone=0.5, region="arm-alu", material=AAL, pen=0.35),
            rect(-0.05, AY - 0.05, 0.1, 0.1, rad=0.008, id="arm-clamp", bone="arm", role="detail", z=2.2, tone=0.3, region="arm-alu", material=AAL),
            capsule(0.0, AY - 0.46, 0.0, AY - 0.012, 0.012, 0.012, id="arm-brace", bone="arm", role="body", z=1.9, tone=0.45, region="arm-alu", material=AAL, pen=0.35),
            path(f"M -0.12 {AY-0.01} C -0.12 {AY+0.09} 0.12 {AY+0.09} 0.12 {AY-0.01} L 0.1 {AY-0.05} C 0.05 {AY-0.062} -0.05 {AY-0.062} -0.1 {AY-0.05} Z", id="head-housing", bone="head", role="body", z=4, tone=0.5, region="lamp-housing", material=AAL, pen=0.41),
            rect(-0.035, AY - 0.02, 0.07, 0.05, rad=0.006, id="head-spigot", bone="head", role="detail", z=4.05, tone=0.35, region="lamp-housing", material=AAL),
            poly([(-0.08, AY + 0.04), (0.08, AY + 0.04)], closed=False, id="head-fin-0", bone="head", role="detail", z=4.1, tone=0.3, filled=False, region="lamp-housing", material=AAL, pen=0.13),
            poly([(-0.045, AY + 0.056), (0.045, AY + 0.056)], closed=False, id="head-fin-1", bone="head", role="detail", z=4.1, tone=0.3, filled=False, region="lamp-housing", material=AAL, pen=0.13),
            path(f"M -0.1 {AY-0.05} C -0.05 {AY-0.088} 0.05 {AY-0.088} 0.1 {AY-0.05} L 0.1 {AY-0.046} C 0.05 {AY-0.076} -0.05 {AY-0.076} -0.1 {AY-0.046} Z", id="lens", bone="head", role="glazing", z=5, tone=0.85, region="lamp-lens", material=PC),
        ]
        p += [
            rect(-0.15, -0.15, 0.3, 0.3, id="plinth", bone="foundation", role="body", z=1, tone=0.65, region="plinth-concrete", material=CON, pen=0.35),
            rect(-0.15, -0.15, 0.3, 0.3, rad=0.004, id="base-plate", bone="foundation", role="body", z=2, tone=0.35, region="pole-galv", material=GALV),
        ]
        for i, (sx, sy) in enumerate([(-1, -1), (1, -1), (-1, 1), (1, 1)]):
            p.append(ellipse(0.12 * sx, 0.12 * sy, 0.013, 0.013, id=f"base-nut-{i}", bone="foundation", role="detail", z=2.1, tone=0.2, regions=["pole-galv"], material=GALV, pen=0.13))
        p += [
            ellipse(0.0, 0.0, 0.12, 0.12, id="base-cover", bone="foundation", role="body", z=2.5, tone=0.33, regions=["pole-galv"], material=GALV, pen=0.35),
            pole_plan(0.085, 0.04, pen=0.35),
            ellipse(0.0, 0.0, 0.055, 0.055, id="pole-collar", bone="arm", role="detail", z=3.5, tone=0.3, region="pole-galv", material=GALV),
            capsule(0.0, 0.0, 0.95, 0.0, 0.034, 0.01, follow=True, id="arm", bone="arm", role="body", z=2, tone=0.5, region="arm-alu", material=AAL, pen=0.35),
            rect(-0.05, -0.05, 0.1, 0.1, rad=0.008, id="arm-clamp", bone="arm", role="detail", z=2.2, tone=0.3, region="arm-alu", material=AAL),
            path(f"M {HX} -0.07 C {HX+0.03} -0.13 {HX+0.25} -0.13 {HX+0.3} -0.08 C {HX+0.34} -0.04 {HX+0.34} 0.04 {HX+0.3} 0.08 C {HX+0.25} 0.13 {HX+0.03} 0.13 {HX} 0.07 Z", id="head-housing", bone="head", role="body", z=4, tone=0.5, region="lamp-housing", material=AAL, pen=0.41),
            rect(HX - 0.04, -0.035, 0.07, 0.07, rad=0.006, id="head-spigot", bone="head", role="detail", z=4.05, tone=0.35, region="lamp-housing", material=AAL),
            path(f"M {HX+0.03} 0.055 C {HX+0.04} 0.095 {HX+0.11} 0.098 {HX+0.13} 0.07", id="head-driver", bone="head", role="detail", z=4.1, tone=0.3, filled=False, region="lamp-housing", material=AAL),
        ]
        for k in range(6):
            x = HX + 0.15 + 0.028 * k
            p.append(poly([(x, -0.075), (x, 0.075)], closed=False, id=f"head-fin-{k}", bone="head", role="detail", z=4.1, tone=0.3, filled=False, region="lamp-housing", material=AAL, pen=0.13))
        p.append(poly([(HX + 0.04, -0.09), (HX + 0.04, 0.09)], closed=False, id="head-rim", bone="head", role="detail", z=4.2, tone=0.3, filled=False, region="lamp-housing", material=AAL))
    return f, p


T = {
    500: (
        ("Front view: the 0.28 mm pole tapering from 0.14 m to 0.10 m over the full height, a plinth block and the lamp head seen end-on as one small block at the top. Plan view: plinth square, the pole as a stretching capsule (it leans with the pole), the arm as a line and the head as one rounded block; the arm and head follow the sway of the pole.",
         "Vorderansicht: der 0,14 mm breite Mast, der sich von 0,14 m auf 0,10 m über die ganze Höhe verjüngt, ein Sockelblock und der Leuchtenkopf stirnseitig als ein kleiner Block oben. Draufsicht: das Sockelquadrat, der Mast als sich dehnende Kapsel (er neigt sich mit dem Mast), der Arm als Linie und der Kopf als ein abgerundeter Block; Arm und Kopf folgen dem Wiegen des Mastes."),
        ("Front and plan view: plinth, pole, arm and head.",
         "Vorder- und Draufsicht: Sockel, Mast, Arm und Kopf."),
        ("Front and plan view: the arm is hidden end-on in the front view; base cover, collar and brace are not drawn.",
         "Vorder- und Draufsicht: der Arm verschwindet in der Vorderansicht stirnseitig; Fußhaube, Manschette und Strebe sind nicht gezeichnet."),
    ),
    200: (
        ("Front view: the tapering pole with plinth, the arm seen end-on as a dot, its brace as a vertical line and the head with its lens line as a rounded block. Plan view: the plinth, the pole capsule that stretches as the pole sways, the arm line and the head outline with the lens line.",
         "Vorderansicht: der verjüngte Mast mit Sockel, der Arm stirnseitig als Punkt, seine Strebe als senkrechte Linie und der Kopf mit der Linse als abgerundeter Block. Draufsicht: der Sockel, die Mastkapsel, die sich beim Wiegen des Mastes dehnt, die Armlinie und die Kopfkontur mit der Linsenlinie."),
        ("Front view: brace, lens line. Plan view: lens line.",
         "Vorderansicht: Strebe, Linsenlinie. Draufsicht: Linsenlinie."),
        ("Front and plan view: no base cover, collar, fins or clamp.",
         "Vorder- und Draufsicht: keine Fußhaube, Manschette, Kühlrippen oder Klemme."),
    ),
    100: (
        ("Front view: plinth, base cover, tapering pole with collar, the arm end-on with its vertical brace line, and the head as a domed block with the lens strip below. Plan view: plinth square, round base cover, the pole capsule (0.17 m to 0.08 m, stretching with the sway), collar ring on the arm, the tapering arm capsule and the head with its rim line.",
         "Vorderansicht: Sockel, Fußhaube, verjüngter Mast mit Manschette, der Arm stirnseitig mit senkrechter Strebenlinie und der Kopf als gewölbter Block mit dem Linsenstreifen darunter. Draufsicht: das Sockelquadrat, die runde Fußhaube, die Mastkapsel (0,17 m bis 0,08 m, dehnt sich mit dem Wiegen), der Manschettenring am Arm, die verjüngte Armkapsel und der Kopf mit Randlinie."),
        ("Front view: collar, brace line. Plan view: collar ring, rim line.",
         "Vorderansicht: Manschette, Strebenlinie. Draufsicht: Manschettenring, Randlinie."),
        ("Front and plan view: base plate, nuts, fins, seams and highlight are not drawn.",
         "Vorder- und Draufsicht: Fußplatte, Muttern, Kühlrippen, Nähte und Glanzlinie sind nicht gezeichnet."),
    ),
    50: (
        ("Front view: plinth with joint, base plate with nuts, base cover, tapering pole with highlight, two seams, collar and cap, the arm end-on with clamp and a capsule brace, and the head with spigot, two fin lines and the lens strip. Plan view: stippled plinth, base plate with four nuts, base cover, the pole capsule that stretches as the pole sways, collar ring, tapering arm with clamp, the head with spigot, driver outline, six cooling fins and rim line; arm and head follow the sway.",
         "Vorderansicht: Sockel mit Fuge, Fußplatte mit Muttern, Fußhaube, verjüngter Mast mit Glanzlinie, zwei Nähten, Manschette und Kappe, der Arm stirnseitig mit Klemme und kapselförmiger Strebe und der Kopf mit Zapfen, zwei Rippenlinien und dem Linsenstreifen. Draufsicht: der gepunktete Sockel, die Fußplatte mit vier Muttern, die Fußhaube, die Mastkapsel, die sich beim Wiegen dehnt, der Manschettenring, der verjüngte Arm mit Klemme, der Kopf mit Zapfen, Treiberumriss, sechs Kühlrippen und Randlinie; Arm und Kopf folgen dem Wiegen."),
        ("Front view: nuts, seams, highlight, cap, fin lines. Plan view: nuts, driver, six fins.",
         "Vorderansicht: Muttern, Nähte, Glanzlinie, Kappe, Rippenlinien. Draufsicht: Muttern, Treiber, sechs Kühlrippen."),
        ("Front view: only two of the six cooling fins are shown, the others hide behind the housing dome.",
         "Vorderansicht: nur zwei der sechs Kühlrippen sind gezeigt, die anderen verschwinden hinter der Gehäusekuppel."),
    ),
}

for scale in (500, 200, 100, 50):
    f, p = build(scale)
    s, ret, omi = T[scale]
    install(doc, scale, f, p, {"silhouette": s, "retained": ret, "omitted": omi})
finish(doc)
save(A, doc)
print("[DEBUG] lamp written")
