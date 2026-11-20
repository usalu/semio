"""🍃 Author small digital subjects from species-specific sections in the local frame of each articulated member."""
import copy
import json
import re
from pathlib import Path

catalog = next(Path.cwd().glob("*teaching/*architecture/*staffage/*catalog"))
models = {
    ("hosta-clump", 200): """
petiole-r1|petiole-l1|petiole-r2|petiole-l2|petiole-r3|petiole-l3|petiole-r4|petiole-l4|petiole-c1 0,1,2 petioles
blade-r1|blade-l1 0,3,5,7,10,13,14 blades-outer
blade-r2|blade-l2 0,2,5,8,11,14,15 blades-outer
blade-r3|blade-l3 0,3,6,9,13,16,18 blades-inner
blade-r4 0,3,6,10,13,16,18 blades-inner
blade-l4 0,2,5,8,11,14,16 blades-inner
blade-c1 0,2,5,7,9,11,12 blades-inner
""",
    ("hosta-clump", 500): """
petiole-r1|petiole-l1|petiole-r2|petiole-l2|petiole-r3|petiole-l3|petiole-r4|petiole-l4|petiole-c1 0,2 petioles
blade-r1|blade-l1 0,3,6,10,14 blades-outer
blade-r3|blade-l3 0,4,10,14,18 blades-inner
blade-r2|blade-l2 0,3,7,11,15 blades-outer
blade-r4 0,5,10,15,18 blades-inner
blade-l4 0,4,9,13,16 blades-inner
blade-c1 0,3,8,12 blades-inner
""",
    ("domestic-cat", 500): """
rump 0,3,8,12 coat-trunk
hindbody 0,5,10,18 coat-trunk
midbody 0,3,9,12 coat-trunk
chest 0,3,8,13 coat-trunk
neck 0,4,11,14 coat-trunk
head 0,4,9,16,22 coat-head
ear-{s} 0,2,5 ears
tail-1|tail-2|tail-3|tail-4 2,4,6,8 coat-tail
tail-5 2,4,6,8,10 coat-tail
thigh-h{s} 0,4,12,18 coat-hindlimbs
shank-h{s} 0,5,16 coat-hindlimbs
hock-h{s} 0,5,14 coat-hindlimbs
paw-h{s} 0,2,5 coat-hindlimbs
shoulder-f{s}|arm-f{s} 0,4,10,14 coat-forelimbs
forearm-f{s} 0,5,16 coat-forelimbs
wrist-f{s} 0,4,10 coat-forelimbs
paw-f{s} 0,2,5 coat-forelimbs
""",
    ("herring-gull", 500): """
body 0,5,13,20,32,42 body-plumage
neck-1 0,5,10,15 neck-plumage
neck-2 0,3,10,14 neck-plumage
head 0,3,8,12,16 head-plumage
bill 0,5,11,15,16 bill
tail 0,3,6 tail-plumage
{side}-coverts 0,3,11,18 wing-plumage
{side}-primaries 0,6,14,20 wing-plumage
{side}-thigh 0,2,5 body-plumage
{side}-shank|{side}-tarsus 0,2,5 legs-skin
{side}-web-inner|{side}-web-outer 0,2,4 web-feet
""",
    ("mallard-duck", 500): """
body 0,3,8,13,19 body-plumage
rump 0,4,10,16 body-plumage
breast 0,4,9,15 body-plumage
neck-1|neck-3 0,5,12 neck-plumage
neck-2 0,6,14 neck-plumage
head 0,3,6,11,14 head-plumage
bill 0,3,9,13,14 bill
tail 0,2,6 tail-plumage
{side}-wing 0,4,10,18 wing-plumage
{side}-primaries 0,4,12 wing-plumage
{side}-thigh|{side}-shank|{side}-tarsus 0,2,5 legs-skin
{side}-web-inner|{side}-web-outer 0,2,4 web-feet
""",
    ("pigeon", 500): """
body 0,5,12,21,29,33 plumage-body
neck-1 0,6,16 plumage-neck-head
neck-2|neck-3 0,5,12 plumage-neck-head
head 0,3,8,14 plumage-neck-head
beak 0,3,6 beak
cere 0,2,4 cere
tail 0,2,6 plumage-tail
far-tail 0,2,5 plumage-tail
{side}-coverts 0,5,12,18 plumage-wings
{side}-primaries 0,3,7 plumage-wings
{side}-shoulder 0,5,14 plumage-wings
{side}-drumstick 0,2,7 plumage-body
{side}-shank 0,2,4 legs-feet
{side}-tarsus 0,3,5 legs-feet
{side}-toes 0,2,6 legs-feet
{side}-hallux 0,2,4 legs-feet
""",
    ("red-fox", 500): """
hindbody 0,3,8,13,17 coat-trunk
chest 0,4,8,13,15 coat-trunk
neck 0,3,10,16 coat-trunk
head 0,3,9,14,18 face
brush-root 0,3,8,12 brush
brush-mid 0,4,7,10 brush
brush-tip 0,3,6,10 brush
{side}-ear 0,2,6 ears
{side}-thigh 0,4,9,14 coat-legs
{side}-gaskin|{side}-forearm 0,5,14 coat-legs
{side}-upper-arm|{side}-shoulder 0,4,12 coat-legs
{side}-hind-cannon|{side}-fore-cannon 0,2,6 coat-legs
{side}-hind-paw|{side}-fore-paw 0,4,7,10 paws
""",
    ("red-squirrel", 200): """
rump 0,3,7,14,22,28 fur-back
chest 0,3,9,15,22,26 fur-back
belly-patch 0,5,12,18,22 fur-belly
belly-patch-rump 0,6,12,17,20 fur-belly
neck 0,3,9,16,22 head-fur
head 0,4,9,16,24,30 head-fur
tail-1 0,4,8,14,18,22 tail-fur
tail-2 0,3,7,14,18,22 tail-fur
{side}-arm|{side}-forearm 0,5,12,19,24 arms-fur
{side}-hand 0,4,8,13,16 arms-fur
{side}-fingers 0,3,6,10 hands-skin
{side}-thigh|{side}-shank 0,3,9,17,23,26 legs-fur
{side}-foot 0,4,9,15,20 legs-fur
{side}-sole 0,5,11,18 feet-skin
{side}-ear 0,3,8,12 ears
""",
    ("red-squirrel", 500): """
rump 0,6,17,28 fur-back
chest 0,5,14,26 fur-back
belly-patch 0,6,14,22 fur-belly
neck 0,8,22 head-fur
head 0,5,14,22,30 head-fur
tail-1 0,5,12,18,22 tail-fur
tail-2 0,5,13,18,22 tail-fur
{side}-arm|{side}-forearm 0,8,24 arms-fur
{side}-hand 0,7,16 hands-skin
{side}-thigh 0,5,17,26 legs-fur
{side}-shank 0,7,26 legs-fur
{side}-foot 0,5,14,20 legs-fur
{side}-sole 0,8,18 feet-skin
{side}-ear 0,5,12 ears
""",
    ("hedgehog", 200): """
belly 0,1,3,5 belly-fur
chest 0,2,4 belly-fur
dome|dome-front 0,1,3,5,6 spines-dome
bowl-rn|bowl-rf|bowl-fn|bowl-ff 0,2,4,7,9 spines-skirt
neck 0,2,3 face-fur
head 0,2,4,6,8 face-fur
nose * nose
ear-{s}|eye-{s} * face-fur
thigh-{s}|arm-{s} 0,1,3,5 limbs-fur
shank-{s}|forearm-{s} 0,2,4,5 limbs-fur
foot-h{s} 0,2,4 limbs-fur
foot-f{s} 0,1,3 limbs-fur
toes-h{s}|toes-f{s} 0,2,4 paws-skin
""",
    ("hedgehog", 500): """
belly 0,2,5 belly-fur
chest 0,2,4 belly-fur
dome|dome-front 0,2,4,6 spines-dome
bowl-rn|bowl-rf|bowl-fn|bowl-ff 0,3,6,9 spines-skirt
head 0,2,5,8 face-fur
neck 0,3 face-fur
nose * nose
thigh-{s}|arm-{s} 0,2,5 limbs-fur
shank-{s}|forearm-{s} 0,3,5 limbs-fur
foot-h{s} 0,2,4 paws-skin
foot-f{s} 0,2,3 paws-skin
toes-h{s}|toes-f{s} 0,2,4 paws-skin
ear-{s}|eye-{s} * face-fur
"""
}

