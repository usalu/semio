"""Regenerate the compact Poll opening patch without touching production sources."""
from difflib import unified_diff
from pathlib import Path
from os.path import relpath

ticket = Path(__file__).parent
plugin = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
wire = plugin / "🧬️schema/🐣️instance-open"
host = plugin / "🖥️host"
guest = plugin / "⚛️reactor/🎟️context/🌉️call/📥️input"
poll = host / "🔁️poll"
session = poll / "🎛️session/🦀️.rs"
unit = host / "🔀️post-turn-relay/🎟️unit/🦀️.rs"
receiver = guest / "🦀️.rs"
changes = {}


def read(path):
    return path.read_text(encoding="utf-8")


def replace(path, before, after):
    if path not in changes:
        old = read(path)
        changes[path] = (old, old)
    old, current = changes[path]
    assert current.count(before) == 1, (path, before[:85], current.count(before))
    changes[path] = old, current.replace(before, after, 1)


replace(plugin / "🚪️io/🦀️.rs", "extern crate semio_os_kernel as protocol;", """#[path = "../🧬️schema/🐣️instance-open/🦀️.rs"]
pub mod original_open_frame;
extern crate semio_os_kernel as protocol;""")
replace(plugin / "🧬️guest-contract/🦀️.rs", "actor_input_wire, job_kinds, operation_authority,", "actor_input_wire, job_kinds, operation_authority, original_open_frame,")
changes[wire / "🦀️.rs"] = "", read(ticket / "instance-open-frame.rs") + """
#[path = "📤️.rs"]
mod source;
pub use source::{OpenSegment, OpenView};
"""
changes[wire / "📤️.rs"] = "", read(ticket / "instance-open-frame-source.rs")
changes[wire / "🧬️schema/🔣️.json"] = "", read(ticket / "instance-open-frame.schema.json")
changes[wire / "🧪️law.json"] = "", read(ticket / "instance-open-frame.law.json")
changes[unit.parent / "🐣️open.rs"] = "", read(ticket / "instance-open-frame-donor.rs")
changes[guest / "🐣️open.rs"] = "", read(ticket / "instance-open-frame-receiver.rs")
open_tests = guest / "🧪️tests/open.rs"
law_path = Path(relpath(wire / "🧪️law.json", open_tests.parent)).as_posix()
changes[open_tests] = "", read(ticket / "instance-open-frame-test.rs").replace("__LAW__",law_path)

replace(unit, "use std::mem::{ManuallyDrop,size_of};", """use std::mem::{ManuallyDrop,size_of};
#[path="🐣️open.rs"]
mod open;""")
replace(unit, "failure_close:Option<ControlledRetirement<OriginalTurnFailure>>,cursor:usize}", "failure_close:Option<ControlledRetirement<OriginalTurnFailure>>,cursor:usize,open_encoding:Option<open::OpenEncoding>}")
replace(unit, "failure_close:None,cursor:0}}", "failure_close:None,cursor:0,open_encoding:None}}")
replace(unit, "if !matches!(self.phase,Phase::Encode|Phase::EncoderClose){", "if self.open_encoding.is_some()||!matches!(self.phase,Phase::Encode|Phase::EncoderClose){")
replace(unit, "if self.phase!=Phase::Encode||!self.encoder.terminal_is_empty(){", "if self.open_encoding.is_some()||self.phase!=Phase::Encode||!self.encoder.terminal_is_empty(){")
replace(unit, "self.phase=Phase::Closing;if !self.encoder.terminal_is_empty()", "self.phase=Phase::Closing;if !self.close_open_encoding(cx)?{return Ok(false)}if !self.encoder.terminal_is_empty()")
replace(unit, "pub fn terminal_is_empty(&self)->bool{self.encoder.terminal_is_empty()", "pub fn terminal_is_empty(&self)->bool{self.open_encoding.is_none()&&self.encoder.terminal_is_empty()")

