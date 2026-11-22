"""🩹️ Finisher B patches of shipped humans: idempotent, named edits of a catalog asset document that mirror the change made in the worker's generator.

python3 patch.py <slug> [--ship]   writes 🗑️generated/FIN-B/draft/<slug>.json, with --ship the catalog file (re-read right before the write)."""
import json
import os
import sys

R = "/Users/ueli/Documents/semio"
HUMANS = f"{R}/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
K = f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
DRAFT = f"{K}/🗑️generated/FIN-B/draft"
PATCHES = {}


def patch(slug):
    """🏷️ Registers the patch of one asset."""
    def register(f):
        PATCHES[slug] = f
        return f
    return register


def tx(en, de):
    """🗣️ A bilingual text."""
    return {"en": en, "de": de}


def clip(a, cid):
    """🎞️ A clip by id."""
    return next(c for c in a["clips"] if c["id"] == cid)


def spans(anchor, pairs):
    """🦶️ Contact spans of one anchor."""
    return [{"anchor": anchor, "fromS": f, "toS": t} for f, t in pairs]


FIELDS = ("angleDeg", "abductionDeg", "twistDeg", "dx", "dy")


def effective(a, key):
    """🦴️ The bone offsets a clip key shows: its pose with the key's per-field overrides."""
    pose = next(p for p in a["poses"] if p["id"] == key["pose"])
    out = {b: dict(v) for b, v in pose["bones"].items()}
    for b, v in key.get("bones", {}).items():
        out[b] = {**out.get(b, {}), **v}
    return out


def halfway(a, key_a, key_b, t):
    """⏸️ A key halfway between two keys, as overrides of the second key's pose: an `in` ease into it and an `out` ease out of it join with one velocity and start and end at rest."""
    ea, eb = effective(a, key_a), effective(a, key_b)
    bones = {}
    for b in sorted(set(ea) | set(eb)):
        va, vb = ea.get(b, {}), eb.get(b, {})
        mid = {f: round((va.get(f, 0) + vb.get(f, 0)) / 2, 4) for f in FIELDS if f in va or f in vb}
        bones[b] = mid
    return {"t": t, "pose": key_b["pose"], "ease": "out", "bones": bones}


@patch("person-queuing")
def person_queuing(a):
    """🧍️ The heels lift in the weight-shift stances, so only the balls of the feet are permanent supports; the heels stay switchable contacts without a support solid."""
    a["structure"]["supports"] = ["ball-l", "ball-r"]
    for vol in a["volumes"].values():
        vol["structure"] = [s for s in vol["structure"] if not (s.get("part") == "support" and s.get("anchor") in ("heel-l", "heel-r"))]


