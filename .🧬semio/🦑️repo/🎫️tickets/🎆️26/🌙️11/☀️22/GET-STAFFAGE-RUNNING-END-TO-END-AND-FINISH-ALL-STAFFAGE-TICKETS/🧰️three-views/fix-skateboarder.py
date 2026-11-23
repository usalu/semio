"""🛹️ skateboarder, 1:500: the board of the front view (deck, trucks and wheels as one typified outline) was a flat outline bound to the board bone, which only travels with it: when the tail strikes the ground in `pop` and the board stands at 30 degrees, the side elevation shows the tilted board down to the ground and the front view a level board 0.10 m above it. The front outline becomes the side outline of the board of 1:500 placed as a solid swept over the 203 mm width of the deck, so the front view shows the board as high as it stands in every pose. Run once: `python3 fix-skateboarder.py`."""
import tv2_doc

doc = tv2_doc.Doc(tv2_doc.catalog_file("skateboarder"))
path = doc.shape(500, "front", "deck")
old = doc.get(*path)
side = doc.get(*doc.shape(500, "elevation", "deck"))
xs = [p[0] for p in old["points"]]
new = {key: value for key, value in old.items() if key != "regions"}
new.update({"points": side["points"], "closed": True, "solid": {"kind": "extruded", "sweep": "w", "z0": min(xs), "z1": max(xs)}, "regions": old["regions"]})
doc.set(path, new)
doc.save()
print(f"[DEBUG] skateboarder: front deck of 1:500 is the swept side outline now, z {min(xs)}..{max(xs)}")
