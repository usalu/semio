"""🧾 standing-adult: assembles the body, thermal and structure lists of every scale, the thermal layers and outlets, and the decisions."""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), os.path.join(HERE, f"{name}.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


body = module("rework-01-standing-adult-body")
skeleton = module("rework-01-standing-adult-skeleton")
HX, HY = body.HX, body.HY
REGION = [
    (("brow-",), "hair", "hair"), (("hair", "head"), "hair", None),
    (("face", "nose", "lips", "chin", "ear-", "eye-", "neck"), "face-skin", None),
    (("hand-", "thenar-", "hypothenar-", "skin-"), "hand-skin", None),
    (("hem-rib", "hoodie", "hood", "kangaroo-pocket", "collar-rib", "drawstring-", "sleeve-", "forearm-", "cuff-rib-"), "hoodie", "sweatshirt"),
    (("jeans-seat", "buttock-", "thigh-", "kneecap-", "shank-"), "jeans", "jeans"),
    (("sock-",), "sneakers", "socks"),
    (("sole-", "shoe-", "toe-sole-", "toe-cap-", "lace-"), "sneakers", "sneaker"),
]
TEE_M, BRIEFS_M, SOCK_M = 0.004, 0.0012, 0.004


def region_of(sid):
    for prefixes, region, layer in REGION:
        if any(sid == p or sid.startswith(p) for p in prefixes):
            return region, layer
    raise ValueError(f"no thermal region for {sid}")


def references(solids, whole_head=False):
    out = []
    for solid in solids:
        region, layer = ("face-skin", None) if whole_head and solid["id"] == "head" else region_of(solid["id"])
        ref = {"id": f"th-{solid['id']}", "of": solid["id"], "region": region}
        if layer:
            ref["layer"] = layer
        out.append(ref)
    return out


def outlets(k, nose, mouth, plume):
    out = [k.ellipsoid("th-nose-breath", "head", "skin", 0.1, [HX + 0.107, HY - 0.011, 0], nose, role="detail", region="face-skin", tags=["emitter", "nose-breath"])]
    out.append(k.ellipsoid("th-mouth-breath", "head", "skin", 0.1, [HX + 0.101, HY - 0.031, 0], mouth, role="detail", region="face-skin", tags=["emitter", "mouth-breath"]))
    out.append(k.ellipsoid("th-body-plume", "head", "hair", 0.5, [HX - 0.004, HY + 0.153, 0], plume, role="detail", region="hoodie", tags=["emitter", "body-plume"]))
    return out


def thermal50(k, solids):
    out = references(solids)
    inner = dict(role="body")
    out.append(k.loft("th-tee", "torso", "cotton-underwear", 0.8, body.hrows(body.HOODIE, -TEE_M), n=2, weights=body.HOODIE_WEIGHTS, region="hoodie", layer="tee", **inner))
    out.append(k.loft("th-briefs", "pelvis", "cotton-underwear", 0.8, body.hrows(body.SEAT[1:], -BRIEFS_M), n=2, region="jeans", layer="briefs", **inner))
    for side, s in body.SIDES:
        out.append(k.loft(f"th-tee-sleeve-{s}", f"arm-{s}", "cotton-underwear", 0.8, k.limb(f"arm-{s}", side, body.grow(body.SLEEVE[:3], -TEE_M)), n=2, start=0.022, region="hoodie", layer="tee", **inner))
        out.append(k.ellipsoid(f"th-briefs-seat-{s}", "pelvis", "cotton-underwear", 0.8, [-0.062, 0.885, side * 0.078], [0.058 - BRIEFS_M, 0.075 - BRIEFS_M, 0.072 - BRIEFS_M], region="jeans", layer="briefs"))
        out.append(k.loft(f"th-sock-foot-{s}", f"foot-{s}", "cotton-sock", 0.8, body.xrows(body.UPPER, side, SOCK_M), n=2, start=0.004, region="sneakers", layer="socks", **inner))
        out.append(k.loft(f"th-sock-toe-{s}", f"toe-{s}", "cotton-sock", 0.8, body.xrows([body.UPPER_TOE[0], body.TOE_CAP[1], body.TOE_CAP[2]], side, SOCK_M), end=0.004, region="sneakers", layer="socks", **inner))
    return out + outlets(k, [0.004, 0.006, 0.010], [0.004, 0.005, 0.013], [0.056, 0.0125, 0.046])


def thermal_coarse(k, solids, d):
    f = {100: 0.015, 200: 0.030, 500: 0.075}[d] / 2
    return references(solids, d >= 200) + outlets(k, [f, f, f], [f, f, f], [max(0.056, f), max(0.0125, f), max(0.046, f)])


DECISIONS = {
    50: [
        {"en": "Body at 1:50, modelled from anthropometry of a 1.69 m, 58 kg adult (head 1/7.5 of stature, bideltoid 0.43 m, hip breadth 0.34 m, thigh girth 0.54 m, calf girth 0.36 m): the braincase is an ellipsoid, the face a loft from brow to chin with nose, lips, chin, eyes, brows and ears, short hair as a cap whose front edge is the hairline. The hooded sweatshirt is one garment blended over lumbar spine and thorax (belly, chest, the round of the upper back, the shoulder slope), with hem rib, kangaroo pocket, collar rib, drawstrings, the hood as a pouch on the upper back and its rolled edge round the neck; set-in sleeves with a deltoid cap, an elbow and bunching above the rib cuffs. Both hands are alike: palm with thenar and hypothenar, a three-bone thumb and four fingers of three phalanges, each phalanx a rounded capsule on its own bone. Jeans: seat with gluteal masses, thighs with quadriceps and hamstring fullness tapering to the knee, kneecap, shank with the calf behind and the shin straight in front, hem above the ankle; ankle socks; canvas sneakers with a 26 to 20 mm rubber sole, toe spring, heel counter, instep, toe cap and five lace bars.", "de": "Körper bei 1:50, modelliert aus der Anthropometrie einer erwachsenen Person von 1,69 m und 58 kg (Kopf 1/7,5 der Körperhöhe, Schulterbreite über die Deltamuskeln 0,43 m, Hüftbreite 0,34 m, Oberschenkelumfang 0,54 m, Wadenumfang 0,36 m): Der Hirnschädel ist ein Ellipsoid, das Gesicht ein Loft von der Braue zum Kinn mit Nase, Lippen, Kinn, Augen, Brauen und Ohren, kurzes Haar als Kappe, deren Vorderkante die Haarlinie ist. Das Kapuzensweatshirt ist ein Kleidungsstück, das über Lendenwirbelsäule und Brustkorb verblendet ist (Bauch, Brust, Rundung des oberen Rückens, Schulterschräge), mit Saumbund, Kängurutasche, Kragenbund, Kordeln, der Kapuze als Beutel auf dem oberen Rücken und ihrem gerollten Rand um den Hals; eingesetzte Ärmel mit Deltakappe, Ellbogen und Stauung über den Bündchen. Beide Hände sind gleich: Handfläche mit Daumen- und Kleinfingerballen, ein dreigliedriger Daumen und vier Finger aus drei Gliedern, jedes Glied eine gerundete Kapsel an seinem eigenen Knochen. Jeans: Gesäß mit Gesäßmuskeln, Oberschenkel mit der Fülle von Quadrizeps und Beugern, zum Knie verjüngt, Kniescheibe, Unterschenkel mit der Wade hinten und dem geraden Schienbein vorn, Saum über dem Knöchel; Knöchelsocken; Stoffsneaker mit Gummisohle von 26 bis 20 mm, Zehensprengung, Fersenkappe, Rist, Zehenkappe und fünf Schnürstegen."},
        {"en": "Thermal at 1:50: the outer garment and skin surfaces are the body solids themselves. Inside lie the real inner layers: the T-shirt as a shell 4 mm under the sweatshirt on the trunk and as short sleeves on the upper arms, the briefs 1.2 mm under the denim on the pelvis, the socks 4 mm inside the sneaker uppers and visible at the ankles. The scalp lies under the hair cap, so only face, ears and neck count as bare skin. Breath leaves from a patch under the nose tip and one on the lips, the body plume from a patch on the crown.", "de": "Thermik bei 1:50: Die äußeren Kleidungs- und Hautflächen sind die Körper der Gestalt selbst. Darin liegen die wirklichen inneren Schichten: das T-Shirt als Schale 4 mm unter dem Sweatshirt am Rumpf und als kurze Ärmel an den Oberarmen, die Unterhose 1,2 mm unter dem Denim am Becken, die Socken 4 mm im Schaft der Sneaker und an den Knöcheln sichtbar. Die Kopfhaut liegt unter der Haarkappe, sodass nur Gesicht, Ohren und Hals als nackte Haut zählen. Der Atem tritt aus einem Fleck unter der Nasenspitze und einem auf den Lippen aus, die Körperfahne aus einem Fleck auf dem Scheitel."},
        {"en": "Structure at 1:50: the skeleton as the load path. Skull with facial skeleton, mandible with rami and zygomatic arches; 7 cervical, 12 thoracic and 5 lumbar vertebrae along the cervical lordosis, thoracic kyphosis and lumbar lordosis, each with body, spinous and transverse processes; sacrum and coccyx; twelve ribs per side from their vertebra round the cage, ribs 1 to 7 joined to the sternum by their cartilages, 8 to 10 to the costal arch, 11 and 12 floating; manubrium, body and xiphoid. Clavicle (S-curve) and scapula with spine, acromion, coracoid and glenoid ride on the shoulder-girdle bone. Hip bones with iliac wing, ischium and pubic rami meeting at the symphysis; femur with head, neck, greater trochanter, bowed shaft and condyles, patella, tibia with plateau and medial malleolus, fibula with lateral malleolus; talus, calcaneus, tarsus, five metatarsals and toes; humerus, radius ahead of the ulna (thumb forward), olecranon, carpus, metacarpals and every phalanx of both hands. Joint solids sit at the rig joints, masses at the de Leva centres of all segments above 0.5 kg.", "de": "Tragwerk bei 1:50: das Skelett als Lastpfad. Schädel mit Gesichtsschädel, Unterkiefer mit Ästen und Jochbögen; 7 Hals-, 12 Brust- und 5 Lendenwirbel entlang der Halslordose, Brustkyphose und Lendenlordose, jeder mit Körper, Dorn- und Querfortsätzen; Kreuz- und Steißbein; zwölf Rippen je Seite von ihrem Wirbel um den Brustkorb, die Rippen 1 bis 7 über ihre Knorpel mit dem Brustbein verbunden, 8 bis 10 mit dem Rippenbogen, 11 und 12 frei; Handgriff, Körper und Schwertfortsatz. Schlüsselbein (S-Schwung) und Schulterblatt mit Gräte, Schulterhöhe, Rabenschnabelfortsatz und Gelenkpfanne sitzen auf dem Schultergürtelknochen. Hüftbeine mit Darmbeinschaufel, Sitzbein und Schambeinästen, die sich an der Symphyse treffen; Oberschenkelknochen mit Kopf, Hals, großem Rollhügel, geschwungenem Schaft und Kondylen, Kniescheibe, Schienbein mit Plateau und Innenknöchel, Wadenbein mit Außenknöchel; Sprungbein, Fersenbein, Fußwurzel, fünf Mittelfußknochen und Zehen; Oberarmknochen, Speiche vor der Elle (Daumen nach vorn), Ellenhaken, Handwurzel, Mittelhandknochen und jedes Fingerglied beider Hände. Gelenkkörper sitzen an den Rig-Gelenken, Massen in den de-Leva-Schwerpunkten aller Segmente über 0,5 kg."},
    ],
    100: [
        {"en": "Body at 1:100, stated anew with fewer sections for a 15 mm feature: braincase, face without eyes, brows and lips, nose, ears, hair cap, neck; the sweatshirt with hem rib, pocket, hood pouch and rolled hood edge; sleeves, forearms and cuffs; both hands with palm, thenar and every finger phalanx; jeans seat with gluteal masses, thighs, kneecaps, shanks; socks; sneakers as sole, upper and a rubber toe box.", "de": "Körper bei 1:100, mit weniger Querschnitten für ein Merkmal von 15 mm neu gesetzt: Hirnschädel, Gesicht ohne Augen, Brauen und Lippen, Nase, Ohren, Haarkappe, Hals; das Sweatshirt mit Saumbund, Tasche, Kapuzenbeutel und gerolltem Kapuzenrand; Ärmel, Unterarme und Bündchen; beide Hände mit Handfläche, Daumenballen und jedem Fingerglied; Jeansgesäß mit Gesäßmuskeln, Oberschenkel, Kniescheiben, Unterschenkel; Socken; Sneaker als Sohle, Schaft und Gummizehenkappe."},
        {"en": "Thermal at 1:100: every region lies on the body solids of this scale, the layers of a garment merged into its outer surface; the three outlets are patches of one smallest feature.", "de": "Thermik bei 1:100: Jede Region liegt auf den Körpern dieses Maßstabs, die Schichten eines Kleidungsstücks gehen in seiner Außenfläche auf; die drei Auslässe sind Flecken von einem kleinsten Merkmal."},
        {"en": "Structure at 1:100: skull with facial skeleton and mandible; the spine as three columns (cervical, thoracic, lumbar) along their curves with the line of the spinous processes behind; sacrum; all twelve ribs per side with cartilages and sternum as slender members at real thickness; clavicle, scapula with spine and acromion; hip bones; long bones with their epiphyses, patella, both hands and feet bone by bone; joints and masses.", "de": "Tragwerk bei 1:100: Schädel mit Gesichtsschädel und Unterkiefer; die Wirbelsäule als drei Säulen (Hals, Brust, Lende) entlang ihrer Krümmungen mit der Linie der Dornfortsätze dahinter; Kreuzbein; alle zwölf Rippen je Seite mit Knorpeln und Brustbein als schlanke Stäbe in wahrer Dicke; Schlüsselbein, Schulterblatt mit Gräte und Schulterhöhe; Hüftbeine; Röhrenknochen mit ihren Epiphysen, Kniescheibe, beide Hände und Füße Knochen für Knochen; Gelenke und Massen."},
    ],
    200: [
        {"en": "Body at 1:200, one mass per part for a 30 mm feature: head as one form with the jaw, hair cap, neck, the sweatshirt from hem to collar with the shoulder slope, the hood pouch, jeans seat, and per side sleeve, forearm with cuff, a mitten hand 32 mm thick, thigh, shank, shoe and toe box.", "de": "Körper bei 1:200, eine Masse je Teil für ein Merkmal von 30 mm: Kopf als eine Form mit dem Kiefer, Haarkappe, Hals, das Sweatshirt vom Saum bis zum Kragen mit der Schulterschräge, der Kapuzenbeutel, Jeansgesäß und je Seite Ärmel, Unterarm mit Bündchen, eine Fausthandschuh-Hand von 32 mm Dicke, Oberschenkel, Unterschenkel, Schuh und Zehenkappe."},
        {"en": "Thermal at 1:200: every region lies on the masses of this scale, layers merged; the three outlets are patches of one smallest feature.", "de": "Thermik bei 1:200: Jede Region liegt auf den Massen dieses Maßstabs, die Schichten gehen auf; die drei Auslässe sind Flecken von einem kleinsten Merkmal."},
        {"en": "Structure at 1:200: the skeleton as its lines of thrust at real thickness: skull, cervical, thoracic and lumbar column along their curves, sacrum, six rib hoops per side closing on the sternum, one girdle line per side (clavicle, acromion, scapula), each hip bone from the sacroiliac joint over crest and acetabulum to the symphysis, humerus, forearm, hand with its finger bones, femur, tibia, foot and toes; masses of the segments above 2 kg.", "de": "Tragwerk bei 1:200: das Skelett als seine Drucklinien in wahrer Dicke: Schädel, Hals-, Brust- und Lendensäule entlang ihrer Krümmungen, Kreuzbein, sechs Rippenreifen je Seite, die am Brustbein schließen, eine Gürtellinie je Seite (Schlüsselbein, Schulterhöhe, Schulterblatt), jedes Hüftbein vom Kreuz-Darmbein-Gelenk über Kamm und Pfanne zur Symphyse, Oberarm, Unterarm, Hand mit ihren Fingerknochen, Oberschenkel, Schienbein, Fuß und Zehen; Massen der Segmente über 2 kg."},
    ],
    500: [
        {"en": "Body at 1:500, the fewest masses that read as a standing person at a 75 mm feature: head, hair cap, neck, trunk with chest, back and shoulder slope, jeans seat, and per side upper arm, forearm, hand, thigh, shank with the calf, and one shoe.", "de": "Körper bei 1:500, die wenigsten Massen, die bei einem Merkmal von 75 mm als stehende Person lesbar sind: Kopf, Haarkappe, Hals, Rumpf mit Brust, Rücken und Schulterschräge, Jeansgesäß und je Seite Oberarm, Unterarm, Hand, Oberschenkel, Unterschenkel mit der Wade und ein Schuh."},
        {"en": "Thermal at 1:500: every region lies on the masses of this scale, layers merged; the three outlets are patches of one smallest feature.", "de": "Thermik bei 1:500: Jede Region liegt auf den Massen dieses Maßstabs, die Schichten gehen auf; die drei Auslässe sind Flecken von einem kleinsten Merkmal."},
        {"en": "Structure at 1:500: the skeleton as slender bones at real thickness, not thickened to the feature: skull, the three spinal columns with their curves, sacrum, three rib hoops per side, sternum, girdle line, hip bone, humerus, forearm, hand and finger bones, femur, tibia, foot and toes; masses of pelvis, lumbar segment, thorax and thighs.", "de": "Tragwerk bei 1:500: das Skelett als schlanke Knochen in wahrer Dicke, nicht auf das Merkmal verdickt: Schädel, die drei Wirbelsäulenabschnitte mit ihren Krümmungen, Kreuzbein, drei Rippenreifen je Seite, Brustbein, Gürtellinie, Hüftbein, Oberarm, Unterarm, Hand und Fingerknochen, Oberschenkel, Schienbein, Fuß und Zehen; Massen von Becken, Lendensegment, Brustkorb und Oberschenkeln."},
    ],
}


def volumes(kit, asset):
    bodies = {50: body.body50(kit), 100: body.body100(kit), 200: body.body200(kit), 500: body.body500(kit)}
    out = {}
    for d, solids in bodies.items():
        thermal = thermal50(kit, solids) if d == 50 else thermal_coarse(kit, solids, d)
        structure = skeleton.structure50(kit, asset) if d == 50 else skeleton.structure100(kit, asset) if d == 100 else skeleton.structure_coarse(kit, asset, d)
        out[str(d)] = {"status": "authored", "denominator": d, "body": solids, "thermal": thermal, "structure": structure, "decisions": DECISIONS[d]}
    return out


def roots(asset):
    asset["dimensions"]["depthM"] = 0.5
    asset["dimensions"]["widthM"] = 0.31
