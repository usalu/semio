"""📥️ Stage native raw-media ownership, input dispatch, and receipt lifts as one product change."""

import hashlib
import json
import re
from pathlib import Path
from runpy import run_path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OUT = TICKET / "🗑️generated"
PLUGIN = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
REACTOR = PLUGIN / "⚛️reactor"
LIFETIME = REACTOR / "🚪️lifetime/🦀️.rs"
CORE_TURN = REACTOR / "🔄️turn"
RUNTIME_OPS = REACTOR / "🏃️runtime/🧰️ops"
RAW_OWNER = PLUGIN / "📜️declaration/🎞️media/📥️original-media"
WIRE = Path("🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress")
KERNEL = Path("🧰️framework/🔨️modules/🎠️kernel")
KERNEL_JSON = KERNEL / "📤️return/🔤️json/🎠️kernel/🦀️.rs"
NATIVE = PLUGIN / "🖥️host/🔗️wit-bindings/🔀️conversion/🦀️.rs"
NATIVE_JSON = PLUGIN / "🖥️host/🔁️poll/📥️events/🦀️.rs"
CHANNEL = Path("🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel")
BRIDGE = REACTOR / "🧬️wit-bridge/🪢️bridge/🦀️.rs"
CTRL = '    OriginalMediaControl(OriginalMediaControlEvent),\n'
REPLY = '    OriginalMediaAck(OriginalMediaWireAck),\n'
FIELDS = (
    "OriginalMediaWireLoan, OriginalMediaWireControl, OriginalMediaControlEvent, "
    "OriginalMediaWirePhase, OriginalMediaWireResult, OriginalMediaWireAck, ORIGINAL_MEDIA_COMMAND_KIND"
)

NATIVE_SLOT = """    /// 🧷️ Installs one paid picker into the exact native allocation before its effect becomes visible.
    pub fn admit_original_media(
        &mut self, key: NativeCloseKey, offered: &mut Option<raw_media::RawMediaIntent>,
        current: semio_plugin_reactor_app::original_media::OriginalMediaLoan<'_>,
        format_closed: bool,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let Some(intent) = offered.as_ref() else { return Ok(false); };
        if intent.loan() != current || intent.kind() != semio_framework::kernel::ORIGINAL_MEDIA_COMMAND_KIND {
            return Err(crate::life_reactor_close::reactor_close_fault("original picker changed before runtime admission").into());
        }
        if key.instance() != current.instance_id || !self.admit_command_origin(key, cx)? { return Ok(false); }
        if self.raw_media.picker.issued_loan() == Some(current) {
            return Err(crate::life_reactor_close::reactor_close_fault("original picker was already issued").into());
        }
        if !self.raw_media.terminal_is_empty() && (!format_closed || !self.raw_media.close(cx)?) { return Ok(false); }
        self.raw_media.admit_issued(offered, current, cx)
    }

"""
POLL_REGISTRY = """    /// 🧭️ Round-robins pending raw command receipts without spinning while waiting for a host page.
    pub fn next_raw_media_poll(&mut self, closing: &[u32]) -> Option<u32> {
        let count = self.slots.values.len();
        if count == 0 { return None; }
        for offset in 0..count {
            let index = (self.raw_media_cursor + offset) % count;
            let Some(slot) = self.slots.get(index) else { continue; };
            let instance = slot.cell.lifetime().instance_id;
            if slot.cell.is_live() && slot.raw_media.picker.receipt_due() && !closing.contains(&instance) {
                self.raw_media_cursor = (index + 1) % count;
                return Some(instance);
            }
        }
        None
    }

"""
APP_TRAIT = """    /// 🧾️ Returns the mounted document's complete raw-media revision.
    fn original_media_live_document(&self) -> Option<(&str, [u8; 32])> { None }
    /// ♻️ Answers whether the app's registered raw format work has completed paid close.
    fn original_media_format_closed(&self) -> bool { true }
    /// 📥️ Drives the registered decoder/transaction or its close under the current command Source.
    fn advance_original_media_format(
        &mut self, _picker: &mut semio_plugin_decl_media::original_media::RawMediaPicker,
        _identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<(), semio_framework_job::OriginalTurnFailure> { Ok(()) }
"""
APP_IMPL = """    fn original_media_live_document(&self) -> Option<(&str, [u8; 32])> {
        Some((self.stores.store.envelope().id.as_str(), self.stores.store.content_revision()))
    }

"""


