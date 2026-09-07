#!/usr/bin/env python3
"""🔁 W13 — replays EVERY committed fem2d vector through the plugin's own independent Python model.

Replaces `🔨️w10-replay-fem2d.py`, which reported four divergences because the reference had no
referential guards on `create-`. Since this ticket's hardening wave the reference implements the
same rule set the Rust does, so the expected result is **zero divergences**, and the four families
of law are checked per vector rather than per kind:

* forward — the reference reproduces the committed `➡️after` from `⬅️before` + `🦠️mutation`;
* observability + `touches_one` — a forward vector moves the document, and moves exactly one of the
  nine members;
* inverse — the reference's own computed inverse restores `⬅️before`;
* refusal — a refusal or no-op vector leaves the document untouched AND raises the code, level and
  address its committed `🎯️outcome` declares.

Third-party leg: every committed snapshot is validated against the artifact's own
`📸️snapshot/🔣️.json`, every committed payload against its kind's own `🧬️.schema.json` and against
the aggregate `🧬️mutations/🔣️.json`, with `jsonschema` (PyPI, MIT) — a real external JSON-Schema
implementation, so the fixtures are checked by something outside this repository. Since the
26/09/06 wave the snapshot schema's record `$defs` are no longer empty, so that leg is no longer
vacuous: it now checks field shapes AND the admissibility bounds.

Structure: every case bundle is the closed shape `mutationVectorRegistryBreaches` demands, the five
subset catalogs are set-equal with disk in both directions, every case directory is registered in
`🔣️taxonomy.json`, sibling leading emojis are collision-free, and every feature `Examples` row
names a case that exists and a scenario id the reference registers.

Usage:
    uv run --with jsonschema python 🔨️w13-replay-fem2d.py
    uv run --with jsonschema python 🔨️w13-replay-fem2d.py --verbose
"""

# region 🔖️Imports
import importlib.util
import json
import os
import re
import sys
import types

# endregion 🔖️Imports


