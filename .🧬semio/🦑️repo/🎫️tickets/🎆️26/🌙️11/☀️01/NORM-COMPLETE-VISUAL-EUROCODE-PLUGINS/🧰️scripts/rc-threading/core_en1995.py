import sys
sys.path.insert(0, ".")
from rc_run import *
NS = "semio_s_plugin_norm_compliance"
R = ART + "/🪵️en1995" + SUBSET
S = R + "/🧬️schema"

def model():
    m = f"{S}/📸️snapshot/🦀️.rs"
    sub_once(m, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};", marker="AnnexChoice, ReliabilityClass}")
    rs_struct_append(m, "En1995Snapshot", "    #[state(artifact)]\n    pub reliability_class: ReliabilityClass,\n")
    clone_lines(m, r"^ +annex: AnnexChoice::\w+,\n", rs_convert, None, marker="reliability_class: ReliabilityClass::RC2,\n")
    sub_once(m, r"Self \{ annex, members: Vec::new\(\),", "Self { annex, reliability_class: ReliabilityClass::RC2, members: Vec::new(),", marker="annex, reliability_class: ReliabilityClass::RC2")
    a = f"{S}/🦀️.rs"
    rs_struct_append(a, "En1995Artifact", f"    #[state(artifact)]\n    pub reliability_class: {NS}::ReliabilityClass,\n")
    sub_once(a, r"En1995Snapshot \{ annex: self\.annex, ", "En1995Snapshot { annex: self.annex, reliability_class: self.reliability_class, ", marker="reliability_class: self.reliability_class")
    sub_once(a, r"Self \{ annex: snapshot\.annex, ", "Self { annex: snapshot.annex, reliability_class: snapshot.reliability_class, ", marker="reliability_class: snapshot.reliability_class")
    d = f"{S}/🔺️diff/🦀️.rs"
    rs_struct_append(d, "En1995Diff", f"    #[state(artifact)]\n    pub reliability_class: Option<{NS}::ReliabilityClass>,\n")
    clone_lines(d, r"^        if other\.annex\.is_some\(\) \{\n            self\.annex = other\.annex;\n        \}\n", rs_convert, 1, marker="other.reliability_class")
    clone_lines(d, r"^            annex: self\.annex\.unwrap_or\(base\.annex\),\n", rs_convert, 1, marker="reliability_class: self.reliability_class.unwrap_or")
    clone_lines(d, r"^            annex: self\.annex\.as_ref\(\)[^\n]*\n", rs_convert, 1, marker="reliability_class: self.reliability_class.as_ref")
    clone_lines(d, r"^        self\.annex\.is_none\(\)\n", lambda s: "            && self.reliability_class.is_none()\n", 1, marker="self.reliability_class.is_none")
    f = f"{S}/⚖️timber/🌲️material-and-annex-factors/🦀️.rs"
    t = rd(f)
    if "pub fn new(" not in t:
        t = t.replace('''    pub fn for_annex(annex: AnnexChoice) -> Self {
        Self { annex, uls_ab: false, factors: semio_s_plugin_norm_compliance::ActionFactors::new(annex, semio_s_plugin_norm_compliance::ReliabilityClass::default(), semio_s_plugin_norm_compliance::DesignSituation::Persistent) }
    }
    /// 🛡️ Selects the reliability class of the project: K_FI of Table B3 scales γ_G,sup and γ_Q of every ULS combination.
    pub fn with_reliability_class(mut self, class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Self {
        self.factors = semio_s_plugin_norm_compliance::ActionFactors::new(self.annex, class, semio_s_plugin_norm_compliance::DesignSituation::Persistent);
        self
    }''', '''    /// 🛡️ The parameters of an annex and the reliability class of the document: K_FI of Table B3 scales γ_G,sup and γ_Q of every ULS combination.
    pub fn new(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Self {
        Self { annex, uls_ab: false, factors: semio_s_plugin_norm_compliance::ActionFactors::new(annex, reliability_class, semio_s_plugin_norm_compliance::DesignSituation::Persistent) }
    }
    /// 🔥 Design charring rate β_n (m/s) of an annex: independent of the reliability class.
    pub fn charring_rate(annex: AnnexChoice, product: TimberProduct) -> f64 {
        semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1995.beta-n.{}", product_key(product)), annex) * 1e-3 / 60.0
    }''')
        t = t.replace("        Self::for_annex(AnnexChoice::En)", "        Self::new(AnnexChoice::En, semio_s_plugin_norm_compliance::ReliabilityClass::RC2)")
        t = t.replace("        Self::for_annex(AnnexChoice::De)", "        Self::new(AnnexChoice::De, semio_s_plugin_norm_compliance::ReliabilityClass::RC2)")
        t = t.replace('''    pub fn beta_n(&self, product: TimberProduct) -> f64 {
        semio_s_plugin_norm_compliance::ndp_number(&format!("ndp.en1995.beta-n.{}", product_key(product)), self.annex) * 1e-3 / 60.0
    }''', '''    pub fn beta_n(&self, product: TimberProduct) -> f64 {
        Self::charring_rate(self.annex, product)
    }''')
        assert "for_annex" not in t
        wr(f, t)
    s = R + "/🎬️scene/🧮️facts/🦀️.rs"
    t = rd(s)
    if "charring_rate" not in t:
        t = t.replace("pub fn deflection(annex: AnnexChoice, m: &TimberMember, props: &TimberProperties) -> Option<Deflection> {\n    let params = t::AnnexParams::for_annex(annex);", "pub fn deflection(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, m: &TimberMember, props: &TimberProperties) -> Option<Deflection> {\n    let params = t::AnnexParams::new(annex, reliability_class);")
        t = t.replace("let beta_n = t::AnnexParams::for_annex(annex).beta_n(product);", "let beta_n = t::AnnexParams::charring_rate(annex, product);")
        t = t.replace("t::AnnexParams::for_annex(document.annex);", "t::AnnexParams::new(document.annex, document.reliability_class);")
        assert "for_annex" not in t
        wr(s, t)
    sub_once(R + "/🎬️scene/📈️diagrams/🧩️special/🦀️.rs", r"t::AnnexParams::for_annex\(d\.annex\)\.beta_n\(", "t::AnnexParams::charring_rate(d.annex, ", marker="charring_rate(d.annex, ")
    sub_once(R + "/🎬️scene/📈️diagrams/🦀️.rs", r"facts::deflection\(d\.annex, m, props\)", "facts::deflection(d.annex, d.reliability_class, m, props)", marker="d.reliability_class, m, props")
    sub_once(R + "/🎬️scene/🧪️tests/🔬️unit/🦀️.rs", r"facts::deflection\(document\.annex, m, &props\)", "facts::deflection(document.annex, document.reliability_class, m, &props)", marker="document.reliability_class, m, &props")
    sub_once(S + "/⚖️timber/🧱️entities/🦀️.rs", r"AnnexParams::for_annex\(document\.annex\)\.with_uls_equation", "AnnexParams::new(document.annex, document.reliability_class).with_uls_equation", marker="AnnexParams::new(document.annex")
    sub_once(S + "/⚖️timber/🏗️structure-and-members/🦀️.rs", r"AnnexParams::for_annex\(annex\)\.with_uls_equation", "AnnexParams::new(annex, document.reliability_class).with_uls_equation", marker="AnnexParams::new(annex")

