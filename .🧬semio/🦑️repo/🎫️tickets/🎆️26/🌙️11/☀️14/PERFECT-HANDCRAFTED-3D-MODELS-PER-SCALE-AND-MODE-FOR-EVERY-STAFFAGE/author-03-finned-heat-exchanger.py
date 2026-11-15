import importlib.util, os, math
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib", os.path.join(here, "author-03-lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
V = lib.Volume("finned-heat-exchanger")
AL = dict(material="aluminium-fin", role="body")
CU = dict(material="copper", role="body")


def circle(cx, cy, r, n=14):
    return [[cx + r * math.cos(2 * math.pi * i / n), cy + r * math.sin(2 * math.pi * i / n)] for i in range(n)]


def holes(r, n):
    return [circle(-0.022, 0.04 + 0.04 * i, r, n) for i in range(12)] + [circle(0.023, 0.06 + 0.04 * i, r, n) for i in range(11)]


def rect(x0, y0, x1, y1):
    return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]


def common(scale, tube_r, hdr_h, stub_r, stub_len_to):
    V.box("header-bottom", "frame", [0, hdr_h / 2, 0], [0.15, hdr_h, 0.5], cr=0.002 * scale / 50, tone=0.4, **CU)
    V.box("header-top", "frame", [0, 0.52 - hdr_h / 2, 0], [0.15, hdr_h, 0.5], cr=0.002 * scale / 50, tone=0.4, **CU)
    V.cyl("inlet-stub", "frame", [0.065, 0.51 if scale == 50 else 0.51, 0.2], [stub_len_to, 0.51, 0.2], stub_r, tone=0.4, **CU)
    V.cyl("outlet-stub", "frame", [0.065, 0.01, -0.2], [stub_len_to, 0.01, -0.2], stub_r, tone=0.4, **CU)


def s50():
    V.box("fin-pack-side", "fins", [0.0462, 0.26, 0], [0.0077, 0.48, 0.46], tone=0.7, rep=dict(count=2, step=[-0.0924, 0, 0]), **AL)
    for sgn, nm in [(1, "end-plate-left"), (-1, "end-plate-right")]:
        V.ext(nm, "fins", "side", rect(-0.05, 0.02, 0.05, 0.5), 0.23 if sgn > 0 else -0.25, 0.25 if sgn > 0 else -0.23, holes=holes(0.0075, 10), tone=0.6, **AL)
    V.cyl("tube-a", "tubes", [-0.022, 0.04, -0.253], [-0.022, 0.04, 0.253], 0.009, tone=0.5, rep=dict(count=12, step=[0, 0.04, 0]), **CU)
    V.cyl("tube-b", "tubes", [0.023, 0.06, -0.253], [0.023, 0.06, 0.253], 0.009, tone=0.5, rep=dict(count=11, step=[0, 0.04, 0]), **CU)
    common(50, 0.009, 0.02, 0.006, 0.13)
    V.to("thermal")
    V.ext("th-fin", "fins", "side", rect(-0.05, 0.02, 0.05, 0.5), -0.2225, -0.215, holes=holes(0.009, 12), tone=0.7, rep=dict(count=20, step=[0, 0, 0.0232]), region="fins", **AL)
    for s in ["tube-a", "tube-b"]:
        V.ref("th-" + s, s, "tubes")
    for s in ["header-bottom", "header-top", "inlet-stub", "outlet-stub"]:
        V.ref("th-" + s, s, "headers")
    V.to("structure")
    for s, m in [("fin-pack-side", "fins"), ("end-plate-left", "fins"), ("end-plate-right", "fins"), ("tube-a", "tubes"), ("tube-b", "tubes"), ("header-bottom", "frame"), ("header-top", "frame"), ("inlet-stub", "frame"), ("outlet-stub", "frame")]:
        V.lists["structure"].append({"id": "member-" + s, "of": s, "member": m, "part": "member"})
    V.box("joint-fins-frame", "fins", [0, 0.0275, 0], [0.09, 0.015, 0.4], tone=0.5, member="fins", part="joint", **AL)
    V.box("joint-tubes-fins", "tubes", [0, 0.4875, 0], [0.09, 0.015, 0.4], tone=0.5, member="tubes", part="joint", **CU)
    V.box("support-base-contact", "frame", [0, 0.00375, 0], [0.15, 0.0075, 0.4], tone=0.3, member="frame", part="support", anchor="base-contact", **CU)
    V.ell("mass-frame", "frame", [0, 0.26, 0], [0.03, 0.12, 0.15], tone=0.2, member="frame", part="mass", **CU)
    V.ell("mass-fins", "fins", [0, 0.24, 0], [0.04, 0.15, 0.18], tone=0.2, member="fins", part="mass", **AL)
    V.ell("mass-tubes", "tubes", [0, 0.24, 0], [0.02, 0.1, 0.12], tone=0.2, member="tubes", part="mass", **CU)
    V.decide("The fin pack is an open box of two 7.7 mm side plates between two perforated end plates because the real 0.2 mm fins at 2.5 mm pitch are far below the 7.5 mm limit; the 23 tubes are real 18 mm copper cylinders in two staggered columns (12 and 11, 40 mm pitch) that run through the pack and stand 3 mm proud of the end plates; headers are 20 mm slabs and the stubs 12 mm pipes at the two ends of the diagonal.",
             "Das Lamellenpaket ist ein offener Kasten aus zwei 7,7 mm dicken Seitenplatten zwischen zwei gelochten Endplatten, weil die echten 0,2 mm dicken Lamellen im Abstand von 2,5 mm weit unter der Grenze von 7,5 mm liegen; die 23 Rohre sind echte Kupferzylinder von 18 mm in zwei versetzten Reihen (12 und 11, Teilung 40 mm), die durch das Paket laufen und 3 mm aus den Endplatten ragen; die Sammler sind 20 mm dicke Platten und die Stutzen 12-mm-Rohre an den Enden der Diagonale.")
    V.decide("Thermal: the fins are modelled as 20 plates of 7.5 mm at 23 mm pitch with the 23 tube holes inside the block, because the block itself has too little surface for the fin region; tubes and headers reference their body solids. The authored areas of tubes (0.08 m2) and headers (0.012 m2) were corrected to the surfaces of the solids (0.6 m2 and 0.36 m2).",
             "Thermik: Die Lamellen sind 20 Platten von 7,5 mm im Abstand von 23 mm mit den 23 Rohrlöchern im Block, weil der Block selbst zu wenig Oberfläche für die Lamellenregion hat; Rohre und Sammler verweisen auf ihre Körper. Die angegebenen Flächen der Rohre (0,08 m2) und Sammler (0,012 m2) wurden auf die Flächen der Körper korrigiert (0,6 m2 und 0,36 m2).")
    V.decide("Structure: the load path runs from the base contact (0.15 m by 0.4 m) through the bottom header into the fin pack and its end plates, with the tubes brazed in; members reference the body solids, the joints are the brazed bands between header and pack, and the three masses sit at the centre of the pack.",
             "Tragwerk: Der Lastpfad läuft vom Fußkontakt (0,15 m mal 0,4 m) über den unteren Sammler in das Lamellenpaket mit den Endplatten, die Rohre sind eingelötet; die Stäbe verweisen auf die Körper, die Gelenke sind die Lötbänder zwischen Sammler und Paket, und die drei Massen sitzen in der Mitte des Pakets.")
    V.finish(50)


