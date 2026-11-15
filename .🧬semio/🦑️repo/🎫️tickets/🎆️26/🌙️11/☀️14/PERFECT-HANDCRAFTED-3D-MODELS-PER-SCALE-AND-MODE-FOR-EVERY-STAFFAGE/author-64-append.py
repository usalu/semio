"""➕ Author 64: appends a justification sentence to the structure decision of given asset-scales (one member solid is right for the object). Idempotent via a marker; atomic; skips sources touched in the last 20 minutes."""
import json, os, sys, time
T = os.path.dirname(os.path.abspath(__file__))
JOBS = {
    "appliances/ceiling-diffuser": {"50": None, "100": None, "200": None, "_": (
        "One member solid is right here: the diffuser is a single rigid part (one rig bone) that hangs from its two ceiling hangers, so the flange plate is the whole load path and the cones and collar are cladding.",
        "Ein Stabkörper ist hier richtig: der Auslass ist ein einziges starres Teil (eine Rig-Bone), das an seinen zwei Deckenhaltern hängt; die Flanschplatte ist der ganze Lastpfad, Kegel und Stutzen sind Verkleidung.")},
    "components/panel-radiator": {"200": None, "_": (
        "One member solid is right at this scale: the radiator is one rigid panel block on its brackets and the brackets are merged into it because the 1:200 drawing does not draw them; the valve head is the only joint.",
        "Ein Stabkörper ist in diesem Maßstab richtig: der Heizkörper ist ein starrer Plattenblock auf seinen Konsolen, und die Konsolen sind darin aufgegangen, weil die Zeichnung 1:200 sie nicht zeigt; der Ventilkopf ist das einzige Gelenk.")},
    "components/steel-column": {"200": None, "500": None, "_": (
        "One member solid is right at this scale: the HEB 200 shaft is merged into one column member (its flanges and web are no longer separate drawn shapes), with its footing support and mass point.",
        "Ein Stabkörper ist in diesem Maßstab richtig: der HEB-200-Schaft ist zu einem Stützenstab zusammengefasst (Flansche und Steg sind keine getrennten Zeichnungsformen mehr), mit Fußauflager und Massenpunkt.")},
    "components/solid-brick-wall-1960": {"100": None, "200": None, "500": None, "_": (
        "One member solid is right: the masonry leaf is the only load-bearing layer of this wall (the render is cladding), carried by one base support.",
        "Ein Stabkörper ist richtig: die Mauerwerksschale ist die einzige tragende Schicht dieser Wand (der Putz ist Verkleidung), getragen von einem Fußauflager.")},
}
MARK = "One member solid is right"
for key, job in JOBS.items():
    path = f"{T}/volumes/{key}.json"
    if (time.time() - os.path.getmtime(path)) / 60 < 20 and "--force-age" not in sys.argv:
        print("SKIP", key); continue
    raw = open(path).read()
    src = json.loads(raw)
    en, de = job["_"]
    for s in job:
        if s == "_":
            continue
        v = src["volumes"][s]
        decs = v["decisions"]
        if any(MARK in d["en"] for d in decs):
            continue
        idx = next((i for i, d in enumerate(decs) if "tructure" in d["en"] and not d["en"].startswith("Minimal")), None)
        if idx is None:
            decs.insert(max(len(decs) - 1, 0), {"en": en, "de": de})
        else:
            decs[idx] = {"en": decs[idx]["en"] + " " + en, "de": decs[idx]["de"] + " " + de}
    out = json.dumps(src, ensure_ascii=False, separators=(",", ":")) + ("\n" if raw.endswith("\n") else "")
    assert open(path).read() == raw
    open(path, "w").write(out)
    print("APPENDED", key)
