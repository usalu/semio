use super::*;

const SEQUENCE_ABI_LEDGER: &str = include_str!("../../🧫️fixtures/📊️.tsv");
const SEQUENCE_ABI_LIMITS: &str = include_str!("../../🧫️fixtures/📐️limits.tsv");
const SEQUENCE_ABI_TRACE: &str = include_str!("../../🧫️fixtures/👣️trace/📊️.tsv");

#[derive(Default)]
struct MockDomain {
    fixture: Vec<u8>,
}

impl SequenceDomain for MockDomain {
    fn execute(&mut self, operation: u16, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure> {
        match operation {
            SEQUENCE_OPERATION_LOAD_FIXTURE => {
                if payload.first() != Some(&b'{') {
                    return Err(SequenceFailure::new(AbiErrorCode::MalformedTag, "json"));
                }
                self.fixture = payload.to_vec();
                Ok(Vec::new())
            }
            SEQUENCE_OPERATION_FIXTURE => Ok(self.fixture.clone()),
            SEQUENCE_OPERATION_CATALOGUE => Ok(vec![b'x'; SEQUENCE_MAX_INLINE_REPLY_BYTES + 1]),
            SEQUENCE_OPERATION_RUN if payload == b"oversized" => Ok(vec![b'x'; SEQUENCE_MAX_OUTPUT_BYTES + 1]),
            SEQUENCE_OPERATION_RUN => Ok(br#"{"status":"ok"}"#.to_vec()),
            _ => Ok(payload.to_vec()),
        }
    }
}

fn request(operation: u16, id: u64, generation: u32, bytes: Vec<u8>) -> AbiMessage {
    AbiMessage::Request(AbiRequest { operation: semio_framework::abi::AbiOperation::try_new(operation).unwrap(), request_id: AbiRequestId(id), generation, bytes: AbiBytes::try_new(bytes).unwrap() })
}

fn open(bridge: &mut SequenceBridge<MockDomain>, id: u64) -> AbiHandle {
    bridge.try_send(request(SEQUENCE_OPERATION_OPEN, id, 1, Vec::new()), AbiWorkBudget::credits(1)).unwrap();
    let AbiPortPoll::Message(AbiMessage::Reply(reply)) = bridge.poll(AbiWorkBudget::credits(1)).unwrap() else {
        panic!("open reply");
    };
    let mut reader = SequencePayloadReader::new(reply.bytes.as_slice());
    reader.handle().unwrap()
}

fn body(handle: AbiHandle, payload: &[u8]) -> Vec<u8> {
    let mut writer = SequencePayloadWriter::default();
    writer.handle(handle);
    let mut bytes = writer.finish();
    bytes.extend_from_slice(payload);
    bytes
}

fn ack_events(bridge: &mut SequenceBridge<MockDomain>) {
    loop {
        match bridge.poll(AbiWorkBudget::credits(usize::MAX)).unwrap() {
            AbiPortPoll::Message(AbiMessage::Event(event)) => {
                bridge.try_send(AbiMessage::Reply(AbiReply { request_id: event.request_id, generation: event.generation, status: AbiStatus::OK, bytes: AbiBytes::default() }), AbiWorkBudget::credits(1)).unwrap();
            }
            AbiPortPoll::Pending if bridge.work.is_empty() => break,
            AbiPortPoll::Pending => {}
            AbiPortPoll::Message(_) => {}
            AbiPortPoll::Closed => break,
        }
    }
}

fn fingerprint(bridge: &SequenceBridge<MockDomain>) -> String {
    let requests = bridge.requests.iter().flatten().map(|entry| (entry.request_id.0, entry.generation, entry.operation.slot(), entry.operation.generation())).collect::<Vec<_>>();
    let events = bridge.events.iter().flatten().map(|entry| (entry.request_id.0, entry.generation)).collect::<Vec<_>>();
    let operations = bridge
        .requests
        .iter()
        .flatten()
        .filter_map(|entry| match bridge.resources.get(entry.operation).ok()? {
            SequenceResource::Operation(operation) => Some((
                operation.request_id.0,
                operation.generation,
                operation.cursor,
                operation.cancelled,
                operation.reader.as_ref().and_then(|reader| reader.page()).map(|page| (page.handle.slot(), page.handle.generation(), page.index, page.bytes.len())),
            )),
            SequenceResource::Session(_) => None,
        })
        .collect::<Vec<_>>();
    format!("{}:{}:{}:{}:{}:{:?}:{:?}:{:?}:{:?}:{:?}:{}", bridge.active_resources, bridge.work.len(), bridge.outbound.len(), bridge.event_count, bridge.sessions.len(), bridge.work, bridge.sessions, requests, events, operations, bridge.closing)
}

#[test]
fn schema_and_language_neutral_ledgers_are_present() {
    assert!(SEQUENCE_ABI_SCHEMA.contains("sequence.browser-abi"));
    assert!(SEQUENCE_ABI_LEDGER.lines().count() >= 5);
    assert!(SEQUENCE_ABI_LIMITS.contains("max_plus_one"));
    assert!(SEQUENCE_ABI_TRACE.contains("terminal_empty"));
}

#[test]
fn constructor_command_and_missing_optional_payload_are_deterministic() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    bridge.try_send(request(SEQUENCE_OPERATION_RUN, 2, 1, body(session, &[])), AbiWorkBudget::credits(1)).unwrap();
    ack_events(&mut bridge);
    assert_eq!(bridge.active_resources, 1);
}

#[test]
fn malformed_json_is_owned_and_terminal() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    bridge.try_send(request(SEQUENCE_OPERATION_LOAD_FIXTURE, 2, 1, body(session, b"[]")), AbiWorkBudget::credits(1)).unwrap();
    ack_events(&mut bridge);
    assert_eq!(bridge.active_resources, 1);
}

