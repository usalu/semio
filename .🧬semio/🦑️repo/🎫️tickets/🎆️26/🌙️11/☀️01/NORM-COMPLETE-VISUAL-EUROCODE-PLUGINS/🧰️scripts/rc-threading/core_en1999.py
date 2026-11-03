import sys
sys.path.insert(0, ".")
from rc_run import *
NS = "semio_s_plugin_norm_compliance"
R = ART + "/🪶️en1999" + SUBSET
S = R + "/🧬️schema"

def model():
    m = f"{S}/📸️snapshot/🦀️.rs"
    sub_once(m, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};", marker="AnnexChoice, ReliabilityClass}")
    rs_struct_append(m, "En1999Snapshot", "    #[state(artifact)]\n    pub reliability_class: ReliabilityClass,\n")
    clone_lines(m, r"^ +annex: AnnexChoice::\w+,\n", rs_convert, None, marker="reliability_class: ReliabilityClass::RC2")
    a = f"{S}/🦀️.rs"
    rs_struct_append(a, "En1999Artifact", f"    #[state(artifact)]\n    pub reliability_class: {NS}::ReliabilityClass,\n")
    clone_lines(a, r"^ +annex: (self|snapshot)\.annex,\n", rs_convert, 2, marker="reliability_class: self.reliability_class")
    d = f"{S}/🔺️diff/🦀️.rs"
    rs_struct_append(d, "En1999Diff", f"    #[state(artifact)]\n    pub reliability_class: Option<{NS}::ReliabilityClass>,\n")
    clone_lines(d, r"^        if other\.annex\.is_some\(\) \{\n            self\.annex = other\.annex;\n        \}\n", rs_convert, 1, marker="other.reliability_class")
    clone_lines(d, r"^        if let Some\(value\) = &self\.annex \{\n            next\.annex = value\.clone\(\);\n        \}\n", rs_convert, 1, marker="next.reliability_class")
    clone_lines(d, r"^            annex: self\.annex\.as_ref\(\)[^\n]*\n", rs_convert, 1, marker="reliability_class: self.reliability_class.as_ref")
    sub_once(d, r"self\.annex\.is_none\(\) && ", "self.annex.is_none() && self.reliability_class.is_none() && ", marker="self.reliability_class.is_none()")

