"""🔩 Joins the body parts of `vehicles/city-bus` that hung in the air: wheel hubs with flange and barrel, spring seats, stub axles, struts, door guide rails, mirror arms, the driver's console, seat pedestal and steering wheel hub.

Usage: python3 rework-city-bus-body-joints.py
Every number is chosen by hand against the reference table in `rework/assets/vehicles--city-bus.md`; the script is idempotent (a part that exists is replaced).
"""
import json

SOURCE = "volumes/vehicles/city-bus.json"
PAINT = {"material": "paint", "role": "detail", "tone": 0.3}


def by_id(solids):
    return {solid["id"]: solid for solid in solids}


def put(solids, solid, after):
    index = next((i for i, s in enumerate(solids) if s["id"] == solid["id"]), None)
    if index is not None:
        solids[index] = solid
        return
    at = next(i for i, s in enumerate(solids) if s["id"] == after)
    solids.insert(at + 1, solid)


def mirrored_profile(profile):
    return [[-a, r] for a, r in profile]


def like(template, **changes):
    solid = {k: v for k, v in template.items() if k in ("id", "bone", "material", "role", "tone", "kind")}
    solid.update(changes)
    return solid


def hubs(body, front, rear):
    parts = by_id(body)
    parts["fl-hub"]["profile"] = front
    parts["fr-hub"]["profile"] = mirrored_profile(front)
    parts["rl-hub"]["profile"] = rear
    parts["rr-hub"]["profile"] = mirrored_profile(rear)


def box(template, ident, bone, size, at, **more):
    return like(template, id=ident, bone=bone, kind="box", size=size, transform={"translation": at}, **more)


def cylinder(template, ident, bone, origin, radius, height):
    return like(template, id=ident, bone=bone, kind="revolve", profile=[[0, radius], [height, radius], [height, 0.0]], axis=[0, 1, 0], origin=origin)


def seats(body, template, after, rear_width):
    for ident, at, width in (
        ("spring-seat-r-l", [3.79, 0.72, 0.47], rear_width),
        ("spring-seat-ra-l", [3.11, 0.72, 0.47], rear_width),
        ("spring-seat-r-r", [3.79, 0.72, -0.47], rear_width),
        ("spring-seat-ra-r", [3.11, 0.72, -0.47], rear_width),
        ("spring-seat-f-l", [9.35, 0.98, 0.7], 0.24),
        ("spring-seat-f-r", [9.35, 0.98, -0.7], 0.24),
    ):
        put(body, box(template, ident, "body", [0.26, 0.2, width], at), after)


def tracks(body, template, after, y0, y1, z_out, z_in):
    size_y, mid_y, size_z, mid_z = round(y1 - y0, 4), round((y0 + y1) / 2, 4), round(z_out - z_in, 4), round(-(z_out + z_in) / 2, 5)
    put(body, box(template, "door-track-m", "body", [3.075, size_y, size_z], [6.5875, mid_y, mid_z]), after)
    put(body, box(template, "door-track-f", "body", [1.82, size_y, size_z], [10.56, mid_y, mid_z]), "door-track-m")


def say(volume, index, en, de):
    text = volume["decisions"][index]
    if en not in text["en"]:
        text["en"] += " " + en
        text["de"] += " " + de