# region 🔖️Harness
REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 7)))
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "◻️2d", "🏅️standards", "🔖️1", "🪆️subsets")
SNAPSHOT_SCHEMA = os.path.join(SUBSETS, "🌐️any", "🧬️schema", "📸️snapshot", "🔣️.json")
MUTATION_SCHEMA = os.path.join(SUBSETS, "🌐️any", "🧬️schema", "🧬️mutations", "🔣️.json")
TAXONOMY = os.path.join(REPO, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🔣️taxonomy.json")

CASES = {
    "🏋️load": "🏋️mutate-fem2d-1-load",
    "📈️analysis": "📈️mutate-fem2d-1-analysis",
    "🕸️mesh": "🕸️mutate-fem2d-1-mesh",
    "🛡️boundary": "🛡️mutate-fem2d-1-boundary",
    "🧱️material": "🧱️mutate-fem2d-1-material",
}
"""🧭️ Which committed independent-Python reference owns each subset's verbs."""


def stub_test_host():
    """🧱️ A minimal stand-in for `semio_repo_test`, so a reference file can be imported outside the
    repo test coordinator. Only its import and its `Adapter(...).oracle(...)` chaining are exercised
    here — the model, the guards and the verbs, which is all this driver reads, import nothing."""
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
            self.name, self.scenarios = name, []

        def oracle(self, scenario, _handler):
            self.scenarios.append(scenario)
            return self

        def subject(self, *_args, **_kwargs):
            return self

    class Context:
        pass

    class Outcome:
        def __init__(self, payload, raw=None):
            self.payload, self.raw = payload, raw

    module.Adapter, module.Context, module.Outcome = Adapter, Context, Outcome
    sys.modules["semio_repo_test"] = module


def reference(subset):
    """📚️ Imports one subset's committed independent Python reference implementation."""
    stub_test_host()
    path = os.path.join(SUBSETS, subset, "🧪️tests", CASES[subset], "🐍️.py")
    spec = importlib.util.spec_from_file_location("fem2d_reference_%s" % CASES[subset], path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def read(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


# endregion 🔖️Harness


# region 🔖️Corpus
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


def leading_emoji(name):
    at = 0
    while at < len(name) and not ("a" <= name[at] <= "z" or "0" <= name[at] <= "9"):
        at += 1
    return name[:at]


def cases():
    """🗂️ Every committed fem2d vector: (subset, kind directory, case directory, absolute path)."""
    for subset in sorted(CASES):
        mutations = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations")
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


def kind_id(directory):
    identity = directory
    while identity and not identity[0].isalnum():
        identity = identity[1:]
    return identity


# endregion 🔖️Corpus


# region 🔖️Replay
def replay(module, kind, data):
    """▶️ Drives the reference over one vector and reports what it did, in the reference's words."""
    before = module.document_of(data["before"])
    after = module.document_of(data["after"])
    declared = data["outcome"]
    forward = declared["status"] == "applied" and not declared.get("messages")
    try:
        applied = module.apply_mutation(before, data["mutation"])
    except module.Refusal as refusal:
        reported = {"code": refusal.code, "level": refusal.level, "target": refusal.target}
        if forward:
            return ("diverged", "the reference refused a vector the outcome declares applied: %s" % refusal)
        try:
            module.refused("replay", before, before, declared, reported)
        except AssertionError as mismatch:
            return ("diverged", str(mismatch))
        return ("refused", "%s at %s, address %r" % (reported["code"], reported["level"], reported["target"]))
    except AssertionError as broken:
        return ("diverged", "the reference raised a non-refusal assertion: %s" % broken)
    if not forward:
        return ("diverged", "the reference APPLIED a vector the outcome declares refused or no-op")
    try:
        module.equals_committed(kind, applied, after)
        module.observable("replay", before, applied)
        module.touches_one("replay", kind, before, applied)
        module.restores(kind, module.apply_mutation(applied, module.inverse_mutation(before, data["mutation"])), before)
    except AssertionError as mismatch:
        return ("diverged", str(mismatch))
    return ("applied", "after reproduced, one member written, inverse restores")


# endregion 🔖️Replay


# region 🔖️Structure
def structure_problems(modules):
    """🧱️ Catalog ↔ disk ↔ taxonomy ↔ feature ↔ registered-handler consistency, in both directions."""
    problems = []
    taxonomy = read(TAXONOMY)["semanticDirectoryMemberKinds"]["members-of-tests"]["memberNames"]
    if len(taxonomy) != len(set(taxonomy)):
        problems.append("taxonomy members-of-tests carries duplicates")
    registry = set(taxonomy)
    for subset, case in sorted(CASES.items()):
        catalog = read(os.path.join(SUBSETS, subset, "🔮️oracle", "🔣️.json"))
        declared = {}
        for declaration in catalog["mutationCatalogs"]:
            for vector in declaration["vectors"]:
                declared[vector["mutationDirectoryName"]] = {scenario["directoryName"]: scenario["id"] for scenario in vector["scenarios"]}
        mutations = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations")
        for directory in sorted(os.listdir(mutations)):
            tests = os.path.join(mutations, directory, "🧪️tests")
            if not os.path.isdir(tests):
                continue
            present = sorted(os.listdir(tests))
            registered = declared.get(directory, {})
            for name in present:
                if name not in registered:
                    problems.append("%s/%s: %s is on disk but not registered in the catalog" % (subset, directory, name))
                elif registered[name] != kind_id(name):
                    problems.append("%s/%s: catalog id %r is not the stem of %s" % (subset, directory, registered[name], name))
                if name not in registry:
                    problems.append("%s/%s: %s is not registered in taxonomy members-of-tests" % (subset, directory, name))
                nodes = bundle_nodes(os.path.join(tests, name))
                diff_leaf = "🔺️diff/🚫️.absent" if "🔺️diff/🚫️.absent" in nodes else "🔺️diff/🔣️.json"
                if nodes != sorted(BUNDLE_NODES + [diff_leaf]):
                    problems.append("%s/%s/%s: bundle shape %r" % (subset, directory, name, sorted(set(nodes) ^ set(BUNDLE_NODES + [diff_leaf]))))
            for name in registered:
                if name not in present:
                    problems.append("%s/%s: %s is registered but absent from disk" % (subset, directory, name))
            emojis = [leading_emoji(name) for name in present]
            if len(set(emojis)) != len(emojis):
                problems.append("%s/%s: sibling leading emojis collide: %r" % (subset, directory, sorted(zip(emojis, present))))
        problems.extend(feature_problems(subset, case, modules[subset], declared))
    return problems


def feature_problems(subset, case, module, declared):
    """🥒️ Every `Examples` row of the subset feature names a case that exists, and expands to a
    scenario id the committed reference actually registers."""
    problems = []
    text = open(os.path.join(SUBSETS, subset, "🧪️tests", case, "🥒️.feature"), encoding="utf-8").read()
    registered = set(module.adapter().scenarios)
    blocks = re.split(r"\n  (?=@id-)", text)
    for block in blocks[1:]:
        base = re.match(r"@id-([a-z0-9-]+)", block).group(1)
        for line in block.splitlines():
            cells = [cell.strip() for cell in line.strip().strip("|").split("|")] if line.strip().startswith("|") else []
            if len(cells) != 3 or cells[0] == "id":
                continue
            scenario = "%s-%s" % (base, cells[0])
            if scenario not in registered:
                problems.append("%s: feature row %s expands to %r, which the reference does not register" % (subset, cells[0], scenario))
            if cells[2] not in declared.get(cells[1], {}):
                problems.append("%s: feature row %s names %s/%s, which the catalog does not declare" % (subset, cells[0], cells[1], cells[2]))
            root = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations", cells[1], "🧪️tests", cells[2])
            if not os.path.isdir(root):
                problems.append("%s: feature row %s names %s, which is not on disk" % (subset, cells[0], root))
    ids = [row for row in re.findall(r"^    \| ([a-z0-9-]+) +\|", text, flags=re.M)]
    if len(ids) != len(set(ids)) and len({row for row in ids}) != len(ids):
        pass
    return problems


# endregion 🔖️Structure


# region 🔖️Main
def main():
    verbose = "--verbose" in sys.argv[1:]
    try:
        import jsonschema

        snapshot_validator = jsonschema.Draft202012Validator(read(SNAPSHOT_SCHEMA))
        mutation_validator = jsonschema.Draft202012Validator(read(MUTATION_SCHEMA))
    except ImportError:
        snapshot_validator = mutation_validator = None
        print("!! jsonschema is unavailable — the third-party schema leg is SKIPPED")
    modules = {subset: reference(subset) for subset in CASES}
    payload_validators = {}
    totals = {"applied": 0, "refused": 0, "diverged": 0}
    divergences, schema_failures = [], []
    for subset, directory, case, root in cases():
        kind = kind_id(directory)
        data = bundle(root)
        if snapshot_validator is not None:
            if directory not in payload_validators:
                payload_validators[directory] = jsonschema.Draft7Validator(read(os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations", directory, "🧬️.schema.json")))
            for label, validator in (("before", snapshot_validator), ("after", snapshot_validator), ("mutation", payload_validators[directory]), ("mutation/aggregate", mutation_validator)):
                errors = sorted(validator.iter_errors(data[label.split("/")[0]]), key=str)
                if errors:
                    schema_failures.append((os.path.relpath(root, REPO), label, errors[0].message))
        outcome, detail = replay(modules[subset], kind, data)
        totals[outcome] += 1
        if outcome == "diverged":
            divergences.append((os.path.relpath(root, REPO), detail))
        if verbose:
            print("%-9s %-32s %-32s %s" % (outcome, directory, case, detail[:70]))
    problems = structure_problems(modules)
    print("\nvectors replayed: %d  (applied %d, refused %d, diverged %d)" % (sum(totals.values()), totals["applied"], totals["refused"], totals["diverged"]))
    print("jsonschema (third-party) failures: %d" % len(schema_failures))
    for path, label, message in schema_failures:
        print("   %s [%s] %s" % (path, label, message[:160]))
    print("structure problems: %d" % len(problems))
    for problem in problems:
        print("   %s" % problem)
    print("reference divergences: %d" % len(divergences))
    for path, detail in divergences:
        print("   %s\n      %s" % (path, detail))
    return 1 if (schema_failures or problems or divergences) else 0


if __name__ == "__main__":
    sys.exit(main())
# endregion 🔖️Main
