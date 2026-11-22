"""Connect the retained Event, WIT and borrowed JSON transport paths."""
from pathlib import Path
from runpy import run_path

staged = run_path(str(Path(__file__).with_name("natural-pdf-open-stage.py")))
plugin = staged["plugin"]
reactor = staged["reactor"]
kernel = staged["wire"]["kernel"]
change = staged["change"]
emit = staged["emit"]

def convert(source, target, variants, field):
    branches = ", ".join(f"{source}::{name} => {target}::{name}" for name in variants.split())
    return f"match {field} {{ {branches} }}"

loan = "NaturalSourceWireLoan"
decl = "NaturalSourceWireDeclaration"
native = "semio_framework::kernel"
host_event = "wit_events"
guest_event = "wit_events"
host_result = "wit_reactor"
guest_result = "wit"
phases = "Prepare Begin Page Complete Cancel"
results = "Retry Pending Accepted Stale Refused"
controls = "Begin Complete Cancel"

def declaration(to, value, clone=""):
    return f"{to} {{ format_kind: {value}.format_kind{clone}, extension: {value}.extension{clone}, media_type: {value}.media_type{clone}, binary: {value}.binary }}"

def loan_record(to, decl_to, v, clone=""):
    return f"{to} {{ operation_id: {v}.operation_id{clone}, generation: {v}.generation{clone}, instance_id: {v}.instance_id, declaration: {declaration(decl_to, v + '.declaration', clone)}, extent: {v}.extent }}"

def payload(ty, declaration_value, variable, extra=""):
    return f"{ty} {{ instance_id: {variable}.instance_id, seq: {variable}.seq, declaration: {declaration_value}, extent: {variable}.extent{extra} }}"

def control_event(ty, offered, phase):
    return f"{ty} {{ loan: {offered}, owner: event.owner, seq: event.seq, kind: event.kind, extent: event.extent, control: {phase} }}"

def launch(to, payload_type, decl_to, value, clone=""):
    if to.startswith("wit"):
        return f"match {value} {{ {native}::ActorInstanceOpenPurpose::Ordinary => {to}::Ordinary, {native}::ActorInstanceOpenPurpose::NaturalSource {{ declaration, extent }} => {to}::NaturalSource({payload_type} {{ declaration: {declaration(decl_to, 'declaration', clone)}, extent: *extent }}) }}"
    return f"match {value} {{ {guest_event}::ActorInstanceOpenPurpose::Ordinary => {to}::Ordinary, {guest_event}::ActorInstanceOpenPurpose::NaturalSource(source) => {to}::NaturalSource {{ declaration: {declaration(decl_to, 'source.declaration')}, extent: source.extent }} }}"

def ack_record(ty, offered, phase, result):
    return f"{ty} {{ loan: {offered}, owner: ack.owner, seq: ack.seq, kind: ack.kind, extent: ack.extent, phase: {phase}, page_index: ack.page_index, result: {result} }}"

wit_prepare = payload("wit_events::NaturalSourcePrepare", declaration("wit_events::NaturalSourceDeclaration", "event.declaration", ".clone()"), "event")
kernel_prepare = payload(native + "::NaturalSourcePrepare", declaration(native + "::" + decl, "event.declaration"), "event")
wire_loan = loan_record("wit_events::NaturalSourceLoan", "wit_events::NaturalSourceDeclaration", "event.loan", ".clone()")
typed_loan = loan_record(native + "::" + loan, native + "::" + decl, "event.loan")
wire_control = control_event("wit_events::NaturalSourceControlEvent", wire_loan, convert(native + "::NaturalSourceWireControl", "wit_events::NaturalSourceControl", controls, "event.control"))
typed_control = control_event(native + "::NaturalSourceControlEvent", typed_loan, convert("wit_events::NaturalSourceControl", native + "::NaturalSourceWireControl", controls, "event.control"))
host_purpose = launch("wit_events::ActorInstanceOpenPurpose", "wit_events::NaturalSourceLaunch", "wit_events::NaturalSourceDeclaration", "purpose", ".clone()")
guest_purpose = launch(native + "::ActorInstanceOpenPurpose", None, native + "::" + decl, "payload.purpose")

