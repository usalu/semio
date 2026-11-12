import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "..", "..", "authoring-machinery-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("scissor-lift")
CP = dict(material="chassis-paint", role="body")
SS = dict(material="scissor-steel", role="body")
DS = dict(material="deck-steel", role="body")
RS = dict(material="rail-steel", role="detail")
RB = dict(material="rubber", role="body")
rect = lambda a0, a1, b0, b1: [(a0, b0), (a1, b0), (a1, b1), (a0, b1)]

V.ext("chassis-body", "chassis", "side", [(-0.925, 0.14), (-0.9, 0.1), (0.9, 0.1), (0.925, 0.14), (0.925, 0.45), (-0.925, 0.45)], -0.37, 0.37, tone=0.45, region="chassis", **CP)
V.box("battery-cover", "chassis", [-0.395, 0.285, 0.376], [0.45, 0.23, 0.012], cr=0.004, tone=0.3, region="battery-box", role="detail", material="chassis-paint", rep=dict(count=2, step=[0, 0, -0.752]))
V.box("pump-cover", "chassis", [0.12, 0.285, 0.376], [0.4, 0.23, 0.012], cr=0.004, tone=0.35, region="motor-pump", role="detail", material="chassis-paint", rep=dict(count=2, step=[0, 0, -0.752]))
for bone, cx in [("wheel-f", 0.55), ("wheel-r", -0.55)]:
    tag = bone[-1]
    for side, sg in [("l", 1), ("r", -1)]:
        prof = [[sg * a, r] for a, r in [(-0.08, 0), (-0.08, 0.2), (-0.07, 0.255), (-0.04, 0.27), (0.04, 0.27), (0.07, 0.255), (0.08, 0.2), (0.08, 0.15), (0.072, 0.14), (0.072, 0)]]
        V.rev(f"tyre-{tag}-{side}", bone, [cx, 0.27, sg * 0.325], [0, 0, 1], prof, tone=0.12, region="tyres", **RB)
        V.cyl(f"wheel-bolt-{tag}-{side}", bone, [cx + 0.065, 0.27, sg * (0.325 + 0.062)], [cx + 0.065, 0.27, sg * (0.325 + 0.078)], 0.011, tone=0.5, region="tyres", maxDenominator=100, material="scissor-steel", role="detail",
              rep=dict(count=5, angle=2 * math.pi / 5, axis=[0, 0, 1], origin=[cx, 0.27, 0]))
    V.cyl(f"axle-{tag}", bone, [cx, 0.27, -0.325], [cx, 0.27, 0.325], 0.04, tone=0.4, region="tyres", material="scissor-steel", role="detail")
V.cyl("slide-roller", "slide", [0.846, 0.45, 0.215], [0.846, 0.45, 0.375], 0.04, tone=0.3, region="arms", material="scissor-steel", role="detail", rep=dict(count=2, step=[0, 0, -0.59]))
for i in range(1, 6):
    for kind in "AB":
        bone = f"{kind}{i}"
        z = 0.34 if kind == "A" else 0.25
        V.box(f"arm-{kind.lower()}{i}", bone, V.W(bone, [0.85, 0, z]), [1.84, 0.12, 0.06], rz=V.angle(bone), cr=0.012, tone=0.4 if kind == "A" else 0.5, region="arms", rep=dict(count=2, step=[0, 0, -2 * z]), **SS)
    a = f"a{i}"
    V.cyl(f"pin-mid-{i}", a, V.W(a, [0.85, 0, 0.215]), V.W(a, [0.85, 0, 0.375]), 0.03, tone=0.1, region="arms", material="scissor-steel", role="detail", rep=dict(count=2, step=[0, 0, -0.59]))
    V.cyl(f"pin-a{i}", a, V.W(a, [0, 0, 0.215]), V.W(a, [0, 0, 0.375]), 0.03, tone=0.1, region="arms", material="scissor-steel", role="detail", rep=dict(count=2, step=[0, 0, -0.59]))
    b = f"b{i}"
    V.cyl(f"pin-b{i}", b, V.W(b, [0, 0, 0.215]), V.W(b, [0, 0, 0.375]), 0.03, tone=0.1, region="arms", material="scissor-steel", role="detail", rep=dict(count=2, step=[0, 0, -0.59]))
V.cyl("roller-top", "a5", V.W("a5", [1.7, 0, 0.215]), V.W("a5", [1.7, 0, 0.375]), 0.03, tone=0.1, region="arms", material="scissor-steel", role="detail", rep=dict(count=2, step=[0, 0, -0.59]))
V.cyl("pin-platform", "platform", [-0.85, 1.0, 0.215], [-0.85, 1.0, 0.375], 0.03, tone=0.1, region="deck", material="deck-steel", role="detail", rep=dict(count=2, step=[0, 0, -0.59]))
V.box("deck-bottom", "platform", [0, 1.005, 0], [1.85, 0.01, 0.74], tone=0.5, region="deck", **DS)
V.box("deck-top", "platform", [0, 1.095, 0], [1.85, 0.01, 0.74], tone=0.5, region="deck", **DS)
V.box("deck-wall", "platform", [0, 1.05, 0.36], [1.85, 0.08, 0.02], tone=0.5, region="deck", rep=dict(count=2, step=[0, 0, -0.72]), **DS)
V.box("deck-ext", "deck-ext", [-0.4, 1.05, 0], [0.9, 0.08, 0.7], tone=0.55, region="deck", **DS)
for lvl, y, md in [("top", 2.175, None), ("mid", 1.645, None), ("mid-2", 1.925, 100)]:
    m = {} if md is None else {"maxDenominator": md}
    V.box(f"rail-{lvl}-side", "platform", [0, y, 0.345], [1.85, 0.05, 0.05], tone=0.3, region="rails", rep=dict(count=2, step=[0, 0, -0.69]), **m, **RS)
    V.box(f"rail-{lvl}-end", "platform", [0.9, y, 0], [0.05, 0.05, 0.64], tone=0.3, region="rails", rep=dict(count=2, step=[-1.8, 0, 0]), **m, **RS)