descriptions = {
    ("hosta-clump", 200): ("Each of the nine leaves bends on its stalk. The cutaway distinguishes the shaded inner blades, the outer tier and the petioles that supply them.", "Jedes der neun Blätter biegt sich auf seinem Stiel. Der Schnitt unterscheidet die schattigen inneren Spreiten, den äußeren Blattkranz und die versorgenden Blattstiele."),
    ("hosta-clump", 500): ("Four drooping and four rising blades surround the upright heart leaf. Each leaf keeps its stalk and mobile blade.", "Vier überhängende und vier aufsteigende Spreiten umgeben das aufrechte Herzblatt. Jedes Blatt behält seinen Stiel und seine bewegliche Spreite."),
    ("domestic-cat", 500): ("The back bends through the rump and ribcage. Four digitigrade legs, pointed ears and five tail joints retain the cat’s stance and balance.", "Der Rücken biegt sich über Kruppe und Brustkorb. Vier Zehengängerbeine, spitze Ohren und fünf Schwanzgelenke erhalten Haltung und Gleichgewicht der Katze."),
    ("herring-gull", 500): ("The long primaries pass the tail. Two neck joints carry the hooked bill above the keel, and both legs end in webbed feet.", "Die langen Handschwingen reichen über den Schwanz hinaus. Zwei Halsgelenke tragen den Hakenschnabel über dem Brustbeinkiel. Beide Beine enden in Schwimmfüßen."),
    ("mallard-duck", 500): ("The drake carries a broad flat bill above the rounded breast. Paired folded wings follow the rump, and two webbed feet spread under the shanks.", "Der Erpel trägt einen breiten flachen Schnabel über der runden Brust. Die angelegten Flügel folgen dem Bürzel. Zwei Schwimmfüße spreizen sich unter den Läufen."),
    ("pigeon", 500): ("The breast projects below a three-joint neck. The cere rises above the short bill, the wing points overlap the tail and the halluces face backward.", "Die Brust wölbt sich unter dem dreigelenkigen Hals vor. Die Wachshaut sitzt über dem kurzen Schnabel, die Flügelspitzen überdecken den Schwanz und die Hinterzehen weisen zurück."),
    ("red-fox", 500): ("Both hocks, long muzzle and pointed ears stay distinct. Three moving sections carry the sweep and pale tip of the brush.", "Beide Sprunggelenke, der lange Fang und die spitzen Ohren bleiben erkennbar. Drei bewegliche Abschnitte bilden Schwung und helle Spitze der Lunte."),
    ("red-squirrel", 200): ("A pale bib and belly contrast with the russet flanks. Both grasping hands, long soles and tufted ears articulate around the upright two-piece tail.", "Heller Brustlatz und Bauch heben sich von den rotbraunen Flanken ab. Beide Greifhände, langen Sohlen und Pinselohren begleiten den aufrechten zweiteiligen Schwanz."),
    ("red-squirrel", 500): ("The curved brush, crouched thighs and long hind feet define the seated squirrel. Each hand remains attached to its own forearm.", "Der gebogene buschige Schwanz, die angezogenen Oberschenkel und die langen Hinterfüße kennzeichnen das sitzende Eichhörnchen. Jede Hand folgt ihrem Unterarm."),
    ("hedgehog", 200): ("The quills form a low dome with four mobile skirt sectors. The pointed nose, ears, forepaws and long hind toes protrude from the coat.", "Die Stacheln bilden eine niedrige Kuppel mit vier beweglichen Randfeldern. Die spitze Nase, die Ohren, die Vorderpfoten und die langen Hinterzehen ragen aus dem Kleid."),
    ("hedgehog", 500): ("Broad dorsal and flank sections describe the curled coat. The pointed snout and all four short legs still move independently under it.", "Breite Rücken- und Flankenabschnitte beschreiben das gerundete Stachelkleid. Die spitze Schnauze und alle vier kurzen Beine bleiben darunter beweglich.")
}

