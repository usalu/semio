import sys
sys.path.insert(0, ".")
from rc_run import *
NS = "semio_s_plugin_norm_compliance"
R = ART + "/🏛️en1992" + SUBSET
S = R + "/🧬️schema"

def model():
    m = f"{S}/📸️snapshot/🦀️.rs"
    sub_once(m, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};", marker="AnnexChoice, ReliabilityClass}")
    rs_struct_append(m, "En1992Snapshot", "    #[state(artifact)]\n    pub reliability_class: ReliabilityClass,\n")
    clone_lines(m, r"^ +annex: AnnexChoice::\w+,\n", rs_convert, None, marker="reliability_class: ReliabilityClass::RC2")
    t = rd(m)
    t = t.replace("its annex and the reliability class RC2 (K_FI = 1.0) until a project-level class is stored;", "its annex and reliability class (K_FI scales the partial factors);")
    t = t.replace(f"ActionFactors::new(self.annex, {NS}::ReliabilityClass::default(),", f"ActionFactors::new(self.annex, self.reliability_class,")
    wr(m, t)
    a = f"{S}/🦀️.rs"
    rs_struct_append(a, "En1992Artifact", f"    #[state(artifact)]\n    pub reliability_class: {NS}::ReliabilityClass,\n")
    clone_lines(a, r"^ +annex: (self|snapshot)\.annex,\n", rs_convert, 2, marker="reliability_class: self.reliability_class")
    d = f"{S}/🔺️diff/🦀️.rs"
    rs_struct_append(d, "En1992Diff", f"    #[state(artifact)]\n    pub reliability_class: Option<{NS}::ReliabilityClass>,\n")
    clone_lines(d, r"^        if other\.annex\.is_some\(\) \{\n            self\.annex = other\.annex;\n        \}\n", rs_convert, 1, marker="other.reliability_class")
    clone_lines(d, r"^        if let Some\(value\) = &self\.annex \{\n            next\.annex = value\.clone\(\);\n        \}\n", rs_convert, 1, marker="next.reliability_class")
    clone_lines(d, r"^            annex: self\.annex\.[^\n]*\n", rs_convert, 1, marker="reliability_class: self.reliability_class")
    sub_once(d, r"self\.annex\.is_none\(\) && ", "self.annex.is_none() && self.reliability_class.is_none() && ", marker="self.reliability_class.is_none()")

def sqlite():
    Q = R + "/🚪️io/🪶️sqlite/📸️snapshot"
    sub_once(Q + "/🗄️.sql", r"(cement_type TEXT NOT NULL,)(\n design_working_life_years_ieee754_bits)", r"\1 reliability_class TEXT NOT NULL CHECK(reliability_class IN ('RC1','RC2','RC3')),\2", marker="reliability_class TEXT")
    r = Q + "/🦀️.rs"
    sub_once(r, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice,ReliabilityClass};", marker="AnnexChoice,ReliabilityClass}")
    clone_lines(r, r"^fn read_annex[^\n]*\n", lambda s: 'fn reliability_class(a:ReliabilityClass)->&\'static str{match a{ReliabilityClass::RC1=>"RC1",ReliabilityClass::RC2=>"RC2",ReliabilityClass::RC3=>"RC3"}}\nfn read_reliability_class(a:&str)->Result<ReliabilityClass, ValueError>{match a{"RC1"=>Ok(ReliabilityClass::RC1),"RC2"=>Ok(ReliabilityClass::RC2),"RC3"=>Ok(ReliabilityClass::RC3),_=>Err(invalid("EN1992 reliability class differs"))}}\n', 1, marker="fn reliability_class(")
    sub_once(r, r"Cell::Text\(cement_class_text\(s\.cement_type\)\)\],DOCUMENT\)", "Cell::Text(cement_class_text(s.cement_type)),Cell::Text(reliability_class(s.reliability_class))],DOCUMENT)", marker="Cell::Text(reliability_class(")
    sub_once(r, r'entities\(d,"en1992_document",6,', 'entities(d,"en1992_document",7,', marker='"en1992_document",7,')
    sub_once(r, r"placements,cases\};c\.checkpoint", "placements,cases,reliability_class:read_reliability_class(r.text(6)?)?};c.checkpoint", marker="reliability_class:read_reliability_class")
    cl = Q + "/📏️cells/🦀️.rs"
    sub_once(cl, r"use crate::sqlite::snapshot::\{invalid, annex,", "use crate::sqlite::snapshot::{invalid, annex, reliability_class,", marker="annex, reliability_class,")
    sub_once(cl, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice,ReliabilityClass};", marker="AnnexChoice,ReliabilityClass}")
    sub_once(cl, r"Bool,Annex,Member,", "Bool,Annex,Reliability,Member,", marker="Annex,Reliability,")
    sub_once(cl, r'\("en1992_document",10\)', '("en1992_document",11)', marker='("en1992_document",11)')
    sub_once(cl, r"(Annex=>Ok\(annex\(scalar::<AnnexChoice>\(value,n\)\?\)\.len\(\)\),)", r"\1Reliability=>Ok(reliability_class(scalar::<ReliabilityClass>(value,n)?).len()),", marker="Reliability=>")
    sub_once(cl, r"shape\(source,12\)", "shape(source,13)", marker="shape(source,13)")
    sub_once(cl, r"\(3,Float\),\(4,Text\)\],n\)\?\)\?,n\)\?;", "(3,Float),(4,Text),(12,Reliability)],n)?)?,n)?;", marker="(12,Reliability)")
    t = Q + "/🟦️.ts"
    sub_once(t, r"(const CEMENT_CLASSS:)", r'const RELIABILITY_CLASSS=["RC1","RC2","RC3"]as const;\n\1', marker="const RELIABILITY_CLASSS")
    sub_once(t, r"cementType:CementClass;concreteGrades:En1992SqliteConcrete\[\];", "cementType:CementClass;reliabilityClass:typeof RELIABILITY_CLASSS[number];concreteGrades:En1992SqliteConcrete[];", marker="reliabilityClass:typeof")
    sub_once(t, r'"cementType","concreteGrades"', '"cementType","reliabilityClass","concreteGrades"', marker='"reliabilityClass","concreteGrades"')
    sub_once(t, r"s\.deltaCDev,s\.cementType\],DOCUMENT", "s.deltaCDev,s.cementType,choice(s.reliabilityClass,RELIABILITY_CLASSS)],DOCUMENT", marker="choice(s.reliabilityClass")
    sub_once(t, r"entities\(t\[0\]!,6,DOCUMENT,o\)", "entities(t[0]!,7,DOCUMENT,o)", marker="entities(t[0]!,7,DOCUMENT")
    sub_once(t, r"(cementType:choice\(artifactSqliteText\(r,5\),CEMENT_CLASSS\),)", r"\1reliabilityClass:choice(artifactSqliteText(r,6),RELIABILITY_CLASSS),", marker="reliabilityClass:choice(")
    x = Q + "/🧪️tests/🦀️.rs"
    sub_once(x, r"\(6,SqliteValue::Null\),", '(6,SqliteValue::Null),(6,SqliteValue::Text("unknown class".into())),', marker='"unknown class"')
    y = Q + "/🧪️tests/🟦️.ts"
    sub_once(y, r'\["en1992_document",6,null\]\]as const', '["en1992_document",6,null],["en1992_document",6,"unknown"]]as const', marker='["en1992_document",6,"unknown"]')
    fx = Q + "/🧪️tests/🧫️fixtures/🔣️.json"
    sub_once(fx, r'("annex": "De",)', r'\1\n    "reliabilityClass": "RC3",', marker='"reliabilityClass"')
    sub_once(fx, r'("en1992_document",\n +)10\n', r'\g<1>11\n', marker=None)

