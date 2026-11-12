import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
from authoring_furniture_lib import *
c = Ctx("picnic-table")
PI, GV = "pine-timber", "steel-galvanised"
c.ext("leg-l", "frame", PI, "body", 0.55, [(-0.25, 0.71), (-0.19, 0.71), (-0.66, 0.0), (-0.708, 0.0), (-0.72, 0.012)], "xy", 0.7775, 0.8225, region="a-frame", repeat={"count": 2, "translation": [0, 0, -1.6]})
c.ext("leg-r", "frame", PI, "body", 0.55, [(0.19, 0.71), (0.25, 0.71), (0.72, 0.012), (0.708, 0.0), (0.66, 0.0)], "xy", 0.7775, 0.8225, region="a-frame", repeat={"count": 2, "translation": [0, 0, -1.6]})
c.bw("top-beam", "frame", PI, "body", 0.5, (-0.3, 0.3), (0.665, 0.71), (0.7775, 0.8225), corner=0.003, region="a-frame", repeat={"count": 2, "translation": [0, 0, -1.6]})
c.bw("beam", "frame", PI, "body", 0.5, (-0.48, 0.48), (0.355, 0.4), (0.7775, 0.8225), corner=0.003, region="a-frame", repeat={"count": 2, "translation": [0, 0, -1.6]})
c.bw("hinge-l", "frame", GV, "body", 0.2, (-0.5, -0.42), (0.34, 0.44), (0.67, 0.78), region="hinges", repeat={"count": 2, "translation": [0, 0, -1.45]})
c.bw("hinge-r", "frame", GV, "body", 0.2, (0.42, 0.5), (0.34, 0.44), (0.67, 0.78), region="hinges", repeat={"count": 2, "translation": [0, 0, -1.45]})
c.bw("top-plank", "top", PI, "body", 0.7, (-0.35, -0.212), (0.71, 0.75), (-0.9, 0.9), corner=0.004, region="table-top", repeat={"count": 5, "translation": [0.14, 0, 0]})
c.bw("bench-r", "bench-r", PI, "body", 0.7, (0.4, 0.76), (0.4, 0.44), (-0.85, 0.85), corner=0.01, region="bench-seats")
c.bw("bench-l", "bench-l", PI, "body", 0.7, (-0.76, -0.4), (0.4, 0.44), (-0.85, 0.85), corner=0.01, region="bench-seats")
c.dec("Both A-frames (z = +-0.8) are two splayed legs, a top beam and a cross beam of 45 mm boards; each leg is the drawn parallelogram extruded 45 mm, left and right pair via repeat 1.6 m apart, so the four legs stand at real lateral positions.",
      "Beide A-Rahmen (z = +-0,8) bestehen aus zwei gespreizten Beinen, einem oberen und einem unteren Querbalken aus 45-mm-Brettern; jedes Bein ist das gezeichnete Parallelogramm, 45 mm stark extrudiert, das Paar links und rechts liegt per repeat 1,6 m auseinander, sodass die vier Beine an echten seitlichen Positionen stehen.")
c.dec("The 0.70 by 1.80 m top is five 138 mm planks with 2 mm gaps (the plan draws the plank joints as texture lines); the 0.36 m benches are single 40 mm boards on their own revolute bones hinged at x = +-0.46 so they fold down with the hinge pose.",
      "Die 0,70 mal 1,80 m grosse Tischplatte besteht aus fuenf 138-mm-Bohlen mit 2 mm Fugen (der Grundriss zeichnet die Bohlenstoesse als Texturlinien); die 0,36 m breiten Baenke sind einzelne 40-mm-Bretter auf eigenen Drehknochen mit Scharnier bei x = +-0,46 und klappen mit der Scharnierpose nach unten.")
c.dec("Hinge blocks are drawn 8 mm clear of the A-frame beams in the front view; they are extended to z = 0.78 so that they seat on the frame. The screw heads drawn inside the hinge blocks are hidden and not modelled.",
      "Scharnierbloecke sind in der Vorderansicht 8 mm vom A-Rahmen-Balken entfernt gezeichnet; sie sind bis z = 0,78 verlaengert, damit sie auf dem Rahmen aufsitzen. Die im Scharnierblock gezeichneten Schraubenkoepfe sind verdeckt und nicht modelliert.")
c.write()
