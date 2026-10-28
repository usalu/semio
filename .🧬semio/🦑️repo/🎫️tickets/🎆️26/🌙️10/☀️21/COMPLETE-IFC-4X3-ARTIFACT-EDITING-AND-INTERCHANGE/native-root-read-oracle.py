"""🧪 Compare derived recomputation through owned SDK and SQLite test interfaces."""
import json, pathlib, platform, sqlite3
import ifcopenshell

class DerivedOracle:
    def evaluate(self, name):
        model = ifcopenshell.file(schema="IFC4X3_ADD2")
        direction = model.create_entity("IfcDirection", (1.0, 2.0))
        if name in ("local-scalar-dependency", "constructed-scalar-dependency"):
            before = direction.Dim
            direction.DirectionRatios = (1.0, 2.0, 3.0)
            return before * 10 + direction.Dim
        if name == "nested-derived-dependency":
            direction.DirectionRatios = (1.0, 2.0, 3.0)
            vector = model.create_entity("IfcVector", direction, 1.0)
            before = vector.Dim
            direction.DirectionRatios = (1.0, 2.0)
            return before * 10 + vector.Dim
        axis = model.create_entity("IfcAxis2Placement3D", model.create_entity("IfcCartesianPoint", (0.0, 0.0, 0.0)), None, None)
        first = axis.P[0]
        words = list(first.DirectionRatios)
        words[0] = 9.0
        first.DirectionRatios = words
        return first.DirectionRatios[0] * 10 + axis.P[0].DirectionRatios[0]

def main():
    root = pathlib.Path(__file__).resolve().parent
    repo = root.parents[6]
    validation = repo / "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation"
    cases = json.loads((validation / "root-memo-cases.json").read_text(encoding="utf-8"))["cases"]
    observations = []
    oracle = DerivedOracle()
    connection = sqlite3.connect(":memory:")
    for row in cases:
        try:
            actual = oracle.evaluate(row["id"])
            observation = {"id": row["id"], "status": "exact" if float(row["expected"]) == actual else "different", "value": actual}
        except Exception as error:
            observation = {"id": row["id"], "status": "unsupported", "error": {"type": type(error).__name__, "message": str(error)}}
        if row["expected"] != "91":
            before, after = (3, 2) if row["expected"] == "32" else (2, 3)
            observation["sqlite"] = connection.execute("SELECT ?*10+?", (before, after)).fetchone()[0]
        else:
            observation["sqlite"] = {"status": "outside-interface", "reason": "No EXPRESS constructed entity identity in SQLite scalar adapter"}
        observations.append(observation)
    output = {"python": platform.python_version(), "ifcOpenShell": ifcopenshell.version, "sqlite": sqlite3.sqlite_version, "scope": "Derived dependency and recomputation; SDK entities are intentionally mutated, not a source-isolation proof", "nativeExecuted": False, "cases": observations}
    (root / "🗑️generated/native-root-read-oracle.json").write_text(json.dumps(output, indent=2) + "\n", encoding="utf-8")
    print("[DEBUG] " + json.dumps({"cases": len(observations), "statuses": [row["status"] for row in observations]}))

if __name__ == "__main__":
    main()
