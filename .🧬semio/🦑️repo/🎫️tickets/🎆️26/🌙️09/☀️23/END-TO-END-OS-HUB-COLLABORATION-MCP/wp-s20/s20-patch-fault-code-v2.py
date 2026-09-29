r"""🧯️ S20 T6 prepared set (coordinator decisions 2026-09-29 08:0x/08:3x): an app's refusal crosses the interactive-job
boundary as a TYPED RECORD `semio.typed-operation-fault.v1` {schema, code, origin, message} — schema-first
(`🔌️plugin/🧬️schema/🧯️typed-operation-fault/🔣️.json`), decoded by type on both sides, never a string-packed side channel
(`<code>\u{1f}<message>`) and never the wrapper `interactive-job.app-owned-output` with the domain code flattened into the
text (LW1: remodeling `exportQcReport` answered "…rejected operation: remodeling.qc-report.missing Run the quality check…").

Rust (`semio-framework-plugin`): `app::TypedOperationFault` (`of_fault`, `encode`, `decode`, `into_fault`) is the job fault
detail a job that ran an app reducer hands the framework AND every Fault-lane page (all 8 producers: terminal/job fault,
worker fault, stalled publication, retry exhaustion, cancellation, latest-wins rejection, completion refusal, child
publication); `ArtifactBoundedToolFault` keeps code + origin + message and publishes its record; the agent-lane preview and
the registered fixture answer the record's own code/origin; the `<code>\u{1f}` framing and its separator/page constants are
gone. `retained-command`: `reducer_fault` hands the record; the prose prefix and `reducer_fault_detail/_of_detail` are
removed. Puzzle's retained job no longer DROPS a reducer fault for a fixed sentence.

TypeScript host (`🎭️actor/🖼️wire-turn/🟦️.ts`): `TypedOperationFaultV1` + `decodeTypedOperationFaultV1` (the TS twin) +
`typedOperationPageFaultV1`; `scanTypedOperationPages().faults` and `typedOperationPageAnswerV1().fault` are records;
React `PluginRuntime` routes records and rejects a host call with `TypedOperationFaultError` (carries the record).

Laws: fixture `🧫️fixtures/🧯️typed-operation-fault.json` (Rust `app::typed_operation_fault_tests` + AJV twin
`🧪️tests/🧯️typed-operation-fault/🟦️.ts`, registered in the plugin SDK `test` script + actor vitest decoding every case);
agent-lane preview fixture/Rust law/AJV twin move to records (10 cases); actor + plugin-runtime vitest cases move to
records; remodeling `export_qc_report_without_a_report_is_refused_with_its_domain_code` replaces the silent-success law.
New directories (R10 taxonomy): `🔌️plugin/🧬️schema/🧯️typed-operation-fault/`, `🔌️plugin/🧪️tests/🧯️typed-operation-fault/`.

Usage: python3 s20-patch-fault-code.py [--dry-run] [--root <tree or overlay>]   (idempotent; set-level landing guard; CONFLICT → nothing written)
"""
import hashlib
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
HERE = Path(__file__).resolve().parent / "s20-fault-code-v2"
DRY = "--dry-run" in sys.argv

PLUGIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
SDK = f"{PLUGIN}/🦀️.rs"
RETAINED = f"{PLUGIN}/🧵️retained-command/🦀️.rs"
RETAINED_TESTS = f"{PLUGIN}/🧵️retained-command/🧪️tests/🔬️unit/🦀️.rs"
FULL_OPERATION_TESTS = f"{PLUGIN}/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs"
AGENT_LAW = f"{PLUGIN}/🧪️tests/🤖️agent-lane-preview/🦀️.rs"
AGENT_FIXTURE = f"{PLUGIN}/🧫️fixtures/🤖️agent-lane-preview-verdicts.json"
AGENT_TWIN = f"{PLUGIN}/🧪️tests/🤖️agent-lane-preview/🟦️.ts"
PLUGIN_SCRIPT = f"{PLUGIN}/📦️packages/🦀️rust/📜️script.ts"
WIRE = "🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts"
WIRE_TESTS = "🧰️framework/🔨️modules/🎭️actor/🧪️tests/🗞️typed-operation-page/🟦️.ts"
RUNTIME = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx"
RUNTIME_TESTS = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx"
PUZZLE = "✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs"
REMODELING = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎞️import-frames/🧪️tests/🔬️unit/🦀️.rs"