@patch("dancer")
def dancer(a):
    """💃️ The pirouette without jerks: the turn spins up over the first 45 degrees and down over the last 45 (quadratic eases meeting the constant 450 degrees per second), the descent from demi-pointe into fourth takes 0.5 s and the second foot loads once it is down."""
    spin0, land, down, end = 0.75, float(os.environ.get("LAND", 2.25)), float(os.environ.get("DOWN", 2.15)), float(os.environ.get("END", 2.8))
    off = float(os.environ.get("OFF", 0.45))
    c = clip(a, "pirouette")
    if abs(c["durationS"] - end) < 1e-9:
        dancer_texts(a)
        return
    head = [k for k in c["keys"] if k["t"] < spin0 - 1e-9]
    passe = next(k for k in c["keys"] if k["pose"] == "passe-releve")
    tail = [k for k in c["keys"] if k["pose"] != "passe-releve" and k["t"] > spin0]
    turn = []
    for i, (deg, ease) in enumerate([(0, "in"), (45, "linear"), (135, "linear"), (225, "linear"), (315, "out"), (360, "smooth")]):
        k = json.loads(json.dumps(passe))
        k["t"], k["ease"] = round(spin0 + 0.2 * i, 3), ease
        k["bones"]["pelvis"] = {"twistDeg": deg}
        turn.append(k)
    up = turn[-1]["t"]
    turn[-1]["ease"] = "in"
    for k in head:
        k["ease"] = "linear"
    head[-1]["ease"] = "in"
    tail[0]["t"], tail[-1]["t"] = land, end
    tail[0]["ease"] = tail[-1]["ease"] = "linear"
    rise = halfway(a, head[-1], turn[0], round((head[-1]["t"] + spin0) / 2, 3))
    lower = halfway(a, turn[-1], tail[0], round((up + land) / 2, 3))
    c["keys"] = head + [rise] + turn + [lower, tail[0], tail[-1]]
    c["durationS"] = end
    c["contacts"] = spans("contact-l", [[0, end]]) + spans("heel-l", [[0, off], [down, end]]) + spans("contact-r", [[0, off], [down, end]]) + spans("heel-r", [[0, off], [down, end]])
    for p in c["phases"]:
        if p["id"] == "preparation":
            p["toS"] = off
        if p["id"] == "turn":
            p["fromS"], p["toS"] = off, up
        if p["id"] == "landing":
            p["fromS"], p["toS"] = up, end
    for t in c["tracks"]:
        t["keys"][-1]["t"] = end
    for e in c["events"]:
        e["t"] = down if e["kind"] == "contact-on" else off
    c["description"] = tx("From the preparation in fourth the dancer pushes up onto the ball of the left foot into passé and turns once to the right about the ball in one second, spinning up over the first eighth of the turn and slowing over the last, then lowers back into fourth in half a second; the ball of the supporting foot stays on its spot and the arms close to first for the turn.",
                          "Aus der Vorbereitung in der vierten Position drückt sich der Tänzer auf den Ballen des linken Fußes ins Passé und dreht sich in einer Sekunde einmal um den Ballen, im ersten Achtel der Drehung beschleunigend und im letzten abbremsend, dann senkt er sich in einer halben Sekunde wieder in die vierte Position; der Ballen des Standfußes bleibt auf seinem Fleck und die Arme schließen sich für die Drehung zur ersten Position.")
    dancer_texts(a)


import re

GENERIC = {
    "thinElements": ("No filled shape is thinner than the smallest printable feature of this scale; nothing is thickened.", "Keine gefüllte Form ist dünner als das kleinste druckbare Merkmal dieses Maßstabs; nichts wird verdickt."),
    "hatching": ("None; fill tones carry the materials.", "Keine; Fülltöne tragen die Materialien."),
    "styles": ("Body pen {pen} mm for the masses; light tones for skin, mid and dark tones separate the garments, shoes and hair.", "Körperstift {pen_de} mm für die Massen; helle Töne für Haut, mittlere und dunkle Töne trennen Kleidung, Schuhe und Haar."),
    "overlays": ("Every filled shape carries the thermal region of the garment or body part it depicts in all three views, so thermal and structural overlays read per region.", "Jede gefüllte Form trägt in allen drei Ansichten die thermische Region des Kleidungsstücks oder Körperteils, das sie zeigt, sodass thermische und statische Überlagerungen je Region lesbar sind."),
    "volume": ("Stated with the solids of this scale as at the larger scale, reduced to its smallest feature.", "Mit den Körpern dieses Maßstabs wie im größeren Maßstab angegeben, auf sein kleinstes Merkmal reduziert."),
}


def drop_sentences(text, pattern):
    """✂️ A bilingual text without the sentences that match the stale pattern."""
    return {lang: " ".join(x for x in re.split(r"(?<=\.) ", text[lang]) if not re.search(pattern, x)).strip() for lang in ("en", "de")}


def redecide(a, stale, spec=None, volume=None):
    """🗒️ Rewrites the per-scale decisions that still describe the base figure: the fields named in `spec` (tiers `fine` for 1:50 and 1:100, `coarse` for 1:200 and 1:500) are replaced, other fields that match the stale pattern get the neutral sentence of their kind, and the volume decisions lose their stale sentences and gain the `volume` sentence on their first entry."""
    for scale, rep in a["representations"].items():
        if rep.get("status") != "authored":
            continue
        tier = "fine" if scale in ("50", "100") else "coarse"
        pen = rep["pens"]["body"]
        fill = lambda pair: tx(pair[0].format(d=scale, pen=pen), pair[1].format(d=scale, pen=pen, pen_de=str(pen).replace(".", ",")))
        for field, text in list(rep["decisions"].items()):
            if spec and field in spec[tier]:
                rep["decisions"][field] = fill(spec[tier][field])
            elif re.search(stale, text["en"] + " " + text["de"]):
                kept = drop_sentences(text, stale)
                rep["decisions"][field] = kept if kept["en"] and kept["de"] else fill(GENERIC.get(field, GENERIC["thinElements"]))
    for scale, vol in a["volumes"].items():
        if vol.get("status") != "authored":
            continue
        out = []
        for i, text in enumerate(vol["decisions"]):
            kept = drop_sentences(text, stale) if re.search(stale, text["en"] + " " + text["de"]) else text
            if i == 0 and volume and volume[0] not in kept["en"]:
                kept = {"en": f"{kept['en']} {volume[0]}".strip(), "de": f"{kept['de']} {volume[1]}".strip()}
            out.append(kept if kept["en"] and kept["de"] else tx(*GENERIC["volume"]))
        vol["decisions"] = out


