#!/usr/bin/env python3
"""🐍️ Replay every committed fem3d mutation vector through the plugin's OWN hardened Python
reference, and check that the differential is actually wired end to end.

Successor to `🔨️w11-replay-fem3d.py`, which measured 75 vectors with 5 DIVERGE and 12 schema
failures. W12 hardened the Rust, repaired the schemas and taught the reference the same refusal
rules, so this driver measures the same things again and must report zero of both.

The reference is not re-implemented here. Each subset's independent second implementation lives in
`🗿️artifacts/🧊️3d/…/🪆️subsets/🌐️any/🧪️tests/<case>/🐍️.py`; this driver loads those modules with a
stub `semio_repo_test` (the framework test host is only importable inside a planned run) and calls
their own `validate` / `apply_mutation` / `inverse_mutation` plus their own laws `observable`,
`touches_one`, `restores` and `equals_committed`.

Four verdicts per vector, each read off the committed `🎯️outcome` first and then demanded of the
reference:

* `agree`         — an applied vector: the reference lands on `➡️after` and all four laws hold;
* `agree-noop`    — a declared no-op: the reference leaves the model exactly where it was;
* `agree-refused` — a declared rejection: the reference refuses it;
* `DIVERGE`       — anything else.

Then: every snapshot against `🌐️any/🧬️schema/📸️snapshot/🔣️.json`, every payload against its kind's
`🧬️.schema.json` and every tagged envelope against `🌐️any/🧬️schema/🧬️mutations/🔣️.json` with the
third-party `jsonschema`; the on-disk bundle structure against the five oracle catalogs and the repo
taxonomy; and every `🥒️.feature` scenario id against BOTH adapters' registrations.

Usage: uv run python 🔨️w12-replay-fem3d.py [--only <substring>]
"""

import json
import os
import re
import sys
import types

REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *([".."] * 7)))
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets")
ANY_TESTS = os.path.join(SUBSETS, "🌐️any", "🧪️tests")
SNAPSHOT_SCHEMA = os.path.join(SUBSETS, "🌐️any", "🧬️schema", "📸️snapshot", "🔣️.json")
FACET_SCHEMA = os.path.join(SUBSETS, "🌐️any", "🧬️schema", "🧬️mutations", "🔣️.json")

REFERENCE_CASE = {
    "🕸️mesh": "🕸️mutate-fem3d-1-any-mesh",
    "🧱️material": "🧱️mutate-fem3d-1-any-material",
    "🛡️boundary": "🛡️mutate-fem3d-1-any-boundary",
    "🏋️load": "🏋️mutate-fem3d-1-any-load",
    "📈️analysis": "📈️mutate-fem3d-1-any-analysis",
}
SUBJECT_CASE = {
    "🕸️mesh": "🕸️mutate-fem3d-1-mesh",
    "🧱️material": "🧱️mutate-fem3d-1-material",
    "🛡️boundary": "🛡️mutate-fem3d-1-boundary",
    "🏋️load": "🏋️mutate-fem3d-1-load",
    "📈️analysis": "📈️mutate-fem3d-1-analysis",
}


# region 🔖️Reference loading
REGISTERED = {}


class Recorder:
    """📝️ A stub `Adapter` that records which scenario ids the reference registers."""

    subset = None

    def __init__(self, name):
        self.name = name

    def oracle(self, identifier, handler):
        REGISTERED.setdefault(Recorder.subset, set()).add(identifier)
        return self


def stub_host():
    module = types.ModuleType("semio_repo_test")
    module.Adapter = Recorder
    module.Context = type("Context", (), {})
    module.Outcome = type("Outcome", (), {"__init__": lambda self, payload, raw=None: None})
    return module


def load_reference(subset):
    sys.modules.setdefault("semio_repo_test", stub_host())
    path = os.path.join(ANY_TESTS, REFERENCE_CASE[subset], "🐍️.py")
    namespace = types.ModuleType("reference_%s" % REFERENCE_CASE[subset])
    namespace.__file__ = path
    with open(path, encoding="utf-8") as handle:
        exec(compile(handle.read(), path, "exec"), namespace.__dict__)
    Recorder.subset = subset
    namespace.adapter()
    return namespace


# endregion 🔖️Reference loading


# region 🔖️Discovery
def vectors():
    for subset in sorted(REFERENCE_CASE):
        mutations = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations")
        for kind_directory in sorted(os.listdir(mutations)):
            tests = os.path.join(mutations, kind_directory, "🧪️tests")
            if not os.path.isdir(tests):
                continue
            for case in sorted(os.listdir(tests)):
                yield subset, kind_directory, case, os.path.join(tests, case)


