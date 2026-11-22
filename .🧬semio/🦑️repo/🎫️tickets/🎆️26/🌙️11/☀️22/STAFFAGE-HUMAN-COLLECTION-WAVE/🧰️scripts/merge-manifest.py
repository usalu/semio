"""🧍 Merges the human worker manifest rows into the staffage manifest in one write, adds the human collections and extends the existing ones, holding the publish lock of the used-component tickets.

Usage: `python3 merge-manifest.py [--write]` (dry run without `--write`). Rows: `../📥️manifest-rows/<id>.json`.
See `../📋️brief.md` and `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json`.
"""
import json, os, pathlib, sys, time

TICKET = pathlib.Path(__file__).resolve().parent.parent
REPO = TICKET.parents[6]
CATALOG = REPO / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
MANIFEST = CATALOG / "🔣️.json"
LOCK = TICKET.parent / "RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION/publish.lock"

COLLECTIONS = [
    ("workplaces", "Workplaces", "Arbeitsplätze",
     "People at work in offices, cafés, on construction sites, in logistics and on delivery rounds.",
     "Menschen bei der Arbeit im Büro, im Café, auf der Baustelle, in der Logistik und auf Zustelltour.",
     ["standing-desk-worker", "presenter-at-flipchart", "office-worker-carrying-laptop", "barista", "waiter-with-tray",
      "postal-carrier", "worker-with-pallet-jack", "person-carrying-ladder", "construction-worker-with-shovel",
      "worker-with-jackhammer", "surveyor-with-total-station"]),
    ("care-and-health", "Care and Health", "Pflege und Gesundheit",
     "Medical staff, patients and people with aids for hospitals, practices, care homes and barrier-free design.",
     "Medizinisches Personal, Patientinnen und Patienten und Menschen mit Hilfsmitteln für Krankenhäuser, Praxen, Pflegeheime und barrierefreies Planen.",
     ["nurse-walking", "doctor-with-clipboard", "patient-with-iv-pole", "elderly-person-knitting", "mobility-scooter-user",
      "person-with-arm-sling", "person-with-guide-dog", "person-signing"]),
    ("sports-and-leisure", "Sports and Leisure", "Sport und Freizeit",
     "People playing, exercising, making music and art, taking photos and resting outdoors.",
     "Menschen beim Spielen, Trainieren, Musizieren und Malen, beim Fotografieren und beim Ausruhen im Freien.",
     ["tennis-player", "basketball-player", "hiker-with-trekking-poles", "person-stretching", "dancer",
      "standing-violinist", "painter-at-easel", "photographer-with-camera", "person-taking-selfie", "sunbather-on-towel",
      "picnicker-sitting-on-ground"]),
    ("urban-mobility", "Urban Mobility", "Mobilität in der Stadt",
     "People on the move and waiting in the city: riding, pushing, pulling luggage, queuing and going to school.",
     "Menschen unterwegs und wartend in der Stadt: fahrend, schiebend, mit Gepäck, in der Schlange und auf dem Schulweg.",
     ["e-scooter-rider", "traveller-with-rolling-suitcase", "person-pushing-bicycle", "mobility-scooter-user",
      "person-queuing", "person-checking-watch", "schoolchild-with-backpack", "teenager-with-headphones"]),
    ("everyday-life", "Everyday Life", "Alltag",
     "Household chores, eating and drinking, greetings, weather clothing and families with small children.",
     "Hausarbeit, Essen und Trinken, Begrüßungen, Wetterkleidung und Familien mit kleinen Kindern.",
     ["person-sweeping", "person-hanging-laundry", "person-carrying-shopping-bags", "person-eating-standing",
      "person-drinking-coffee", "waving-person", "person-in-rain-poncho", "person-in-winter-coat",
      "parent-with-child-on-shoulders", "walking-toddler"]),
]
EXTENSIONS = {
    "accessibility": ["mobility-scooter-user", "person-with-guide-dog", "person-signing", "person-with-arm-sling"],
    "construction-site": ["construction-worker-with-shovel", "worker-with-jackhammer", "surveyor-with-total-station"],
}


def merged() -> dict:
    """🧮 Builds the manifest with the human rows, the new collections and the extended ones."""
    manifest = json.loads(MANIFEST.read_text("utf8"))
    rows = [json.loads(p.read_text("utf8")) for p in sorted((TICKET / "📥️manifest-rows").glob("*.json"))]
    folder = CATALOG / next(c["folder"] for c in manifest["categories"] if c["id"] == "humans")
    for row in rows:
        assert row["category"] == "humans", row["id"]
        assert (folder / row["id"] / "🔣️.json").is_file(), f"missing asset {row['id']}"
    ids = {r["id"] for r in rows}
    entries = [e for e in manifest["entries"] if e["id"] not in ids]
    last = max(i for i, e in enumerate(entries) if e["category"] == "humans")
    entries = entries[: last + 1] + sorted(rows, key=lambda e: e["title"]["en"]) + entries[last + 1 :]
    known = {e["id"] for e in entries}
    covered = {a for c in COLLECTIONS for a in c[5]}
    assert ids <= covered, sorted(ids - covered)
    collections = [c for c in manifest["collections"] if c["id"] not in {k[0] for k in COLLECTIONS}]
    for collection in collections:
        extra = [a for a in EXTENSIONS.get(collection["id"], []) if a in known and a not in collection["assets"]]
        collection["assets"] = collection["assets"] + extra
    for cid, en, de, den, dde, assets in COLLECTIONS:
        missing = [a for a in assets if a not in known]
        assert not missing, (cid, missing)
        collections.append({"id": cid, "title": {"en": en, "de": de}, "description": {"en": den, "de": dde}, "assets": assets})
    for c in manifest["categories"]:
        if c["id"] == "humans":
            c["description"] = {
                "en": "Pedestrians, workers, carers, patients, children, athletes and people in everyday postures, chores and gestures, with rigs for gait, seated and working poses.",
                "de": "Passantinnen und Passanten, Arbeitende, Pflegende, Patientinnen und Patienten, Kinder, Sporttreibende und Menschen in alltäglichen Haltungen, Tätigkeiten und Gesten, mit Modellen für Gang, Sitzen und Arbeiten.",
            }
    print(f"rows {len(rows)}, entries {len(entries)}, collections {len(collections)}")
    return {**manifest, "entries": entries, "collections": collections}


def main(write: bool) -> None:
    """🔒️ Merges under the publish lock so that no publisher interleaves its manifest write."""
    if not write:
        merged()
        return
    for _ in range(600):
        try:
            os.mkdir(LOCK)
            break
        except FileExistsError:
            time.sleep(0.2)
    else:
        raise SystemExit(f"publish.lock is held for more than two minutes: {LOCK}")
    try:
        MANIFEST.write_text(json.dumps(merged(), ensure_ascii=False, indent=2) + "\n", "utf8")
    finally:
        os.rmdir(LOCK)


if __name__ == "__main__":
    main("--write" in sys.argv)
