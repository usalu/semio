"""🚰 Pass 1 (second round), stage 2 of objects/drinking-fountain: two tie rods on cast lugs, drain and supply inside, riser rib on the back of the capital, 2D follows."""
import json, math, pathlib, sys

HERE = pathlib.Path(__file__).parent
SOURCE = HERE / "volumes/objects/drinking-fountain.json"
CATALOG = pathlib.Path(sys.argv[1])
SEAT = [[0.735, 0.056], [0.735, 0.04], [0.723, 0.04], [0.723, 0.056]]
SEAT_100 = [[0.73, 0.056], [0.73, 0.04], [0.715, 0.04], [0.715, 0.056]]
HALF_TURN = {"count": 2, "rotationAxis": [1, 0, 0], "rotationRad": 3.141593, "origin": [0, 0, 0]}
ROD_W = 0.042
RIB = {"50": [[0.742, 0.078], [0.75, 0.096], [0.772, 0.11], [0.8085, 0.118], [0.805, 0.106], [0.797, 0.09], [0.79, 0.078]],
       "100": [[0.742, 0.078], [0.75, 0.096], [0.772, 0.11], [0.8085, 0.118], [0.797, 0.09], [0.79, 0.078]],
       "200": [[0.742, 0.078], [0.772, 0.11], [0.8085, 0.118], [0.79, 0.078]]}
DROPS = {"drop-0": ("\"cx\": 0.14374,\n              \"cy\": -0.065694,\n              \"rx\": 0.005,", "\"cx\": 0.150202,\n              \"cy\": -0.077009,\n              \"rx\": 0.004,"),
         "drop-1": ("\"cx\": 0.118311,\n              \"cy\": -0.080505,", "\"cx\": 0.144341,\n              \"cy\": -0.08247,")}


def hexagon(v, w, r):
    """🔩 Hexagon across corners 2 r about a centre in the v, w plane."""
    return [[round(v + r * math.cos(k * math.pi / 3), 6), round(w + r * math.sin(k * math.pi / 3), 6)] for k in range(6)]


def cast(ident, role, tone, **rest):
    """🧱 Cast-iron solid on the pedestal bone."""
    return {"id": ident, "bone": "pedestal", "material": "cast-iron", "role": role, "tone": tone, **rest}


def rib(scale):
    """🦴 Riser rib of a scale: a 40 mm console in the sagittal plane."""
    return cast("riser-rib", "body", 0.6, kind="extrude", profile=RIB[scale], plane="xy", **{"from": -0.02, "to": 0.02})


def without(solids, idents):
    """🧹 List without the named solids."""
    return [s for s in solids if s["id"] not in idents]


def place_before(solids, ident, parts):
    """📌 Insert solids before a named solid."""
    at = next(i for i, s in enumerate(solids) if s["id"] == ident)
    return solids[:at] + parts + solids[at:]


def say(decisions, index, old, new, add):
    """✍️ Replace and append text of one decision, English and German."""
    for language in ("en", "de"):
        text = decisions[index][language]
        if old and old[language] not in text:
            raise ValueError(old[language])
        text = text.replace(old[language], new[language]) if old else text
        decisions[index][language] = text + (add[language] if add[language] not in text else "")


document = json.loads(SOURCE.read_text())
volumes = document["volumes"]

fine = volumes["50"]
gone = ("tie-rod", "tie-nut-foot", "tie-nut-top", "tie-yoke", "tie-lug", "drain-pipe", "supply-pipe", "riser-rib")
fine["body"] = without(fine["body"], gone)
for solid in (next(s for s in fine["body"] if s["id"] == "capital"), next(s for s in fine["structure"] if s["id"] == "member-pedestal")):
    solid["profile"] = [p for p in solid["profile"] if p not in SEAT]
