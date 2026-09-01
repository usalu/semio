import json

P = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️package-language-kind-handoff/🔣️.json"
d = json.load(open(P, encoding="utf-8"))

new_words = {
    "activation": "🪪️", "assembly": "🎟️", "authority": "🪪️", "base64": "🔤️",
    "binding": "🔗️", "bindings": "🔗️", "bootstrap": "🏗️", "budget": "⏱️",
    "bytes": "🔢️", "cancellation": "🚫️", "causal-add": "➕️", "clock": "⏱️",
    "commit": "🔗️", "compare": "⚖️", "composition": "🏘️", "content": "📦️",
    "cooperative": "⏱️", "copied": "📋️", "copy": "📋️", "credit": "🎟️",
    "enqueue": "📥️", "entries": "📚️", "evidence": "🧾️", "fault": "🧯️",
    "fixed": "🗃️", "fonts": "🗚️", "framing": "📄️", "graph": "🔬️",
    "hash": "#⃣", "inbound": "📨️", "inbox": "📥️", "index": "🗂️",
    "input": "📥️", "json": "🧩️", "lifetime": "🚪️", "list": "📋️",
    "local-interaction": "🏠️", "message": "💌️",
    "mutation-leaf-contract": "🧬️", "mutation-leaf-source-contract": "🧬️",
    "nodes": "🗂️", "numeric": "🔢️", "operations": "🩹️", "ordered": "🗂️",
    "ownership": "📏️", "pack": "🧩️", "page": "📄️", "pages": "📄️",
    "payload": "📦️", "pending": "📨️", "poll": "📥️", "read-lease": "📖️",
    "reader": "📖️", "release": "🧾️", "resident": "🎟️", "response": "📨️",
    "retirement": "♻️", "return": "📤️", "set": "🧺️", "slot": "📨️",
    "source": "🏠️", "string": "🔤️", "tail": "🏁️", "transaction": "🔄️",
    "tutorial": "🎬️", "update": "🩹️", "validation": "🛡️", "value": "🧾️",
    "whole": "📄️", "wire-retirement": "🧹️", "writer": "✍️",
}

new_cases = []
for word, emoji in sorted(new_words.items()):
    name = f"{emoji}{word}"
    new_cases.append({
        "id": f"vocab-gap-{word}",
        "steps": [{"name": name, "parentKindId": "packages", "kindId": word, "canonicalName": name, "violationCodes": []}],
    })

# end-to-end regression: a co-located test file directly inside one of the new
# domain-module directories now resolves to test-case instead of colliding with
# test-fixture-member (semantic-stem-ambiguous fix).
new_cases.append({
    "id": "co-located-test-case-under-domain-module",
    "steps": [
        {"name": "⏱️clock", "parentKindId": "packages", "kindId": "clock", "canonicalName": "⏱️clock", "violationCodes": []},
        {"name": "🧪️contention", "parentKindId": "previous", "kindId": "test-case", "canonicalName": "🧪️contention", "violationCodes": []},
    ],
})
# regression guard: nested fixture members under an actual resolved test-case still
# resolve to test-fixture-member, not test-case.
new_cases.append({
    "id": "fixture-member-nested-under-resolved-test-case",
    "steps": [
        {"name": "🧪️tests", "parentKindId": "packages", "kindId": "tests", "canonicalName": "🧪️tests", "violationCodes": []},
        {"name": "🧪️mycase", "parentKindId": "previous", "kindId": "test-case", "canonicalName": "🧪️mycase", "violationCodes": []},
        {"name": "🧪️member", "parentKindId": "previous", "kindId": "test-fixture-member", "canonicalName": "🧪️member", "violationCodes": []},
    ],
})

existing_ids = {c["id"] for c in d["cases"]}
for c in new_cases:
    assert c["id"] not in existing_ids, c["id"]
d["cases"].extend(new_cases)

out = json.dumps(d, indent=2, ensure_ascii=False) + "\n"
open(P, "w", encoding="utf-8").write(out)
print("added", len(new_cases), "cases; total now", len(d["cases"]))
