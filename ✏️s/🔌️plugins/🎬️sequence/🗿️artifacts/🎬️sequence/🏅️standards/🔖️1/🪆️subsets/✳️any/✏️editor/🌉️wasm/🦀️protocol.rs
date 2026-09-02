//! 🧬️ Dependency-free Sequence editor protocol over the owned framework ABI.

use crate::semio_framework::abi::{
    ABI_MAX_BODY_BYTES, ABI_MAX_IN_FLIGHT_HANDLES, ABI_MAX_IN_FLIGHT_REQUESTS, ABI_MAX_MESSAGE_BYTES, ABI_MAX_TRANSFER_BYTES, AbiBytes, AbiControl, AbiError, AbiErrorCode, AbiEvent, AbiEventCode, AbiHandle, AbiHandleTable, AbiMessage,
    AbiMessageBytes, AbiOperation, AbiPageReader, AbiPort, AbiPortPoll, AbiPortRejection, AbiReply, AbiReplyLedger, AbiRequest, AbiRequestId, AbiStatus, AbiStatusCode, AbiWorkBudget,
};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

//#region 🧬️Contract

pub const SEQUENCE_ABI_SCHEMA: &str = include_str!("🧬️schema/🔣️.json");
pub const SEQUENCE_ABI_LEDGER: &str = include_str!("🧪️fixtures/📊️.tsv");
pub const SEQUENCE_ABI_LIMITS: &str = include_str!("🧪️fixtures/📐️limits.tsv");
pub const SEQUENCE_ABI_TRACE: &str = include_str!("🧪️fixtures/🧪️trace/📊️.tsv");

pub const SEQUENCE_OPERATION_OPEN: u16 = 2_300;
pub const SEQUENCE_OPERATION_LOAD_FIXTURE: u16 = 2_301;
pub const SEQUENCE_OPERATION_FIXTURE: u16 = 2_302;
pub const SEQUENCE_OPERATION_CATALOGUE: u16 = 2_303;
pub const SEQUENCE_OPERATION_ADD_STEP: u16 = 2_304;
pub const SEQUENCE_OPERATION_ADD_STEP_DROPPED: u16 = 2_305;
pub const SEQUENCE_OPERATION_ADD_STEP_TO_SLOT: u16 = 2_306;
pub const SEQUENCE_OPERATION_SET_STEP_COLLAPSED: u16 = 2_307;
pub const SEQUENCE_OPERATION_PICK_STEP: u16 = 2_308;
pub const SEQUENCE_OPERATION_BUILD_PATH: u16 = 2_309;
pub const SEQUENCE_OPERATION_REMOVE_STEP: u16 = 2_310;
pub const SEQUENCE_OPERATION_SET_STEP_PARAMS: u16 = 2_311;
pub const SEQUENCE_OPERATION_CONNECT_STEPS: u16 = 2_312;
pub const SEQUENCE_OPERATION_DISCONNECT_STEPS: u16 = 2_313;
pub const SEQUENCE_OPERATION_COMPILE_TEXT: u16 = 2_314;
pub const SEQUENCE_OPERATION_COMPILED_WIRE: u16 = 2_315;
pub const SEQUENCE_OPERATION_RUN: u16 = 2_316;
pub const SEQUENCE_OPERATION_ATTACH_SURFACE: u16 = 2_317;
pub const SEQUENCE_OPERATION_GPU_READY: u16 = 2_318;
pub const SEQUENCE_OPERATION_SET_SIZE: u16 = 2_319;
pub const SEQUENCE_OPERATION_RENDER_FRAME: u16 = 2_320;
pub const SEQUENCE_OPERATION_WORLD_FROM_SCREEN: u16 = 2_321;
pub const SEQUENCE_OPERATION_POINTER_DOWN: u16 = 2_322;
pub const SEQUENCE_OPERATION_POINTER_MOVE: u16 = 2_323;
pub const SEQUENCE_OPERATION_POINTER_UP: u16 = 2_324;
pub const SEQUENCE_OPERATION_WHEEL: u16 = 2_325;
pub const SEQUENCE_OPERATION_REORGANIZE: u16 = 2_326;
pub const SEQUENCE_OPERATION_LOD_SCALE: u16 = 2_327;
pub const SEQUENCE_OPERATION_SET_AUTOMATIC_LOD: u16 = 2_328;
pub const SEQUENCE_OPERATION_SET_FORCED_LOD: u16 = 2_329;
pub const SEQUENCE_OPERATION_DRAW_LOD: u16 = 2_330;
pub const SEQUENCE_OPERATION_SET_THEME: u16 = 2_331;
pub const SEQUENCE_OPERATION_SELECTED_NODES: u16 = 2_332;
pub const SEQUENCE_OPERATION_SET_SELECTION: u16 = 2_333;
pub const SEQUENCE_OPERATION_LABEL_OVERLAY: u16 = 2_334;
pub const SEQUENCE_OPERATION_HOVERED_NODE: u16 = 2_335;
pub const SEQUENCE_OPERATION_PRESELECT_NODES: u16 = 2_336;
pub const SEQUENCE_OPERATION_SELECTION_PREVIEW_POINTS: u16 = 2_337;
pub const SEQUENCE_OPERATION_SELECTION_PREVIEW_CROSSING: u16 = 2_338;
pub const SEQUENCE_OPERATION_SELECTION_PREVIEW_METHOD: u16 = 2_339;
pub const SEQUENCE_OPERATION_SELECTION_BOUNDS: u16 = 2_340;
pub const SEQUENCE_OPERATION_SET_SELECTION_OPTIONS: u16 = 2_341;
pub const SEQUENCE_OPERATION_SET_GHOST_STEP: u16 = 2_342;
pub const SEQUENCE_OPERATION_CLEAR_GHOST_STEP: u16 = 2_343;
pub const SEQUENCE_OPERATION_PLAY: u16 = 2_344;
pub const SEQUENCE_OPERATION_PAUSE: u16 = 2_345;
pub const SEQUENCE_OPERATION_STOP: u16 = 2_346;