host_ack = ack_record(native + "::NaturalSourceWireAck",
    "ack.loan.map(|loan| " + loan_record(native + "::" + loan, native + "::" + decl, "loan") + ")",
    convert("wit_reactor::NaturalSourcePhase", native + "::NaturalSourceWirePhase", phases, "ack.phase"),
    convert("wit_reactor::NaturalSourceResult", native + "::NaturalSourceWireResult", results, "ack.result"))
guest_ack = ack_record("wit::NaturalSourceAck",
    "ack.loan.map(|loan| " + loan_record("wit_events::NaturalSourceLoan", "wit_events::NaturalSourceDeclaration", "loan") + ")",
    convert(native + "::NaturalSourceWirePhase", "wit::NaturalSourcePhase", phases, "ack.phase"),
    convert(native + "::NaturalSourceWireResult", "wit::NaturalSourceResult", results, "ack.result"))

change(reactor / "🧬️wit-bridge/🪢️bridge/🦀️.rs", [
    ("    let completion_bytes = ", """    let declaration_bytes = |value: &wit_events::NaturalSourceDeclaration| value.format_kind.len() + value.extension.len() + value.media_type.len();
    let completion_bytes = """),
    ("wit::Event::InstanceOpen(open) => open.config.len()", """wit::Event::InstanceOpen(open) => (match &open.purpose {
                wit_events::ActorInstanceOpenPurpose::Ordinary => 0,
                wit_events::ActorInstanceOpenPurpose::NaturalSource(launch) => declaration_bytes(&launch.declaration),
            }) + open.config.len()"""),
    ("            wit::Event::OriginalMediaControl(event) =>", """            wit::Event::NaturalSourcePrepare(event) => declaration_bytes(&event.declaration),
            wit::Event::NaturalSourceControl(event) => event.loan.operation_id.len() + event.loan.generation.len() + declaration_bytes(&event.loan.declaration),
            wit::Event::OriginalMediaControl(event) =>"""),
    ("kind: 0, cursor: idle_command_cursor_to_wit(), fault: Vec::new(),\n            original_media_ack", "kind: 0, cursor: idle_command_cursor_to_wit(), fault: Vec::new(), natural_source_ack: None,\n            original_media_ack"),
    ("        semio_framework::kernel::CommandIngressStatus::OriginalMediaAck(_) => unreachable!(),", f"""        semio_framework::kernel::CommandIngressStatus::OriginalMediaAck(_) => unreachable!(),
        semio_framework::kernel::CommandIngressStatus::NaturalSourceAck(ack) => return wit::CommandIngressStatus {{
            kind: 0, cursor: idle_command_cursor_to_wit(), fault: Vec::new(), original_media_ack: None,
            natural_source_ack: Some({guest_ack}),
        }},"""),
    ("wit::CommandIngressStatus { kind, cursor, fault, original_media_ack: None }", "wit::CommandIngressStatus { kind, cursor, fault, original_media_ack: None, natural_source_ack: None }"),
    ("        W::InstanceOpen(payload) => Event::InstanceOpen {", f"        W::InstanceOpen(payload) => Event::InstanceOpen {{\n            purpose: {guest_purpose},"),
    ("        W::Wake => Event::Wake,", f"        W::NaturalSourcePrepare(event) => Event::NaturalSourcePrepare({kernel_prepare}),\n        W::NaturalSourceControl(event) => Event::NaturalSourceControl({typed_control}),\n        W::Wake => Event::Wake,"),
])
change(plugin / "🖥️host/🔗️wit-bindings/🔀️conversion/🦀️.rs", [
    ("    if let Some(ack) = status.original_media_ack {", f"""    if status.natural_source_ack.is_some() && status.original_media_ack.is_some() {{
        return {native}::CommandIngressStatus::Fault {{ cursor: wit_command_cursor_to_kernel(status.cursor), fault: b"plugin.multiple-command-receipts".to_vec() }};
    }}
    if let Some(ack) = status.natural_source_ack {{
        return {native}::CommandIngressStatus::NaturalSourceAck({host_ack});
    }}
    if let Some(ack) = status.original_media_ack {{"""),
    ("Event::InstanceOpen { request, app_id, actor, config, assets, capabilities, quotas }", "Event::InstanceOpen { request, app_id, actor, config, assets, capabilities, quotas, purpose }"),
    ("wit_events::Event::InstanceOpen(wit_events::InstanceOpenEvent {\n                instance:", f"wit_events::Event::InstanceOpen(wit_events::InstanceOpenEvent {{\n                purpose: {host_purpose},\n                instance:"),
    ("        Event::Wake => wit_events::Event::Wake,", f"        Event::NaturalSourcePrepare(event) => wit_events::Event::NaturalSourcePrepare({wit_prepare}),\n        Event::NaturalSourceControl(event) => wit_events::Event::NaturalSourceControl({wire_control}),\n        Event::Wake => wit_events::Event::Wake,"),
])

