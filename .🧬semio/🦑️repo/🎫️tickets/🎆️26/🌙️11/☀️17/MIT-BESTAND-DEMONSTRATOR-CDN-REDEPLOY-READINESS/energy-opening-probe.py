"""🔬️ Toggle bounded tracing of the Energy actor opening."""
from pathlib import Path
import sys

plugin = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
opening = plugin / "⚛️reactor/🏃️runtime/📦️instances/🚪️opening/🦀️.rs"
factory = plugin / "🧵️instance/🧱️construction/🏭️factory/🦀️.rs"

opening_anchor = "opening.validate(cx)?;"
opening_probe = """
 thread_local! { static ENERGY_OPENING: std::cell::Cell<(u32, u16)> = const { std::cell::Cell::new((0, u16::MAX)) }; }
 if opening.source.as_ref().is_none_or(|(app, _)|app.starts_with("s.energy.model")) {
  let phase = u16::from(opening.loan.is_some()) | u16::from(opening.app.is_some())<<1 | u16::from(opening.closing)<<2 | u16::from(opening.construction.is_some())<<3 | u16::from(opening.identity_bound)<<4 | u16::from(opening.issuer_ready)<<5 | u16::from(opening.returning_loan)<<6 | u16::from(opening.source.is_some())<<7 | u16::from(cx.should_yield())<<8;
  if ENERGY_OPENING.with(|last|last.replace((instance_id,phase))!=(instance_id,phase)) { eprintln!("[DEBUG] energy opening instance={instance_id} flags={phase:#x}"); }
 }"""
error_anchor = "opening.closing=true;*opening.failure=Some(failure);"
error_probe = """if opening.source.as_ref().is_none_or(|(app,_)|app.starts_with("s.energy.model")){eprintln!("[DEBUG] energy opening failure {failure:?}");}"""
factory_anchor = "Result<bool,OriginalTurnFailure>{if !self.binding.matches(cx)||self.closing||output.is_some()&&!self.finishing"
factory_probe = """
  thread_local! { static ENERGY_FACTORY: std::cell::Cell<(u16,Option<(u8,Option<(u8,bool,bool)>,bool)>)> = const { std::cell::Cell::new((u16::MAX,None)) }; }
  if source.definition.id.starts_with("s.energy.model") {
   let flags = u16::from(self.finishing) | u16::from(self.source.is_some())<<1 | u16::from(self.actor.is_some())<<2 | u16::from(self.app.is_some())<<3 | u16::from(self.lanes.is_some())<<4 | u16::from(self.registration.is_some())<<5 | u16::from(self.parts.is_some())<<6 | u16::from(self.admission.is_some())<<7 | u16::from(self.candidate.is_some())<<8 | u16::from(self.closing)<<9;
   let stores = self.stores.as_ref().map(|owner|owner.progress());
   if ENERGY_FACTORY.with(|last|last.replace((flags,stores))!=(flags,stores)) { eprintln!("[DEBUG] energy factory flags={flags:#x} stores={stores:?}"); }
  }
  """

add = sys.argv[1] == "add"
patches = (
    (opening, opening_anchor, opening_anchor + opening_probe),
    (opening, error_anchor, error_probe + error_anchor),
    (factory, factory_anchor, factory_anchor.replace("{if", "{" + factory_probe + "if", 1)),
)
for path, original, instrumented in patches:
    source, target = (original, instrumented) if add else (instrumented, original)
    text = path.read_text()
    if text.count(source) != 1:
        raise RuntimeError(f"unexpected probe anchor in {path}")
    path.write_text(text.replace(source, target, 1))
