"""🧰 Round 3 cast drinking fountain: root fixes outside `volumes` (rig, anchors, poses, emitter, areas, texts).

`apply(asset, areas)` changes the asset in place; `areas` maps region id to the measured `areaM2` (or None to keep the values).

@see r3-drinking-fountain-kit.py — the geometry these numbers come from
"""
import importlib.util
import os

T = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("kit", os.path.join(T, "r3-drinking-fountain-kit.py"))
K = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(K)


def text(en, de):
    """🗣️ A bilingual text, English first."""
    return {"en": en, "de": de}


def rig(asset):
    """🦴 Upright bones at their real joints: pedestal up its axis to the bowl seat, bowl up to the rim, goose-neck up from its base, button out of its boss, jet along the outlet axis."""
    bones = {b["id"]: b for b in asset["rig"]["bones"]}
    ox, oy = K.outlet()
    spec = {
        "pedestal": (None, 0.0, 0.0, 90, K.Y_BASIN, {"comX": 0.3, "comY": 0.0}),
        "basin": ("pedestal", K.Y_BASIN, 0.0, 0, K.r6(K.RIM_Y + K.BEAD_R - K.Y_BASIN), {"comX": 0.06, "comY": 0.0}),
        "bubbler": ("basin", K.r6(K.Y_BUBBLER - K.Y_BASIN), K.r6(-K.BX), 0, K.r6(1.0 - K.Y_BUBBLER), {"comX": 0.1, "comY": -0.03}),
        "button": ("pedestal", K.BTN_Y, K.r6(-K.BTN_X), -90, 0.0225, {"comX": 0.01, "comY": 0.0}),
        "jet": ("bubbler", K.r6(oy - K.Y_BUBBLER), K.r6(-(ox - K.BX)), K.r6(K.OUT_DEG - 90), 0.04, None),
    }
    for bid, (parent, x, y, angle, length, mass) in spec.items():
        b = bones[bid]
        b["parent"], b["x"], b["y"], b["angleDeg"], b["lengthM"] = parent, x, y, angle, length
        if mass:
            b["mass"].update(mass)
    bones["pedestal"]["title"] = text("Cast pedestal: base, fluted shaft and capital", "Gusssockel: Fuß, kannelierter Schaft und Kapitell")
    bones["basin"]["title"] = text("Spun stainless bowl", "Gedrücktes Edelstahlbecken")
    bones["button"]["joint"] = {"type": "prismatic", "minM": 0, "maxM": 0.01, "axisDeg": 90}
    bones["jet"]["joint"] = {"type": "prismatic", "minM": -K.JET_TRAVEL, "maxM": 0, "axisDeg": 0}
    bones["jet"]["title"] = text("Water jet (sinks into the pedestal when off)", "Wasserstrahl (versinkt im Sockel, wenn aus)")


def anchors(asset):
    """⚓ Flange on the footing, nozzle mouth, the front of the rim where a drinker leans."""
    a = {x["id"]: x for x in asset["anchors"]}
    ox, oy = K.outlet()
    a["base-fixed"].update({"x": 0.0, "y": 0.0})
    p = K.local2("bubbler", ox, oy)
    a["nozzle"].update({"x": p[0], "y": p[1]})
    a["nozzle"]["title"] = text("Mouth of the goose-neck nozzle", "Mund der Schwanenhalsdüse")
    p = K.local2("basin", K.BEAD_C, K.RIM_Y + K.BEAD_R)
    a["lean-point"].update({"x": p[0], "y": p[1]})
    a["lean-point"]["title"] = text("Hands on the front of the bowl rim", "Hände vorn auf dem Beckenrand")


def poses(asset):
    """🎚️ The jet sinks down the pedestal axis, the button slides into its boss."""
    for p in asset["poses"]:
        jet = p["bones"].get("jet", {})
        off = jet.get("dx", 0) if "dx" in jet else (-K.JET_TRAVEL if jet.get("dy", 0) < 0 else 0)
        p["bones"]["jet"] = {"dx": off}
        if "button" in p["bones"]:
            p["bones"]["button"] = {"dy": 0.01}
        if p["id"] == "idle-sun":
            p["description"] = text("Tap off, the jet sunk into the pedestal, sun on the bowl and the neck.", "Hahn aus, der Strahl im Sockel versunken, Sonne auf Becken und Hals.")
        if p["id"] == "drinking":
            p["description"] = text("Button pressed 10 mm, the jet falls from the nozzle in an arc into the bowl and a person leans on the front of the rim with 200 N.", "Taster 10 mm gedrückt, der Strahl fällt von der Düse im Bogen ins Becken und eine Person stützt sich mit 200 N vorn auf den Rand.")