def sqlite():
    Q = R + "/🚪️io/🪶️sqlite/📸️snapshot"
    sub_once(Q + "/🗄️.sql", r"(  uls_equation TEXT NOT NULL CHECK \(uls_equation IN \('eq610','eq610ab'\)\))\n\);", r"\1,\n  reliability_class TEXT NOT NULL CHECK (reliability_class IN ('RC1','RC2','RC3'))\n);", marker="reliability_class TEXT")
    r = Q + "/🦀️.rs"
    sub_once(r, r"use semio_s_plugin_norm_compliance::AnnexChoice;", "use semio_s_plugin_norm_compliance::{AnnexChoice,ReliabilityClass};", marker="AnnexChoice,ReliabilityClass}")
    clone_lines(r, r"^fn read_annex[^\n]*\n", lambda s: 'fn reliability_class_text(value:ReliabilityClass)-> &\'static str{match value{ReliabilityClass::RC1=>"RC1",ReliabilityClass::RC2=>"RC2",ReliabilityClass::RC3=>"RC3"}}\nfn read_reliability_class(value:&str)->Result<ReliabilityClass, ValueError>{match value{"RC1"=>Ok(ReliabilityClass::RC1),"RC2"=>Ok(ReliabilityClass::RC2),"RC3"=>Ok(ReliabilityClass::RC3),_=>Err(invalid("EN1995 unsupported reliability class"))}}\n', 1, marker="fn reliability_class_text(")
    sub_once(r, r"Cell::Text\(uls_equation_text\(self\.uls_equation\)\)\]\)\?;", "Cell::Text(uls_equation_text(self.uls_equation)),Cell::Text(reliability_class_text(self.reliability_class))])?;", marker="Cell::Text(reliability_class_text(")
    sub_once(r, r'entities\(database,"en1995_document",3,&\[\],control\)', 'entities(database,"en1995_document",4,&[],control)', marker='"en1995_document",4,')
    sub_once(r, r"decks\};control\.checkpoint\(SqliteSnapshotPhase::ReconstructSnapshot,1,1\)\?;Ok\(snapshot\)", "decks,reliability_class:read_reliability_class(document.text(3)?)?};control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(snapshot)", marker="reliability_class:read_reliability_class")
    t = Q + "/🟦️.ts"
    sub_once(t, r'Omit<En1995Snapshot,"annex"\|', 'Omit<En1995Snapshot,"annex"|"reliabilityClass"|', marker='"annex"|"reliabilityClass"')
    sub_once(t, r'&\{annex:"En"\|"De";', '&{annex:"En"|"De";reliabilityClass:"RC1"|"RC2"|"RC3";', marker='reliabilityClass:"RC1"')
    clone_lines(t, r"^function annex\([^\n]*\n", lambda s: 'function reliabilityClass(value:string):En1995SqliteSnapshot["reliabilityClass"]{if(value!=="RC1"&&value!=="RC2"&&value!=="RC3")throw Error("EN1995 unsupported reliability class");return value}\n', 1, marker="function reliabilityClass(")
    sub_once(t, r"\[annex\(snapshot\.annex\),ulsEquation\(snapshot\.ulsEquation\)\]", "[annex(snapshot.annex),ulsEquation(snapshot.ulsEquation),reliabilityClass(snapshot.reliabilityClass)]", marker="reliabilityClass(snapshot.reliabilityClass)")
    sub_once(t, r"entities\(tables\[0\]!,3,\[\],options\)", "entities(tables[0]!,4,[],options)", marker="entities(tables[0]!,4,")
    sub_once(t, r"(ulsEquation:ulsEquation\(artifactSqliteText\(d,2\)\),)", r"\1reliabilityClass:reliabilityClass(artifactSqliteText(d,3)),", marker="reliabilityClass:reliabilityClass(")
    x = Q + "/🧪️tests/🦀️.rs"
    sub_once(x, r'\("annex",true\),', '("annex",true),("reliabilityClass",true),', marker='("reliabilityClass",true)')
    sub_once(x, r'("en1995_document",0,1,SqliteValue::Text\("unknown"\.into\(\)\)\),)', r'\1("en1995_document",0,3,SqliteValue::Text("unknown".into())),', marker='("en1995_document",0,3,')
    y = Q + "/🧪️tests/🟦️.ts"
    sub_once(y, r'annex:f\.annex as En1995SqliteSnapshot\["annex"\],', 'annex:f.annex as En1995SqliteSnapshot["annex"],reliabilityClass:f.reliabilityClass as En1995SqliteSnapshot["reliabilityClass"],', marker="reliabilityClass:f.reliabilityClass")
    sub_once(y, r'"UPDATE en1995_document SET annex=\'unknown\'",', '"UPDATE en1995_document SET annex=\'unknown\'","UPDATE en1995_document SET reliability_class=\'unknown\'",', marker="SET reliability_class='unknown'")
    contract_update(Q + "/🧪️tests/🧫️fixtures/🎛️semantic.json", Q + "/🗄️.sql", 3, [y])
    a = Q + "/🛂️admission/🦀️.rs"
    sub_once(a, r'n::cells\(root,\[Enumeration\(0,&\["En","De"\]\),Text\(3\)\]\)', 'n::cells(root,[Enumeration(0,&["En","De"]),Text(3),Enumeration(15,&["RC1","RC2","RC3"])])', marker='Enumeration(15,')
    c = Q + "/🧪️tests/🧫️fixtures/🛂️choices.json"
    sub_once(c, r'("annex":\["En","De"\],)', r'\1"reliabilityClass":["RC1","RC2","RC3"],', marker='"reliabilityClass"')
    fx = Q + "/🧪️tests/🧫️fixtures/🔣️.json"
    sub_once(fx, r'("annex": "De",)', r'\1\n  "reliabilityClass": "RC3",', marker='"reliabilityClass"')

