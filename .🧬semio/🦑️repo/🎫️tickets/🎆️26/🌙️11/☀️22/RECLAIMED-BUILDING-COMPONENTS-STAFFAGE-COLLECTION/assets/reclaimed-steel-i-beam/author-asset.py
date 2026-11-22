"""🧾️ Rig, anchors, poses, clips, thermal and structural model, materials and provenance of `reclaimed-steel-i-beam`, assembled around the drawings and volumes of the main author script.

@see ./author-reclaimed-steel-i-beam.py — the entry point that calls `document`
"""


def document(G, TEXT, tx, m, ar, heat, mass_total, representations, volumes):
    """📄️ The whole asset document."""
    regions_all = ["top-flange", "web", "bottom-flange", "end-cleat", "bearers"]
    materials = [
        {"id": "steel-rusted", "title": tx("Historic mild steel St 37 with rust patina", "Historischer Flussstahl St 37 mit Rostpatina"), "emissivity": 0.8, "solarAbsorptivity": 0.78, "densityKgM3": G.RHO_STEEL, "conductivityWmK": 50, "specificHeatJKgK": G.C_STEEL, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "authored-estimate", "reference": "rust-radiation"},
        {"id": "softwood-weathered", "title": tx("Weathered softwood C24, squared", "Verwittertes Nadelholz C24, Kantholz"), "emissivity": 0.9, "solarAbsorptivity": 0.7, "densityKgM3": G.RHO_TIMBER, "conductivityWmK": 0.13, "specificHeatJKgK": G.C_TIMBER, "elasticModulusPa": 11000000000, "strengthPa": 24000000, "source": "literature", "reference": "en-338"},
    ]
    bones = [
        {"id": "beam", "parent": None, "title": tx("Beam INP 240 (rigid root)", "Träger INP 240 (starre Wurzel)"), "x": 0, "y": round(G.BEAM_Y, 6), "angleDeg": 0, "lengthM": G.L, "joint": {"type": "fixed"},
         "mass": {"kg": round(m["beam"], 2), "comX": 3.0, "comY": 0, "distribution": "uniform", "source": "literature", "reference": "din-1025-1"},
         "section": {"areaM2": 0.00461, "secondMomentM4": 4.25e-05, "extremeFibreM": 0.12, "material": "steel-rusted", "title": tx("INP 240, DIN 1025-1 (A 46.1 cm², Iy 4250 cm⁴)", "INP 240, DIN 1025-1 (A 46,1 cm², Iy 4250 cm⁴)")}},
        {"id": "cleat", "parent": "beam", "title": tx("Riveted end cleat, two angles L 80 x 8", "Genieteter Endwinkel, zwei Winkel L 80 x 8"), "x": G.CLEAT_X, "y": 0, "angleDeg": 0, "lengthM": G.CLEAT_END, "joint": {"type": "fixed"},
         "mass": {"kg": round(m["cleat"], 2), "comX": 0.045, "comY": 0, "distribution": "point", "source": "literature", "reference": "din-1028"}},
    ]
    for i, x in enumerate(G.BEARER_X):
        c = "ab"[i]
        bones.append({"id": f"bearer-{c}", "parent": "beam", "title": tx(f"Squared-timber bearer {c.upper()} (stays on the ground)", f"Kantholzunterleger {c.upper()} (bleibt am Boden)"), "x": x, "y": round(-G.HALF - G.BEARER / 2, 6), "angleDeg": 0, "lengthM": G.BEARER,
                      "joint": {"type": "prismatic", "axisDeg": 90, "minM": -1.0, "maxM": 0},
                      "mass": {"kg": round(m["bearer"], 2), "comX": G.BEARER / 2, "comY": 0, "distribution": "point", "source": "literature", "reference": "en-338"}})
    bones.append({"id": "load-carrier", "parent": "beam", "title": tx("Load carrier (massless, moves the point load)", "Lastträger (masselos, bewegt die Einzellast)"), "x": G.LOAD_X0, "y": G.HALF, "angleDeg": 0, "lengthM": 0, "joint": {"type": "prismatic", "axisDeg": 0, "minM": 0, "maxM": 5.4}})
    anchors = []
    for i in range(2):
        c = "ab"[i]
        anchors.append({"id": f"ground-{c}", "bone": f"bearer-{c}", "x": G.BEARER / 2, "y": -G.BEARER / 2, "kind": "contact", "title": tx(f"Bearer {c.upper()} on the yard ground", f"Unterleger {c.upper()} auf dem Lagerplatz"), "support": "contact", "footprintLengthM": G.BEARER, "footprintWidthM": G.BEARER_LEN, "frictionCoefficient": 0.5})
    anchors += [
        {"id": "sling-a", "bone": "beam", "x": G.SLING_X[0], "y": G.HALF, "kind": "support", "title": tx("Sling point A on the top flange (leg at 60°)", "Anschlagpunkt A am Obergurt (Strang unter 60°)"), "support": "pin"},
        {"id": "sling-b", "bone": "beam", "x": G.SLING_X[1], "y": G.HALF, "kind": "support", "title": tx("Sling point B on the top flange (leg at 120°)", "Anschlagpunkt B am Obergurt (Strang unter 120°)"), "support": "roller", "normalDeg": 120},
        {"id": "bearing-wall", "bone": "beam", "x": G.BEARING_WALL_X, "y": -G.HALF, "kind": "support", "title": tx("Flame-cut end on a wall bearing plate, roller", "Brenngeschnittenes Ende auf Mauerlagerplatte, Rollenlager"), "support": "roller", "normalDeg": 90, "footprintLengthM": G.BEARING_LEN, "footprintWidthM": G.B},
        {"id": "bearing-cleat", "bone": "cleat", "x": G.CLEAT_END - G.CLEAT_T / 2, "y": 0, "kind": "support", "title": tx("Cleat bolted to a girder web, pin", "Winkelanschluss an Unterzugsteg, Festlager"), "support": "pin"},
        {"id": "load-point", "bone": "load-carrier", "x": 0, "y": 0, "kind": "load", "title": tx("Concentrated load on the top flange", "Einzellast auf dem Obergurt")},
    ]
    sun = {"stored": {"top-flange": (1, 1), "web": (1, 0.4), "bottom-flange": (0.97, 0), "end-cleat": (1, 0.5)},
           "lifted": {"top-flange": (1, 1), "web": (1, 0.4), "bottom-flange": (1, 0), "end-cleat": (1, 0.5)},
           "reinstalled": {"top-flange": (0.25, 0), "web": (1, 0), "bottom-flange": (1, 0), "end-cleat": (0.6, 0)}}

    def binding(kind):
        """🌞️ The exhaustive region binding of a pose family."""
        r = {k: {"exposedFraction": v[0], "sunlitFraction": v[1]} for k, v in sun[kind].items()}
        if kind == "reinstalled":
            r["top-flange"]["contact"] = {"fraction": 0.75, "tempC": 20}
            r["end-cleat"]["contact"] = {"fraction": 0.4, "tempC": 20}
        r["bearers"] = {"exposedFraction": 0.75, "sunlitFraction": 0.45, "contact": {"fraction": 0.2, "tempC": 18}}
        return {"regions": r, "emitters": {}}

    total_span_load = lambda: [
        {"id": "floor-dead", "kind": "carried", "title": tx("Floor build-up 2.4 kN/m (2.0 kN/m² over 1.2 m)", "Deckenaufbau 2,4 kN/m (2,0 kN/m² über 1,2 m)"), "fxN": 0, "fyN": 0, "distributed": {"bone": "beam", "fromM": 0, "toM": G.L, "wxNPerM": 0, "wyNPerM": -2400}, "source": "authored-estimate", "reference": "en-1991-1-1"},
    ]
    poses = [
        {"id": "stored", "title": tx("Stored on two bearers", "Auf zwei Unterlegern gelagert"), "description": tx("The beam lies upright on two 100 x 100 mm squared-timber bearers at 1.25 m from each end (about 0.21 L, where the support and span moments of the self-weight are smallest), as the component exchange keeps it in the yard.", "Der Träger liegt aufrecht auf zwei Kanthölzern 100 x 100 mm, je 1,25 m vom Ende (etwa 0,21 L, wo Stütz- und Feldmoment aus Eigengewicht am kleinsten sind), so wie die Bauteilbörse ihn im Lager hält."), "bones": {}, "ground": "authored", "contacts": ["ground-a", "ground-b"], "thermal": binding("stored"), "loads": []},
        {"id": "lifted", "title": tx("Lifted by a two-leg sling", "Am Zweistranggehänge angehoben"), "description": tx("A two-leg chain sling with legs at 60° is choked around the top flange at the two lifting points above the bearers; the beam hangs 0.60 m above the bearers, which stay on the ground. The sling points carry the whole beam.", "Ein Zweistrang-Kettengehänge mit Strängen unter 60° ist an den beiden Anschlagpunkten über den Unterlegern um den Obergurt geschnürt; der Träger hängt 0,60 m über den Unterlegern, die am Boden bleiben. Die Anschlagpunkte tragen den ganzen Träger."), "bones": {"bearer-a": {"dy": -G.LIFT}, "bearer-b": {"dy": -G.LIFT}}, "root": {"x": 0, "y": G.LIFT, "angleDeg": 0}, "ground": "authored", "contacts": ["sling-a", "sling-b"], "thermal": binding("lifted"), "loads": []},
        {"id": "reinstalled", "title": tx("Reinstalled as a floor beam", "Als Deckenträger wieder eingebaut"), "description": tx("Back in service as a simply supported floor beam: the flame-cut end on a wall bearing plate (roller, 200 mm bearing), the cleat bolted to a girder web (pin), span 5.91 m, carrying a floor of 2.0 kN/m² dead and 2.0 kN/m² imposed load over a beam spacing of 1.2 m.", "Wieder im Einsatz als frei aufliegender Deckenträger: das brenngeschnittene Ende auf einer Mauerlagerplatte (Rollenlager, 200 mm Auflagertiefe), der Winkel an einen Unterzugsteg geschraubt (Festlager), Stützweite 5,91 m, mit einer Decke von 2,0 kN/m² ständiger und 2,0 kN/m² Nutzlast bei 1,2 m Trägerabstand."), "bones": {"bearer-a": {"dy": -G.INSTALL}, "bearer-b": {"dy": -G.INSTALL}}, "root": {"x": 0, "y": G.INSTALL, "angleDeg": 0}, "ground": "authored", "contacts": ["bearing-wall", "bearing-cleat"], "thermal": binding("reinstalled"),
         "loads": total_span_load() + [{"id": "floor-imposed", "kind": "imposed", "title": tx("Imposed floor load 2.4 kN/m (category A, 2.0 kN/m² over 1.2 m)", "Nutzlast 2,4 kN/m (Kategorie A, 2,0 kN/m² über 1,2 m)"), "fxN": 0, "fyN": 0, "distributed": {"bone": "beam", "fromM": 0, "toM": G.L, "wxNPerM": 0, "wyNPerM": -2400}, "source": "literature", "reference": "en-1991-1-1"}]},
        {"id": "reinstalled-point-load", "title": tx("Reinstalled, point load at midspan", "Wieder eingebaut, Einzellast in Feldmitte"), "description": tx("The reinstalled beam with its floor build-up and the 2 kN concentrated imposed load of EN 1991-1-1 at midspan.", "Der wieder eingebaute Träger mit Deckenaufbau und der Einzellast 2 kN nach EN 1991-1-1 in Feldmitte."), "bones": {"bearer-a": {"dy": -G.INSTALL}, "bearer-b": {"dy": -G.INSTALL}, "load-carrier": {"dx": round((G.BEARING_WALL_X + G.CLEAT_X + G.CLEAT_END - G.CLEAT_T / 2) / 2 - G.LOAD_X0, 4)}}, "root": {"x": 0, "y": G.INSTALL, "angleDeg": 0}, "ground": "authored", "contacts": ["bearing-wall", "bearing-cleat"], "thermal": binding("reinstalled"),
         "loads": total_span_load() + [{"id": "point-load", "kind": "live", "title": tx("Concentrated load 2 kN", "Einzellast 2 kN"), "anchor": "load-point", "fxN": 0, "fyN": -2000, "source": "literature", "reference": "en-1991-1-1"}]},
    ]
    clips = [
        {"id": "lift", "title": tx("Lift off the bearers", "Abheben von den Unterlegern"), "description": tx("The sling takes up the load for two seconds, then the crane hoists the beam 0.60 m clear of the bearers, which stay behind on the ground.", "Das Gehänge nimmt zwei Sekunden lang die Last auf, dann hebt der Kran den Träger 0,60 m von den Unterlegern ab, die am Boden zurückbleiben."), "durationS": 8, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "linear"}, {"t": 2, "pose": "stored", "ease": "smooth"}, {"t": 8, "pose": "lifted", "ease": "smooth"}],
         "contacts": [{"anchor": "ground-a", "fromS": 0, "toS": 2}, {"anchor": "ground-b", "fromS": 0, "toS": 2}, {"anchor": "sling-a", "fromS": 2, "toS": 8}, {"anchor": "sling-b", "fromS": 2, "toS": 8}],
         "phases": [{"id": "take-up", "fromS": 0, "toS": 2, "title": tx("Sling takes up the load", "Gehänge nimmt die Last auf")}, {"id": "hoist", "fromS": 2, "toS": 8, "title": tx("Hoist", "Heben")}],
         "events": [{"t": 2, "kind": "contact-off", "title": tx("The beam leaves the bearers", "Der Träger verlässt die Unterleger")}, {"t": 2, "kind": "contact-on", "title": tx("The sling points carry the beam", "Die Anschlagpunkte tragen den Träger")}], "tracks": []},
        {"id": "moving-load", "title": tx("Moving point load on the reinstalled beam", "Wandernde Einzellast auf dem wieder eingebauten Träger"), "description": tx("A 2 kN load travels along the top flange from the wall bearing to the cleat end over the floor build-up, so the span moment and the reactions shift with it.", "Eine Last von 2 kN wandert über dem Deckenaufbau auf dem Obergurt vom Mauerauflager zum Winkelende; Feldmoment und Auflagerkräfte verschieben sich mit ihr."), "durationS": 6, "fps": 30, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "reinstalled-point-load", "ease": "smooth", "bones": {"load-carrier": {"dx": 0.1}}}, {"t": 6, "pose": "reinstalled-point-load", "ease": "smooth", "bones": {"load-carrier": {"dx": 5.4}}}],
         "contacts": [{"anchor": "bearing-wall", "fromS": 0, "toS": 6}, {"anchor": "bearing-cleat", "fromS": 0, "toS": 6}],
         "phases": [{"id": "travel", "fromS": 0, "toS": 6, "title": tx("Travel", "Wanderung")}],
         "events": [{"t": 0, "kind": "motion", "title": tx("Load starts beside the wall bearing", "Last beginnt neben dem Mauerauflager")}], "tracks": []},
    ]
    region_meta = {
        "top-flange": (tx("Top flange (upper face, edges, sloped inner faces)", "Obergurt (Oberseite, Kanten, geneigte Innenflächen)"), "horizontal-plate-up", G.B, 0.9),
        "web": (tx("Web faces and their fillets (both sides)", "Stegflächen mit Ausrundungen (beidseitig)"), "vertical-plate", 0.19, 0.35),
        "bottom-flange": (tx("Bottom flange (soffit, edges, sloped inner faces)", "Untergurt (Unterseite, Kanten, geneigte Innenflächen)"), "horizontal-plate-down", G.B, 0.1),
        "end-cleat": (tx("Riveted end cleat with its rivet heads", "Genieteter Endwinkel mit Nietköpfen"), "vertical-plate", G.CLEAT_LEN, 0.4),
        "bearers": (tx("Squared-timber bearers", "Kantholzunterleger"), "horizontal-plate-up", G.BEARER, 0.5),
    }
    regions = []
    for rid in regions_all:
        title, geom, lc, svf = region_meta[rid]
        regions.append({"id": rid, "title": title, "kind": "surface", "model": "passive", "material": "softwood-weathered" if rid == "bearers" else "steel-rusted", "areaM2": round(ar[rid], 3), "characteristicLengthM": lc, "geometry": geom, "skyViewFactor": svf, "layers": [], "heatCapacityJM2K": heat[rid], "source": "authored-estimate", "reference": "en-338" if rid == "bearers" else "rust-radiation"})
    thermal = {"environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 18, "relativeHumidity": 0.5, "windMS": 1, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.2, "pressurePa": 101325},
               "operating": [], "regions": regions, "emitters": [], "assumptions": TEXT["thermal"], "passengers": []}
    structure = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": TEXT["structure"], "clinical": False}
    asset = {
        "id": "reclaimed-steel-i-beam", "category": "components",
        "title": tx("Reclaimed Steel I-Beam", "Gebrauchter Stahlträger"),
        "summary": TEXT["summary"],
        "tags": ["component", "reclaimed", "steel", "beam", "i-section", "historic", "riveted", "structural"],
        "provenance": {"author": "Semio staffage component authors", "method": "handcrafted", "created": "2026-11-22", "references": TEXT["references"], "notes": TEXT["notes"]},
        "dimensions": {"heightM": round(G.BEARER + G.H, 4), "widthM": round(G.CLEAT_X + G.CLEAT_END, 4), "depthM": G.BEARER_LEN, "massKg": round(mass_total, 2)},
        "materials": materials,
        "rig": {"kind": "structural", "root": "beam", "bones": bones},
        "volumes": volumes,
        "anchors": anchors,
        "representations": representations,
        "symbols": [],
        "poses": poses,
        "clips": clips,
        "thermal": thermal,
        "structure": structure,
        "presentation": {"defaultPose": "stored", "defaultClip": "lift", "defaultScale": 100},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
