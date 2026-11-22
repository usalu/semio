"""🏗️ Closes the load path of the structure lists of `vehicles/city-bus` at 1:100, 1:200 and 1:500: roof unit rails bear on the roof bows, door headers carry the leaf carriages between two posts, spring struts carry the air springs from the longerons.

Usage: python3 rework-city-bus-structure-path.py
Every number is chosen by hand; the script is idempotent (a part that exists is replaced).
"""
import copy
import json

SOURCE = "volumes/vehicles/city-bus.json"


def put(solids, solid, after):
    index = next((i for i, s in enumerate(solids) if s["id"] == solid["id"]), None)
    if index is not None:
        solids[index] = solid
        return
    at = next(i for i, s in enumerate(solids) if s["id"] == after)
    solids.insert(at + 1, solid)


def member(template, ident, **geometry):
    solid = {k: v for k, v in template.items() if k in ("bone", "material", "role", "tone", "member", "part")}
    return {"id": ident, **solid, **geometry}


def rails(parts, drop):
    for ident in ("battery-rail-l", "battery-rail-r", "hvac-rail-l", "hvac-rail-r"):
        parts[ident]["transform"]["translation"][1] = drop


def headers(structure, template, y, height, z, depth):
    put(structure, member(template, "door-header-m", kind="box", size=[3.24, height, depth], transform={"translation": [6.67, y, z]}), "cant-rail-r")
    put(structure, member(template, "door-header-f", kind="box", size=[1.82, height, depth], transform={"translation": [10.56, y, z]}), "door-header-m")


def struts(structure, template, radius, front_top, rear_top):
    for side, z in (("l", 1), ("r", -1)):
        put(structure, member(template, f"spring-strut-f-{side}", kind="capsule", p0=[9.35, 0.36, 0.26 * z], p1=[9.35, front_top, 0.7 * z], r0=radius, r1=radius), f"longeron-{side}")
        if rear_top is not None:
            put(structure, member(template, f"spring-strut-r-{side}", kind="capsule", p0=[3.45, 0.36, 0.26 * z], p1=[3.45, rear_top, 0.47 * z], r0=radius, r1=radius), f"longeron-{side}")


def say(volume, en, de):
    text = volume["decisions"][2]
    if en not in text["en"]:
        text["en"] += " " + en
        text["de"] += " " + de