def main():
    with open(SOURCE, encoding="utf8") as handle:
        document = json.load(handle)
    volumes = document["volumes"]

    body = volumes["50"]["body"]
    parts = by_id(body)
    seat = parts["spring-seat-f-l"]
    hubs(
        body,
        [[0.215, 0], [0.212, 0.045], [0.195, 0.072], [0.165, 0.08], [0.16, 0.08], [0.16, 0.105], [0.148, 0.105], [0.148, 0.2], [0.118, 0.2], [0.118, 0.11], [-0.07, 0.11], [-0.07, 0]],
        [[0.105, 0], [0.105, 0.06], [0.085, 0.088], [0.012, 0.088], [0.012, 0.105], [-0.012, 0.105], [-0.012, 0.2], [-0.042, 0.2], [-0.042, 0.11], [-0.23, 0.11], [-0.23, 0]],
    )
    parts["mirror-arm-r"]["p0"] = [11.42, 2.58, -1.225]
    put(body, box(parts["dashboard"], "dashboard-plinth", "body", [0.3, 0.38, 1.9], [11.62, 0.52, 0], cornerRadius=0.02), "dashboard")
    put(body, box(parts["driver-seat"], "driver-seat-pedestal", "body", [0.3, 0.48, 0.36], [11.02, 0.57, 0.72], cornerRadius=0.02), "driver-seat-back")
    put(body, like(parts["steering-wheel"], id="steering-wheel-hub", profile=[[-0.02, 0], [-0.02, 0.06], [-0.006, 0.21], [0.006, 0.21], [0.02, 0.06], [0.02, 0]], axis=[-0.5, 0.866, 0], origin=[11.36, 1.2, 0.72]), "steering-wheel")
    tracks(body, parts["rub-rail-r-1"], "charge-flap", 2.385, 2.445, 1.30, 1.2425)
    say(
        volumes["50"],
        0,
        "Every part is carried: each hub has a pilot through the wheel disc, a bolting flange behind it and a barrel that runs on the stub axle or spindle and carries the brake disc; the door leaves hang under a guide rail on the wall over each doorway that spans their travel; the console stands on a plinth, the driver's seat on its pedestal, the steering wheel on a dished hub, the kerb-side mirror arm is fixed in the cant rail.",
        "Jedes Teil ist getragen: jede Nabe hat einen Zentrierbund durch die Radscheibe, dahinter einen Anschraubflansch und einen Nabenkörper, der auf dem Achsschenkel oder Achsstummel läuft und die Bremsscheibe trägt; die Türflügel hängen unter einer Führungsschiene an der Wand über jeder Türöffnung, die ihren Laufweg überspannt; die Konsole steht auf einem Sockel, der Fahrersitz auf seinem Fuß, das Lenkrad auf einer geschüsselten Nabe, der bordsteinseitige Spiegelarm sitzt im Dachlängsträger.",
    )

    body = volumes["100"]["body"]
    parts = by_id(body)
    hubs(
        body,
        [[0.215, 0], [0.205, 0.06], [0.16, 0.08], [0.16, 0.105], [0.148, 0.105], [0.148, 0.2], [0.118, 0.2], [0.118, 0.11], [-0.07, 0.11], [-0.07, 0]],
        [[0.105, 0], [0.105, 0.07], [0.012, 0.088], [0.012, 0.105], [-0.012, 0.105], [-0.012, 0.2], [-0.042, 0.2], [-0.042, 0.11], [-0.23, 0.11], [-0.23, 0]],
    )
    seats(body, seat, "wheelhouse-wall-f-r", 0.28)
    tracks(body, parts["mirror-arm-l"], "mirror-arm-l", 2.38, 2.45, 1.31, 1.2425)
    say(
        volumes["100"],
        0,
        "Hubs keep their flange behind the disc and the barrel that carries the brake disc on the stub axle or spindle; the six spring seats under the wheelhouses take the air springs, and a guide rail over each doorway carries the leaves.",
        "Die Naben behalten ihren Flansch hinter der Scheibe und den Nabenkörper, der die Bremsscheibe auf Achsschenkel oder Achsstummel trägt; die sechs Federsitze unter den Radkästen nehmen die Luftfedern auf, und eine Führungsschiene über jeder Türöffnung trägt die Flügel.",
    )

    body = volumes["200"]["body"]
    parts = by_id(body)
    beam = parts["axle-beam-f"]
    seats(body, seat, "wheelhouse-wall-f-r", 0.28)
    tracks(body, parts["mirror-arm-l"], "mirror-arm-l", 2.37, 2.45, 1.33, 1.24)
    for side, z in (("l", 1), ("r", -1)):
        put(body, like(beam, id=f"stub-axle-f-{side}", kind="capsule", p0=[0, 0, 0.8 * z], p1=[0, 0, 1.0 * z], r0=0.05, r1=0.05), "axle-beam-f")
        put(body, cylinder(beam, f"air-spring-f-{side}", "axle-f", [0, -0.2, 0.7 * z], 0.11, 0.6), f"stub-axle-f-{side}")
        put(body, box(beam, f"spring-carrier-r-{side}", "axle-r", [0.8, 0.07, 0.12], [0, -0.215, 0.47 * z]), "axle-beam-r")
        put(body, cylinder(beam, f"air-spring-r-{side}-a", "axle-r", [-0.34, -0.18, 0.47 * z], 0.11, 0.32), f"spring-carrier-r-{side}")
        put(body, cylinder(beam, f"air-spring-r-{side}-b", "axle-r", [0.34, -0.18, 0.47 * z], 0.11, 0.32), f"air-spring-r-{side}-a")
    say(
        volumes["200"],
        0,
        "The running gear hangs together: stub axles join the front beam to the wheels, one plain air spring cylinder stands on the beam each side at the front and two on a carrier each side at the rear, all under spring seats of the wheelhouses; a guide rail over each doorway carries the leaves.",
        "Das Fahrwerk hängt zusammen: Achsschenkel verbinden den Vorderachsträger mit den Rädern, vorn steht je Seite ein glatter Luftfederzylinder auf dem Träger, hinten stehen je Seite zwei auf einem Federträger, alle unter Federsitzen der Radkästen; eine Führungsschiene über jeder Türöffnung trägt die Flügel.",
    )

    body = volumes["500"]["body"]
    parts = by_id(body)
    beam = parts["axle-beam-f"]
    section = dict(beam["sections"][0])
    middle = [s for s in beam["sections"] if abs(s["at"][2]) < 0.9]
    beam["sections"] = [dict(section, at=[0, 0, -0.95]), *middle, dict(section, at=[0, 0, 0.95])]
    rear = parts["axle-beam-r"]
    rear["sections"] = [dict(rear["sections"][0], at=[0, -0.278, -0.68]), dict(rear["sections"][0], at=[0, -0.278, 0.68])]
    for side, z in (("l", 1), ("r", -1)):
        put(body, cylinder(beam, f"air-spring-f-{side}", "axle-f", [0, -0.2, 0.7 * z], 0.11, 0.8), "axle-beam-f")
        put(body, cylinder(beam, f"air-spring-r-{side}", "axle-r", [0, -0.2, 0.47 * z], 0.11, 0.8), "axle-beam-r")
    put(body, box(parts["mirror-l"], "mirror-arm-l", "body", [0.33, 0.08, 0.1], [11.56, 2.0, 1.28]), "mirror-l")
    put(body, box(parts["mirror-r"], "mirror-arm-r", "body", [0.41, 0.08, 0.1], [11.6, 2.05, -1.28]), "mirror-r")
    say(
        volumes["500"],
        0,
        "The axle beams run into the wheel discs, one suspension strut each side stands on each beam up into the wheelhouse arch, and each mirror ear sits on a short arm from the window frame, so that nothing floats.",
        "Die Achsträger laufen in die Radscheiben, je Seite steht ein Federbein auf jedem Träger bis in den Radkastenbogen, und jedes Spiegelohr sitzt an einem kurzen Arm am Fensterrahmen, sodass nichts schwebt.",
    )

    with open(SOURCE, "w", encoding="utf8") as handle:
        json.dump(document, handle, ensure_ascii=False, indent=1)
        handle.write("\n")


if __name__ == "__main__":
    main()
