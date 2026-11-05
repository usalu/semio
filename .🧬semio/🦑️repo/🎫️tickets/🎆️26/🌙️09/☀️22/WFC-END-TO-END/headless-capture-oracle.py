"""🧮️ Independently checks capture page frontiers and output bytes with NumPy."""
import json
from pathlib import Path
import numpy as np

root = Path("🧰️framework/🔨️modules/🧵️job/🏃️headless/📥️capture/🧪️tests")
law = json.loads((root / "frontiers.json").read_text(encoding="utf-8"))
values = (np.arange(law["length"], dtype=np.uint64) % law["modulus"]).astype(np.uint8)
page_lengths = [len(values[index:index + law["pageBytes"]]) for index in range(0, len(values), law["pageBytes"])]
copies = int(np.ceil(np.asarray(page_lengths) / law["copyBytes"]).sum())
actual = {"pages": len(page_lengths), "copyTurns": copies, "advanceTurns": copies + 2, "checksum": int(values.sum(dtype=np.uint64))}
assert actual == law["expected"], (actual, law["expected"])
assert len(law["cases"]) == len(set(law["cases"])) == 8
print("[DEBUG] NumPy agrees with capture bytes/page/copy/work frontiers for all 8 named cases")
