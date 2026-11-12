import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
import authoring_components_lib as L
T, F = "track", "leaf"
rect = lambda a0, a1, b0, b1: [(a0, b0), (a0, b1), (a1, b1), (a1, b0)]
def tube(id, bone, region, tone, outer, inner, plane, f, t, repeat=None):
    return L.ext(id, bone, "steel", "body", tone, outer, plane, f, t, region, holes=[inner], repeat=repeat)
s = [
 tube("track", T, "track-steel", 0.45, rect(-0.05, 0.05, -0.05, 0.05), rect(-0.046, 0.046, -0.046, 0.046), "yz", -0.05, 3.05),
 L.box("track-cavity", T, "frame-cavity", "detail", 0.9, [3.1, 0.092, 0.092], [1.5, 0, 0], "track-steel"),
 tube("jamb", T, "jamb-steel", 0.45, rect(-0.05, 0.05, -0.05, 0.05), rect(-0.046, 0.046, -0.046, 0.046), "zx", -2.1, -0.05, {"count": 2, "translation": [3, 0, 0]}),
 L.box("jamb-cavity", T, "frame-cavity", "detail", 0.9, [0.092, 2.05, 0.092], [0, -1.075, 0], "jamb-steel", repeat={"count": 2, "translation": [3, 0, 0]}),
]
for x in (0.3, 1, 2, 2.7):
    s.append(L.cyl("track-screw-%s" % str(x).replace(".", "-"), T, "steel", "detail", 0.2, [x, 0, -0.05], [0, 0, -1], 0.002, 0.012, "track-steel", maxD=50))
s += [
 tube("stile", F, "leaf-frame", 0.5, rect(-0.5, -0.45, -0.025, 0.025), rect(-0.496, -0.454, -0.021, 0.021), "yz", 0.1, 2.0, {"count": 2, "translation": [0, 0.95, 0]}),
 L.box("stile-cavity", F, "frame-cavity", "detail", 0.9, [1.9, 0.042, 0.042], [1.05, -0.475, 0], "leaf-frame", repeat={"count": 2, "translation": [0, 0.95, 0]}),
 tube("rail", F, "leaf-frame", 0.5, rect(-0.025, 0.025, 0.1, 0.15), rect(-0.021, 0.021, 0.104, 0.146), "zx", -0.45, 0.45, {"count": 2, "translation": [1.85, 0, 0]}),
 L.box("rail-cavity", F, "frame-cavity", "detail", 0.9, [0.042, 0.9, 0.042], [0.125, 0, 0], "leaf-frame", repeat={"count": 2, "translation": [1.85, 0, 0]}),
 L.box("leaf-glass", F, "glass", "glazing", 0.92, [1.8, 0.9, 0.01], [1.05, 0, 0], "leaf-glass"),
 L.box("bottom-seal", F, "steel", "body", 0.2, [0.02, 1.0, 0.05], [2.01, 0, 0], "leaf-frame"),
 L.box("hanger", F, "steel", "body", 0.3, [0.05, 0.04, 0.05], [0.075, -0.16, 0], "leaf-frame", repeat={"count": 2, "translation": [0, 0.32, 0]}),
 L.cyl("hanger-bolt", F, "steel", "detail", 0.1, [0.075, -0.16, -0.025], [0, 0, -1], 0.002, 0.008, "leaf-frame", maxD=50, repeat={"count": 2, "translation": [0, 0.32, 0]}),
 L.box("handle-base", F, "steel", "body", 0.3, [0.29, 0.06, 0.004], [1.025, 0.42, 0.007], "leaf-frame", repeat={"count": 2, "translation": [0, 0, -0.014]}),
 L.cap("handle-grip", F, "steel", "body", 0.25, [0.9375, 0.415, 0.0165], [1.1125, 0.415, 0.0165], 0.0075, region="leaf-frame", repeat={"count": 2, "translation": [0, 0, -0.033]}),
]
L.write("sliding-door", s, [
 ("Track, jambs and leaf frame members are hollow steel sections: a 4 mm skin tube (extrude with a hole) filled by a separate cavity solid of frame-cavity material, so skin and cavity layers have their own solids for the thermal view; the 2D thermal layers sum to 58 mm while the plan draws 50 mm, the drawn 50 mm sections are used with a 42 mm cavity.", "Schiene, Pfosten und Flügelrahmen sind Stahlhohlprofile: ein 4 mm dickes Hautrohr (Extrusion mit Loch), gefüllt durch einen eigenen Hohlraumkörper aus dem Material frame-cavity, sodass Haut und Hohlraum eigene Körper für die thermische Ansicht haben; die thermischen 2D-Schichten summieren sich auf 58 mm, die Draufsicht zeichnet 50 mm, es gelten die gezeichneten 50 mm mit 42 mm Hohlraum."),
 ("The leaf is hung 50 mm under the track by two hangers at v = +-0.16 (elevation positions; the plan draws them at the stiles, 2D inconsistency) and keeps its bone on the track so it slides with the prismatic joint; in the open pose its east stile overlaps the east jamb by 50 mm as the joint travel of 1 m dictates.", "Der Flügel hängt 50 mm unter der Schiene an zwei Laufwagen bei v = +-0,16 (Positionen der Ansicht; die Draufsicht zeichnet sie an den Pfosten, 2D-Widerspruch) und bleibt mit seinem Knochen an der Schiene, sodass er mit dem Schubgelenk gleitet; in der offenen Pose überlappt sein östlicher Pfosten den östlichen Rahmenpfosten um 50 mm, wie es der Gelenkweg von 1 m vorgibt."),
 ("The 10 mm glass pane sits between the frame members; the pull handle is a base plate and a grip tube on each glass face; screw heads and bolt heads (only at 1:50) protrude 2 mm; jamb seal lines and glass reflexes are 2D texture.", "Die 10 mm Glasscheibe sitzt zwischen den Rahmenprofilen; der Griff ist je Glasseite eine Grundplatte mit Griffrohr; Schrauben- und Bolzenköpfe (nur bei 1:50) stehen 2 mm vor; Dichtungslinien der Pfosten und Glasreflexe sind 2D-Textur."),
])
