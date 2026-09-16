"""🪪️ Repairs process3d fixtures after `brep_child_handle`/`flow_child_handle` started minting
`target.artifact_id == child_id` (the cad/sourcing convention `ChildRestoreProjection` enforces).
Every handle already carries its child id, so the target id is derived from the record itself:
DSL hex pairs `[hex(child_id),hex("<old-id>!<kind>@<std>/<subset>")]` and JSON
`{"childId", "target": {"artifactId"}}` handles."""
import json, re, sys
from pathlib import Path

ROOT = Path("✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any")
PAIR = re.compile(r"\[([0-9a-f]+),([0-9a-f]+)\]")

def hx(s: str) -> str: return s.encode().hex()
def unhx(h: str) -> str: return bytes.fromhex(h).decode()

def repair_pair(m: re.Match) -> str:
    child_id = unhx(m.group(1)); uri = unhx(m.group(2))
    if "!" not in uri: return m.group(0)
    _, tail = uri.split("!", 1)
    return f"[{hx(child_id)},{hx(child_id + '!' + tail)}]"

def repair_dsl_text(text: str) -> str:
    return PAIR.sub(repair_pair, text)

def repair_json_value(value):
    if isinstance(value, dict):
        if "childId" in value and isinstance(value.get("target"), dict) and "artifactId" in value["target"]:
            value["target"]["artifactId"] = value["childId"]
        for v in value.values(): repair_json_value(v)
    elif isinstance(value, list):
        for v in value: repair_json_value(v)

changed = []
for path in ROOT.rglob("*"):
    if not path.is_file(): continue
    if path.name == "🗣️.dsl.semio" or path.suffix == ".rs":
        before = path.read_text(encoding="utf-8"); after = repair_dsl_text(before)
        if after != before: path.write_text(after, encoding="utf-8"); changed.append(str(path))
    elif path.suffix == ".json":
        before = path.read_text(encoding="utf-8")
        if "childId" not in before: continue
        data = json.loads(before); repair_json_value(data)
        after = json.dumps(data, ensure_ascii=False, indent=2) + "\n"
        if json.loads(after) != json.loads(before): path.write_text(after, encoding="utf-8"); changed.append(str(path))
print("\n".join(changed)); print(len(changed), "files repaired")
