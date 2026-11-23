"""🛌️ sleeping-person, 1:50 front view: the left arm was not drawn at all (sleeve, forearm sleeve, cuff and hand of the right arm are there; 1:100 to 1:500 draw both arms). Under the duvet of the default pose nobody sees it, but while he turns over (clip `turn-over` at 3.2 s) the left arm swings up to 0.92 m and the side elevation and the plan show it. The four shapes are added as the same solids the other two views place: the three sleeves are the bodies of revolution about their bones that the side elevation draws, the hand is the side profile of the right hand of the front view at the lateral rest position of the left hand (the plan places the left hand there). Run once: `python3 fix-sleeping-person.py`."""
import tv2_doc

doc = tv2_doc.Doc(tv2_doc.catalog_file("sleeping-person"))
front = ("asset", "representations", "50", "geometry", "front")
ids = [shape["id"] for shape in doc.get(*front)]
after = ids.index("hand-r") + 1
added = []
for source_view, source_id, new_id, z in (("elevation", "sleeve-l", "sleeve-l", 1.1), ("elevation", "foresleeve-l", "foresleeve-l", 1.3), ("elevation", "cuff-l", "cuff-l", 1.4)):
    added.append({**doc.get(*doc.shape(50, source_view, source_id)), "id": new_id, "z": z})
right = doc.get(*doc.shape(50, "front", "hand-r"))
left_plan = doc.get(*doc.shape(50, "plan", "hand-l"))
added.append({**right, "id": "hand-l", "bone": "hand-l", "solid": dict(left_plan["solid"])})
for offset, shape in enumerate(added):
    if shape["id"] in ids:
        raise SystemExit(f"front already has {shape['id']}")
    doc.insert(front, after + offset, shape)

doc.save()
print(f"[DEBUG] sleeping-person: front 1:50 got {[s['id'] for s in added]}")
