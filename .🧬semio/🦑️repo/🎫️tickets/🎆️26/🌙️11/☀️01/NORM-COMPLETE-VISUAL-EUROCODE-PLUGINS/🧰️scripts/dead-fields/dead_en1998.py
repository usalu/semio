import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sys
from dead_lib import *
R = ART + "/🫨️en1998" + SUBSET
S = R + "/🧬️schema"
M = S + "/🧬️mutations"
Q = R + "/🚪️io/🪶️sqlite/📸️snapshot"
T = R + "/🧪️tests"
K = KIND


def run(dry):
    rc_lib.DRY = dry
    rmtree(M + "/🎯" + K)
    rmtree(T + "/🧫️fixtures/🧬️mutations/🎯" + K)
    drop_lines(M + "/🏢️buildings-annex-building/🦀️.rs", r'^#\[path = "\.\./🎯change-reliability-class/🦀️\.rs"\]\npub mod change_reliability_class;\n')
    sub(M + "/🏢️buildings/🦀️.rs", r"change_reliability_class, ", "")
    f = M + "/🦀️.rs"
    drop_lines(f, r"^use [\w:]+::change_reliability_class;\n")
    drop_lines(f, r"^    ChangeReliabilityClass\(.*\),\n")
    drop_lines(f, r'^    "change-reliability-class",\n')
    f = M + "/🎲️demo-cases/🦀️.rs"
    drop_lines(f, r"^use [\w:]+::change_reliability_class(::ChangeReliabilityClass)?;\n", 2)
    drop_lines(f, r"^        En1998Mutation::ChangeReliabilityClass\(.*\n")
    f = R + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    drop_lines(f, r"^use [\w:]+::change_reliability_class(::ChangeReliabilityClass)?;\n", 2)
    drop_lines(f, r"^        En1998Mutation::ChangeReliabilityClass\(.*\n")
    drop_lines(f, r'^        "change-reliability-class" => .*\n')
    drop_lines(M + "/🧭️edit-rules/🦀️.rs", r'^        SetFieldRule \{ path: "reliabilityClass".*\n')
    drop_lines(R + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", r"^record change-reliability-class tag=\d+\nfield payload bytes\n")
    f = T + "/🫨️mutate-en1998-1/🦀️.rs"
    drop_lines(f, r'^ *"change-reliability-class"(,| => \()\n', 2)
    drop_lines(f, r"^.*🎯change-reliability-class.*\n", 4)
    drop_lines(T + "/🫨️mutate-en1998-1/🐍️.py", r'^ *"change-reliability-class"(,|: .*)\n', 2)
    drop_lines(T + "/🫨️mutate-en1998-1/🥒️.feature", r"^ *\| change-reliability-class .*\n", 2)
    drop_lines(T + "/↩️round-trip-en1998-1/🥒️.feature", r"^ *\| change-reliability-class .*\n")
    f = R + "/🔮️oracles/🔣️.json"
    drop_objects(f, '"mutationId": "change-reliability-class"', 1)
    drop_lines(f, r'^ *"change-reliability-class",\n')
    drop_objects(f, '"id": "change-reliability-class"', 1)

    f = R + "/🏷️field-meta/🦀️.rs"
    sub(f, r'^const RC: &\[NormFieldChoice\] = &\[\n(    NormFieldChoice .*\n){3}\];\n', "", 1, re.M)
    drop_lines(f, r'^    \("reliabilityClass", .*\n')
    drop_lines(S + "/💡️inferences/🧾outline/🦀️.rs", r'^    "reliabilityClass",\n')
    sub(S + "/💡️inferences/🦀️.rs", r'"annex", "reliabilityClass", ', '"annex", ')
    f = S + "/📸️snapshot/🦀️.rs"
    drop_lines(f, r"^            reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass::RC2,\n")
    f = S + "/🦀️.rs"
    drop_lines(f, r"^            reliability_class: (self|snapshot)\.reliability_class,\n", 2)
    f = S + "/🔺️diff/🦀️.rs"
    sub(f, r"^        if other\.reliability_class\.is_some\(\) \{\n            self\.reliability_class = other\.reliability_class;\n        \}\n", "", 1, re.M)
    sub(f, r"^        if let Some\(value\) = &self\.reliability_class \{\n            next\.reliability_class = value\.clone\(\);\n        \}\n", "", 1, re.M)
    drop_lines(f, r"^            && self\.reliability_class\.is_none\(\)\n")
    drop_lines(f, r"^            reliability_class: self\.reliability_class\.as_ref\(\).*\n")
    f = S + "/🧪️tests/⚖️compliance/🦀️.rs"
    sub(f, r"\n#\[test\]\nfn reliability_class_rc3_raises.*?\n}\n\n#\[test\]\nfn reliability_class_of_the_document_reaches.*?\n}\n", "", 1, re.S)
    sub(f, r"\{ActionFactors, AnnexChoice, CheckReport, CheckStatus, DesignSituation, ReliabilityClass\}", "{AnnexChoice, CheckReport, CheckStatus}")

    f = Q + "/🗄️.sql"
    sub(f, r",reliability_class TEXT NOT NULL CHECK\(reliability_class IN \('RC1','RC2','RC3'\)\)", "")
    f = Q + "/🦀️.rs"
    drop_lines(f, r"^fn (read_)?reliability_class(_text)?\(.*\n", 2)
    sub(f, r",Cell::Text\(reliability_class_text\(self\.reliability_class\)\)", "")
    sub(f, r"bound\.repeated\(3,24\)\?;", "")
    sub(f, r'entities\(database,"en1998_document",3,', 'entities(database,"en1998_document",2,')
    sub(f, r"reliability_class:read_reliability_class\(d\.text\(2\)\?\)\?,", "")
    f = Q + "/🟦️.ts"
    drop_lines(f, r"^function reliabilityClass\(.*\n")
    sub(f, r",reliabilityClass\(snapshot\.reliabilityClass\)", "")
    sub(f, r"entities\(tables\[0\]!,3,\[\]", "entities(tables[0]!,2,[]")
    sub(f, r"reliabilityClass:reliabilityClass\(artifactSqliteText\(d,2\)\),", "")

    import glob
    for p in glob.glob(R + "/🖼️assets/*/*/🗣️.dsl.semio"):
        drop_dsl_token(p)

    drop_required(S + "/🔣️.json")
    drop_required(S + "/📸️snapshot/🔣️.json")
    for j in (S + "/🔣️.json", S + "/📸️snapshot/🔣️.json", S + "/🔺️diff/🔣️.json"):
        drop_json_key(j, "reliabilityClass")
    print("fixtures", sweep_fixtures(R))
    finish(dry)


if __name__ == "__main__":
    run("--apply" not in sys.argv)
