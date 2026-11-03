import sys, json
sys.path.insert(0, ".")
from rc_run import *
NS = "semio_s_plugin_norm_compliance"
R = ART + "/🌍️en1997" + SUBSET
S = R + "/🧬️schema"

def model():
    m = f"{S}/📸️snapshot/🦀️.rs"
    sub_once(m, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};", marker="AnnexChoice, ReliabilityClass}")
    rs_struct_append(m, "En1997Snapshot", "    #[state(artifact)]\n    pub reliability_class: ReliabilityClass,\n")
    clone_lines(m, r"^ +annex: AnnexChoice::\w+,\n", rs_convert, None, marker="reliability_class: ReliabilityClass::RC2")
    a = f"{S}/🦀️.rs"
    rs_struct_append(a, "En1997Artifact", f"    #[state(artifact)]\n    pub reliability_class: {NS}::ReliabilityClass,\n")
    clone_lines(a, r"^ +annex: (self|snapshot)\.annex,\n", rs_convert, 2, marker="reliability_class: self.reliability_class")
    d = f"{S}/🔺️diff/🦀️.rs"
    rs_struct_append(d, "En1997Diff", f"    #[state(artifact)]\n    pub reliability_class: Option<{NS}::ReliabilityClass>,\n")
    clone_lines(d, r"^        if other\.annex\.is_some\(\) \{\n            self\.annex = other\.annex;\n        \}\n", rs_convert, 1, marker="other.reliability_class")
    clone_lines(d, r"^        if let Some\(value\) = &self\.annex \{\n            next\.annex = value\.clone\(\);\n        \}\n", rs_convert, 1, marker="next.reliability_class")
    clone_lines(d, r"^            annex: self\.annex\.as_ref\(\)[^\n]*\n", rs_convert, 1, marker="reliability_class: self.reliability_class.as_ref")
    sub_once(d, r"self\.annex\.is_none\(\) && ", "self.annex.is_none() && self.reliability_class.is_none() && ", marker="self.reliability_class.is_none()")

