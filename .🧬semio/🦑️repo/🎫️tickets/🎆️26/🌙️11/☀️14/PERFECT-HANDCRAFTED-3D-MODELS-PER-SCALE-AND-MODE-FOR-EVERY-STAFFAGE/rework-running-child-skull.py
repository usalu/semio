"""💀️ Running child: the skull of a six-year-old at 1:50 and 1:100, the shoulder line of the 2D drawings and the structure texts.

Usage: python3 rework-running-child-skull.py     writes the volumes source and the member `representations` of the catalog file

Skull in the head frame (u up from the atlanto-occipital joint, v back, w left): large neurocranium with frontal bulge and flat base, small face
(maxilla narrowing to the nasal bridge so that the orbits stay open between brow, lateral rim and cheek bone), zygomatic arch to the temporal bone,
mandible as one arch from condyle over the angle to the chin. Run after `rework-running-child-girdles.py`; idempotent.
"""
import copy
import importlib.util
import json
import os

T = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("girdles", os.path.join(T, "rework-running-child-girdles.py"))
girdles = importlib.util.module_from_spec(spec)
spec.loader.exec_module(girdles)
Q = [0, -0.70711, 0, 0.70711]


def part(id, kind, **fields):
    return {"id": id, "kind": kind, "bone": "head", "material": "bone", "role": "body", "tone": 0.85, **fields, "member": "head", "part": "member"}


def ball(id, at, radii):
    return part(id, "ellipsoid", radii=list(radii), transform={"translation": list(at)})


def bar(id, p0, p1, r0, r1=None):
    return part(id, "capsule", p0=list(p0), p1=list(p1), r0=r0, r1=r0 if r1 is None else r1)


def pair(id, p0, p1, r0, r1=None):
    flip = lambda p: (p[0], p[1], -p[2])
    return [bar(id + "-l", p0, p1, r0, r1), bar(id + "-r", flip(p0), flip(p1), r0, r1)]


def skull(sc):
    f = girdles.FLOOR[sc]
    condyle, angle, chin = (0.006, -0.008, 0.044), (-0.022, -0.018, 0.034), (-0.044, -0.064, 0.010)
    cheek, arch = (0.008, -0.060, 0.026), (0.014, -0.032, 0.054)
    out = [ball("sk-cranium", (0.068, 0.006, 0), (0.062, 0.082, 0.064)), ball("sk-skull-base", (0.010, -0.004, 0), (0.012, 0.050, 0.050))]
    if sc == 50:
        out.append(ball("sk-frontal", (0.062, -0.040, 0), (0.034, 0.038, 0.054)))
        rows = [(-0.012, -0.060, 0.022, 0.014), (0.006, -0.058, 0.030, 0.018), (0.016, -0.059, 0.019, 0.016), (0.030, -0.060, 0.009, 0.012)]
    else:
        rows = [(-0.012, -0.058, 0.024, 0.016), (0.006, -0.056, 0.032, 0.022), (0.034, -0.058, 0.030, 0.020)]
    out.append(part("sk-face", "loft", sections=[{"at": [u, v, 0], "rx": rx, "ry": ry, "exponent": 2.4, "rotation": Q} for u, v, rx, ry in rows], capStart=True, capEnd=True))
    if sc == 50:
        out.append(bar("sk-nasal", (0.036, -0.070, 0), (0.014, -0.074, 0), 0.005, 0.0045))
        out += pair("sk-brow", (0.036, -0.068, 0.006), (0.034, -0.058, 0.044), 0.0045)
        out += pair("sk-orbital-rim", (0.034, -0.058, 0.044), (0.010, -0.058, 0.034), 0.0042)
    out += pair("sk-zygoma", cheek, arch, max(0.006, f), max(0.0045, f))
    out += pair("sk-zygomatic-arch", arch, (0.012, 0.004, 0.054), max(0.004, f))
    out += pair("sk-temporal", (0.012, 0.004, 0.054), (0.030, 0.010, 0.050), max(0.007, f))
    out += pair("sk-mandible-ramus", condyle, angle, max(0.005, f))
    out += pair("sk-mandible-body", angle, chin, max(0.0052, f), max(0.006, f))
    out.append(bar("sk-mandible-chin", (chin[0], chin[1] - 0.002, -0.010), (chin[0], chin[1] - 0.002, 0.010), max(0.006, f)))
    return out


