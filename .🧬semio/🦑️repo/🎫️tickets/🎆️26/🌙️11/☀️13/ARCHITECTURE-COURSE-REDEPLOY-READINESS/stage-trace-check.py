"""🔍️ Runs the pets stage-trace keeper's `verify` on the committed vectors without the harness adapter."""
import json, pathlib, sys, types, importlib.util
stub = types.ModuleType("semio_repo_test"); stub.Adapter = object; stub.Outcome = object; sys.modules["semio_repo_test"] = stub
root = pathlib.Path(sys.argv[1])
spec = importlib.util.spec_from_file_location("keeper", root / "🧰️framework/🛍️products/🐾️pets/🧪️tests/🎪️stage-trace/🐍️.py")
keeper = importlib.util.module_from_spec(spec); spec.loader.exec_module(keeper)
document = json.loads((root / "🧰️framework/🛍️products/🐾️pets/🧪️tests/🧫️fixtures/🎪️stage-trace/🔣️.json").read_text())
keeper.verify(document)
print("stage-trace ok:", len(document["scripts"]), "scripts")
