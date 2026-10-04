"""Rewrite the crowd-client fixture for the challenge levels (design §1 "Crowd?", §5 "Showing").

The gate gains a fact: `hidden`, whether a run of the quiz that hides the keys is open. Such a run never offers the
others in the run nor on the quiz's page (they would show the keys); the results stay as they were. The gates stay the
whole truth table (3 choices x 3 places x asked x submitted x hidden). The fixture's sheet is dealt at a challenge now:
it gets `challenge: "medium"` (its matching cards and sorting items are those of a sheet that shows the keys).
Text edits only, so the hand-authored layout of the rest of the file stays. Idempotent.
Run: python crowd_gate_hidden_fixture.py
"""

import json
import os
import pathlib
import re

path = pathlib.Path("C:/git/semio/🧰️framework/🛍️products/❓️quiz/🧫️fixtures/💭️crowd-client/🔣️.json")
raw = path.read_bytes().decode("utf-8").replace("\r\n", "\n")
text = raw[: raw.index("\n}\n") + 3]


def gate(others, place, asked, submitted, hidden):
    if others == "never" or (hidden and place != "results"):
        return "off"
    if others == "always" or place == "results" or (place == "quiz" and submitted):
        return "open"
    return "asked" if asked else "locked"


def flag(value):
    return "true" if value else "false"


lines = [
    f'    {{ "others": "{others}", "place": "{place}", "asked": {flag(asked)}, "submitted": {flag(submitted)}, "hidden": {flag(hidden)}, "expected": "{gate(others, place, asked, submitted, hidden)}" }}'
    for others in ["never", "submitted", "always"]
    for place in ["run", "quiz", "results"]
    for asked in [False, True]
    for submitted in [False, True]
    for hidden in [False, True]
]
gates = '  "gates": [\n' + ",\n".join(lines) + "\n  ],"
text, count = re.subn(r'  "gates": \[\n.*?\n  \],', lambda _: gates, text, count=1, flags=re.S)
assert count == 1, "gates block not found"

OLD = "and whether the learner has submitted a run of the quiz:"
NEW = "whether the learner has submitted a run of the quiz and whether a run of the quiz that hides the keys is open (`hidden`, which turns the gate `off` in the run and on the quiz's page, never on the results):"
if OLD in text:
    text = text.replace(OLD, NEW, 1)
assert NEW in text, "description anchor moved"

text = text.replace('"schema": "semio.quiz.crowd-client/v2"', '"schema": "semio.quiz.crowd-client/v3"', 1)
if '"challenge": "medium"' not in text.split('"others": [', 1)[0]:
    text = text.replace('    "seed": 7,\n    "title"', '    "seed": 7,\n    "challenge": "medium",\n    "title"', 1)

quiz = json.loads(text)["quiz"]
for task in json.loads(text)["sheet"]["tasks"]:
    if task["kind"] != "sorting" or "keys" in task:
        continue
    source = next(candidate for candidate in quiz["tasks"] if candidate["id"] == task["id"])
    values = {item["id"]: item["value"] for item in source["items"]}
    keys = sorted(values[item["id"]] for item in task["items"])
    start = text.index('"sheet": {')
    start = text.index(f'"id": "{task["id"]}"', start)
    found = re.compile(r'(\n        "quantity": [^\n]*,)(\n        "items": \[)').search(text, start)
    assert found is not None, "sorting quantity anchor moved"
    text = text[: found.end(1)] + f'\n        "keys": [{", ".join(str(key) for key in keys)}],' + text[found.end(1) :]

document = json.loads(text)
assert document["sheet"]["challenge"] == "medium"
assert all("keys" in task for task in document["sheet"]["tasks"] if task["kind"] == "sorting")
assert len(document["gates"]) == 72
data = text.encode("utf-8")
staged = path.with_name("🔣️.json.next")
staged.write_bytes(data)
os.replace(staged, path)
assert path.read_bytes() == data, "the file was not rewritten whole"
print(len(document["gates"]), "gates;", sum(1 for entry in document["gates"] if entry["expected"] == "off"), "off")
