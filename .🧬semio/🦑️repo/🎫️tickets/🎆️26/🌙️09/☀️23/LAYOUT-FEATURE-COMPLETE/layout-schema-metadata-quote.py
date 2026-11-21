import sys
from pathlib import Path

p = Path("🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🔎️schema/🎛️drive.rs")
s = p.read_text()

def edit(old, new):
    global s
    if s.count(old) != 1:
        raise SystemExit(f"cannot place selected schema Work change: {old!r}")
    s = s.replace(old, new)

edit(
    "pub const NATIVE_SCHEMA_METADATA_ITEMS:usize=3;",
    "pub const NATIVE_SCHEMA_METADATA_ITEMS:usize=3;\nconst NATIVE_SCHEMA_METADATA_WORK:u64=3;",
)
edit(
    "S::CheckpointRecovery|S::KeyMetadata(_)=>Ok(1),_=>Err(invalid())",
    "S::CheckpointRecovery=>Ok(1),S::KeyMetadata(_)=>Ok(NATIVE_SCHEMA_METADATA_WORK),_=>Err(invalid())",
)
edit(
    "  match self.selection(){\n   Ok(S::KeyAcknowledgement|S::KeyMetadata(_))",
    "  match self.selection(){\n   Ok(S::KeyMetadata(_)) if maximum<NATIVE_SCHEMA_METADATA_WORK||grant.maximum_items<NATIVE_SCHEMA_METADATA_ITEMS=>idle(0),\n   Ok(S::KeyAcknowledgement|S::KeyMetadata(_))",
)
edit(
    "  control.with_original_work_and_construction(|work,physical|{\n   let mut total=RetainedCloneProgress::default();",
    """  control.with_original_work_and_construction(|work,physical|{
   let mut quote=None;
   let admitted=work.prepare(NATIVE_SCHEMA_METADATA_WORK,&mut quote);
   if quote.is_some(){work.cancel(&mut quote)?;}
   if admitted?!=Some(true){return Ok(Default::default())}
   let mut total=RetainedCloneProgress::default();""",
)
edit(
    "   physical.begin(demand,&mut original,work)?;physical.quote(original.as_mut().ok_or_else(invalid)?)?;let grant=physical.recover_grant(original.as_mut().ok_or_else(invalid)?)?.ok_or_else(invalid)??;\n   if work.prepare(1,&mut original_work)?!=Some(true){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,\"schema acknowledgement action lacks original Work\"))}",
    "   if work.prepare(1,&mut original_work)?!=Some(true){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,\"schema acknowledgement action lacks original Work\"))}\n   physical.begin(demand,&mut original,work)?;physical.quote(original.as_mut().ok_or_else(invalid)?)?;let grant=physical.recover_grant(original.as_mut().ok_or_else(invalid)?)?.ok_or_else(invalid)??;",
)
if sys.argv[1:] == ["--write"]:
    p.write_text(s)
print("Schema metadata Work and physical quote change validated")
