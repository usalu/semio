import sys
sys.path.insert(0, ".")
from rc_run import *
NS = "semio_s_plugin_norm_compliance"
R = ART + "/🫨️en1998" + SUBSET
S = R + "/🧬️schema"

def model():
    m = f"{S}/📸️snapshot/🦀️.rs"
    t = rd(m)
    print([l for l in t.split("\n") if "use semio_s_plugin_norm_compliance" in l][:3])
    if "ReliabilityClass" not in t.split("pub struct En1998Snapshot")[0]:
        t = t.replace("#![allow(async_fn_in_trait)]", "#![allow(async_fn_in_trait)]", 1)
    rs_struct_append(m, "En1998Snapshot", f"    #[state(artifact)]\n    pub reliability_class: {NS}::ReliabilityClass,\n")
    clone_lines(m, r'^ +annex: "[a-z]+"\.into\(\),\n', lambda s: f"            reliability_class: {NS}::ReliabilityClass::RC2,\n", None, marker="reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass::RC2")
    a = f"{S}/🦀️.rs"
    rs_struct_append(a, "En1998Artifact", f"    #[state(artifact)]\n    pub reliability_class: {NS}::ReliabilityClass,\n")
    clone_lines(a, r"^ +annex: (self\.annex\.clone\(\)|snapshot\.annex),\n", lambda s: "            reliability_class: " + ("self" if "self" in s else "snapshot") + ".reliability_class,\n", 2, marker="reliability_class: self.reliability_class")
    d = f"{S}/🔺️diff/🦀️.rs"
    rs_struct_append(d, "En1998Diff", f"    #[state(artifact)]\n    pub reliability_class: Option<{NS}::ReliabilityClass>,\n")
    clone_lines(d, r"^        if other\.annex\.is_some\(\) \{\n            self\.annex = other\.annex;\n        \}\n", rs_convert, 1, marker="other.reliability_class")
    clone_lines(d, r"^        if let Some\(value\) = &self\.annex \{\n            next\.annex = value\.clone\(\);\n        \}\n", rs_convert, 1, marker="next.reliability_class")
    clone_lines(d, r"^            annex: self\.annex\.as_ref\(\)[^\n]*\n", rs_convert, 1, marker="reliability_class: self.reliability_class.as_ref")
    clone_lines(d, r"^        self\.annex\.is_none\(\)\n", lambda s: "            && self.reliability_class.is_none()\n", 1, marker="self.reliability_class.is_none")