#[test]
fn interruption_and_zero_credit_do_not_advance_the_byte_cursor() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    bridge.try_send(request(SEQUENCE_OPERATION_RUN, 2, 1, body(session, b"abc")), AbiWorkBudget::credits(1)).unwrap();
    let entry = bridge.requests[request_slot(AbiRequestId(2))].unwrap();
    let before = match bridge.resources.get(entry.operation).unwrap() {
        SequenceResource::Operation(operation) => operation.cursor,
        _ => unreachable!(),
    };
    assert_eq!(bridge.poll(AbiWorkBudget { byte_credit: 1, now_ms: 0, deadline_ms: None, cancelled: false, interrupted: true }), Err(AbiErrorCode::Interrupted));
    assert_eq!(bridge.poll(AbiWorkBudget::credits(0)), Err(AbiErrorCode::NoCredit));
    let after = match bridge.resources.get(entry.operation).unwrap() {
        SequenceResource::Operation(operation) => operation.cursor,
        _ => unreachable!(),
    };
    assert_eq!(before, after);
}

#[test]
fn unacknowledged_event_does_not_advance_the_byte_cursor() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    bridge.try_send(request(SEQUENCE_OPERATION_RUN, 2, 1, body(session, b"abc")), AbiWorkBudget::credits(1)).unwrap();
    let entry = bridge.requests[request_slot(AbiRequestId(2))].unwrap();
    let AbiPortPoll::Message(AbiMessage::Event(_)) = bridge.poll(AbiWorkBudget::credits(1)).unwrap() else {
        panic!("admitted event");
    };
    assert_eq!(bridge.poll(AbiWorkBudget::credits(1)).unwrap(), AbiPortPoll::Pending);
    let SequenceResource::Operation(operation) = bridge.resources.get(entry.operation).unwrap() else { unreachable!() };
    assert_eq!(operation.cursor, 0);
}

#[test]
fn cancel_during_compute_and_close_are_terminal_empty() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    bridge.try_send(request(SEQUENCE_OPERATION_RUN, 2, 1, body(session, &[7; 32])), AbiWorkBudget::credits(1)).unwrap();
    bridge.try_send(AbiMessage::Control(AbiControl::Cancel { request_id: AbiRequestId(2), generation: 1 }), AbiWorkBudget::credits(1)).unwrap();
    ack_events(&mut bridge);
    bridge.close_session(session).unwrap();
    bridge.begin_close();
    while !bridge.terminal_is_empty() {
        match bridge.poll(AbiWorkBudget::credits(64)).unwrap() {
            AbiPortPoll::Message(AbiMessage::Event(event)) => {
                bridge.try_send(AbiMessage::Reply(AbiReply { request_id: event.request_id, generation: event.generation, status: AbiStatus::OK, bytes: AbiBytes::default() }), AbiWorkBudget::credits(1)).unwrap();
            }
            AbiPortPoll::Message(_) | AbiPortPoll::Pending | AbiPortPoll::Closed => {}
        }
    }
    assert!(bridge.terminal_is_empty());
}

