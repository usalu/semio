#!/usr/bin/env python3
"""🔁 W10 — replays every fem2d mutation vector through the plugin's OWN independent Python model.

The subject is the committed evidence on disk; the reference is
`◻️2d/…/🌐️any/🧪️tests/*/🐍️.py`, the from-scratch Python re-implementation of the `s.fem.fem2d`
document and its typed verbs that the plugin already ships as its differential oracle ("no Rust was
read to write this"). This driver imports those files verbatim and drives their `apply_mutation`,
`inverse_mutation`, `validate`, `touches_one` and `restores` over EVERY committed case directory —
the 25 pre-existing ones and the 50 this ticket adds — instead of only the one row per kind that
each subset's `🥒️.feature` names.

Third-party leg: every committed snapshot is additionally validated against the artifact's own
`🌐️any/🧬️schema/📸️snapshot/🔣️.json` with `jsonschema` (PyPI, MIT), a real external
JSON-Schema implementation, so the fixtures are checked by something outside this repository.

Usage:
    uv run python 🔨️w10-replay-fem2d.py            # every fem2d case
    uv run python 🔨️w10-replay-fem2d.py --verbose  # one line per case
"""

# region 🔖️Imports
import importlib.util
import json
import os
import sys
import types

# endregion 🔖️Imports


# region 🔖️Harness
REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 7)))
ARTIFACT = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "◻️2d", "🏅️standards", "🔖️1", "🪆️subsets")
SNAPSHOT_SCHEMA = os.path.join(ARTIFACT, "🌐️any", "🧬️schema", "📸️snapshot", "🔣️.json")

REFERENCES = {
    "🏋️load": "🏋️mutate-fem2d-1-any-load",
    "🕸️mesh": "🕸️mutate-fem2d-1-any-mesh",
    "🧱️material": "🧱️mutate-fem2d-1-any-material",
    "🛡️boundary": "🛡️mutate-fem2d-1-any-boundary",
    "📈️analysis": "📈️mutate-fem2d-1-any-analysis",
}
"""🧭️ Which committed independent-Python reference owns each subset's verbs."""


def stub_test_host():
    """🧱️ A minimal stand-in for `semio_repo_test`, so a reference file can be imported outside the
    repo test coordinator. Only its import and its `Adapter(...).oracle(...)` chaining are exercised
    here — the model and the verbs, which is all this driver reads, import nothing."""
    if "semio_repo_test" in sys.modules:
        return
    try:
        import semio_repo_test  # noqa: F401

        return
    except ImportError:
        pass
    module = types.ModuleType("semio_repo_test")

    class Adapter:
        def __init__(self, name):
            self.name = name

        def oracle(self, *_args, **_kwargs):
            return self

        def subject(self, *_args, **_kwargs):
            return self

    class Context:  # noqa: D401
        pass

    class Outcome:
        def __init__(self, payload, raw=None):
            self.payload, self.raw = payload, raw

    module.Adapter, module.Context, module.Outcome = Adapter, Context, Outcome
    sys.modules["semio_repo_test"] = module


