import sys
p = sys.argv[1]
s = open(p, encoding="utf-8").read()
def rep(a, b):
    global s
    assert a in s, a[:80]
    s = s.replace(a, b, 1)
rep('''fn snapshot_at(path: &str) -> crate::ModelSnapshot {
    let text = std::fs::read_to_string(format!("{FIXTURES}/{path}/🔣️.json")).unwrap_or_else(|error| panic!("{path}: {error}"));
    semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{path} decodes: {error:?}"))
}''', '''const ASSETS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets");

fn decoded(file: &str) -> crate::ModelSnapshot {
    let text = std::fs::read_to_string(file).unwrap_or_else(|error| panic!("{file}: {error}"));
    semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{file} decodes: {error:?}"))
}

fn snapshot_at(path: &str) -> crate::ModelSnapshot {
    decoded(&format!("{FIXTURES}/{path}/🔣️.json"))
}''')
rep('''        ("🧗️wall-depth/🧗️wall-depth.ifc", snapshot_at("💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot")),
''', '''        ("🧗️wall-depth/🧗️wall-depth.ifc", snapshot_at("💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot")),
        ("🏡️example-house/🏡️example-house.ifc", decoded(&format!("{ASSETS}/🏡️house/📸️snapshot.json"))),
        ("🏢️example-office/🏢️example-office.ifc", decoded(&format!("{ASSETS}/🏢️office/📸️snapshot.json"))),
''')
open(p, "w", encoding="utf-8").write(s)
print("ok")
