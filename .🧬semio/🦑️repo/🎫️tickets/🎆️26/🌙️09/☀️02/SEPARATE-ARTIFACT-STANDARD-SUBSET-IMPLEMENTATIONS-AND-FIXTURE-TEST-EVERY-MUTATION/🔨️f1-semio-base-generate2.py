#!/usr/bin/env python3
"""🔨️ F1 — real before/after envelope fixture pairs for `s.stdio.semio@v1/base`'s remaining
`apply-<arm>` routing verbs: image, text, table, graph, object, kit — six more arms that (unlike
brep/mesh/document/cad/drawing) already carry a registered `verified-native-second-implementation`
Python oracle of their own, discovered after `🔨️f1-semio-base-generate.py` closed the first seven.
Same method: load that arm's own real committed document through its own `parse_dsl`, apply one
real, already-implemented verb through its own unmodified `apply_mutation`, wrap both sides in the
envelope's own `SemioSnapshot{schema, subset}` shape.

Run: `uv run --group test python3 🔨️f1-semio-base-generate2.py`
"""
import hashlib
import importlib.util
import json
import os
import sys
import types

REPO = os.getcwd()
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION"
ROOT = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"


def stub_harness():
    mod = types.ModuleType("semio_repo_test")

    class Adapter:
        def __init__(self, lang):
            self.lang = lang

        def oracle(self, name, fn):
            return self

    class Context:
        pass

    class Outcome:
        def __init__(self, value, raw=None):
            self.value = value
            self.raw = raw

    def digest(b):
        return hashlib.sha256(b).hexdigest()

    mod.Adapter = Adapter
    mod.Context = Context
    mod.Outcome = Outcome
    mod.digest = digest
    sys.modules["semio_repo_test"] = mod