def change(text, old, new):
    if text.count(old) != 1:
        raise ValueError(f"Raw reactor anchor drifted: {old}")
    return text.replace(old, new)


def kernel_to_wit_loan(source):
    return f"""wit_events::OriginalMediaLoan {{
        operation_id: {source}.operation_id.clone(), generation: {source}.generation.clone(),
        instance_id: {source}.instance_id, document_id: {source}.document_id.clone(), base_revision: {source}.base_revision.clone(),
    }}"""


def wit_to_kernel_loan(source):
    return f"""semio_framework::kernel::OriginalMediaWireLoan {{
        operation_id: {source}.operation_id, generation: {source}.generation,
        instance_id: {source}.instance_id, document_id: {source}.document_id, base_revision: {source}.base_revision,
    }}"""


def mapped_enum(source, src, dst, variants):
    return "match " + source + " { " + ", ".join(f"{src}::{value} => {dst}::{value}" for value in variants.split()) + " }"


def to_wit_ack():
    phase = mapped_enum("ack.phase", "semio_framework::kernel::OriginalMediaWirePhase", "wit::OriginalMediaPhase", "Begin Page Complete Cancel")
    result = mapped_enum("ack.result", "semio_framework::kernel::OriginalMediaWireResult", "wit::OriginalMediaResult", "Retry Pending Accepted Stale Refused")
    return f"""    if let semio_framework::kernel::CommandIngressStatus::OriginalMediaAck(ack) = &status {{
        return wit::CommandIngressStatus {{
            kind: 0, cursor: idle_command_cursor_to_wit(), fault: Vec::new(),
            original_media_ack: Some(wit::OriginalMediaAck {{
                loan: {kernel_to_wit_loan("ack.loan")}, owner: ack.owner, seq: ack.seq, kind: ack.kind, extent: ack.extent,
                phase: {phase}, result: {result}, page_index: ack.page_index,
            }}),
        }};
    }}
"""