pub const SEQUENCE_EVENT_ADMITTED: u16 = 2_400;
pub const SEQUENCE_EVENT_PROGRESS: u16 = 2_401;
pub const SEQUENCE_EVENT_CHECKPOINT: u16 = 2_402;
pub const SEQUENCE_EVENT_SURFACE: u16 = 2_403;
pub const SEQUENCE_EVENT_RENDER: u16 = 2_404;
pub const SEQUENCE_EVENT_PLAYBACK: u16 = 2_405;
pub const SEQUENCE_EVENT_OUTPUT: u16 = 2_406;
pub const SEQUENCE_EVENT_TERMINAL: u16 = 2_407;

pub const SEQUENCE_MAX_REQUEST_BYTES: usize = ABI_MAX_BODY_BYTES;
pub const SEQUENCE_MAX_OUTPUT_BYTES: usize = ABI_MAX_TRANSFER_BYTES;
pub const SEQUENCE_MAX_INLINE_REPLY_BYTES: usize = ABI_MAX_MESSAGE_BYTES;
pub const SEQUENCE_MAX_RESOURCES: usize = ABI_MAX_IN_FLIGHT_HANDLES;
pub const SEQUENCE_MAX_REQUESTS: usize = ABI_MAX_IN_FLIGHT_REQUESTS;
pub const SEQUENCE_MAX_OUTBOUND: usize = 64;
pub const SEQUENCE_MAX_EVENTS_IN_FLIGHT: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceId(u32);

