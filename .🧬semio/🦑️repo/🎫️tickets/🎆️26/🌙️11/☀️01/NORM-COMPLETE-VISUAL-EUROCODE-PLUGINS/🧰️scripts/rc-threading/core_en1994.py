import sys
sys.path.insert(0, ".")
from rc_run import *
NS = "semio_s_plugin_norm_compliance"
R = ART + "/🧩️en1994" + SUBSET
S = R + "/🧬️schema"

def model():
    m = f"{S}/📸️snapshot/🦀️.rs"
    sub_once(m, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};", marker="AnnexChoice, ReliabilityClass}")
    rs_struct_append(m, "En1994Snapshot", "    #[state(artifact)]\n    pub reliability_class: ReliabilityClass,\n")
    clone_lines(m, r"^ +annex: AnnexChoice::\w+,\n", rs_convert, None, marker="reliability_class: ReliabilityClass::RC2")
    a = f"{S}/🦀️.rs"
    rs_struct_append(a, "En1994Artifact", f"    #[state(artifact)]\n    pub reliability_class: {NS}::ReliabilityClass,\n")
    clone_lines(a, r"^ +annex: (self|snapshot)\.annex,\n", rs_convert, 2, marker="reliability_class: self.reliability_class")
    d = f"{S}/🔺️diff/🦀️.rs"
    rs_struct_append(d, "En1994Diff", f"    #[state(artifact)]\n    pub reliability_class: Option<{NS}::ReliabilityClass>,\n")
    d = f"{S}/🔺️diff/🔁️apply-and-inverse/🦀️.rs"
    clone_lines(d, r"^        if other\.annex\.is_some\(\) \{\n            self\.annex = other\.annex;\n        \}\n", rs_convert, 1, marker="other.reliability_class")
    clone_lines(d, r"^        if let Some\(value\) = &self\.annex \{\n            next\.annex = value\.clone\(\);\n        \}\n", rs_convert, 1, marker="next.reliability_class")
    clone_lines(d, r"^            annex: self\.annex\.as_ref\(\)[^\n]*\n", rs_convert, 1, marker="reliability_class: self.reliability_class.as_ref")
    sub_once(d, r"self\.annex\.is_none\(\) && ", "self.annex.is_none() && self.reliability_class.is_none() && ", marker="self.reliability_class.is_none()")