rod = cast("tie-rod", "detail", 0.3, kind="revolve", profile=[[0.014, 0.0], [0.014, 0.006], [0.755, 0.006], [0.755, 0.0]], axis=[1, 0, 0], origin=[0.0, 0.0, ROD_W], slender=True, repeat=HALF_TURN)
nut = lambda ident, a, b: cast(ident, "detail", 0.3, kind="extrude", profile=hexagon(0.0, ROD_W, 0.011), plane="yz", **{"from": a, "to": b}, repeat=HALF_TURN)
lug = cast("tie-lug", "detail", 0.55, kind="box", size=[0.012, 0.03, 0.026], transform={"translation": [0.735, 0.0, 0.045]}, repeat=HALF_TURN)
drain = cast("drain-pipe", "detail", 0.4, kind="revolve", profile=[[0.014, 0.0], [0.014, 0.016], [0.767, 0.016], [0.767, 0.0]], axis=[1, 0, 0], origin=[0.0, 0.0, 0.0])
drain["material"] = "stainless-basin"
supply = {"id": "supply-pipe", "kind": "loft", "bone": "pedestal", "material": "chrome-brass", "role": "detail", "tone": 0.65, "slender": True, "capStart": True, "capEnd": True,
          "sections": [{"at": [u, v, 0.0], "rx": 0.0075, "ry": 0.0075} for u, v in
                       ((0.014, 0.035), (0.74, 0.035), (0.757, 0.04), (0.77, 0.055), (0.78, 0.075), (0.79, 0.092), (0.798, 0.099), (0.8045, 0.1))]}
fine["body"] = place_before(fine["body"], "button-boss", [rib("50"), lug, rod, nut("tie-nut-foot", 0.016, 0.026), nut("tie-nut-top", 0.741, 0.751), drain, supply])
fine["structure"] = without(fine["structure"], ("member-tie-rod", "joint-tie-nut-foot", "joint-tie-nut-top", "joint-tie-yoke", "joint-tie-lug") + gone)
fine["structure"] = place_before(fine["structure"], "joint-anchor-stud", [
    {"id": "member-tie-rod", "of": "tie-rod", "member": "pedestal", "part": "member"},
    {"id": "joint-tie-lug", "of": "tie-lug", "member": "pedestal", "part": "joint"},
    {"id": "joint-tie-nut-foot", "of": "tie-nut-foot", "member": "pedestal", "part": "joint"},
    {"id": "joint-tie-nut-top", "of": "tie-nut-top", "member": "pedestal", "part": "joint"}])
fine["thermal"] = place_before(without(fine["thermal"], ("th-riser-rib",)), "th-button-boss", [{"id": "th-riser-rib", "of": "riser-rib", "region": "pedestal-cast", "layer": "casting"}])

mid = volumes["100"]
mid["body"] = place_before(without(mid["body"], ("riser-rib", "drain-pipe")), "bezel", [rib("100"), {**drain, "profile": [[0.014, 0.0], [0.014, 0.016], [0.781, 0.016], [0.781, 0.0]]}])
mid["thermal"] = place_before(without(mid["thermal"], ("th-riser-rib",)), "th-basin", [{"id": "th-riser-rib", "of": "riser-rib", "region": "pedestal-cast", "layer": "casting"}])
frame = next(s for s in mid["structure"] if s["id"] == "member-pedestal")
frame["profile"] = [p for p in frame["profile"] if p not in SEAT_100]
mid["structure"] = without(mid["structure"], ("tie-rod", "tie-yoke", "joint-tie-lug"))
mid["structure"] = place_before(mid["structure"], "member-basin", [
    {**rod, "member": "pedestal", "part": "member"},
    {**cast("joint-tie-lug", "detail", 0.55, kind="box", size=[0.015, 0.03, 0.026], transform={"translation": [0.7375, 0.0, 0.045]}, repeat=HALF_TURN), "member": "pedestal", "part": "joint"}])