def load_module(py_path, name):
    spec = importlib.util.spec_from_file_location(name, py_path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def sha256_of_bytes(b):
    return f"sha256:{hashlib.sha256(b).hexdigest()}", len(b)


def write_json_pair(fixture_dir, before, after):
    os.makedirs(fixture_dir, exist_ok=True)
    before_bytes = (json.dumps(before, indent=2, ensure_ascii=False) + "\n").encode("utf-8")
    after_bytes = (json.dumps(after, indent=2, ensure_ascii=False) + "\n").encode("utf-8")
    with open(os.path.join(fixture_dir, "before.json"), "wb") as f:
        f.write(before_bytes)
    with open(os.path.join(fixture_dir, "after.json"), "wb") as f:
        f.write(after_bytes)
    return before_bytes, after_bytes


def camel(arm):
    return arm[0].upper() + arm[1:]


CONFIG = {
    "image": {
        "py": f"{ROOT}/✳️image/🧪️tests/mutate-semio-image/🐍️.py",
        "base_doc": f"{ROOT}/✳️image/🧪️tests/mutate-semio-image/🧫️fixtures/🗣️.dsl.semio",
        "oracle": "semio-image-python-pillow-independent",
        "mutation": lambda doc: {"mutation": "setDimensions", "width": doc["width"] + 1, "height": doc["height"] + 1},
    },
    "text": {
        "py": f"{ROOT}/✳️text/🧪️tests/mutate-semio-text/🐍️.py",
        "base_doc": f"{ROOT}/✳️text/🧪️tests/mutate-semio-text/🧫️fixtures/🧪️zukunft-bau-entwerfen-mit-bestand/🗣️.dsl.semio",
        "oracle": "semio-text-python-independent",
        "mutation": lambda doc: {"EditRun": {"index": 0, "new_content": "F1 mutated run content"}},
    },
    "table": {
        "py": f"{ROOT}/✳️table/🧪️tests/mutate-semio-table/🐍️.py",
        "base_doc": f"{ROOT}/✳️table/🧪️tests/mutate-semio-table/🧫️fixtures/📊️reuse-marketplaces.dsl.semio",
        "oracle": "semio-table-python-independent",
        "mutation": lambda doc: {"EditCell": {"row_index": 0, "column_name": doc["columns"][0]["name"], "new_value": {"kind": "str", "value": "F1-mutated"}}},
    },
    "graph": {
        "py": f"{ROOT}/✳️graph/🧪️tests/mutate-semio-graph/🐍️.py",
        "base_doc": f"{ROOT}/✳️graph/🧪️tests/mutate-semio-graph/🧫️fixtures/🧪️nakagin-capsule-tower/🗣️.dsl.semio",
        "oracle": "semio-graph-python-independent",
        "mutation": lambda doc: {"ChangeNodeLabel": {"id": doc["nodes"][0]["id"], "new_label": "F1 mutated label"}},
    },
    "object": {
        "py": f"{ROOT}/✳️object/🧪️tests/mutate-semio-object/🐍️.py",
        "base_doc": f"{ROOT}/✳️object/📚️examples/📦️crate/🖼️assets/🗣️.dsl.semio",
        "oracle": "semio-object-python-independent",
        "mutation": lambda doc: {"MoveObject": {"translation": {"x": doc["transform"]["translation"]["x"] + 1000.0, "y": doc["transform"]["translation"]["y"] + 1000.0, "z": doc["transform"]["translation"]["z"] + 1000.0}}},
    },
    "kit": {
        "py": f"{ROOT}/✳️kit/🧪️tests/mutate-semio-kit/🐍️.py",
        "base_doc": f"{ROOT}/✳️kit/🧪️tests/mutate-semio-kit/🧫️fixtures/🧪️nakagin-capsule-tower/🗣️.dsl.semio",
        "oracle": "semio-kit-python-independent",
        "mutation": lambda doc: {"RenameType": {"id": doc["types"][0]["id"], "new_name": "F1-mutated-type-name"}},
    },
}


def main():
    stub_harness()
    base_root = os.path.join(REPO, ROOT, "✳️base")
    fragments = []

    for arm, cfg in CONFIG.items():
        mod = load_module(os.path.join(REPO, cfg["py"]), f"semio_base_gen2_{arm}")
        with open(os.path.join(REPO, cfg["base_doc"]), encoding="utf-8") as f:
            text = f.read()
        arm_before = mod.parse_dsl(text)
        inner_mutation = cfg["mutation"](arm_before)
        arm_after = mod.apply_mutation(arm_before, inner_mutation)

        if json.dumps(arm_before, sort_keys=True) == json.dumps(arm_after, sort_keys=True):
            raise AssertionError(f"{arm}: mutation was a no-op — before == after")

        wrapped_before_subset = dict(arm_before)
        wrapped_before_subset["subset"] = arm
        wrapped_after_subset = dict(arm_after)
        wrapped_after_subset["subset"] = arm

        envelope_before = {"schema": "stdio.semio", "subset": wrapped_before_subset}
        envelope_after = {"schema": "stdio.semio", "subset": wrapped_after_subset}
        envelope_mutation = {"mutation": f"apply{camel(arm)}", "payload": {"mutation": inner_mutation}}

        fixture_id = f"apply-{arm}-applied"
        fixture_dir = os.path.join(base_root, "🧫️fixtures", fixture_id)
        before_bytes, after_bytes = write_json_pair(fixture_dir, envelope_before, envelope_after)
        before_sha, before_len = sha256_of_bytes(before_bytes)
        after_sha, after_len = sha256_of_bytes(after_bytes)

        fragments.append(
            {
                "id": fixture_id,
                "mutation": arm,
                "oracle": cfg["oracle"],
                "envelope_mutation": envelope_mutation,
                "files": [
                    {"role": "expected-before-json", "path": f"../🧫️fixtures/{fixture_id}/before.json", "mediaType": "application/json", "sha256": before_sha, "bytes": before_len},
                    {"role": "expected-after-json", "path": f"../🧫️fixtures/{fixture_id}/after.json", "mediaType": "application/json", "sha256": after_sha, "bytes": after_len},
                ],
            }
        )
        print(f"apply-{arm}-applied: before={before_len}B after={after_len}B")

    out = os.path.join(REPO, TICKET, "🗑️generated", "f1-semio-base-fragments2.json")
    with open(out, "w") as f:
        json.dump(fragments, f, indent=2, ensure_ascii=False)
    print("wrote", out)


if __name__ == "__main__":
    main()
