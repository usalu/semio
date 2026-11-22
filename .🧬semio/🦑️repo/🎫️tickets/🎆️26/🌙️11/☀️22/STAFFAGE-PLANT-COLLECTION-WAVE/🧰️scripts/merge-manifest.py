"""🌿 Merges the plant worker manifest rows into the staffage manifest in one write and adds the plant collections.

Usage: `python3 merge-manifest.py [--write]` (dry run without `--write`). Rows: `../📥️manifest-rows/<id>.json`.
See `../📋️brief.md` and `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json`.
"""
import json, pathlib, sys

TICKET = pathlib.Path(__file__).resolve().parent.parent
REPO = TICKET.parents[6]
CATALOG = REPO / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
MANIFEST = CATALOG / "🔣️.json"

COLLECTIONS = [
    ("park-and-street-trees", "Park and Street Trees", "Park- und Straßenbäume",
     "Large deciduous trees for avenues, parks and squares, from maples and beech to ginkgo and plane.",
     "Große Laubbäume für Alleen, Parks und Plätze, von Ahorn und Buche bis Ginkgo und Platane.",
     ["norway-maple", "sycamore-maple", "field-maple", "horse-chestnut", "european-beech", "common-ash", "ginkgo",
      "honey-locust", "sweetgum", "tulip-tree", "red-oak", "black-locust", "english-oak", "small-leaved-lime",
      "london-plane-pollarded", "lombardy-poplar"]),
    ("flowering-and-small-trees", "Flowering and Small Trees", "Blühende und kleine Bäume",
     "Ornamental and native small trees with blossom, fruit and berry states for gardens and courtyards.",
     "Zier- und heimische Kleinbäume mit Blüten-, Frucht- und Beerenzuständen für Gärten und Höfe.",
     ["japanese-cherry", "saucer-magnolia", "crab-apple", "apple-tree", "rowan", "hawthorn", "black-alder",
      "japanese-maple", "multi-stem-birch", "weeping-willow"]),
    ("conifers", "Conifers", "Nadelbäume",
     "Evergreen and deciduous conifers from narrow columns to tiered plates, with snow load states.",
     "Immergrüne und sommergrüne Nadelbäume von schmalen Säulen bis zu Etagenkronen, mit Schneelastzuständen.",
     ["norway-spruce", "european-larch", "atlas-cedar", "italian-cypress", "english-yew", "dawn-redwood", "scots-pine",
      "stone-pine"]),
    ("mediterranean-and-desert", "Mediterranean and Desert", "Mediterran und Wüste",
     "Drought-adapted trees, palms and succulents for hot, dry climates, including CAM plants that transpire at night.",
     "Trockenheitsangepasste Bäume, Palmen und Sukkulenten für heiße, trockene Klimate, darunter CAM-Pflanzen mit nächtlicher Transpiration.",
     ["olive-tree", "fig-tree", "stone-pine", "date-palm", "windmill-palm", "saguaro-cactus", "prickly-pear", "yucca",
      "agave", "rosemary-bush", "lavender-bush"]),
    ("shrubs-and-hedges", "Shrubs and Hedges", "Sträucher und Hecken",
     "Flowering shrubs, clipped hedges and topiary for gardens, borders and boundaries.",
     "Blühende Sträucher, geschnittene Hecken und Formgehölze für Gärten, Rabatten und Grenzen.",
     ["common-lilac", "hydrangea", "rhododendron", "boxwood-ball", "privet-hedge", "cherry-laurel", "hornbeam-hedge",
      "shrub-rose", "lavender-bush", "rosemary-bush"]),
    ("grasses-and-perennials", "Grasses and Perennials", "Gräser und Stauden",
     "Ornamental grasses, reeds, perennials and tropical foliage plants with wind and transpiration states.",
     "Ziergräser, Röhricht, Stauden und tropische Blattpflanzen mit Wind- und Transpirationszuständen.",
     ["pampas-grass", "common-reed", "sunflower", "miscanthus-grass", "bamboo-clump", "hosta-clump", "banana-plant",
      "tree-fern"]),
    ("climbers-and-crops", "Climbers and Crops", "Kletterpflanzen und Nutzpflanzen",
     "Climbing plants on supports and agricultural rows for facades, pergolas and productive landscapes.",
     "Kletterpflanzen an Rankhilfen und landwirtschaftliche Reihen für Fassaden, Pergolen und produktive Landschaften.",
     ["wisteria-pergola", "ivy-on-trellis", "grapevine-row", "maize-row"]),
    ("indoor-plants", "Indoor Plants", "Zimmerpflanzen",
     "Potted foliage plants for interiors in still indoor air, with watered and dry states.",
     "Topfpflanzen für Innenräume in ruhender Raumluft, mit gegossenen und trockenen Zuständen.",
     ["potted-monstera", "fiddle-leaf-fig", "snake-plant"]),
]


def main(write: bool) -> None:
    manifest = json.loads(MANIFEST.read_text("utf8"))
    rows = [json.loads(p.read_text("utf8")) for p in sorted((TICKET / "📥️manifest-rows").glob("*.json"))]
    folder = CATALOG / next(c["folder"] for c in manifest["categories"] if c["id"] == "vegetation")
    for row in rows:
        assert row["category"] == "vegetation", row["id"]
        assert (folder / row["id"] / "🔣️.json").is_file(), f"missing asset {row['id']}"
    ids = {r["id"] for r in rows}
    entries = [e for e in manifest["entries"] if e["id"] not in ids]
    last = max(i for i, e in enumerate(entries) if e["category"] == "vegetation")
    vegetation = sorted([e for e in entries if e["category"] == "vegetation"] + rows, key=lambda e: e["title"]["en"])
    entries = [e for e in entries[: last + 1] if e["category"] != "vegetation"] + vegetation + entries[last + 1 :]
    known = {e["id"] for e in entries}
    collections = [c for c in manifest["collections"] if c["id"] not in {k[0] for k in COLLECTIONS}]
    for cid, en, de, den, dde, assets in COLLECTIONS:
        present = [a for a in dict.fromkeys(assets) if a in known]
        missing = [a for a in assets if a not in known]
        if missing:
            print(f"[DEBUG] {cid}: skipped missing {missing}")
        collections.append({"id": cid, "title": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "assets": present})
    for c in manifest["categories"]:
        if c["id"] == "vegetation":
            c["description"] = {
                "en": "Trees, conifers, palms, succulents, shrubs, hedges, grasses, climbers, crops and indoor plants with wind sway, storm, snow, flowering and fruit states and leaf transpiration over the day.",
                "de": "Bäume, Nadelbäume, Palmen, Sukkulenten, Sträucher, Hecken, Gräser, Kletter-, Nutz- und Zimmerpflanzen mit Windschwingung, Sturm-, Schnee-, Blüten- und Fruchtzuständen und Blatttranspiration im Tagesgang.",
            }
    print(f"[DEBUG] rows {len(rows)}, vegetation {len(vegetation)}, entries {len(entries)}, collections {len(collections)}")
    if write:
        MANIFEST.write_text(json.dumps({**manifest, "entries": entries, "collections": collections}, ensure_ascii=False, indent=2) + "\n", "utf8")


if __name__ == "__main__":
    main("--write" in sys.argv)