def main():
    with open(SOURCE, encoding="utf8") as handle:
        document = json.load(handle)
    volumes = document["volumes"]

    structure = volumes["200"]["structure"]
    parts = {s["id"]: s for s in structure}
    rails(parts, -0.045)
    headers(structure, parts["cant-rail-r"], 2.415, 0.07, -1.24, 0.14)
    for side, z in (("l", 1), ("r", -1)):
        parts[f"axle-f-spring-{side}"]["origin"] = [0, -0.2, 0.7 * z]
        parts[f"axle-f-spring-{side}"]["profile"] = [[0, 0], [0, 0.11], [0.6, 0.11], [0.6, 0]]
    struts(structure, parts["longeron-l"], 0.035, 0.9, 0.58)
    say(
        volumes["200"],
        "The load path is closed: the rails of both roof units bear on the roof bows, a header over each doorway spans from post to post and carries the leaf carriages, the front air springs stand on the dropped beam, and an inclined strut each side takes every spring head back to the longeron.",
        "Der Lastpfad ist geschlossen: die Schienen beider Dachgeräte liegen auf den Dachspriegeln, ein Sturz über jeder Türöffnung spannt von Pfosten zu Pfosten und trägt die Laufwagen der Flügel, die vorderen Luftfedern stehen auf dem gekröpften Träger, und je Seite führt eine schräge Strebe jeden Federkopf zum Längsträger zurück.",
    )

    structure = volumes["500"]["structure"]
    parts = {s["id"]: s for s in structure}
    rails(parts, -0.04)
    previous = "roof-bow-3-3"
    for source, ident in (
        ("post-l-1-lower", "post-l-4-lower"),
        ("post-l-1", "post-l-4"),
        ("post-r-1-lower", "post-r-4-lower"),
        ("post-r-1", "post-r-4"),
        ("cross-member-1", "cross-member-4"),
        ("roof-bow-1-1", "roof-bow-4-1"),
        ("roof-bow-1-2", "roof-bow-4-2"),
        ("roof-bow-1-3", "roof-bow-4-3"),
    ):
        ring = copy.deepcopy(parts[source])
        ring["id"] = ident
        ring["transform"]["translation"][0] = 8.25
        put(structure, ring, previous)
        previous = ident
    headers(structure, parts["cant-rail-r"], 2.425, 0.09, -1.25, 0.14)
    struts(structure, parts["longeron-l"], 0.045, 0.7, None)
    text = volumes["500"]["decisions"][2]
    text["en"] = text["en"].replace("tied by three ring frames", "tied by four ring frames (rear, centre door, front axle, front)")
    text["de"] = text["de"].replace("verbunden durch drei Ringspante", "verbunden durch vier Ringspante (Heck, Mitteltür, Vorderachse, Front)")
    say(
        volumes["500"],
        "The rails of both roof units bear on the roof bows, a header over each doorway spans from post to post and carries the leaf carriages, the axle beams run into the rims, and a strut each side takes the front spring head to the longeron, so the load path is closed down to the wheels.",
        "Die Schienen beider Dachgeräte liegen auf den Dachspriegeln, ein Sturz über jeder Türöffnung spannt von Pfosten zu Pfosten und trägt die Laufwagen der Flügel, die Achsträger laufen in die Felgen, und je Seite führt eine Strebe den vorderen Federkopf zum Längsträger, sodass der Lastpfad bis zu den Rädern geschlossen ist.",
    )

    structure = volumes["100"]["structure"]
    parts = {s["id"]: s for s in structure}
    rails(parts, -0.05)
    headers(structure, parts["cant-rail-r"], 2.42, 0.06, -1.235, 0.13)
    for leaf in ("leaf-m-aft", "leaf-m-fwd", "leaf-f-aft", "leaf-f-fwd"):
        stile = parts[f"{leaf}-stile-b-upper"]
        put(structure, member(stile, f"{leaf}-rail-top", kind="box", size=[0.595, 0.05, 0.02], transform={"translation": [0.3125, 2.005, -1.264]}), f"{leaf}-stile-b-upper")
        put(structure, member(stile, f"{leaf}-rail-bottom", kind="box", size=[0.595, 0.05, 0.02], transform={"translation": [0.3125, 0.025, -1.293]}), f"{leaf}-rail-top")
    for side, z in (("l", 1), ("r", -1)):
        put(structure, {"id": f"axle-f-carrier-{side}", "of": f"spring-carrier-f-{side}", "member": "axle-f", "part": "member"}, "axle-f-beam")
        put(structure, {"id": f"axle-r-carrier-{side}", "of": f"spring-carrier-r-{side}", "member": "axle-r", "part": "member"}, "axle-r-beam")
        put(structure, member(parts["longeron-l"], f"spring-strut-f-{side}", kind="capsule", p0=[9.35, 0.36, 0.26 * z], p1=[9.35, 0.9, 0.7 * z], r0=0.03, r1=0.03), f"longeron-{side}")
        put(structure, member(parts["longeron-l"], f"spring-strut-r-{side}-a", kind="capsule", p0=[3.11, 0.36, 0.26 * z], p1=[3.11, 0.64, 0.47 * z], r0=0.03, r1=0.03), f"longeron-{side}")
        put(structure, member(parts["longeron-l"], f"spring-strut-r-{side}-b", kind="capsule", p0=[3.79, 0.36, 0.26 * z], p1=[3.79, 0.64, 0.47 * z], r0=0.03, r1=0.03), f"spring-strut-r-{side}-a")
    say(
        volumes["100"],
        "The load path is closed: the rails of both roof units bear on the roof bows, a header over each doorway spans between the posts and carries the leaf carriages, each leaf frame has its top and bottom rail, the spring carriers tie the air springs to the axle beams, and an inclined strut takes every spring head back to the longeron.",
        "Der Lastpfad ist geschlossen: die Schienen beider Dachgeräte liegen auf den Dachspriegeln, ein Sturz über jeder Türöffnung spannt zwischen den Pfosten und trägt die Laufwagen der Flügel, jeder Flügelrahmen hat seinen oberen und unteren Riegel, die Federträger binden die Luftfedern an die Achsträger, und eine schräge Strebe führt jeden Federkopf zum Längsträger zurück.",
    )

    with open(SOURCE, "w", encoding="utf8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")


if __name__ == "__main__":
    main()