VIEWS_EN = "side view from the right, front view and plan are the same solids from the side, from ahead and from above."
VIEWS_DE = "Seitenansicht von rechts, Vorderansicht und Grundriss sind dieselben Körper von der Seite, von vorn und von oben."
MASSES_EN = "front view and plan are the same masses from ahead and from above."
MASSES_DE = "Vorderansicht und Grundriss sind dieselben Massen von vorn und von oben."


@patch("person-in-rain-poncho")
def person_in_rain_poncho(a):
    """🌧️ Decisions of the figure under the poncho instead of the commuter with coat, scarf and shoulder bag."""
    for an in a["anchors"]:
        if re.search(r"bag strap", an["title"]["en"]):
            an["title"] = tx("Grip of the right hand under the poncho", "Griff der rechten Hand unter dem Poncho")
    for rep in a["representations"].values():
        for h in rep.get("hatches", []):
            if re.search(r"overcoat", h["title"]["en"]):
                h["title"] = tx("Jacket hatching", "Schraffur der Jacke")
    redecide(a, r"\bbag\b|strap|overcoat|lapel|scarf|button|shirt cuff|derby|1\.78 m|Tasche|Riemen|Mantel|Revers|Schal|Knopf|Knöpfe|Manschette|Derby|1,78 m", {
        "fine": {
            "silhouette": ("Every shape is the orthographic outline of the 1:{d} model standing in the rain: the hooded poncho as one bell from the shoulders to just above the knees, the hood round the face, the jacket and sleeves under it where they show, trousers, the shafts of the wellington boots and the boots; " + VIEWS_EN, "Jede Form ist der orthogonale Umriss des 1:{d}-Modells im Regen stehend: der Poncho mit Kapuze als eine Glocke von den Schultern bis knapp über die Knie, die Kapuze um das Gesicht, Jacke und Ärmel darunter, wo sie sichtbar sind, Hose, die Schäfte der Gummistiefel und die Stiefel; " + VIEWS_DE),
            "retained": ("The bell of the poncho with its hem, the hood opening with the face, the boot shafts over the trousers.", "Die Glocke des Ponchos mit ihrem Saum, die Kapuzenöffnung mit dem Gesicht, die Stiefelschäfte über der Hose."),
            "merged": ("Arms and hands lie under the poncho and merge into its outline; the collar merges into the hood.", "Arme und Hände liegen unter dem Poncho und gehen in seinem Umriss auf; der Kragen geht in der Kapuze auf."),
            "omitted": ("Press studs, seams and folds of the poncho, the drawcord of the hood and the tread of the boots.", "Druckknöpfe, Nähte und Falten des Ponchos, die Kordel der Kapuze und das Profil der Stiefel."),
            "thinElements": ("The poncho film is drawn as the outline of its volume, never as a separate thin sheet; nothing is thickened.", "Die Ponchofolie ist als Umriss ihres Volumens gezeichnet, nie als eigene dünne Platte; nichts wird verdickt."),
        },
        "coarse": {
            "silhouette": ("The orthographic outline of the 1:{d} model: the poncho bell with the hood as the dominant mass, the head inside the hood, legs and wellington boots below; " + MASSES_EN, "Der orthogonale Umriss des 1:{d}-Modells: die Ponchoglocke mit der Kapuze als bestimmende Masse, der Kopf in der Kapuze, Beine und Gummistiefel darunter; " + MASSES_DE),
            "retained": ("Poncho, hood, legs and boots stay separate masses.", "Poncho, Kapuze, Beine und Stiefel bleiben getrennte Massen."),
            "merged": ("Arms, hands and jacket merge into the poncho, the face into the head.", "Arme, Hände und Jacke gehen im Poncho auf, das Gesicht im Kopf."),
            "omitted": ("Face details, seams, press studs and the hem line.", "Gesichtsdetails, Nähte, Druckknöpfe und die Saumlinie."),
            "thinElements": GENERIC["thinElements"],
        },
    }, ("The poncho is one closed shell over trunk and arms from the shoulders to above the knees with the hood as a second shell round the head; the wellington shafts enclose the shanks.", "Der Poncho ist eine geschlossene Schale über Rumpf und Armen von den Schultern bis über die Knie, die Kapuze eine zweite Schale um den Kopf; die Gummistiefelschäfte umschließen die Unterschenkel."))


