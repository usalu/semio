#!/usr/bin/env python3
"""🔍️ F1 — loads each semio subset's own already-complete Python second implementation and dumps
its real base document as JSON, so mutation params for the missing kinds can be built from the
ACTUAL committed shape rather than guessed."""
import importlib.util
import json
import sys
import types

REPO = "/Users/ueli/Documents/semio"


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
        import hashlib

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


def main():
    stub_harness()
    subset, py_path, base_path = sys.argv[1], sys.argv[2], sys.argv[3]
    mod = load_module(f"{REPO}/{py_path}", f"semio_{subset}")
    with open(f"{REPO}/{base_path}", encoding="utf-8") as f:
        text = f.read()
    doc = mod.parse_dsl(text)
    print(json.dumps(doc, indent=2, ensure_ascii=False))


if __name__ == "__main__":
    main()