def s100():
    V.box("fin-pack-side", "fins", [0.0422, 0.26, 0], [0.0155, 0.48, 0.5], tone=0.7, rep=dict(count=2, step=[-0.0844, 0, 0]), **AL)
    V.box("fin-pack-end", "fins", [0, 0.26, 0.2422], [0.1, 0.48, 0.0155], tone=0.7, rep=dict(count=2, step=[0, 0, -0.4844]), **AL)
    V.cyl("tube-a", "tubes", [-0.022, 0.04, -0.25], [-0.022, 0.04, 0.25], 0.011, tone=0.5, rep=dict(count=12, step=[0, 0.04, 0]), **CU)
    V.cyl("tube-b", "tubes", [0.023, 0.06, -0.25], [0.023, 0.06, 0.25], 0.011, tone=0.5, rep=dict(count=11, step=[0, 0.04, 0]), **CU)
    common(100, 0.011, 0.025, 0.01, 0.13)
    V.to("thermal")
    V.ext("th-fin", "fins", "side", rect(-0.05, 0.02, 0.05, 0.5), -0.2225, -0.2075, holes=holes(0.011, 10), tone=0.7, rep=dict(count=20, step=[0, 0, 0.0232]), region="fins", **AL)
    for s in ["tube-a", "tube-b"]:
        V.ref("th-" + s, s, "tubes")
    for s in ["header-bottom", "header-top", "inlet-stub", "outlet-stub"]:
        V.ref("th-" + s, s, "headers")
    V.to("structure")
    for s, m in [("fin-pack-side", "fins"), ("tube-a", "tubes"), ("tube-b", "tubes"), ("header-bottom", "frame"), ("header-top", "frame"), ("inlet-stub", "frame"), ("outlet-stub", "frame")]:
        V.lists["structure"].append({"id": "member-" + s, "of": s, "member": m, "part": "member"})
    V.box("joint-fins-frame", "fins", [0, 0.035, 0], [0.09, 0.03, 0.4], tone=0.5, member="fins", part="joint", **AL)
    V.box("support-base-contact", "frame", [0, 0.0075, 0], [0.15, 0.015, 0.4], tone=0.3, member="frame", part="support", anchor="base-contact", **CU)
    V.ell("mass-pack", "fins", [0, 0.26, 0], [0.04, 0.15, 0.18], tone=0.2, member="fins", part="mass", **AL)
    V.decide("At 1:100 the end plates join the fin block (the drawing does the same), the tubes are 22 mm cylinders at the same staggered 40 mm pitch and 15 mm slabs are the headers; the stubs widen to 20 mm.",
             "Bei 1:100 gehen die Endplatten im Lamellenblock auf (wie in der Zeichnung), die Rohre sind 22-mm-Zylinder im gleichen versetzten Abstand von 40 mm, die Sammler 25 mm dicke Platten und die Stutzen 20 mm dick.")
    V.decide("Thermal: the same 20 fin plates (now 15 mm thick, 23 mm pitch, holes of 22 mm) carry the fin region; tubes and headers reference their solids.",
             "Thermik: Dieselben 20 Lamellenplatten (jetzt 15 mm dick, Teilung 23 mm, Löcher von 22 mm) tragen die Lamellenregion; Rohre und Sammler verweisen auf ihre Körper.")
    V.decide("Structure: pack, tubes and headers as members, one brazed band as joint, the contact pad (0.15 m by 0.4 m) and one mass for the whole pack.",
             "Tragwerk: Paket, Rohre und Sammler als Stäbe, ein Lötband als Gelenk, die Kontaktfläche (0,15 m mal 0,4 m) und eine Masse für das ganze Paket.")
    V.finish(100)