change(reactor / "🧬️wit-bridge/🎟️event-lift/🦀️.rs", [
    ("/// 🎟️ Lifts one event value", """fn closed_natural_fields(value: &DslValue, expected: &[&str]) -> Lift<()> {
    let DslValue::Object(fields) = value else { return Err(outside()); };
    if fields.len() != expected.len() || expected.iter().any(|key| value.get(key).is_none()) {
        return Err(outside());
    }
    Ok(())
}

fn natural_declaration(value: &DslValue) -> Lift<ev::NaturalSourceDeclaration> {
    closed_natural_fields(value, &["formatKind", "extension", "mediaType", "binary"])?;
    Ok(ev::NaturalSourceDeclaration {
        format_kind: bounded_media_text(value, "formatKind", 128)?,
        extension: bounded_media_text(value, "extension", 16)?,
        media_type: bounded_media_text(value, "mediaType", 128)?,
        binary: flag(value, "binary")?,
    })
}

fn natural_purpose(value: &DslValue) -> Lift<ev::ActorInstanceOpenPurpose> {
    let (tag, val) = tagged(value)?;
    Ok(match (tag.as_str(), val) {
        ("ordinary", None) => {
            closed_natural_fields(value, &["tag"])?;
            ev::ActorInstanceOpenPurpose::Ordinary
        }
        ("natural-source", Some(source)) => {
            closed_natural_fields(value, &["tag", "val"])?;
            closed_natural_fields(source, &["declaration", "extent"])?;
            ev::ActorInstanceOpenPurpose::NaturalSource(ev::NaturalSourceLaunch {
                declaration: natural_declaration(field(source, "declaration")?)?,
                extent: word32(source, "extent")?,
            })
        }
        _ => return Err(outside()),
    })
}

fn natural_prepare(value: &DslValue) -> Lift<ev::NaturalSourcePrepare> {
    closed_natural_fields(value, &["instanceId", "seq", "declaration", "extent"])?;
    Ok(ev::NaturalSourcePrepare {
        instance_id: word32(value, "instanceId")?,
        seq: word64(value, "seq")?,
        declaration: natural_declaration(field(value, "declaration")?)?,
        extent: word32(value, "extent")?,
    })
}

fn natural_control(value: &DslValue) -> Lift<ev::NaturalSourceControlEvent> {
    closed_natural_fields(value, &["loan", "owner", "seq", "kind", "extent", "control"])?;
    let loan = field(value, "loan")?;
    closed_natural_fields(loan, &["operationId", "generation", "instanceId", "declaration", "extent"])?;
    let hex = |key| {
        let value = bounded_media_text(loan, key, 16)?;
        if value.len() != 16 || value == "0000000000000000" ||
           !value.bytes().all(|ch| ch.is_ascii_digit() || (b'a'..=b'f').contains(&ch)) { return Err(outside()); }
        Ok(value)
    };
    let control = match field(value, "control")?.as_str().ok_or_else(outside)? {
        "begin" => ev::NaturalSourceControl::Begin,
        "complete" => ev::NaturalSourceControl::Complete,
        "cancel" => ev::NaturalSourceControl::Cancel,
        _ => return Err(outside()),
    };
    Ok(ev::NaturalSourceControlEvent {
        loan: ev::NaturalSourceLoan {
            operation_id: hex("operationId")?,
            generation: hex("generation")?,
            instance_id: word32(loan, "instanceId")?,
            declaration: natural_declaration(field(loan, "declaration")?)?,
            extent: word32(loan, "extent")?,
        },
        owner: word64(value, "owner")?, seq: word64(value, "seq")?,
        kind: u8::try_from(word64(value, "kind")?).map_err(|_| outside())?,
        extent: word32(value, "extent")?, control,
    })
}

/// 🎟️ Lifts one event value"""),
    ('        "instance-open" => ev::Event::InstanceOpen(ev::InstanceOpenEvent {', '''        "natural-source-prepare" => ev::Event::NaturalSourcePrepare(natural_prepare(val)?),
        "natural-source-control" => ev::Event::NaturalSourceControl(natural_control(val)?),
        "instance-open" => ev::Event::InstanceOpen(ev::InstanceOpenEvent {
            purpose: natural_purpose(field(val, "purpose")?)?,'''),
])

