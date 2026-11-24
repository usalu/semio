"""🕸️ Writes the repair ops of `used-climbing-net-pyramid`: the plan of 1:50 gets the seven frayed rope covers at the step points that its side and front elevation draw (zigzags 0.18 m long, 3.6 mm on paper, on the border rope 0.11 m and the first ring 0.55 m above the sand), at the places the two elevations give; 1:100 omits them as under 15 mm. Runs after the rope-end ops of the silhouette round R2-O and touches none of their shapes."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
from lib import asset, like, shape, write
a = asset("used-climbing-net-pyramid")
BORDER, RING, HALF = 3.0, 2.6594, 0.012
across = lambda at, along: [[at + (HALF if i % 2 else -HALF) * (1 if at > 0 else -1), v] for i, v in enumerate(along)]
swap = lambda points: [[y, x] for x, y in points]
FRAYS = {
    "fray-1": ("front", across(BORDER, [-1.14, -1.0806, -1.0212, -0.96])),
    "fray-2": ("front", across(RING, [0.718041, 0.770698, 0.823354, 0.877606])),
    "fray-6": ("front", across(-BORDER, [-1.26, -1.3194, -1.3788, -1.44])),
    "fray-7": ("front", across(-RING, [0.611665, 0.559008, 0.506352, 0.4521])),
    "fray-3": ("elevation", swap(across(-BORDER, [0.66, 0.7194, 0.7788, 0.84]))),
    "fray-4": ("elevation", swap(across(-RING, [-1.276518, -1.223861, -1.171205, -1.116953]))),
    "fray-5": ("elevation", swap(across(BORDER, [0.39, 0.3306, 0.2712, 0.21]))),
}
ops = [{"op": "shape-add", "scale": "50", "view": "plan", "after": "head", "shape": like(shape(a, 50, view, sid), points=[[round(x, 6), round(y, 6)] for x, y in points])} for sid, (view, points) in FRAYS.items()]
ops.append({"op": "note", "scale": "50", "decision": "retained", "en": "Plan: the seven frayed covers on the border rope and the first ring.", "de": "Draufsicht: die sieben aufgescheuerten Mäntel am Randseil und am ersten Ring."})
write("used-climbing-net-pyramid", ops)
