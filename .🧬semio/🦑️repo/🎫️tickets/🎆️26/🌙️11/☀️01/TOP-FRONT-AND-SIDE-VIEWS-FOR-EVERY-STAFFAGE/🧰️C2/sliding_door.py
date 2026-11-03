from lib import *

A = "sliding-door"
doc = load(A)
T, L = "track", "leaf"
TS, JS = "track-steel", "jamb-steel"


def front(sc):
    s = []
    if sc == 500:
        return [R("portal", T, "body", 1, -0.05, 0, 0.1, 2.15, tone=0.45, regions=[TS, JS], material="steel")]
    s.append(R("jamb-east", T, "body", 2, -0.05, 0, 0.1, 2.05, tone=0.45, region=JS, material="steel"))
    s.append(R("track", T, "body", 1, -0.05, 2.05, 0.1, 0.1, tone=0.45, region=TS, material="steel"))
    if sc == 200:
        return s
    s.append(R("leaf-behind", L, "detail", 3, -0.025, 0.1, 0.05, 1.95, tone=0.3, region="leaf-frame", filled=False))
    if sc == 100:
        return s
    s.append(R("hanger-behind", L, "detail", 3, -0.025, 1.92, 0.05, 0.13, tone=0.3, region="leaf-frame", filled=False))
    s.append(line("leaf-glass-behind", L, "detail", 3, [(0, 0.15), (0, 1.95)], region="leaf-glass"))
    s.append(line("jamb-seal", T, "detail", 2.5, [(-0.035, 0), (-0.035, 2.05)], region=JS))
    s.append(line("jamb-seal-b", T, "detail", 2.5, [(0.035, 0), (0.035, 2.05)], region=JS))
    s.append(R("bottom-seal-behind", L, "detail", 3, -0.025, 0.08, 0.05, 0.02, tone=0.3, region="leaf-frame", filled=False))
    return s


def plan(sc):
    if sc == 500:
        return [R("portal", T, "body", 1, -0.05, -0.05, 3.1, 0.1, tone=0.45, regions=[TS, JS], material="steel"),
                R("leaf", L, "body", 2, 1.0, -0.025, 1.0, 0.05, tone=0.8, regions=["leaf-glass", "leaf-frame"], material="glass")]
    s = [R("jamb-west", T, "body", 1, -0.05, -0.05, 0.1, 0.1, tone=0.45, region=JS, material="steel"),
         R("jamb-east", T, "body", 1, 2.95, -0.05, 0.1, 0.1, tone=0.45, region=JS, material="steel"),
         P("track-above", T, "detail" if sc != 200 else "detail", 0.5, [(-0.05, -0.05), (3.05, -0.05), (3.05, 0.05), (-0.05, 0.05)], tone=0.3, region=TS, filled=False)]
    if sc == 200:
        s.append(R("leaf-frame", L, "body", 2, 1.0, -0.025, 1.0, 0.05, tone=0.5, region="leaf-frame", material="steel"))
        s.append(R("leaf-glass", L, "glazing", 3, 1.05, -0.01, 0.9, 0.02, tone=0.92, region="leaf-glass", material="glass"))
        return s
    if sc == 100:
        s.append(R("leaf-frame", L, "body", 2, 1.0, -0.025, 1.0, 0.05, tone=0.5, region="leaf-frame", material="steel"))
        s.append(R("leaf-glass", L, "glazing", 3, 1.05, -0.005, 0.9, 0.01, tone=0.92, region="leaf-glass", material="glass"))
        return s
    s.append(R("leaf-stile-west", L, "body", 2, 1.0, -0.025, 0.05, 0.05, tone=0.5, region="leaf-frame", material="steel"))
    s.append(R("leaf-stile-east", L, "body", 2, 1.95, -0.025, 0.05, 0.05, tone=0.5, region="leaf-frame", material="steel"))
    s.append(R("leaf-glass", L, "glazing", 3, 1.05, -0.005, 0.9, 0.01, tone=0.92, region="leaf-glass", material="glass"))
    s.append(line("jamb-west-seal", T, "detail", 1.5, [(0.035, -0.05), (0.035, 0.05)], region=JS))
    s.append(line("jamb-east-seal", T, "detail", 1.5, [(2.965, -0.05), (2.965, 0.05)], region=JS))
    s.append(E("hanger-above-a", L, "detail", 2.5, 1.075, 0.0, 0.012, tone=0.1, region="leaf-frame", filled=False))
    s.append(E("hanger-above-b", L, "detail", 2.5, 1.925, 0.0, 0.012, tone=0.1, region="leaf-frame", filled=False))
    return s