SLEEVE_SIDE = {
    "50": [[0.10464, -0.03648], [0.10438, 0.04072], [0.012, 0.0395], [-0.008, 0.036], [-0.026, 0.028], [-0.034, 0.018], [-0.038, 0.006], [-0.038, -0.007], [-0.034, -0.019], [-0.026, -0.028], [-0.008, -0.0355], [0.012, -0.0375]],
    "100": [[0.10432, -0.03554], [0.10408, 0.03973], [0.012, 0.0393], [-0.012, 0.035], [-0.03, 0.024], [-0.038, 0.006], [-0.038, -0.008], [-0.03, -0.024], [-0.012, -0.0345], [0.012, -0.0372]],
    "200": [[0.10369, -0.03443], [0.1021, 0.03923], [0.0116, 0.03975], [-0.022, 0.03], [-0.038, 0.008], [-0.038, -0.008], [-0.022, -0.03], [0.0141, -0.038]],
    "500": [[0.10183, -0.03013], [0.09783, 0.0335], [-0.034, 0.0233], [-0.028, -0.0312]],
}
SLEEVE_TURNED = {
    "50": [[-0.038, 0.0014], [-0.038, 0.009], [-0.03, 0.0255], [-0.012, 0.0345], [0.012, 0.0375], [0.105, 0.0384], [0.105, 0.0014]],
    "100": [[-0.038, 0.0014], [-0.038, 0.009], [-0.03, 0.0255], [-0.008, 0.0355], [0.105, 0.0384], [0.105, 0.0014]],
    "200": [[-0.038, 0.0014], [-0.038, 0.009], [-0.03, 0.0255], [-0.008, 0.0355], [0.105, 0.0384], [0.105, 0.0014]],
    "500": [[-0.038, 0.0012], [-0.038, 0.0099], [-0.006, 0.036], [0.105, 0.0382], [0.105, 0.0012]],
}


def draw(representations):
    """✏️ The shoulder of the drawings follows the body: sleeve cap up to the acromion, shirt 16 mm broader at the shoulder line."""
    for scale, representation in representations.items():
        geometry = representation["geometry"]
        for view in ("elevation", "front", "plan"):
            for shape in geometry[view]:
                if shape["id"] in ("sleeve-l", "sleeve-r") and shape.get("kind") == "poly":
                    shape["points"] = copy.deepcopy((SLEEVE_SIDE if view == "elevation" else SLEEVE_TURNED)[scale])
                if shape["id"] == "shirt" and view == "front":
                    shape["points"] = [[x, (0.1 if y > 0 else -0.1)] if x == 0.262 and abs(abs(y) - 0.092) < 1e-9 else [x, y] for x, y in shape["points"]]


