"""Independent model of the early cutoff of derived facts in the graph fact index.

One command reads one JSON document on stdin and prints one JSON document:
  plan  {"owners": {name: {"raw": [...], "derived": [...]}}, "derived": {question: {"answer": s, "support": [...]}},
         "stale": [...], "answers": {question: s}}
        -> {"dirty": [...], "held": [...], "changed": [...]}
A project is dirty when it holds a stale raw row itself, or holds a derived question whose support has a stale row
and whose answer is different now (an unequal digest, `!error` included). A question whose support is stale but whose
answer is equal is held: its holders stay valid. With a `scope` (project names) only projects of the scope can be
dirty and only questions a project of the scope holds are asked; the others stay stale. Standard library only.
"""
import json
import sys


def plan(world):
    stale = set(world["stale"])
    scope = set(world["scope"]) if "scope" in world else set(world["owners"])
    direct = {name for name, row in world["owners"].items() if name in scope and stale.intersection(row["raw"])}
    held, changed = set(), set()
    for question, row in world["derived"].items():
        if not any(name in scope for name, owner in world["owners"].items() if question in owner["derived"]):
            continue
        if not stale.intersection(row["support"]):
            continue
        (held if world["answers"].get(question, "!error") == row["answer"] else changed).add(question)
    dirty = set(direct)
    for name, row in world["owners"].items():
        if name in scope and changed.intersection(row["derived"]):
            dirty.add(name)
    return {"dirty": sorted(dirty), "held": sorted(held), "changed": sorted(changed)}


if __name__ == "__main__":
    command = sys.argv[1]
    world = json.load(sys.stdin)
    if command != "plan":
        raise SystemExit("unknown command " + command)
    print(json.dumps(plan(world), sort_keys=True))