@patch("person-in-winter-coat")
def person_in_winter_coat(a):
    """🧥️ Decisions of the figure in parka, scarf, beanie and winter boots instead of the commuter with a shoulder bag."""
    for an in a["anchors"]:
        if re.search(r"bag strap", an["title"]["en"]):
            an["title"] = tx("Grip of the right hand in the pocket", "Griff der rechten Hand in der Tasche")
    redecide(a, r"\bbag\b|strap|lapel|three buttons|shirt cuff|derby|Umhängetasche|die Tasche|der Tasche|Tasche mit|Riemen|Revers|drei Knöpfe|Manschette|Derby", {
        "fine": {
            "silhouette": ("Every shape is the orthographic outline of the 1:{d} model standing with the hands in the side pockets: the knee-length quilted parka with its waist seam, skirt and centre front, the stand collar under a chunky scarf, the beanie over hair and ears, sleeves ending in the pockets, trousers tucked into the lined boot shafts and the boots; " + VIEWS_EN, "Jede Form ist der orthogonale Umriss des 1:{d}-Modells, stehend mit den Händen in den Seitentaschen: der knielange Steppparka mit Taillennaht, Schoß und vorderer Mitte, der Stehkragen unter einem dicken Schal, die Mütze über Haar und Ohren, Ärmel, die in den Taschen enden, die Hose in den gefütterten Stiefelschäften und die Stiefel; " + VIEWS_DE),
            "retained": ("The quilted volume of the parka, the scarf, the beanie and the boot shafts.", "Das gesteppte Volumen des Parkas, der Schal, die Mütze und die Stiefelschäfte."),
            "merged": ("The gloved hands merge into pockets and sleeves, the stand collar into the scarf.", "Die Hände in Handschuhen gehen in Taschen und Ärmeln auf, der Stehkragen im Schal."),
            "omitted": ("Quilting seams, zip, pocket flaps and the laces of the boots.", "Steppnähte, Reißverschluss, Taschenklappen und die Schnürung der Stiefel."),
            "thinElements": GENERIC["thinElements"],
        },
        "coarse": {
            "silhouette": ("The orthographic outline of the 1:{d} model: the parka as waist and skirt masses, the scarf ring, head with beanie, sleeves, legs, boot shafts and boots; " + MASSES_EN, "Der orthogonale Umriss des 1:{d}-Modells: der Parka als Taillen- und Schoßmasse, der Schalring, Kopf mit Mütze, Ärmel, Beine, Stiefelschäfte und Stiefel; " + MASSES_DE),
            "retained": ("Parka, scarf, beanie and boots stay separate masses.", "Parka, Schal, Mütze und Stiefel bleiben getrennte Massen."),
            "merged": ("Hands into the sleeves, collar into the scarf, face and ears into the head.", "Hände in den Ärmeln, Kragen im Schal, Gesicht und Ohren im Kopf."),
            "omitted": ("Face details, seams and pocket lines.", "Gesichtsdetails, Nähte und Taschenlinien."),
            "thinElements": GENERIC["thinElements"],
        },
    }, ("The parka is the coat shell filled out by a 20 mm down layer with a stand collar, the beanie a shell over the hair, the boot shafts enclose the tucked-in trousers.", "Der Parka ist die Mantelschale, um eine 20 mm starke Daunenschicht gefüllt, mit Stehkragen, die Mütze eine Schale über dem Haar, die Stiefelschäfte umschließen die eingesteckte Hose."))