def meta():
    f = R + "/✏️editor/🏷️field-meta/🦀️.rs"
    sub_once(f, r"^const EXPOSURE: ", 'const RC: &[NormFieldChoice] = &[\n    choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"),\n    choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"),\n    choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)"),\n];\n\nconst EXPOSURE: ', flags=re.M, marker="const RC: ")
    clone_lines(f, r'^    \("annex", m\([^\n]*\n', lambda s: '    ("reliabilityClass", m("Reliability class RC", "Zuverlässigkeitsklasse RC", None, Some(RC))),\n', 1, marker='("reliabilityClass"')
    sub_once(S + "/💡️inferences/🧾outline/🦀️.rs", r'^    "annex",\n', '    "annex",\n    "reliabilityClass",\n', flags=re.M, marker='"reliabilityClass"')
    sub_once(S + "/💡️inferences/🦀️.rs", r'reads: &\["annex","members"', 'reads: &["annex","reliabilityClass","members"', marker='"annex","reliabilityClass"')

def python():
    for p, pat, rep in [
        (R + "/🧬️schema/🧪️tests/⚖️compliance/🐍️.py", r"FACTORS\.ActionFactors\(annex\)", 'FACTORS.ActionFactors(annex, reliability_class=int(str(snap["reliabilityClass"])[-1]))'),
        (R + "/🔮️oracles/🚦️check-outcome/🐍️.py", r"fac = FACTORS\.ActionFactors\(annex\)", 'fac = FACTORS.ActionFactors(annex, reliability_class=int(str(doc["reliabilityClass"])[-1]))'),
    ]:
        sub_once(p, pat, rep, marker="reliabilityClass")

def run(dry):
    rc_lib.DRY = dry
    twins(S)
    model()
    sqlite()
    fixtures(R, S, ["🏢compliant-office-frame", "🧵compliant-prestressed-beam", "💥failing-prestressed-beam", "❌️failing-under-reinforced", "🛢️liquid-retaining-fem-anchor"])
    leaf(R, S, "🏛️en1992", "en1992", "En1992")
    cf = R + "/🧪️tests/🚦️check-outcome-en1992-1/🥒️.feature"
    t = rd(cf)
    t = re.sub(r"^      \| pass-04 \| shared://🧬️mutations/🎯change-reliability-class[^\n]*\n", "", t, flags=re.M)
    wr(cf, t)
    meta()
    finish(dry)

if __name__ == "__main__":
    run("--apply" not in sys.argv)