change(plugin / "🖥️host/🧬️component-codec/🎟️retained-turn/🦀️.rs", [
    ("/// 🎟️ The lowered form of one WIT", """fn wire_natural_declaration(declaration: &crate::wit_events::NaturalSourceDeclaration) -> serde_json::Value {
    serde_json::json!({"formatKind":declaration.format_kind,"extension":declaration.extension,"mediaType":declaration.media_type,"binary":declaration.binary})
}

fn wire_open_purpose(purpose: &crate::wit_events::ActorInstanceOpenPurpose) -> serde_json::Value {
    use crate::wit_events::ActorInstanceOpenPurpose;
    match purpose {
        ActorInstanceOpenPurpose::Ordinary => serde_json::json!({"tag":"ordinary"}),
        ActorInstanceOpenPurpose::NaturalSource(source) => serde_json::json!({"tag":"natural-source","val":{"declaration":wire_natural_declaration(&source.declaration),"extent":source.extent}}),
    }
}

/// 🎟️ The lowered form of one WIT"""),
    ('"instance":open.instance,"activationGeneration"', '"instance":open.instance,"purpose":wire_open_purpose(&open.purpose),"activationGeneration"'),
    ('  W::Wake=>serde_json::json!({"tag":"wake"}),', '''  W::NaturalSourcePrepare(event)=>serde_json::json!({"tag":"natural-source-prepare","val":{"instanceId":event.instance_id,"seq":event.seq,"declaration":wire_natural_declaration(&event.declaration),"extent":event.extent}}),
  W::NaturalSourceControl(event)=>{
   use crate::wit_events::NaturalSourceControl;
   let control=match event.control { NaturalSourceControl::Begin=>"begin", NaturalSourceControl::Complete=>"complete", NaturalSourceControl::Cancel=>"cancel" };
   serde_json::json!({"tag":"natural-source-control","val":{
    "loan":{"operationId":event.loan.operation_id,"generation":event.loan.generation,"instanceId":event.loan.instance_id,
     "declaration":wire_natural_declaration(&event.loan.declaration),"extent":event.loan.extent},
    "owner":event.owner,"seq":event.seq,"kind":event.kind,"extent":event.extent,"control":control,
   }})
  }
  W::Wake=>serde_json::json!({"tag":"wake"}),'''),
])