@patch("sunbather-on-towel")
def sunbather_on_towel(a):
    """🏖️ Decisions of the lying figure in a swimsuit instead of the standing adult in sweatshirt, jeans and sneakers."""
    towel = ("The towel keeps its true thickness and is a thin band in the elevations; at 1:500 its outline uses a 0.20 mm pen so that it prints.", "Das Tuch behält seine wahre Dicke und ist in den Ansichten ein dünnes Band; bei 1:500 hat sein Umriss einen 0,20-mm-Stift, damit es druckt.")
    redecide(a, r"sweatshirt|hood|jeans|sneaker|shoe|sock|pocket|lace|cuff|\bhem\b|drawstring|Kapuze|Jeans|Sneaker|Schuh|Socke|Tasche|Schnür|Bündchen|Saum|Kordel|Pullover|Hoodie", {
        "fine": {
            "silhouette": ("Every shape is the orthographic outline of the 1:{d} body lying on its back on the towel: head with hair, neck, the trunk in a one-piece swimsuit, bare arms with the hands beside the body, bare thighs, shanks and feet, and the towel as a flat rectangle of its true 1.80 by 0.90 m; the side view shows the lying profile, the front view looks along the body and the plan from above.", "Jede Form ist der orthogonale Umriss des 1:{d}-Körpers, der auf dem Rücken auf dem Tuch liegt: Kopf mit Haar, Hals, der Rumpf im einteiligen Badeanzug, bloße Arme mit den Händen neben dem Körper, bloße Oberschenkel, Unterschenkel und Füße und das Tuch als flaches Rechteck in seinen wahren 1,80 mal 0,90 m; die Seitenansicht zeigt das liegende Profil, die Vorderansicht blickt entlang des Körpers und der Grundriss von oben."),
            "retained": ("The swimsuit against the bare skin by tone, the stripes of the towel in the plan, eye, brow, mouth and ear at 1:50.", "Der Badeanzug gegen die bloße Haut durch den Ton, die Streifen des Tuchs im Grundriss, Auge, Braue, Mund und Ohr bei 1:50."),
            "merged": ("Fingers into the hands, toes into the feet, the straps of the swimsuit into the trunk.", "Finger in den Händen, Zehen in den Füßen, die Träger des Badeanzugs im Rumpf."),
            "omitted": ("Seams of the swimsuit and the terry texture of the towel.", "Nähte des Badeanzugs und die Frotteestruktur des Tuchs."),
            "thinElements": towel,
        },
        "coarse": {
            "silhouette": ("The orthographic outline of the 1:{d} body lying on the towel: head with hair, trunk in the swimsuit, arms, legs and feet as one mass per segment and the towel as a flat rectangle; front view and plan are the same masses seen along the body and from above.", "Der orthogonale Umriss des 1:{d}-Körpers auf dem Tuch: Kopf mit Haar, Rumpf im Badeanzug, Arme, Beine und Füße als je eine Masse je Segment und das Tuch als flaches Rechteck; Vorderansicht und Grundriss sind dieselben Massen entlang des Körpers und von oben gesehen."),
            "retained": ("The lying posture, the dark swimsuit and the towel under the figure.", "Die liegende Haltung, der dunkle Badeanzug und das Tuch unter der Figur."),
            "merged": ("Hands into mitten forms and each foot into one form.", "Hände in Fäustlingsformen und jeder Fuß in einer Form."),
            "omitted": ("Face, fingers, toes and at 1:500 the stripes of the towel.", "Gesicht, Finger, Zehen und bei 1:500 die Streifen des Tuchs."),
            "thinElements": towel,
        },
    }, ("The clothing solids of the base figure are pared down to the bare body: the trunk is one swimsuit shell, arms, legs and feet are skin; the towel is a declared sheet under the figure.", "Die Kleidungskörper der Ausgangsfigur sind auf den bloßen Körper zurückgeführt: der Rumpf ist eine Badeanzugschale, Arme, Beine und Füße sind Haut; das Tuch ist eine erklärte Platte unter der Figur."))