def sqlite():
    Q = R + "/🚪️io/🪶️sqlite/📸️snapshot"
    sub_once(Q + "/🗄️.sql", r"(CREATE TABLE en1999_document \(id INTEGER PRIMARY KEY CHECK\(id=1\), annex TEXT NOT NULL CHECK\(annex IN \('En','De'\)\))\);", r"\1, reliability_class TEXT NOT NULL CHECK(reliability_class IN ('RC1','RC2','RC3')));", marker="reliability_class TEXT")
    r = Q + "/🦀️.rs"
    sub_once(r, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice,ReliabilityClass};", marker="AnnexChoice,ReliabilityClass}")
    clone_lines(r, r"^fn read_annex[^\n]*\n", lambda s: 'fn reliability_class(v:ReliabilityClass)->&\'static str{match v{ReliabilityClass::RC1=>"RC1",ReliabilityClass::RC2=>"RC2",ReliabilityClass::RC3=>"RC3"}}\nfn read_reliability_class(v:&str)->Result<ReliabilityClass, ValueError>{match v{"RC1"=>Ok(ReliabilityClass::RC1),"RC2"=>Ok(ReliabilityClass::RC2),"RC3"=>Ok(ReliabilityClass::RC3),_=>Err(invalid("EN1999 unknown reliability class"))}}\n', 1, marker="fn reliability_class(")
    sub_once(r, r"&\[Cell::Text\(annex\(self\.annex\)\)\]\)\?;", "&[Cell::Text(annex(self.annex)),Cell::Text(reliability_class(self.reliability_class))])?;", marker="Cell::Text(reliability_class(")
    sub_once(r, r'entities\(d,"en1999_document",2,', 'entities(d,"en1999_document",3,', marker='"en1999_document",3,')
    sub_once(r, r"Ok\(Self\{annex:read_annex\(doc\.text\(1\)\?\)\?,", "Ok(Self{annex:read_annex(doc.text(1)?)?,reliability_class:read_reliability_class(doc.text(2)?)?,", marker="reliability_class:read_reliability_class")
    t = Q + "/🟦️.ts"
    sub_once(t, r'export type En1999SqliteSnapshot=\{annex:"En"\|"De";', 'export type En1999SqliteSnapshot={annex:"En"|"De";reliabilityClass:"RC1"|"RC2"|"RC3";', marker='reliabilityClass:"RC1"')
    clone_lines(t, r"^function annex\([^\n]*\n", lambda s: 'function reliabilityClass(s:string):"RC1"|"RC2"|"RC3"{if(s!=="RC1"&&s!=="RC2"&&s!=="RC3")throw Error("EN1999 unknown reliability class");return s}\n', 1, marker="function reliabilityClass(")
    sub_once(t, r'fields\(s,\["annex","materials"', 'fields(s,["annex","reliabilityClass","materials"', marker='"annex","reliabilityClass"')
    sub_once(t, r'out\.insert\("en1999_document",\[annex\(s\.annex\)\],1n\)', 'out.insert("en1999_document",[annex(s.annex),reliabilityClass(s.reliabilityClass)],1n)', marker="reliabilityClass(s.reliabilityClass)")
    sub_once(t, r"return\{annex:annex\(artifactSqliteText\(doc,1\)\),", "return{annex:annex(artifactSqliteText(doc,1)),reliabilityClass:reliabilityClass(artifactSqliteText(doc,2)),", marker="reliabilityClass:reliabilityClass(")
    import re as _re
    tx = rd(t)
    assert "entities(" in tx
    sub_once(t, r'"en1999_document",?\s*', '"en1999_document",', marker="@@@") if False else None
    y = Q + "/🧪️tests/🟦️.ts"
    sub_once(y, r'return\{annex:"De",materials:', 'return{annex:"De",reliabilityClass:"RC3",materials:', marker='annex:"De",reliabilityClass')
    sub_once(y, r"const source:En1999SqliteSnapshot=\{annex,materials:\[\],", 'const source:En1999SqliteSnapshot={annex,reliabilityClass:"RC2",materials:[],', marker='{annex,reliabilityClass:"RC2"')
    contract_update(Q + "/🧪️tests/🧫️fixtures/🎛️semantic.json", Q + "/🗄️.sql", 3, [y])
    a = Q + "/🛂️admission/🦀️.rs"
    sub_once(a, r'n::cells\(root,\[Enumeration\(0,&\["En","De"\]\)\]\)', 'n::cells(root,[Enumeration(0,&["En","De"]),Enumeration(16,&["RC1","RC2","RC3"])])', marker='Enumeration(16,')
    fx = Q + "/🧪️tests/🧫️fixtures/🔣️.json"
    sub_once(fx, r'("annex": "De",)', r'\1\n    "reliabilityClass": "RC3",', marker='"reliabilityClass"')

def meta():
    f = R + "/✏️editor/🏷️field-meta/🦀️.rs"
    sub_once(f, r"^const SUPPORT: ", 'const RC: &[NormFieldChoice] = &[\n    choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"),\n    choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"),\n    choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)"),\n];\nconst SUPPORT: ', flags=re.M, marker="const RC: ")
    clone_lines(f, r'^    \("annex", m\([^\n]*\n', lambda s: '    ("reliabilityClass", m("Reliability class RC", "Zuverlässigkeitsklasse RC", None, Some(RC))),\n', 1, marker='("reliabilityClass"')
    sub_once(S + "/💡️inferences/🧾outline/🦀️.rs", r'^    "annex",\n', '    "annex",\n    "reliabilityClass",\n', flags=re.M, marker='"reliabilityClass"')
    sub_once(S + "/💡️inferences/🦀️.rs", r'reads: &\["annex", "materials"', 'reads: &["annex", "reliabilityClass", "materials"', marker='"annex", "reliabilityClass"')
    sub_once(S + "/💡️inferences/⚖️checks/🧮️units/🦀️.rs", r'Rows::new\(\["annex"\]\)', 'Rows::new(["annex", "reliabilityClass"])', marker='["annex", "reliabilityClass"]')

def run(dry):
    rc_lib.DRY = dry
    twins(S)
    model()
    sqlite()
    import glob
    fixtures(R, S, [p.split("/")[-1] for p in glob.glob(R + "/🖼️assets/*")])
    leaf(R, S, "🪶️en1999", "en1999", "En1999")
    meta()
    finish(dry)

if __name__ == "__main__":
    run("--apply" not in sys.argv)