#: 🏁️ Set-level landing markers `(repo path, text)` — `None` = the set deletes that file. All present → the set is
#: landed and nothing is applied (per-hunk checks alone cannot see an insert whose text a later codemod reworded).
LANDED = [(SDK, "pub struct TypedOperationFault {"), (WIRE, "export function decodeTypedOperationFaultV1("), (f"{PLUGIN}/🧫️fixtures/🧯️typed-operation-fault.json", "semio.typed-operation-fault.v1")]


def landed_guard() -> bool:
    """🏁️ True when every landing marker is in the tree; a partial landing is a conflict, never a second write."""
    present = [(not (ROOT / rel).exists()) if marker is None else ((ROOT / rel).exists() and marker in (ROOT / rel).read_text()) for rel, marker in LANDED]
    if all(present):
        print("landed: every set marker is in the tree — nothing to apply")
        return True
    if any(present):
        raise SystemExit(f"CONFLICT: set partially landed (markers {present}) — nothing written")
    return False


def block(name: str) -> str:
    return (HERE / name).read_text()


#: 🆕️ Files this set creates: (repo path, source in `s20-fault-code/`).
NEW_FILES = [
    (f"{PLUGIN}/🧬️schema/🧯️typed-operation-fault/🔣️.json", "files/schema.json"),
    (f"{PLUGIN}/🧫️fixtures/🧯️typed-operation-fault.json", "files/fixture.json"),
    (f"{PLUGIN}/🧪️tests/🧯️typed-operation-fault/🦀️.rs", "files/law.rs"),
    (f"{PLUGIN}/🧪️tests/🧯️typed-operation-fault/🟦️.ts", "plugin-twin.ts"),
]

#: 🔁️ Whole-file replacements guarded by the preparation-time sha256 of the file they replace.
WHOLE = [
    (AGENT_FIXTURE, "341fc336e9dc0d2672e95319a8acd48f90291bd82144b6e5565415150f890c53", "files/agent-lane-fixture.json"),
    (AGENT_TWIN, "e4766926b5291ea0fe1b1896cfdab31e3b2212b0e1bc7aa534dbc12747a17840", "files/agent-lane-twin.ts"),
]

RECORD = 'schema: "semio.typed-operation-fault.v1"'


