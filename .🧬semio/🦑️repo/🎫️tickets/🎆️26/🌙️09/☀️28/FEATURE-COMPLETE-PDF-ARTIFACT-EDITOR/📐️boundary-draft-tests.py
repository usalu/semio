"""📐 Exercises the staged binary64 grammar and the actual receipt lanes without publishing source."""

from pathlib import Path
import json
import os
import struct
import subprocess
import sys

TICKET = Path(__file__).resolve().parent
NARROW = os.environ.get("PDF_BOUNDARY_NARROW") == "1"
STAGE = TICKET / "🗑️generated" / ("boundary-draft-narrow" if NARROW else "boundary-draft")
MAP = json.loads((STAGE / "overlay.json").read_text())
FIXTURE = next(STAGE / target for path, target in MAP.items() if path.endswith("/📝️native-page-edit/🔣️.json"))
LAW = json.loads(FIXTURE.read_text())["pageBoundaries"]["decimalDrafts"]


def source(suffix):
    return next((STAGE / target).read_text() for path, target in MAP.items() if path.endswith(suffix))


def rust_text(text):
    return "std::str::from_utf8(&[" + ",".join(map(str, text.encode())) + "]).unwrap()"


def native(pdf):
    wgpu = source("control-commit-rules/🦀️.rs")
    ui = wgpu if NARROW else source("slider-geometry-and-key-handling/🦀️.rs")
    ui = ui[ui.index("/// 📏 A blur-committed"):]
    start = wgpu.index("pub(super) fn commits_on_blur(")
    end = wgpu.index("/// 🔎", start)
    wgpu = wgpu[start:end].replace("pub(super) ", "")
    doc = (pdf / "✏️editor/📄️document/🦀️.rs").read_text()
    parser = doc[doc.index("fn page_box_kind("):doc.index("pub fn set_page_user_unit(")]
    checks = []
    for case in LAW["spellings"]:
        raw = rust_text(case["raw"])
        expected = case["values"]
        if expected is None:
            checks.append(f"assert!(ui_contract::decimal_quad_text({raw}).is_none());")
        else:
            bits = ",".join(hex(int.from_bytes(struct.pack(">d",v),"big")) for v in expected)
            checks.append(f'let quad = ui_contract::decimal_quad_text({raw}).unwrap(); assert_eq!(quad.split_ascii_whitespace().map(|part|part.parse::<f64>().unwrap().to_bits()).collect::<Vec<_>>(), vec![{bits}]);')
    movement = LAW["move"]
    checks += [
        'let node=Node { spec: (UiNode::Input(Field { commit: Some(ui_contract::UI_DECIMAL_QUAD_COMMIT.into()), input_kind: "text".into(), ..Default::default() }),) };',
        'assert!(commits_on_blur(&node.spec.0));',
        f'let (trigger, DslValue::String(accepted)) = edit_commit_action(&node, {rust_text(movement["coordinates"])}).unwrap() else {{ unreachable!() }};',
        'assert_eq!(trigger, Trigger::Commit); assert_eq!(accepted, "150 45.125 175.125 255");',
        'let pdf = PdfSnapshot { pages: vec![()] };',
        'let edits = set_page_boundary(&pdf, 0, "trim", &accepted).unwrap(); let [PdfMutation::SetPageBox(edit)] = edits.as_slice() else { unreachable!() };',
        'assert_eq!(edit.rect, Some([150., 45.125, 175.125, 255.]));',
        'assert!(edit_commit_action(&node, "0 0 NaN 2").is_none());',
        'assert!(set_page_boundary(&pdf, 0, "media", "").is_err());',
        'let edits = set_page_boundary(&pdf, 0, "trim", "").unwrap(); let [PdfMutation::SetPageBox(edit)] = edits.as_slice() else { unreachable!() };',
        'assert_eq!(edit.rect, None);',
        'println!("[DEBUG] Exact native grammar, WGPU blur route, and a single PDF mutation for translated TrimBox passed");'
    ]
    main = STAGE / "probe.rs"
    if NARROW:
        checks = [line.replace("ui_contract::", "") for line in checks]
    support = "\n" + ui + "\n" if NARROW else "\nmod ui_contract {\n" + ui + "\n}\n"
    main.write_text(NATIVE_STUBS + support + wgpu + parser + "\nfn main() {\n" + "\n".join(checks) + "\n}\n")
    binary = STAGE / ("boundary-draft.exe" if os.name == "nt" else "boundary-draft")
    subprocess.run(["repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20", "rustc",
                    "--edition=2021", "-Awarnings", str(main), "-o", str(binary)], check=True)
    subprocess.run([str(binary)], check=True)


