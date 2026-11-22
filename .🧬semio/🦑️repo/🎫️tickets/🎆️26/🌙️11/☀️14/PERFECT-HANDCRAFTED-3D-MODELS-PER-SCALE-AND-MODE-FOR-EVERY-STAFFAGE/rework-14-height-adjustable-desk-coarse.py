"""🖥️ Height-adjustable desk, second round: the monitor of 1:100, 1:200 and 1:500 was the 1:50 parts list (bezel frame, backplane, rear panel, five-section bulge, VESA plate, hinge ears, hinge pin, four 4 mm screws). Each coarse scale gets a monitor modelled for its own feature: 1:100 a 14 mm panel slab with the glass set 1 mm into its face, the rear bulge and one 116 mm mount block that reaches the stand; 1:200 a 29 mm slab, the glass and a two-section rear housing; 1:500 one 59 mm block with the glass. Thermal and decisions follow.

Run from the ticket folder: python3 rework-14-height-adjustable-desk-coarse.py
"""
import json

PATH = "volumes/furniture/height-adjustable-desk.json"
document = json.load(open(PATH))
HOUSING = {"bone": "screen-tilt", "material": "equipment-housing", "role": "body"}
GONE = ("monitor-backplane", "monitor-rear-panel", "monitor-bulge", "monitor-vesa-plate", "monitor-hinge-ear", "monitor-hinge", "vesa-screw-bl", "vesa-screw-br", "vesa-screw-tl", "vesa-screw-tr")


def slab(front, back, radius=None):
    solid = {"id": "monitor-panel", "kind": "box", **HOUSING, "tone": 0.18, "size": [0.614, 0.368, round(back - front, 4)], "transform": {"translation": [0, -0.004, round((front + back) / 2, 5)]}}
    if radius:
        solid["cornerRadius"] = radius
    return solid


