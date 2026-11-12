import sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from _lib import *




def half_x(y):
    if y <= 0.024:
        return 0.138 + 0.012 * math.sqrt(max(0.0, 1 - ((0.024 - y) / 0.012) ** 2))
    if y <= 0.15:
        return 0.15
    return 0.114 + 0.036 * math.sqrt(max(0.0, 1 - ((y - 0.15) / 0.038) ** 2))


def half_z(y):
    if y <= 0.019:
        return 0.078 + 0.007 * math.sqrt(max(0.0, 1 - ((0.019 - y) / 0.007) ** 2))
    if y <= 0.166:
        return 0.085
    return 0.063 + 0.022 * math.sqrt(max(0.0, 1 - ((y - 0.166) / 0.022) ** 2))


ys = [0.012, 0.0125, 0.014, 0.0165, 0.0195, 0.024, 0.15, 0.1545, 0.1595, 0.1655, 0.171, 0.177, 0.1815, 0.185, 0.1872, 0.188]
casing = loft("casing", "base", "chrome-steel", "body", 0.82, "casing", [((0, y, 0), half_z(y), half_x(y), 9) for y in ys], along="v")

bread_pts = [(-0.052, 0.085), (0.052, 0.085), (0.052, 0.168)] + bezier((0.052, 0.168), (0.062, 0.172), (0.064, 0.19), (0.046, 0.196)) + bezier((0.046, 0.196), (0.03, 0.202), (-0.03, 0.202), (-0.046, 0.196)) + bezier((-0.046, 0.196), (-0.064, 0.19), (-0.062, 0.172), (-0.052, 0.168))
bread_rear = [(x * 0.049 / 0.052 if abs(y) < 0 else x * (0.049 / 0.052), y) for x, y in bread_pts]

S = [
    casing,
    loft("top-plate", "base", "mica-steel", "body", 0.55, "top-plate", [((0, 0.172, 0), 0.071, 0.128, 12), ((0, 0.1885, 0), 0.057, 0.114, 12)], along="v"),
    extrude("slot-1", "base", "mica-steel", "body", 0.12, "top-plate", rrect(0.03, 0, 0.029, 0.12, rad=0.006), "zx", 0.1888, 0.1892),
    extrude("slot-2", "base", "mica-steel", "body", 0.12, "top-plate", rrect(-0.03, 0, 0.029, 0.12, rad=0.006), "zx", 0.1888, 0.1892),
    extrude("slice-rear", "slices", "bread-crust", "body", 0.7, "bread", bread_rear, "xy", 0.0235, 0.0365),
    extrude("slice-front", "slices", "bread-crust", "body", 0.78, "bread", bread_pts, "xy", -0.0365, -0.0235),
    revolve("foot-l", "base", "rubber-foot", "body", 0.15, "feet", [(0, 0.021), (0.0075, 0.025), (0.012, 0.024)], axis=(0, 1, 0), translate=[-0.1, 0, 0.06], repeat={"count": 2, "translation": [0.2, 0, 0]}),
    revolve("foot-r", "base", "rubber-foot", "body", 0.15, "feet", [(0, 0.021), (0.0075, 0.025), (0.012, 0.024)], axis=(0, 1, 0), translate=[-0.1, 0, -0.06], repeat={"count": 2, "translation": [0.2, 0, 0]}),
    box("lever-track", "base", "chrome-steel", "detail", 0.3, "casing", (0.012, 0.11, 0.0016), at=(0.1, 0.107, -0.0852), corner=0.0006, maxd=100),
    box("lever", "lever", "pp-black", "body", 0.2, "control", (0.03, 0.024, 0.004), at=(0, 0, -0.0865), corner=0.0019, maxd=100),
    box("cancel-button", "base", "pp-black", "body", 0.2, "control", (0.024, 0.014, 0.003), at=(-0.028, 0.127, -0.0865), corner=0.0014, maxd=50),
    revolve("dial", "dial", "pp-black", "body", 0.25, "control", [(-0.085, 0.015), (-0.0865, 0.015), (-0.0875, 0.0138), (-0.088, 0.011)], axis=(0, 0, 1), maxd=100),
    box("dial-pointer", "dial", "pp-black", "detail", 0.9, "control", (0.002, 0.012, 0.0012), at=(0, 0.006, -0.0874), maxd=50),
]
D = [dec("The chrome casing is one 16-section superelliptic loft along the height; its radii follow the elevation shoulder (36 mm) and the front shoulder (22 mm) and its exponent 9 reproduces the 22 mm plan corner radius.",
         "Das Chromgehäuse ist ein Loft aus 16 superelliptischen Schnitten entlang der Höhe; die Radien folgen der Schulter der Seitenansicht (36 mm) und der Vorderansicht (22 mm), der Exponent 9 bildet den 22-mm-Eckradius des Grundrisses nach."),
     dec("The two slices are extrusions of their dome outline across the 13 mm slot width, bound to the carriage bone so they lift with every pose; slot-1 and slot-2 are dark 0.4 mm plates on the top plate, the cavities are not cut.",
         "Die beiden Brotscheiben sind Extrusionen ihrer Kuppelkontur über die 13 mm Schlitzbreite und am Schlittenknochen befestigt, sodass sie in jeder Pose mitfahren; slot-1 und slot-2 sind dunkle 0,4 mm dicke Platten auf der Deckplatte, die Hohlräume werden nicht geschnitten."),
     dec("Lever, lever track, cancel button and dial sit on the right (-z) side face; the front and plan drawings do not show them, so they stand at most 3 mm proud to keep the depth at 0.173 m against 0.17 m. Four rubber feet are two solids repeated along x.",
         "Hebel, Hebelschiene, Abbruchtaste und Drehknopf sitzen auf der rechten Seitenfläche (-z); Vorderansicht und Grundriss zeigen sie nicht, daher stehen sie höchstens 3 mm vor und halten die Tiefe bei 0,173 m gegenüber 0,17 m. Vier Gummifüße sind zwei Körper, entlang x wiederholt."),
     dec("Hairline details without material (crumb edge, seam, tray slot, highlight, dial pointer line) are not modelled except the pointer, which is a 1.2 mm ridge.",
         "Haarliniendetails ohne Material (Krümelkante, Naht, Schalenschlitz, Glanzlicht) werden nicht modelliert, der Zeiger ist ein 1,2 mm hoher Steg.")]
write("toaster", S, D)
