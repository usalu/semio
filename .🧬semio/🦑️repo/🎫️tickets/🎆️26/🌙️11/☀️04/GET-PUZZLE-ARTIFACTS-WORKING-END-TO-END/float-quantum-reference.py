"""🧮 Independent system binary64 reference for the neutral one-work decimal corpus."""
import json
import struct
from pathlib import Path

root = Path.cwd()
corpus = root / "🧰️framework/🔨️modules/🌱️value/🛬️decode/🔢️number/🧪️laws/📁️cases.json"
results = []
for case in json.loads(corpus.read_text())["quantum"]["cases"]:
    kind = case["kind"]
    if kind == "literal":
        word = case["word"]
    elif kind == "repeat":
        word = case["prefix"] + case["repeat"] * case["count"] + case["suffix"]
    else:
        n, d = int(case["numerator"]), case["denominatorPower"]
        coefficient = n * (5 ** d if d >= 0 else 2 ** -d)
        exponent = -d if d >= 0 else 0
        variant = case["variant"]
        if variant == "exact":
            word = str(coefficient) + "e" + str(exponent)
        elif variant == "above":
            word = str(coefficient) + "5e" + str(exponent - 1)
        else:
            word = str(coefficient - 1) + "9e" + str(exponent - 1)
    try:
        actual = struct.pack(">d", float(word)).hex()
    except ValueError:
        actual = None
    assert actual == case["expectedBits"], (case, actual)
    results.append({"case": case, "length": len(word), "actualBits": actual})
output = Path(__file__).parent / "🗑️generated" / "float-quantum-reference.json"
output.parent.mkdir(exist_ok=True)
output.write_text(json.dumps(results, indent=2) + "\n")
print(f"{len(results)} independent binary64 vectors passed")
