"""📐 Stage the raw picker control and receipt on the actual actor WIT interface."""

import hashlib
import json
from runpy import run_path
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
PLUGIN = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
SCHEMA = PLUGIN / "🧬️schema"
EVENTS = SCHEMA / "📜️events.wit"
REACTOR = SCHEMA / "📜️reactor.wit"
EVENT_LIFT = PLUGIN / "⚛️reactor/🧬️wit-bridge/🎟️event-lift/🦀️.rs"
ORIGIN = PLUGIN / "⚛️reactor/🏃️runtime/🎞️command/📥️custody/🎟️source/🦀️.rs"
EDIT = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor")
OUTPUT = TICKET / "🗑️generated" / "raw-picker-schema.json"

RECORDS = """  record original-media-loan {
    operation-id: string,
    generation: string,
    instance-id: instance-id,
    document-id: string,
    base-revision: string,
  }
  record original-media-begin {
    name: string,
    media-type: string,
  }
  variant original-media-control {
    begin(original-media-begin),
    complete,
    cancel,
  }
  record original-media-control-event {
    loan: original-media-loan,
    owner: u64,
    seq: u64,
    kind: u8,
    extent: u32,
    control: original-media-control,
  }

"""

ACK = """  enum original-media-phase { begin, page, complete, cancel }
  enum original-media-result { retry, pending, accepted, stale, refused }
  record original-media-ack {
    loan: original-media-loan,
    owner: u64,
    seq: u64,
    kind: u8,
    extent: u32,
    phase: original-media-phase,
    page-index: option<u32>,
    %result: original-media-result,
  }
"""

LIFT = """fn bounded_media_text(value: &DslValue, key: &str, maximum: usize) -> Lift<String> {
    let value = field(value, key)?.as_str().ok_or_else(outside)?;
    if value.is_empty() || value.chars().take(maximum + 1).count() > maximum { return Err(outside()); }
    Ok(value.to_owned())
}

fn original_media_control(value: &DslValue) -> Lift<ev::OriginalMediaControlEvent> {
    let loan = field(value, "loan")?;
    let operation_id = bounded_media_text(loan, "operationId", 16)?;
    let generation = bounded_media_text(loan, "generation", 16)?;
    let base_revision = bounded_media_text(loan, "baseRevision", 64)?;
    let valid_hex = |value: &str, width| value.len() == width &&
        value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if !valid_hex(&operation_id, 16) || !valid_hex(&generation, 16) || !valid_hex(&base_revision, 64) {
        return Err(outside());
    }
    let control = field(value, "control")?;
    let phase = field(control, "tag")?.as_str().ok_or_else(outside)?;
    let control = match (phase, control.get("val")) {
        ("begin", Some(begin)) => ev::OriginalMediaControl::Begin(ev::OriginalMediaBegin {
            name: bounded_media_text(begin, "name", 255)?,
            media_type: bounded_media_text(begin, "mediaType", 255)?,
        }),
        ("complete", None) => ev::OriginalMediaControl::Complete,
        ("cancel", None) => ev::OriginalMediaControl::Cancel,
        _ => return Err(outside()),
    };
    Ok(ev::OriginalMediaControlEvent {
        loan: ev::OriginalMediaLoan {
            operation_id, generation, base_revision, instance_id: word32(loan, "instanceId")?,
            document_id: bounded_media_text(loan, "documentId", 256)?,
        },
        owner: word64(value, "owner")?, seq: word64(value, "seq")?,
        kind: u8::try_from(word64(value, "kind")?).map_err(|_| outside())?,
        extent: word32(value, "extent")?, control,
    })
}

"""


def digest(body):
    return hashlib.sha256(body).hexdigest()


def replace(body, needle, new):
    if body.count(needle) != 1:
        raise ValueError(f"Raw picker schema anchor drifted: {needle}")
    return body.replace(needle, new)


sources = {}
candidate = {}
anchors = {
    PLUGIN / "⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs": (
        "command_page_invalid(runtime,cursor,page", "slot.command_origin.admit_step(cx)?",
        "continue_generic_assembly"
    ),
    PLUGIN / "⚛️reactor/🧬️wit-bridge/🪢️bridge/🦀️.rs": (
        "kernel_command_ingress_to_wit", "args: args.map(|value| pack(&value))"
    ),
    PLUGIN / "⚛️reactor/🔄️turn/📨️event-dispatch/🦀️.rs": ("fn select_turn_event", "Event::CommandIngressPage"),
    EDIT / "📥️local-image/🦀️.rs": ("BoundedArtifactCommandWork::new", "Pdf17EditorCommand::PickImage", "picker(snapshot, operation"),
    EDIT / "📄️document/🖼️image-authoring/🧵️decode.rs": ("ImageDecodeJob::new", "job.advance(payload, effect)"),
}
for path in (EVENTS, REACTOR, EVENT_LIFT, ORIGIN, *anchors):
    raw = (ROOT / path).read_bytes()
    sources[str(path)] = digest(raw)
    body = raw.decode()
    if not all(needle in body for needle in anchors.get(path, ())):
        raise ValueError(f"Raw picker integration source drifted: {path}")
    if path == EVENTS:
        body = replace(body, "  variant event {", RECORDS + "  variant event {")
        body = replace(body, "    request(request-event),", "    request(request-event),\n    original-media-control(original-media-control-event),")
        candidate[str(path)] = body
    if path == REACTOR:
        body = replace(body, "use events.{event};", "use events.{event, original-media-loan};")
        body = replace(body, "  record command-ingress-status {", ACK + "  record command-ingress-status {\n    original-media-ack: option<original-media-ack>,")
        candidate[str(path)] = body
    if path == EVENT_LIFT:
        body = replace(body, "/// 🎟️ Lifts one event value;", LIFT + "/// 🎟️ Lifts one event value;")
        body = replace(body, '        "instance-open" =>', '        "original-media-control" => ev::Event::OriginalMediaControl(original_media_control(val)?),\n        "instance-open" =>')
        candidate[str(path)] = body
    if path == ORIGIN:
        candidate[str(path)] = run_path(str(TICKET / "raw-picker-origin-stage.py"))["stage"](body)

for name, expected in sources.items():
    if digest((ROOT / name).read_bytes()) != expected:
        raise ValueError(f"Raw picker preimage changed: {name}")

OUTPUT.parent.mkdir(exist_ok=True)
OUTPUT.write_text(json.dumps({"preimages": sources, "candidate": candidate}, ensure_ascii=False), encoding="utf8")
print(f"[DEBUG] Guarded {len(candidate)} raw picker WIT, event-lift and command-origin candidates and {len(anchors)} integration sources")
