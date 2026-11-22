"""🖥️ Desktop monitor: what the back of the real object carries is added by hand to the sound panel, hump and stand: the port bay on the lower back of the hump, power and video plugs with their boots, both cables hanging to a clamp on the riser (skinned between panel and riser so the tilt opens no crack), running down to the base plate and off its rear edge, the menu buttons under the chin; the 1:50 drawings follow (the plug outline is the one r3-project2d measures: the boot outside the hump, edge on edge with the housing shape)."""
import json

from rework_16a_lib import box, capsule, band, of, shape, poly, rect, run, source_path

SLUG = "desktop-monitor"
PORT, BOOT_END = (-0.137, -0.012), (-0.1485, 0.0135)
SIDE = 0.038
HANG = [(0.0939, 0.0005), (0.078, 0.004), (0.062, 0.007), (0.046, 0.008)]
DOWN, BACK = (0.004, 0.008), (0.004, 0.030)
R = 0.00375
MINE = ("port-bay", "menu-buttons", "plug-", "cable-")


def cable(name, w):
    """🔌 One plug with its boot and its cable: three skinned links from the boot to the clamp, then rigid on the riser."""
    parts = [capsule(f"plug-{name}", "panel", "housing-plastic", [PORT[0], PORT[1], w], [BOOT_END[0], BOOT_END[1], w], 0.0055, role="detail", tone=0.15)]
    for k, share in enumerate((0.75, 0.5, 0.25)):
        a, b = HANG[k], HANG[k + 1]
        parts.append(capsule(f"cable-{name}-hang-{k + 1}", "neck", "stand-plastic", [a[0], a[1], w], [b[0], b[1], w], R, role="detail", tone=0.15, weights=[{"bone": "panel", "weight": share}, {"bone": "neck", "weight": 1 - share}]))
    parts.append(capsule(f"cable-{name}-drop", "neck", "stand-plastic", [HANG[-1][0], HANG[-1][1], w], [DOWN[0], DOWN[1], w], R, role="detail", tone=0.15))
    parts.append(capsule(f"cable-{name}-run", "neck", "stand-plastic", [DOWN[0], DOWN[1], w], [BACK[0], BACK[1], w], R, role="detail", tone=0.15))
    return parts


def additions():
    return [
        box("port-bay", "panel", "bezel-plastic", [0.020, 0.0012, 0.150], [0.0, 0.0, 0.0], role="texture", tone=0.1, texture=True, transform={"rotation": [0.0, 0.0, 0.2105, 0.9776], "translation": [-0.137, -0.0115, 0.0]}),
        box("menu-buttons", "panel", "bezel-plastic", [0.003, 0.006, 0.010], [-0.1705, -0.035, -0.215], role="texture", tone=0.35, texture=True, repeat={"count": 5, "translation": [0.0, 0.0, 0.016]}),
        *cable("power", -SIDE),
        *cable("video", SIDE),
        box("cable-clamp", "neck", "stand-plastic", [0.012, 0.030, 0.090], [0.046, 0.008, 0.0], role="detail", tone=0.3, radius=0.004),
    ]


