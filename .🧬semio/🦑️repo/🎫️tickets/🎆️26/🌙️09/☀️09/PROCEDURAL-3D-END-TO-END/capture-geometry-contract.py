"""📐 Capture a native geometry-service golden only from its four observed executions."""
import json
from pathlib import Path
import sys

root = Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🔨️modules/🏠️host/📐️geometry-service")
relative = root / "🧪️tests/🧫️fixtures/🔣️.json"
text = Path(sys.argv[1]).read_text()
message = "[DEBUG] native contract " if "[DEBUG] native contract " in text else "the service must reproduce all contract cases: "
if message not in text:
    raise SystemExit("The native law has not emitted its observed contract")
observed, _ = json.JSONDecoder().raw_decode(text.split(message,1)[1])
if set(observed) != {"request", "complete", "stepped", "faulted"}:
    raise ValueError("Incomplete native service observation")
for name in ["complete", "stepped", "faulted"]:
    result = observed[name]
    if not result["widgets"] or not 0 <= result["progress"]["completed"] <= result["progress"]["total"]:
        raise ValueError("Invalid native progress")
if any([row["id"] for row in observed[name]["widgets"]] != sorted(row["id"] for row in observed[name]["widgets"]) for name in ("complete","stepped","faulted")):
    raise ValueError("The service has not yet published the required widget ordering")
if observed["complete"]["complete"] is not True or observed["stepped"]["complete"] is not False or observed["faulted"]["faulted"] < 1:
    raise ValueError("Missing terminal, stepped or faulted observations")
for parent in [Path("."), Path(".cache/g3d-native/s")]:
    fixture = parent / relative
    current = json.loads(fixture.read_text())
    if current == observed: continue
    if current:
        def logical(contract):
            import copy
            result = copy.deepcopy(contract)
            for name in ("complete", "stepped", "faulted"):
                del result[name]["fuelUsed"]
                for widget in result[name]["widgets"]:
                    del widget["dep"]
            return result
        if logical(current) != logical(observed):
            raise ValueError("Observed geometry differs beyond cache fingerprints and measured work")
        import re
        for name in ("complete", "stepped", "faulted"):
            if any(not re.fullmatch(r"[0-9a-f]{64}", widget["dep"]) for widget in observed[name]["widgets"]):
                raise ValueError("A computed cached widget lacks its complete dependency fingerprint")
    fixture.write_text(json.dumps(observed,ensure_ascii=False,indent=2)+"\n")
print("[DEBUG] Captured four native geometry service observations")
