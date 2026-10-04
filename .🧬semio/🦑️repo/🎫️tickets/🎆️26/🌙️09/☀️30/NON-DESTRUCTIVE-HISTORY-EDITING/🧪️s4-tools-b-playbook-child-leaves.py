"""🧬️ S4-TOOLS-B — playbook child-lane leaf vectors, written by an independent Python implementation (design §20.15).

Second implementation of the playbook `flow`-child edit semantics: the step ORDER is the chain of `sequence` edges
(`seq-<a>-<b>`, port `next` → `prev`), not the node vector; every verb yields the exact stdio flow leaf kinds in applied order,
and the steps it leaves are computed here by plain list operations (never by folding leaves), so the Rust law
`child_leaf_vectors_hold_and_every_edit_undoes_exactly` (which folds the Rust leaves through the stdio fold and undoes them)
checks two implementations against each other. Writes `🗿️artifacts/📖️playbook/🧫️fixtures/🧫️child-leaves/🔣️.json`;
`--check` exits 1 when the committed file differs.
"""
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
OUT = ROOT / "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🧫️fixtures/🧫️child-leaves/🔣️.json"


def block(block_id, kind="text", label=None):
    return {"id": block_id, "label": label or block_id, "kind": kind}


def step(step_id, blocks=(), title=None):
    return {"id": step_id, "title": title or step_id.capitalize(), "blocks": list(blocks)}


def chain(steps):
    return [(a["id"], b["id"]) for a, b in zip(steps, steps[1:])]


def chain_kinds(present, wanted):
    removed = ["remove-edge" for pair in present if pair not in wanted]
    inserted = ["insert-edge" for pair in wanted if pair not in present]
    return removed + inserted


def pairs(order):
    return list(zip(order, order[1:]))


def run(base, verb):
    ids = [s["id"] for s in base]
    present = chain(base)
    kind = verb["kind"]
    after = json.loads(json.dumps(base))
    find = {s["id"]: s for s in after}
    if kind == "add-step":
        if verb["step"]["id"] in ids:
            return {"refusal": "playbook.step.duplicate"}
        order = ids + [verb["step"]["id"]]
        return {"leaves": ["insert-node"] + chain_kinds(present, pairs(order)), "after": after + [verb["step"]]}
    if kind in ("remove-step", "move-step"):
        if verb["stepId"] not in ids:
            return {"refusal": "playbook.step.missing"}
        order = [i for i in ids if i != verb["stepId"]]
        if kind == "remove-step":
            return {"leaves": chain_kinds(present, pairs(order)) + ["remove-node"], "after": [s for s in after if s["id"] != verb["stepId"]]}
        order.insert(min(verb["index"], len(order)), verb["stepId"])
        return {"leaves": chain_kinds(present, pairs(order)), "after": [find[i] for i in order]}
    if kind == "add-block":
        target = find.get(verb["stepId"])
        if target is None:
            return {"refusal": "playbook.step.missing"}
        index = verb.get("index", len(target["blocks"]))
        target["blocks"].insert(min(index, len(target["blocks"])), verb["block"])
        return {"leaves": ["set-node-param"], "after": after}
    if kind == "remove-block":
        target = find.get(verb["stepId"])
        if target is None:
            return {"refusal": "playbook.step.missing"}
        if not any(b["id"] == verb["blockId"] for b in target["blocks"]):
            return {"refusal": "playbook.block.missing"}
        target["blocks"] = [b for b in target["blocks"] if b["id"] != verb["blockId"]]
        return {"leaves": ["set-node-param"], "after": after}
    if kind == "move-block":
        source, target = find.get(verb["fromStepId"]), find.get(verb["toStepId"])
        if source is None or target is None:
            return {"refusal": "playbook.step.missing"}
        moved = next((b for b in source["blocks"] if b["id"] == verb["blockId"]), None)
        if moved is None:
            return {"refusal": "playbook.block.missing"}
        source["blocks"].remove(moved)
        target["blocks"].insert(min(verb["index"], len(target["blocks"])), moved)
        return {"leaves": ["set-node-param"] if source is target else ["set-node-param", "set-node-param"], "after": after}
    raise SystemExit(f"unknown verb {kind}")


ABC = [step("a", [block("a1"), block("a2", "number")]), step("b"), step("c", [block("c1")])]
CASES = [
    ("add-step appends a node and its chain edge", [step("s", title="Steps")], {"kind": "add-step", "step": step("n", [block("n1", "note")], "Imported")}),
    ("add-step refuses a present step id", ABC, {"kind": "add-step", "step": step("b")}),
    ("remove-step bridges the chain over a middle step", ABC, {"kind": "remove-step", "stepId": "b"}),
    ("move-step rewires the chain only", ABC, {"kind": "move-step", "stepId": "c", "index": 0}),
    ("remove-step refuses a missing step", ABC, {"kind": "remove-step", "stepId": "x"}),
    ("add-block appends one absolute blocks set", ABC, {"kind": "add-block", "stepId": "b", "block": block("b1", "slider", "Width")}),
    ("remove-block sets the step's remaining blocks", ABC, {"kind": "remove-block", "stepId": "a", "blockId": "a1"}),
    ("remove-block refuses a block its step does not hold", ABC, {"kind": "remove-block", "stepId": "b", "blockId": "a1"}),
    ("move-block across steps sets both steps' blocks", ABC, {"kind": "move-block", "blockId": "a2", "fromStepId": "a", "toStepId": "c", "index": 0}),
]


def main():
    cases = []
    for name, base, verb in CASES:
        cases.append({"name": name, "base": base, "verb": verb, **run(base, verb)})
    text = json.dumps({"schemaVersion": 1, "producer": "🧪️s4-tools-b-playbook-child-leaves.py", "cases": cases}, indent=2, ensure_ascii=False) + "\n"
    if "--check" in sys.argv[1:]:
        sys.exit(0 if OUT.exists() and OUT.read_text() == text else 1)
    if any(arg != "--check" for arg in sys.argv[1:]):
        sys.exit(f"unknown arguments {sys.argv[1:]}")
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(text)
    print(f"{len(cases)} cases → {OUT.relative_to(ROOT)}")


main()
