import sys
sys.path.insert(0, ".")
from rc_lib import *
import rc_lib

def sqlite_en1996(R):
    Q = R + "/🚪️io/🪶️sqlite/📸️snapshot"
    sub_once(Q + "/🗄️.sql", r"(  storeys INTEGER NOT NULL CHECK \(storeys BETWEEN 0 AND 4294967295\))\n\);", r"\1,\n  reliability_class TEXT NOT NULL CHECK (reliability_class IN ('RC1','RC2','RC3'))\n);", marker="reliability_class TEXT")
    r = Q + "/🦀️.rs"
    sub_once(r, r"use semio_s_plugin_norm_compliance::\{AnnexChoice, DesignSituation\};", "use semio_s_plugin_norm_compliance::{AnnexChoice, DesignSituation, ReliabilityClass};", marker="DesignSituation, ReliabilityClass}")
    clone_lines(r, r"^choice!\(annex,[^\n]*\n", lambda s: 'choice!(reliability_class,read_reliability_class,ReliabilityClass,{ReliabilityClass::RC1=>"RC1",ReliabilityClass::RC2=>"RC2",ReliabilityClass::RC3=>"RC3"});\n', 1, marker="choice!(reliability_class")
    sub_once(r, r"Cell::Integer\(i64::from\(self\.storeys\)\)\]\)\?;", "Cell::Integer(i64::from(self.storeys)),Cell::Text(reliability_class(self.reliability_class))])?;", marker="Cell::Text(reliability_class(")
    sub_once(r, r'entities\(database,"en1996_document",5,', 'entities(database,"en1996_document",6,', marker='"en1996_document",6,')
    sub_once(r, r"(storeys:u32::try_from\(document\.integer\(4\)\?\)\.map_err\(\|error\| invalid\(error\.to_string\(\)\)\)\?,)", r"\1reliability_class:read_reliability_class(document.text(5)?)?,", marker="reliability_class:read_reliability_class")
    t = Q + "/🟦️.ts"
    sub_once(t, r'^const ANNEX=\["En","De"\]as const;\n', 'const ANNEX=["En","De"]as const;\nconst RELIABILITY_CLASS=["RC1","RC2","RC3"]as const;\n', flags=re.M, marker="const RELIABILITY_CLASS")
    sub_once(t, r'Omit<En1996Snapshot,"annex"\|', 'Omit<En1996Snapshot,"annex"|"reliabilityClass"|', marker='"annex"|"reliabilityClass"')
    sub_once(t, r"\{annex:typeof ANNEX\[number\];", "{annex:typeof ANNEX[number];reliabilityClass:typeof RELIABILITY_CLASS[number];", marker="reliabilityClass:typeof")
    sub_once(t, r"unsigned\(snapshot\.storeys,4294967295\)\],1n\)", "unsigned(snapshot.storeys,4294967295),choice(snapshot.reliabilityClass,RELIABILITY_CLASS)],1n)", marker="choice(snapshot.reliabilityClass")
    sub_once(t, r"storeys:readUnsigned\(d,4,4294967295n\),", "storeys:readUnsigned(d,4,4294967295n),reliabilityClass:choice(artifactSqliteText(d,5),RELIABILITY_CLASS),", marker="reliabilityClass:choice(")
    x = Q + "/🧪️tests/🦀️.rs"
    sub_once(x, r'\("designSituation",true\),', '("designSituation",true),("reliabilityClass",true),', marker='("reliabilityClass",true)')
    y = Q + "/🧪️tests/🟦️.ts"
    sub_once(y, r'annex:f\.annex as En1996SqliteSnapshot\["annex"\],', 'annex:f.annex as En1996SqliteSnapshot["annex"],reliabilityClass:f.reliabilityClass as En1996SqliteSnapshot["reliabilityClass"],', marker="reliabilityClass:f.reliabilityClass")
    sub_once(y, r'for\(const field of\["annex","masonryClass","designSituation"\]\)expect', 'for(const field of["annex","masonryClass","designSituation","reliabilityClass"])expect', marker='"designSituation","reliabilityClass"])expect')
    sub_once(y, r'for\(const field of\["annex","masonryClass","designSituation"\]as const\)', 'for(const field of["annex","masonryClass","designSituation","reliabilityClass"]as const)', marker='"designSituation","reliabilityClass"]as const')
    sub_once(y, r'"UPDATE en1996_document SET design_situation=\'unknown\'",', '"UPDATE en1996_document SET design_situation=\'unknown\'","UPDATE en1996_document SET reliability_class=\'unknown\'",', marker="SET reliability_class='unknown'")
    contract_update(Q + "/🧪️tests/🧫️fixtures/🎛️semantic.json", Q + "/🗄️.sql", 3, [Q + "/🧪️tests/🟦️.ts"])
    a = Q + "/🛂️admission/🦀️.rs"
    sub_once(a, r'Unsigned\(3,u32::MAX as u64\)\]\)\?\)\?;', 'Unsigned(3,u32::MAX as u64),Enumeration(9,&["RC1","RC2","RC3"])])?)?;', marker='Enumeration(9,&["RC1"')
    c = Q + "/🧪️tests/🧫️fixtures/🛂️choices.json"
    sub_once(c, r'(  "designSituation": \[\n(?:    "[A-Za-z]+",?\n)+  \],\n)', r'\1  "reliabilityClass": [\n    "RC1",\n    "RC2",\n    "RC3"\n  ],\n', marker='"reliabilityClass"')
    f = Q + "/🧪️tests/🧫️fixtures/🔣️.json"
    sub_once(f, r'("designSituation": *"Seismic",)', r'\1 "reliabilityClass": "RC3",', marker='"reliabilityClass"')

