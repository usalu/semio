import sys
sys.path.insert(0, ".")
from rc_run import *
NS = "semio_s_plugin_norm_compliance"
R = ART + "/🔩️en1993" + SUBSET
S = R + "/🧬️schema"

def model():
    m = f"{S}/📸️snapshot/🦀️.rs"
    sub_once(m, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};", marker="AnnexChoice, ReliabilityClass}")
    rs_struct_append(m, "En1993Snapshot", "    #[state(artifact)]\n    pub reliability_class: ReliabilityClass,\n")
    clone_lines(m, r"^ +annex: AnnexChoice::\w+,\n", rs_convert, None, marker="reliability_class: ReliabilityClass::RC2")
    a = f"{S}/🦀️.rs"
    rs_struct_append(a, "En1993Artifact", f"    #[state(artifact)]\n    pub reliability_class: {NS}::ReliabilityClass,\n")
    clone_lines(a, r"^ +annex: (self|snapshot)\.annex,\n", rs_convert, 2, marker="reliability_class: self.reliability_class")
    d = f"{S}/🔺️diff/🦀️.rs"
    rs_struct_append(d, "En1993Diff", f"    #[state(artifact)]\n    pub reliability_class: Option<{NS}::ReliabilityClass>,\n")
    clone_lines(d, r"^        if other\.annex\.is_some\(\) \{\n            self\.annex = other\.annex;\n        \}\n", rs_convert, 1, marker="other.reliability_class")
    clone_lines(d, r"^        if let Some\(value\) = self\.annex \{\n            next\.annex = value;\n        \}\n", rs_convert, 1, marker="next.reliability_class")
    clone_lines(d, r"^            annex: self\.annex\.[^\n]*\n", rs_convert, 1, marker="reliability_class: self.reliability_class")
    sub_once(d, r"(        self\.annex\.is_none\(\)\n)", r"\1            && self.reliability_class.is_none()\n", marker="self.reliability_class.is_none")
    h = f"{S}/💡️inferences/⚖️checks/🧰️structure-helpers/🦀️.rs"
    t = rd(h)
    if "gamma_factors(document)" not in t:
        t = t.replace('''/// 🧮 EN 1990 γ_G / γ_Q / ξ (Eq. 6.10a/b) — STR values identical for EN recommended and DE NA.
fn gamma_factors() -> (f64, f64, f64) {
    let n = semio_s_plugin_norm_compliance::ndp_common;
    (n("ndp.en1990.gG_sup.str.A1"), n("ndp.en1990.gQ.str.A1"), n("ndp.en1990.xi.str.A1"))
}''', '''/// 🧮 EN 1990 γ_G,sup / γ_Q / ξ (Eq. 6.10a/b) of the document's annex and reliability class (K_FI scales γ_G and γ_Q).
fn gamma_factors(document: &En1993Snapshot) -> (f64, f64, f64) {
    let factors = semio_s_plugin_norm_compliance::ActionFactors::new(document.annex, document.reliability_class, semio_s_plugin_norm_compliance::DesignSituation::Persistent);
    (factors.gamma_g_sup(), factors.gamma_q_building(), factors.xi())
}''')
        assert t.count("= gamma_factors();") == 2
        t = t.replace("= gamma_factors();", "= gamma_factors(document);")
        wr(h, t)