def read(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


# endregion 🔖️Discovery


# region 🔖️Schema
def schema_check(subset, kind_directory, case_directory, root, problems):
    import jsonschema

    snapshot_schema = read(SNAPSHOT_SCHEMA)
    facet_schema = read(FACET_SCHEMA)
    for side in ("⬅️before", "➡️after"):
        document = read(os.path.join(root, "📸️snapshot", side, "🔣️.json"))
        try:
            jsonschema.validate(document, snapshot_schema)
        except jsonschema.ValidationError as error:
            problems.append("%s/%s %s snapshot: %s" % (kind_directory, case_directory, side, error.message))
    envelope = read(os.path.join(root, "🦠️mutation", "🔣️.json"))
    payload_schema_path = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations", kind_directory, "🧬️.schema.json")
    payload = {key: value for key, value in envelope.items() if key != "mutation"}
    try:
        jsonschema.validate(payload, read(payload_schema_path))
    except jsonschema.ValidationError as error:
        problems.append("%s/%s mutation payload: %s" % (kind_directory, case_directory, error.message))
    try:
        jsonschema.validate(envelope, facet_schema)
    except jsonschema.ValidationError as error:
        problems.append("%s/%s tagged envelope: %s" % (kind_directory, case_directory, error.message))


# endregion 🔖️Schema


# region 🔖️Replay
def replay(reference, root, kind):
    before = reference.document_of(read(os.path.join(root, "📸️snapshot", "⬅️before", "🔣️.json")))
    after = reference.document_of(read(os.path.join(root, "📸️snapshot", "➡️after", "🔣️.json")))
    mutation = read(os.path.join(root, "🦠️mutation", "🔣️.json"))
    outcome = read(os.path.join(root, "🎯️outcome", "🔣️.json"))
    rejected = outcome.get("status") == "rejected"
    no_op = any(message.get("code") == "mutation.no-op" for message in outcome.get("messages", []))
    try:
        applied = reference.apply_mutation(before, mutation)
    except AssertionError as error:
        if rejected:
            return "agree-refused", "the reference refuses it too: %s" % str(error)[:100]
        return "DIVERGE", "the reference refused a vector committed as %s: %s" % ("a no-op" if no_op else "applied", str(error)[:140])
    if rejected:
        return "DIVERGE", "the reference APPLIED a vector committed as rejected"
    if applied != after:
        return "DIVERGE", "the reference landed somewhere other than the committed after-snapshot"
    if no_op:
        reference.observable("reject-%s" % kind, before, applied, moves=False)
        return "agree-noop", "unchanged, as the committed no-op claims"
    reference.observable("spec-vector-%s" % kind, before, applied)
    reference.touches_one("spec-vector-%s" % kind, kind, before, applied)
    reference.equals_committed(kind, applied, after)
    reference.restores(kind, reference.apply_mutation(applied, reference.inverse_mutation(before, mutation)), before)
    return "agree", "apply, touches-one, equals-committed and inverse all hold"


# endregion 🔖️Replay


# region 🔖️Structure
BUNDLE = sorted(["🦠️mutation/", "📸️snapshot/", "📸️snapshot/⬅️before/", "📸️snapshot/➡️after/", "🔺️diff/", "🎯️outcome/", "🦀️.rs", "🦠️mutation/🔣️.json", "📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json", "🎯️outcome/🔣️.json"])


def bundle_nodes(root):
    found = []
    for base, directories, files in os.walk(root):
        prefix = os.path.relpath(base, root).replace(os.sep, "/")
        prefix = "" if prefix == "." else prefix + "/"
        found.extend(prefix + name + "/" for name in directories)
        found.extend(prefix + name for name in files)
    return sorted(found)


def leading_emoji(name):
    emoji = ""
    for character in name:
        if character.isalnum() or character in "-_.":
            break
        emoji += character
    return emoji.replace("️", "")


def structure_problems():
    """🧾️ Catalog ↔ disk set-equality, scenario identity binding, the closed source bundle, the
    sibling-emoji namespace, and taxonomy registration — everything the platform checks in TS."""
    problems, advisories = [], []
    taxonomy = json.load(open(os.path.join(REPO, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🔣️taxonomy.json"), encoding="utf-8"))
    registered = set(taxonomy["semanticDirectoryMemberKinds"]["members-of-tests"]["memberNames"])
    for subset in sorted(REFERENCE_CASE):
        catalog = json.load(open(os.path.join(SUBSETS, subset, "🔮️oracle", "🔣️.json"), encoding="utf-8"))
        for vector in catalog["mutationCatalogs"][0]["vectors"]:
            tests = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations", vector["sourceMutationDirectoryName"], "🧪️tests")
            on_disk = sorted(entry for entry in os.listdir(tests) if os.path.isdir(os.path.join(tests, entry)))
            declared = sorted(scenario["directoryName"] for scenario in vector["scenarios"])
            if on_disk != declared:
                problems.append("%s/%s catalog %r != disk %r" % (subset, vector["mutationId"], declared, on_disk))
            emojis = [leading_emoji(name) for name in on_disk]
            if len(set(emojis)) != len(emojis):
                problems.append("%s/%s sibling case directories share a leading emoji: %r" % (subset, vector["mutationId"], emojis))
            for scenario in vector["scenarios"]:
                name = scenario["directoryName"]
                if name[len(leading_emoji(name)) :].lstrip("️") != scenario["id"]:
                    problems.append("%s: id %r is not the stem of %r" % (vector["mutationId"], scenario["id"], name))
                if name not in registered:
                    problems.append("%s: %s is not registered in taxonomy members-of-tests" % (vector["mutationId"], name))
                if len(name) > 28:
                    advisories.append("%s: %s is %d characters, over the 28-character Windows-path budget this ticket adopted" % (vector["mutationId"], name, len(name)))
                root = os.path.join(tests, name)
                expected = sorted(BUNDLE + ["🔺️diff/🚫️.absent" if os.path.exists(os.path.join(root, "🔺️diff", "🚫️.absent")) else "🔺️diff/🔣️.json"])
                if bundle_nodes(root) != expected:
                    problems.append("%s/%s is not the closed 13-node source bundle: %r" % (vector["mutationId"], name, bundle_nodes(root)))
    return problems, advisories


# endregion 🔖️Structure


# region 🔖️Wiring
def feature_scenarios(path):
    """🥒️ Every expanded scenario id a feature file declares — `@id-<base>` plus each row's `id`."""
    found = []
    base = None
    columns = None
    for line in open(path, encoding="utf-8"):
        stripped = line.strip()
        if stripped.startswith("@id-"):
            base = stripped[len("@id-") :]
            columns = None
        elif stripped.startswith("|") and base is not None:
            cells = [cell.strip() for cell in stripped.strip("|").split("|")]
            if columns is None:
                columns = cells
            else:
                found.append("%s-%s" % (base, cells[columns.index("id")]))
        elif stripped.startswith("Scenario:") and base is not None:
            found.append(base)
    return found


def wiring_problems():
    """🔌️ Every planned scenario needs BOTH a subject and an oracle registration
    (`validateRegistration` in the test platform's `🟦️.ts`), so a feature row without one of them is
    a hard failure at plan time, not a skipped row."""
    problems = []
    for subset in sorted(REFERENCE_CASE):
        feature = os.path.join(SUBSETS, subset, "🧪️tests", SUBJECT_CASE[subset], "🥒️.feature")
        subject = open(os.path.join(SUBSETS, subset, "🧪️tests", SUBJECT_CASE[subset], "🦀️.rs"), encoding="utf-8").read()
        planned = feature_scenarios(feature)
        if len(planned) != len(set(planned)):
            problems.append("%s: the feature declares a duplicate scenario id" % subset)
        oracle = REGISTERED.get(subset, set())
        for identifier in planned:
            if identifier not in oracle:
                problems.append("%s: no python ORACLE registration for scenario %r" % (subset, identifier))
            stem = identifier.split("-", 1)
            if identifier.startswith("reject-"):
                if '"%s"' % identifier[len("reject-") :] not in subject:
                    problems.append("%s: no rust SUBJECT registration for scenario %r" % (subset, identifier))
            elif identifier.startswith("hall-vector-"):
                if "hall-vector-{kind}" not in subject:
                    problems.append("%s: no rust SUBJECT registration for the hall-vector family" % subset)
    return problems


# endregion 🔖️Wiring


def main():
    only = sys.argv[sys.argv.index("--only") + 1] if "--only" in sys.argv else ""
    references = {subset: load_reference(subset) for subset in REFERENCE_CASE}
    problems = []
    tally = {}
    rows = []
    for subset, kind_directory, case_directory, root in vectors():
        if only and only not in case_directory:
            continue
        tag = read(os.path.join(root, "🦠️mutation", "🔣️.json"))["mutation"]
        reference = references[subset]
        kind = next(name for name, value in reference.TAGS.items() if value == tag)
        schema_check(subset, kind_directory, case_directory, root, problems)
        try:
            verdict, note = replay(reference, root, kind)
        except AssertionError as error:
            verdict, note = "DIVERGE", str(error)[:200]
        tally[verdict] = tally.get(verdict, 0) + 1
        rows.append((kind, case_directory, verdict, note))
    width = max(len(row[0]) for row in rows) if rows else 0
    for kind, case_directory, verdict, note in rows:
        print("%-*s  %-28s  %-14s  %s" % (width, kind, case_directory, verdict, note))
    print("")
    print("vectors: %d" % len(rows))
    for verdict in sorted(tally):
        print("  %-14s %d" % (verdict, tally[verdict]))
    print("schema problems: %d" % len(problems))
    for problem in problems:
        print("  " + problem)
    structural, advisories = structure_problems()
    print("structure problems: %d" % len(structural))
    for problem in structural:
        print("  " + problem)
    wiring = wiring_problems()
    print("wiring problems: %d" % len(wiring))
    for problem in wiring:
        print("  " + problem)
    print("structure advisories (pre-existing names, not authored here): %d" % len(advisories))
    for advisory in advisories:
        print("  " + advisory)
    return 1 if structural or wiring or problems or tally.get("DIVERGE") else 0


if __name__ == "__main__":
    sys.exit(main())
