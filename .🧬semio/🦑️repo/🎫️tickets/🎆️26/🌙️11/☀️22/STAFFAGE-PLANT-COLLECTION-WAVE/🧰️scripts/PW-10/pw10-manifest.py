"""📥️ Writes one manifest row per PW-10 asset to `📥️manifest-rows/<id>.json`, in the shape of the `entries` rows of the catalog manifest, derived from the shipped asset document (power is 0 W: plants have no rated element and no living body)."""
import json
import pathlib
import sys

WAVE = pathlib.Path(__file__).resolve().parents[2]
CATALOG = WAVE.parents[6] / "🎓️teaching" / "🏛️architecture" / "🧍️staffage" / "🗂️catalog" / "🌳️vegetation"
DISTINCTION = {
    "windmill-palm": {
        "en": "The hardy fan palm of temperate gardens: an 8 m stem wrapped in brown leaf-sheath fibre under a round crown of stiff palmate leaves with split, drooping segments, plus a skirt of dead leaves; fans fold and roll edge-on in wind and bend under snow.",
        "de": "Die winterharte Fächerpalme gemäßigter Gärten: ein 8 m hoher, in braune Blattscheidenfasern gehüllter Stamm unter einer runden Krone steifer handförmiger Blätter mit geteilten, überhängenden Segmenten, dazu ein Rock toter Blätter; die Fächer falten und drehen sich im Wind hochkant und biegen sich unter Schnee.",
    },
    "banana-plant": {
        "en": "A herbaceous clump, not a tree: a soft pseudostem of rolled leaf sheaths with nine huge paddle leaves torn into strips by the wind, a furled cigar leaf, dead leaves and two suckers; the whole plant leans and streams in wind and its leaves fold in the midday heat.",
        "de": "Ein krautiger Horst, kein Baum: ein weicher Scheinstamm aus gerollten Blattscheiden mit neun riesigen, vom Wind in Streifen gerissenen Paddelblättern, einem eingerollten Zigarrenblatt, toten Blättern und zwei Kindeln; die ganze Pflanze neigt sich und strömt im Wind, und ihre Blätter falten sich in der Mittagshitze.",
    },
    "tree-fern": {
        "en": "A fern with a trunk of roots: a 4 m column of fibrous root mantle flaring at the base, crowned by a 5 m shuttlecock of arching bipinnate fronds with unrolling crosiers and a skirt of dead fronds; it stands in light shade and its fronds sag in a dry spell.",
        "de": "Ein Farn mit einem Stamm aus Wurzeln: eine 4 m hohe Säule aus faserigem Wurzelmantel, am Fuß verbreitert, gekrönt von einem 5 m weiten Federball überhängender doppelt gefiederter Wedel mit sich entrollenden Farnschnecken und einem Rock toter Wedel; er steht im lichten Schatten und seine Wedel hängen in einer Trockenperiode durch.",
    },
}

for slug in sys.argv[1:] or list(DISTINCTION):
    a = json.loads((CATALOG / slug / "🔣️.json").read_text())["asset"]
    row = {
        "id": a["id"], "category": "vegetation", "title": a["title"],
        "heightM": a["dimensions"]["heightM"], "widthM": a["dimensions"]["widthM"], "powerW": 0,
        "poses": [p["id"] for p in a["poses"]], "clips": [c["id"] for c in a["clips"]],
        "representations": {k: a["representations"][k]["status"] for k in ["50", "100", "200", "500"]},
        "symbols": [s["id"] for s in a.get("symbols", [])],
        "analytics": ["thermal", "transport", "structural"], "exports": a["exports"],
        "distinction": DISTINCTION[slug], "completion": a["completion"],
    }
    out = WAVE / "📥️manifest-rows" / f"{slug}.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(row, ensure_ascii=False, indent=2) + "\n")
    print(f"written {out}")
