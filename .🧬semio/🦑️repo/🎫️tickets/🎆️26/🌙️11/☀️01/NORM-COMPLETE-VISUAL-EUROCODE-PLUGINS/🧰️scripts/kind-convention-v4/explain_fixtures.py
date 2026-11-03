"""🔎 Prints the first differing JSON paths between the engine result and the committed after snapshot of every failing apply scenario (debug companion of verify_fixtures.py)."""
import glob, json, os, sys
sys.argv = [sys.argv[0]] + sys.argv[1:]
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(os.path.dirname(HERE), "leaf-kinds"))
from common import *
import verify_fixtures as v


def diffs(a, b, path=""):
    if type(a) != type(b):
        yield path, a, b
    elif isinstance(a, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b:
                yield path + "." + k, a.get(k, "<absent>"), b.get(k, "<absent>")
            else:
                yield from diffs(a[k], b[k], path + "." + k)
    elif isinstance(a, list):
        if len(a) != len(b):
            yield path + "[len]", len(a), len(b)
        for i, (x, y) in enumerate(zip(a, b)):
            yield from diffs(x, y, "%s[%d]" % (path, i))
    elif a != b:
        yield path, a, b


def main():
    short, only = sys.argv[1], sys.argv[2]
    v.load("semio_norm_vocabulary_document", "📄️document")
    verbs = v.load("semio_norm_vocabulary_verbs", "🔧️verbs")
    verbs.declare_kinds([r for r in json.loads(rd(TAXONOMY))["rows"] if r["artifact"] == v.EMOJI[short] + short])
    verbs.declare_catalogue(v.load("semio_norm_vocabulary_catalogue", "📚️catalogue").section_catalogue(NORM))
    document = sys.modules["semio_norm_vocabulary_document"]
    base = NORM + "/🗿️artifacts/" + v.EMOJI[short] + short + "/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/"
    for folder in glob.glob(base + "*"):
        kind = v.kind_of(folder)
        if kind != only:
            continue
        for sc in sorted(glob.glob(folder + "/*")):
            before = json.loads(rd(sc + "/🔣️.before.json")); after = json.loads(rd(sc + "/🔣️.after.json")); wire = json.loads(rd(sc + "/🔣️.mutation.json"))
            _t, args = document.unwrap(wire)
            try:
                got = verbs.apply_mutation(before, kind, args)
            except document.Refused as r:
                print(sc, "refused", r); continue
            for n, (p, x, y) in enumerate(diffs(got, after)):
                if n < 6: print(os.path.basename(sc), p, "engine=", str(x)[:80], "committed=", str(y)[:80])


main()