def meta(R):
    f = R + "/✏️editor/🏷️field-meta/🦀️.rs"
    sub_once(f, r"^const DESIGN_SITUATION: ", 'const RC: &[NormFieldChoice] = &[choice("RC1", "RC1 (K_FI 0.9)", "RC1 (K_FI 0,9)"), choice("RC2", "RC2 (K_FI 1.0)", "RC2 (K_FI 1,0)"), choice("RC3", "RC3 (K_FI 1.1)", "RC3 (K_FI 1,1)")];\nconst DESIGN_SITUATION: ', flags=re.M, marker="const RC: ")
    clone_lines(f, r'^    \("annex", FieldSpec::choice\([^\n]*\n', lambda s: '    ("reliabilityClass", FieldSpec::choice("Reliability class RC", "Zuverlässigkeitsklasse RC", RC)),\n', 1, marker='("reliabilityClass"')
    o = R + "/🧬️schema/💡️inferences/🧾outline/🦀️.rs"
    sub_once(o, r'&\["annex", "masonryClass"', '&["annex", "reliabilityClass", "masonryClass"', count=1, marker='"annex", "reliabilityClass"')
    i = R + "/🧬️schema/💡️inferences/🦀️.rs"
    sub_once(i, r'reads: &\["annex", "masonryClass"', 'reads: &["annex", "reliabilityClass", "masonryClass"', count=1, marker='"annex", "reliabilityClass"')
    c = R + "/🧬️schema/🧪️tests/⚖️compliance/🦀️.rs"
    sub_once(c, r'"annex", "masonryClass", "designSituation", "storeys",', '"annex", "reliabilityClass", "masonryClass", "designSituation", "storeys",', count=1, marker='"annex", "reliabilityClass"')