#[test]
fn stale_future_and_aba_session_handles_are_distinct() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let first = open(&mut bridge, 1);
    bridge.close_session(first).unwrap();
    let second = open(&mut bridge, 2);
    assert_eq!(first.slot(), second.slot());
    assert!(second.generation() > first.generation());
    let old = bridge.try_send(request(SEQUENCE_OPERATION_RUN, 3, 1, body(first, &[])), AbiWorkBudget::credits(1)).unwrap_err();
    assert_eq!(old.code, AbiErrorCode::AbaHandle);
    let future = AbiHandle::try_new(second.slot(), second.generation() + 1).unwrap();
    let future = bridge.try_send(request(SEQUENCE_OPERATION_RUN, 4, 1, body(future, &[])), AbiWorkBudget::credits(1)).unwrap_err();
    assert_eq!(future.code, AbiErrorCode::StaleGeneration);
}

#[test]
fn missing_canvas_and_surface_are_rejected() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    bridge.try_send(request(SEQUENCE_OPERATION_RENDER_FRAME, 2, 1, body(session, &[])), AbiWorkBudget::credits(1)).unwrap();
    ack_events(&mut bridge);
    assert_eq!(bridge.active_resources, 1);
}

#[test]
fn paged_output_requires_exact_ack_and_rejects_duplicate() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    bridge.try_send(request(SEQUENCE_OPERATION_CATALOGUE, 2, 1, body(session, &[])), AbiWorkBudget::credits(1)).unwrap();
    let mut operation = None;
    let mut page = None;
    for _ in 0..32 {
        match bridge.poll(AbiWorkBudget::credits(usize::MAX)).unwrap() {
            AbiPortPoll::Message(AbiMessage::Event(event)) => {
                if event.event.get() == SEQUENCE_EVENT_ADMITTED {
                    operation = Some(SequencePayloadReader::new(event.bytes.as_slice()).handle().unwrap());
                }
                bridge.try_send(AbiMessage::Reply(AbiReply { request_id: event.request_id, generation: event.generation, status: AbiStatus::OK, bytes: AbiBytes::default() }), AbiWorkBudget::credits(1)).unwrap();
            }
            AbiPortPoll::Message(AbiMessage::Page(value)) => {
                page = Some(value);
                break;
            }
            _ => {}
        }
    }
    let page = page.expect("page");
    assert_eq!(page.handle, operation.unwrap());
    let ack = AbiControl::Acknowledge { handle: page.handle, index: page.index };
    bridge.try_send(AbiMessage::Control(ack), AbiWorkBudget::credits(1)).unwrap();
    assert_eq!(bridge.try_send(AbiMessage::Control(ack), AbiWorkBudget::credits(1)).unwrap_err().code, AbiErrorCode::DuplicateAcknowledgement);
}