poll_source = poll / "🦀️.rs"
replace(poll_source, "self.advance_shared(runtime,instance,Some(input),budget,execution,cx)}", """self.advance_shared(runtime,instance,Some(BorrowedPollInput::Json(input)),budget,execution,cx)}
    fn advance_input_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,input:BorrowedPollInput<'_>,budget:Option<OriginalJobBudget>,execution:Option<OriginalGuestExecutionBudget>,cx:&mut StepContext<'_>)->Result<GuestPollStatus,OriginalTurnFailure>{
        self.advance_shared(runtime,instance,Some(input),budget,execution,cx)
    }""")
replace(poll_source, "input: Option<&dyn JsonWriteSource>, budget: Option<OriginalJobBudget>", "input: Option<BorrowedPollInput<'_>>, budget: Option<OriginalJobBudget>")
replace(poll_source, """            Phase::Encode => { if execution.is_some() { return Err(invalid("Poll encoding cannot consume VM execution")); } if self.unit.encode_step(input.ok_or_else(||invalid("Poll encoding requires its actual immutable input"))?, cx)? { self.phase = Phase::Input; } }""", """            Phase::Encode => {
                if execution.is_some(){return Err(invalid("Poll encoding cannot consume VM execution"))}
                let ready=match input.ok_or_else(||invalid("Poll encoding requires its actual immutable input"))? {
                    BorrowedPollInput::Json(input)=>self.unit.encode_step(input,cx)?,
                    BorrowedPollInput::Open(input)=>self.unit.encode_open_frame_step(input,cx)?,
                };
                if ready{self.phase=Phase::Input;}
            }""")
entry = read(poll_source)
mark = next(line for line in entry.splitlines() if line.startswith("use "))
replace(poll_source, mark, """#[derive(Clone,Copy)]
enum BorrowedPollInput<'a> {
    Json(&'a dyn JsonWriteSource),
    Open(semio_plugin_guest_contract::original_open_frame::OpenView<'a>),
}

""" + mark)

replace(session, "operation:(u64,u64),_pinned:std::marker::PhantomPinned}", "operation:(u64,u64),open:Option<semio_plugin_guest_contract::original_open_frame::OpenView<'a>>,_pinned:std::marker::PhantomPinned}")
replace(session, "generation),_pinned:std::marker::PhantomPinned})}", "generation),open:None,_pinned:std::marker::PhantomPinned})}")
replace(session, "  let mut input = self.input(source)?;\n  match page", """  let mut input = self.input(source)?;
  if let ([event],None)=(source.0,page){
   input.open=semio_plugin_guest_contract::original_open_frame::OpenView::select(event,input.limits);
  }
  match page""")
replace(session, "fn advance_native(&mut self,input:Option<&dyn JsonWriteSource>", "fn advance_native(&mut self,input:Option<BorrowedPollInput<'_>>")
replace(session, "fn drive_inner(&mut self,input:Option<&dyn JsonWriteSource>", "fn drive_inner(&mut self,input:Option<BorrowedPollInput<'_>>")
replace(session, "  self.drive_inner(Some(input),false,receive)", """  let input=input.open.map_or_else(||BorrowedPollInput::Json(input),BorrowedPollInput::Open);
  self.drive_inner(Some(input),false,receive)""")
for call in ["poll.advance_step(runtime,instance,input,budget,Some(execution),cx)", "poll.advance_step(runtime,instance,input,budget,None,cx)"]:
    replace(session, call, call.replace(".advance_step", ".advance_input_step"))

replace(receiver, "mod job_frame;", """mod job_frame;
#[path="🐣️open.rs"]
mod open_frame;
#[cfg(test)]
#[path="🧪️tests/open.rs"]
mod open_tests;""")
replace(guest / "🧪️tests/🦀️.rs", "fn with_input_decode_stack(check: fn())", "pub(super) fn with_input_decode_stack(check: fn())")
replace(receiver, "    job_frame:Option<job_frame::JobFrame>,", """    job_frame:Option<job_frame::JobFrame>,
    open_frame:Option<open_frame::OpenInput>,
    #[cfg(test)]
    diagnostic_json_open:bool,""")