def volumes():
    doc = json.load(open(source_path(SLUG), encoding="utf-8"))["volumes"]
    v = doc["50"]
    for name in ("body", "thermal", "structure"):
        v[name] = [s for s in v[name] if not s["id"].replace("th-", "").replace("member-", "").startswith(MINE)]
    new = additions()
    v["body"] += new
    for s in new:
        if s.get("texture"):
            continue
        region = ("housing", "housing-shell") if s["id"].startswith("plug-") else ("stand", "stand-body")
        v["thermal"].append(of("th-" + s["id"], s["id"], region=region[0], layer=region[1]))
    v["structure"].append(of("member-cable-clamp", "cable-clamp", member="neck", part="member"))
    v["decisions"] = [d for d in v["decisions"] if not d["en"].startswith("Back and cables:")]
    v["decisions"].append({"en": "Back and cables: the port bay is a dark strip on the lower back of the hump; a power and a video plug sit in it 76 mm apart with 11 mm boots; each 7.5 mm cable hangs in three links from its boot to the clamp on the riser 46 mm above the plate, the links weighted between panel and riser so that tilting bends the cable instead of opening a crack, then drops beside the riser to the base plate and runs to its rear edge, where it is cut; five menu buttons sit under the right end of the chin.", "de": "Rückseite und Kabel: Das Anschlussfeld ist ein dunkler Streifen am unteren Rücken des Buckels; darin stecken im Abstand von 76 mm ein Netz- und ein Videostecker mit 11-mm-Tüllen; jedes 7,5-mm-Kabel hängt in drei Gliedern von seiner Tülle zur Klemme am Hals 46 mm über der Platte, die Glieder zwischen Panel und Hals gewichtet, sodass das Neigen das Kabel biegt statt einen Spalt zu öffnen, fällt dann neben dem Hals auf die Fußplatte und läuft zu deren Hinterkante, wo es endet; fünf Menütasten sitzen unter dem rechten Ende des Kinns."})
    return doc


def mutate(asset):
    geometry = asset["representations"]["50"]["geometry"]
    for view in geometry.values():
        view[:] = [s for s in view if not s["id"].startswith(("plug", "cable"))]
    path = [(u, v, R, 0) for u, v in HANG + [DOWN]]
    boot = [[-0.13234, -0.00878], [-0.14376, 0.01669], [-0.1492, 0.01904], [-0.15401, 0.01241], [-0.14276, -0.01493]]
    geometry["elevation"] += [
        shape("plug", "panel", "body", 3.3, 0.15, "housing", "housing-plastic", **poly(boot)),
        shape("cable-hang", "neck", "body", 3.25, 0.15, "stand", "stand-plastic", **poly(band(path))),
        shape("cable-run", "neck", "body", 3.25, 0.15, "stand", "stand-plastic", **rect(DOWN[0] - R, DOWN[1] - R, 2 * R, BACK[1] - DOWN[1] + 2 * R, R)),
        shape("cable-clamp", "neck", "body", 3.3, 0.3, "stand", "stand-plastic", **rect(0.040, -0.007, 0.012, 0.030, 0.003)),
    ]
    x0 = -0.06 - BACK[1] - R
    geometry["plan"] += [
        shape("cable-run-power", "neck", "body", 3.15, 0.15, "stand", "stand-plastic", **rect(x0, -SIDE - R, BACK[1] - DOWN[1] + 2 * R, 2 * R, R)),
        shape("cable-run-video", "neck", "body", 3.15, 0.15, "stand", "stand-plastic", **rect(x0, SIDE - R, BACK[1] - DOWN[1] + 2 * R, 2 * R, R)),
        shape("cable-clamp", "neck", "body", 3.18, 0.3, "stand", "stand-plastic", **rect(-0.083, -0.045, 0.030, 0.090, 0.004)),
    ]
    text = asset["representations"]["50"]["decisions"]
    text["retained"] = {"en": text["retained"]["en"].split(" Also retained:")[0] + " Also retained: the two plugs under the hump, their cables hanging to the clamp on the riser and running over the base plate to its rear edge.", "de": text["retained"]["de"].split(" Ebenfalls erhalten:")[0] + " Ebenfalls erhalten: die zwei Stecker unter dem Buckel, ihre Kabel zur Klemme am Hals und über die Fußplatte bis zu deren Hinterkante."}
    text["omitted"] = {"en": "The port sockets themselves, the menu buttons, the cables beyond the base plate and the VESA screws, which are hidden or below the scale. The recessed glass is hidden by the bezel in the side view; hinge, bracket, housing, plugs and cables are hidden behind the panel in the front view.", "de": "Die Anschlussbuchsen selbst, die Menütasten, die Kabel jenseits der Fußplatte und die VESA-Schrauben, die verdeckt oder unter dem Maßstab liegen. Das vertiefte Glas ist in der Seitenansicht vom Rahmen verdeckt; Gelenk, Halter, Gehäuse, Stecker und Kabel liegen in der Vorderansicht hinter dem Panel."}


run(SLUG, mutate, volumes())