def sqlite():
    Q = R + "/🚪️io/🪶️sqlite/📸️snapshot"
    sub_once(Q + "/🗄️.sql", r"(fatigue_detail TEXT NOT NULL,)(steel_f_y_pa_ieee754_bits)", r"\1reliability_class TEXT NOT NULL CHECK(reliability_class IN ('RC1','RC2','RC3')),\2", marker="reliability_class TEXT")
    r = Q + "/🦀️.rs"
    sub_once(r, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice,ReliabilityClass};", marker="AnnexChoice,ReliabilityClass}")
    clone_lines(r, r"^fn read_annex[^\n]*\n", lambda s: 'fn reliability_class(value:ReliabilityClass)-> &\'static str{match value{ReliabilityClass::RC1=>"RC1",ReliabilityClass::RC2=>"RC2",ReliabilityClass::RC3=>"RC3"}}\nfn read_reliability_class(value:&str)->Result<ReliabilityClass, ValueError>{match value{"RC1"=>Ok(ReliabilityClass::RC1),"RC2"=>Ok(ReliabilityClass::RC2),"RC3"=>Ok(ReliabilityClass::RC3),_=>Err(invalid("EN1994 unsupported reliability class"))}}\n', 1, marker="fn reliability_class(")
    sub_once(r, r"Cell::Text\(&self\.fatigue_detail\)\],DOCUMENT\)", "Cell::Text(&self.fatigue_detail),Cell::Text(reliability_class(self.reliability_class))],DOCUMENT)", marker="Cell::Text(reliability_class(")
    sub_once(r, r'entities\(database,"en1994_document",7,', 'entities(database,"en1994_document",8,', marker='"en1994_document",8,')
    sub_once(r, r"(fatigue_detail:reconstruct_text\(control,d\.text\(6\)\?\)\?,)", r"\1reliability_class:read_reliability_class(d.text(7)?)?,", marker="reliability_class:read_reliability_class")
    t = Q + "/🟦️.ts"
    sub_once(t, r'"annex"\|"beams"\|"columns"', '"annex"|"reliabilityClass"|"beams"|"columns"', marker='"annex"|"reliabilityClass"')
    sub_once(t, r'&\{annex:"En"\|"De";beams:', '&{annex:"En"|"De";reliabilityClass:"RC1"|"RC2"|"RC3";beams:', marker='reliabilityClass:"RC1"')
    clone_lines(t, r"^function annex\([^\n]*\n", lambda s: 'function reliabilityClass(value:string):"RC1"|"RC2"|"RC3"{if(value!=="RC1"&&value!=="RC2"&&value!=="RC3")throw Error("EN1994 unsupported reliability class");return value}\n', 1, marker="function reliabilityClass(")
    sub_once(t, r"snapshot\.insulationThicknessM,snapshot\.fatigueDetail\],DOCUMENT", "snapshot.insulationThicknessM,snapshot.fatigueDetail,reliabilityClass(snapshot.reliabilityClass)],DOCUMENT", marker="reliabilityClass(snapshot.reliabilityClass)")
    sub_once(t, r"(fatigueDetail:artifactSqliteText\(d,6\),)", r"\1reliabilityClass:reliabilityClass(artifactSqliteText(d,7)),", marker="reliabilityClass:reliabilityClass(")
    y = Q + "/🧪️tests/🟦️.ts"
    sub_once(y, r'annex:"En",beams:fixture\.beams', 'annex:"En",reliabilityClass:fixture.reliabilityClass as En1994SqliteSnapshot["reliabilityClass"],beams:fixture.beams', marker="reliabilityClass:fixture.reliabilityClass")
    sub_once(y, r'"UPDATE en1994_document SET annex=\'unknown\'",', '"UPDATE en1994_document SET annex=\'unknown\'","UPDATE en1994_document SET reliability_class=\'unknown\'",', marker="SET reliability_class='unknown'")
    contract_update(Q + "/🧪️tests/🧫️fixtures/🎛️semantic.json", Q + "/🗄️.sql", 3, [y])
    x = Q + "/🧪️tests/🦀️.rs"
    sub_once(x, r'\("en1994_document",0,1,SqliteValue::Text\("unknown"\.into\(\)\)\),', '("en1994_document",0,1,SqliteValue::Text("unknown".into())),("en1994_document",0,7,SqliteValue::Text("unknown".into())),', marker='("en1994_document",0,7,')
    a = Q + "/🛂️admission/🦀️.rs"
    sub_once(a, r'Text\(7\),Real\(8\),Text\(9\)\]\)\?,DOCUMENT\)', 'Text(7),Real(8),Text(9),Enumeration(10,&["RC1","RC2","RC3"])])?,DOCUMENT)', marker='Enumeration(10,')
    c = Q + "/🧪️tests/🧫️fixtures/🛂️choices.json"
    sub_once(c, r'("annex":\["En","De"\])', r'\1,"reliabilityClass":["RC1","RC2","RC3"]', marker='"reliabilityClass"')
    fx = Q + "/🧪️tests/🧫️fixtures/🔣️.json"
    sub_once(fx, r'("annex":"En",)', r'\1"reliabilityClass":"RC3",', marker='"reliabilityClass"')

def meta():
    f = R + "/✏️editor/🏷️field-meta/🦀️.rs"
    sub_once(f, r"^const STRUCTURE: ", 'const RC: &[NormFieldChoice] = &[\n    choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"),\n    choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"),\n    choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)"),\n];\nconst STRUCTURE: ', flags=re.M, marker="const RC: ")
    clone_lines(f, r'^    \("annex", mc\([^\n]*\n', lambda s: '    ("reliabilityClass", mc("Reliability class RC", "Zuverlässigkeitsklasse RC", RC)),\n', 1, marker='("reliabilityClass"')
    sub_once(S + "/💡️inferences/🧾outline/🦀️.rs", r'^    "annex",\n', '    "annex",\n    "reliabilityClass",\n', flags=re.M, marker='"reliabilityClass"')
    sub_once(S + "/💡️inferences/🦀️.rs", r'reads: &\["annex", "structureKind"', 'reads: &["annex", "reliabilityClass", "structureKind"', marker='"annex", "reliabilityClass"')
    sub_once(S + "/💡️inferences/🧮️units/🦀️.rs", r'unit\.reads = \["annex", "beams', 'unit.reads = ["annex", "reliabilityClass", "beams', marker='"annex", "reliabilityClass"')

def run(dry):
    rc_lib.DRY = dry
    twins(S)
    model()
    sqlite()
    import glob
    fixtures(R, S, [p.split("/")[-1] for p in glob.glob(R + "/🖼️assets/*")])
    leaf(R, S, "🧩️en1994", "en1994", "En1994")
    meta()
    finish(dry)

if __name__ == "__main__":
    run("--apply" not in sys.argv)