replace(receiver, "job_id:None,job_frame:None,failed:false", "job_id:None,job_frame:None,open_frame:None,#[cfg(test)] diagnostic_json_open:true,failed:false")
replace(receiver, "owner.job_frame=job_frame::JobFrame::select(wire,limits.maximum_bytes,policy.maximum_owned_bytes);owner}", """owner.job_frame=job_frame::JobFrame::select(wire,limits.maximum_bytes,policy.maximum_owned_bytes);
        if wire.starts_with(&semio_plugin_abi::original_open_frame::MAGIC){
            owner.open_frame=Some(open_frame::OpenInput::new(limits.maximum_bytes,policy.maximum_owned_bytes));
        }
        #[cfg(test)]
        {owner.diagnostic_json_open=false;}
        owner
    }
    fn json_open_requires_compact(&self)->bool{
        #[cfg(test)]
        if self.diagnostic_json_open{return false}
        matches!(self.input.as_ref(),Some(ActorInput::Poll(poll))
            if poll.command_page.is_none()&&poll.cold_pair_page.is_none()&&matches!(poll.events.as_slice(),[semio_framework::kernel::Event::InstanceOpen{..}]))
    }""")
replace(receiver, "        self.validate(wire,cx)?;\n        if self.capture.is_admitted()", """        self.validate(wire,cx)?;
        let compact=self.job_frame.is_some()||self.open_frame.is_some();
        if self.capture.is_admitted()""")
replace(receiver, "if self.closing&&(self.job_frame.is_some()||matches!(self.input.as_ref(),Some(ActorInput::Job(_))))", "if self.closing&&(compact||self.json_open_requires_compact()||matches!(self.input.as_ref(),Some(ActorInput::Job(_))))")
replace(receiver, "if self.job_frame.is_some()||self.decoder.terminal_is_empty()", "if compact||self.decoder.terminal_is_empty()")
replace(receiver, "if self.failed||self.job_frame.is_none()&&self.input.is_some()", "if self.failed||!compact&&self.input.is_some()")
replace(receiver, "let raw=if let Some(frame)=self.job_frame.as_mut(){", """let raw=if let Some(frame)=self.open_frame.as_mut(){
                match frame.advance(wire.ok_or_else(invalid)?,&mut *self.input,cx){
                    Ok(ready)=>{self.returning=ready;None},
                    Err(open_frame::OpenRefusal::Wire(error))=>Some(input_refusal(error)),
                    Err(open_frame::OpenRefusal::Source(error))=>return Err(error),
                }
            }else if let Some(frame)=self.job_frame.as_mut(){""")
replace(receiver, '                    .then(||input_refusal(ValueError::literal(ValueRefusalKind::InvalidValue,"owned Job input requires a compact SJS1 frame")))', """                    .then(||input_refusal(ValueError::literal(ValueRefusalKind::InvalidValue,"owned Job input requires a compact SJS1 frame")))
                    .or_else(||(!compact&&self.json_open_requires_compact()).then(||input_refusal(ValueError::literal(ValueRefusalKind::InvalidValue,"owned InstanceOpen without pages requires a compact SPO1 frame"))))""")

def lines(source):
    return (source if not source or source.endswith("\n") else source+"\n").splitlines(keepends=True)


patch = "".join(line for path, (old, new) in changes.items()
                for line in unified_diff(lines(old), lines(new), fromfile=f"a/{path}" if old else "/dev/null", tofile=f"b/{path}"))
assert all(len(str(path.resolve())) <= 256 for path in changes)
(ticket / "🗑️generated/instance-open-frame.patch").write_text(patch, encoding="utf-8")
print(f"[DEBUG] Authored {len(changes)} paths and {len(patch.splitlines())} diff lines")