def reference(subset):
    """📚️ Imports one subset's committed independent Python reference implementation."""
    stub_test_host()
    path = os.path.join(ARTIFACT, "🌐️any", "🧪️tests", REFERENCES[subset], "🐍️.py")
    spec = importlib.util.spec_from_file_location("fem2d_reference_%s" % REFERENCES[subset], path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# endregion 🔖️Harness


# region 🔖️Cases
def read(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def cases():
    """🗂️ Every committed fem2d vector: (subset, kind directory, case directory, absolute path)."""
    for subset in sorted(REFERENCES):
        mutations = os.path.join(ARTIFACT, subset, "🧬️schema", "🧬️mutations")
        for kind in sorted(os.listdir(mutations)):
            tests = os.path.join(mutations, kind, "🧪️tests")
            if not os.path.isdir(tests):
                continue
            for case in sorted(os.listdir(tests)):
                yield subset, kind, case, os.path.join(tests, case)


def bundle(root):
    """📦️ The committed quartet plus whichever diff alternative the case carries."""
    payload = {
        "before": read(os.path.join(root, "📸️snapshot", "⬅️before", "🔣️.json")),
        "after": read(os.path.join(root, "📸️snapshot", "➡️after", "🔣️.json")),
        "mutation": read(os.path.join(root, "🦠️mutation", "🔣️.json")),
        "outcome": read(os.path.join(root, "🎯️outcome", "🔣️.json")),
    }
    json_diff = os.path.join(root, "🔺️diff", "🔣️.json")
    absent = os.path.join(root, "🔺️diff", "🚫️.absent")
    payload["diff"] = read(json_diff) if os.path.exists(json_diff) else None
    payload["absent"] = os.path.exists(absent)
    if os.path.exists(json_diff) == os.path.exists(absent):
        raise AssertionError("a vector carries exactly one diff alternative")
    return payload


BUNDLE_NODES = sorted(
    [
        "🦠️mutation/",
        "📸️snapshot/",
        "📸️snapshot/⬅️before/",
        "📸️snapshot/➡️after/",
        "🔺️diff/",
        "🎯️outcome/",
        "🦀️.rs",
        "🦠️mutation/🔣️.json",
        "📸️snapshot/⬅️before/🔣️.json",
        "📸️snapshot/➡️after/🔣️.json",
        "🎯️outcome/🔣️.json",
    ]
)
"""🧾️ The closed physical shape `mutationVectorRegistryBreaches` demands, minus the diff leaf."""


def bundle_nodes(root):
    nodes = []
    for directory, _, files in os.walk(root):
        rel = os.path.relpath(directory, root).replace(os.sep, "/")
        if rel != ".":
            nodes.append(rel + "/")
        for name in files:
            nodes.append(("" if rel == "." else rel + "/") + name)
    return sorted(nodes)


# endregion 🔖️Cases


# region 🔖️Replay
def replay(module, kind_id, data):
    """▶️ Drives the reference over one vector and reports what it did, in the reference's words."""
    before = module.document_of(data["before"])
    after = module.document_of(data["after"])
    declared = data["outcome"]["status"]
    try:
        applied = module.apply_mutation(before, data["mutation"])
    except AssertionError as refusal:
        return ("refused", str(refusal))
    if applied != after:
        try:
            module.equals_committed(kind_id, applied, after)
        except AssertionError as mismatch:
            return ("diverged", str(mismatch))
    if declared == "applied" and not data["outcome"].get("messages"):
        module.observable("spec-vector-%s" % kind_id, before, applied)
        module.touches_one("spec-vector-%s" % kind_id, kind_id, before, applied)
        module.restores(kind_id, module.apply_mutation(applied, module.inverse_mutation(before, data["mutation"])), before)
        return ("applied", "after reproduced, one member written, inverse restores")
    if applied != before:
        raise AssertionError("a no-op/refused vector must not move the document")
    return ("unchanged", "document reproduced untouched")


EXPECTED = {"applied": {"applied"}, "rejected": {"refused"}}
"""🎯️ What the declared outcome means for the reference — a rejection it also implements refuses."""


def main():
    verbose = "--verbose" in sys.argv[1:]
    try:
        import jsonschema

        validator = jsonschema.Draft202012Validator(read(SNAPSHOT_SCHEMA))
    except ImportError:
        validator = None
        print("!! jsonschema is unavailable — the third-party schema leg is SKIPPED")
    modules = {subset: reference(subset) for subset in REFERENCES}
    totals = {"applied": 0, "refused": 0, "unchanged": 0, "diverged": 0}
    divergences, schema_failures, shape_failures = [], [], []
    for subset, kind, case, root in cases():
        kind_id = kind[2:] if not kind[0].isalnum() else kind
        while not (kind_id[0].isalnum()):
            kind_id = kind_id[1:]
        data = bundle(root)
        expected_nodes = sorted(BUNDLE_NODES + ["🔺️diff/🚫️.absent" if data["absent"] else "🔺️diff/🔣️.json"])
        if bundle_nodes(root) != expected_nodes:
            shape_failures.append((root, sorted(set(bundle_nodes(root)) ^ set(expected_nodes))))
        if validator is not None:
            for label in ("before", "after"):
                errors = sorted(validator.iter_errors(data[label]), key=str)
                if errors:
                    schema_failures.append((root, label, errors[0].message))
        module = modules[subset]
        outcome, detail = replay(module, kind_id, data)
        totals[outcome] += 1
        declared = data["outcome"]["status"]
        wanted = EXPECTED[declared]
        if declared == "applied" and data["outcome"].get("messages"):
            wanted = {"unchanged"}
        if outcome not in wanted:
            divergences.append((os.path.relpath(root, REPO), declared, outcome, detail))
        if verbose:
            print("%-9s %-32s %-30s %s" % (outcome, kind, case, detail[:70]))
    print("\ncases replayed: %d  (applied %d, refused %d, unchanged %d, diverged %d)" % (sum(totals.values()), totals["applied"], totals["refused"], totals["unchanged"], totals["diverged"]))
    print("bundle-shape failures: %d" % len(shape_failures))
    for root, delta in shape_failures:
        print("   %s  %s" % (os.path.relpath(root, REPO), delta))
    print("jsonschema (third-party) snapshot failures: %d" % len(schema_failures))
    for root, label, message in schema_failures:
        print("   %s [%s] %s" % (os.path.relpath(root, REPO), label, message))
    print("reference divergences (declared vs. what the independent Python model did): %d" % len(divergences))
    for path, declared, outcome, detail in divergences:
        print("   %s\n      declared %s, reference %s — %s" % (path, declared, outcome, detail))
    return 1 if (shape_failures or schema_failures) else 0


if __name__ == "__main__":
    sys.exit(main())
# endregion 🔖️Replay