@patch("person-checking-watch")
def person_checking_watch(a):
    """⌚️ Decisions of the figure reading a wristwatch instead of the smartphone user."""
    for an in a["anchors"]:
        if re.search(r"phone call", an["title"]["en"]):
            an["title"] = tx("Ear", "Ohr")
    redecide(a, r"phone\b|texting|screen|Telefon|Smartphone|Handy|Bildschirm|Tipp|tippen", {
        "fine": {
            "silhouette": ("Every shape is the orthographic outline of the 1:{d} body reading the watch: the head bowed to the raised left wrist, the hair pulled back into a ponytail, the neck with the headphones round it, the hooded sweatshirt with hood, hem and pocket, the left forearm lifted across the chest with the wristwatch, the right arm hanging, slim jeans and canvas sneakers; " + VIEWS_EN, "Jede Form ist der orthogonale Umriss des 1:{d}-Körpers beim Blick auf die Uhr: der Kopf zum gehobenen linken Handgelenk geneigt, das Haar zum Pferdeschwanz zurückgebunden, der Hals mit den Kopfhörern darum, der Kapuzenpullover mit Kapuze, Saum und Tasche, der linke Unterarm mit der Armbanduhr vor die Brust gehoben, der rechte Arm hängend, enge Jeans und Stoffsneaker; " + VIEWS_DE),
            "retained": ("Watch case and strap on the left wrist, the headphone cups and band, the ponytail, the kangaroo pocket and the fingers of both hands.", "Uhrgehäuse und Armband am linken Handgelenk, Kopfhörermuscheln und -bügel, der Pferdeschwanz, die Bauchtasche und die Finger beider Hände."),
            "merged": ("Hem rib into the sweatshirt, socks into the sneakers, hair tie into the ponytail.", "Saumbündchen im Pullover, Socken in den Sneakern, Haargummi im Pferdeschwanz."),
            "omitted": ("The dial and hands of the watch, seams and the cable of the headphones.", "Zifferblatt und Zeiger der Uhr, Nähte und das Kabel der Kopfhörer."),
            "thinElements": ("Watch case and strap are drawn at their true size and never thickened.", "Uhrgehäuse und Armband sind in ihrer wahren Größe gezeichnet und nie verdickt."),
        },
        "coarse": {
            "silhouette": ("The orthographic outline of the 1:{d} body: bowed head with hair and ponytail, neck, sweatshirt with hood, the raised left forearm, the hanging right arm, legs and shoes, one mass per part; " + MASSES_EN, "Der orthogonale Umriss des 1:{d}-Körpers: geneigter Kopf mit Haar und Pferdeschwanz, Hals, Pullover mit Kapuze, der gehobene linke Unterarm, der hängende rechte Arm, Beine und Schuhe, eine Masse je Teil; " + MASSES_DE),
            "retained": ("The raised forearm that identifies the gesture, the hood and the ponytail.", "Der gehobene Unterarm, der die Geste kenntlich macht, die Kapuze und der Pferdeschwanz."),
            "merged": ("Thumb and fingers into the hand, the watch into the wrist.", "Daumen und Finger in der Hand, die Uhr im Handgelenk."),
            "omitted": ("Face, the watch as a separate shape, laces and seams.", "Gesicht, die Uhr als eigene Form, Schnürung und Nähte."),
            "thinElements": GENERIC["thinElements"],
        },
    }, ("The smartphone of the base figure is removed; the wristwatch is a small case with its strap on the left forearm where the scale can state it.", "Das Smartphone der Ausgangsfigur ist entfernt; die Armbanduhr ist ein kleines Gehäuse mit Armband am linken Unterarm, wo der Maßstab es angeben kann."))


@patch("person-carrying-shopping-bags")
def person_carrying_shopping_bags(a):
    """🛍️ Volume decisions of the two paper bags instead of the commuter's shoulder bag on its strap."""
    redecide(a, r"strap|leather bag|shoulder bag|Riemen|Ledertasche|Umhängetasche", None, ("Each paper bag is a block with a pinched rim and its two handles in the fist, the groceries stand out of the rim; both arms hang straight with the load.", "Jede Papiertüte ist ein Block mit eingezogenem Rand und ihren zwei Henkeln in der Faust, die Einkäufe ragen über den Rand; beide Arme hängen gestreckt unter der Last."))