impl SurfaceId {
    pub fn try_new(value: u32) -> Result<Self, AbiErrorCode> {
        (value != 0).then_some(Self(value)).ok_or(AbiErrorCode::UnknownHandle)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanvasId(u32);

impl CanvasId {
    pub fn try_new(value: u32) -> Result<Self, AbiErrorCode> {
        (value != 0).then_some(Self(value)).ok_or(AbiErrorCode::UnknownHandle)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SequenceFailure {
    pub code: AbiErrorCode,
    pub message: String,
}

impl SequenceFailure {
    pub fn new(code: AbiErrorCode, message: impl Into<String>) -> Self {
        Self { code, message: message.into() }
    }
}

pub trait SequenceDomain {
    fn execute(&mut self, operation: u16, payload: &[u8]) -> Result<Vec<u8>, SequenceFailure>;
}

//#endregion 🧬️Contract

//#region 🧱️PayloadCodec

pub struct SequencePayloadReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> SequencePayloadReader<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    pub fn u8(&mut self) -> Result<u8, AbiErrorCode> {
        Ok(self.take(1)?[0])
    }

    pub fn u32(&mut self) -> Result<u32, AbiErrorCode> {
        let bytes: [u8; 4] = self.take(4)?.try_into().map_err(|_| AbiErrorCode::MalformedLength)?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn f64(&mut self) -> Result<f64, AbiErrorCode> {
        let bytes: [u8; 8] = self.take(8)?.try_into().map_err(|_| AbiErrorCode::MalformedLength)?;
        let value = f64::from_le_bytes(bytes);
        value.is_finite().then_some(value).ok_or(AbiErrorCode::MalformedTag)
    }

    pub fn bytes(&mut self) -> Result<&'a [u8], AbiErrorCode> {
        let length = self.u32()? as usize;
        self.take(length)
    }

    pub fn text(&mut self) -> Result<&'a str, AbiErrorCode> {
        std::str::from_utf8(self.bytes()?).map_err(|_| AbiErrorCode::InvalidUtf8)
    }

    pub fn optional_text(&mut self) -> Result<Option<&'a str>, AbiErrorCode> {
        match self.u8()? {
            0 => Ok(None),
            1 => self.text().map(Some),
            _ => Err(AbiErrorCode::MalformedTag),
        }
    }

    pub fn handle(&mut self) -> Result<AbiHandle, AbiErrorCode> {
        AbiHandle::try_new(self.u32()?, self.u32()?)
    }

    pub fn finish(self) -> Result<(), AbiErrorCode> {
        (self.cursor == self.bytes.len()).then_some(()).ok_or(AbiErrorCode::MalformedLength)
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], AbiErrorCode> {
        let end = self.cursor.checked_add(length).ok_or(AbiErrorCode::MalformedLength)?;
        let value = self.bytes.get(self.cursor..end).ok_or(AbiErrorCode::MalformedLength)?;
        self.cursor = end;
        Ok(value)
    }
}

#[derive(Default)]
pub struct SequencePayloadWriter {
    bytes: Vec<u8>,
}

impl SequencePayloadWriter {
    pub fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn handle(&mut self, value: AbiHandle) {
        self.u32(value.slot());
        self.u32(value.generation());
    }

