#!/usr/bin/env python3
"""🖨️ CD1: takes the `print_step_exchange_fixture` capture (cargo test output with the regenerated fixture JSON printed at column 0)
and writes that JSON as the step-exchange fixture of the given tree root. Usage: cd1-fill-fixture.py <capture.txt> <tree-root>"""
import json
import sys

FIXTURE = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧫️fixtures/📤️step-exchange/🔣️.json"
lines = open(sys.argv[1], encoding="utf-8").read().splitlines()
start = lines.index("{")
end = start + next(i for i, line in enumerate(lines[start:]) if line == "}")
root = json.loads("\n".join(lines[start:end + 1]))
assert root["schema"] == "s.stdio.semio.brep.step-exchange/v1" and all("step" in case for case in root["cases"]), "regenerated fixture incomplete"
open(f"{sys.argv[2]}/{FIXTURE}", "w", encoding="utf-8").write(json.dumps(root, indent=2, ensure_ascii=False) + "\n")
print(f"fixture: {len(root['cases'])} cases, {sum(len(case['step']) for case in root['cases'])} exchange bytes → {FIXTURE}")