V.box("post-corner", "platform", [0.9, 1.65, 0.345], [0.05, 1.1, 0.05], tone=0.3, region="rails", rep=dict(count=2, step=[-1.8, 0, 0]), **dict(RS, role="detail"))
V.box("post-corner-rear", "platform", [0.9, 1.65, -0.345], [0.05, 1.1, 0.05], tone=0.3, region="rails", rep=dict(count=2, step=[-1.8, 0, 0]), **RS)
V.box("post-mid", "platform", [0.025, 1.625, 0.345], [0.05, 1.05, 0.05], tone=0.3, region="rails", maxDenominator=50, rep=dict(count=2, step=[0, 0, -0.69]), **RS)
V.box("toe-board-side", "platform", [0, 1.175, 0.365], [1.85, 0.15, 0.01], tone=0.3, region="rails", rep=dict(count=2, step=[0, 0, -0.73]), **RS)
V.box("toe-board-end", "platform", [0.92, 1.175, 0], [0.01, 0.15, 0.72], tone=0.3, region="rails", rep=dict(count=2, step=[-1.84, 0, 0]), **RS)
V.box("control-box", "platform", [0.66, 1.875, -0.3575], [0.22, 0.25, 0.095], cr=0.008, tone=0.15, region="rails", maxDenominator=50, material="chassis-paint", role="detail")
V.decide("Every scissor arm is its own box (1.84 m by 0.12 m by 0.06 m, rounded) on the bone that drives it; A arms sit outside at z +-0.34, B arms inside at z +-0.25 as the front and plan views show, each as one solid with a mirror repeat. Pins (r 0.03 m, z 0.215 to 0.375 on each side, mirrored) at every arm origin and every crossing hold each side's links together at all five levels; there is no cross tube in the middle because the front view draws the scissor open between the two sides.",
         "Jeder Scherenarm ist ein eigener Kasten (1,84 m mal 0,12 m mal 0,06 m, abgerundet) auf dem Knochen, der ihn antreibt; A-Arme aussen bei z +-0,34, B-Arme innen bei z +-0,25 wie Vorder- und Grundriss zeigen, jeder ein Koerper mit Spiegelwiederholung. Bolzen (r 0,03 m, z 0,215 bis 0,375 je Seite, gespiegelt) an jedem Armursprung und jeder Kreuzung halten die Glieder jeder Seite ueber alle fuenf Ebenen zusammen; ein Querrohr in der Mitte gibt es nicht, weil die Vorderansicht die Schere zwischen beiden Seiten offen zeichnet.")
V.decide("Wheels are revolved tyres (profile with a dished outer face) with five hub bolts that make the rolling readable, on the wheel bones; tyres sit at z +-0.245 to +-0.405 outside the 0.74 m chassis body (front view), so the 0.81 m depth comes from the tyres. The plan draws the chassis 0.81 m wide; the front view 0.74 m wins.",
         "Raeder sind gedrehte Reifen (Profil mit eingezogener Aussenseite) mit fuenf Radbolzen, die das Rollen lesbar machen, auf den Radknochen; die Reifen sitzen bei z +-0,245 bis +-0,405 ausserhalb des 0,74 m breiten Fahrgestells (Vorderansicht), die Tiefe 0,81 m kommt von den Reifen. Der Grundriss zeichnet das Fahrgestell 0,81 m breit; die Vorderansicht mit 0,74 m gilt.")
V.decide("The deck is a hollow slot (bottom plate, top plate, two side walls) in which the 0.9 m extension plate on its own prismatic bone is stowed; guardrails are square tubes with posts, three rail levels, toe boards and the control box clamped on the -z side rail, all on the platform bone. The 2D extension moves only 0.9 m so it ends 25 mm beyond the deck front.",
         "Das Deck ist ein hohler Schacht (Bodenblech, Deckblech, zwei Seitenwaende), in dem die 0,9 m lange Verlaengerungsplatte auf eigenem Schubknochen verstaut ist; Gelaender sind Vierkantrohre mit Pfosten, drei Holmebenen, Fussleisten und dem an den -z-Holm geklemmten Steuerkasten, alles auf dem Knochen platform. Die 2D-Verlaengerung faehrt nur 0,9 m und ragt damit nur 25 mm ueber die Deckkante hinaus.")
V.write(os.path.join(here, "scissor-lift.json"))
