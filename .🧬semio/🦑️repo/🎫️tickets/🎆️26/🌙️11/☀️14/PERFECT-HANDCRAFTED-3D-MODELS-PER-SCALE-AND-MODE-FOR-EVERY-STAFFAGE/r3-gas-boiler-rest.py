"""🧱 Round 3, appliances/gas-boiler: the lower wall rest as a second support. With the wall bracket at its true depth (0.165 m behind the mid-plane) one pin leaves the overturning moment of the weight unbalanced; the real boiler leans with the lower edge of its back sheet against the wall. Adds the anchor `wall-rest` (roller, normal away from the wall), lists it in `structure.supports`, the pose and clip contacts, and rewrites the structure assumption with the statics. Writes the candidate to 🗑️generated/r3-gas-boiler/🔣️.json and validates it; `--apply` writes the changed members into the catalog file.

Usage: python3 r3-gas-boiler-rest.py [--apply]
"""
import copy
import importlib.util
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("roots", os.path.join(HERE, "r3-gas-boiler-roots.py"))
R = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(R)

REST = {"id": "wall-rest", "bone": "casing", "x": 0, "y": 0.075, "z": 0.165, "kind": "support", "title": {"en": "Lower edge of the back sheet resting against the wall", "de": "Unterkante des Rückblechs an der Wand anliegend"}, "support": "roller", "normalDeg": 0, "tiltDeg": -90, "footprintLengthM": 0.03, "footprintWidthM": 0.3, "frictionCoefficient": 0.3}
ASSUME = {"en": "The 30 kg boiler hangs with the pressed pocket of its back sheet on the wall rail at 0.74 m (pin) and rests with the lower edge of the back sheet at 0.075 m against the wall (roller pressed by the wall): the rail carries the weight of 292 N, and the pair resists the overturning moment of the centre of mass 0.165 m in front of the rail with about 73 N of pull at the rail and of push at the lower edge (292 N x 0.165 m / 0.665 m).",
          "de": "Der 30 kg schwere Kessel hängt mit der gepressten Tasche seines Rückblechs an der Wandschiene auf 0,74 m (Festlager) und lehnt mit der Unterkante des Rückblechs auf 0,075 m an der Wand (Rollenlager, von der Wand gedrückt): die Schiene trägt das Gewicht von 292 N, und das Paar nimmt das Kippmoment des Schwerpunkts 0,165 m vor der Schiene mit etwa 73 N Zug an der Schiene und Druck an der Unterkante auf (292 N x 0,165 m / 0,665 m)."}


def rest(asset):
    """🧷 Adds the wall rest everywhere it belongs."""
    a = copy.deepcopy(asset)
    if not any(x["id"] == "wall-rest" for x in a["anchors"]):
        i = next(k for k, x in enumerate(a["anchors"]) if x["id"] == "wall-bracket")
        a["anchors"].insert(i + 1, REST)
    if "wall-rest" not in a["structure"]["supports"]:
        a["structure"]["supports"].append("wall-rest")
    a["structure"]["assumptions"][0] = ASSUME
    for p in a["poses"]:
        if "wall-rest" not in p["contacts"]:
            p["contacts"].append("wall-rest")
    for c in a["clips"]:
        if not any(x["anchor"] == "wall-rest" for x in c["contacts"]):
            first = c["contacts"][0]
            c["contacts"].append({"anchor": "wall-rest", "fromS": first["fromS"], "toS": first["toS"]})
    return a


def main():
    """🏗️ Builds, validates and optionally applies."""
    doc = json.load(open(R.CATALOG, encoding="utf-8"))
    doc["asset"] = rest(doc["asset"])
    doc["asset"]["volumes"] = json.load(open(R.SOURCE, encoding="utf-8"))["volumes"]
    os.makedirs(R.OUT, exist_ok=True)
    json.dump(doc, open(R.CANDIDATE, "w", encoding="utf-8"), indent=2, ensure_ascii=False)
    bun = sorted(os.popen("ls ~/Library/Caches/semio/runtimes/bun/*/bun").read().split())[0]
    r = subprocess.run([bun, "r3-validate.ts", "--file", R.CANDIDATE, "--volumes"], cwd=HERE, capture_output=True, text=True)
    print(r.stdout[-3000:], r.stderr[-1000:])
    if "--apply" in sys.argv:
        if r.returncode != 0:
            raise SystemExit("candidate INVALID: not applied")
        want = doc["asset"]
        text = open(R.CATALOG, encoding="utf-8").read()
        have = json.loads(text)["asset"]
        changed = []
        for key in want:
            if key == "volumes" or have.get(key) == want[key]:
                continue
            start, end = R.NS["member_range"](text, "    ", key)
            old = text[start:end]
            style = R.indent2 if R.indent2(have[key]) in old else (lambda v: R.js_numbers(R.NS["dump"](v, 2)))
            text = text[:start] + "    " + json.dumps(key) + ": " + style(want[key]) + text[end:]
            changed.append(key)
        got = json.loads(text)["asset"]
        if any(got[k] != want[k] for k in want if k != "volumes") or list(got) != list(have):
            raise SystemExit("write check failed")
        open(R.CATALOG, "w", encoding="utf-8").write(text)
        print("written members:", ", ".join(changed))


if __name__ == "__main__":
    main()