EN = {
    500: ("Front and plan: the front is the end view along the track, jamb and track as one outline (the leaf hides behind the jamb); the plan is the portal strip with the leaf as a lighter bar that slides along it.",
          "Vorder- und Draufsicht: Die Vorderansicht ist die Ansicht entlang der Schiene, Pfosten und Schiene in einem Umriss (der Flügel verschwindet hinter dem Pfosten); die Draufsicht ist der Portalstreifen mit dem Flügel als hellerem Balken, der darin entlangfährt."),
    200: ("Front and plan: the front is the end view along the track (jamb and track head, the leaf is hidden behind the jamb). The plan is the horizontal section at 1.0 m: two filled jamb squares and the leaf as one bar, the track above the cut as a thin outline; the leaf slides along the track in every pose.",
          "Vorder- und Draufsicht: Die Vorderansicht ist die Ansicht entlang der Schiene (Pfosten und Schienenkopf, der Flügel ist hinter dem Pfosten verdeckt). Die Draufsicht ist der Horizontalschnitt in 1,0 m Höhe: zwei gefüllte Pfostenquadrate und der Flügel als ein Balken, die Schiene über der Schnittebene als dünne Umrisslinie; der Flügel fährt in jeder Pose entlang der Schiene."),
    100: ("Front and plan: the front adds the leaf frame behind the jamb as a thin hidden outline. The plan is the horizontal section at 1.0 m, the glass 10 mm as a thin pane between the two stiles drawn as one frame bar; the jamb squares are filled, the track above the cut is a thin outline.",
          "Vorder- und Draufsicht: Die Vorderansicht ergänzt den Flügelrahmen hinter dem Pfosten als dünne verdeckte Umrisslinie. Die Draufsicht ist der Horizontalschnitt in 1,0 m Höhe, das 10-mm-Glas als dünne Scheibe zwischen den beiden Pfosten des Flügels, als ein Rahmenbalken gezeichnet; die Pfostenquadrate sind gefüllt, die Schiene über der Schnittebene ist eine dünne Umrisslinie."),
    50: ("Front and plan: the front is the end view along the track with the jamb seals; the leaf frame, hanger, glass line and bottom seal behind the jamb are thin hidden outlines. The plan is the horizontal section at 1.0 m: the cut jamb squares, the two leaf stiles (50 x 50 mm) with the 10 mm glass between them, jamb seals and the two hangers and the track above the cut as thin outlines.",
         "Vorder- und Draufsicht: Die Vorderansicht ist die Ansicht entlang der Schiene mit den Pfostendichtungen; Flügelrahmen, Aufhängung, Glaslinie und Bodendichtung hinter dem Pfosten sind dünne verdeckte Umrisslinien. Die Draufsicht ist der Horizontalschnitt in 1,0 m Höhe: die geschnittenen Pfostenquadrate, die beiden Flügelpfosten (50 x 50 mm) mit dem 10-mm-Glas dazwischen, Pfostendichtungen sowie die beiden Aufhängungen und die Schiene über der Schnittebene als dünne Umrisslinien."),
}
for sc in (500, 200, 100, 50):
    install(A, sc, front(sc), plan(sc), doc, *EN[sc])
finish(A, doc)
print("ok", A)
