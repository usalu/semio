"""Prints the shape of the `icons-1*` sheets of the sheet-assembly fixture: per challenge, each task's kind, items, seconds and which keys it carries."""

import json
from pathlib import Path

root = Path(__file__).resolve().parents[7]
fixture = json.loads((root / "🧰️framework" / "🛍️products" / "❓️quiz" / "🧫️fixtures" / "🃏️sheet-assembly" / "🔣️.json").read_text(encoding="utf-8"))
for vector in fixture["sheets"]:
    if not vector["id"].startswith("icons-1"):
        continue
    sheet = vector["sheet"]
    print(vector["id"], sheet.get("challenge"))
    for task in sheet["tasks"]:
        facts = [task["kind"], task["id"], f"items={len(task['items'])}", f"seconds={task.get('seconds')}"]
        if task["kind"] == "sorting":
            facts.append(f"keys={task.get('keys')}")
        if task["kind"] == "matching":
            facts.append("cards=" + ",".join(str(len(d.get("cards", []))) if "cards" in d else "none" for d in task["dimensions"]))
        if task["kind"] == "classification":
            facts.append(f"axes={[sorted(a.keys()) for a in task.get('axes', [])][:1]} desc={[('description' in c) for c in task['categories']]}")
        print("  ", " ".join(facts))