#[test]
fn rejected_cancel_event_ack_and_close_preserve_every_ledger_then_valid_controls_progress() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    bridge.try_send(request(SEQUENCE_OPERATION_RUN, 2, 1, body(session, b"abc")), AbiWorkBudget::credits(1)).unwrap();
    let before_cancel = fingerprint(&bridge);
    for (request_id, generation, code) in [(AbiRequestId(999), 1, AbiErrorCode::UnknownHandle), (AbiRequestId(2), 0, AbiErrorCode::AbaHandle), (AbiRequestId(2), 2, AbiErrorCode::StaleGeneration)] {
        let rejection = bridge.try_send(AbiMessage::Control(AbiControl::Cancel { request_id, generation }), AbiWorkBudget::credits(1)).unwrap_err();
        assert_eq!(rejection.code, code);
        assert_eq!(fingerprint(&bridge), before_cancel);
    }
    let malformed = AbiMessage::Event(AbiEvent { request_id: AbiRequestId(999), generation: 1, sequence: 1, event: AbiEventCode::try_new(SEQUENCE_EVENT_PROGRESS).unwrap(), status: AbiStatus::OK, bytes: AbiBytes::default() });
    assert_eq!(bridge.try_send(malformed, AbiWorkBudget::credits(1)).unwrap_err().code, AbiErrorCode::MalformedTag);
    assert_eq!(fingerprint(&bridge), before_cancel);

    bridge.try_send(AbiMessage::Control(AbiControl::Cancel { request_id: AbiRequestId(2), generation: 1 }), AbiWorkBudget::credits(1)).unwrap();
    let cancelled = fingerprint(&bridge);
    assert_eq!(bridge.try_send(AbiMessage::Control(AbiControl::Cancel { request_id: AbiRequestId(2), generation: 1 }), AbiWorkBudget::credits(1)).unwrap_err().code, AbiErrorCode::Cancelled);
    assert_eq!(fingerprint(&bridge), cancelled);
    ack_events(&mut bridge);

    let close_before = fingerprint(&bridge);
    for (handle, code) in [(AbiHandle::try_new(99, session.generation()).unwrap(), AbiErrorCode::UnknownHandle), (AbiHandle::try_new(session.slot(), session.generation() + 1).unwrap(), AbiErrorCode::StaleGeneration)] {
        assert_eq!(bridge.try_send(AbiMessage::Control(AbiControl::Close { handle }), AbiWorkBudget::credits(1)).unwrap_err().code, code);
        assert_eq!(fingerprint(&bridge), close_before);
    }
    bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: session }), AbiWorkBudget::credits(1)).unwrap();
    let closed = fingerprint(&bridge);
    assert_eq!(bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: session }), AbiWorkBudget::credits(1)).unwrap_err().code, AbiErrorCode::UnknownHandle);
    assert_eq!(fingerprint(&bridge), closed);

    let reused = open(&mut bridge, 3);
    let reuse_before = fingerprint(&bridge);
    assert_eq!(bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: session }), AbiWorkBudget::credits(1)).unwrap_err().code, AbiErrorCode::AbaHandle);
    assert_eq!(fingerprint(&bridge), reuse_before);
    bridge.begin_close();
    let closing = fingerprint(&bridge);
    assert_eq!(bridge.try_send(AbiMessage::Control(AbiControl::Close { handle: reused }), AbiWorkBudget::credits(1)).unwrap_err().code, AbiErrorCode::Closed);
    assert_eq!(fingerprint(&bridge), closing);
}

#[test]
fn rejected_page_and_event_acknowledgements_preserve_retained_ownership_then_valid_ack_progresses() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    bridge.try_send(request(SEQUENCE_OPERATION_CATALOGUE, 2, 1, body(session, &[])), AbiWorkBudget::credits(1)).unwrap();
    let event = loop {
        let AbiPortPoll::Message(message) = bridge.poll(AbiWorkBudget::credits(usize::MAX)).unwrap() else { continue };
        if let AbiMessage::Event(event) = message {
            break event;
        }
    };
    let event_before = fingerprint(&bridge);
    for (request_id, generation) in [(event.request_id, event.generation + 1), (AbiRequestId(event.request_id.0 + 99), event.generation)] {
        let reply = AbiReply { request_id, generation, status: AbiStatus::OK, bytes: AbiBytes::default() };
        assert_eq!(bridge.try_send(AbiMessage::Reply(reply), AbiWorkBudget::credits(1)).unwrap_err().code, AbiErrorCode::LateReply);
        assert_eq!(fingerprint(&bridge), event_before);
    }
    let acknowledgement = AbiReply { request_id: event.request_id, generation: event.generation, status: AbiStatus::OK, bytes: AbiBytes::default() };
    bridge.try_send(AbiMessage::Reply(acknowledgement.clone()), AbiWorkBudget::credits(1)).unwrap();
    let event_acked = fingerprint(&bridge);
    assert_eq!(bridge.try_send(AbiMessage::Reply(acknowledgement), AbiWorkBudget::credits(1)).unwrap_err().code, AbiErrorCode::LateReply);
    assert_eq!(fingerprint(&bridge), event_acked);

    let page = loop {
        match bridge.poll(AbiWorkBudget::credits(usize::MAX)).unwrap() {
            AbiPortPoll::Message(AbiMessage::Event(event)) => {
                bridge.try_send(AbiMessage::Reply(AbiReply { request_id: event.request_id, generation: event.generation, status: AbiStatus::OK, bytes: AbiBytes::default() }), AbiWorkBudget::credits(1)).unwrap();
            }
            AbiPortPoll::Message(AbiMessage::Page(page)) => break page,
            AbiPortPoll::Message(_) | AbiPortPoll::Pending | AbiPortPoll::Closed => {}
        }
    };
    let page_before = fingerprint(&bridge);
    for (handle, index, code) in [
        (AbiHandle::try_new(page.handle.slot() + 1, page.handle.generation()).unwrap(), page.index, AbiErrorCode::UnknownHandle),
        (AbiHandle::try_new(page.handle.slot(), page.handle.generation() + 1).unwrap(), page.index, AbiErrorCode::StaleGeneration),
        (page.handle, page.index + 1, AbiErrorCode::OutOfOrderPage),
        (page.handle, u32::MAX, AbiErrorCode::OutOfOrderPage),
    ] {
        let control = AbiControl::Acknowledge { handle, index };
        assert_eq!(bridge.try_send(AbiMessage::Control(control), AbiWorkBudget::credits(1)).unwrap_err().code, code);
        assert_eq!(fingerprint(&bridge), page_before);
    }
    let valid = AbiControl::Acknowledge { handle: page.handle, index: page.index };
    bridge.try_send(AbiMessage::Control(valid), AbiWorkBudget::credits(1)).unwrap();
    let page_acked = fingerprint(&bridge);
    assert_eq!(bridge.try_send(AbiMessage::Control(valid), AbiWorkBudget::credits(1)).unwrap_err().code, AbiErrorCode::DuplicateAcknowledgement);
    assert_eq!(fingerprint(&bridge), page_acked);
    ack_events(&mut bridge);
    assert_eq!(bridge.active_resources, 1);
}

