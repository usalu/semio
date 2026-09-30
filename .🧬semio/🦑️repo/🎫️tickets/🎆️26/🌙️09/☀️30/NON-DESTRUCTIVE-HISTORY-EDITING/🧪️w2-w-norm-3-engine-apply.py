"""🧪️ W2-W-norm-3: applies every staged vector's mutation to its before-snapshot through the shared Python norm engine
(and its own inverse), before any after-snapshot exists — the quick check for engine gaps per subset.

`python3 🧪️w2-w-norm-3-engine-apply.py <subset dir>`
"""
import importlib.util
import json
import os
import sys
import traceback

REPO = os.getcwd()
spec = importlib.util.spec_from_file_location("semio_repo_test", os.path.join(REPO, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py"))
host = importlib.util.module_from_spec(spec)
sys.modules["semio_repo_test"] = host
spec.loader.exec_module(host)
sys.path.insert(0, os.path.join(REPO, "✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution"))
engine = importlib.import_module("🐍️")

subset = sys.argv[1]
fixtures = f"{subset}/🧫️fixtures/🧬️mutations"
kinds = []
for leaf in sorted(os.listdir(fixtures)):
    descriptor = json.load(open(f"{subset}/🧬️schema/🧬️mutations/{leaf}/🔣️.json", encoding="utf-8"))
    kinds.append(descriptor["semanticKind"])
ok, failed = 0, []
for leaf in sorted(os.listdir(fixtures)):
    kind = json.load(open(f"{subset}/🧬️schema/🧬️mutations/{leaf}/🔣️.json", encoding="utf-8"))["semanticKind"]
    for scenario in os.listdir(f"{fixtures}/{leaf}"):
        stem = f"{fixtures}/{leaf}/{scenario}"
        base = json.load(open(f"{stem}/📸️snapshot/⬅️before/🔣️.json", encoding="utf-8"))
        _, arguments = engine.unwrap(json.load(open(f"{stem}/🦠️mutation/🔣️.json", encoding="utf-8")))
        try:
            mutated = engine.apply_mutation(base, kind, arguments)
            if mutated == base:
                raise AssertionError("did not move")
            after = f"{stem}/📸️snapshot/➡️after/🔣️.json"
            if os.path.exists(after) and json.load(open(after, encoding="utf-8")) != mutated:
                raise AssertionError("differs from the committed after-snapshot")
            steps = engine.inverse_mutation(kinds, base, kind, arguments)
            if not steps:
                raise AssertionError("empty inverse")
            restored = mutated
            for step, payload in steps:
                restored = engine.apply_mutation(restored, step, payload)
            if restored != base:
                raise AssertionError("inverse does not restore")
            ok += 1
        except Exception as error:
            failed.append((kind, "".join(traceback.format_exception_only(type(error), error)).strip()[:300]))
for kind, error in failed:
    print("FAIL", kind, "::", error)
print(f"{ok}/{ok + len(failed)} vectors apply and invert through the engine")
