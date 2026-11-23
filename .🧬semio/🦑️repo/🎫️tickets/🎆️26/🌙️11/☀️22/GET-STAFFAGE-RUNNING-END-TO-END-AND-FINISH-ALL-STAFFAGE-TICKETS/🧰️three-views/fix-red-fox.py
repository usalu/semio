"""🦊️ red-fox, 1:50, 1:100 and 1:200: the two fore paws of the front view were profiles revolved about the paw's own normal, a disc 40 mm wide whatever way the paw points; in `curled-sleeping` the fore legs are folded sideways and the plan shows the paws lying across (0.42 m to the left), the front view only 0.38 m. They become the side profile of the paw of that scale placed as a solid swept over the width of the paw, as the side elevation already places it, at the lateral rest position of the paw (z ±0.032 m): the front view then follows the paw wherever it points. Run once: `python3 fix-red-fox.py`."""
import tv2_doc

REST_Z = {"paw-f-f": 0.032, "paw-f-n": -0.032}

doc = tv2_doc.Doc(tv2_doc.catalog_file("red-fox"))
for scale in (50, 100, 200):
    for shape_id, z in REST_Z.items():
        path = doc.shape(scale, "front", shape_id)
        old = doc.get(*path)
        side = doc.get(*doc.shape(scale, "elevation", shape_id))
        half = side["solid"]["z1"]
        doc.set(path, {**old, "points": side["points"], "solid": {"kind": "extruded", "sweep": "w", "z0": round(z - half, 5), "z1": round(z + half, 5)}})
doc.save()
print("[DEBUG] red-fox: 6 fore paw shapes of the front view are swept side profiles now")
