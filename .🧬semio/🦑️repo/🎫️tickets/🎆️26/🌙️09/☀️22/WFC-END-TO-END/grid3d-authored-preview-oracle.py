"""🧮️ NumPy independently projects the portable current authored grid preview."""
import json
from pathlib import Path
import numpy as np

fixture = Path("✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/🧫️authored-preview/🔣️.json")
cases = json.loads(fixture.read_text(encoding="utf-8"))["cases"]
for case in cases:
    positions = np.array([pin[:3] for pin in case["pins"]], dtype=np.int64).reshape(-1, 3)
    tiles = np.array([pin[3] for pin in case["pins"]], dtype=str)
    masked = np.array(case["masked"], dtype=np.int64).reshape(-1, 3)
    valid = (positions < np.array(case["size"], dtype=np.int64)).all(axis=1)
    valid &= np.isin(tiles, np.array(case["tiles"], dtype=str))
    if len(masked):
        valid &= ~(positions[:, None, :] == masked[None, :, :]).all(axis=2).any(axis=1)
    projected = {tuple(map(int, xyz)): str(tile) for xyz, tile in zip(positions[valid], tiles[valid])}
    actual = [[*xyz, projected[xyz]] for xyz in sorted(projected)]
    assert actual == case["expected"], (case["id"], actual, case["expected"])
print(json.dumps({"oracle": "numpy", "cases": len(cases), "matched": len(cases)}))