def run(dry):
    rc_lib.DRY = dry
    R = ART + "/🪨️en1996" + SUBSET
    S = R + "/🧬️schema"
    for rel in ["🔣️.json", "📸️snapshot/🔣️.json"]:
        json_clone_property(f"{S}/{rel}", 1, converter=json_rc_enum)
        json_required_after_annex(f"{S}/{rel}", 1)
    json_clone_property(f"{S}/🔣️.diff.json", 1, converter=json_rc_enum)
    show_all = True
    # model
    m = f"{S}/📸️snapshot/🦀️.rs"
    sub_once(m, r"use semio_s_plugin_norm_compliance::\{AnnexChoice, DesignSituation\};", "use semio_s_plugin_norm_compliance::{AnnexChoice, DesignSituation, ReliabilityClass};", marker="DesignSituation, ReliabilityClass}")
    rs_struct_append(m, "En1996Snapshot", "    #[state(artifact)]\n    pub reliability_class: ReliabilityClass,\n")
    clone_lines(m, r"^ +annex: AnnexChoice::\w+,\n", rs_convert, 2, marker="reliability_class: ReliabilityClass::RC2")
    a = f"{S}/🦀️.rs"
    rs_struct_append(a, "En1996Artifact", "    #[state(artifact)]\n    pub reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,\n")
    clone_lines(a, r"^ +annex: (self|snapshot)\.annex,\n", rs_convert, 2, marker="reliability_class: self.reliability_class")
    d = f"{S}/🔺️diff/🦀️.rs"
    rs_struct_append(d, "En1996Diff", "    #[state(artifact)]\n    pub reliability_class: Option<semio_s_plugin_norm_compliance::ReliabilityClass>,\n")
    clone_lines(d, r"^        if other\.annex\.is_some\(\) \{\n            self\.annex = other\.annex;\n        \}\n", rs_convert, 1, marker="other.reliability_class")
    clone_lines(d, r"^            annex: self\.annex\.[^\n]*\n", rs_convert, 2, marker="reliability_class: self.reliability_class")
    clone_lines(d, r"^            && self\.annex\.is_none\(\)\n|^        self\.annex\.is_none\(\)\n", lambda s: "            && self.reliability_class.is_none()\n", 1, marker="self.reliability_class.is_none")
    for rel in ["🔗️.graphql", "📸️snapshot/🔗️.graphql"]:
        graphql_clone(f"{S}/{rel}", 1)
    graphql_clone(f"{S}/🔺️diff/🔗️.graphql", 1)
    for rel in ["🛰️.proto", "📸️snapshot/🛰️.proto", "🔺️diff/🛰️.proto"]:
        proto_clone(f"{S}/{rel}")
    for rel in ["🟦️.ts", "📸️snapshot/🟦️.ts"]:
        ts_clone(f"{S}/{rel}", False)
    ts_clone(f"{S}/🔺️diff/🟦️.ts", True)
    sqlite_en1996(R)
    import json
    sch = json.load(open(f"{S}/📸️snapshot/🔣️.json"))
    dif = json.load(open(f"{S}/🔣️.diff.json"))
    req = (set(sch["required"]) | set()) - {"reliabilityClass"}
    n = fixture_sweep(R, req, set(dif["properties"]) - {"reliabilityClass"})
    print("fixtures", n)
    for a in ["🧱️loadbearing-wall", "❌️multi-fail-masonry"]:
        dsl_asset_insert(f"{R}/🖼️assets/{a}/{a}/🗣️.dsl.semio")
    import rc_leaf
    M = S + "/🧬️mutations"
    ad = rc_leaf.find_annex_dir(M)
    tag = rc_leaf.leaf_sources(R, "en1996", "En1996", M, ad)
    rc_leaf.fixtures(R + "/🧪️tests/🧫️fixtures/🧬️mutations", ad)
    fp = rc_leaf.footprint(R, ad)
    rc_leaf.line_register(fp["line"], rc_leaf.lc)
    for p in fp["mount"]:
        rc_leaf.mount_register(p)
    for p in fp["proto"]:
        rc_leaf.proto_register(p)
    for p in fp["graphql"]:
        rc_leaf.graphql_register(p)
    for p in fp["semio"]:
        rc_leaf.protocol_semio_register(p, tag)
    for p in fp["json"]:
        (rc_leaf.oracles_register if "oracles" in p else rc_leaf.mutations_json_register)(p)
    rc_leaf.taxonomy_register("🪨️en1996", ad)
    meta(R)
    for p in [x for x in CHANGED if "fixtures/" in x]:
        CHANGED.remove(p)
    for p in CHANGED:
        show(p)

if __name__ == "__main__":
    run("--apply" not in sys.argv)
