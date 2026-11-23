"""💼️ seated-office-worker, all four scales: the two hands of the front view and of the plan are one solid, the outline of the hand in the plane of the palm swept 0.8 mm along its normal, but the front view drew that outline 30 to 90 mm longer than the plan and the side elevation (front 0.19 to 0.23 m along the hand; plan and side 0.135 to 0.175 m, the fingers curled over the keys). While he leans back and the forearms swing up (clip `settle-back` at 4.2 s) the front view therefore showed the left hand up to 1.61 m and the side elevation to 1.54 m. The front hand takes the outline of the plan hand of its scale: one solid, one outline. Run once: `python3 fix-seated-office-worker.py`."""
import tv2_doc

doc = tv2_doc.Doc(tv2_doc.catalog_file("seated-office-worker"))
for scale in (50, 100, 200, 500):
    for hand in ("hand-l", "hand-r"):
        path = doc.shape(scale, "front", hand)
        plan = doc.get(*doc.shape(scale, "plan", hand))
        if doc.get(*path)["solid"] != plan["solid"]:
            raise SystemExit(f"1:{scale} {hand}: front and plan are not the same solid")
        doc.set(path + ("points",), plan["points"])
doc.save()
print("[DEBUG] seated-office-worker: 8 front hands carry the outline of the plan hand now")