def stage():
    run_path(str(TICKET / "raw-picker-product-stage.py"))["stage"]()
    run_path(str(TICKET / "raw-picker-schema-stage.py"))
    product = json.loads((OUT / "raw-sdk-overlay.json").read_bytes())
    schema = json.loads((OUT / "raw-picker-schema.json").read_bytes())
    preimages, post = product["preimages"], product["candidate"]
    preimages.update(schema["preimages"])
    post.update({p: text for p, text in schema["candidate"].items()
                 if not p.endswith("custody/🎟️source/🦀️.rs") and "/🎟️source/" not in p})

    kind = int(re.search(r"ORIGINAL_MEDIA_COMMAND_KIND: u8 = (\d+)", (TICKET / "raw-reactor-wire.rs").read_text(encoding="utf8"))[1])
    registry = ("🗜️codec/📥️command-decoder/🦀️.rs", "🧩️command-decode/🧭️command-cursor/🦀️.rs",
                "🧩️command-decode/🛣️route-and-media-decode/🦀️.rs", "🧩️command-decode/🎟️original/🦀️.rs")
    for file in registry:
        path = CHANNEL / file
        current = (ROOT / path).read_bytes()
        limits = re.findall(r"\b(\d+)(?:\.\.=(\d+))?\s*(?:=>|\|)", current.decode())
        if any(int(start) <= kind <= int(end or start) for start, end in limits) or kind == 28:
            raise ValueError(f"Raw media command kind overlaps the ordinary app command roster: {path}")
        preimages[str(path)] = hashlib.sha256(current).hexdigest()

    def edit(path, update):
        key = str(path)
        original = (ROOT / path).read_bytes()
        preimages.setdefault(key, hashlib.sha256(original).hexdigest())
        post[key] = update(post[key] if key in post else original.decode())

    def add(path, source):
        if (ROOT / path).exists() or len(str(ROOT / path)) > 256:
            raise ValueError(f"Cannot stage raw reactor source: {path}")
        preimages[str(path)] = None
        post[str(path)] = (TICKET / source).read_text(encoding="utf8")

    add(WIRE / "🎞️original-media.rs", "raw-reactor-wire.rs")
    edit(WIRE / "🦀️.rs", lambda s: change(change(s, '#[path = "📄️command-page-behavior/🦀️.rs"]',
        '#[path = "🎞️original-media.rs"]\nmod original_media;\npub use original_media::*;\n\n#[path = "📄️command-page-behavior/🦀️.rs"]'),
        '    CommandComplete(CommandPageCursor),\n', '    CommandComplete(CommandPageCursor),\n' + REPLY))
    edit(KERNEL / "🦀️.rs", lambda s: change(change(s, 'INVOCATION_RESULT_PACK_MAXIMUM_BYTES\n', 'INVOCATION_RESULT_PACK_MAXIMUM_BYTES,\n    ' + FIELDS + '\n'),
        '    ColdDocumentPairPage(ColdDocumentPairPage),\n', '    ColdDocumentPairPage(ColdDocumentPairPage),\n' + CTRL))
    edit(WIRE / "🚚️batch-driver-behavior/🦀️.rs", lambda s: change(change(s, '            CommandIngressStatus::Idle => {',
        '            CommandIngressStatus::OriginalMediaAck(_) => return Ok(CommandBatchProgress::Waiting),\n            CommandIngressStatus::Idle => {'),
        '            CommandIngressStatus::Idle => unreachable!(),','            CommandIngressStatus::Idle | CommandIngressStatus::OriginalMediaAck(_) => unreachable!(),'))

    def json_status(s):
        src = (TICKET / "raw-reactor-wire.rs").read_text(encoding="utf8")
        names = ("OriginalMediaWireLoan", "OriginalMediaWirePhase", "OriginalMediaWireResult", "OriginalMediaWireAck")
        records = []
        for name in names:
            start = min(i for i in (src.find("pub struct " + name), src.find("pub enum " + name)) if i >= 0)
            end = src.index("}", start) + 1
            records.append('semio_framework_value::native_json_codec!{\n#[borrowed_json(protocol="crate::projection::KernelJson")]\n#[value(rename_all = "camelCase")]\n' + src[start:end] + '\n}\n')
        return change(change(s, 'pub enum CommandIngressStatus {\n', 'pub enum CommandIngressStatus {\n' + REPLY),
           'semio_framework_value::native_json_codec!{\n#[borrowed_json(protocol="crate::projection::KernelJson")]\n#[value(rename_all = "camelCase")]\npub enum CommandIngressStatus',
           ''.join(records) + 'semio_framework_value::native_json_codec!{\n#[borrowed_json(protocol="crate::projection::KernelJson")]\n#[value(rename_all = "camelCase")]\npub enum CommandIngressStatus')

    edit(KERNEL_JSON, json_status)
    add(RAW_OWNER / "🧭️runtime.rs", "raw-reactor-owner.rs")
    root = RAW_OWNER / "🦀️.rs"
    post[str(root)] += '#[path = "🧭️runtime.rs"]\npub mod runtime;\n'

    def lifetime(s):
        s = change(s, 'pub use command_source::OriginalCommandOrigin;', 'pub use command_source::OriginalCommandOrigin;\npub use semio_plugin_reactor_app::original_media::runtime as raw_media;')
        s = change(s, '    pub command_origin:OriginalCommandOrigin,', '    pub command_origin:OriginalCommandOrigin,\n    pub raw_media: raw_media::OriginalMediaAdmission,')
        s = change(s, '    /// 🪪 Admits one command Source', NATIVE_SLOT + '    /// 🪪 Admits one command Source')
        s = change(s, 'command_origin:OriginalCommandOrigin::empty(),','command_origin:OriginalCommandOrigin::empty(), raw_media:raw_media::OriginalMediaAdmission::empty(),')
        s = change(s, '    pub cursor: usize,','    pub cursor: usize,\n    raw_media_cursor: usize,')
        s = change(s, 'serial: GuestLifecycleSerial::new(0), cursor: 0','serial: GuestLifecycleSerial::new(0), cursor: 0, raw_media_cursor: 0')
        s = change(s, '    pub fn pending_opening(', POLL_REGISTRY + '    pub fn pending_opening(')
        return change(s, 'if !owner.terminal_is_empty(peers)?{return Ok(false)}cx.with_original_retirement(|cx|self.command_origin.return_step(cx))',
            'if !owner.terminal_is_empty(peers)?{return Ok(false)}self.command_origin.validate(cx)?;if !self.raw_media.close(cx)?{return Ok(false)}cx.with_original_retirement(|cx|self.command_origin.return_step(cx))')

    edit(LIFETIME, lifetime)
    app = REACTOR / "🔌️app/🦀️.rs"
    edit(app, lambda s: change(s, "use semio_framework::kernel::FixedCommandPage;",
        "use semio_framework::kernel::FixedCommandPage;\npub use semio_plugin_decl_media::original_media;"))
    edit(app, lambda s: change(s, "pub trait PluginApp: Send + 'static {\n", "pub trait PluginApp: Send + 'static {\n" + APP_TRAIT))
    edit(PLUGIN / "🧵️instance/🔌️plugin-app-impl/🦀️.rs", lambda s: change(s, "    async fn app_id(&self) -> &str {", APP_IMPL + "    async fn app_id(&self) -> &str {"))

    add(RUNTIME_OPS / "🎞️raw-media/🦀️.rs", "raw-reactor-prologue.rs")
    edit(RUNTIME_OPS / "🦀️.rs", lambda s: change(s, '#[path = "🎞️media/🦀️.rs"]',
        '#[path = "🎞️raw-media/🦀️.rs"]\npub mod ops_raw_media;\n#[path = "🎞️media/🦀️.rs"]'))
    poll = CORE_TURN / "🌀️poll-kernel-turn/🦀️.rs"
    edit(poll, lambda s: change(s,
        "    let command_ingress = drive_command_ingress(runtime, command_page, close_instances, &mut effects, identity, cx).await?;",
        """    let raw = semio_plugin_runtime_ops::ops_raw_media::drive_raw_media(runtime, events, command_page, &close_instances, identity, cx)?;
    let command_ingress = match raw {
        Some(status) => {
            drive_command_ingress(runtime, &mut None, close_instances, &mut effects, identity, cx).await?;
            status
        }
        None => drive_command_ingress(runtime, command_page, close_instances, &mut effects, identity, cx).await?,
    };"""))
    ed = CORE_TURN / "📨️event-dispatch/🦀️.rs"
    edit(ed, lambda s: change(change(s, "Some(Event::Completed { .. }|Event::StagedFormRequested { .. })",
        "Some(Event::Completed { .. }|Event::StagedFormRequested { .. }|Event::OriginalMediaControl(_))"),
        "            Event::CommandIngressPage { .. } =>", "            Event::OriginalMediaControl(_) => unreachable!(\"raw event uses the original command prologue\"),\n            Event::CommandIngressPage { .. } =>"))

    def guest_bridge(s):
        s = change(s, "            wit::Event::UiIntent(intent) => intent.intent.len(),", """            wit::Event::OriginalMediaControl(event) => event.loan.operation_id.len() + event.loan.generation.len() +
                event.loan.base_revision.len() + event.loan.document_id.len() + match &event.control {
                    wit_events::OriginalMediaControl::Begin(begin) => begin.name.len() + begin.media_type.len(), _ => 0,
                },
            wit::Event::UiIntent(intent) => intent.intent.len(),""")
        s = change(s, "    wit::CommandIngressStatus { kind, cursor, fault }", "    wit::CommandIngressStatus { kind, cursor, fault, original_media_ack: None }")
        s = change(s, "        semio_framework::kernel::CommandIngressStatus::Idle =>", "        semio_framework::kernel::CommandIngressStatus::OriginalMediaAck(_) => unreachable!(),\n        semio_framework::kernel::CommandIngressStatus::Idle =>")
        s = change(s, "    let (kind, cursor, fault) = match status {", to_wit_ack() + "    let (kind, cursor, fault) = match status {")
        raw = f"""        W::OriginalMediaControl(event) => Event::OriginalMediaControl(semio_framework::kernel::OriginalMediaControlEvent {{
            loan: {wit_to_kernel_loan("event.loan")}, owner: event.owner, seq: event.seq, kind: event.kind, extent: event.extent,
            control: match event.control {{
                wit_events::OriginalMediaControl::Begin(begin) => semio_framework::kernel::OriginalMediaWireControl::Begin {{ name: begin.name, media_type: begin.media_type }},
                wit_events::OriginalMediaControl::Complete => semio_framework::kernel::OriginalMediaWireControl::Complete,
                wit_events::OriginalMediaControl::Cancel => semio_framework::kernel::OriginalMediaWireControl::Cancel,
            }},
        }}),
"""
        return change(s, "        W::Wake => Event::Wake,", raw + "        W::Wake => Event::Wake,")
    edit(BRIDGE, guest_bridge)

    def native_wire(s):
        phase = mapped_enum("ack.phase", "wit_reactor::OriginalMediaPhase", "semio_framework::kernel::OriginalMediaWirePhase", "Begin Page Complete Cancel")
        result = mapped_enum("ack.result", "wit_reactor::OriginalMediaResult", "semio_framework::kernel::OriginalMediaWireResult", "Retry Pending Accepted Stale Refused")
        raw_ack = f"""    if let Some(ack) = status.original_media_ack {{
        return semio_framework::kernel::CommandIngressStatus::OriginalMediaAck(semio_framework::kernel::OriginalMediaWireAck {{
            loan: {wit_to_kernel_loan("ack.loan")}, owner: ack.owner, seq: ack.seq, kind: ack.kind, extent: ack.extent,
            phase: {phase}, result: {result}, page_index: ack.page_index,
        }});
    }}
"""
        raw_event = f"""        Event::OriginalMediaControl(event) => wit_events::Event::OriginalMediaControl(wit_events::OriginalMediaControlEvent {{
            loan: {kernel_to_wit_loan("event.loan")}, owner: event.owner, seq: event.seq, kind: event.kind, extent: event.extent,
            control: match &event.control {{
                semio_framework::kernel::OriginalMediaWireControl::Begin {{ name, media_type }} => wit_events::OriginalMediaControl::Begin(wit_events::OriginalMediaBegin {{ name: name.clone(), media_type: media_type.clone() }}),
                semio_framework::kernel::OriginalMediaWireControl::Complete => wit_events::OriginalMediaControl::Complete,
                semio_framework::kernel::OriginalMediaWireControl::Cancel => wit_events::OriginalMediaControl::Cancel,
            }},
        }}),
"""
        s = change(s, "    let cursor = wit_command_cursor_to_kernel(status.cursor);", raw_ack + "    let cursor = wit_command_cursor_to_kernel(status.cursor);")
        return change(s, "        Event::Wake => wit_events::Event::Wake,", raw_event + "        Event::Wake => wit_events::Event::Wake,")

    edit(NATIVE, native_wire)

    def native_json(s):
        s = change(s, "use semio_framework_kernel::{Event,", "use semio_framework_kernel::{OriginalMediaControlEvent,OriginalMediaWireLoan,OriginalMediaWireControl,Event,")
        s = change(s, "Body(&'a Event),Bytes", "Body(&'a Event),Raw(&'a OriginalMediaControlEvent),RawLoan(&'a OriginalMediaWireLoan),RawControl(&'a OriginalMediaWireControl),RawBegin(&'a str,&'a str),Bytes")
        s = change(s, 'Event::CommandIngressPage{..}=>"commandIngressPage",', 'Event::CommandIngressPage{..}=>"commandIngressPage",Event::OriginalMediaControl(_)=>"originalMediaControl",')
        s = change(s, "  Self::Outcome(value)=>match", '  Self::RawControl(value)=>match value{OriginalMediaWireControl::Begin{..}=>"begin",OriginalMediaWireControl::Complete=>"complete",OriginalMediaWireControl::Cancel=>"cancel"},\n  Self::Outcome(value)=>match')
        s = change(s, "Self::Activation(ActivationEvent::OnStartupFinished))}", "Self::Activation(ActivationEvent::OnStartupFinished)|Self::RawControl(OriginalMediaWireControl::Complete|OriginalMediaWireControl::Cancel))}")
        s = change(s, "Self::Event(Event::ColdDocumentPairPage(v))=>Self::ColdPage(v),", "Self::Event(Event::ColdDocumentPairPage(v))=>Self::ColdPage(v),Self::Event(Event::OriginalMediaControl(v))=>Self::Raw(v),Self::RawControl(OriginalMediaWireControl::Begin{name,media_type})=>Self::RawBegin(name,media_type),")
        s = change(s, "Event::ColdDocumentPairPage(_)|Event::SuspendRequest", "Event::ColdDocumentPairPage(_)|Event::OriginalMediaControl(_)|Event::SuspendRequest")
        fields = '''   Self::Raw(v)=>fields!(index;"loan"=>Self::RawLoan(&v.loan),"owner"=>uint(v.owner),"seq"=>uint(v.seq),"kind"=>uint(v.kind),"extent"=>uint(v.extent),"control"=>Self::RawControl(&v.control)),
   Self::RawLoan(v)=>fields!(index;"operationId"=>Self::Text(&v.operation_id),"generation"=>Self::Text(&v.generation),"instanceId"=>uint(v.instance_id),"documentId"=>Self::Text(&v.document_id),"baseRevision"=>Self::Text(&v.base_revision)),
   Self::RawBegin(name,media_type)=>fields!(index;"name"=>Self::Text(name),"mediaType"=>Self::Text(media_type)),
'''
        return change(s, "   Self::Open(v)=>fields!", fields + "   Self::Open(v)=>fields!")
    edit(NATIVE_JSON, native_json)

    for key, expected in preimages.items():
        path = ROOT / key
        actual = hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None
        if actual != expected:
            raise ValueError(f"Raw reactor product preimage changed: {key}")
    (OUT / "raw-reactor-overlay.json").write_text(json.dumps({"preimages": preimages, "candidate": post}, ensure_ascii=False), encoding="utf8")
    linked = OUT / "rm-reactor"
    linked.mkdir(exist_ok=True)
    for name, target in (("prologue.rs", RUNTIME_OPS / "🎞️raw-media/🦀️.rs"),):
        (linked / name).write_text(post[str(target)], encoding="utf8")
    for name in ("🦀️.rs", "🧭️runtime.rs"):
        (OUT / "rm-sdk" / name).write_text(post[str(RAW_OWNER / name)], encoding="utf8")
    print(f"[DEBUG] Guarded {len(post)} combined raw reactor candidate files")


if __name__ == "__main__":
    stage()