def sqlite():
    Q = R + "/🚪️io/🪶️sqlite/📸️snapshot"
    sub_once(Q + "/🗄️.sql", r"(annex TEXT NOT NULL CHECK\(annex IN \('En','De'\)\))\);", r"\1,reliability_class TEXT NOT NULL CHECK(reliability_class IN ('RC1','RC2','RC3')));", marker="reliability_class TEXT")
    r = Q + "/🦀️.rs"
    sub_once(r, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice,ReliabilityClass};", marker="AnnexChoice,ReliabilityClass}")
    clone_lines(r, r"^fn read_annex[^\n]*\n", lambda s: 'fn reliability_class(a:ReliabilityClass)->&\'static str{match a{ReliabilityClass::RC1=>"RC1",ReliabilityClass::RC2=>"RC2",ReliabilityClass::RC3=>"RC3"}}\nfn read_reliability_class(s:&str)->Result<ReliabilityClass, ValueError>{match s{"RC1"=>Ok(ReliabilityClass::RC1),"RC2"=>Ok(ReliabilityClass::RC2),"RC3"=>Ok(ReliabilityClass::RC3),_=>Err(invalid("EN1993 reliability class differs"))}}\n', 1, marker="fn reliability_class(")
    sub_once(r, r"&\[Cell::Text\(annex\(self\.annex\)\)\]\)\?;", "&[Cell::Text(annex(self.annex)),Cell::Text(reliability_class(self.reliability_class))])?;", marker="Cell::Text(reliability_class(")
    sub_once(r, r'entities\(d,"en1993_document",2,&\[\],c\)\?;', 'entities(d,"en1993_document",3,&[],c)?;', marker='"en1993_document",3,')
    sub_once(r, r"(let annex=read_annex\(documents\[&1\]\.text\(1\)\?\)\?;)", r"\1let reliability_class=read_reliability_class(documents[&1].text(2)?)?;", marker="let reliability_class=read_reliability_class")
    sub_once(r, r"frame_bars,placements\}\)", "frame_bars,placements,reliability_class})", marker="placements,reliability_class}")
    t = Q + "/🟦️.ts"
    sub_once(t, r"(placements:En1993SqlitePlacement\[\])\};", r'\1;reliabilityClass:"RC1"|"RC2"|"RC3"};', marker='reliabilityClass:"RC1"')
    clone_lines(t, r"^function annex\([^\n]*\n", lambda s: 'function reliabilityClass(v:string):"RC1"|"RC2"|"RC3"{if(v!=="RC1"&&v!=="RC2"&&v!=="RC3")throw Error("EN1993 reliability class differs");return v}\n', 1, marker="function reliabilityClass(")
    sub_once(t, r'fields\(s,\["annex","materials"', 'fields(s,["annex","reliabilityClass","materials"', marker='"annex","reliabilityClass"')
    sub_once(t, r"(placements:\[\])\};", r"\1,reliabilityClass:reliabilityClass(artifactSqliteText(document,2))};", marker="reliabilityClass:reliabilityClass(")
    x = Q + "/🧪️tests/🟦️.ts"
    sub_once(x, r"frameBars:\[\],placements:\[\]\};const d=await project\(s,\{maxRows:1\}\)", 'frameBars:[],placements:[],reliabilityClass:"RC2"};const d=await project(s,{maxRows:1})', marker='placements:[],reliabilityClass:"RC2"}')
    sub_once(x, r'\["en1993_document",1,"unknown"\],', '["en1993_document",1,"unknown"],["en1993_document",2,"unknown"],', marker='["en1993_document",2,"unknown"]')
    contract_update(Q + "/🧪️tests/🧫️fixtures/🎛️semantic.json", Q + "/🗄️.sql", 3, [x])
    a = Q + "/🛂️admission/🦀️.rs"
    sub_once(a, r'n::cells\(root,\[Enumeration\(0,&\["En","De"\]\)\]\)', 'n::cells(root,[Enumeration(0,&["En","De"]),Enumeration(27,&["RC1","RC2","RC3"])])', marker='Enumeration(27,')
    fx = Q + "/🧪️tests/🧫️fixtures/🔣️.json"
    sub_once(fx, r'("annex": "De",)', r'\1\n    "reliabilityClass": "RC3",', marker='"reliabilityClass"')

def meta():
    f = R + "/✏️editor/🏷️field-meta/🦀️.rs"
    sub_once(f, r"^const ACTION_KIND: ", 'const RC: &[NormFieldChoice] = &[\n    choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"),\n    choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"),\n    choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)"),\n];\nconst ACTION_KIND: ', flags=re.M, marker="const RC: ")
    clone_lines(f, r'^    \("annex", FieldSpec::choice\([^\n]*\n', lambda s: '    ("reliabilityClass", FieldSpec::choice("Reliability class RC", "Zuverlässigkeitsklasse RC", RC)),\n', 1, marker='("reliabilityClass"')
    sub_once(S + "/💡️inferences/🧾outline/🦀️.rs", r'^    "annex",\n', '    "annex",\n    "reliabilityClass",\n', flags=re.M, marker='"reliabilityClass"')
    sub_once(S + "/💡️inferences/🦀️.rs", r'reads: &\["annex", "materials"', 'reads: &["annex", "reliabilityClass", "materials"', marker='"annex", "reliabilityClass"')
    u = S + "/💡️inferences/⚖️checks/🧮️units/🦀️.rs"
    sub_once(u, r'SHARED_READS: \[&str; 5\] = \["annex", ', 'SHARED_READS: [&str; 6] = ["annex", "reliabilityClass", ', marker='"annex", "reliabilityClass"')

def run(dry):
    rc_lib.DRY = dry
    twins(S)
    model()
    sqlite()
    fixtures(R, S, ["✅️heb240-compliant", "🔩️high-strength-connection"])
    leaf(R, S, "🔩️en1993", "en1993", "En1993")
    meta()
    finish(dry)

if __name__ == "__main__":
    run("--apply" not in sys.argv)