def sqlite():
    Q = R + "/🚪️io/🪶️sqlite/📸️snapshot"
    sub_once(Q + "/🗄️.sql", r"(CREATE TABLE en1998_document \(id INTEGER PRIMARY KEY CHECK\(id=1\),annex TEXT NOT NULL)\);", r"\1,reliability_class TEXT NOT NULL CHECK(reliability_class IN ('RC1','RC2','RC3')));", marker="reliability_class TEXT")
    r = Q + "/🦀️.rs"
    clone_lines(r, r"^fn read_ground[^\n]*\n", lambda s: 'fn reliability_class_text(a:semio_s_plugin_norm_compliance::ReliabilityClass)->&\'static str{match a{semio_s_plugin_norm_compliance::ReliabilityClass::RC1=>"RC1",semio_s_plugin_norm_compliance::ReliabilityClass::RC2=>"RC2",semio_s_plugin_norm_compliance::ReliabilityClass::RC3=>"RC3"}}\nfn read_reliability_class(a:&str)->Result<semio_s_plugin_norm_compliance::ReliabilityClass, ValueError>{match a{"RC1"=>Ok(semio_s_plugin_norm_compliance::ReliabilityClass::RC1),"RC2"=>Ok(semio_s_plugin_norm_compliance::ReliabilityClass::RC2),"RC3"=>Ok(semio_s_plugin_norm_compliance::ReliabilityClass::RC3),_=>Err(invalid("EN1998 reliability class differs"))}}\n', 1, marker="fn reliability_class_text(")
    sub_once(r, r'insert_key_float\("en1998_document",1,&\[Cell::Text\(&self\.annex\)\],&\[\]\)', 'insert_key_float("en1998_document",1,&[Cell::Text(&self.annex),Cell::Text(reliability_class_text(self.reliability_class))],&[])', marker="Cell::Text(reliability_class_text(")
    sub_once(r, r"bound\.add\(2\*16384\)\?;for text in \[&self\.annex,", "bound.add(2*16384)?;bound.repeated(3,24)?;for text in [&self.annex,", marker="bound.repeated(3,24)")
    sub_once(r, r'entities\(database,"en1998_document",2,', 'entities(database,"en1998_document",3,', marker='"en1998_document",3,') if False else sub_once(r, r'entities\(database,"en1998_document",2,', 'entities(database,"en1998_document",3,', marker='"en1998_document",3,')
    sub_once(r, r"Self\{annex:reconstruct_text\(control,d\.text\(1\)\?\)\?,", "Self{annex:reconstruct_text(control,d.text(1)?)?,reliability_class:read_reliability_class(d.text(2)?)?,", marker="reliability_class:read_reliability_class")
    t = Q + "/🟦️.ts"
    clone_lines(t, r"^function ground\([^\n]*\n", lambda s: 'function reliabilityClass(value:string):string{if(value!=="RC1"&&value!=="RC2"&&value!=="RC3")throw Error("EN1998 unsupported reliability class");return value}\n', 1, marker="function reliabilityClass(")
    sub_once(t, r'write\(out,"en1998_document",\[snapshot\.annex\]', 'write(out,"en1998_document",[snapshot.annex,reliabilityClass(snapshot.reliabilityClass)]', marker="reliabilityClass(snapshot.reliabilityClass)")
    sub_once(t, r"documents=await entities\(tables\[0\]!,2,\[\],options\)", "documents=await entities(tables[0]!,3,[],options)", marker="entities(tables[0]!,3,[]")
    sub_once(t, r"(const snapshot:En1998SqliteSnapshot=\{annex:artifactSqliteText\(d,1\),)", r"\1reliabilityClass:reliabilityClass(artifactSqliteText(d,2)),", marker="reliabilityClass:reliabilityClass(")
    fx = Q + "/🧪️tests/🧫️fixtures/🔣️.json"
    sub_once(fx, r'("annex":"[^"]*",)', r'\1"reliabilityClass":"RC3",', marker='"reliabilityClass"')
    y = Q + "/🧪️tests/🟦️.ts"
    return y

def meta():
    f = R + "/🏷️field-meta/🦀️.rs"
    sub_once(f, r"^const ANNEX: ", 'const RC: &[NormFieldChoice] = &[\n    choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"),\n    choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"),\n    choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)"),\n];\nconst ANNEX: ', flags=re.M, marker="const RC: ")
    clone_lines(f, r'^    \("annex", m!\([^\n]*\n', lambda s: '    ("reliabilityClass", m!("Reliability class RC", "Zuverlässigkeitsklasse RC", choices RC)),\n', 1, marker='("reliabilityClass"')
    sub_once(S + "/💡️inferences/🧾outline/🦀️.rs", r'^    "annex",\n', '    "annex",\n    "reliabilityClass",\n', flags=re.M, marker='"reliabilityClass"')
    sub_once(S + "/💡️inferences/🦀️.rs", r'reads: &\["annex", "site"', 'reads: &["annex", "reliabilityClass", "site"', marker='"annex", "reliabilityClass"')

def run(dry):
    rc_lib.DRY = dry
    twins(S)
    model()
    sqlite()
    import glob
    fixtures(R, S, [p.split("/")[-1] for p in glob.glob(R + "/🖼️assets/*")])
    leaf(R, S, "🫨️en1998", "en1998", "En1998")
    meta()
    finish(dry)

if __name__ == "__main__":
    run("--apply" not in sys.argv)
