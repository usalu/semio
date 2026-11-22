"""📬️ Exercises the original ordered text handoff and bounded DWG base64 transport."""
from pathlib import Path
import base64
import json
import subprocess

out = Path(__file__).parent / "🗑️generated"
root = Path("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any")
owner = (root / "🚪️io/📐️geometry/💾️brep-interchange/🦀️.rs").read_text()
inputs = (root / "🧬️schema/💡️inferences/📐️geometry/🔌️inputs/🦀️.rs").read_text()
codec = Path("🧰️framework/🔨️modules/🚪️io/🔤️base64/🦀️.rs").resolve()
fixtures = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📤️export/🧪️tests/🧫️fixtures")
fixture = json.loads((fixtures / "drawing-transport.json").read_text())
expected = bytes((i * fixture["byteMultiplier"]) % 256 for i in range(fixture["byteCount"]))
(out / "dwg-transport-expected.bin").write_bytes(expected)
(out / "dwg-transport-input.txt").write_bytes(fixture["leading"].encode() + base64.b64encode(expected) + fixture["trailing"].encode())
transport = owner[owner.index("fn dwg_input_decoder"):owner.index("//#endregion 🔖️Import")]
move_text = inputs[inputs.index("    pub fn into_text"):inputs.index("    pub fn vector", inputs.index("    pub fn into_text"))]
source = out / "dwg-transport-proof.rs"
source.write_text("#[path = " + json.dumps(str(codec), ensure_ascii=False) + "]\nmod base64;\nuse base64::*;\n" + r"""
use std::collections::BTreeMap;
type WidgetFault = String;
fn decode_fault(error: impl std::fmt::Display) -> WidgetFault { error.to_string() }
fn empty(_: &str) -> WidgetFault { "empty text".into() }
fn cancelled_fault() -> WidgetFault { "cancelled".into() }
struct StepContext<'a> { fuel: &'a mut usize }
impl StepContext<'_> { fn should_yield(&self) -> bool { *self.fuel == 0 } }
fn spend(cx: &mut StepContext<'_>, units: usize) { *cx.fuel -= units; }
struct WidgetInputs { values: BTreeMap<String, GeometryValue> }
enum GeometryValue { Text(String), Other }
impl WidgetInputs {
    fn mismatch(&self, port: &str, _: (&str, &str)) -> String { format!("{port} is not a text") }
    fn missing(&self, port: &str) -> String { format!("{port} is missing") }
""" + move_text + "}\n" + transport + r"""
#[test]
fn the_funded_transport_decodes_exact_original_text_in_multiple_turns() {
    let text = include_str!("dwg-transport-input.txt").to_string();
    let address = text.as_ptr();
    let value = WidgetInputs { values: [("data".into(), GeometryValue::Text(text))].into() };
    let text = value.into_text("data").unwrap();
    assert_eq!(text.as_ptr(), address);
    let mut decoder = dwg_input_decoder(text).unwrap();
    assert!(matches!(decoder.input(), Base64Input::OwnedText(text) if text.as_ptr() == address));
    let expected = include_bytes!("dwg-transport-expected.bin");
    assert_eq!(decoder.expected_output_bytes(), expected.len());
    assert!(step_dwg_transport(&mut decoder, &mut StepContext { fuel: &mut 0 }).unwrap().is_none());
    assert_eq!(decoder.progress().completed, 0);
    let mut turns = 0;
    loop {
        let data = step_dwg_transport(&mut decoder, &mut StepContext { fuel: &mut 1 }).unwrap();
        assert!(decoder.last_turn_receipt().read_bytes <= 4096);
        turns += 1;
        if let Some(bytes) = data { assert_eq!(&bytes, expected); break; }
    }
    assert!(turns > 10);
    assert!(decoder.take_output().is_none());
}

#[test]
fn invalid_and_empty_text_leave_no_payload() {
    assert!(dwg_input_decoder(" \n\t".into()).is_err());
    for invalid in INVALID {
        let mut decoder = dwg_input_decoder(invalid.into()).unwrap();
        let error = loop {
            match step_dwg_transport(&mut decoder, &mut StepContext { fuel: &mut 1 }) {
                Ok(None) => continue,
                Ok(Some(_)) => panic!("invalid input released bytes"),
                Err(error) => break error,
            }
        };
        assert_eq!(step_dwg_transport(&mut decoder, &mut StepContext { fuel: &mut 1 }).unwrap_err(), error);
        assert!(decoder.take_output().is_none());
    }
    let value = WidgetInputs { values: [("data".into(), GeometryValue::Other)].into() };
    assert_eq!(value.into_text("data").unwrap_err(), "data is not a text");
    assert_eq!((WidgetInputs { values: BTreeMap::new() }).into_text("data").unwrap_err(), "data is missing");
}
""".replace("INVALID", json.dumps(fixture["invalid"])))
binary = out / "dwgtransport"
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20",
                "rustc", "--edition=2024", "--test", "-A", "dead_code", "-A", "unused_imports", str(source), "-o", str(binary)], check=True)
subprocess.run(["repo", "tools", "exec", "--light", "python", "--", "-c",
                "import os,sys; os.execv(sys.argv[1], sys.argv[1:])", str(binary)], check=True)
