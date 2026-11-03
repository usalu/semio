"""🧪 Regenerates the before/after snapshots of the en1993 insert/remove frame-bar, frame-node and joint-row vectors from ONE base with the Python engine (`insert.before` is the base, `insert.after` and `remove.before` are the inserted document, `remove.after` is the base again); the Rust apply path (`insert_*::diff`: clamped insertion, `remove_*::diff`: removal at the index) has no further semantics, so nothing is hand-fixed.

`python3 -I en1993_single_base.py` writes the six `after`/`before` snapshots; idempotent."""
import glob, importlib.util, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *

ENGINE = NORM + "/🔮️oracles/🏃️execution"
BASE = NORM + "/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations"


def load(alias, folder):
    spec = importlib.util.spec_from_file_location(alias, os.path.join(ENGINE, folder, "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


def scenario(kind):
    return glob.glob("%s/*%s/✅apply" % (BASE, kind))[0]


def read(path):
    return json.loads(rd(path))


def write(path, value):
    open(path, "w", encoding="utf-8").write(dumps(value))


def main():
    document = load("semio_norm_vocabulary_document", "📄️document")
    verbs = load("semio_norm_vocabulary_verbs", "🔧️verbs")
    verbs.declare_kinds([row for row in read(TAXONOMY)["rows"] if row["artifact"] == "🔩️en1993"])
    for noun in ("frame-bar", "frame-node", "joint-row"):
        insert, remove = scenario("insert-" + noun), scenario("remove-" + noun)
        base = read(insert + "/🔣️.before.json")
        _tag, arguments = document.unwrap(read(insert + "/🔣️.mutation.json"))
        inserted = verbs.apply_mutation(base, "insert-" + noun, arguments)
        _tag, removal = document.unwrap(read(remove + "/🔣️.mutation.json"))
        assert verbs.apply_mutation(inserted, "remove-" + noun, removal) == base, noun
        write(insert + "/🔣️.after.json", inserted)
        write(remove + "/🔣️.before.json", inserted)
        write(remove + "/🔣️.after.json", base)
        print("regenerated", noun)


if __name__ == "__main__":
    main()
