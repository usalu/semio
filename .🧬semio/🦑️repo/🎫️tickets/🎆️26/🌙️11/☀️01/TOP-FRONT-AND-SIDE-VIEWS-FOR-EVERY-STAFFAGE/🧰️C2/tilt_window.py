from lib import *

A = "tilt-window"
doc = load(A)
S, H = "sill", "sash"
TM = "frame-timber"


def X(shape, z0, z1):
    shape["solid"] = {"kind": "extruded", "z0": z0, "z1": z1}
    return shape


def sash(sc, view):
    t = 0.045
    out = []
    if sc == 200:
        out.append(X(R("sash-frame", H, "body", 3, 0, -t, 1.2, 2 * t, tone=0.6, region=TM, material="timber"), -0.4, 0.4))
        out.append(X(R("pane", H, "glazing", 3.5, 0.06, -0.012, 1.08, 0.024, tone=0.9, region="pane-glass", material="glass"), -0.34, 0.34))
        return out
    out += [X(R("stile-left", H, "body", 3, 0, -t, 1.2, 2 * t, tone=0.6, region=TM, material="timber"), 0.355, 0.4),
            X(R("stile-right", H, "body", 3, 0, -t, 1.2, 2 * t, tone=0.6, region=TM, material="timber"), -0.4, -0.355),
            X(R("rail-bottom", H, "body", 3, 0, -t, 0.06, 2 * t, tone=0.6, region=TM, material="timber"), -0.355, 0.355),
            X(R("rail-top", H, "body", 3, 1.14, -t, 0.06, 2 * t, tone=0.6, region=TM, material="timber"), -0.355, 0.355),
            X(R("pane", H, "glazing", 3.5, 0.06, -0.012, 1.08, 0.024, tone=0.9, region="pane-glass", material="glass"), -0.355, 0.355)]
    if sc == 50:
        out.append(X(R("pane-spacer", H, "glazing", 4, 0.072, -0.012, 1.056, 0.024, tone=0.5, region="pane-edge", material="edge-zone", filled=False), -0.343, 0.343))
    return out


def front(sc):
    s = [R("sill", S, "body", 1, -0.45, 0, 0.9, 0.05, tone=0.6, region="frame-sill", material="timber"),
         R("jamb-left", S, "body", 2, 0.4, 0.05, 0.05, 1.2, tone=0.6, region=TM, material="timber"),
         R("jamb-right", S, "body", 2, -0.45, 0.05, 0.05, 1.2, tone=0.6, region=TM, material="timber")]
    s += sash(sc, "front")
    if sc in (100, 50):
        s.append(P("glass-reflection-a", H, "texture", 3.6, [(-0.2, 0.3), (0.1, 0.9)], closed=False, filled=False, tone=0.3, region="pane-glass", follow=True))
    if sc == 50:
        s += [P("glass-reflection-b", H, "texture", 3.6, [(-0.1, 0.25), (0.2, 0.85)], closed=False, filled=False, tone=0.3, region="pane-glass", follow=True),
              line("sill-drip", S, "detail", 1.5, [(-0.45, 0.02), (0.45, 0.02)], region="frame-sill"),
              E("hinge-left", H, "detail", 5, -0.3, 0.07, 0.015, tone=0.3, region=TM, filled=True),
              E("hinge-right", H, "detail", 5, 0.3, 0.07, 0.015, tone=0.3, region=TM, filled=True)]
    return s


def plan(sc):
    s = [R("sill", S, "body", 1, -0.15, -0.45, 0.3, 0.9, tone=0.6, region="frame-sill", material="timber"),
         R("jamb-left", S, "body", 2, -0.15, 0.4, 0.3, 0.05, tone=0.6, region=TM, material="timber"),
         R("jamb-right", S, "body", 2, -0.15, -0.45, 0.3, 0.05, tone=0.6, region=TM, material="timber")]
    s += sash(sc, "plan")
    if sc == 50:
        s += [line("sill-drip", S, "detail", 1.5, [(0.13, -0.4), (0.13, 0.4)], region="frame-sill"),
              E("hinge-left", S, "detail", 5, 0.0, 0.3, 0.015, tone=0.3, region=TM, filled=True),
              E("hinge-right", S, "detail", 5, 0.0, -0.3, 0.015, tone=0.3, region=TM, filled=True)]
    return s


EN = {
    200: ("Front and plan: the front is the outer face, sill and jambs fixed, the sash as one frame with the pane, both extruded from the elevation profile of the sash, so the sash shortens as it tilts. The plan is the top view of the sill and the two jambs with the sash extruded from its elevation profile, so it sweeps inward over the interior side as a plate.",
          "Vorder- und Draufsicht: Die Vorderansicht ist die Außenseite, Sohlstück und Pfosten fest, der Flügel als ein Rahmen mit der Scheibe, beide aus dem Aufrissprofil des Flügels extrudiert, sodass der Flügel beim Kippen kürzer wird. Die Draufsicht ist die Sicht von oben auf Sohlstück und die beiden Pfosten, der Flügel wird aus seinem Aufrissprofil extrudiert und schwenkt als Platte nach innen über die Raumseite."),
    100: ("Front and plan: the front splits the sash into stiles, rails and the pane (all extruded from the sash profile) with one reflection line. The plan has the two timber stiles, the two rails and the glass pane as extruded profiles, which show the closed sash as a thin bar and the tilted sash as a wide plate over the interior side.",
          "Vorder- und Draufsicht: Die Vorderansicht teilt den Flügel in Pfosten, Riegel und Scheibe (alle aus dem Flügelprofil extrudiert) mit einer Spiegellinie. Die Draufsicht hat die zwei Holzpfosten, die zwei Riegel und die Glasscheibe als extrudierte Profile; sie zeigen den geschlossenen Flügel als dünnen Balken und den gekippten Flügel als breite Platte über der Raumseite."),
    50: ("Front and plan: the front is the outer face with the stiles, rails, pane, spacer frame, two reflection lines, the sill drip edge and both hinges; the handle sits on the interior face and stays hidden. The plan shows the sill, both jambs, the two stiles, both rails and the glass pane of the sash as extruded profiles, both hinges and the drip edge, so the top rail and the stiles sweep as in the elevation.",
         "Vorder- und Draufsicht: Die Vorderansicht ist die Außenseite mit Pfosten, Riegeln, Scheibe, Abstandhalterrahmen, zwei Spiegellinien, der Tropfkante des Sohlstücks und beiden Bändern; der Griff sitzt auf der Raumseite und bleibt verdeckt. Die Draufsicht zeigt Sohlstück, beide Pfosten, die zwei Flügelpfosten, beide Riegel und die Glasscheibe des Flügels als extrudierte Profile, beide Bänder und die Tropfkante, sodass Oberriegel und Pfosten wie im Aufriss ausschwenken."),
}
for sc in (200, 100, 50):
    install(A, sc, front(sc), plan(sc), doc, *EN[sc])
finish(A, doc)
print("ok", A)
