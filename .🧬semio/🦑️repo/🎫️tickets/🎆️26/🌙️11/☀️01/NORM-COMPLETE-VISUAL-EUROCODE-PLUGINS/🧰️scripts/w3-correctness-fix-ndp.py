"""W3-CORRECTNESS-FIX: registers the independent-samples row of the EN 1997 characteristic values and annotates the two EN 1991 crane test-load rows whose inversion is suspected (values unchanged, unverified).
Usage: /home/deck/semio/.venv/bin/python -I w3-correctness-fix-ndp.py"""
import json, subprocess, sys

NDP = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/"
CODEGEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp/codegen.py"

SAMPLES = {
    "id": "ndp.en1997.2.characteristic-samples-per-sounding", "code": "EN 1997", "part": "2", "clause": "EN 1997-1 2.4.5.2",
    "symbol": "independent samples contributed by one investigation point to the characteristic value of a layer",
    "recommended": 1.0, "naDe": 1.0, "unit": "",
    "source": {"document": "EN 1997-1 / DIN EN 1997-1/NA", "edition": "to be recorded at verification", "locator": "EN 1997-1 2.4.5.2"},
    "status": "unverified",
    "specRef": "en1997-en1998-target.md G-02 / GI-13 give the sample set x_1..x_n without independence or a scale of fluctuation; W3 correctness review: the readings of one CPT are strongly autocorrelated, counting each as a sample drives k_n towards 0.1 and X_k towards the mean (unconservative)",
    "asWritten": "One independent sample per investigation point and layer: the mean of the readings of one sounding inside the layer (spatial averaging over the layer thickness), with the Student t factor of n - 1 degrees of freedom for n = number of samples. A value k > 1 splits the readings of one sounding into k depth segments with the same reading count and averages each (scale-of-fluctuation reading). With fewer than 3 samples the cautious estimate is the lowest reading inside the layer. UNVERIFIED: the repo spec text states neither the independence rule nor a scale of fluctuation; compare with EN 1997-1 2.4.5.2 and EN 1997-2 Annex C of the print.",
    "evidence": "",
}

CRANE = {
    "ndp.en1991.3.test-static": "Static test load Q_T = 1.1 Q_H as written on the spec line E3.04 (clause 2.9). UNVERIFIED and probably inverted: the same spec file writes the dynamic test as 1.1 on the phi_6 line (en1990-en1991-target.md l.337 'phi_6 test load factor 1.0? (dynamic test 1.1?)'), and crane practice (EN 1991-3 test loads, EN 13001) loads the static test with 1.25 Q_H and the dynamic test with phi_6 Q_H, phi_6 = 0.5 (1 + phi_2) of about 1.1. The two repo lines contradict each other, so no value is changed here; compare with the print.",
    "ndp.en1991.3.test-dynamic": "Dynamic test load Q_T = 1.25 Q_H as written on the spec line E3.04 (clause 2.9). UNVERIFIED and probably inverted: the same spec file writes the dynamic test as 1.1 on the phi_6 line (en1990-en1991-target.md l.337 'phi_6 test load factor 1.0? (dynamic test 1.1?)'), and crane practice (EN 1991-3 test loads, EN 13001) loads the static test with 1.25 Q_H and the dynamic test with phi_6 Q_H, phi_6 = 0.5 (1 + phi_2) of about 1.1. The two repo lines contradict each other, so no value is changed here; compare with the print.",
}


def main():
    path = NDP + "🔣️.json"
    text = open(path, encoding="utf-8").read()
    data = json.loads(text)
    assert json.dumps(data, indent=1, ensure_ascii=False) == text, "registry formatting changed"
    changed = 0
    for e in data["entries"]:
        if e["id"] in CRANE and e["asWritten"] != CRANE[e["id"]]:
            e["asWritten"] = CRANE[e["id"]]
            changed += 1
    if SAMPLES["id"] not in {e["id"] for e in data["entries"]}:
        data["entries"] = sorted(data["entries"] + [SAMPLES], key=lambda e: e["id"])
        changed += 1
    if not changed:
        print("registry already complete")
        return
    open(path, "w", encoding="utf-8").write(json.dumps(data, indent=1, ensure_ascii=False))
    subprocess.run([sys.executable, "-I", CODEGEN, path, NDP + "🗃️table.rs"], check=True)
    print("changed", changed)


main()