def thermal(asset, areas):
    """🌡️ The outlet direction of the goose-neck and the areas of the honest model."""
    asset["thermal"]["emitters"][0]["directionDeg"] = K.r6(K.OUT_DEG)
    by = {r["id"]: r for r in asset["thermal"]["regions"]}
    by["bubbler-brass"]["title"] = text("Goose-neck, nozzle, button and bezel", "Schwanenhals, Düse, Taster und Blende")
    by["bubbler-brass"]["layers"][0]["title"] = text("Chrome-plated brass tube 28 x 2 mm", "Verchromtes Messingrohr 28 x 2 mm")
    by["basin-steel"]["title"] = text("Spun stainless bowl with waste", "Gedrücktes Edelstahlbecken mit Ablauf")
    by["pedestal-cast"]["title"] = text("Cast pedestal (three castings)", "Gusssockel (drei Gussteile)")
    if areas:
        for rid, value in areas.items():
            by[rid]["areaM2"] = value


def provenance(asset):
    """📚 Rewritten notes: the construction and the physics of the jet."""
    refs = asset["provenance"]["references"]
    for r in refs:
        if r["id"] == "fountain-class":
            r["citation"] = "Cast-iron pedestal drinking fountains of the heritage class (stacked castings on a tie rod, 0.9 m rim, 0.3 to 0.4 m bowl, bubbler or spout flow about 2-4 L/min, mass 50-90 kg) from manufacturer datasheets of the product class; authored-estimate summary, no single datasheet."
    asset["provenance"]["notes"] = text(
        "A heritage pedestal fountain 1.0 m high: three stacked castings of 12 mm wall (base 0.40 m with die 0.236 m, fluted shaft 0.184 to 0.160 m, capital with a cup) clamped by a tie rod and bolted to the footing with four M12 anchors; a spun 1.5 mm stainless bowl of 0.40 m, rim at 0.90 m; a 28 mm chrome-plated brass goose-neck whose nozzle looks 40 degrees down over the bowl centre with a 57 mm air gap above the rim (EN 1717); a 38 mm push button in a boss on the neck. Mass 68 kg and a flow of 3 L/min are class values (authored estimates). The jet leaves the 12 mm mouth at 0.44 m/s (3 L/min), falls 0.16 m on a parabola and thins to 6 mm as it speeds up to 1.8 m/s. The jet is a bone that sinks 0.63 m down the axis into the hollow die when the tap is off, so it is not drawn as running water in the idle poses. The jet emitter releases air saturated at the 12 degC supply temperature (humidity ratio 0.0087 kg/kg, 14 K below the 26 degC ambient of the reference case, here applied as a 4 K cooling of the entrained air) at 0.0003 m3/s: an authored estimate of the entrained boundary-layer air of the jet.",
        "Ein historisierender Standbrunnen von 1,0 m Höhe: drei gestapelte Gussteile mit 12 mm Wand (Fuß 0,40 m mit Sockelkörper 0,236 m, kannelierter Schaft 0,184 bis 0,160 m, Kapitell mit Schale), durch einen Zuganker verspannt und mit vier M12-Ankern am Fundament verschraubt; ein gedrücktes 1,5-mm-Edelstahlbecken von 0,40 m, Rand auf 0,90 m; ein verchromter 28-mm-Messing-Schwanenhals, dessen Düse 40 Grad nach unten über die Beckenmitte zeigt, mit 57 mm freiem Auslauf über dem Rand (EN 1717); ein 38-mm-Drucktaster in einer Nabe am Hals. Masse 68 kg und ein Durchfluss von 3 L/min sind Klassenwerte (Schätzwerte). Der Strahl verlässt den 12-mm-Mund mit 0,44 m/s (3 L/min), fällt 0,16 m auf einer Parabel und dünnt auf 6 mm aus, während er auf 1,8 m/s beschleunigt. Der Strahl ist ein Knochen, der bei geschlossenem Hahn 0,63 m entlang der Achse in den hohlen Sockelkörper versinkt, damit er in den Ruheposen nicht als fließendes Wasser gezeichnet wird. Der Strahl-Emitter gibt bei 0,0003 m3/s Luft ab, die bei der Zulauftemperatur von 12 degC gesättigt ist (Feuchtegrad 0,0087 kg/kg, 14 K unter der 26 degC Umgebung des Referenzfalls, hier als 4 K Abkühlung der mitgerissenen Luft angesetzt): eine Schätzung der mitgerissenen Grenzschichtluft des Strahls.")
    asset["summary"] = text(
        "A 1.0 m cast-iron pedestal drinking fountain with a stainless bowl and a goose-neck spout: a push button opens the tap, the water jet falls in an arc into the bowl as a cool, wet source of moisture, and the pedestal warms in the sun.",
        "Ein 1,0 m hoher Trinkbrunnen mit Gusssockel, Edelstahlbecken und Schwanenhals-Auslauf: ein Drucktaster öffnet den Hahn, der Wasserstrahl fällt im Bogen ins Becken als kühle, feuchte Feuchtequelle und der Sockel erwärmt sich in der Sonne.")


def apply(asset, areas=None):
    """🛠️ Every root fix in order."""
    rig(asset)
    anchors(asset)
    poses(asset)
    thermal(asset, areas)
    provenance(asset)