    pub fn bytes(&mut self, value: &[u8]) -> Result<(), AbiErrorCode> {
        let length = u32::try_from(value.len()).map_err(|_| AbiErrorCode::LimitExceeded)?;
        self.u32(length);
        self.bytes.extend_from_slice(value);
        Ok(())
    }

    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

//#endregion 🧱️PayloadCodec

//#region 🔖️Runtime

struct SequenceSession<D> {
    domain: D,
    surface: Option<(SurfaceId, CanvasId)>,
    playback: PlaybackState,
    closed: bool,
}

enum OperationPhase {
    Inspect,
    Checkpoint,
    Execute,
    Stream,
    Terminal,
}

struct SequenceOperation<D> {
    session: Rc<RefCell<SequenceSession<D>>>,
    session_handle: AbiHandle,
    request_id: AbiRequestId,
    generation: u32,
    operation: u16,
    payload: Vec<u8>,
    cursor: usize,
    reader: Option<AbiPageReader>,
    phase: OperationPhase,
    cancelled: bool,
}

enum SequenceResource<D> {
    Session(Rc<RefCell<SequenceSession<D>>>),
    Operation(SequenceOperation<D>),
}

#[derive(Clone, Copy)]
struct RequestEntry {
    request_id: AbiRequestId,
    generation: u32,
    operation: AbiHandle,
}

#[derive(Clone, Copy)]
struct EventEntry {
    request_id: AbiRequestId,
    generation: u32,
}

pub struct SequenceBridge<D: SequenceDomain> {
    factory: fn() -> D,
    resources: AbiHandleTable<SequenceResource<D>>,
    work: VecDeque<AbiHandle>,
    outbound: VecDeque<AbiMessage>,
    request_ledger: AbiReplyLedger,
    requests: [Option<RequestEntry>; SEQUENCE_MAX_REQUESTS],
    events: [Option<EventEntry>; SEQUENCE_MAX_REQUESTS],
    sessions: Vec<AbiHandle>,
    event_count: usize,
    active_resources: usize,
    next_event_sequence: u32,
    closing: bool,
}

impl<D: SequenceDomain> SequenceBridge<D> {
    pub fn new(factory: fn() -> D) -> Self {
        Self {
            factory,
            resources: AbiHandleTable::new(),
            work: VecDeque::new(),
            outbound: VecDeque::new(),
            request_ledger: AbiReplyLedger::new(),
            requests: [None; SEQUENCE_MAX_REQUESTS],
            events: [None; SEQUENCE_MAX_REQUESTS],
            sessions: Vec::new(),
            event_count: 0,
            active_resources: 0,
            next_event_sequence: 1,
            closing: false,
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.active_resources == 0 && self.work.is_empty() && self.outbound.is_empty() && self.event_count == 0
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        for entry in self.requests.iter().flatten() {
            if let Ok(SequenceResource::Operation(operation)) = self.resources.get_mut(entry.operation) {
                operation.cancelled = true;
            }
        }
        let sessions = std::mem::take(&mut self.sessions);
        for session in sessions {
            let _ = self.close_session(session);
        }
    }

    fn accept_request(&mut self, request: AbiRequest) -> Result<(), AbiPortRejection> {
        let message = || AbiMessage::Request(request.clone());
        if self.closing {
            return Err(AbiPortRejection { code: AbiErrorCode::Closed, message: message() });
        }
        if request.generation == 0 || request.bytes.len() > SEQUENCE_MAX_REQUEST_BYTES {
            return Err(AbiPortRejection { code: AbiErrorCode::LimitExceeded, message: message() });
        }
        if request.operation.get() == SEQUENCE_OPERATION_OPEN {
            if !request.bytes.is_empty() {
                return Err(AbiPortRejection { code: AbiErrorCode::MalformedLength, message: message() });
            }
            if self.active_resources == SEQUENCE_MAX_RESOURCES || self.outbound.len() == SEQUENCE_MAX_OUTBOUND {
                return Err(AbiPortRejection { code: AbiErrorCode::LimitExceeded, message: message() });
            }
            self.request_ledger.admit(request.request_id, request.generation).map_err(|code| AbiPortRejection { code, message: message() })?;
            let session = Rc::new(RefCell::new(SequenceSession { domain: (self.factory)(), surface: None, playback: PlaybackState::Stopped, closed: false }));
            let handle = self.resources.open(SequenceResource::Session(session)).map_err(|(code, _)| AbiPortRejection { code, message: message() })?;
            self.active_resources += 1;
            self.sessions.push(handle);
            let mut writer = SequencePayloadWriter::default();
            writer.handle(handle);
            self.push_reply(success_reply(request.request_id, request.generation, writer.finish())).map_err(|code| AbiPortRejection { code, message: message() })?;
            self.request_ledger.accept(&success_reply(request.request_id, request.generation, Vec::new())).map_err(|code| AbiPortRejection { code, message: message() })?;
            return Ok(());
        }
        let mut reader = SequencePayloadReader::new(request.bytes.as_slice());
        let session_handle = reader.handle().map_err(|code| AbiPortRejection { code, message: message() })?;
        let payload = request.bytes.as_slice()[8..].to_vec();
        let session = match self.resources.get(session_handle) {
            Ok(SequenceResource::Session(session)) if !session.borrow().closed => session.clone(),
            Ok(SequenceResource::Session(_)) => return Err(AbiPortRejection { code: AbiErrorCode::Closed, message: message() }),
            Ok(SequenceResource::Operation(_)) => return Err(AbiPortRejection { code: AbiErrorCode::UnknownHandle, message: message() }),
            Err(code) => return Err(AbiPortRejection { code, message: message() }),
        };
        if self.active_resources == SEQUENCE_MAX_RESOURCES || self.outbound.len() == SEQUENCE_MAX_OUTBOUND || self.event_count == SEQUENCE_MAX_EVENTS_IN_FLIGHT {
            return Err(AbiPortRejection { code: AbiErrorCode::LimitExceeded, message: message() });
        }
        let event_request = AbiRequestId(request.request_id.0 ^ ((self.next_event_sequence as u64) << 32));
        if self.events[request_slot(event_request)].is_some() {
            return Err(AbiPortRejection { code: AbiErrorCode::Busy, message: message() });
        }
        self.next_event_sequence.checked_add(1).ok_or_else(|| AbiPortRejection { code: AbiErrorCode::GenerationExhausted, message: message() })?;
        self.request_ledger.admit(request.request_id, request.generation).map_err(|code| AbiPortRejection { code, message: message() })?;
        let index = request_slot(request.request_id);
        let operation =
            SequenceOperation { session, session_handle, request_id: request.request_id, generation: request.generation, operation: request.operation.get(), payload, cursor: 0, reader: None, phase: OperationPhase::Inspect, cancelled: false };
        let handle = self.resources.open(SequenceResource::Operation(operation)).map_err(|(code, _)| AbiPortRejection { code, message: message() })?;
        self.active_resources += 1;
        self.requests[index] = Some(RequestEntry { request_id: request.request_id, generation: request.generation, operation: handle });
        self.work.push_back(handle);
        let mut body = SequencePayloadWriter::default();
        body.handle(handle);
        self.push_event(request.request_id, request.generation, SEQUENCE_EVENT_ADMITTED, AbiStatus::OK, body.finish()).map_err(|code| AbiPortRejection { code, message: message() })
    }

    fn accept_control(&mut self, control: AbiControl) -> Result<(), AbiErrorCode> {
        match control {
            AbiControl::Cancel { request_id, generation } => {
                let entry = self.requests[request_slot(request_id)].ok_or(AbiErrorCode::UnknownHandle)?;
                if entry.request_id != request_id {
                    return Err(AbiErrorCode::UnknownHandle);
                }
                if generation < entry.generation {
                    return Err(AbiErrorCode::AbaHandle);
                }
                if generation > entry.generation {
                    return Err(AbiErrorCode::StaleGeneration);
                }
                let SequenceResource::Operation(operation) = self.resources.get_mut(entry.operation)? else {
                    return Err(AbiErrorCode::UnknownHandle);
                };
                if operation.cancelled {
                    return Err(AbiErrorCode::Cancelled);
                }
                operation.cancelled = true;
                if let Some(reader) = operation.reader.as_mut() {
                    reader.cancel();
                }
                self.outbound.retain(|message| !matches!(message, AbiMessage::Page(page) if page.handle == entry.operation));
                Ok(())
            }
            AbiControl::Acknowledge { handle, index } => {
                let SequenceResource::Operation(operation) = self.resources.get_mut(handle)? else {
                    return Err(AbiErrorCode::UnknownHandle);
                };
                operation.reader.as_mut().ok_or(AbiErrorCode::UnknownHandle)?.acknowledge(AbiControl::Acknowledge { handle, index })
            }
            AbiControl::Close { handle } => {
                if self.closing {
                    return Err(AbiErrorCode::Closed);
                }
                if matches!(self.resources.get(handle)?, SequenceResource::Session(_)) {
                    self.close_session(handle)?;
                } else {
                    let SequenceResource::Operation(operation) = self.resources.get_mut(handle)? else {
                        return Err(AbiErrorCode::UnknownHandle);
                    };
                    if operation.cancelled {
                        return Err(AbiErrorCode::Cancelled);
                    }
                    operation.cancelled = true;
                }
                Ok(())
            }
        }
    }

    fn accept_event_ack(&mut self, reply: AbiReply) -> Result<(), AbiErrorCode> {
        let index = request_slot(reply.request_id);
        let entry = self.events[index].ok_or(AbiErrorCode::LateReply)?;
        if entry.request_id != reply.request_id || entry.generation != reply.generation {
            return Err(AbiErrorCode::LateReply);
        }
        self.events[index] = None;
        self.event_count -= 1;
        Ok(())
    }

    fn advance(&mut self, budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
        validate_budget(budget)?;
        let Some(handle) = self.work.pop_front() else {
            return Ok(());
        };
        let request_id = match self.resources.get(handle)? {
            SequenceResource::Operation(operation) => operation.request_id,
            SequenceResource::Session(_) => return Err(AbiErrorCode::UnknownHandle),
        };
        let next_event_request = AbiRequestId(request_id.0 ^ ((self.next_event_sequence as u64) << 32));
        if self.event_count == SEQUENCE_MAX_EVENTS_IN_FLIGHT || self.events[request_slot(next_event_request)].is_some() {
            self.work.push_back(handle);
            return Ok(());
        }
        let mut retain = true;
        let mut event = None;
        let mut page = None;
        let mut reply = None;
        {
            let SequenceResource::Operation(operation) = self.resources.get_mut(handle)? else {
                return Err(AbiErrorCode::UnknownHandle);
            };
            if operation.cancelled || operation.session.borrow().closed {
                if let Some(reader) = operation.reader.as_mut() {
                    let _ = reader.close_step(budget);
                    if !reader.terminal_is_empty() {
                        self.work.push_back(handle);
                        return Ok(());
                    }
                }
                reply = Some(failure_reply(operation.request_id, operation.generation, AbiStatusCode::Cancelled, AbiErrorCode::Cancelled, "cancelled"));
                operation.phase = OperationPhase::Terminal;
                retain = false;
            } else {
                match operation.phase {
                    OperationPhase::Inspect => {
                        let remaining = operation.payload.len().saturating_sub(operation.cursor);
                        if remaining == 0 {
                            operation.phase = OperationPhase::Checkpoint;
                        } else {
                            let admitted = admitted_bytes(budget, remaining)?;
                            operation.cursor += admitted;
                            let mut body = SequencePayloadWriter::default();
                            body.handle(handle);
                            body.u64(operation.cursor as u64);
                            body.u64(operation.payload.len() as u64);
                            event = Some((operation.request_id, operation.generation, SEQUENCE_EVENT_PROGRESS, AbiStatus::OK, body.finish()));
                            if operation.cursor == operation.payload.len() {
                                operation.phase = OperationPhase::Checkpoint;
                            }
                        }
                    }
                    OperationPhase::Checkpoint => {
                        let mut body = SequencePayloadWriter::default();
                        body.handle(handle);
                        body.u64(operation.cursor as u64);
                        event = Some((operation.request_id, operation.generation, SEQUENCE_EVENT_CHECKPOINT, AbiStatus::OK, body.finish()));
                        operation.phase = OperationPhase::Execute;
                    }
                    OperationPhase::Execute => match execute_operation(operation) {
                        Err(code) => {
                            reply = Some(failure_reply(operation.request_id, operation.generation, AbiStatusCode::Failed, code, "operation failed"));
                            operation.phase = OperationPhase::Terminal;
                            retain = false;
                        }
                        Ok(output) if output.len() > SEQUENCE_MAX_OUTPUT_BYTES => {
                            reply = Some(failure_reply(operation.request_id, operation.generation, AbiStatusCode::Rejected, AbiErrorCode::LimitExceeded, "output limit"));
                            operation.phase = OperationPhase::Terminal;
                            retain = false;
                        }
                        Ok(output) if output.len() <= SEQUENCE_MAX_INLINE_REPLY_BYTES => {
                            reply = Some(success_reply(operation.request_id, operation.generation, output));
                            operation.phase = OperationPhase::Terminal;
                            retain = false;
                        }
                        Ok(output) => {
                            let total = output.len();
                            operation.reader = Some(AbiPageReader::try_new(handle, output).map_err(|rejected| rejected.code)?);
                            operation.phase = OperationPhase::Stream;
                            let mut body = SequencePayloadWriter::default();
                            body.handle(handle);
                            body.u64(total as u64);
                            event = Some((operation.request_id, operation.generation, SEQUENCE_EVENT_OUTPUT, AbiStatus::OK, body.finish()));
                        }
                    },
                    OperationPhase::Stream => {
                        let reader = operation.reader.as_mut().ok_or(AbiErrorCode::UnknownHandle)?;
                        match reader.read_step(budget)? {
                            crate::semio_framework::abi::AbiCursorStep::PageComplete(_) => {
                                page = Some(reader.page().cloned().ok_or(AbiErrorCode::UnknownHandle)?);
                            }
                            crate::semio_framework::abi::AbiCursorStep::Complete => {
                                reply = Some(success_reply(operation.request_id, operation.generation, Vec::new()));
                                operation.phase = OperationPhase::Terminal;
                                retain = false;
                            }
                            _ => {}
                        }
                    }
                    OperationPhase::Terminal => retain = false,
                }
            }
        }
        if let Some((request_id, generation, code, status, bytes)) = event {
            self.push_event(request_id, generation, code, status, bytes)?;
        }
        if let Some(page) = page {
            self.push_outbound(AbiMessage::Page(page))?;
        }
        if let Some(reply) = reply {
            self.push_reply(reply)?;
        }
        if retain {
            self.work.push_back(handle);
        } else {
            self.retire_operation(handle)?;
        }
        Ok(())
    }

    fn retire_operation(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        let SequenceResource::Operation(operation) = self.resources.close(handle)? else {
            return Err(AbiErrorCode::UnknownHandle);
        };
        let index = request_slot(operation.request_id);
        if self.requests[index].is_some_and(|entry| entry.operation == handle) {
            self.requests[index] = None;
        }
        self.request_ledger.accept(&success_reply(operation.request_id, operation.generation, Vec::new()))?;
        self.active_resources -= 1;
        Ok(())
    }

    fn push_reply(&mut self, reply: AbiReply) -> Result<(), AbiErrorCode> {
        self.push_outbound(AbiMessage::Reply(reply))
    }

    fn push_event(&mut self, request_id: AbiRequestId, generation: u32, code: u16, status: AbiStatus, bytes: Vec<u8>) -> Result<(), AbiErrorCode> {
        if self.event_count == SEQUENCE_MAX_EVENTS_IN_FLIGHT {
            return Err(AbiErrorCode::LimitExceeded);
        }
        let event_request = AbiRequestId(request_id.0 ^ ((self.next_event_sequence as u64) << 32));
        let index = request_slot(event_request);
        if self.events[index].is_some() {
            return Err(AbiErrorCode::Busy);
        }
        let event = AbiEvent { request_id: event_request, generation, sequence: self.next_event_sequence, event: AbiEventCode::try_new(code)?, status, bytes: AbiBytes::try_new(bytes).map_err(|rejected| rejected.code)? };
        self.next_event_sequence = self.next_event_sequence.checked_add(1).ok_or(AbiErrorCode::GenerationExhausted)?;
        self.events[index] = Some(EventEntry { request_id: event_request, generation });
        self.event_count += 1;
        self.push_outbound(AbiMessage::Event(event))
    }

    fn push_outbound(&mut self, message: AbiMessage) -> Result<(), AbiErrorCode> {
        if self.outbound.len() == SEQUENCE_MAX_OUTBOUND {
            return Err(AbiErrorCode::LimitExceeded);
        }
        self.outbound.push_back(message);
        Ok(())
    }

    pub fn close_session(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
        let SequenceResource::Session(session) = self.resources.close(handle)? else {
            return Err(AbiErrorCode::UnknownHandle);
        };
        session.borrow_mut().closed = true;
        self.sessions.retain(|candidate| *candidate != handle);
        self.active_resources -= 1;
        Ok(())
    }
}

impl<D: SequenceDomain> AbiPort for SequenceBridge<D> {
    fn try_send(&mut self, message: AbiMessage, budget: AbiWorkBudget) -> Result<(), AbiPortRejection> {
        if let Err(code) = validate_budget(budget) {
            return Err(AbiPortRejection { code, message });
        }
        match message {
            AbiMessage::Request(request) => self.accept_request(request),
            AbiMessage::Control(control) => self.accept_control(control).map_err(|code| AbiPortRejection { code, message: AbiMessage::Control(control) }),
            AbiMessage::Reply(reply) => {
                let copy = reply.clone();
                self.accept_event_ack(reply).map_err(|code| AbiPortRejection { code, message: AbiMessage::Reply(copy) })
            }
            other => Err(AbiPortRejection { code: AbiErrorCode::MalformedTag, message: other }),
        }
    }

