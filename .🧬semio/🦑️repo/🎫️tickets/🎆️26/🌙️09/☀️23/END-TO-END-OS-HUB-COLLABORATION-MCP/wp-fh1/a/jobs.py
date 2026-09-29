"""💼️ FH1 family A — builtin bounded jobs raise literal codes: the fault helpers take a `FaultCode`, the shared
two-phase machine names its kind (`BuiltinJobKind`) whose refusal codes are literal per kind, never `format!`ed."""
import re, sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.tmp-ticket/wp-fh1")
from fh1_edit import apply, regex, O
J = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/"
M = "🦀️.rs"
KINDS = [("IoRun", "io-run"), ("IoSniff", "io-sniff"), ("Infer", "infer"), ("Migrate", "migrate"), ("MutationPlan", "mutation-plan")]


def table(method, suffix, doc):
    arms = "\n".join(f'            Self::{variant} => semio_framework::FaultCode::new("job.{stem}.{suffix}"),' for variant, stem in KINDS)
    return f'''    /// {doc}
    // 🚫️async: E1 pure code table consumed by the sync fault constructors of the two-phase machine — see R9.
    pub(crate) fn {method}(self) -> semio_framework::FaultCode {{
        match self {{
{arms}
        }}
    }}
'''


labels = "\n".join(f'            Self::{variant} => "job.{stem}",' for variant, stem in KINDS)
kind_enum = f'''/// 🏷️ The builtin bounded job kinds the shared two-phase machine drives; each names the literal refusal codes it
/// raises, so the fault census reads every one (`verify faults`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BuiltinJobKind {{
    IoRun,
    IoSniff,
    Infer,
    Migrate,
    MutationPlan,
}}

impl BuiltinJobKind {{
    /// 🔤️ The kind's name in developer detail.
    // 🚫️async: E1 pure name table — see R9.
    pub(crate) fn label(self) -> &'static str {{
        match self {{
{labels}
        }}
    }}

{table("suspended", "suspended", "⏸️ A bounded step drove a framework call that suspended.")}
{table("cancelled", "cancelled", "🛑️ Cancelled before its next state action.")}
{table("budget_exhausted", "budget-exhausted", "⛽️ Granted fewer work units than its next state action costs.")}
{table("terminal", "terminal", "🏁️ Stepped after its last state action.")}}}
'''

text = open(O + J + M).read()
text, n = re.subn(r'(?<![\w:])fault_bytes\("([^"]+)"', r'fault_bytes(semio_framework::FaultCode::new("\1")', text)
assert n == 4, n
text, n = re.subn(r'(?<![\w:.])fault\("([^"]+)"', r'fault(semio_framework::FaultCode::new("\1")', text)
assert n == 10, n
open(O + J + M, "w").write(text)
for sub, count in [("💡️infer/🦀️.rs", 27), ("🔀️migrate/🦀️.rs", 7), ("🧬️mutation-plan/🦀️.rs", 2)]:
    regex(J, sub, r'super::fault\("([^"]+)"', r'super::fault(semio_framework::FaultCode::new("\1")', count)
