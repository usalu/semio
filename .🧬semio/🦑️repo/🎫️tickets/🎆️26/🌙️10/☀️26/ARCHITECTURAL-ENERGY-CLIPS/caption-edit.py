"""✂️ Apply reviewed narration-only overflow repairs and register portable regression cases."""
import ast
import json
from pathlib import Path

TICKET = Path(__file__).parent
WORKSPACE = next(parent for parent in TICKET.parents if (parent / "🧰️framework").is_dir())
CONTRACT = WORKSPACE / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json"
repairs = json.loads((TICKET / "caption-repairs.json").read_text(encoding="utf-8"))
contract = json.loads(CONTRACT.read_text(encoding="utf-8"))
patches, sources = {}, {}
for repair in repairs:
    path = WORKSPACE / "🎓️teaching" / repair["script"]
    source = path.read_bytes().decode("utf-8")
    assert sources.setdefault(path, source.encode("utf-8")) == source.encode("utf-8"), "Concurrent edit; retry fresh source"
    tree = ast.parse(source)
    scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == repair["scene"])
    assignment = next(node for node in scene.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
    row = next(node for node in assignment.value.elts if ast.literal_eval(node.elts[0]) == repair["key"])
    values = [ast.literal_eval(node) for node in row.elts]
    case = next((item for item in contract["captions"] if item["script"] == repair["script"] and item["scene"] == repair["scene"] and item["key"] == repair["key"]), None)
    if case is None:
        case = {"script": repair["script"], "scene": repair["scene"], "key": repair["key"], "maximum_lines": 2, "en": values[1], "de": values[2], "previous": {}}
        contract["captions"].append(case)
    case[repair["language"]] = repair["text"]
    case["previous"][repair["language"]] = repair["previous"]
    column = 1 if repair["language"] == "en" else 2
    node = row.elts[column]
    assert ast.literal_eval(node) in (repair["previous"], repair["text"]), (path, repair)
    if ast.literal_eval(node) == repair["text"]:
        continue
    lines = source.encode("utf-8").splitlines(keepends=True)
    start = sum(map(len, lines[:node.lineno - 1])) + node.col_offset
    end = sum(map(len, lines[:node.end_lineno - 1])) + node.end_col_offset
    patches.setdefault(path, []).append((start, end, json.dumps(repair["text"], ensure_ascii=False).encode("utf-8")))
CONTRACT.write_text(json.dumps(contract, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
for path, edits in patches.items():
    original = path.read_bytes()
    assert original == sources[path], "Concurrent edit; retry fresh source"
    for start, end, replacement in sorted(edits, reverse=True):
        original = original[:start] + replacement + original[end:]
    path.write_bytes(original)
    print(f"[DEBUG] Repaired {len(edits)} narration clauses in {path.relative_to(WORKSPACE)}", flush=True)