    fn poll(&mut self, budget: AbiWorkBudget) -> Result<AbiPortPoll, AbiErrorCode> {
        validate_budget(budget)?;
        if let Some(message) = self.outbound.pop_front() {
            return Ok(AbiPortPoll::Message(message));
        }
        self.advance(budget)?;
        if let Some(message) = self.outbound.pop_front() {
            Ok(AbiPortPoll::Message(message))
        } else if self.terminal_is_empty() {
            Ok(AbiPortPoll::Closed)
        } else {
            Ok(AbiPortPoll::Pending)
        }
    }
}

fn execute_operation<D: SequenceDomain>(operation: &mut SequenceOperation<D>) -> Result<Vec<u8>, AbiErrorCode> {
    let mut session = operation.session.borrow_mut();
    if session.closed {
        return Err(AbiErrorCode::Closed);
    }
    match operation.operation {
        SEQUENCE_OPERATION_ATTACH_SURFACE => {
            let mut reader = SequencePayloadReader::new(&operation.payload);
            let surface = SurfaceId::try_new(reader.u32()?)?;
            let canvas = CanvasId::try_new(reader.u32()?)?;
            reader.finish()?;
            session.surface = Some((surface, canvas));
            Ok(Vec::new())
        }
        SEQUENCE_OPERATION_GPU_READY => Ok(vec![u8::from(session.surface.is_some())]),
        SEQUENCE_OPERATION_RENDER_FRAME if session.surface.is_none() => Err(AbiErrorCode::UnknownHandle),
        SEQUENCE_OPERATION_PLAY => {
            session.playback = PlaybackState::Playing;
            Ok(vec![1])
        }
        SEQUENCE_OPERATION_PAUSE => {
            session.playback = PlaybackState::Paused;
            Ok(vec![2])
        }
        SEQUENCE_OPERATION_STOP => {
            session.playback = PlaybackState::Stopped;
            Ok(vec![0])
        }
        operation_code => session.domain.execute(operation_code, &operation.payload).map_err(|failure| failure.code),
    }
}

fn success_reply(request_id: AbiRequestId, generation: u32, bytes: Vec<u8>) -> AbiReply {
    AbiReply { request_id, generation, status: AbiStatus::OK, bytes: AbiBytes::try_new(bytes).expect("bounded Sequence reply") }
}

fn failure_reply(request_id: AbiRequestId, generation: u32, status: AbiStatusCode, code: AbiErrorCode, message: &str) -> AbiReply {
    let message = AbiMessageBytes::from_text(message).unwrap_or_default();
    AbiReply { request_id, generation, status: AbiStatus { code: status, error: Some(AbiError { code, message }) }, bytes: AbiBytes::default() }
}

fn request_slot(request_id: AbiRequestId) -> usize {
    (request_id.0 % SEQUENCE_MAX_REQUESTS as u64) as usize
}

fn validate_budget(budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
    if budget.cancelled {
        Err(AbiErrorCode::Cancelled)
    } else if budget.interrupted {
        Err(AbiErrorCode::Interrupted)
    } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
        Err(AbiErrorCode::DeadlineExceeded)
    } else if budget.byte_credit == 0 {
        Err(AbiErrorCode::NoCredit)
    } else {
        Ok(())
    }
}

fn admitted_bytes(budget: AbiWorkBudget, remaining: usize) -> Result<usize, AbiErrorCode> {
    validate_budget(budget)?;
    Ok(budget.byte_credit.min(remaining))
}

//#endregion 🔖️Runtime

//#region 🧪️Laws

#[cfg(test)]
mod tests {
    use super::*;

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
        AbiMessage::Request(AbiRequest { operation: AbiOperation::try_new(operation).unwrap(), request_id: AbiRequestId(id), generation, bytes: AbiBytes::try_new(bytes).unwrap() })
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
}

//#endregion 🧪️Laws
