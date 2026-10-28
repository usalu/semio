"""🔬 Compare SET-returning neutral operations through the actual independent EXPRESS set implementation."""
import json
import pathlib
import sys

ticket = pathlib.Path(__file__).resolve().parent
sys.path.insert(0, str(ticket / "🗑️generated" / "ifc083"))
import ifcopenshell
from ifcopenshell.express.rules import IFC4X3_ADD2 as rules

root = next(path for path in ticket.parents if (path / "AGENTS.md").exists())
validation = root / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation"
contract = json.loads((validation / "aggregate-kind-cases.json").read_text("utf8"))

class AggregateOracle:
    def run(self):
        names = rules.express_set(["INTEGER", "REAL", "NUMBER"])
        bag = ["x", "x", "y"]
        selected = rules.express_set(["x", "z"])
        other = rules.express_set(["x", "y"])
        return {
            "scalar-set-union": "INTEGER" + names,
            "set-scalar-union": names + "INTEGER",
            "bag-set-intersection": bag * selected,
            "set-bag-intersection": selected * bag,
            "mixed-intersection-nested-union": (bag * selected) + ["x"],
            "set-set-intersection": selected * other,
            "set-bag-union": selected + bag,
        }

actual = AggregateOracle().run()
rows = []
for case in contract["cases"]:
    if case["id"] not in actual:
        rows.append({"id": case["id"], "checked": False, "reason": "Installed express_set helper is not a general BAG implementation"})
        continue
    value = actual[case["id"]]
    observed = {"aggregate": "SET" if isinstance(value, rules.express_set) else type(value).__name__, "values": sorted(value)}
    assert observed == {"aggregate": case["aggregate"], "values": case["values"]}, (case["id"], observed)
    rows.append({"id": case["id"], "checked": True, **observed})
result = {"oracle": "IfcOpenShell", "version": ifcopenshell.version, "python": sys.version.split()[0], "nativeExecution": False, "rows": rows, "containingSelectClosureProven": False}
output = ticket / "🗑️generated" / "retained-express"
(output / "aggregate-kind-oracle.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf8")
print(json.dumps({"oracle": result["oracle"], "version": result["version"], "python": result["python"], "checked": len(actual), "matches": sum(row["checked"] for row in rows), "nativeExecution": False}))