change(plugin / "🖥️host/🔁️poll/📥️events/🦀️.rs", [
    ("use semio_framework_kernel::{OriginalMediaControlEvent,", "use semio_framework_kernel::{ActorInstanceOpenPurpose,NaturalSourceControlEvent,NaturalSourcePrepare,NaturalSourceWireControl,NaturalSourceWireDeclaration,NaturalSourceWireLoan,OriginalMediaControlEvent,"),
    ("Raw(&'a OriginalMediaControlEvent),", "Purpose(&'a ActorInstanceOpenPurpose),NaturalLaunch(&'a NaturalSourceWireDeclaration,u32),NaturalDecl(&'a NaturalSourceWireDeclaration),NaturalLoan(&'a NaturalSourceWireLoan),NaturalPrepare(&'a NaturalSourcePrepare),NaturalControl(&'a NaturalSourceControlEvent),Raw(&'a OriginalMediaControlEvent),"),
    ('Event::OriginalMediaControl(_)=>"originalMediaControl",', 'Event::OriginalMediaControl(_)=>"originalMediaControl",Event::NaturalSourcePrepare(_)=>"naturalSourcePrepare",Event::NaturalSourceControl(_)=>"naturalSourceControl",'),
    ("  Self::RawControl(value)=>", '  Self::Purpose(value)=>match value{ActorInstanceOpenPurpose::Ordinary=>"ordinary",ActorInstanceOpenPurpose::NaturalSource{..}=>"naturalSource"},\n  Self::RawControl(value)=>'),
    ("Self::Activation(ActivationEvent::OnStartupFinished)|Self::RawControl", "Self::Activation(ActivationEvent::OnStartupFinished)|Self::Purpose(ActorInstanceOpenPurpose::Ordinary)|Self::RawControl"),
    ("Self::Event(Event::OriginalMediaControl(v))=>Self::Raw(v),", "Self::Event(Event::OriginalMediaControl(v))=>Self::Raw(v),Self::Event(Event::NaturalSourcePrepare(v))=>Self::NaturalPrepare(v),Self::Event(Event::NaturalSourceControl(v))=>Self::NaturalControl(v),Self::Purpose(ActorInstanceOpenPurpose::NaturalSource{declaration,extent})=>Self::NaturalLaunch(declaration,*extent),"),
    ('Event::InstanceOpen{request,app_id,actor,config,assets,capabilities,quotas}=>fields!(index;"request"=>Self::Open(*request),', 'Event::InstanceOpen{request,app_id,actor,config,assets,capabilities,quotas,purpose}=>fields!(index;"request"=>Self::Open(*request),"purpose"=>Self::Purpose(purpose),'),
    ("|Event::OriginalMediaControl(_)|Event::SuspendRequest", "|Event::OriginalMediaControl(_)|Event::NaturalSourcePrepare(_)|Event::NaturalSourceControl(_)|Event::SuspendRequest"),
    ('   Self::Raw(v)=>fields!', '''   Self::NaturalLaunch(declaration,extent)=>fields!(index;"declaration"=>Self::NaturalDecl(declaration),"extent"=>uint(extent)),
   Self::NaturalDecl(v)=>fields!(index;"formatKind"=>Self::Text(&v.format_kind),"extension"=>Self::Text(&v.extension),"mediaType"=>Self::Text(&v.media_type),"binary"=>Self::Bool(v.binary)),
   Self::NaturalLoan(v)=>fields!(index;"operationId"=>Self::Text(&v.operation_id),"generation"=>Self::Text(&v.generation),"instanceId"=>uint(v.instance_id),"declaration"=>Self::NaturalDecl(&v.declaration),"extent"=>uint(v.extent)),
   Self::NaturalPrepare(v)=>fields!(index;"instanceId"=>uint(v.instance_id),"seq"=>uint(v.seq),"declaration"=>Self::NaturalDecl(&v.declaration),"extent"=>uint(v.extent)),
   Self::NaturalControl(v)=>fields!(index;"loan"=>Self::NaturalLoan(&v.loan),"owner"=>uint(v.owner),"seq"=>uint(v.seq),"kind"=>uint(v.kind),"extent"=>uint(v.extent),"control"=>Self::Text(match v.control{NaturalSourceWireControl::Begin=>"begin",NaturalSourceWireControl::Complete=>"complete",NaturalSourceWireControl::Cancel=>"cancel"})),
   Self::Raw(v)=>fields!'''),
])
change(kernel / "📤️return/🔤️json/🎠️kernel/🦀️.rs", [
    ("    OriginalMediaAck(OriginalMediaWireAck),", "    OriginalMediaAck(OriginalMediaWireAck),\n    NaturalSourceAck(NaturalSourceWireAck),"),
])
emit("natural-source-transport.patch")
