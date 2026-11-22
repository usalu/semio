"""📜️ Assembles the asset document of the reclaimed lattice girder from the geometry of author-girder.py: materials, rig, anchors, poses, clips, thermal and structural models, drawings and volumes."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
AREAS = os.path.join(HERE, "author-areas.json")


def document(g):
    """🏺️ The whole asset document."""
    T, L, X, Y0, m, texts = g["T"], g["L"], g["X"], g["Y0"], g["m"], g["texts"]
    r = lambda x: round(x, 4)
    total = round(sum(m.values()), 2)
    areas = json.load(open(AREAS)) if os.path.exists(AREAS) else {}

    materials = [
        {"id": "steel-painted", "title": T("Riveted mild steel St 37, weathered red-lead primer with grey top-coat remains", "Genieteter Flussstahl St 37, verwitterte Bleimennige mit Resten der grauen Deckschicht"), "emissivity": 0.9, "solarAbsorptivity": 0.72, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "din-17100"},
        {"id": "steel-bare", "title": T("Bare rusted steel of the bearing plates", "Blanker, angerosteter Stahl der Lagerplatten"), "emissivity": 0.85, "solarAbsorptivity": 0.8, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "ashrae-fundamentals"},
        {"id": "steel-cut", "title": T("Flame-cut steel, oxide skin of the cut faces", "Brenngeschnittener Stahl, Zunderhaut der Schnittflächen"), "emissivity": 0.8, "solarAbsorptivity": 0.85, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 460, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "authored-estimate", "reference": "ashrae-fundamentals"},
        {"id": "softwood-weathered", "title": T("Weathered squared spruce timber of the bearers and props", "Verwittertes Fichtenkantholz der Unterleghölzer und Streben"), "emissivity": 0.9, "solarAbsorptivity": 0.65, "densityKgM3": 450, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
        {"id": "polyester", "title": T("Violet polyester round sling, 1000 kg working load", "Violetter Polyester-Rundschlinge, 1000 kg Tragfähigkeit"), "emissivity": 0.9, "solarAbsorptivity": 0.6, "densityKgM3": 600, "conductivityWmK": 0.2, "specificHeatJKgK": 1300, "elasticModulusPa": 2000000000, "source": "authored-estimate", "reference": "en-1492-2"},
    ]

    I_eq = 2 * (2 * g["CH"]["A"]) * ((1.0 - 2 * g["CH"]["e"]) / 2) ** 2 + 4 * g["CH"]["I"]
    gm = m["girder"]
    bones = [
        {"id": "yard", "parent": None, "title": T("Yard floor of the component exchange", "Lagerplatz der Bauteilbörse"), "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0, "joint": {"type": "fixed"}},
        {"id": "girder", "parent": "yard", "title": T("Lattice girder", "Fachwerkträger"), "x": 0, "y": Y0, "angleDeg": 0, "lengthM": L, "joint": {"type": "prismatic", "axisDeg": 90, "minM": g["SEAT_DY"], "maxM": 0.8},
         "mass": {"kg": round(gm, 2), "comX": r(g["com"][0]), "comY": r(g["com"][1]), "distribution": "uniform", "source": "literature", "reference": "din-1028"},
         "section": {"areaM2": round(4 * g["CH"]["A"], 6), "secondMomentM4": round(I_eq, 7), "extremeFibreM": 0.5, "material": "steel-painted", "title": T("Parallel chords 2 L 80 x 80 x 8 each, 955 mm between chord axes", "Parallelgurte je 2 L 80 x 80 x 8, 955 mm zwischen den Gurtachsen")}},
    ]
    for n, x in zip(("a", "b"), g["BEARER_X"]):
        bones.append({"id": f"bearer-{n}", "parent": "yard", "title": T(f"Bearer {n.upper()} (squared timber 120 x 120)", f"Unterlegholz {n.upper()} (Kantholz 120 x 120)"), "x": r(x), "y": 0, "angleDeg": 0, "lengthM": 0.12,
                      "joint": {"type": "prismatic", "axisDeg": 0, "minM": -g["BEARER_DX"], "maxM": g["BEARER_DX"]},
                      "mass": {"kg": round(m[f"bearer-{n}"], 3), "comX": 0, "comY": 0.06, "distribution": "point", "source": "authored-estimate", "reference": "en-338"},
                      "section": {"areaM2": 0.0144, "secondMomentM4": 1.728e-05, "extremeFibreM": 0.06, "material": "softwood-weathered", "title": T("Squared timber 120 x 120 mm", "Kantholz 120 x 120 mm")}})
    for (name, side), x in zip(g["PROPS"], [g["PROP_X"][0], g["PROP_X"][0], g["PROP_X"][1], g["PROP_X"][1]]):
        o = g["ORIGIN"][name]
        prof, _ = g["prop_profile"](side)
        cv = sum(p[0] for p in prof) / 4
        sideword = ("left", "links") if side > 0 else ("right", "rechts")
        bones.append({"id": name, "parent": "yard", "title": T(f"Prop {name[5]} {sideword[0]} (squared timber 80 x 80)", f"Strebe {name[5]} {sideword[1]} (Kantholz 80 x 80)"), "x": r(o[0]), "y": 0, "z": r(o[2]), "angleDeg": 0, "lengthM": 0.08,
                      "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 0, "twistMinDeg": -125 if side > 0 else 0, "twistMaxDeg": 0 if side > 0 else 125},
                      "mass": {"kg": round(m[name], 3), "comX": 0, "comY": r(cv), "distribution": "point", "source": "authored-estimate", "reference": "en-338"},
                      "section": {"areaM2": 0.0064, "secondMomentM4": 3.413e-06, "extremeFibreM": 0.04, "material": "softwood-weathered", "title": T("Squared timber 80 x 80 mm", "Kantholz 80 x 80 mm")}})
    for n, u in zip(("a", "b"), g["SLING"]["u"]):
        bones.append({"id": f"sling-{n}", "parent": "girder", "title": T(f"Round sling {n.upper()}", f"Rundschlinge {n.upper()}"), "x": u, "y": r(1.0 + g["SLING"]["r"]), "z": g["SLING"]["z"], "angleDeg": 0, "lengthM": g["SLING"]["l"],
                      "joint": {"type": "revolute", "minDeg": 0, "maxDeg": 135},
                      "mass": {"kg": round(m[f"sling-{n}"], 3), "comX": 1.2, "comY": 0, "distribution": "uniform", "source": "authored-estimate", "reference": "en-1492-2"},
                      "section": {"areaM2": 0.000707, "secondMomentM4": 3.98e-08, "extremeFibreM": 0.015, "material": "polyester", "title": T("Round sling, 30 mm sleeve", "Rundschlinge, 30-mm-Schlauch")}})
    rig = {"kind": "structural", "root": "yard", "bones": bones}

    anchors = []
    for n in ("a", "b"):
        for s, word in ((1, ("left", "links")), (-1, ("right", "rechts"))):
            anchors.append({"id": f"bearer-{n}-ground-{word[0][0]}", "bone": f"bearer-{n}", "x": 0, "y": 0, "z": 0.55 * s, "kind": "contact", "title": T(f"Bearer {n.upper()} on the yard, {word[0]} end", f"Unterlegholz {n.upper()} auf dem Lagerplatz, Ende {word[1]}"), "support": "contact", "footprintLengthM": 0.12, "footprintWidthM": 0.1, "frictionCoefficient": 0.5})
    anchors += [
        {"id": "bearing-left", "bone": "girder", "x": X[0], "y": -g["PLATE"]["t"], "kind": "support", "title": T("Left bearing plate on its column head, fixed", "Linke Lagerplatte auf dem Stützenkopf, fest"), "support": "pin", "footprintLengthM": 0.24, "footprintWidthM": 0.26},
        {"id": "bearing-right", "bone": "girder", "x": X[10], "y": -g["PLATE"]["t"], "kind": "support", "title": T("Right bearing plate on its column head, sliding in the slotted holes", "Rechte Lagerplatte auf dem Stützenkopf, verschieblich in den Langlöchern"), "support": "roller", "normalDeg": 90, "footprintLengthM": 0.24, "footprintWidthM": 0.26},
        {"id": "hook-a", "bone": "sling-a", "x": g["SLING"]["l"], "y": 0, "kind": "support", "title": T("Crane hook, sling A", "Kranhaken, Schlinge A"), "support": "pin"},
        {"id": "hook-b", "bone": "sling-b", "x": g["SLING"]["l"], "y": 0, "kind": "support", "title": T("Crane hook, sling B", "Kranhaken, Schlinge B"), "support": "pin"},
    ]
    ground = [a["id"] for a in anchors if a["id"].startswith("bearer")]

    def bind(stored, sun):
        return {"regions": {
            "lattice-paint": {"exposedFraction": 1, "sunlitFraction": round(0.5 * sun, 3)},
            "bearing-steel": {"exposedFraction": 1, "sunlitFraction": round(0.2 * sun, 3)},
            "bearers": {"exposedFraction": 0.83 if stored else 0.83, "sunlitFraction": round(0.4 * sun, 3), "contact": {"fraction": 0.17, "tempC": 12}},
            "props": {"exposedFraction": 1 if stored else 0.75, "sunlitFraction": round(0.5 * sun, 3), **({} if stored else {"contact": {"fraction": 0.25, "tempC": 12}})},
            "slings": {"exposedFraction": 0.85 if stored else 1, "sunlitFraction": round(0.6 * sun, 3)},
        }, "emitters": {}}

    sa, sb = g["SLING_ANGLE"], 180 - g["SLING_ANGLE"]
    props_down = {name: {"twistDeg": -125 if side > 0 else 125} for name, side in g["PROPS"]}
    slings_up = {"sling-a": {"angleDeg": round(sa, 4)}, "sling-b": {"angleDeg": round(sb, 4)}}
    seat = {"girder": {"dy": g["SEAT_DY"]}, "bearer-a": {"dx": -g["BEARER_DX"]}, "bearer-b": {"dx": g["BEARER_DX"]}}
    cleats = g["CLEAT_NODES"] + [6]
    purlin = lambda k: 1.75 if k in (0, 10) else 3.5
    snow = lambda k: 3.4 if k in (0, 10) else 6.8
    purlin_nodes = [0, 2, 4, 6, 8, 10]
    dead = [{"id": f"purlin-dead-{k:02d}", "kind": "carried", "title": T(f"Roof and purlin at node {k}, {purlin(k)} kN", f"Dach und Pfette an Knoten {k}, {purlin(k)} kN".replace(".", ",")), "bone": "girder", "x": r(X[k]), "y": 1.08, "fxN": 0, "fyN": -purlin(k) * 1000, "source": "authored-estimate", "reference": "en-1991-1-1"} for k in purlin_nodes]
    snowl = [{"id": f"snow-{k:02d}", "kind": "snow", "title": T(f"Snow on the roof at node {k}, {snow(k)} kN", f"Schnee auf dem Dach an Knoten {k}, {snow(k)} kN".replace(".", ",")), "bone": "girder", "x": r(X[k]), "y": 1.08, "fxN": 0, "fyN": -snow(k) * 1000, "source": "literature", "reference": "en-1991-1-3"} for k in purlin_nodes]
    poses = [
        {"id": "stored", "title": T("Stored on edge in the yard", "Hochkant gelagert auf dem Platz"), "description": T("The girder stands upright on two squared-timber bearers under its bottom nodes 1 and 9, braced on both faces by four inclined props against the top chord flange; the two round slings stay choked round the top chord at the lifting points and are laid along it.", "Der Träger steht aufrecht auf zwei Kanthölzern unter seinen Untergurtknoten 1 und 9, auf beiden Seiten von vier schrägen Streben gegen den Obergurtflansch gesichert; die beiden Rundschlingen bleiben an den Anschlagpunkten um den Obergurt geschnürt und liegen auf ihm."), "bones": {}, "ground": "authored", "contacts": ground, "thermal": bind(True, 1), "loads": []},
        {"id": "slings-taut", "key": True, "title": T("Slings tensioned", "Schlingen gespannt"), "description": T("The crane hook has taken up the two slings at 51 degrees over the middle of the girder; the girder still stands on its bearers and props.", "Der Kranhaken hat die beiden Schlingen unter 51 Grad über der Trägermitte aufgenommen; der Träger steht noch auf Unterleghölzern und Streben."), "bones": dict(slings_up), "ground": "authored", "contacts": ground, "thermal": bind(True, 1), "loads": []},
        {"id": "props-cleared", "key": True, "title": T("Props laid down", "Streben abgelegt"), "description": T("With the slings taut the four props are tipped outwards about their outer foot edges and lie on the yard.", "Bei gespannten Schlingen sind die vier Streben um ihre äußere Fußkante nach außen gekippt und liegen auf dem Platz."), "bones": {**slings_up, **props_down}, "ground": "authored", "contacts": ground, "thermal": bind(False, 1), "loads": []},
        {"id": "lifted", "title": T("Lifted on two round slings", "An zwei Rundschlingen angehoben"), "description": T("The girder hangs 0.5 m clear of its bearers from the crane hook on two choked 1 t round slings at 51 degrees, attached to the top chord 1.5 m either side of midspan; bearers and props stay behind on the yard.", "Der Träger hängt 0,5 m über seinen Unterleghölzern am Kranhaken, an zwei geschnürten 1-t-Rundschlingen unter 51 Grad, angeschlagen am Obergurt 1,5 m beiderseits der Feldmitte; Unterleghölzer und Streben bleiben auf dem Platz zurück."), "bones": {**slings_up, **props_down, "girder": {"dy": g["LIFT_DY"]}}, "ground": "authored", "contacts": ["hook-a", "hook-b"], "thermal": bind(False, 1), "loads": []},
        {"id": "reinstalled", "title": T("Reinstalled under the roof", "Wieder eingebaut unter dem Dach"), "description": T("Back in its role as a roof girder of a 5 m hall bay: seated on its bearing plates on two column heads (fixed left, sliding right), carrying roof and purlins of 0.35 kN/m² at the six purlin nodes; bearers pulled out, props laid down.", "Zurück in seiner Rolle als Dachbinder eines 5-m-Hallenfeldes: auf seinen Lagerplatten auf zwei Stützenköpfen (links fest, rechts verschieblich), mit Dach und Pfetten von 0,35 kN/m² an den sechs Pfettenknoten; Unterleghölzer herausgezogen, Streben abgelegt."), "bones": {**props_down, **seat}, "ground": "authored", "contacts": ["bearing-left", "bearing-right"], "thermal": bind(False, 0.1), "loads": dead},
        {"id": "reinstalled-snow", "title": T("Reinstalled under roof and snow", "Wieder eingebaut unter Dach und Schnee"), "description": T("As reinstalled, with the roof snow load of 0.68 kN/m² (sk 0.85 kN/m², shape coefficient 0.8) added at the purlin nodes.", "Wie wieder eingebaut, zusätzlich mit der Schneelast auf dem Dach von 0,68 kN/m² (sk 0,85 kN/m², Formbeiwert 0,8) an den Pfettenknoten."), "bones": {**props_down, **seat}, "ground": "authored", "contacts": ["bearing-left", "bearing-right"], "thermal": bind(False, 0.1), "loads": dead + snowl},
    ]
    clips = [
        {"id": "lift", "title": T("Lifting out of the store", "Ausheben aus dem Lager"), "description": T("The crane tensions the two slings, the props are tipped down, then the girder rises 0.5 m off its bearers.", "Der Kran spannt die beiden Schlingen, die Streben werden abgelegt, dann hebt der Träger 0,5 m von seinen Unterleghölzern ab."),
         "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "linear"}, {"t": 2.5, "pose": "slings-taut", "ease": "linear"}, {"t": 4.5, "pose": "props-cleared", "ease": "linear"}, {"t": 7.5, "pose": "lifted", "ease": "linear"}, {"t": 8, "pose": "lifted", "ease": "linear"}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 4.7} for a in ground] + [{"anchor": "hook-a", "fromS": 2.5, "toS": 8}, {"anchor": "hook-b", "fromS": 2.5, "toS": 8}],
         "phases": [{"id": "rig", "fromS": 0, "toS": 2.5, "title": T("Slings taken up", "Schlingen aufnehmen")}, {"id": "clear", "fromS": 2.5, "toS": 4.5, "title": T("Props laid down", "Streben ablegen")}, {"id": "hoist", "fromS": 4.5, "toS": 7.5, "title": T("Hoist", "Heben")}, {"id": "hold", "fromS": 7.5, "toS": 8, "title": T("Hold", "Halten")}],
         "events": [{"t": 2.5, "kind": "contact-on", "title": T("Hook takes the load", "Haken nimmt die Last"), "target": "hook-a"}, {"t": 4.7, "kind": "contact-off", "title": T("Girder leaves its bearers", "Träger verlässt die Unterleghölzer"), "target": "bearer-a-ground-l"}], "tracks": []},
        {"id": "snow-load", "title": T("Snow settling on the roof", "Schnee legt sich aufs Dach"), "description": T("In the reinstalled girder the roof snow load grows from nothing to its characteristic value over four seconds and holds.", "Im wieder eingebauten Träger wächst die Schneelast auf dem Dach in vier Sekunden von null auf ihren charakteristischen Wert und bleibt."),
         "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-snow", "ease": "smooth"}, {"t": 6, "pose": "reinstalled-snow", "ease": "smooth"}],
         "contacts": [{"anchor": "bearing-left", "fromS": 0, "toS": 6}, {"anchor": "bearing-right", "fromS": 0, "toS": 6}],
         "phases": [{"id": "ramp", "fromS": 0, "toS": 4, "title": T("Snow builds up", "Schnee baut sich auf")}, {"id": "hold", "fromS": 4, "toS": 6, "title": T("Full snow load", "Volle Schneelast")}],
         "events": [{"t": 4, "kind": "load-on", "title": T("Snow load at its characteristic value", "Schneelast auf charakteristischem Wert")}],
         "tracks": [{"target": f"load:snow-{k:02d}", "ease": "smooth", "keys": [{"t": 0, "v": 0}, {"t": 4, "v": 1}, {"t": 6, "v": 1}]} for k in purlin_nodes]},
    ]

    hc_steel = round(gm * 460 / areas.get("lattice-paint", 24.0))
    regions = [
        {"id": "lattice-paint", "title": T("Painted lattice: chords, verticals, diagonals, gussets, rivets and cleats", "Gestrichenes Fachwerk: Gurte, Pfosten, Diagonalen, Knotenbleche, Niete und Knaggen"), "kind": "surface", "model": "passive", "material": "steel-painted", "areaM2": areas.get("lattice-paint", 24.0), "characteristicLengthM": 0.08, "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": hc_steel, "source": "authored-estimate", "reference": "en-iso-10456"},
        {"id": "bearing-steel", "title": T("Bare bearing plates", "Blanke Lagerplatten"), "kind": "surface", "model": "passive", "material": "steel-bare", "areaM2": areas.get("bearing-steel", 0.27), "characteristicLengthM": 0.24, "geometry": "horizontal-plate-down", "skyViewFactor": 0.1, "layers": [], "heatCapacityJM2K": round(0.016 * 7850 * 460 / 2), "source": "authored-estimate", "reference": "en-iso-10456"},
        {"id": "bearers", "title": T("Squared-timber bearers", "Kanthölzer der Unterlage"), "kind": "surface", "model": "passive", "material": "softwood-weathered", "areaM2": areas.get("bearers", 1.21), "characteristicLengthM": 0.12, "geometry": "horizontal-plate-up", "skyViewFactor": 0.6, "layers": [], "heatCapacityJM2K": 26000, "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "props", "title": T("Squared-timber props", "Kantholzstreben"), "kind": "surface", "model": "passive", "material": "softwood-weathered", "areaM2": areas.get("props", 1.8), "characteristicLengthM": 0.08, "geometry": "cylinder", "skyViewFactor": 0.6, "layers": [], "heatCapacityJM2K": 24000, "source": "authored-estimate", "reference": "en-iso-13786"},
        {"id": "slings", "title": T("Round slings", "Rundschlingen"), "kind": "surface", "model": "passive", "material": "polyester", "areaM2": areas.get("slings", 0.5), "characteristicLengthM": 0.03, "geometry": "cylinder", "skyViewFactor": 0.7, "layers": [], "heatCapacityJM2K": 6000, "source": "authored-estimate", "reference": "en-iso-10456"},
    ]
    thermal = {"environment": {"airTempC": 18, "meanRadiantTempC": 18, "skyTempC": 4, "groundTempC": 16, "relativeHumidity": 0.6, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 650, "diffuseFraction": 0.2, "pressurePa": 101325},
               "operating": [], "regions": regions, "emitters": [], "passengers": [], "assumptions": texts["thermal"]}
    structure = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": texts["structure"], "clinical": False}

    reps, vols = {}, {}
    bands = {50: (20, 75), 100: (75, 150), 200: (150, 350), 500: (350, 1000)}
    pens = {50: {"body": 0.25, "detail": 0.13, "texture": 0.1, "glazing": None, "ground": 0.35},
            100: {"body": 0.18, "detail": 0.1, "texture": 0.1, "glazing": None, "ground": 0.25},
            200: {"body": 0.13, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.18},
            500: {"body": 0.13, "detail": 0.1, "texture": None, "glazing": None, "ground": 0.18}}
    for s in (50, 100, 200, 500):
        parts = g["body"](s)
        E, F, P = g["drawings"](s, parts)
        reps[str(s)] = {"status": "authored", "denominator": s, "validRange": {"minDenominator": bands[s][0], "maxDenominator": bands[s][1]}, "pens": pens[s], "hatches": [], "geometry": {"elevation": E, "front": F, "plan": P}, "decisions": texts["representations"][str(s)]}
        vols[str(s)] = g["scale_volume"](s, parts)

    xs, ys, zs = [0.0, L], [0.0, Y0 + 1.08], []
    for name, side in g["PROPS"]:
        o = g["ORIGIN"][name]
        zs.append(o[2])
    depth = round(2 * max(abs(z) for z in zs), 4)
    asset = {
        "id": "reclaimed-lattice-girder", "category": "components",
        "title": T("Reclaimed Riveted Lattice Girder", "Gebrauchter genieteter Fachwerkträger"),
        "summary": texts["summary"],
        "tags": ["component", "reclaimed", "structural", "steel", "riveted", "truss", "girder", "lattice", "roof"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": texts["references"], "notes": texts["notes"]},
        "dimensions": {"heightM": r(ys[1]), "widthM": r(L), "depthM": depth, "massKg": total},
        "materials": materials, "rig": rig, "anchors": anchors, "representations": reps, "symbols": [], "poses": poses, "clips": clips, "thermal": thermal, "structure": structure, "volumes": vols,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