@patch("picnicker-sitting-on-ground")
def picnicker(a):
    """🧺️ Volume decisions without the guitar and the stool of the base figure."""
    redecide(a, r"guitar|Gitarre|stool|Hocker", None, ("The figure sits on a picnic blanket (a declared sheet on the ground) and holds an apple, a body of revolution on its own bone in the right hand.", "Die Figur sitzt auf einer Picknickdecke (eine erklärte Platte auf dem Boden) und hält einen Apfel, einen Rotationskörper an einem eigenen Knochen in der rechten Hand."))
    for m in a["materials"]:
        if re.search(r"strings|frets", m["title"]["en"]):
            m["title"] = tx("Nickel-plated steel (buckles and eyelets)", "Vernickelter Stahl (Schnallen und Ösen)")


@patch("parent-with-child-on-shoulders")
def parent_with_child(a):
    """👨‍👧️ Texts of the shoulder carry instead of the base's carry on the forearm."""
    for p in a["poses"]:
        for old, new in ((", the free far arm swings against the legs, the near arm keeps the toddler seated on the forearm against the chest and the child bobs slightly with the trunk.", ", both hands keep their hold on the child's shanks at the chest and the child on the shoulders bobs slightly with the trunk."),):
            p["description"]["en"] = p["description"]["en"].replace(old, new)
        for old, new in ((", der freie ferne Arm schwingt gegenläufig, der nahe Arm hält das Kleinkind auf dem Unterarm an der Brust, und das Kind wippt leicht mit dem Rumpf.", ", beide Hände halten die Unterschenkel des Kindes an der Brust, und das Kind auf den Schultern wippt leicht mit dem Rumpf."),):
            p["description"]["de"] = p["description"]["de"].replace(old, new)
    d = a["representations"]["50"]["decisions"]["silhouette"]
    d["en"] = d["en"].replace("the toddler on the near forearm against the chest with its large hatted head beside the parent's cheek, the far arm across the child's belly;", "the child astride the shoulders with its legs down the parent's chest, its shanks held in the parent's hands and its hatted head above the parent's beanie;")
    d["de"] = d["de"].replace("das Kleinkind auf dem nahen Unterarm an der Brust, sein großer Kopf mit Mütze neben der Wange des Elternteils, der ferne Arm vor dem Bauch des Kindes;", "das Kind rittlings auf den Schultern, die Beine vor der Brust des Elternteils, die Unterschenkel in dessen Händen und sein Kopf mit Mütze über der Mütze des Elternteils;")


@patch("person-with-guide-dog")
def person_with_guide_dog(a):
    """🦮️ The walking breathing pattern names the guide dog instead of the base's cane."""
    for pattern in a["thermal"]["respiration"]["patterns"]:
        if "cane" in pattern["title"]["en"]:
            pattern["title"] = tx("Brisk walking with the guide dog", "Zügiges Gehen mit dem Führhund")


def dancer_texts(a):
    """🩰️ Garment and symbol texts of the dancer instead of the jogger's."""
    for region in a["thermal"]["regions"]:
        for layer in region.get("layers", []):
            if layer["title"]["en"].startswith("Polyester running shirt"):
                layer["title"] = tx("Fitted polyester dance top (0.09 clo)", "Eng anliegendes Tanzoberteil aus Polyester (0,09 clo)")
            if layer["title"]["en"].startswith("Polyester running shorts"):
                layer["title"] = tx("Polyester dance shorts (0.05 clo)", "Tanzshorts aus Polyester (0,05 clo)")
    for symbol in a.get("symbols", []):
        for field in ("title", "description", "dimensionalNote"):
            if field in symbol:
                symbol[field] = {"en": symbol[field]["en"].replace("runner", "dancer").replace("Runner", "Dancer"), "de": symbol[field]["de"].replace("Laufrichtung eines Läufers", "Blickrichtung eines Tänzers").replace("Läufers", "Tänzers").replace("Läufer", "Tänzer")}


def main():
    slug, ship = sys.argv[1], "--ship" in sys.argv
    path = f"{HUMANS}/{slug}/🔣️.json"
    with open(path, encoding="utf8") as f:
        doc = json.load(f)
    PATCHES[slug](doc["asset"])
    os.makedirs(DRAFT, exist_ok=True)
    out = path if ship else f"{DRAFT}/{slug}.json"
    with open(out, "w", encoding="utf8") as f:
        f.write(json.dumps(doc, indent=2, ensure_ascii=False) + "\n")
    print(out)


if __name__ == "__main__":
    main()