def sqlite():
    Q = R + "/🚪️io/🪶️sqlite/📸️snapshot"
    sub_once(Q + "/🗄️.sql", r"( groundwater_level REAL, investigation_depth REAL,)\n", r"\1 reliability_class TEXT NOT NULL CHECK(reliability_class IN ('RC1','RC2','RC3')),\n", marker="reliability_class TEXT")
    r = Q + "/🦀️.rs"
    sub_once(r, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice,ReliabilityClass};", marker="AnnexChoice,ReliabilityClass}")
    clone_lines(r, r"^fn read_annex[^\n]*\n", lambda s: 'fn reliability_class(a:ReliabilityClass)->&\'static str{match a{ReliabilityClass::RC1=>"RC1",ReliabilityClass::RC2=>"RC2",ReliabilityClass::RC3=>"RC3"}}\nfn read_reliability_class(a:&str)->Result<ReliabilityClass, ValueError>{match a{"RC1"=>Ok(ReliabilityClass::RC1),"RC2"=>Ok(ReliabilityClass::RC2),"RC3"=>Ok(ReliabilityClass::RC3),_=>Err(invalid("EN1997 reliability class differs"))}}\n', 1, marker="fn reliability_class(")
    sub_once(r, r"(Cell::Real\(self\.groundwater_level\),Cell::Real\(self\.investigation_depth\))\]", r"\1,Cell::Text(reliability_class(self.reliability_class))]", marker="Cell::Text(reliability_class(")
    sub_once(r, r'entities\(d,"en1997_document",8,', 'entities(d,"en1997_document",9,', marker='"en1997_document",9,')
    sub_once(r, r"investigation_depth:r\.real\(7\)\?,", "investigation_depth:r.real(7)?,reliability_class:read_reliability_class(r.text(8)?)?,", marker="reliability_class:read_reliability_class")
    t = Q + "/🟦️.ts"
    sub_once(t, r"investigationDepth:Binary64;layers:", 'investigationDepth:Binary64;reliabilityClass:"RC1"|"RC2"|"RC3";layers:', marker='reliabilityClass:"RC1"')
    clone_lines(t, r"^function annex\([^\n]*\n", lambda s: 'function reliabilityClass(s:string):"RC1"|"RC2"|"RC3"{if(s!=="RC1"&&s!=="RC2"&&s!=="RC3")throw Error("EN1997 reliability class differs");return s}\n', 1, marker="function reliabilityClass(")
    sub_once(t, r'"investigationDepth","layers"', '"investigationDepth","reliabilityClass","layers"', marker='"reliabilityClass","layers"')
    sub_once(t, r"s\.groundwaterLevel,s\.investigationDepth\],DOCUMENT", "s.groundwaterLevel,s.investigationDepth,reliabilityClass(s.reliabilityClass)],DOCUMENT", marker="reliabilityClass(s.reliabilityClass)")
    sub_once(t, r'entities\(t\[0\]!,8,DOCUMENT,o\)', 'entities(t[0]!,9,DOCUMENT,o)', marker='entities(t[0]!,9,DOCUMENT')
    sub_once(t, r"(investigationDepth:readBinary64\(r,7,DOCUMENT\),)", r"\1reliabilityClass:reliabilityClass(artifactSqliteText(r,8)),", marker="reliabilityClass:reliabilityClass(")
    x = Q + "/🧪️tests/🦀️.rs"
    clone_lines(x, r"[^\n]*\(1,SqliteValue::Text\(\"unknown annex\"[^\n]*\n", lambda s: "", None, marker="@@never@@") if False else None
    y = Q + "/🧪️tests/🟦️.ts"
    sub_once(y, r'\["en1997_document",5,"unknown annex"\],', '["en1997_document",5,"unknown annex"],["en1997_document",8,"unknown class"],', marker='["en1997_document",8,"unknown class"]')
    contract_update(Q + "/🧪️tests/🧫️fixtures/🎛️semantic.json", Q + "/🗄️.sql", 3, [y])
    a = Q + "/🛂️admission/🦀️.rs"
    sub_once(a, r'Enumeration\(4,&\["En","De"\]\),Real\(5\),Real\(6\)\]\)\?,DOCUMENT\)', 'Enumeration(4,&["En","De"]),Real(5),Real(6),Enumeration(20,&["RC1","RC2","RC3"])])?,DOCUMENT)', marker='Enumeration(20,')
    fx = Q + "/🧪️tests/🧫️fixtures/🔣️.json"
    sub_once(fx, r'("annex": "[A-Za-z]+",)', r'\1\n  "reliabilityClass": "RC3",', marker='"reliabilityClass"')
    ne = Q + "/🧪️tests/🧫️fixtures/📏️native-extent/🔣️.json"
    t = rd(ne)
    if '"@rc"' not in t and "rcBytes" not in t:
        d = json.loads(t)
        for c in d["cases"]:
            c["valueBytes"] += 3
        d["rcBytes"] = 3
        wr(ne, json.dumps(d, indent=2, ensure_ascii=False) + "\n")
    ind = Q + "/🧪️tests/🧫️fixtures/🔬️independent/🔣️.json"
    t = rd(ind)
    d = json.loads(t)
    assert json.dumps(d, indent=2, ensure_ascii=False) + "\n" == t, "independent fixture format"
    for v in d["variants"]:
        for tb in v["tables"]:
            if tb["name"] == "en1997_document" and "reliability_class" not in tb["columns"]:
                i = tb["columns"].index("investigation_depth") + 1
                tb["columns"].insert(i, "reliability_class")
                for row in tb["rows"]:
                    row.insert(i, "RC3")
    wr(ind, json.dumps(d, indent=2, ensure_ascii=False) + "\n")

def meta():
    f = S + "/💡️inferences/⚖️checks/🦀️.rs"
    sub_once(f, r"^    const ANNEX: ", '    const RC: &[NormFieldChoice] = &[\n        choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"),\n        choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"),\n        choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)"),\n    ];\n    const ANNEX: ', flags=re.M, marker="const RC: ")
    clone_lines(f, r'^        \("annex", FieldSpec::choice\([^\n]*\n', lambda s: '        ("reliabilityClass", FieldSpec::choice("Reliability class RC", "Zuverlässigkeitsklasse RC", RC)),\n', 1, marker='("reliabilityClass"')
    sub_once(S + "/💡️inferences/🧾outline/🦀️.rs", r'^    "annex",\n', '    "annex",\n    "reliability_class",\n', flags=re.M, marker='"reliability_class"')

def run(dry):
    rc_lib.DRY = dry
    twins(S)
    model()
    sqlite()
    import glob
    fixtures(R, S, [p.split("/")[-1] for p in glob.glob(R + "/🖼️assets/*")])
    leaf(R, S, "🌍️en1997", "en1997", "En1997")
    meta()
    finish(dry)

if __name__ == "__main__":
    run("--apply" not in sys.argv)
