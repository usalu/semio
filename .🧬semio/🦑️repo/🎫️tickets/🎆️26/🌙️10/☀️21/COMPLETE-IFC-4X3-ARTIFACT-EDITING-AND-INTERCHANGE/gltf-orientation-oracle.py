"""🧭 Compare neutral binary64 determinant signs with independent mpmath LU decomposition."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated" / "determinant"
output.mkdir(parents=True, exist_ok=True)
if not (output / "mpmath" / "__init__.py").is_file():
    subprocess.run([sys.executable, "-m", "pip", "install", "--disable-pip-version-check", "--no-compile", "--target", str(output), "mpmath==1.3.0"], check=True)
sys.path.insert(0, str(output))
import mpmath

mpmath.mp.prec = 16384
root = ticket.parents[6]
source = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/🧪️tests/gltf-framing-cases.json"
rows = []
for row in json.loads(source.read_text(encoding="utf-8"))["orientations"]:
    linear = [mpmath.mpf(float(value)) for value in row["linear"]]
    determinant = mpmath.det(mpmath.matrix([linear[0:3], linear[3:6], linear[6:9]]))
    sign = int(mpmath.sign(determinant))
    rows.append({"name": row["name"], "expected": row["sign"], "actual": sign})
    assert sign == row["sign"], row["name"]
report = {"implementation": "mpmath", "version": mpmath.__version__, "precisionBits": mpmath.mp.prec, "cases": rows}
(output / "oracle.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
print("[DEBUG] " + json.dumps({"cases": len(rows), "mismatches": 0, "implementation": report["implementation"], "version": report["version"]}))