#[test]
fn event_and_output_max_plus_one_are_pre_admission() {
    let mut events = SequenceBridge::new(MockDomain::default);
    assert_eq!(events.event_count, 0);
    for id in 0..SEQUENCE_MAX_EVENTS_IN_FLIGHT as u64 {
        events.push_event(AbiRequestId(id), 1, SEQUENCE_EVENT_PROGRESS, AbiStatus::OK, Vec::new()).unwrap();
    }
    assert_eq!(events.event_count, SEQUENCE_MAX_EVENTS_IN_FLIGHT);
    assert_eq!(events.push_event(AbiRequestId(65), 1, SEQUENCE_EVENT_PROGRESS, AbiStatus::OK, Vec::new()), Err(AbiErrorCode::LimitExceeded));

    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    bridge.try_send(request(SEQUENCE_OPERATION_RUN, 2, 1, body(session, b"oversized")), AbiWorkBudget::credits(1)).unwrap();
    ack_events(&mut bridge);
    assert_eq!(bridge.active_resources, 1);
}

#[test]
fn request_and_resource_max_plus_one_are_pre_admission() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let mut sessions = Vec::new();
    for id in 1..=SEQUENCE_MAX_RESOURCES as u64 {
        sessions.push(open(&mut bridge, id));
    }
    let rejected = bridge.try_send(request(SEQUENCE_OPERATION_OPEN, 65, 1, Vec::new()), AbiWorkBudget::credits(1)).unwrap_err();
    assert_eq!(rejected.code, AbiErrorCode::LimitExceeded);
    assert_eq!(bridge.active_resources, SEQUENCE_MAX_RESOURCES);
    for session in sessions {
        bridge.close_session(session).unwrap();
    }
}

#[test]
fn playback_transitions_and_output_are_deterministic() {
    let mut bridge = SequenceBridge::new(MockDomain::default);
    let session = open(&mut bridge, 1);
    for (index, operation) in [SEQUENCE_OPERATION_PLAY, SEQUENCE_OPERATION_PAUSE, SEQUENCE_OPERATION_STOP].into_iter().enumerate() {
        bridge.try_send(request(operation, index as u64 + 2, 1, body(session, &[])), AbiWorkBudget::credits(1)).unwrap();
        ack_events(&mut bridge);
    }
    let SequenceResource::Session(state) = bridge.resources.get(session).unwrap() else { unreachable!() };
    assert_eq!(state.borrow().playback, PlaybackState::Stopped);
}