MONITOR = {
    "100": [
        slab(-0.0607, -0.047, 0.004),
        {"id": "monitor-bulge", "kind": "loft", **HOUSING, "tone": 0.24, "capStart": True, "capEnd": True, "sections": [
            {"at": [0, -0.015, -0.047], "rx": 0.26, "ry": 0.15, "exponent": 5},
            {"at": [0, -0.015, -0.03], "rx": 0.245, "ry": 0.142, "exponent": 4},
            {"at": [0, -0.015, -0.016], "rx": 0.212, "ry": 0.12, "exponent": 4}]},
        {"id": "monitor-vesa-plate", "kind": "box", "bone": "screen-tilt", "material": "steel-desk", "role": "body", "tone": 0.31, "size": [0.116, 0.116, 0.016], "transform": {"translation": [0, 0, -0.01]}},
    ],
    "200": [
        slab(-0.0607, -0.0317),
        {"id": "monitor-bulge", "kind": "loft", **HOUSING, "tone": 0.24, "capStart": True, "capEnd": True, "sections": [
            {"at": [0, -0.015, -0.0317], "rx": 0.25, "ry": 0.145, "exponent": 5},
            {"at": [0, -0.015, -0.002], "rx": 0.15, "ry": 0.085, "exponent": 4}]},
    ],
    "500": [slab(-0.0607, -0.002)],
}
REAR = {"100": ["monitor-bulge", "monitor-vesa-plate"], "200": ["monitor-bulge"], "500": []}
FIRST = {
    "100": ("the tilt hinge, the rear bulge of the monitor (merged into a 60 mm panel block that reaches the stand)", "the tilt hinge with its ears and the four VESA screws (merged into one 116 x 116 x 16 mm mount block between rear bulge and stand), the bezel step and the backplane of the monitor (merged into a 14 mm panel slab that carries the glass set 1 mm into its face)",
            "das Neigegelenk, die rückseitige Wölbung des Monitors (in einen 60-mm-Panelblock verschmolzen, der bis zum Ständer reicht)", "das Neigegelenk mit seinen Laschen und die vier VESA-Schrauben (in einen Halteblock 116 x 116 x 16 mm zwischen Rückwölbung und Ständer verschmolzen), die Rahmenstufe und die Rückwand des Monitors (in eine 14-mm-Panelscheibe verschmolzen, in deren Vorderseite das Glas 1 mm eingelassen ist)"),
    "200": ("the monitor keeps base, stand and a 60 mm panel block", "the monitor keeps base, stand, a 29 mm panel slab with the glass set 1 mm into its face and a 30 mm rear housing that tapers to the stand",
            "der Monitor behält Fuß, Ständer und einen 60-mm-Panelblock", "der Monitor behält Fuß, Ständer, eine 29-mm-Panelscheibe mit dem 1 mm eingelassenen Glas auf der Vorderseite und ein 30 mm tiefes Rückgehäuse, das sich zum Ständer verjüngt"),
    "500": ("the monitor as panel and stand and the laptop and control box as thin blocks; the tray, the monitor base and every rounding are omitted", "the monitor as one 59 mm panel block with its glass, base plate and stand, and the laptop and control box as thin blocks; the tray, the side rails and every rounding are omitted",
            "der Monitor als Panel und Ständer, Laptop und Steuerkasten als dünne Blöcke; Wanne, Monitorfuß und alle Rundungen entfallen", "der Monitor als ein 59-mm-Panelblock mit Glas, Fußplatte und Ständer, Laptop und Steuerkasten als dünne Blöcke; Wanne, Seitenzargen und alle Rundungen entfallen"),
}
THIRD = {
    "100": ("The monitor is modelled for the 15 mm feature: glass sheet, panel slab, rear bulge in three sections and the mount block; hinge pin, ears and screws exist at 1:50 only. In the thermal list the housing is the hollow shell it is (8 mm bezel ring and 3 mm rear wall around the display cavity), so the face behind the glass is no outer surface.", "Der Monitor ist für das 15-mm-Merkmal modelliert: Glasscheibe, Panelscheibe, Rückwölbung in drei Schnitten und Halteblock; Gelenkbolzen, Laschen und Schrauben gibt es nur bei 1:50. In der Thermik ist das Gehäuse die hohle Schale, die es ist (8-mm-Rahmenring und 3-mm-Rückwand um den Displayraum), sodass die Fläche hinter dem Glas keine Außenfläche ist."),
    "200": ("The monitor is modelled for the 30 mm feature: glass sheet, panel slab and a rear housing in two sections that meets the stand; mount plate and hinge are inside that housing.", "Der Monitor ist für das 30-mm-Merkmal modelliert: Glasscheibe, Panelscheibe und ein Rückgehäuse in zwei Schnitten, das an den Ständer anschließt; Halteplatte und Gelenk liegen in diesem Gehäuse."),
    "500": ("The monitor is one block of its full 59 mm depth from the glass to the stand, with the glass as the sheet on its face that the display region needs.", "Der Monitor ist ein Block seiner vollen Tiefe von 59 mm vom Glas bis zum Ständer, mit dem Glas als Scheibe auf seiner Vorderseite, die die Displayregion braucht."),
}
for scale, solids in MONITOR.items():
    volume = document["volumes"][scale]
    body = [s for s in volume["body"] if s["id"] not in GONE and s["id"] != "monitor-panel"]
    index = next(i for i, s in enumerate(body) if s["id"] == "monitor-screen")
    body[index:index] = solids[:1]
    body[index + 2:index + 2] = solids[1:]
    volume["body"] = body
    thermal = [s for s in volume["thermal"] if s.get("of") not in GONE]
    at = next(i for i, s in enumerate(thermal) if s["id"] == "th-monitor-screen") + 1
    thermal[at:at] = [{"id": f"th-{name}", "of": name, "region": "monitor-housing", "layer": "housing"} for name in REAR[scale]]
    if scale == "100":
        shell = {"bone": "screen-tilt", "material": "equipment-housing", "role": "body", "tone": 0.18, "region": "monitor-housing", "layer": "housing"}
        ring = {"id": "th-monitor-panel", "kind": "extrude", **shell, "plane": "xy", "profile": [[0.307, 0.18], [-0.307, 0.18], [-0.307, -0.188], [0.307, -0.188]], "holes": [[[0.299, -0.164], [-0.299, -0.164], [-0.299, 0.172], [0.299, 0.172]]], "from": -0.0607, "to": -0.047}
        wall = {"id": "th-monitor-rear-wall", "kind": "box", **shell, "size": [0.614, 0.368, 0.003], "transform": {"translation": [0, -0.004, -0.0485]}}
        place = next(i for i, s in enumerate(thermal) if s["id"] == "th-monitor-panel")
        thermal[place:place + 1] = [ring, wall]
    volume["thermal"] = thermal
    first = volume["decisions"][0]
    old_en, new_en, old_de, new_de = FIRST[scale]
    assert old_en in first["en"] and old_de in first["de"], scale
    first["en"] = first["en"].replace(old_en, new_en)
    first["de"] = first["de"].replace(old_de, new_de)
    third = volume["decisions"][2]
    marker_en = "The 15 mm bezel surrounds a 2 mm display glass. The contoured rear cover bears on a four-screw VESA plate. The hinge supports the display on the neck while its angle changes."
    marker_de = "Der 15 mm tiefe Rahmen fasst ein 2 mm starkes Deckglas. Das profilierte Rückgehäuse sitzt auf einer VESA-Platte mit vier Schrauben. Das Gelenk trägt das neigbare Display am Standfuß."
    assert marker_en in third["en"] and marker_de in third["de"], scale
    third["en"] = third["en"].replace(marker_en, THIRD[scale][0])
    third["de"] = third["de"].replace(marker_de, THIRD[scale][1])
json.dump(document, open(PATH, "w"), ensure_ascii=False, indent=1)