def meta():
    f = R + "/✏️editor/🏷️field-meta/🦀️.rs"
    sub_once(f, r"^const ULS_EQUATION: ", 'const RC: &[NormFieldChoice] = &[\n    choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"),\n    choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"),\n    choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)"),\n];\nconst ULS_EQUATION: ', flags=re.M, marker="const RC: ")
    clone_lines(f, r'^    \("annex", FieldSpec::choice\([^\n]*\n', lambda s: '    ("reliabilityClass", FieldSpec::choice("Reliability class RC", "Zuverlässigkeitsklasse RC", RC)),\n', 1, marker='("reliabilityClass"')
    sub_once(S + "/⚖️timber/🧮️units/🦀️.rs", r'Rows::new\(\["annex", "ulsEquation"\]\)', 'Rows::new(["annex", "ulsEquation", "reliabilityClass"])', marker='"ulsEquation", "reliabilityClass"')
    sub_once(S + "/💡️inferences/🦀️.rs", r'reads: &\["members", "connections", "annex"\]', 'reads: &["members", "connections", "annex", "reliabilityClass"]', marker='"annex", "reliabilityClass"')
    sub_once(S + "/💡️inferences/🧾outline/🦀️.rs", r'&\["annex", "members"', '&["annex", "reliabilityClass", "members"', marker='"annex", "reliabilityClass"')

def run(dry):
    rc_lib.DRY = dry
    twins(S)
    model()
    sqlite()
    import glob
    fixtures(R, S, [p.split("/")[-1] for p in glob.glob(R + "/🖼️assets/*")])
    leaf(R, S, "🪵️en1995", "en1995", "En1995")
    meta()
    finish(dry)

if __name__ == "__main__":
    run("--apply" not in sys.argv)
