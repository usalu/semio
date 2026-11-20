from pathlib import Path
import json
import re
import subprocess
import tempfile

ticket = Path(__file__).parent
extension = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep")
implementation = (extension / "〰️analytic-curves/🧊️deconstruct/🎯️labels/🦀️.rs").read_text()
rows = json.loads((extension / "🧪️tests/🔬️unit/🔣️deconstruct.json").read_text())["labels"]
parser = implementation[implementation.index("#[derive(Clone, Copy"):implementation.index("#[derive(Default, semio_framework_value::RetireOwned)]\npub(crate) struct LabelSelection")]
parser = parser.replace(", semio_framework_value::RetireOwned", "")
parser = re.sub(r"^semio_framework_value::artifact_retire_leaf!.*\n", "", parser, flags=re.M)
checks = []
for row in rows["accepted"]:
    expected = ",".join(row["unique"])
    checks.append(f'assert_eq!(parse({json.dumps(row["text"])}).unwrap(), vec![{expected}]);')
for row in rows["rejected"]:
    checks.append(f'assert!(parse({json.dumps(row)}).is_err());')
source = (
    "use std::mem::take; type ValueError = &'static str; fn invalid(error: ValueError) -> ValueError { error }\n"
    + parser
    + """
fn parse(input: &str) -> Result<Vec<u64>, ValueError> {
    let mut parser = Parser::default();
    let mut labels = std::collections::BTreeSet::new();
    for byte in input.bytes() {
        if let Some(value) = parser.byte(byte)? { labels.insert(value); }
    }
    if !matches!(parser.token, Token::End) { return Err(bad_label()); }
    Ok(labels.into_iter().collect())
}
"""
    + "fn main() {" + "\n".join(checks) + "}"
)
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(dir=generated) as folder:
    path = Path(folder) / "check.rs"
    artifact = Path(folder) / "check"
    path.write_text(source)
    subprocess.run(["repo","tools","exec","--light","rustup","--","run","nightly-2026-07-20","rustc","--edition=2024",str(path),"-o",str(artifact)], check=True)
    subprocess.run([str(artifact)], check=True)
print(f"{len(checks)} original Rust parser cases passed")
