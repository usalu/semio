r"""🧯️ S20 T6 prepared set (coordinator decision 2026-09-29 08:0x): an app's refusal crosses the interactive-job boundary
with its OWN code structurally — never the wrapper `interactive-job.app-owned-output` with the domain code flattened into
the message (LW1: remodeling `exportQcReport` answered `interactive-job.app-owned-output` / "…rejected operation:
remodeling.qc-report.missing Run the quality check…").

- SDK `app`: `typed_operation_fault_detail(&Fault)` frames a job fault detail as `<code>\u{1f}<message>` (the framing the
  fault page and `decode_typed_operation_fault_page` already use; message clipped on a char boundary, unframeable code →
  unframed); `ArtifactBoundedToolFault::from_payload` splits that framing into its code field, so the terminal fault page a
  shell/fixture reads and the agent-lane preview both answer the app's code (preview origin `App` for a coded refusal).
- retained-command job: `reducer_fault` hands the framed detail; the prose prefix `retained command reducer rejected
  operation: <code> <message>`, `reducer_fault_detail`, `reducer_fault_of_detail` and their byte bound are removed.
- puzzle retained command job: a reducer `Err(fault)` is no longer DROPPED for a fixed sentence — it crosses framed.
- laws: retained-command unit laws (structural code, char-boundary clip, unframeable codes); the language-agnostic
  agent-lane preview fixture + its AJV twin (the prose reducer case becomes the framed reducer case, the unframed case is a
  framework-internal detail); remodeling `export_qc_report_without_a_report_is_refused_with_its_domain_code` (was the
  silent-success `export_qc_report_is_a_no_op_without_a_report`).

Usage: python3 s20-patch-fault-code.py [--dry-run]   (idempotent: a hunk whose result is present is `applied`; CONFLICT →
nothing written)
"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
HERE = Path(__file__).resolve().parent / "s20-fault-code"
DRY = "--dry-run" in sys.argv

SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
RETAINED = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs"
RETAINED_TESTS = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️unit/🦀️.rs"
FIXTURE = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🤖️agent-lane-preview-verdicts.json"
TWIN = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🤖️agent-lane-preview/🟦️.ts"
PUZZLE = "✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs"
REMODELING = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎞️import-frames/🧪️tests/🔬️unit/🦀️.rs"


def block(name: str) -> str:
    return (HERE / name).read_text()


def hunks() -> list[tuple[str, str, str]]:
    return [
        (SDK, "    const TYPED_OPERATION_FAULT_BYTES: usize = 256;\n", "    pub(crate) const TYPED_OPERATION_FAULT_BYTES: usize = 256;\n"),
        (SDK, "    const TYPED_OPERATION_FAULT_SEPARATOR: u8 = 0x1f;\n    const TYPED_OPERATION_FAULT_CODE_BYTES: usize = 64;\n",
         "    pub(crate) const TYPED_OPERATION_FAULT_SEPARATOR: u8 = 0x1f;\n    pub(crate) const TYPED_OPERATION_FAULT_CODE_BYTES: usize = 64;\n"),
        (SDK, '            None => (FaultCode::new("interactive-job.app-owned-output"), String::from_utf8_lossy(bytes).into_owned()),\n        }\n    }\n\n    struct ArtifactBoundedToolFault {\n', block("new-helper.txt")),
        (SDK, block("old-from-payload.txt"), block("new-from-payload.txt")),
        (SDK,
         "    /// ([`decode_typed_operation_fault_page`] over the job's bounded detail), except that a retained reducer refusal\n    /// answers the reducer's own code ([`crate::retained_command::reducer_fault_of_detail`]); a cancellation, an\n",
         "    /// (the job's bounded detail split by [`ArtifactBoundedToolFault::from_payload`]): an app refusal framed by\n    /// [`typed_operation_fault_detail`] answers the app's own code under origin `App`, any other detail the generic\n    /// app-owned output code under origin `Framework`; a cancellation, an\n"),
        (SDK,
         "                    let (code, message) = decode_typed_operation_fault_page(ArtifactBoundedToolFault::from_payload(&fault.detail).as_bytes());\n                    Some(Err(crate::retained_command::reducer_fault_of_detail(&message).unwrap_or_else(|| Fault::new(FaultOrigin::Framework, code, message))))\n",
         "                    let bounded = ArtifactBoundedToolFault::from_payload(&fault.detail);\n                    let origin = if bounded.code_len == 0 { FaultOrigin::Framework } else { FaultOrigin::App };\n                    let answered = bounded.into_fault();\n                    Some(Err(Fault::new(origin, answered.code, answered.message)))\n"),
        (RETAINED,
         "/// 🧯️ The longest reducer fault detail one job fault carries — the app's own code and message, clipped\n/// to a single fault page so a runaway message narrows the report instead of losing it.\nconst ARTIFACT_COMMAND_FAULT_DETAIL_MAXIMUM_BYTES: usize = 480;\n",
         ""),
        (RETAINED,
         "    /// 🧯️ Carries the REDUCER's own fault code and message into the job fault instead of replacing it\n    /// with a fixed sentence. The app's `step` is the only party that knows why an operation was\n    /// refused, and dropping its `Fault` made every refusal read `retained command reducer rejected\n    /// operation` — one message for a missing session, an unsupported format and a rejected route\n    /// alike, which is a defect this lane had to reach for a browser probe to diagnose at all\n    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Truncated to the fault stream's own page, so an\n    /// oversized message narrows rather than replacing the report with nothing.\n",
         "    /// 🧯️ Carries the REDUCER's own fault into the job fault, structurally: the app's `step` is the only party that\n    /// knows why an operation was refused, and its code crosses in the typed-operation fault framing\n    /// (`crate::app::typed_operation_fault_detail`), so the shell, a fixture and an agent all read the app's code —\n    /// never a fixed sentence (ticket 26/09/09/PROCEDURAL-3D-END-TO-END) and never a wrapper code with the domain code\n    /// in its text (ticket 26/09/23, S20 LW1). A runaway message narrows on a char boundary.\n"),
        (RETAINED,
         "        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, reducer_fault_detail(fault).as_bytes()) })\n",
         "        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, &crate::app::typed_operation_fault_detail(fault)) })\n"),
        (RETAINED, block("old-retained-helpers.txt"), ""),
        (RETAINED_TESTS, block("old-retained-tests.txt"), block("new-retained-tests.txt")),
        (PUZZLE,
         "    fn fault(&mut self, cx: &mut StepContext<'_>, message: &'static [u8]) -> StepOutcome {\n        self.phase = PuzzleCommandPhase::Fault;\n        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, message) })\n    }\n",
         "    fn fault(&mut self, cx: &mut StepContext<'_>, message: &'static [u8]) -> StepOutcome {\n        self.phase = PuzzleCommandPhase::Fault;\n        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, message) })\n    }\n\n"
         "    /// 🧯️ A refused reducer step: the app's own fault crosses in the typed-operation fault framing\n"
         "    /// (`semio_framework_plugin::app::typed_operation_fault_detail`), so its code reaches the shell and agents structurally.\n"
         "    fn reducer_fault(&mut self, cx: &mut StepContext<'_>, fault: &Fault) -> StepOutcome {\n"
         "        self.phase = PuzzleCommandPhase::Fault;\n"
         "        StepOutcome::Fault(JobFault { detail: Self::retained_payload(cx, JobPayloadStream::Fault, &semio_framework_plugin::app::typed_operation_fault_detail(fault)) })\n"
         "    }\n"),
        (PUZZLE,
         '                    Err(_) => self.fault(cx, b"puzzle command reducer rejected the admitted operation"),\n',
         "                    Err(fault) => self.reducer_fault(cx, &fault),\n"),
        (FIXTURE,
         "except that a retained reducer refusal (`retained command reducer rejected operation: <code> <message>`) answers the reducer's own code;",
         "and every job that runs an app's reducer frames the app's refusal that way (`typed_operation_fault_detail`), so an app refusal answers the app's own code;"),
        (FIXTURE,
         '{ "name": "a job fault keeps the job\'s own detail under the app-owned output code", "clockMicrosPerRead": 1, "script": ["progress", { "fault": "puzzle command reducer rejected the admitted operation" }], "verdict": { "fault": { "code": "interactive-job.app-owned-output", "message": "puzzle command reducer rejected the admitted operation" } } },',
         '{ "name": "a job fault without a frozen code keeps the job\'s own detail under the app-owned output code", "clockMicrosPerRead": 1, "script": ["progress", { "fault": "puzzle command wire payload is malformed" }], "verdict": { "fault": { "code": "interactive-job.app-owned-output", "message": "puzzle command wire payload is malformed" } } },'),
        (FIXTURE,
         '{ "name": "a retained reducer refusal answers the reducer\'s own code", "clockMicrosPerRead": 1, "script": ["progress", { "fault": "retained command reducer rejected operation: app.command.targets-required patchNodes needs the nodes it renames" }], "verdict": { "fault": { "code": "app.command.targets-required", "message": "patchNodes needs the nodes it renames" } } },',
         '{ "name": "a retained reducer refusal answers the app\'s own code, carried in its own field", "clockMicrosPerRead": 1, "script": ["progress", { "fault": "remodeling.qc-report.missing\\u001fRun the quality check before exporting its report." }], "verdict": { "fault": { "code": "remodeling.qc-report.missing", "message": "Run the quality check before exporting its report." } } },'),
        (TWIN, 'const REDUCER_PREFIX = "retained command reducer rejected operation: ";\n', ""),
        (TWIN, '  reducer: schemaKind({ type: "string", pattern: "^retained command reducer rejected operation: [^ ]+ " }),\n', ""),
        (TWIN,
         "/** 🧯️ What the shell's fault page carries for one job fault detail, then the reducer's own code when the reducer wrote it. */\nfunction refusal(detail: string): Refusal {\n  const at = detail.indexOf(\"\\u001f\");\n  const [code, message] = kinds.framed(detail) ? [detail.slice(0, at), detail.slice(at + 1)] : [\"interactive-job.app-owned-output\", at === 0 ? detail.slice(1) : detail];\n  if (!kinds.reducer(message)) return { code, message };\n  const rest = message.slice(REDUCER_PREFIX.length);\n  const space = rest.indexOf(\" \");\n  return { code: rest.slice(0, space), message: rest.slice(space + 1) };\n}\n",
         "/** 🧯️ What the shell's fault page carries for one job fault detail: a framed detail's own code, else the app-owned output code. */\nfunction refusal(detail: string): Refusal {\n  const at = detail.indexOf(\"\\u001f\");\n  const [code, message] = kinds.framed(detail) ? [detail.slice(0, at), detail.slice(at + 1)] : [\"interactive-job.app-owned-output\", at === 0 ? detail.slice(1) : detail];\n  return { code, message };\n}\n"),
        (REMODELING, block("old-remodeling-law.txt"), block("new-remodeling-law.txt")),
    ]



#: 🏁️ Set-level landing markers `(repo path, text)` — `None` = the set deletes that file. All present → the set is
#: landed and nothing is applied (per-hunk checks alone cannot see an insert whose text a later codemod reworded).
LANDED = [('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs', 'pub fn typed_operation_fault_detail(fault: &Fault) -> Vec<u8> {')]


def landed_guard() -> bool:
    """🏁️ True when every landing marker is in the tree; a partial landing is a conflict, never a second write."""
    tree = Path("/Users/ueli/Documents/semio")
    present = [(not (tree / rel).exists()) if marker is None else ((tree / rel).exists() and marker in (tree / rel).read_text()) for rel, marker in LANDED]
    if all(present):
        print("landed: every set marker is in the tree — nothing to apply")
        return True
    if any(present):
        raise SystemExit(f"CONFLICT: set partially landed (markers {present}) — nothing written")
    return False


def main() -> None:
    if landed_guard():
        return
    texts: dict[str, str] = {}
    notes: list[str] = []
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
    changed = [rel for rel, text in texts.items() if text != (ROOT / rel).read_text()]
    if not DRY:
        for rel in changed:
            (ROOT / rel).write_text(texts[rel])
    print(f"{'dry-run' if DRY else 'applied'}: {len(changed)} files {'would change' if DRY else 'changed'}")


if __name__ == "__main__":
    main()