coarse = volumes["200"]
coarse["body"] = place_before(without(coarse["body"], ("riser-rib",)), "button", [rib("200")])
coarse["thermal"] = place_before(without(coarse["thermal"], ("th-riser-rib",)), "th-basin", [{"id": "th-riser-rib", "of": "riser-rib", "region": "pedestal-cast", "layer": "casting"}])

say(fine["decisions"], 0, None, None, {
    "en": " Inside the shells: two M12 tie rods from tapped bosses in the base plate through cast lugs in the capital with a nut on each, the 32 mm drain on the axis from the waste to the base, and the 15 mm supply, which leaves the bore through a cored port into the riser rib, a 40 mm cast console on the back of the capital that carries it under the bowl to the goose-neck. The two splash beads sit on the bowl where the jet lands.",
    "de": " In den Hohlkörpern: zwei M12-Zuganker aus Gewindeaugen der Bodenplatte durch Gusslaschen im Kapitell mit je einer Mutter, der 32-mm-Ablauf auf der Achse vom Ablaufventil zum Fuß und der 15-mm-Zulauf, der die Bohrung durch einen gekernten Kanal in die Steigrippe verlässt, eine 40 mm breite Gusskonsole an der Rückseite des Kapitells, die ihn unter dem Becken zum Schwanenhals führt. Die zwei Spritzperlen sitzen auf dem Becken, wo der Strahl auftrifft."})
say(fine["decisions"], 2,
    {"en": "clamped by a 16 mm tie rod between the floor nut and the yoke under the cup", "de": "verspannt durch einen 16-mm-Zuganker zwischen Bodenmutter und Joch unter der Schale"},
    {"en": "clamped by two M12 tie rods between the base plate and cast lugs in the capital", "de": "verspannt durch zwei M12-Zuganker zwischen Bodenplatte und Gusslaschen im Kapitell"}, {"en": "", "de": ""})
say(mid["decisions"], 0, None, None, {
    "en": " The riser rib on the back of the capital and the 32 mm drain on the axis are kept; tie rods, lugs and the supply are below 15 mm and left out of the body.",
    "de": " Die Steigrippe an der Rückseite des Kapitells und der 32-mm-Ablauf auf der Achse bleiben; Zuganker, Laschen und Zulauf liegen unter 15 mm und entfallen im Körper."})
say(mid["decisions"], 2, {"en": "tie rod and yoke", "de": "Zuganker und Joch"}, {"en": "two tie rods at their true section on cast lugs", "de": "zwei Zuganker im wahren Querschnitt auf Gusslaschen"}, {"en": "", "de": ""})
say(coarse["decisions"], 0, None, None, {
    "en": " The riser rib stays as a plain wedge under the bowl at the back.",
    "de": " Die Steigrippe bleibt als einfacher Keil hinten unter dem Becken."})
SOURCE.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n")

text = CATALOG.read_text(encoding="utf-8")
for old, new in DROPS.values():
    if old in text:
        text = text.replace(old, new, 1)
marker = "\"elevation\": [\n"
cuts = []
at = text.index("\"representations\"")
for _ in range(4):
    at = text.index(marker, at) + len(marker)
    cuts.append(at)
for scale, cut in sorted(zip(("500", "200", "100", "50"), cuts), key=lambda pair: -pair[1]):
    if scale not in RIB or "\"id\": \"riser-rib\"" in text[cut:cut + 200]:
        continue
    shape = {"id": "riser-rib", "bone": "pedestal", "role": "body", "z": 3.19, "filled": True, "tone": 0.6, "kind": "poly", "points": RIB[scale], "closed": True, "region": "pedestal-cast", "material": "cast-iron"}
    block = "\n".join(" " * 12 + line for line in json.dumps(shape, ensure_ascii=False, indent=2).split("\n")) + ",\n"
    text = text[:cut] + block + text[cut:]
json.loads(text)
CATALOG.write_text(text, encoding="utf-8")
print("[DEBUG] stage 2 written")
