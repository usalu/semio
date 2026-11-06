from pathlib import Path

job = Path('🧰️framework/🔨️modules/🧵️job')
protocol = job / '🚦️clock-stride-and-job-protocol/🦀️.rs'
text = protocol.read_text()
start = text.index('//#region 🪜️ClockStride')
end = text.index('//#endregion 🪜️ClockStride', start) + len('//#endregion 🪜️ClockStride')
stride = text[start:end].replace('pub(super) stride: u32', 'stride: u32').replace('pub(super) remaining: u32', 'remaining: u32')
path = job / '🕰️clock-identity-and-budget/🪜️stride/🦀️.rs'
path.parent.mkdir(parents=True, exist_ok=True)
path.write_text('//! 🪜️ Adaptive monotonic clock sampling for bounded interactive turns.\n\n' + stride + '\n')
protocol.write_text(text[:start] + text[end:])
clock = job / '🕰️clock-identity-and-budget/🦀️.rs'
text = clock.read_text()
anchor = 'use std::future::Future;'
assert anchor in text
clock.write_text(text.replace(anchor, '#[path="🪜️stride/🦀️.rs"]\nmod stride;\npub use stride::ClockStride;\n\n' + anchor, 1))
context = job / '⏱️context/🦀️.rs'
text = context.read_text()
old = 'pub use clock_stride_and_job_protocol::{Checkpoint, ClockStride, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, StepOutcome};'
new = 'pub use clock_stride_and_job_protocol::{Checkpoint, CommitCandidate, InteractiveJob, InteractiveJobCloseStep, JobFault, StepOutcome};\npub use semio_framework_job_clock::ClockStride;'
assert old in text
context.write_text(text.replace(old, new, 1))