def hunks() -> list[tuple[str, str, str]]:
    return [
        (SDK, block("old-fault-region.txt"), block("new-fault-region.txt")),
        (SDK,
         "            let page = match self.terminal_fault.as_ref() {\n                Some(fault) => {\n                    let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];\n                    let len = fault.framed_page_bytes(&mut framed);\n                    TypedOperationResultPage::try_new(self.next_token(), TypedOperationResultLane::Fault, &framed[..len])?\n                }\n                None => {\n                    let lane=operation_progress::cancellation_result_lane(self.user_cancel_requested,false);\n                    let detail:&[u8]=if lane==TypedOperationResultLane::Terminal {b\"\"}else{b\"typed-operation cancelled before its next publication unit\"};\n                    TypedOperationResultPage::try_new(self.next_token(),lane,detail)?\n                }\n            };\n",
         "            let page = match self.terminal_fault.as_ref() {\n                Some(fault) => {\n                    let record = fault.record();\n                    TypedOperationResultPage::try_serialize(self.next_token(), TypedOperationResultLane::Fault, &record)?\n                }\n                None => {\n                    let lane = operation_progress::cancellation_result_lane(self.user_cancel_requested, false);\n                    if lane == TypedOperationResultLane::Terminal {\n                        TypedOperationResultPage::try_new(self.next_token(), lane, b\"\")?\n                    } else {\n                        let record = TypedOperationFault::of_fault(&Fault::new(FaultOrigin::Framework, FaultCode::new(\"interactive-job.cancelled\"), \"typed-operation cancelled before its next publication unit\"));\n                        TypedOperationResultPage::try_serialize(self.next_token(), lane, &record)?\n                    }\n                }\n            };\n"),
        (SDK,
         "            let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];\n            let framed_len = bounded.framed_page_bytes(&mut framed);\n            mounted.stage = MountedTypedCommandFullOperationStage::Publishing;\n            let page = TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, &framed[..framed_len])?;\n",
         "            mounted.stage = MountedTypedCommandFullOperationStage::Publishing;\n            let page = bounded.page(mounted.next_token())?;\n"),
        (SDK,
         "            let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];\n            let framed_len = fault.framed_page_bytes(&mut framed);\n            let page = TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, &framed[..framed_len])?;\n            mounted.terminal_fault = Some(fault);\n",
         "            let page = fault.page(mounted.next_token())?;\n            mounted.terminal_fault = Some(fault);\n"),
        (SDK,
         "                    let bounded = ArtifactBoundedToolFault::from_fault(&fault);\n                    let mut framed = [0; TYPED_OPERATION_FAULT_PAGE_BYTES];\n                    let framed_len = bounded.framed_page_bytes(&mut framed);\n                    let page = TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, &framed[..framed_len])?;\n",
         "                    let bounded = ArtifactBoundedToolFault::from_fault(&fault);\n                    let page = bounded.page(mounted.next_token())?;\n"),
        (SDK,
         '                mounted.queue_page(TypedOperationResultPage::try_new(mounted.next_token(), TypedOperationResultLane::Fault, b"latest-wins command admission cancelled or rejected before worker publication")?)?;\n',
         '                let rejected = ArtifactBoundedToolFault::from_fault(&Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), "latest-wins command admission cancelled or rejected before worker publication"));\n                mounted.queue_page(rejected.page(mounted.next_token())?)?;\n'),
        (SDK,
         "                ArtifactToolCompletionValue::Emit(Err(fault), _) | ArtifactToolCompletionValue::Download(Err(fault), _) => TypedOperationResultPage::try_new(token, TypedOperationResultLane::Fault, fault.as_bytes())?,\n",
         "                ArtifactToolCompletionValue::Emit(Err(fault), _) | ArtifactToolCompletionValue::Download(Err(fault), _) => fault.page(token)?,\n"),
        (SDK,
         "            self.fault.as_ref().map(|fault| ArtifactBoundedToolFault { bytes: fault.bytes, len: fault.len, code: fault.code, code_len: fault.code_len }.into_fault())\n",
         "            self.fault.as_ref().map(|fault| fault.clone().into_fault())\n"),
        (SDK,
         "    /// ([`decode_typed_operation_fault_page`] over the job's bounded detail), except that a retained reducer refusal\n    /// answers the reducer's own code ([`crate::retained_command::reducer_fault_of_detail`]); a cancellation, an\n",
         "    /// (the job's detail read by type, [`ArtifactBoundedToolFault::from_payload`]): a [`TypedOperationFault`] record's own\n    /// code, origin and message, any other detail [`TYPED_OPERATION_UNTYPED_FAULT_CODE`]; a cancellation, an\n"),
        (SDK,
         "                    let (code, message) = decode_typed_operation_fault_page(ArtifactBoundedToolFault::from_payload(&fault.detail).as_bytes());\n                    Some(Err(crate::retained_command::reducer_fault_of_detail(&message).unwrap_or_else(|| Fault::new(FaultOrigin::Framework, code, message))))\n",
         "                    Some(Err(ArtifactBoundedToolFault::from_payload(&fault.detail).into_fault()))\n"),
        (SDK,
         '                        let (code, detail) = crate::app::decode_typed_operation_fault_page(page.bytes());\n                        super::Fault::new(super::FaultOrigin::Plugin, code, format!("registered fixture typed operation fault: {detail}"))\n',
         '                        match crate::app::TypedOperationFault::decode(page.bytes()) {\n                            Some(record) => super::Fault::new(record.origin, super::FaultCode::new(record.code), format!("registered fixture typed operation fault: {}", record.message)),\n                            None => super::Fault::new(super::FaultOrigin::Framework, super::FaultCode::new("interactive-job.fault-page-invalid"), format!("registered fixture fault page carries no typed record: {}", String::from_utf8_lossy(page.bytes()))),\n                        }\n'),
        (FULL_OPERATION_TESTS,
         "            let code = if page.lane == TypedOperationResultLane::Fault { crate::app::decode_typed_operation_fault_page(page.bytes()).0 .0 } else { String::new() };\n",
         '            let code = if page.lane == TypedOperationResultLane::Fault { crate::app::TypedOperationFault::decode(page.bytes()).expect("a fault page is a typed record").code } else { String::new() };\n'),
        (RETAINED,
         "/// 🧯️ The longest reducer fault detail one job fault carries — the app's own code and message, clipped\n/// to a single fault page so a runaway message narrows the report instead of losing it.\nconst ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES: usize = 480;\n",
         ""),
        (RETAINED,
         "    /// 🧯️ Carries the REDUCER's own fault code and message into the job fault instead of replacing it\n    /// with a fixed sentence. The app's `step` is the only party that knows why an operation was\n    /// refused, and dropping its `Fault` made every refusal read `retained command reducer rejected\n    /// operation` — one message for a missing session, an unsupported format and a rejected route\n    /// alike, which is a defect this lane had to reach for a browser probe to diagnose at all\n    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Truncated to the fault stream's own page, so an\n    /// oversized message narrows rather than replacing the report with nothing.\n",
         "    /// 🧯️ Carries the REDUCER's own fault into the job fault as its typed record (`crate::app::TypedOperationFault`):\n    /// the app's `step` is the only party that knows why an operation was refused, and its code, origin and message\n    /// reach the shell, a fixture and an agent by type — never a fixed sentence (ticket 26/09/09/PROCEDURAL-3D-END-TO-END)\n    /// and never a wrapper code with the domain code flattened into the text (ticket 26/09/23, S20 LW1).\n"),
        (RETAINED,
         "        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, reducer_fault_detail(fault).as_bytes()) })\n",
         "        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, &crate::app::TypedOperationFault::of_fault(fault).encode()) })\n"),
        (RETAINED, block("old-retained-helpers.txt"), ""),
        (RETAINED_TESTS, block("old-retained-tests.txt"), ""),
        (PUZZLE,
         "    fn fault(&mut self, cx: &mut StepContext<'_>, message: &'static [u8]) -> StepOutcome {\n        self.phase = PuzzleCommandPhase::Fault;\n        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, message) })\n    }\n",
         "    fn fault(&mut self, cx: &mut StepContext<'_>, message: &'static [u8]) -> StepOutcome {\n        self.phase = PuzzleCommandPhase::Fault;\n        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, message) })\n    }\n\n"
         "    /// 🧯️ A refused reducer step: the app's own fault crosses as its typed record\n"
         "    /// (`semio_framework_plugin::app::TypedOperationFault`), so its code reaches the shell and agents by type.\n"
         "    fn reducer_fault(&mut self, cx: &mut StepContext<'_>, fault: &Fault) -> StepOutcome {\n"
         "        self.phase = PuzzleCommandPhase::Fault;\n"
         "        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, &semio_framework_plugin::app::TypedOperationFault::of_fault(fault).encode()) })\n"
         "    }\n"),
        (PUZZLE,
         '                    Err(_) => self.fault(cx, b"puzzle command reducer rejected the admitted operation"),\n',
         "                    Err(fault) => self.reducer_fault(cx, &fault),\n"),
        (AGENT_LAW,
         '        match (value.as_str(), value.get("fault").and_then(serde_json::Value::as_str)) {\n',
         '        match (value.as_str(), value.get("fault")) {\n'),
        (AGENT_LAW,
         "            (None, Some(detail)) => ScriptedStep::Fault(detail.to_string()),\n",
         "            (None, Some(serde_json::Value::String(detail))) => ScriptedStep::Fault(detail.clone()),\n            (None, Some(record)) if record.is_object() => ScriptedStep::Fault(record.to_string()),\n"),
        (AGENT_LAW,
         '        assert!(cases.len() >= 9, "the fixture keeps every verdict\'s case");\n',
         '        assert!(cases.len() >= 10, "the fixture keeps every verdict\'s case");\n'),
        (AGENT_LAW,
         '                    assert_eq!(refused.code.0, fault["code"].as_str().expect("fault code"), "{name}: {}", refused.message);\n',
         '                    assert_eq!(refused.code.0, fault["code"].as_str().expect("fault code"), "{name}: {}", refused.message);\n                    if let Some(origin) = fault["origin"].as_str() {\n                        assert_eq!(<FaultOrigin as protocol::ToValue>::to_value(&refused.origin), DslValue::String(origin.to_string()), "{name}: origin");\n                    }\n'),
        (PLUGIN_SCRIPT,
         'import { agentLanePreviewVerdictOracle } from "../../🧪️tests/🤖️agent-lane-preview/🟦️.ts";\n',
         'import { agentLanePreviewVerdictOracle } from "../../🧪️tests/🤖️agent-lane-preview/🟦️.ts";\nimport { typedOperationFaultOracle } from "../../🧪️tests/🧯️typed-operation-fault/🟦️.ts";\n'),
        (PLUGIN_SCRIPT,
         "    console.log(`agent-lane-preview-verdict-oracle cases=${agentLanePreviewVerdictOracle()}`);\n",
         "    console.log(`agent-lane-preview-verdict-oracle cases=${agentLanePreviewVerdictOracle()}`);\n    console.log(`typed-operation-fault-oracle cases=${typedOperationFaultOracle()}`);\n"),
        (WIRE,
         "/** 🛤️ The lane a fault result page rides. */\nexport const TYPED_OPERATION_LANE_FAULT = 11;\n",
         "/** 🛤️ The lane a fault result page rides. */\nexport const TYPED_OPERATION_LANE_FAULT = 11;\n" + block("wire-record.ts")),
        (WIRE,
         "  readonly terminal: boolean;\n  readonly faults: readonly string[];\n};\n",
         "  readonly terminal: boolean;\n  readonly faults: readonly TypedOperationFaultV1[];\n};\n"),
        (WIRE,
         "  const faults: string[] = [];\n",
         "  const faults: TypedOperationFaultV1[] = [];\n"),
        (WIRE,
         "    if (page.lane === TYPED_OPERATION_LANE_FAULT) faults.push(new TextDecoder().decode(page.payload));\n",
         "    if (page.lane === TYPED_OPERATION_LANE_FAULT) faults.push(typedOperationPageFaultV1(page));\n"),
        (WIRE,
         "export function typedOperationPageAnswerV1(page: TypedOperationPage): Readonly<{ acknowledgement: TypedOperationAckEvent; refused: boolean; artifact: boolean }> {\n  return { acknowledgement: page.acknowledgement, refused: page.lane === TYPED_OPERATION_LANE_FAULT, artifact: page.lane === TYPED_OPERATION_LANE_ARTIFACT };\n}\n",
         "export function typedOperationPageAnswerV1(page: TypedOperationPage): Readonly<{ acknowledgement: TypedOperationAckEvent; refused: boolean; fault: TypedOperationFaultV1 | null; artifact: boolean }> {\n  const refused = page.lane === TYPED_OPERATION_LANE_FAULT;\n  return { acknowledgement: page.acknowledgement, refused, fault: refused ? typedOperationPageFaultV1(page) : null, artifact: page.lane === TYPED_OPERATION_LANE_ARTIFACT };\n}\n"),
        (WIRE,
         "    {\n      scanTypedOperationPages,\n      shellFrameBytes,\n",
         "    {\n      decodeTypedOperationFaultV1,\n      scanTypedOperationPages,\n      shellFrameBytes,\n"),
        (WIRE_TESTS,
         "  const {\n    scanTypedOperationPages,\n",
         "  const {\n    decodeTypedOperationFaultV1,\n    scanTypedOperationPages,\n"),
        (WIRE_TESTS,
         '    it("names a fault page without choosing a policy for it", () => {\n      const token = fixture.shellMessageStream.token;\n      const scan = scanTypedOperationPages([shellMessage(token.receiver, encodePage(token, fixture.faultTag, "extension.missing"))]);\n      expect(scan.faults).toEqual(["extension.missing"]);\n',
         '    it("names a fault page\'s typed record without choosing a policy for it", () => {\n      const token = fixture.shellMessageStream.token;\n      const record = { schema: "semio.typed-operation-fault.v1", code: "extension.missing", origin: "framework", message: "the extension is not installed" };\n      const scan = scanTypedOperationPages([shellMessage(token.receiver, encodePage(token, fixture.faultTag, JSON.stringify(record)))]);\n      expect(scan.faults).toEqual([record]);\n      const untyped = scanTypedOperationPages([shellMessage(token.receiver, encodePage(token, fixture.faultTag, "extension.missing"))]);\n      expect(untyped.faults.map((fault: { readonly code: string }) => fault.code)).toEqual(["interactive-job.fault-page-invalid"]);\n'),
        (WIRE_TESTS,
         "        expect(answer.refused, lane.name).toBe(lane.tag === fixture.faultTag);\n",
         "        expect(answer.refused, lane.name).toBe(lane.tag === fixture.faultTag);\n        expect(answer.fault === null, lane.name).toBe(lane.tag !== fixture.faultTag);\n"),
        (WIRE_TESTS,
         '    it("answers every declared lane\'s page with its own acknowledgement, and only a fault page refuses the action", () => {\n',
         '    it("decodes a fault page by type exactly as the language-agnostic typed-operation fault fixture states", () => {\n      const faults = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧯️typed-operation-fault.json"), "utf8")) as { readonly decodes: readonly { readonly name: string; readonly bytes: string; readonly record: Readonly<Record<string, string>> | null }[] };\n      for (const { name, bytes, record } of faults.decodes) {\n        expect(decodeTypedOperationFaultV1(new TextEncoder().encode(bytes)), name).toEqual(record === null ? null : { schema: "semio.typed-operation-fault.v1", ...record });\n      }\n    });\n\n    it("answers every declared lane\'s page with its own acknowledgement, and only a fault page refuses the action", () => {\n'),
        (RUNTIME,
         "TYPED_OPERATION_PAGE_MAGIC, typedOperationAcknowledgements as wireTypedOperationAcknowledgements,",
         "TYPED_OPERATION_PAGE_MAGIC, typedOperationAcknowledgements as wireTypedOperationAcknowledgements, typedOperationPageFaultV1, type TypedOperationFaultV1,"),
        (RUNTIME,
         "/** 🧭️ One host call's typed-operation ownership scope",
         "/** 🧯️ A host call rejected with the typed record its operation's Fault-lane page carried — the refusal's own frozen\n * code, origin and message (`TypedOperationFaultV1`), never flattened into the error text alone. */\nexport class TypedOperationFaultError extends Error {\n  constructor(readonly fault: TypedOperationFaultV1) {\n    super(`typed-operation failed: ${fault.code}: ${fault.message}`);\n    this.name = \"TypedOperationFaultError\";\n  }\n}\n\n/** 🧭️ One host call's typed-operation ownership scope"),
        (RUNTIME,
         "  fault(operation: bigint, sequence: number, message: string): boolean { return this.router.fault(this, operation, sequence, message); }\n  takeFault(): string | null { return this.router.takeFault(this); }\n",
         "  fault(operation: bigint, sequence: number, fault: TypedOperationFaultV1): boolean { return this.router.fault(this, operation, sequence, fault); }\n  takeFault(): TypedOperationFaultV1 | null { return this.router.takeFault(this); }\n"),
        (RUNTIME,
         "  private readonly parked: { readonly call: TypedOperationCall; readonly operation: bigint; readonly message: string }[] = [];\n",
         "  private readonly parked: { readonly call: TypedOperationCall; readonly operation: bigint; readonly fault: TypedOperationFaultV1 }[] = [];\n"),
        (RUNTIME,
         "  fault(call: TypedOperationCall, operation: bigint, sequence: number, message: string): boolean {\n    if (this.observe(call, operation, sequence)) return true;\n    const owner = this.owners.get(operation);\n    if (!owner || !this.live.has(owner)) {\n      this.report(TYPED_OPERATION_UNATTRIBUTED_FAULT, `${message} (operation ${operation} sequence ${sequence} observed by ${call.label})`);\n      return false;\n    }\n    if (this.parked.length >= TYPED_OPERATION_PARK_CAPACITY) {\n      const evicted = this.parked.shift()!;\n      this.report(TYPED_OPERATION_PARK_EVICTION_FAULT, `${evicted.message} (operation ${evicted.operation} parked for ${evicted.call.label} evicted at capacity ${TYPED_OPERATION_PARK_CAPACITY})`);\n    }\n    this.parked.push({ call: owner, operation, message });\n    return false;\n  }\n  takeFault(call: TypedOperationCall): string | null {\n    const index = this.parked.findIndex((entry) => entry.call === call);\n    return index === -1 ? null : this.parked.splice(index, 1)[0]!.message;\n  }\n",
         "  fault(call: TypedOperationCall, operation: bigint, sequence: number, fault: TypedOperationFaultV1): boolean {\n    if (this.observe(call, operation, sequence)) return true;\n    const owner = this.owners.get(operation);\n    if (!owner || !this.live.has(owner)) {\n      this.report(TYPED_OPERATION_UNATTRIBUTED_FAULT, `${fault.code}: ${fault.message} (operation ${operation} sequence ${sequence} observed by ${call.label})`);\n      return false;\n    }\n    if (this.parked.length >= TYPED_OPERATION_PARK_CAPACITY) {\n      const evicted = this.parked.shift()!;\n      this.report(TYPED_OPERATION_PARK_EVICTION_FAULT, `${evicted.fault.code}: ${evicted.fault.message} (operation ${evicted.operation} parked for ${evicted.call.label} evicted at capacity ${TYPED_OPERATION_PARK_CAPACITY})`);\n    }\n    this.parked.push({ call: owner, operation, fault });\n    return false;\n  }\n  takeFault(call: TypedOperationCall): TypedOperationFaultV1 | null {\n    const index = this.parked.findIndex((entry) => entry.call === call);\n    return index === -1 ? null : this.parked.splice(index, 1)[0]!.fault;\n  }\n"),
        (RUNTIME,
         "      this.report(TYPED_OPERATION_UNATTRIBUTED_FAULT, `${dropped.message} (operation ${dropped.operation} outlived ${call.label})`);\n",
         "      this.report(TYPED_OPERATION_UNATTRIBUTED_FAULT, `${dropped.fault.code}: ${dropped.fault.message} (operation ${dropped.operation} outlived ${call.label})`);\n"),
        (RUNTIME,
         "    if (fault !== null) throw new Error(`typed-operation failed: ${fault}`);\n",
         "    if (fault !== null) throw new TypedOperationFaultError(fault);\n"),
        (RUNTIME,
         "    if (page.lane === TYPED_OPERATION_LANE_FAULT) {\n      const message = new TextDecoder().decode(page.payload);\n      if (!call || call.fault(page.token.operation, page.token.sequence, message)) throw new Error(`typed-operation failed: ${message}`);\n      continue;\n    }\n",
         "    if (page.lane === TYPED_OPERATION_LANE_FAULT) {\n      const fault = typedOperationPageFaultV1(page);\n      if (!call || call.fault(page.token.operation, page.token.sequence, fault)) throw new TypedOperationFaultError(fault);\n      continue;\n    }\n"),
        (RUNTIME_TESTS,
         "        const faulted = { uiPatches: [], effects: [page(routing.foreignOperation, routing.faultSequence, 11, routing.fault)], nextWake: null, status: { tag: \"idle\" } };\n",
         "        const faulted = { uiPatches: [], effects: [page(routing.foreignOperation, routing.faultSequence, 11, JSON.stringify({ " + RECORD + ", code: \"test.routed-fault\", origin: \"app\", message: routing.fault }))], nextWake: null, status: { tag: \"idle\" } };\n"),
        (RUNTIME_TESTS,
         "        const total = routing.capacity + routing.overflow;\n        for (let index = 0; index < total; index += 1) {\n          const operation = BigInt(routing.ownedOperation) + BigInt(index);\n          expect(router.observe(owner, operation, routing.firstSequence)).toBe(true);\n          expect(router.fault(observer, operation, routing.faultSequence, `${routing.fault} ${index}`)).toBe(false);\n        }\n",
         "        const total = routing.capacity + routing.overflow;\n        const record = (message: string) => ({ " + RECORD + ", code: \"test.parked-fault\", origin: \"app\", message });\n        for (let index = 0; index < total; index += 1) {\n          const operation = BigInt(routing.ownedOperation) + BigInt(index);\n          expect(router.observe(owner, operation, routing.firstSequence)).toBe(true);\n          expect(router.fault(observer, operation, routing.faultSequence, record(`${routing.fault} ${index}`))).toBe(false);\n        }\n"),
        (RUNTIME_TESTS,
         "        for (let fault = owner.takeFault(); fault !== null; fault = owner.takeFault()) drained.push(fault);\n",
         "        for (let parked = owner.takeFault(); parked !== null; parked = owner.takeFault()) drained.push(parked.message);\n"),
        (RUNTIME_TESTS,
         "        expect(router.fault(observer, unowned, routing.faultSequence, routing.fault)).toBe(false);\n        expect(reported.at(-1)).toEqual({ code: routing.unattributedCode, message: `${routing.fault} (operation ${unowned} sequence ${routing.faultSequence} observed by observer)` });\n",
         "        expect(router.fault(observer, unowned, routing.faultSequence, record(routing.fault))).toBe(false);\n        expect(reported.at(-1)).toEqual({ code: routing.unattributedCode, message: `test.parked-fault: ${routing.fault} (operation ${unowned} sequence ${routing.faultSequence} observed by observer)` });\n"),
        (REMODELING, block("old-remodeling-law.txt"), block("new-remodeling-law.txt")),
    ]


