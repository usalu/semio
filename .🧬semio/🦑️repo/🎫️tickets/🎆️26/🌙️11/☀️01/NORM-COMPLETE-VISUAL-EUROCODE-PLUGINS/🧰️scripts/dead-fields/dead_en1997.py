import glob, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from dead_lib import *
R = ART + "/🌍️en1997" + SUBSET
S = R + "/🧬️schema"
M = S + "/🧬️mutations"
Q = R + "/🚪️io/🪶️sqlite/📸️snapshot"
T = R + "/🧪️tests"
F = Q + "/🧪️tests/🧫️fixtures"
K = KIND


def contract():
    old_sql = rd(Q + "/🗄️.sql")
    sem_path = F + "/🎛️semantic.json"
    sem = json.loads(rd(sem_path))
    old_b, old_w = rc_lib.measure(old_sql)
    print("[contract] stored==replayed:", sem["schemaBytes"] == old_b, sem["tableWidths"] == old_w, old_b)
    return sem_path, sem


def run(dry):
    rc_lib.DRY = dry
    sem_path, sem = contract()
    rmtree(M + "/🎯" + K)
    rmtree(T + "/🧫️fixtures/🧬️mutations/🎯" + K)
    drop_lines(M + "/🌍️design/🦀️.rs", r'^#\[path = "\.\./🎯change-reliability-class/🦀️\.rs"\]\npub mod change_reliability_class;\n')
    f = M + "/🦀️.rs"
    drop_lines(f, r"^use [\w:]+::change_reliability_class;\n")
    drop_lines(f, r"^    ChangeReliabilityClass\(.*\),\n")
    drop_lines(f, r'^    "change-reliability-class",\n')
    drop_lines(M + "/🧭️edit-rules/🦀️.rs", r'^        SetFieldRule \{ path: "reliabilityClass".*\n')
    drop_lines(R + "/🚪️io/💾️binary/🧬️mutations/🦀️.rs", r"^use [\w:]+::\{change_reliability_class, .*\n")
    drop_lines(R + "/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio", r"^record change-reliability-class tag=\d+\nfield payload bytes\n")
    f = R + "/🚪️io/📝️text/🧬️mutations/🦀️.rs"
    drop_lines(f, r"^use [\w:]+::\{change_reliability_class, .*\n")
    drop_lines(f, r"^use [\w:]+::change_reliability_class;\n")
    sub(f, r"\{AnnexChoice, ReliabilityClass\}", "AnnexChoice")
    drop_lines(f, r"^    ChangeReliabilityClass \{.*\n")
    drop_lines(f, r"^        En1997Mutation::ChangeReliabilityClass\(.*\n")
    drop_lines(f, r"^        En1997MutationDsl::ChangeReliabilityClass \{.*\n")
    f = T + "/🌍️mutate-en1997-1/🦀️.rs"
    drop_lines(f, r'^ *"change-reliability-class"(,| => \()\n', 2)
    drop_lines(f, r"^.*🎯change-reliability-class.*\n", 4)
    drop_lines(T + "/🌍️mutate-en1997-1/🐍️.py", r'^ *"change-reliability-class"(,|: .*)\n', 2)
    drop_lines(T + "/🌍️mutate-en1997-1/🥒️.feature", r"^ *\| change-reliability-class .*\n", 2)
    drop_lines(T + "/↩️round-trip-en1997-1/🥒️.feature", r"^ *\| change-reliability-class .*\n")
    f = R + "/🔮️oracles/🔣️.json"
    drop_objects(f, '"mutationId": "change-reliability-class"', 3)
    drop_lines(f, r'^ *"change-reliability-class",\n', 3)
    drop_objects(f, '"id": "change-reliability-class"', 3)

    f = S + "/💡️inferences/⚖️checks/🦀️.rs"
    sub(f, r'^    const RC: &\[NormFieldChoice\] = &\[\n(        choice\(.*\n){3}    \];\n', "", 1, re.M)
    drop_lines(f, r'^        \("reliabilityClass", .*\n')
    drop_lines(S + "/💡️inferences/🧾outline/🦀️.rs", r'^    "reliability_class",\n')
    f = S + "/💡️inferences/⚖️checks/🧭️design-approach-and-parameters/🦀️.rs"
    sub(f, r"parse_design_situation\(situation\), reliability_class\)", "parse_design_situation(situation))")
    sub(f, r"The reliability class and K_FI of EN 1990 Annex B do not scale them", "The K_FI of EN 1990 Annex B does not scale them")
    f = S + "/🧪️tests/⚖️compliance/🦀️.rs"
    sub(f, r"\n#\[test\]\nfn reliability_class_does_not_scale.*?\n}\n\n#\[test\]\nfn reliability_class_of_the_document_leaves.*?\n}\n(?=\n|$)", "", 1, re.S)
    sub(f, r"\{AnnexChoice, CheckStatus, ReliabilityClass\}", "{AnnexChoice, CheckStatus}")
    f = S + "/📸️snapshot/🦀️.rs"
    sub(f, r"\{AnnexChoice, ReliabilityClass\}", "AnnexChoice")
    drop_lines(f, r"^        reliability_class: ReliabilityClass::RC2,\n")
    f = S + "/🦀️.rs"
    drop_lines(f, r"^            reliability_class: (self|snapshot)\.reliability_class,\n", 2)
    f = S + "/🔺️diff/🦀️.rs"
    sub(f, r"^        if other\.reliability_class\.is_some\(\) \{\n            self\.reliability_class = other\.reliability_class;\n        \}\n", "", 1, re.M)
    sub(f, r"^        if let Some\(value\) = &self\.reliability_class \{\n            next\.reliability_class = value\.clone\(\);\n        \}\n", "", 1, re.M)
    sub(f, r" && self\.reliability_class\.is_none\(\)", "")
    drop_lines(f, r"^            reliability_class: self\.reliability_class\.as_ref\(\).*\n")
    sub(f, r"    #\[state\(artifact\)\]\n    pub reliability_class: Option<semio_s_plugin_norm_compliance::ReliabilityClass>,\n", "")

    sub(Q + "/🗄️.sql", r" reliability_class TEXT NOT NULL CHECK\(reliability_class IN \('RC1','RC2','RC3'\)\),", "")
    f = Q + "/🦀️.rs"
    sub(f, r"\{AnnexChoice,ReliabilityClass\}", "AnnexChoice")
    drop_lines(f, r"^fn (read_)?reliability_class\(.*\n", 2)
    sub(f, r",Cell::Text\(reliability_class\(self\.reliability_class\)\)", "")
    sub(f, r'entities\(d,"en1997_document",9,', 'entities(d,"en1997_document",8,')
    sub(f, r"reliability_class:read_reliability_class\(r\.text\(8\)\?\)\?,", "")
    f = Q + "/🟦️.ts"
    sub(f, r'reliabilityClass:"RC1"\|"RC2"\|"RC3";', "")
    drop_lines(f, r"^function reliabilityClass\(.*\n")
    sub(f, r'"investigationDepth","reliabilityClass",', '"investigationDepth",')
    sub(f, r",reliabilityClass\(s\.reliabilityClass\)", "")
    sub(f, r"entities\(t\[0\]!,9,DOCUMENT", "entities(t[0]!,8,DOCUMENT")
    sub(f, r"reliabilityClass:reliabilityClass\(artifactSqliteText\(r,8\)\),", "")
    f = Q + "/🛂️admission/🦀️.rs"
    sub(f, r',Enumeration\(20,&\["RC1","RC2","RC3"\]\)', "")
    sub(Q + "/🧪️tests/🟦️.ts", r'\["en1997_document",8,"unknown class"\],', "")

    new_b, new_w = rc_lib.measure(rd(Q + "/🗄️.sql"))
    sem["schemaBytes"], sem["tableWidths"] = new_b, new_w
    for case in sem["cases"]:
        case["valueBytes"] -= 3
    wr(sem_path, json.dumps(sem, indent=2, ensure_ascii=False) + "\n")
    t = Q + "/🧪️tests/🟦️.ts"
    text = rd(t)
    text = re.sub(r'(\["schemaBytes"\]\)\.toEqual\()\d+', lambda m: m.group(1) + str(new_b), text)
    text = re.sub(r'(\["tableWidths"\]\)\.toEqual\()\{[^}]*\}', lambda m: m.group(1) + json.dumps(new_w, separators=(",", ":")), text)
    wr(t, text)
    p = F + "/📏️native-extent/🔣️.json"
    ne = json.loads(rd(p))
    for case in ne["cases"]:
        case["valueBytes"] -= 3
    wr(p, json.dumps(ne, indent=2, ensure_ascii=False) + "\n")
    p = F + "/🔬️independent/🔣️.json"
    ind = json.loads(rd(p))
    for v in ind["variants"]:
        for tbl in v["tables"]:
            if tbl["name"] == "en1997_document":
                i = tbl["columns"].index("reliability_class")
                del tbl["columns"][i]
                for row in tbl["rows"]:
                    assert row[i] == "RC3"
                    del row[i]
                ind["totalCells"] -= len(tbl["rows"])
    wr(p, json.dumps(ind, indent=2, ensure_ascii=False) + "\n")

    for p in glob.glob(R + "/🖼️assets/*/🗣️.dsl.semio"):
        drop_dsl_token(p)
    for j in (S + "/🔣️.json", S + "/📸️snapshot/🔣️.json"):
        drop_required(j)
    for j in (S + "/🔣️.json", S + "/📸️snapshot/🔣️.json", S + "/🔺️diff/🔣️.json"):
        drop_json_key(j, "reliabilityClass")
    print("fixtures", sweep_fixtures(R))
    finish(dry)


if __name__ == "__main__":
    run("--apply" not in sys.argv)