thermal_extras = {
    "domestic-cat": "nose eye-n eye-f tongue pad-hn pad-hf pad-fn pad-ff",
    "pigeon": "eye eye-f",
    "red-squirrel": "nose",
}

def landmarks(spec, original):
    result = copy.deepcopy(original)
    if spec != "*":
        field = "sections" if result["kind"] == "loft" else "profile"
        indices = [int(i) for i in spec.split(",")]
        assert max(indices) < len(result[field]), (result["id"],indices,len(result[field]))
        result[field] = [result[field][i] for i in indices]
    return result

def collapse(solid, denominator):
    result = copy.deepcopy(solid)
    field = "sections" if solid.get("kind") == "loft" else "profile"
    rows = result.get(field, [])
    if len(rows) > 4:
        if denominator == 200:
            indices = [0, (len(rows) - 1) // 3, 2 * (len(rows) - 1) // 3, len(rows) - 1]
        else:
            indices = [0, max(range(1,len(rows)-1), key=lambda i: rows[i]["rx"] * rows[i]["ry"] if field=="sections" else rows[i][1]), len(rows)-1]
        result[field] = [rows[i] for i in indices]
    return result

def make(asset, denominator):
    slug = asset["id"]
    fine = asset["volumes"]["50"]
    source = {s["id"]: s for s in fine["body"]}
    coarse = next(v for s, v in reversed(asset["volumes"].items()) if asset["representations"][s]["status"]=="authored")
    body, thermal = [], []
    for line in models[slug, denominator].strip().splitlines():
        names, stations, region = line.split()
        for template in names.split("|"):
            for side, short in [("near","n"),("far","f")] if "{" in template else [("", "")]:
                name = template.format(side=side, s=short)
                body.append(landmarks(stations, source[name]))
                ref = next((r for r in fine["thermal"] if r.get("of")==name and r["region"]==region), {})
                if not ref:
                    ref = next((r for r in fine["thermal"] if r.get("id")=="th-"+name and r["region"]==region), {})
                thermal.append({"id": "th-" + name, "of": name, "region": region, **({"layer":ref["layer"]} if "layer" in ref else {})})
    for name in thermal_extras.get(slug, "").split():
        ref = next((e for e in fine["thermal"] if e.get("of")==name), None)
        region = ref["region"] if ref else next(e["region"] for e in fine["thermal"] if e["id"]=="th-"+name)
        patch = collapse(source[name], denominator)
        thermal.append({**patch, "id":"th-"+name, "region":region})
    thermal.extend(collapse(s, denominator) for s in fine["thermal"] if "emitter" in s.get("tags",[]))
    structure = []
    for s in coarse["structure"]:
        if s.get("part") == "joint" and denominator == 500: continue
        if denominator == 500 and slug == "hedgehog" and re.fullmatch("rib-[nf]-[1357]", s["id"]): continue
        if denominator == 500 and slug == "red-fox" and re.fullmatch("(near|far)-rib-(2|8)", s["id"]): continue
        if slug=="hosta-clump" and denominator==500 and s["id"]=="member-root-plate":
            s=next(m for m in asset["volumes"]["200"]["structure"] if m["id"]==s["id"])
        structure.append(collapse(s, denominator))
    en, de = descriptions[slug, denominator]
    return {"status":"authored", "denominator":denominator, "body":body, "thermal":thermal, "structure":structure,
            "decisions":[{"en":en, "de":de}]}

def replace_entry(raw, group, key, data):
    prefix = raw.index('"'+group+'": {')
    start = raw.index('"'+str(key)+'": ', prefix) + len('"'+str(key)+'": ')
    _, length = json.JSONDecoder().raw_decode(raw[start:])
    indent = len(raw[raw.rfind("\n",0,start)+1:start]) - len('"'+str(key)+'": ')
    output = json.dumps(data, ensure_ascii=False, allow_nan=False, indent=2).replace("\n","\n"+" "*indent)
    return raw[:start]+output+raw[start+length:]

for slug, denominator in models:
    path = next(catalog.glob("*/"+slug+"/*.json"))
    before = path.read_text()
    asset = json.loads(before)["asset"]
    target = make(asset, denominator)
    if asset["volumes"][str(denominator)] == target: continue
    assert path.read_text() == before
    path.write_text(replace_entry(before, "volumes", denominator, target))
    print(slug, denominator, *(len(target[k]) for k in ("body", "thermal", "structure")))