def browser():
    ui = source("🧩️component/🟦️.ts")
    ui = ui[ui.index("/** 📏 A blur-committed"):]
    interpreter = next(Path(path).parent.parent for path in MAP if path.endswith("✍️input-views/🟦️.tsx"))
    lane = (interpreter / "🎯️commit-lane/🟦️.ts").resolve()
    ledger = (interpreter.parent / "🏛️ShellHost/🎯️input-ledger/🟦️.ts").resolve()
    script = STAGE / "probe.ts"
    script.write_text(ui + f'''
import assert from "node:assert/strict";
import {{ InputCommitLaneV1 }} from {json.dumps(lane.as_posix(), ensure_ascii=False)};
import {{ inputAppliedV1, inputCommitReceiptV1 }} from {json.dumps(ledger.as_posix(), ensure_ascii=False)};
const law = {json.dumps(LAW)};
''' + BROWSER_TEST)
    subprocess.run(["repo", "tools", "exec", "bun", "--", str(script)], check=True)


NATIVE_STUBS = r'''
#[derive(Default)] struct Field {
    commit: Option<String>, input_kind: String, display_factor: Option<f64>, precision: Option<u16>,
    min: Option<f64>, max: Option<f64>, limits: Option<()>, on_change: (),
}
#[derive(Default)] struct Stepper {
    value: f64, display_factor: Option<f64>, precision: Option<u16>, snaps: Vec<f64>,
    min: Option<f64>, max: Option<f64>, limits: Option<()>, on_absolute: (),
}
enum UiNode { Input(Field), IconSelect(Field), NumberStepper(Stepper), Other }
struct Node { spec: (UiNode,) }
#[derive(Debug, PartialEq)] enum Trigger { Commit, Change }
enum DslValue { String(String), Float(f64) }
impl DslValue { fn float(value: f64) -> Self { Self::Float(value) } }
type FiredAction = (Trigger, DslValue);
fn fired_action(_: &(), trigger: Trigger, value: DslValue) -> Option<FiredAction> { Some((trigger, value)) }
fn typed_number<T>(_: &str, _: Option<f64>, _: Option<u16>, _: T, _: Option<f64>, _: Option<f64>, _: Option<&()>) -> Result<f64, ()> { unreachable!() }
fn current_stored(_: &Field) {}
fn constrain_number_field(_: f64, _: &Field) -> f64 { unreachable!() }
fn constrain_stepper_value(_: f64, _: &Stepper) -> f64 { unreachable!() }
type PdfRect = [f64; 4];
type Fault = String;
struct PdfSnapshot { pages: Vec<()> }
enum PdfPageBox { Crop, Bleed, Trim, Art }
#[derive(Debug)] enum PdfMutation { SetPageBox(SetPageBox), SetPageMediaBox(SetPageMediaBox) }
#[derive(Debug)] struct SetPageMediaBox { index: usize, media_box: PdfRect }
#[derive(Debug)] struct SetPageBox { index: usize, kind: PdfPageBox, rect: Option<PdfRect> }
impl std::fmt::Debug for PdfPageBox { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("page box") } }
fn fault(message: impl Into<String>) -> Fault { message.into() }
fn require_page(snapshot: &PdfSnapshot, page: usize) -> Result<(), Fault> { if snapshot.pages.get(page).is_some() { Ok(()) } else { Err(fault("missing page")) } }
'''

BROWSER_TEST = r'''
assert.equal(law.policy, UI_DECIMAL_QUAD_COMMIT);
assert(uiInputCommitsOnBlur(UI_DECIMAL_QUAD_COMMIT));
for (const row of law.spellings) {
  const value = uiDecimalQuad(row.raw);
  if (row.values === null) assert.equal(value, undefined, JSON.stringify(row.raw));
  else assert.deepEqual(value === "" ? [] : value!.split(" ").map(Number), row.values);
}
const exact = (text: string) => uiDecimalQuad(text) ?? text;
const owner = {active: true, subscribeRetirement: () => () => undefined} as ConstructorParameters<typeof InputCommitLaneV1>[0];
async function commit(request: string, publication: string, revision = "2", name = law.move.object) {
  const entry = { target: law.move.object, value: law.move.before, revision: "1" };
  return new InputCommitLaneV1(owner).submit({
    expected: exact(request),
    read: () => ({ ...entry, value: exact(entry.value) }),
    active: () => true,
    subscribe: () => () => undefined,
    send: async () => {
      entry.value = publication;
      entry.target = name;
      entry.revision = revision;
      return inputAppliedV1(1, inputCommitReceiptV1(1n, 2n));
    },
  });
}
assert.equal((await commit(law.move.coordinates, "150 45.125 175.125 255")).kind, "applied");
assert.equal((await commit("1e21 -0 1.001e21 2", "1000000000000000000000 0 1001000000000000000000 2")).kind, "applied");
assert.equal((await commit(" \t ", "")).kind, "applied");
await assert.rejects(commit(law.move.coordinates, "150 45.125 175.12500000000003 255"), /changed the requested value/);
await assert.rejects(commit(law.move.coordinates, "150 45.125 175.125 255", "3"), /belongs to another mutation/);
await assert.rejects(commit(law.move.coordinates, "150 45.125 175.125 255", "2", "box.crop"), /target changed/);
console.log("[DEBUG] Atomic decimal tuple acknowledgement preserves the exact revision, target, and binary64 value");
'''

if __name__ == "__main__":
    native(Path(sys.argv[1]))
    browser()