edits = [
    (M, '''fn fault(code: &'static str, message: impl Into<String>) -> semio_framework::Fault {
    semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new(code), message.into())
}''', '''fn fault(code: semio_framework::FaultCode, message: impl Into<String>) -> semio_framework::Fault {
    semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin, code, message.into())
}'''),
    (M, '''fn fault_bytes(code: &str, message: String) -> Vec<u8> {''', '''fn fault_bytes(code: semio_framework::FaultCode, message: String) -> Vec<u8> {'''),
    (M, '''fn settle_in_step(fault_prefix: &str, future: impl Future<Output = Result<Vec<u8>, semio_framework::Fault>>) -> Result<Vec<u8>, semio_framework::Fault> {''', '''fn settle_in_step(kind: BuiltinJobKind, future: impl Future<Output = Result<Vec<u8>, semio_framework::Fault>>) -> Result<Vec<u8>, semio_framework::Fault> {'''),
    (M, '''        Poll::Pending => Err(fault(&format!("{fault_prefix}.suspended"), "a bounded job step drove a framework call that suspended; a builtin job kind may only drive calls that complete within one step")),''', '''        Poll::Pending => Err(fault(kind.suspended(), "a bounded job step drove a framework call that suspended; a builtin job kind may only drive calls that complete within one step")),'''),
    (M, '''pub(crate) const PHASE_DECODED: &[u8] = b"phase.decoded";
''', '''pub(crate) const PHASE_DECODED: &[u8] = b"phase.decoded";

''' + kind_enum),
    (M, '''pub(crate) struct TwoPhaseBoundedJob {
    fault_prefix: &'static str,''', '''pub(crate) struct TwoPhaseBoundedJob {
    kind: BuiltinJobKind,'''),
    (M, '''    pub(crate) fn admit(fault_prefix: &'static str, input: &[u8], restored: Option<&[u8]>, decode: BuiltinPhaseFn, execute: BuiltinPhaseFn) -> Self {''', '''    pub(crate) fn admit(kind: BuiltinJobKind, input: &[u8], restored: Option<&[u8]>, decode: BuiltinPhaseFn, execute: BuiltinPhaseFn) -> Self {'''),
    (M, '''        Self { fault_prefix, state, input: input.to_vec(), decode, execute, cancelled: false }''', '''        Self { kind, state, input: input.to_vec(), decode, execute, cancelled: false }'''),
    (M, '''            return JobStep::Failed(fault_bytes(&format!("{}.cancelled", self.fault_prefix), format!("{} was cancelled before its next state action", self.fault_prefix)));''', '''            return JobStep::Failed(fault_bytes(self.kind.cancelled(), format!("{} was cancelled before its next state action", self.kind.label())));'''),
    (M, '''            return JobStep::Failed(fault_bytes(&format!("{}.budget-exhausted", self.fault_prefix), format!("{} needs {price} work units for its next state action and was granted {}", self.fault_prefix, budget.fuel)));''', '''            return JobStep::Failed(fault_bytes(self.kind.budget_exhausted(), format!("{} needs {price} work units for its next state action and was granted {}", self.kind.label(), budget.fuel)));'''),
    (M, '''            TwoPhaseState::Complete => JobStep::Failed(fault_bytes(&format!("{}.terminal", self.fault_prefix), format!("{} has no state action left to advance", self.fault_prefix))),''', '''            TwoPhaseState::Complete => JobStep::Failed(fault_bytes(self.kind.terminal(), format!("{} has no state action left to advance", self.kind.label()))),'''),
    (M, 'TwoPhaseBoundedJob::admit("job.io-run", input,', 'TwoPhaseBoundedJob::admit(BuiltinJobKind::IoRun, input,'),
    (M, 'TwoPhaseBoundedJob::admit("job.io-sniff", input,', 'TwoPhaseBoundedJob::admit(BuiltinJobKind::IoSniff, input,'),
    (M, '    decode_io_hop(input, "job.io-run", JOB_KIND_IO_RUN)', '    decode_io_hop(input, semio_framework::FaultCode::new("job.io-run.decode"), semio_framework::FaultCode::new("job.io-run"), JOB_KIND_IO_RUN)'),
    (M, '    decode_io_hop(input, "job.io-sniff", JOB_KIND_IO_SNIFF)', '    decode_io_hop(input, semio_framework::FaultCode::new("job.io-sniff.decode"), semio_framework::FaultCode::new("job.io-sniff"), JOB_KIND_IO_SNIFF)'),
    (M, '''// 🚫️async: E1 shared pure parse consumed by the two sync phase slots above; `code` is the kind's
// fault-code stem, so a malformed envelope keeps the pre-bounded `<stem>.decode` code and an
// unparseable dialect coordinate keeps the bare `<stem>` code the execute body used to raise.
fn decode_io_hop(input: &[u8], code: &str, kind: &str) -> Result<Vec<u8>, semio_framework::Fault> {
    let decode_code = format!("{code}.decode");
    let input_text = std::str::from_utf8(input).map_err(|_| fault(&decode_code, format!("invalid {kind} input")))?;
    let IoRunInput { source, target, .. } = dsl::os_pack::json::from_json_str::<IoRunInput>(input_text).map_err(|_| fault(&decode_code, format!("invalid {kind} input")))?;
    let source = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&source).map_err(|message| fault(code, message))?;
    let target = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&target).map_err(|message| fault(code, message))?;''', '''// 🚫️async: E1 shared pure parse consumed by the two sync phase slots above; `decode_code` refuses a
// malformed envelope (the pre-bounded `<stem>.decode` code) and `code` an unparseable dialect
// coordinate (the bare `<stem>` code the execute body used to raise).
fn decode_io_hop(input: &[u8], decode_code: semio_framework::FaultCode, code: semio_framework::FaultCode, kind: &str) -> Result<Vec<u8>, semio_framework::Fault> {
    let input_text = std::str::from_utf8(input).map_err(|_| fault(decode_code.clone(), format!("invalid {kind} input")))?;
    let IoRunInput { source, target, .. } = dsl::os_pack::json::from_json_str::<IoRunInput>(input_text).map_err(|_| fault(decode_code, format!("invalid {kind} input")))?;
    let source = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&source).map_err(|message| fault(code.clone(), message))?;
    let target = semio_framework::io_schema::ArtifactDialect::parse_coordinate(&target).map_err(|message| fault(code, message))?;'''),
    (M, '    settle_in_step("job.io-run", run_io_run(input))', '    settle_in_step(BuiltinJobKind::IoRun, run_io_run(input))'),
    (M, '    settle_in_step("job.io-sniff", run_io_sniff(input))', '    settle_in_step(BuiltinJobKind::IoSniff, run_io_sniff(input))'),
    ("💡️infer/🦀️.rs", 'TwoPhaseBoundedJob::admit("job.infer", input,', 'TwoPhaseBoundedJob::admit(super::BuiltinJobKind::Infer, input,'),
    ("💡️infer/🦀️.rs", 'super::settle_in_step("job.infer", ', 'super::settle_in_step(super::BuiltinJobKind::Infer, ', 2),
    ("🔀️migrate/🦀️.rs", 'TwoPhaseBoundedJob::admit("job.migrate", input,', 'TwoPhaseBoundedJob::admit(super::BuiltinJobKind::Migrate, input,'),
    ("🔀️migrate/🦀️.rs", 'super::settle_in_step("job.migrate", ', 'super::settle_in_step(super::BuiltinJobKind::Migrate, ', 2),
    ("🧬️mutation-plan/🦀️.rs", 'TwoPhaseBoundedJob::admit("job.mutation-plan", input,', 'TwoPhaseBoundedJob::admit(super::BuiltinJobKind::MutationPlan, input,'),
    ("🧬️mutation-plan/🦀️.rs", 'super::settle_in_step("job.mutation-plan", ', 'super::settle_in_step(super::BuiltinJobKind::MutationPlan, ', 2),
]
apply(J, edits)
