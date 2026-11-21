//! 🧪 Drives the staged WGPU sender and queue against the repository's actual kernel types.
extern crate self as semio_framework;
pub extern crate semio_framework_kernel as kernel;
pub use semio_framework_value::DslValue;

#[path = "media.rs"]
mod media;
#[path = "queue.rs"]
mod queue;

use kernel::{CommandIngressStatus as Status, Event, OriginalMediaWireControl as Control,
    OriginalMediaWireAck, OriginalMediaWireLoan, OriginalMediaWirePhase as Phase,
    OriginalMediaWireResult as Receipt, COMMAND_PAGE_MAXIMUM_BYTES, ORIGINAL_MEDIA_COMMAND_KIND};
use media::{NativeOriginalMediaRequest, NativeOriginalMediaTransfer};
use std::{cell::Cell, future::Future, io::{self, Read}, task::{Context, Poll, Waker}};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [bytes, trigger, operation, generation, instance, document, revision, sequence, maximum] = args.as_slice() else {
        panic!("expected fixture, trigger and issued picker fields");
    };
    let bytes = std::fs::read(bytes).unwrap();
    let instance: u32 = instance.parse().unwrap();
    let sequence: u64 = sequence.parse().unwrap();
    let maximum: u64 = maximum.parse().unwrap();
    let loan = OriginalMediaWireLoan { operation_id: operation.clone(), generation: generation.clone(),
        instance_id: instance, document_id: document.clone(), base_revision: revision.clone() };
    let wire_loan = DslValue::object([
        ("operationId".into(), DslValue::String(loan.operation_id.clone())), ("generation".into(), DslValue::String(loan.generation.clone())),
        ("instanceId".into(), DslValue::Number(semio_framework_value::Number::UInt(u64::from(instance)))), ("documentId".into(), DslValue::String(loan.document_id.clone())),
        ("baseRevision".into(), DslValue::String(loan.base_revision.clone())),
    ]);
    let request_value = DslValue::object([("originalMedia".into(), DslValue::object([
        ("loan".into(), wire_loan), ("kind".into(), DslValue::Number(semio_framework_value::Number::UInt(u64::from(ORIGINAL_MEDIA_COMMAND_KIND)))),
        ("maximumFileBytes".into(), DslValue::Number(semio_framework_value::Number::UInt(maximum))),
    ]))]);
    let request = NativeOriginalMediaRequest::parse(&request_value, instance).unwrap().unwrap();
    assert!(NativeOriginalMediaRequest::parse(&request_value, instance + 1).is_err());
    assert!(request.matches_document(instance, Some(document)));
    assert!(!request.matches_document(instance + 1, Some(document)));
    assert!(!request.matches_document(instance, Some("foreign"))); 
    assert!(!request.matches_document(instance, None));
    let owned: Option<(std::io::Cursor<Vec<u8>>, String, &str, u64)> = None;
    fn require_send(_: impl Future<Output = Result<bool, String>> + Send) {}
    require_send(NativeOriginalMediaTransfer::run(&request, sequence, Ok(owned),
        |_| std::future::ready(Ok(Status::Idle)), || true, |_, _| {}));
    let current = Cell::new(true);
    let mut source = Source { bytes: &bytes, current: &current, trigger, reads: Vec::new() };
    let selected = (trigger != "cancel-picker").then(|| (&mut source, "registered-picker.png", "image/png", bytes.len() as u64));
    let mut phases = Vec::new();
    let mut progress = Vec::new();
    let mut last = None;
    let mut reached_trigger = false;
    let mut delivered = Vec::new();
    let result = run(NativeOriginalMediaTransfer::run(&request, sequence, Ok(selected), |event| std::future::ready((|| {
        let queued = queue::QueuedOriginalMedia::admit(event).expect("production queue must admit the exact event");
        let credits = queued.credits();
        let is_poll = matches!(queued, queue::QueuedOriginalMedia::Poll);
        let mut cursor = None;
        let mut packet = None;
        let ack = match queued.into_event() {
            Event::OriginalMediaControl(event) => {
                assert_eq!(credits.0, 0);
                let phase = match event.control {
                    Control::Begin { name, media_type } => {
                        assert_eq!(name, "registered-picker.png"); assert_eq!(media_type, "image/png"); Phase::Begin
                    }
                    Control::Complete => Phase::Complete, Control::Cancel => Phase::Cancel,
                };
                OriginalMediaWireAck { loan: event.loan, owner: event.owner, seq: event.seq, kind: event.kind,
                    extent: event.extent, phase, page_index: None, result: Receipt::Accepted }
            }
            Event::CommandIngressPage { cursor: index, bytes: page } => {
                assert_eq!(credits, (1, COMMAND_PAGE_MAXIMUM_BYTES));
                assert_eq!(index.generation, u64::from_str_radix(generation, 16).unwrap());
                assert_eq!(index.instance, instance);
                assert_eq!(index.page_count, 2);
                let offset = index.page_index as usize * COMMAND_PAGE_MAXIMUM_BYTES;
                assert_eq!(page.as_slice(), &bytes[offset..(offset + COMMAND_PAGE_MAXIMUM_BYTES).min(bytes.len())]);
                let ack = OriginalMediaWireAck { loan: loan.clone(), owner: index.owner, seq: index.seq,
                    kind: index.kind, extent: index.metadata, phase: Phase::Page,
                    page_index: Some(index.page_index), result: Receipt::Accepted };
                cursor = Some(index);
                packet = Some(page);
                ack
            }
            Event::Wake => {
                assert_eq!(credits, (0, 0));
                last.take().expect("poll only follows retained input")
            }
            _ => panic!("unexpected crossing"),
        };
        assert_eq!(ack.loan, loan);
        assert_eq!((ack.owner, ack.seq, ack.kind), (sequence, sequence, ORIGINAL_MEDIA_COMMAND_KIND));
        if ack.phase != Phase::Cancel || trigger != "cancel-picker" { assert_eq!(ack.extent as usize, bytes.len()); }
        else { assert_eq!(ack.extent, 0); }
        let label = match ack.phase {
            Phase::Page => format!("page#{}", ack.page_index.unwrap()),
            Phase::Begin => "begin".to_string(), Phase::Complete => "complete".to_string(), Phase::Cancel => "cancel".to_string(),
        };
        phases.push(if is_poll { format!("poll:{label}") } else { label.clone() });
        let mut ack = ack;
        if !reached_trigger {
            match (trigger.as_str(), label.as_str()) {
                ("retry-control", "begin") | ("retry-page", "page#0") => { reached_trigger = true; ack.result = Receipt::Retry; }
                ("retry-backpressure", "page#0") => {
                    reached_trigger = true; return Ok(Status::Backpressure(cursor.unwrap()));
                }
                ("control-deferred", "begin") => {
                    reached_trigger = true; last = Some(ack); return Ok(Status::Idle);
                }
                ("pending-page", "page#0") | ("pending-complete", "complete") => {
                    reached_trigger = true; last = Some(ack.clone()); ack.result = Receipt::Pending;
                    if let Some(packet) = packet.take() { delivered.extend_from_slice(packet.as_slice()); }
                }
                ("wrong-ack", "page#0") => { reached_trigger = true; ack.loan.base_revision.pop(); ack.loan.base_revision.push('2'); }
                ("guest-stale", "page#1") => { reached_trigger = true; ack.result = Receipt::Stale; }
                _ => {}
            }
        }
        if ack.result == Receipt::Accepted && ack.loan == loan {
            if let Some(packet) = packet { delivered.extend_from_slice(packet.as_slice()); }
        }
        Ok(Status::OriginalMediaAck(ack))
    })()), || current.get(), |done, _| progress.push(done)));
    let result = match result {
        Ok(true) => { assert_eq!(delivered, bytes); "complete".to_owned() }
        Ok(false) => "cancelled".to_owned(),
        Err(error) => error,
    };
    println!("{}", result);
    println!("{}", phases.join(","));
    println!("{}", source.reads.iter().map(usize::to_string).collect::<Vec<_>>().join(","));
    println!("{}", progress.iter().map(u64::to_string).collect::<Vec<_>>().join(","));
}

struct Source<'a> {
    bytes: &'a [u8],
    current: &'a Cell<bool>,
    trigger: &'a str,
    reads: Vec<usize>,
}

impl Read for Source<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let length = self.bytes.read(buffer)?;
        if length != 0 { self.reads.push(length); }
        if matches!(self.trigger, "stale-read" | "abort-read") { self.current.set(false); }
        Ok(length)
    }
}

fn run<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("fixture exchanges are immediate"),
    }
}