def main() -> None:
    if landed_guard():
        return
    texts: dict[str, str] = {}
    notes: list[str] = []
    for rel, source in NEW_FILES:
        wanted = block(source)
        path = ROOT / rel
        if path.exists() and path.read_text() == wanted:
            notes.append(f"applied   new {rel.split('/')[-2]}/{rel.split('/')[-1]}")
        elif path.exists():
            notes.append(f"CONFLICT  new {rel}: exists with other content")
        else:
            texts[rel] = wanted
            notes.append(f"create    {rel.split('/')[-2]}/{rel.split('/')[-1]}")
    for rel, digest, source in WHOLE:
        current = (ROOT / rel).read_text()
        wanted = block(source)
        if current == wanted:
            notes.append(f"applied   whole {rel.split('/')[-1]}")
        elif hashlib.sha256(current.encode()).hexdigest() == digest:
            texts[rel] = wanted
            notes.append(f"apply     whole {rel.split('/')[-2]}/{rel.split('/')[-1]}")
        else:
            notes.append(f"CONFLICT  whole {rel}: content moved since preparation")
    for rel, before, after in hunks():
        text = texts.setdefault(rel, (ROOT / rel).read_text())
        head = before.strip().splitlines()[0][:80]
        if (after != "" and text.count(after) == 1) or (after == "" and before not in text):
            notes.append(f"applied   {rel.split('/')[-2]}: {head}")
        elif text.count(before) == 1:
            texts[rel] = text.replace(before, after)
            notes.append(f"apply     {rel.split('/')[-2]}: {head}")
        else:
            notes.append(f"CONFLICT  {rel}: anchor found {text.count(before)}× — {head}")
    print("\n".join(notes))
    if any(line.startswith("CONFLICT") for line in notes):
        raise SystemExit("conflict: nothing written")
    changed = [rel for rel, text in texts.items() if not (ROOT / rel).exists() or text != (ROOT / rel).read_text()]
    if not DRY:
        for rel in changed:
            (ROOT / rel).parent.mkdir(parents=True, exist_ok=True)
            (ROOT / rel).write_text(texts[rel])
    print(f"{'dry-run' if DRY else 'applied'}: {len(changed)} files {'would change' if DRY else 'changed'}")


if __name__ == "__main__":
    main()
