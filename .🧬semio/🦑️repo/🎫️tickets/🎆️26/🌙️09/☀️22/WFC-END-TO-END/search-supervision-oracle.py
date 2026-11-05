"""🧮 Validate the portable paid supervision phase with a NumPy transition matrix."""
import json
from pathlib import Path
import numpy as np
fixture = json.loads(Path("✏️s/🔌️plugins/🀄️wfc/⚙️engine/🔍️search/🏃️batch/🧪️tests/🧫️phase/🔣️.json").read_text(encoding="utf-8"))
phase = np.array([1, 0], dtype=np.int64)
transition = np.array([[0, 1], [1, 0]], dtype=np.int64)
actual = []
for _ in fixture["turns"]:
    yielded = bool(phase[1])
    phase = phase @ transition
    actual.append({"yielded": yielded, "yieldNext": bool(phase[1])})
assert actual == fixture["turns"]
print("NumPy supervision phase: 3/3 portable turns match")