def s200():
    V.box("fin-pack-side", "fins", [0.0345, 0.26, 0], [0.031, 0.46, 0.5], tone=0.6, rep=dict(count=2, step=[-0.069, 0, 0]), **AL)
    V.box("fin-pack-end", "fins", [0, 0.26, 0.2345], [0.1, 0.46, 0.031], tone=0.6, rep=dict(count=2, step=[0, 0, -0.469]), **AL)
    V.box("header-bottom", "frame", [0, 0.02, 0], [0.15, 0.04, 0.5], tone=0.4, **CU)
    V.box("header-top", "frame", [0, 0.5, 0], [0.15, 0.04, 0.5], tone=0.4, **CU)
    V.cyl("inlet-stub", "frame", [0.065, 0.505, 0.2], [0.135, 0.505, 0.2], 0.015, tone=0.4, **CU)
    V.cyl("outlet-stub", "frame", [0.065, 0.015, -0.2], [0.135, 0.015, -0.2], 0.015, tone=0.4, **CU)
    V.to("thermal")
    V.box("th-fin", "fins", [0, 0.26, -0.21], [0.1, 0.46, 0.031], tone=0.6, rep=dict(count=8, step=[0, 0, 0.06]), region="fins", **AL)
    V.box("th-tubes", "tubes", [-0.022, 0.26, 0], [0.03, 0.4, 0.4], tone=0.5, region="tubes", **CU)
    V.box("th-tubes-b", "tubes", [0.023, 0.26, 0], [0.03, 0.4, 0.4], tone=0.5, region="tubes", **CU)
    for s in ["header-bottom", "header-top", "inlet-stub", "outlet-stub"]:
        V.ref("th-" + s, s, "headers")
    V.to("structure")
    for s, m in [("fin-pack-side", "fins"), ("header-bottom", "frame"), ("header-top", "frame"), ("inlet-stub", "frame"), ("outlet-stub", "frame")]:
        V.lists["structure"].append({"id": "member-" + s, "of": s, "member": m, "part": "member"})
    V.box("member-tube-bank", "tubes", [0, 0.26, 0], [0.06, 0.44, 0.46], tone=0.5, member="tubes", part="member", **CU)
    V.box("support-base-contact", "frame", [0, 0.015, 0], [0.15, 0.03, 0.4], tone=0.3, member="frame", part="support", anchor="base-contact", **CU)
    V.ell("mass-pack", "fins", [0, 0.26, 0], [0.04, 0.15, 0.18], tone=0.2, member="fins", part="mass", **AL)
    V.decide("At 1:200 the coil is one block between two 40 mm headers with two 30 mm stubs; fins, end plates and the 23 tubes are one mass (the drawing merges regions fins and tubes into it).",
             "Bei 1:200 ist das Register ein Block zwischen zwei 40 mm dicken Sammlern mit zwei 30-mm-Stutzen; Lamellen, Endplatten und die 23 Rohre sind eine Masse (die Zeichnung fasst die Regionen Lamellen und Rohre darin zusammen).")
    V.decide("Thermal: eight 31 mm plates at 60 mm pitch stand for the fins and two 30 mm tube-bank slabs for the two tube columns, all inside the block; headers reference their solids.",
             "Thermik: Acht 31-mm-Platten im Abstand von 60 mm stehen für die Lamellen und zwei 30-mm-Rohrbänke für die beiden Rohrreihen, alle im Block; die Sammler verweisen auf ihre Körper.")
    V.decide("Structure: block and headers as members, the tube bank as one 60 mm column inside the block, the contact pad and one mass.",
             "Tragwerk: Block und Sammler als Stäbe, die Rohrbank als eine 60 mm breite Säule im Block, die Kontaktfläche und eine Masse.")
    V.finish(200)


s50(); s100(); s200()
V.unavailable(500)
V.write()
