import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

A = "shopping-trolley"
doc = load(A)
ZW, ZT, PG, PP, TY = "zinc-wire", "zinc-tube", "pp-grip", "pu-tyre", None

bones = doc["asset"]["rig"]["bones"]
bones[:] = [x for x in bones if x["id"] != "gate-axis"]
SOL = {"kind": "extruded", "z0": -0.24, "z1": 0.24}
GATE = [(0.014552, 0.003638), (0.134552, -0.476362), (0.105448, -0.483638), (-0.014552, -0.003638)]


def gate_parts(scale):
    solid = scale <= 100
    return [poly(GATE, id="gate", bone="gate", role="body", z=3.4, tone=0.5 if solid else 0.3, filled=solid, region="gate-wire", material=ZW, pen=None if solid else (0.13 if scale == 500 else 0.25), solid=SOL)]


HX, HY = -0.42, 0.88
TOP, BOT, HT, HB = 0.285, 0.2, 0.88, 0.38


def hw(y):
    return BOT + (TOP - BOT) * (y - HB) / (HT - HB)


def wheel_front(side, scale):
    x0 = 0.185 if side > 0 else -0.23
    return x0


def build(scale):
    f, p = [], []
    gate_x = [-0.2, -0.1, 0.0, 0.1, 0.2]
    if scale == 500:
        f += [
            poly([(-TOP, HT), (TOP, HT), (BOT, HB), (-BOT, HB)], id="basket", bone="basket", role="body", z=3, tone=0.15, regions=["basket-wire", "gate-wire"], material=ZW),
            poly([(-0.2, 0.17), (0.2, 0.17)], closed=False, id="chassis-line", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.13),
            poly([(-0.2, 0.38), (-0.215, 0.17), (-0.2, 0.0625)], closed=False, id="struts", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.13),
            poly([(0.2, 0.38), (0.215, 0.17), (0.2, 0.0625)], closed=False, id="front-strut", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.13),
            poly([(-0.245, 0.88), (-0.245, 1.0), (0.245, 1.0), (0.245, 0.88)], closed=False, id="handle", bone="handle", role="body", z=3.5, tone=0.3, filled=False, region="handle-grip", material=PG, pen=0.18),
            rect(-0.23, 0.0, 0.1, 0.15, id="wheel-rear", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(0.13, 0.0, 0.1, 0.15, id="wheel-front", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
        ]
        p += [
            rect(-0.42, -TOP, 0.92, 2 * TOP, rad=0.02, id="basket", bone="basket", role="body", z=3, tone=0.15, regions=["basket-wire", "gate-wire"], material=ZW),
            poly([(-0.36, 0.245), (-0.55, 0.245), (-0.55, -0.245), (-0.36, -0.245)], closed=False, id="handle", bone="handle", role="body", z=3.5, tone=0.3, filled=False, region="handle-grip", material=PG, pen=0.18),
        ]
    elif scale == 200:
        f += [
            poly([(-TOP, HT), (TOP, HT), (BOT, HB), (-BOT, HB)], id="basket", bone="basket", role="body", z=3, tone=0.12, region="basket-wire", material=ZW),
            poly([(-hw(0.54), 0.54), (hw(0.54), 0.54)], closed=False, id="wire-h-0", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-hw(0.7), 0.7), (hw(0.7), 0.7)], closed=False, id="wire-h-1", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-0.215, 0.17), (0.215, 0.17)], closed=False, id="chassis-line", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.282),
            poly([(-0.2, 0.38), (-0.215, 0.17), (-0.2, 0.0625)], closed=False, id="rear-fork", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.18),
            poly([(0.2, 0.38), (0.215, 0.17), (0.2, 0.0625)], closed=False, id="front-fork", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.18),
            poly([(-0.245, 0.88), (-0.245, 0.97)], closed=False, id="handle-post-l", bone="handle", role="body", z=3.5, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.18),
            poly([(0.245, 0.88), (0.245, 0.97)], closed=False, id="handle-post-r", bone="handle", role="body", z=3.5, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.18),
            rect(-0.26, 0.96, 0.52, 0.04, id="grip", bone="handle", role="body", z=3.6, tone=0.7, region="handle-grip", material=PG),
            rect(0.19, 0.0, 0.04, 0.125, id="wheel-front-l", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre", pen=0.231),
            rect(-0.23, 0.0, 0.04, 0.125, id="wheel-front-r", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre", pen=0.231),
        ]
        p += [
            rect(-0.42, -TOP, 0.92, 2 * TOP, rad=0.02, id="basket", bone="basket", role="body", z=2, tone=0.12, region="basket-wire", material=ZW),
            rect(-0.3, -BOT, 0.64, 2 * BOT, id="floor", bone="basket", role="body", z=3, tone=0.12, region="basket-wire", material=ZW),
            poly([(-0.3, 0.0), (0.34, 0.0)], closed=False, id="wire-h-0", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            capsule(-0.51, -0.245, -0.53, 0.245, 0.02, 0.02, id="grip", bone="handle", role="body", z=3.6, tone=0.7, region="handle-grip", material=PG),
            capsule(-0.51, 0.245, -0.36, 0.245, 0.015, 0.015, id="handle-arm-l", bone="handle", role="body", z=3.5, tone=0.3, region="frame-tube", material=ZT),
            capsule(-0.51, -0.245, -0.36, -0.245, 0.015, 0.015, id="handle-arm-r", bone="handle", role="body", z=3.5, tone=0.3, region="frame-tube", material=ZT),
            rect(-0.36, 0.2, 0.125, 0.05, id="wheel-rear-l", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(-0.36, -0.25, 0.125, 0.05, id="wheel-rear-r", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(0.265, 0.2, 0.125, 0.05, id="wheel-front-l", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(0.265, -0.25, 0.125, 0.05, id="wheel-front-r", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
        ]
    elif scale == 100:
        f += [
            poly([(-TOP, HT), (TOP, HT), (BOT, HB), (-BOT, HB)], id="basket", bone="basket", role="body", z=3, tone=0.1, region="basket-wire", material=ZW),
            poly([(-hw(0.48), 0.48), (hw(0.48), 0.48)], closed=False, id="wire-h-0", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-hw(0.58), 0.58), (hw(0.58), 0.58)], closed=False, id="wire-h-1", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-hw(0.68), 0.68), (hw(0.68), 0.68)], closed=False, id="wire-h-2", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-hw(0.78), 0.78), (hw(0.78), 0.78)], closed=False, id="wire-h-3", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-0.13, HB), (-0.19, HT)], closed=False, id="wire-v-0", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(0.0, HB), (0.0, HT)], closed=False, id="wire-v-1", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(0.13, HB), (0.19, HT)], closed=False, id="wire-v-2", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-TOP, 0.865), (TOP, 0.865)], closed=False, id="rim", bone="basket", role="detail", z=3.3, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-0.215, 0.17), (0.215, 0.17)], closed=False, id="chassis-line", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.25),
            poly([(-0.2, 0.38), (-0.215, 0.17), (-0.2, 0.0625)], closed=False, id="rear-fork", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.25),
            poly([(0.2, 0.38), (0.215, 0.17), (0.2, 0.0625)], closed=False, id="front-fork", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.25),
            poly([(-0.245, 0.88), (-0.245, 0.97)], closed=False, id="handle-post-l", bone="handle", role="body", z=3.5, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.25),
            poly([(0.245, 0.88), (0.245, 0.97)], closed=False, id="handle-post-r", bone="handle", role="body", z=3.5, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.25),
            rect(-0.26, 0.96, 0.52, 0.04, rad=0.012, id="grip", bone="handle", role="body", z=3.6, tone=0.7, region="handle-grip", material=PG),
            rect(0.19, 0.0, 0.04, 0.125, rad=0.01, id="wheel-front-l", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(-0.23, 0.0, 0.04, 0.125, rad=0.01, id="wheel-front-r", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(0.195, 0.045, 0.03, 0.035, id="hub-front-l", bone="chassis", role="detail", z=2.6, tone=0.7, region="wheels"),
            rect(-0.225, 0.045, 0.03, 0.035, id="hub-front-r", bone="chassis", role="detail", z=2.6, tone=0.7, region="wheels"),
        ]
        p += [
            rect(-0.42, -TOP, 0.92, 2 * TOP, rad=0.02, id="basket", bone="basket", role="body", z=2, tone=0.1, region="basket-wire", material=ZW),
            rect(-0.3, -BOT, 0.64, 2 * BOT, id="floor", bone="basket", role="body", z=3, tone=0.1, region="basket-wire", material=ZW),
            poly([(-0.3, -0.1), (0.34, -0.1)], closed=False, id="wire-h-0", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-0.3, 0.1), (0.34, 0.1)], closed=False, id="wire-h-1", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(0.0, -BOT), (0.0, BOT)], closed=False, id="wire-v-0", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(0.2, -BOT), (0.2, BOT)], closed=False, id="wire-v-1", bone="basket", role="detail", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-0.42, -TOP), (0.5, -TOP), (0.5, TOP), (-0.42, TOP)], closed=True, id="rim", bone="basket", role="detail", z=3.3, tone=0.3, filled=False, region="basket-wire", material=ZW),
            capsule(-0.51, -0.245, -0.53, 0.245, 0.02, 0.02, id="grip", bone="handle", role="body", z=3.6, tone=0.7, region="handle-grip", material=PG),
            capsule(-0.51, 0.245, -0.36, 0.245, 0.015, 0.015, id="handle-arm-l", bone="handle", role="body", z=3.5, tone=0.3, region="frame-tube", material=ZT),
            capsule(-0.51, -0.245, -0.36, -0.245, 0.015, 0.015, id="handle-arm-r", bone="handle", role="body", z=3.5, tone=0.3, region="frame-tube", material=ZT),
            rect(-0.36, 0.2, 0.125, 0.04, rad=0.01, id="wheel-rear-l", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(-0.36, -0.24, 0.125, 0.04, rad=0.01, id="wheel-rear-r", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(0.265, 0.2, 0.125, 0.04, rad=0.01, id="wheel-front-l", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(0.265, -0.24, 0.125, 0.04, rad=0.01, id="wheel-front-r", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
        ]
    else:
        wh = [(0.48 + 0.06 * i) for i in range(8)]
        f += [
            poly([(-TOP, HT), (TOP, HT), (BOT, HB), (-BOT, HB)], id="basket", bone="basket", role="body", z=3, tone=0.08, region="basket-wire", material=ZW),
        ]
        for i, y in enumerate([0.44, 0.50, 0.56, 0.62, 0.68, 0.74, 0.80, 0.86]):
            f.append(poly([(-hw(y), y), (hw(y), y)], closed=False, id=f"wire-h-{i}", bone="basket", role="texture", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW))
        for i, (xb, xt) in enumerate([(-0.16, -0.23), (-0.08, -0.115), (0.0, 0.0), (0.08, 0.115), (0.16, 0.23)]):
            f.append(poly([(xb, HB), (xt, HT)], closed=False, id=f"wire-v-{i}", bone="basket", role="texture", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW))
        f += [
            poly([(-TOP, 0.865), (TOP, 0.865)], closed=False, id="rim", bone="basket", role="detail", z=3.3, tone=0.3, filled=False, region="basket-wire", material=ZW),
            poly([(-0.2, 0.395), (0.2, 0.395)], closed=False, id="floor-wire", bone="basket", role="detail", z=3.3, tone=0.3, filled=False, region="basket-wire", material=ZW),
            ellipse(-0.24, HY, 0.014, 0.014, id="gate-eye-l", bone="gate", role="detail", z=3.6, tone=0.9, region="gate-wire"),
            ellipse(0.24, HY, 0.014, 0.014, id="gate-eye-r", bone="gate", role="detail", z=3.6, tone=0.9, region="gate-wire"),
            poly([(-0.215, 0.17), (0.215, 0.17)], closed=False, id="chassis-line", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.35),
            poly([(-0.2, 0.38), (-0.215, 0.17), (-0.2, 0.0625)], closed=False, id="rear-fork", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.35),
            poly([(0.2, 0.38), (0.215, 0.17), (0.2, 0.0625)], closed=False, id="front-fork", bone="chassis", role="body", z=2.6, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.35),
            rect(-0.235, 0.165, 0.05, 0.015, id="castor-plate-r", bone="chassis", role="body", z=2.7, tone=0.6, region="frame-tube", material=ZT),
            rect(0.185, 0.165, 0.05, 0.015, id="castor-plate-l", bone="chassis", role="body", z=2.7, tone=0.6, region="frame-tube", material=ZT),
            poly([(-0.245, 0.88), (-0.245, 0.97)], closed=False, id="handle-post-l", bone="handle", role="body", z=3.5, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.35),
            poly([(0.245, 0.88), (0.245, 0.97)], closed=False, id="handle-post-r", bone="handle", role="body", z=3.5, tone=0.3, filled=False, region="frame-tube", material=ZT, pen=0.35),
            rect(-0.26, 0.96, 0.52, 0.04, rad=0.012, id="grip", bone="handle", role="body", z=3.6, tone=0.7, region="handle-grip", material=PG, hatch="grip-ribs"),
            rect(0.19, 0.0, 0.04, 0.125, rad=0.01, id="wheel-front-l", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(-0.23, 0.0, 0.04, 0.125, rad=0.01, id="wheel-front-r", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(0.195, 0.045, 0.03, 0.035, id="hub-front-l", bone="chassis", role="detail", z=2.6, tone=0.7, region="wheels"),
            rect(-0.225, 0.045, 0.03, 0.035, id="hub-front-r", bone="chassis", role="detail", z=2.6, tone=0.7, region="wheels"),
            poly([(0.2, 0.01), (0.2, 0.115)], closed=False, id="tread-front-l", bone="chassis", role="texture", z=2.55, tone=0.3, filled=False, region="wheels"),
            poly([(-0.2, 0.01), (-0.2, 0.115)], closed=False, id="tread-front-r", bone="chassis", role="texture", z=2.55, tone=0.3, filled=False, region="wheels"),
        ]
        p += [
            rect(-0.42, -TOP, 0.92, 2 * TOP, rad=0.02, id="basket", bone="basket", role="body", z=2, tone=0.08, region="basket-wire", material=ZW),
            rect(-0.3, -BOT, 0.64, 2 * BOT, id="floor", bone="basket", role="body", z=3, tone=0.08, region="basket-wire", material=ZW),
        ]
        for i, yv in enumerate([-0.15, -0.075, 0.0, 0.075, 0.15]):
            p.append(poly([(-0.3, yv), (0.34, yv)], closed=False, id=f"wire-h-{i}", bone="basket", role="texture", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW))
        for i, xv in enumerate([-0.2, -0.08, 0.04, 0.16, 0.28]):
            p.append(poly([(xv, -BOT), (xv, BOT)], closed=False, id=f"wire-v-{i}", bone="basket", role="texture", z=3.2, tone=0.3, filled=False, region="basket-wire", material=ZW))
        p += [
            poly([(-0.42, -TOP), (0.5, -TOP), (0.5, TOP), (-0.42, TOP)], closed=True, id="rim", bone="basket", role="detail", z=3.3, tone=0.3, filled=False, region="basket-wire", material=ZW),
            rect(-0.428, -0.24, 0.016, 0.48, id="gate-hinge", bone="basket", role="detail", z=3.35, tone=0.3, region="gate-wire", material=ZW),
        ]
        p += [
            capsule(-0.51, -0.245, -0.53, 0.245, 0.02, 0.02, id="grip", bone="handle", role="body", z=3.6, tone=0.7, region="handle-grip", material=PG, hatch="grip-ribs"),
            capsule(-0.51, 0.245, -0.36, 0.245, 0.015, 0.015, id="handle-arm-l", bone="handle", role="body", z=3.5, tone=0.3, region="frame-tube", material=ZT),
            capsule(-0.51, -0.245, -0.36, -0.245, 0.015, 0.015, id="handle-arm-r", bone="handle", role="body", z=3.5, tone=0.3, region="frame-tube", material=ZT),
            rect(-0.36, 0.2, 0.125, 0.04, rad=0.01, id="wheel-rear-l", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(-0.36, -0.24, 0.125, 0.04, rad=0.01, id="wheel-rear-r", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(0.265, 0.2, 0.125, 0.04, rad=0.01, id="wheel-front-l", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
            rect(0.265, -0.24, 0.125, 0.04, rad=0.01, id="wheel-front-r", bone="chassis", role="body", z=2.5, tone=0.15, region="wheels", material="pu-tyre"),
        ]
    f += gate_parts(scale)
    p += gate_parts(scale)
    return f, p


T = {
    500: (
        ("Front view: the basket as one trapezoid with the rear gate as an outline inside it, a U-shaped push handle above the rim, thin fork lines and two wheel blocks. Plan view: the basket rim as one rounded rectangle, the push handle as a U line behind it and the gate as one outline at the rear edge. The gate is an extruded solid on its hinge bone: swung up, it shrinks to a band at the top of the basket in the front view and reaches forward over the basket in the plan.",
         "Vorderansicht: der Korb als ein Trapez mit der hinteren Klappe als Umriss darin, ein U-förmiger Schiebegriff über dem Rand, dünne Gabellinien und zwei Radblöcke. Draufsicht: der Korbrand als ein abgerundetes Rechteck, der Schiebegriff als U-Linie dahinter und die Klappe als ein Umriss am hinteren Rand. Die Klappe ist ein extrudierter Körper am Scharnierknochen: hochgeschwenkt schrumpft sie in der Vorderansicht zu einem Band am oberen Korbrand und reicht in der Draufsicht nach vorn über den Korb."),
        ("Front and plan view: gate, handle, wheels (front view only).",
         "Vorder- und Draufsicht: Klappe, Griff, Räder (nur Vorderansicht)."),
        ("Front and plan view: wire grid, chassis tube and castor plates are not drawn; in the plan the wheels are hidden below the basket.",
         "Vorder- und Draufsicht: Drahtgitter, Fahrgestellrohr und Rollenplatten sind nicht gezeichnet; in der Draufsicht verschwinden die Räder unter dem Korb."),
    ),
    200: (
        ("Front view: the tapering basket with two wire levels, the gate outline hung from the rear rim, the chassis tube, both forks, handle posts with the grip bar and the two front wheels. Plan view: the rim and floor of the basket, one wire line, the U-shaped push handle with grip bar, four wheel blocks beside the floor and the gate outline at the rear rim. The gate is an extruded solid on its hinge bone, so the swung gate rises into the top band in the front view and reaches forward in the plan.",
         "Vorderansicht: der konische Korb mit zwei Drahtebenen, der am hinteren Rand hängende Klappenumriss, das Fahrgestellrohr, beide Gabeln, Griffpfosten mit Griffstange und die zwei Vorderräder. Draufsicht: Rand und Boden des Korbs, eine Drahtlinie, der U-förmige Schiebegriff mit Griffstange, vier Radblöcke neben dem Boden und der Klappenumriss am hinteren Rand. Die Klappe ist ein extrudierter Körper am Scharnierknochen: die geschwenkte Klappe steigt in der Vorderansicht ins obere Band und reicht in der Draufsicht nach vorn."),
        ("Front view: handle posts, grip bar, forks. Plan view: floor, U handle, wheels, gate.",
         "Vorderansicht: Griffpfosten, Griffstange, Gabeln. Draufsicht: Boden, U-Griff, Räder, Klappe."),
        ("Front and plan view: no hubs, no tread lines, only two wire levels.",
         "Vorder- und Draufsicht: keine Naben, keine Profillinien, nur zwei Drahtebenen."),
    ),
    100: (
        ("Front view: the tapering basket with four wire levels, three verticals, rim line, the rear gate as a filled panel, chassis tube, forks, handle posts with the rounded grip and the two front wheels with hubs. Plan view: the outer rim, the narrower floor with two wires each way, the U-shaped push handle with round grip bar, four wheels at the corners and the gate panel at the rear rim. The gate is an extruded solid on its hinge bone, so the swung gate rises inside the basket in the front view and reaches forward over it in the plan.",
         "Vorderansicht: der konische Korb mit vier Drahtebenen, drei Senkrechten und Randlinie, die hintere Klappe als gefüllte Tafel, Fahrgestellrohr, Gabeln, Griffpfosten mit gerundeter Griffstange und die zwei Vorderräder mit Naben. Draufsicht: der äußere Rand, der engere Boden mit je zwei Drähten pro Richtung, der U-förmige Schiebegriff mit runder Griffstange, vier Räder an den Ecken und die Klappentafel am hinteren Rand. Die Klappe ist ein extrudierter Körper am Scharnierknochen: die geschwenkte Klappe steigt in der Vorderansicht im Korb auf und reicht in der Draufsicht nach vorn darüber."),
        ("Front view: wire levels, rim line, hubs. Plan view: rim outline, floor wires, gate panel.",
         "Vorderansicht: Drahtebenen, Randlinie, Naben. Draufsicht: Randumriss, Bodendrähte, Klappentafel."),
        ("Front and plan view: castor plates and wheel tread are not drawn; the gate wires are one filled panel.",
         "Vorder- und Draufsicht: Rollenplatten und Reifenprofil sind nicht gezeichnet; die Klappendrähte sind eine gefüllte Tafel."),
    ),
    50: (
        ("Front view: the tapering basket with eight wire levels, five verticals, rim line and floor wire, the gate panel with its two hinge eyes, chassis tube with castor plates, forks, handle posts, the ribbed grip bar and both front wheels with hubs and tread lines. Plan view: the outer rim, the floor with five wires each way, the hinge bar and the gate panel, the U-shaped push handle with ribbed grip and four wheels at the corners. The gate is an extruded solid on its hinge bone: swung up, it rises inside the basket in the front view and swings forward over the floor in the plan.",
         "Vorderansicht: der konische Korb mit acht Drahtebenen, fünf Senkrechten, Randlinie und Bodendraht, die Klappentafel mit ihren zwei Scharnierösen, Fahrgestellrohr mit Rollenplatten, Gabeln, Griffpfosten, die geriffelte Griffstange und beide Vorderräder mit Naben und Profillinien. Draufsicht: der äußere Rand, der Boden mit je fünf Drähten pro Richtung, der Scharnierstab und die Klappentafel, der U-förmige Schiebegriff mit Riffelung und vier Räder an den Ecken. Die Klappe ist ein extrudierter Körper am Scharnierknochen: hochgeschwenkt steigt sie in der Vorderansicht im Korb auf und schwenkt in der Draufsicht nach vorn über den Boden."),
        ("Front view: eyes, castor plates, tread lines, floor wire. Plan view: hinge bar, ribbed grip.",
         "Vorderansicht: Ösen, Rollenplatten, Profillinien, Bodendraht. Draufsicht: Scharnierstab, geriffelter Griff."),
        ("Front view: the rear wheels hide behind the front wheels; the gate wires are one panel. Plan view: the chassis tube lies hidden below the floor.",
         "Vorderansicht: die Hinterräder verschwinden hinter den Vorderrädern; die Klappendrähte sind eine Tafel. Draufsicht: das Fahrgestellrohr liegt verdeckt unter dem Boden."),
    ),
}

for scale in (500, 200, 100, 50):
    f, p = build(scale)
    s, ret, omi = T[scale]
    install(doc, scale, f, p, {"silhouette": s, "retained": ret, "omitted": omi})
finish(doc)
save(A, doc)
print("[DEBUG] trolley written")