SHAFT = {"sk-femur": 0.009, "sk-tibia": 0.0087, "sk-humerus": 0.0075}
TEXT = {
    "50": ("Structure 1:50: the skeleton of a six-year-old as one connected load path, each girdle on its own bone of the rig. Skull: large neurocranium with frontal bulge and flat base, small face with open orbits between brow, lateral rim and cheek bone, zygomatic arch to the temporal bone, mandible as one arch from condyle to chin. Column with its curves and spinous processes, twelve rib pairs to sternum, costal arch or free. Shoulder girdle on the clavicle bones (sternoclavicular to acromioclavicular joint, 99 mm, biacromial 0.25 m): S-curved clavicle, scapula blade on the back of the ribcage with spine, acromion, coracoid and glenoid that carries the humeral head. Pelvic girdle on the hip bones (sacroiliac joint to hip joint): ilium blade with crest from the posterior to the anterior superior spine, acetabulum, pubic and ischial rami meeting at the symphysis, sacrum on the pelvis bone. Femur with head, neck and trochanter, patella, tibia, fibula, tarsus, metatarsals, toes, humerus, ulna, radius, carpus, metacarpus and phalanges. The backpack panel hangs by its two straps from the clavicles; the feet stand on their contact anchors.",
           "Tragwerk 1:50: das Skelett eines Sechsjährigen als ein zusammenhängender Lastpfad, jeder Gürtel auf eigenen Knochen des Rigs. Schädel: grosser Hirnschädel mit Stirnwölbung und flacher Basis, kleines Gesicht mit offenen Augenhöhlen zwischen Brauenbogen, seitlichem Rand und Jochbein, Jochbogen zum Schläfenbein, Unterkiefer als ein Bogen vom Gelenkkopf bis zum Kinn. Wirbelsäule mit ihren Krümmungen und Dornfortsätzen, zwölf Rippenpaare zum Brustbein, zum Rippenbogen oder frei. Schultergürtel auf den Schlüsselbeinknochen (Brustbein-Schlüsselbein-Gelenk bis Schultereckgelenk, 99 mm, Schulterhöhenbreite 0,25 m): S-förmiges Schlüsselbein, Schulterblatt auf der Rückseite des Brustkorbs mit Gräte, Schulterhöhe, Rabenschnabelfortsatz und der Pfanne, die den Oberarmkopf trägt. Beckengürtel auf den Hüftbeinknochen (Kreuzbein-Darmbein-Gelenk bis Hüftgelenk): Darmbeinschaufel mit Kamm vom hinteren zum vorderen oberen Darmbeinstachel, Hüftpfanne, Scham- und Sitzbeinäste bis zur Schambeinfuge, Kreuzbein auf dem Beckenknochen. Oberschenkelknochen mit Kopf, Hals und Rollhügel, Kniescheibe, Schienbein, Wadenbein, Fusswurzel, Mittelfuss, Zehen, Oberarmknochen, Elle, Speiche, Handwurzel, Mittelhand und Fingerglieder. Die Rucksackplatte hängt mit ihren zwei Riemen an den Schlüsselbeinen; die Füsse stehen auf ihren Kontaktankern."),
    "100": ("Structure 1:100: the same connected skeleton for a 15 mm feature, girdles on the clavicle and hip bones of the rig: skull of cranium, base, face block, cheek bones and mandible arch; six rib pairs, no spinous processes; clavicle in two bars, scapula blade with spine and glenoid; ilium blade with crest in two bars, acetabulum, pubis, ischium and ramus; two metatarsal rays per foot, one finger blade beside the thumb; pack panel and straps kept.",
            "Tragwerk 1:100: dasselbe zusammenhängende Skelett für 15 mm Mindestmass, die Gürtel auf den Schlüsselbein- und Hüftbeinknochen des Rigs: Schädel aus Hirnschädel, Basis, Gesichtsblock, Jochbeinen und Unterkieferbogen; sechs Rippenpaare, keine Dornfortsätze; Schlüsselbein in zwei Stäben, Schulterblatt mit Gräte und Pfanne; Darmbeinschaufel mit Kamm in zwei Stäben, Hüftpfanne, Schambein, Sitzbein und Ast; zwei Mittelfussstrahlen je Fuss, eine Fingerfläche neben dem Daumen; Rucksackplatte und Riemen bleiben."),
    "200": ("Structure 1:200: the load path in members of 30 mm: skull with jaw, column, ribcage basket, per side one clavicle from sternum to shoulder top with the scapula strut and the strut to the shoulder joint on the clavicle bone, pelvis basin with one hip strut from the sacroiliac joint to the hip joint on the hip bone, one member per limb bone with femur, tibia and humerus at their true shaft of 18, 17 and 15 mm, pack panel on its straps.",
            "Tragwerk 1:200: der Lastpfad in Gliedern von 30 mm: Schädel mit Kiefer, Wirbelsäule, Brustkorb als Korb, je Seite ein Schlüsselbein vom Brustbein zur Schulterhöhe mit Schulterblattstrebe und Strebe zum Schultergelenk auf dem Schlüsselbeinknochen, Beckenschale mit einer Hüftstrebe vom Kreuzbein-Darmbein-Gelenk zum Hüftgelenk auf dem Hüftbeinknochen, ein Glied je Gliedmassenknochen, Oberschenkel-, Schien- und Oberarmknochen mit ihrem wahren Schaft von 18, 17 und 15 mm, Rucksackplatte an ihren Riemen."),
    "500": ("Structure 1:500: the members of 1:200 at the printable thickness of this scale, the girdle struts still on the clavicle and hip bones of the rig; femur, tibia and humerus keep their true shaft and are not thickened.",
            "Tragwerk 1:500: die Glieder von 1:200 in der druckbaren Stärke dieses Massstabs, die Gürtelstreben weiterhin auf den Schlüsselbein- und Hüftbeinknochen des Rigs; Oberschenkel-, Schien- und Oberarmknochen behalten ihren wahren Schaft und werden nicht verdickt."),
}


def main():
    source = json.load(open(girdles.SOURCE, encoding="utf8"))
    for sc in (50, 100):
        solids = source["volumes"][str(sc)]["structure"]
        at = next(i for i, s in enumerate(solids) if s["bone"] == "head")
        rest = [s for s in solids if not (s["bone"] == "head" and s["part"] == "member")]
        source["volumes"][str(sc)]["structure"] = rest[:at] + skull(sc) + rest[at:]
        print(f"[DEBUG] 1:{sc} structure {len(source['volumes'][str(sc)]['structure'])}")
    for sc in ("200", "500"):
        for solid in source["volumes"][sc]["structure"]:
            if solid["id"][:-2] in SHAFT:
                solid.update(r0=SHAFT[solid["id"][:-2]], r1=SHAFT[solid["id"][:-2]], slender=True)
    for scale, (en, de) in TEXT.items():
        decisions = source["volumes"][scale]["decisions"]
        source["volumes"][scale]["decisions"] = [d for d in decisions if not d["en"].startswith("Structure ")] + [{"en": en, "de": de}]
    json.dump(source, open(girdles.SOURCE, "w", encoding="utf8"), ensure_ascii=False, indent=1)
    open(girdles.SOURCE, "a", encoding="utf8").write("\n")
    document = json.load(open(girdles.CATALOG, encoding="utf8"))
    before = copy.deepcopy(document["asset"]["representations"])
    draw(document["asset"]["representations"])
    if before != document["asset"]["representations"]:
        scope = {}
        code = open(os.path.join(T, "volumes-insert.py"), encoding="utf8").read()
        exec(compile(code[: code.index("\ndef main(")], "volumes-insert.py", "exec"), scope)
        text = open(girdles.CATALOG, encoding="utf8").read()
        text = girdles.splice(text, "representations", before, document["asset"]["representations"], scope["dump"])
        open(girdles.CATALOG, "w", encoding="utf8").write(text)
        print("[DEBUG] representations written")


if __name__ == "__main__":
    main()
