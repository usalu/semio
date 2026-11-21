import json
import subprocess
from pathlib import Path


ticket = Path(__file__).resolve().parent
law = json.loads((ticket / "gis-map-json-validation.json").read_text())


def reject_duplicate(pairs):
    if len(dict(pairs)) != len(pairs):
        raise ValueError("duplicate")
    return dict(pairs)


fixture = []
for example in law["objects"]:
    keys = [law["prefix"] + suffix for suffix in example["suffixes"]]
    text = "{" + ",".join(json.dumps(key) + ":null" for key in keys) + "}"
    try:
        json.loads(text, object_pairs_hook=reject_duplicate)
        accepted = True
    except ValueError:
        accepted = False
    assert accepted is example["accept"]
    fixture.append("(&[" + ", ".join(json.dumps(key) for key in keys) + "][..], " + str(accepted).lower() + ")")
print("independent JSON duplicate-key law passed")

prefix = """
mod semio_framework_value {
    #[derive(Clone, Copy)]
    pub enum Number { Float(f64), Int(i64), UInt }
    pub struct ValueError;
    pub enum ValueRefusalKind { InvalidValue }
    impl ValueError {
        pub fn literal(_: ValueRefusalKind, _: &'static str) -> Self { Self }
    }
}
mod semio_framework_pack_json {
    use super::semio_framework_value::{Number, ValueError};
    #[derive(Clone, Copy)]
    pub enum JsonWriteNode { Object(usize), Array(usize), Number(Number), Null }
    pub trait JsonWriteSource {
        fn node_at_path(&self, path: &[usize]) -> Result<JsonWriteNode, ValueError>;
        fn object_key_at_path(&self, path: &[usize], index: usize) -> Result<&str, ValueError>;
    }
}
"""
test = """
use semio_framework_pack_json::{JsonWriteSource,JsonWriteNode};
use semio_framework_value::ValueError;

struct Object<'a>(&'a [&'a str]);
impl JsonWriteSource for Object<'_> {
    fn node_at_path(&self, path: &[usize]) -> Result<JsonWriteNode, ValueError> {
        Ok(if path.is_empty() { JsonWriteNode::Object(self.0.len()) } else { JsonWriteNode::Null })
    }
    fn object_key_at_path(&self, _: &[usize], index: usize) -> Result<&str, ValueError> {
        self.0.get(index).copied().ok_or(ValueError)
    }
}

#[test]
fn original_duplicate_scan_is_exact_across_key_chunks() {
    for (names, accept) in FIXTURE {
        let mut cursor = MapJsonValidation::default();
        let source = Object(names);
        let actual = (0..128).find_map(|_| match cursor.step(&source) {
            Ok(false) => None,
            Ok(true) => Some(true),
            Err(_) => Some(false),
        });
        assert_eq!(actual, Some(accept));
    }
}
""".replace("FIXTURE", "[" + ",".join(fixture) + "]")
body = (ticket / "gis-map-json-validation.rs").read_text().replace("semio_framework_value::artifact_retire_leaf!(MapJsonValidation);", "")
output = ticket / "🗑️generated" / "gis-map-validation-law"
subprocess.run(["repo", "tools", "exec", "rustc", "--", "--edition=2021", "--test", "-", "-o", str(output)],
               input=prefix + "\n" + body + "\n" + test, text=True, check=True)
subprocess.run([str(output)], check=True)
