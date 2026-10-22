import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
x = []
K = "stdio-artifacts/"
CV, VC = "policyStandardsCoverageBreaches", "policyStandardSubsetVocabularyBreaches"
ART = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a"
SC = ART + "/🏅standards/🔖1"
STD_ART = "✏s/🔌plugins/🗄stdio/🗿artifacts/🔸foo"
STD_SC = STD_ART + "/🏅standards/🔖1"


def b(i, kind):
    return [i, K + kind, "high", 0]


def case(id_, rule, files, expect):
    cases.append({"id": id_, "rule": rule, "files": files, "expect": jsort(expect)})


def full(std, name="✳any"):
    f = {std + "/🔨modules/x": x}
    for c in ("🧬schema", "🚪io", "📚examples", "👁viewer", "✏editor"):
        f["%s/🪆subsets/%s/%s/x" % (std, name, c)] = x
    return f


case("complete-standard-is-covered", CV, full(SC), [])
case("standard-slugs-are-lowercase-kebab", CV, full(ART + "/🏅standards/🔖Bad_Slug"), [b("standards-slug-" + ART + "/🏅standards/🔖Bad_Slug", "standards-coverage")])
case("subset-directories-need-the-subset-prefix", CV, full(SC, "plain"), [b("standards-subset-prefix-" + SC + "/🪆subsets/plain", "standards-coverage")])
case("missing-standard-and-subset-children", CV, {SC + "/🪆subsets/✳any/🧬schema/x": x}, [b("standards-standard-child-%s-🔨modules" % SC, "standards-coverage")] + [b("standards-subset-child-%s/🪆subsets/✳any-%s" % (SC, c), "standards-coverage") for c in ("🚪io", "📚examples", "👁viewer", "✏editor")])
VF = "stdio-artifacts/standards-subset-vocabulary"
M = lambda std: std + "/🪆subsets/🔣.json"
vb = lambda k, std: b("standards-subset-vocabulary-%s-%s" % (k, std), "standards-subset-vocabulary")
case("manifest-matching-the-directories-is-clean", VC, {M(SC): ['{"artifact": "a", "standard": "1", "subsets": {"*": {}}}'], SC + "/🪆subsets/✳any/x": x}, [])
case("missing-manifest", VC, {SC + "/🪆subsets/✳any/x": x}, [vb("missing", SC)])
case("unparseable-manifest", VC, {M(SC): ["{not json"], SC + "/🪆subsets/✳any/x": x}, [vb("invalid", SC)])
case("standard-and-any-are-declared", VC, {M(SC): ['{"standard": "2", "subsets": {"x": {}}}'], SC + "/🪆subsets/✳x/y": x},
     [vb("standard-mismatch", SC), vb("missing-any", SC), vb("domain-real-subset", SC), b("standards-subset-vocabulary-undeclared-dir-%s-✳x" % SC, "standards-subset-vocabulary") if False else b("standards-subset-vocabulary-missing-dir-%s-✳any" % SC, "standards-subset-vocabulary")][:3] if False else
     [vb("standard-mismatch", SC), vb("missing-any", SC), vb("domain-real-subset", SC)])
case("subset-ids-need-a-slug", VC, {M(SC): ['{"standard": "1", "subsets": {"*": {}, "Bad_Id": {}}}'], SC + "/🪆subsets/✳any/x": x},
     [b("standards-subset-vocabulary-bad-id-%s-Bad_Id" % SC, "standards-subset-vocabulary"), vb("domain-real-subset", SC), b("standards-subset-vocabulary-unmapped-id-%s-Bad_Id" % SC, "standards-subset-vocabulary")])
case("declared-and-actual-subset-directories-agree", VC, {M(SC): ['{"standard": "1", "subsets": {"*": {}, "x": {}}}'], SC + "/🪆subsets/✳any/a": x, SC + "/🪆subsets/✳y/a": x},
     [vb("domain-real-subset", SC), b("standards-subset-vocabulary-undeclared-dir-%s-✳y" % SC, "standards-subset-vocabulary"), b("standards-subset-vocabulary-missing-dir-%s-✳x" % SC, "standards-subset-vocabulary")])
S1, S2, S3 = (STD_SC + "/🪆subsets/✳" + n for n in ("x", "w", "any"))
case("stdio-subsets-need-a-registered-validator", VC, {
    M(STD_SC): ['{"standard": "1", "subsets": {"*": {}, "x": {}, "w": {}}}'], S3 + "/a": x,
    S1 + "/🚪io/🦀.rs": ["impl SubsetValidator for V {}", "fn init() { register_subset_validator(v); }"], S2 + "/🚪io/🦀.rs": ["impl SubsetValidator for V {}", "// register_subset_validator(v);"],
}, [b("standards-subset-vocabulary-validator-missing-" + S2, "standards-subset-vocabulary")])
rules = [CV, VC]
schema = std_schema("Standards Law Corpus", rules)
ts = std_ts(
    "🧾 The stdio standards laws (slug and child coverage, subset-vocabulary manifest) report exactly the breaches their language-agnostic corpus names over synthetic artifact trees, agree with an independent Python model and reject a malformed corpus.",
    'import { policyStandardSubsetVocabularyBreaches, policyStandardsCoverageBreaches } from "../../🟦.ts";',
    "  %s: (root) => %s(root),\n  %s: (root) => %s(root)," % (CV, CV, VC, VC),
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🚪artifact-io/🧾standards", "🧾standards", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
