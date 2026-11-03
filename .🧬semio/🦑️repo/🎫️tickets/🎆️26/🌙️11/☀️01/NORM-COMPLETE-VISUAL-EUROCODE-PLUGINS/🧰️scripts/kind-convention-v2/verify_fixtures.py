"""🧪 Third-party-style check of the committed mutation vectors of one artifact (v4: collections and selectors come from the `edit` metadata of the mutation taxonomy) with the independent Python norm vocabulary engine.

`python -I verify_fixtures.py <artifact> [kind-substring]` loads the engine (`🔮️oracles/🏃️execution`, no repo test host needed), applies every kind's committed
mutation to its before snapshot and compares with the committed after snapshot, then applies the engine's computed inverse and compares with the before snapshot.
Prints every disagreement and a summary; exit code 1 on any failure. The payload keys are also checked against the kind's schema `required` list and string/integer typing.
"""
import glob, importlib.util, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *

ENGINE = os.environ.get("NORM_ENGINE", NORM + "/🔮️oracles/🏃️execution")
EMOJI = {"en1990": "⚖️", "en1991": "🏋️", "en1992": "🏛️", "en1993": "🔩️", "en1994": "🧩️", "en1995": "🪵️", "en1996": "🪨️", "en1997": "🌍️", "en1998": "🫨️", "en1999": "🪶️"}


def load(alias, folder):
    spec = importlib.util.spec_from_file_location(alias, os.path.join(ENGINE, folder, "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


def kind_of(folder):
    name = os.path.basename(folder)
    return name[next(i for i, c in enumerate(name) if c.isascii() and c.isalpha()):]


def schema_problems(schema, arguments):
    problems = []
    for key in schema.get("required", []):
        if key not in arguments and key != "mutation":
            problems.append("missing %s" % key)
    for key, value in arguments.items():
        prop = schema.get("properties", {}).get(key)
        if prop is None:
            problems.append("unknown %s" % key)
        elif prop.get("type") == "string" and not isinstance(value, str):
            problems.append("%s must be a string" % key)
        elif prop.get("type") == "integer" and not (isinstance(value, int) and not isinstance(value, bool)):
            problems.append("%s must be an integer" % key)
    return problems


def main():
    short, only = sys.argv[1], (sys.argv[2] if len(sys.argv) > 2 else "")
    load("semio_norm_vocabulary_document", "📄️document")
    verbs = load("semio_norm_vocabulary_verbs", "🔧️verbs")
    verbs.declare_kinds([row for row in json.loads(rd(TAXONOMY))["rows"] if row["artifact"] == EMOJI[short] + short])
    verbs.declare_catalogue(load("semio_norm_vocabulary_catalogue", "📚️catalogue").section_catalogue(NORM))
    document = sys.modules["semio_norm_vocabulary_document"]
    base = NORM + "/🗿️artifacts/" + EMOJI[short] + short + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
    kinds = sorted({kind_of(d) for d in glob.glob(base + "/🧪️tests/🧫️fixtures/🧬️mutations/*") if os.path.isdir(d)})
    failures, checked, inverted, orphans = [], 0, 0, []
    for kind in kinds:
        if only and only not in kind:
            continue
        folder = [d for d in glob.glob(base + "/🧪️tests/🧫️fixtures/🧬️mutations/*") if kind_of(d) == kind][0]
        schema_path = [d for d in glob.glob(base + "/🧬️schema/🧬️mutations/*") if kind_of(d) == kind]
        if not schema_path:
            orphans.append(kind)
            continue
        schema = json.loads(rd(schema_path[0] + "/🧬️schema/🔣️.json")) if schema_path else {}
        for scenario in sorted(glob.glob(folder + "/*")):
            try:
                before = json.loads(rd(scenario + "/🔣️.before.json"))
                after = json.loads(rd(scenario + "/🔣️.after.json"))
                wire = json.loads(rd(scenario + "/🔣️.mutation.json"))
                status = json.loads(rd(scenario + "/🔣️.outcome.json")).get("status")
            except FileNotFoundError:
                continue
            _tag, arguments = document.unwrap(wire)
            for problem in schema_problems(schema, arguments):
                failures.append("%s/%s: schema: %s" % (kind, os.path.basename(scenario), problem))
            checked += 1
            try:
                got = verbs.apply_mutation(before, kind, arguments)
            except document.Refused as reason:
                if status == "applied":
                    failures.append("%s/%s: refused: %s" % (kind, os.path.basename(scenario), reason))
                continue
            if status == "applied" and got != after:
                failures.append("%s/%s: after-snapshot differs" % (kind, os.path.basename(scenario)))
                continue
            if status != "applied":
                continue
            steps = verbs.inverse_mutation(kinds, before, kind, arguments)
            restored = got
            view = document.derived_view(before)
            try:
                for step, payload in steps:
                    restored = verbs.apply_mutation(restored, step, payload, view)
            except document.Refused as reason:
                failures.append("%s/%s: inverse step refused: %s" % (kind, os.path.basename(scenario), reason))
                continue
            inverted += 1
            if restored != before:
                failures.append("%s/%s: inverse does not restore the before snapshot" % (kind, os.path.basename(scenario)))
    print("\n".join(failures))
    print("orphan fixture dirs (no kind): %s" % ", ".join(orphans))
    print("%s: %d scenarios, %d inverted, %d failures" % (short, checked, inverted, len(failures)))
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
