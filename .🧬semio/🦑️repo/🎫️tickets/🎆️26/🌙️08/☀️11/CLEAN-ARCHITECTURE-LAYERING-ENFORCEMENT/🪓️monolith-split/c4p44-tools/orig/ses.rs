//! 🪢️ Session of the bridge crate: Shell bridge connection handle, outbox and agent conversation.

use semio_framework_async::Job;
use semio_framework_async::Lane;
use semio_framework_async::ProcessKind;
use semio_framework_async::WorkerPool;
use semio_framework_async::WorkerPoolConfig;
use semio_framework_async::WorkerSubmitErrorKind;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use crate::shell_command_dispatch::BridgeSlot;
use semio_framework_os_mcp_bridge_codec::bridge::{ApprovalDecision, BridgeFlags, BridgeInstanceRef, GatewayToShell, ShellToGateway, truncate_conversation_text};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_wire::job_registry::job_registry;
use semio_framework_os_mcp_wire::notify::agent_messages_changed;

//#region 🔖️BridgeHandle

/// 🆔️ One live `/bridge` connection's id — `Copy`/`Eq`/`Hash` so it keys a map and passes around
/// freely; `Display` is the same string the connection's own `Welcome.connection` field carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShellConnectionId(pub u64);

impl std::fmt::Display for ShellConnectionId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "conn_{}", self.0)
    }
}

struct ConnectionEntry {
    pub generation: u64,
    pub outbox: Arc<BridgeOutbox>,
    /// 🚩️ The flags this shell declared in its own `Hello`. `relay_app_commands` is what
    /// `🐚️channel::SessionChannelBinding` reads to decide whether the artifact verbs of a session
    /// may execute IN this shell: a shell that does not claim the relay is never handed one.
    pub flags: BridgeFlags,
    pub last_shell_state: Option<ShellToGateway>,
    pub last_instances: Option<Vec<BridgeInstanceRef>>,
    pub last_command_result: Option<(u64, bool, Option<String>)>,
    /// 🗿️ The shell's most recent `AppFrames` reply, by correlation id — `(in_reply_to,
    /// instance_id, frames)`. Read by `ShellArtifactChannel::exchange`, which only ever accepts the
    /// entry whose `in_reply_to` equals the `seq` it minted, exactly as `ui_focus`/`ui_reveal`
    /// already do with `last_command_result`.
    pub last_app_frames: Option<(u64, String, Vec<Vec<u8>>)>,
    pub last_approval: Option<(String, ApprovalDecision, Option<String>)>,
    /// 💬️ Human turns this shell typed at the agent, oldest first, capped at
    /// [`BRIDGE_AGENT_INBOX_MAX_ITEMS`]. Read-once: `take_agent_messages` drains it, so the
    /// `semio://ui/agent-messages` resource the agent polls never replays a turn it already read
    /// and never grows without bound if no agent is polling.
    agent_inbox: std::collections::VecDeque<AgentInboxMessage>,
}

/// 💬️ One human turn waiting for the connected agent to read it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgentInboxMessage {
    pub message_id: String,
    pub text: String,
    pub received_at_ms: u64,
}

/// 📥️ How many unread human turns one connection retains. A shell whose agent never polls drops
/// its OLDEST turn rather than growing without bound — the same fixed-credit rule the outbox uses.
pub const BRIDGE_AGENT_INBOX_MAX_ITEMS: usize = 64;

pub(crate) const BRIDGE_OUTBOX_MAX_ITEMS: usize = 64;

pub(crate) const BRIDGE_OUTBOX_MAX_BYTES: usize = 1_048_576;

pub(crate) const BRIDGE_BROADCAST_MAX_RECIPIENTS: usize = 64;

const BRIDGE_BROADCAST_MAX_BYTES: usize = BRIDGE_OUTBOX_MAX_BYTES * BRIDGE_BROADCAST_MAX_RECIPIENTS;

pub(crate) const BRIDGE_OUTBOX_PAGE_BYTES: usize = 16_384;

pub(crate) const BRIDGE_OUTBOX_MAX_PAGES: usize = BRIDGE_OUTBOX_MAX_BYTES.div_ceil(BRIDGE_OUTBOX_PAGE_BYTES);

const BRIDGE_BROADCAST_MAX_PENDING: usize = 64;

const BRIDGE_RETIREMENT_MAX_PENDING: usize = 256;

const BRIDGE_ASYNC_RETRY_MS: u64 = 1;

#[derive(Clone, Copy)]
struct BridgeRetirementGrant {
    pub generation: u64,
}

struct BridgeRetirementCursor {
    pub(crate) pages: [Option<Box<[u8; BRIDGE_OUTBOX_PAGE_BYTES]>>; BRIDGE_OUTBOX_MAX_PAGES],
    pub page: usize,
}

pub(crate) enum BridgeRecipientState {
    Claimed { id: ShellConnectionId, outbox: Arc<BridgeOutbox>, grant: BridgeOutboxGrant },
    Published { id: ShellConnectionId },
    RecipientClosed { id: ShellConnectionId, grant: BridgeOutboxGrant },
}

pub(crate) struct BridgeBroadcastCursor {
    pub frame: Option<GatewayToShell>,
    pub expected: usize,
    pub offset: usize,
    pub(crate) encoded: BridgeEncodedFrame,
    pub(crate) shared: Option<Arc<BridgeEncodedFrame>>,
    pub(crate) recipients_state: [Option<BridgeRecipientState>; BRIDGE_BROADCAST_MAX_RECIPIENTS],
    pub(crate) recipients: usize,
    pub(crate) recipient_cursor: usize,
    pub(crate) delivered: usize,
    pub(crate) recipient_closed: usize,
}

pub(crate) enum BridgeBroadcastStep {
    Pending(BridgeBroadcastCursor),
    Complete(BridgeBroadcastCompletion),
}

#[derive(Debug, PartialEq)]
pub enum BridgeBroadcastCompletion {
    Delivered { delivered: usize, recipient_closed: usize },
    Undelivered { frame: GatewayToShell, recipient_closed: usize },
}

impl BridgeBroadcastCursor {
    pub fn step(mut self) -> BridgeBroadcastStep {
        if self.offset != self.expected {
            let bytes = BRIDGE_OUTBOX_PAGE_BYTES.min(self.expected - self.offset);
            let mut page = Box::new([0; BRIDGE_OUTBOX_PAGE_BYTES]);
            let written = self.frame.as_ref().expect("bridge broadcast frame disappeared").copy_encoded_page(self.offset, &mut page[..bytes]);
            assert_eq!(written, bytes, "bridge broadcast page preflight changed");
            self.encoded.pages[self.offset / BRIDGE_OUTBOX_PAGE_BYTES] = Some(page);
            self.offset += bytes;
            return BridgeBroadcastStep::Pending(self);
        }
        if self.shared.is_none() {
            self.shared = Some(Arc::new(std::mem::replace(&mut self.encoded, BridgeEncodedFrame::empty(0, None))));
            return BridgeBroadcastStep::Pending(self);
        }
        if self.recipient_cursor < self.recipients {
            let index = self.recipient_cursor;
            let state = self.recipients_state[index].take().expect("bridge recipient state disappeared");
            match state {
                BridgeRecipientState::Claimed { id, outbox, grant } => match outbox.publish(grant, Arc::clone(self.shared.as_ref().expect("shared bridge frame disappeared"))) {
                    Ok(()) => {
                        self.recipients_state[index] = Some(BridgeRecipientState::Published { id });
                        self.delivered += 1;
                    }
                    Err(rejected) => {
                        self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant: rejected.grant });
                        self.recipient_closed += 1;
                        drop(rejected.encoded);
                    }
                },
                BridgeRecipientState::Published { id } => self.recipients_state[index] = Some(BridgeRecipientState::Published { id }),
                BridgeRecipientState::RecipientClosed { id, grant } => self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant }),
            }
            self.recipient_cursor += 1;
            return BridgeBroadcastStep::Pending(self);
        }
        let completion = if self.delivered == 0 {
            BridgeBroadcastCompletion::Undelivered { frame: self.frame.take().expect("undelivered bridge frame disappeared"), recipient_closed: self.recipient_closed }
        } else {
            self.frame.take();
            BridgeBroadcastCompletion::Delivered { delivered: self.delivered, recipient_closed: self.recipient_closed }
        };
        BridgeBroadcastStep::Complete(completion)
    }

    fn close_one_claim(mut self) -> BridgeBroadcastStep {
        while self.recipient_cursor < self.recipients {
            let index = self.recipient_cursor;
            self.recipient_cursor += 1;
            let state = self.recipients_state[index].take().expect("bridge recipient state disappeared");
            match state {
                BridgeRecipientState::Claimed { id, outbox, grant } => {
                    outbox.cancel(grant);
                    self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant });
                    self.recipient_closed += 1;
                    return BridgeBroadcastStep::Pending(self);
                }
                BridgeRecipientState::Published { id } => self.recipients_state[index] = Some(BridgeRecipientState::Published { id }),
                BridgeRecipientState::RecipientClosed { id, grant } => self.recipients_state[index] = Some(BridgeRecipientState::RecipientClosed { id, grant }),
            }
        }
        let completion = if self.delivered == 0 {
            BridgeBroadcastCompletion::Undelivered { frame: self.frame.take().expect("terminal bridge frame disappeared"), recipient_closed: self.recipient_closed }
        } else {
            self.frame.take();
            BridgeBroadcastCompletion::Delivered { delivered: self.delivered, recipient_closed: self.recipient_closed }
        };
        BridgeBroadcastStep::Complete(completion)
    }
}

struct BridgeAsyncState {
    pub(crate) broadcasts: Box<[Option<BridgeBroadcastCursor>]>,
    broadcast_head: usize,
    pub(crate) broadcast_len: usize,
    pub(crate) broadcast_reserved: usize,
    broadcast_driving: bool,
    completions: Box<[Option<BridgeBroadcastCompletion>]>,
    completion_head: usize,
    completion_len: usize,
    completion_reserved: usize,
    retirements: Box<[Option<BridgeRetirementCursor>]>,
    retirement_head: usize,
    pub(crate) retirement_len: usize,
    retirement_reserved: usize,
    retirement_generation: u64,
}

impl BridgeAsyncState {
    fn new() -> Self {
        Self {
            broadcasts: semio_framework_async::boxed_slots(BRIDGE_BROADCAST_MAX_PENDING, || None),
            broadcast_head: 0,
            broadcast_len: 0,
            broadcast_reserved: 0,
            broadcast_driving: false,
            completions: semio_framework_async::boxed_slots(BRIDGE_BROADCAST_MAX_PENDING, || None),
            completion_head: 0,
            completion_len: 0,
            completion_reserved: 0,
            retirements: semio_framework_async::boxed_slots(BRIDGE_RETIREMENT_MAX_PENDING, || None),
            retirement_head: 0,
            retirement_len: 0,
            retirement_reserved: 0,
            retirement_generation: 1,
        }
    }

    fn has_work(&self) -> bool {
        self.broadcast_len != 0 || self.retirement_len != 0
    }
}

struct BridgeAsyncAuthority {
    pub pool: WorkerPool,
    pub state: Mutex<BridgeAsyncState>,
    pub scheduled: AtomicBool,
    pub retry_armed: AtomicBool,
    pub retry_generation: AtomicU64,
    pub retry_job: Mutex<Option<Job>>,
    pub terminal_job: Mutex<Option<(WorkerSubmitErrorKind, Job)>>,
    pub terminal: AtomicBool,
}

impl BridgeAsyncAuthority {
    fn new(pool: WorkerPool) -> Arc<Self> {
        Arc::new(Self {
            pool,
            state: Mutex::new(BridgeAsyncState::new()),
            scheduled: AtomicBool::new(false),
            retry_armed: AtomicBool::new(false),
            retry_generation: AtomicU64::new(0),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            terminal: AtomicBool::new(false),
        })
    }

    pub(crate) fn reserve_broadcast(&self) -> Result<(), ()> {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if self.terminal.load(Ordering::Acquire) || state.broadcast_len.saturating_add(state.broadcast_reserved) == BRIDGE_BROADCAST_MAX_PENDING || state.completion_len.saturating_add(state.completion_reserved) == BRIDGE_BROADCAST_MAX_PENDING {
            return Err(());
        }
        state.broadcast_reserved += 1;
        state.completion_reserved += 1;
        Ok(())
    }

    fn cancel_broadcast_reservation(&self) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        state.broadcast_reserved = state.broadcast_reserved.saturating_sub(1);
        state.completion_reserved = state.completion_reserved.saturating_sub(1);
    }

    pub(crate) fn reserve_retirement(&self) -> Result<BridgeRetirementGrant, ()> {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if self.terminal.load(Ordering::Acquire) || state.retirement_len.saturating_add(state.retirement_reserved) == BRIDGE_RETIREMENT_MAX_PENDING {
            return Err(());
        }
        state.retirement_reserved += 1;
        Ok(BridgeRetirementGrant { generation: state.retirement_generation })
    }

    fn cancel_retirement(&self, grant: BridgeRetirementGrant) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if grant.generation == state.retirement_generation {
            state.retirement_reserved = state.retirement_reserved.saturating_sub(1);
        }
    }

    fn enqueue_broadcast(self: &Arc<Self>, cursor: BridgeBroadcastCursor) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        state.broadcast_reserved = state.broadcast_reserved.saturating_sub(1);
        let index = (state.broadcast_head + state.broadcast_len) % BRIDGE_BROADCAST_MAX_PENDING;
        state.broadcasts[index] = Some(cursor);
        state.broadcast_len += 1;
        drop(state);
        self.request_schedule();
    }

    fn retain_completion(state: &mut BridgeAsyncState, completion: BridgeBroadcastCompletion) {
        state.completion_reserved = state.completion_reserved.saturating_sub(1);
        let index = (state.completion_head + state.completion_len) % BRIDGE_BROADCAST_MAX_PENDING;
        state.completions[index] = Some(completion);
        state.completion_len += 1;
    }

    fn publish_retirement(self: &Arc<Self>, grant: BridgeRetirementGrant, pages: [Option<Box<[u8; BRIDGE_OUTBOX_PAGE_BYTES]>>; BRIDGE_OUTBOX_MAX_PAGES]) {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if grant.generation != state.retirement_generation || state.retirement_reserved == 0 {
            drop(state);
            drop(pages);
            return;
        }
        state.retirement_reserved -= 1;
        let index = (state.retirement_head + state.retirement_len) % BRIDGE_RETIREMENT_MAX_PENDING;
        state.retirements[index] = Some(BridgeRetirementCursor { pages, page: 0 });
        state.retirement_len += 1;
        drop(state);
        self.request_schedule();
    }

    pub fn request_schedule(self: &Arc<Self>) {
        if self.terminal.load(Ordering::Acquire) || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let authority = Arc::clone(self);
        self.submit_exact(Box::new(move || authority.drive_one()));
    }

    pub fn submit_exact(self: &Arc<Self>, job: Job) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => match error.kind() {
                WorkerSubmitErrorKind::Contended | WorkerSubmitErrorKind::Saturated => {
                    *self.retry_job.lock().expect("bridge retry lock poisoned") = Some(error.into_job());
                    self.arm_retry();
                }
                kind @ (WorkerSubmitErrorKind::Shutdown | WorkerSubmitErrorKind::Poisoned) => {
                    self.terminal.store(true, Ordering::Release);
                    self.retry_generation.fetch_add(1, Ordering::AcqRel);
                    *self.terminal_job.lock().expect("bridge terminal lock poisoned") = Some((kind, error.into_job()));
                }
            },
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = self.retry_generation.fetch_add(1, Ordering::AcqRel).wrapping_add(1);
        let authority = Arc::clone(self);
        self.pool.callback_at(self.pool.now_ms().saturating_add(BRIDGE_ASYNC_RETRY_MS), move || {
            if generation != authority.retry_generation.load(Ordering::Acquire) {
                return;
            }
            authority.retry_armed.store(false, Ordering::Release);
            let job = authority.retry_job.lock().expect("bridge retry lock poisoned").take();
            if let Some(job) = job {
                authority.submit_exact(job);
            }
        });
    }

    pub fn drive_one(self: Arc<Self>) {
        if self.terminal.load(Ordering::Acquire) {
            self.scheduled.store(false, Ordering::Release);
            return;
        }
        let broadcast = {
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            if state.broadcast_len == 0 || state.broadcast_driving {
                None
            } else {
                let head = state.broadcast_head;
                state.broadcast_driving = true;
                state.broadcasts[head].take()
            }
        };
        if let Some(cursor) = broadcast {
            let step = cursor.step();
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            state.broadcast_driving = false;
            let head = state.broadcast_head;
            match step {
                BridgeBroadcastStep::Pending(cursor) => state.broadcasts[head] = Some(cursor),
                BridgeBroadcastStep::Complete(completion) => {
                    state.broadcast_head = (head + 1) % BRIDGE_BROADCAST_MAX_PENDING;
                    state.broadcast_len -= 1;
                    Self::retain_completion(&mut state, completion);
                }
            }
        } else {
            let page = {
                let mut state = self.state.lock().expect("bridge async lock poisoned");
                if state.retirement_len == 0 {
                    None
                } else {
                    let head = state.retirement_head;
                    let cursor = state.retirements[head].as_mut().expect("bridge retirement cursor disappeared");
                    while cursor.page < BRIDGE_OUTBOX_MAX_PAGES && cursor.pages[cursor.page].is_none() {
                        cursor.page += 1;
                    }
                    if cursor.page == BRIDGE_OUTBOX_MAX_PAGES {
                        state.retirements[head] = None;
                        state.retirement_head = (head + 1) % BRIDGE_RETIREMENT_MAX_PENDING;
                        state.retirement_len -= 1;
                        None
                    } else {
                        let page = cursor.pages[cursor.page].take();
                        cursor.page += 1;
                        page
                    }
                }
            };
            drop(page);
        }
        self.scheduled.store(false, Ordering::Release);
        if self.state.lock().expect("bridge async lock poisoned").has_work() {
            self.request_schedule();
        }
    }

    fn take_terminal_job(&self) -> Option<(WorkerSubmitErrorKind, Job)> {
        self.terminal_job.lock().expect("bridge terminal lock poisoned").take()
    }

    pub fn cancel(&self) {
        self.terminal.store(true, Ordering::Release);
        self.retry_generation.fetch_add(1, Ordering::AcqRel);
    }

    pub(crate) fn take_broadcast_completion(&self) -> Option<BridgeBroadcastCompletion> {
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        if state.completion_len == 0 {
            return None;
        }
        let head = state.completion_head;
        let completion = state.completions[head].take();
        state.completion_head = (head + 1) % BRIDGE_BROADCAST_MAX_PENDING;
        state.completion_len -= 1;
        completion
    }

    pub(crate) fn close_one_terminal_broadcast_claim(&self) -> bool {
        if !self.terminal.load(Ordering::Acquire) {
            return false;
        }
        let cursor = {
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            if state.broadcast_len == 0 || state.broadcast_driving {
                return false;
            }
            let head = state.broadcast_head;
            state.broadcast_driving = true;
            state.broadcasts[head].take().expect("terminal bridge broadcast cursor disappeared")
        };
        let step = cursor.close_one_claim();
        let mut state = self.state.lock().expect("bridge async lock poisoned");
        state.broadcast_driving = false;
        let head = state.broadcast_head;
        match step {
            BridgeBroadcastStep::Pending(cursor) => state.broadcasts[head] = Some(cursor),
            BridgeBroadcastStep::Complete(completion) => {
                state.broadcast_head = (head + 1) % BRIDGE_BROADCAST_MAX_PENDING;
                state.broadcast_len -= 1;
                Self::retain_completion(&mut state, completion);
            }
        }
        true
    }

    pub(crate) fn close_one_terminal_retired_page(&self) -> bool {
        if !self.terminal.load(Ordering::Acquire) {
            return false;
        }
        let page = {
            let mut state = self.state.lock().expect("bridge async lock poisoned");
            if state.retirement_len == 0 {
                return false;
            }
            let head = state.retirement_head;
            let cursor = state.retirements[head].as_mut().expect("bridge retirement cursor disappeared");
            while cursor.page < BRIDGE_OUTBOX_MAX_PAGES && cursor.pages[cursor.page].is_none() {
                cursor.page += 1;
            }
            let page = if cursor.page < BRIDGE_OUTBOX_MAX_PAGES {
                let page = cursor.pages[cursor.page].take();
                cursor.page += 1;
                page
            } else {
                None
            };
            if cursor.page == BRIDGE_OUTBOX_MAX_PAGES || cursor.pages[cursor.page..].iter().all(Option::is_none) {
                state.retirements[head] = None;
                state.retirement_head = (head + 1) % BRIDGE_RETIREMENT_MAX_PENDING;
                state.retirement_len -= 1;
            }
            page
        };
        drop(page);
        true
    }
}

pub(crate) struct BridgeEncodedFrame {
    pub(crate) pages: [Option<Box<[u8; BRIDGE_OUTBOX_PAGE_BYTES]>>; BRIDGE_OUTBOX_MAX_PAGES],
    pub len: usize,
    pub(crate) retirement: Option<(Arc<BridgeAsyncAuthority>, BridgeRetirementGrant)>,
}

impl BridgeEncodedFrame {
    pub(crate) fn empty(len: usize, retirement: Option<(Arc<BridgeAsyncAuthority>, BridgeRetirementGrant)>) -> Self {
        Self { pages: std::array::from_fn(|_| None), len, retirement }
    }

    pub fn encode(frame: &GatewayToShell, expected: usize) -> Self {
        let mut encoded = Self::empty(0, None);
        match frame {
            GatewayToShell::Welcome { bridge_version, connection, principal } => {
                encoded.write_u8(0);
                encoded.write_bytes(&bridge_version.to_le_bytes());
                encoded.write_field(connection.as_bytes());
                encoded.write_field(principal.as_bytes());
            }
            GatewayToShell::ShellCommand { seq, command } => {
                encoded.write_u8(1);
                encoded.write_bytes(&seq.to_le_bytes());
                encoded.write_field(command);
            }
            GatewayToShell::AppCommand { seq, instance_id, command } => {
                encoded.write_u8(2);
                encoded.write_bytes(&seq.to_le_bytes());
                encoded.write_field(instance_id.as_bytes());
                encoded.write_field(command);
            }
            GatewayToShell::ApprovalRequested { approval_id, summary } => {
                encoded.write_u8(3);
                encoded.write_field(approval_id.as_bytes());
                encoded.write_field(summary.as_bytes());
            }
            GatewayToShell::ApprovalResolved { approval_id, decision } => {
                encoded.write_u8(4);
                encoded.write_field(approval_id.as_bytes());
                encoded.write_u8(decision.to_tag());
            }
            GatewayToShell::AgentPresence { active, label, invocation_id } => {
                encoded.write_u8(5);
                encoded.write_u8(*active as u8);
                encoded.write_field(label.as_bytes());
                encoded.write_u8(invocation_id.is_some() as u8);
                if let Some(invocation_id) = invocation_id {
                    encoded.write_field(invocation_id.as_bytes());
                }
            }
            GatewayToShell::Pong => encoded.write_u8(6),
            GatewayToShell::Bye { reason } => {
                encoded.write_u8(7);
                encoded.write_field(reason.as_bytes());
            }
            GatewayToShell::AgentToolCall { invocation_id, tool_name, arguments } => {
                encoded.write_u8(8);
                encoded.write_field(invocation_id.as_bytes());
                encoded.write_field(tool_name.as_bytes());
                encoded.write_field(arguments.as_bytes());
            }
            GatewayToShell::AgentToolResult { invocation_id, tool_name, ok, summary } => {
                encoded.write_u8(9);
                encoded.write_field(invocation_id.as_bytes());
                encoded.write_field(tool_name.as_bytes());
                encoded.write_u8(*ok as u8);
                encoded.write_field(summary.as_bytes());
            }
            GatewayToShell::AgentReply { reply_id, in_reply_to, text, complete } => {
                encoded.write_u8(10);
                encoded.write_field(reply_id.as_bytes());
                encoded.write_u8(in_reply_to.is_some() as u8);
                if let Some(in_reply_to) = in_reply_to {
                    encoded.write_field(in_reply_to.as_bytes());
                }
                encoded.write_field(text.as_bytes());
                encoded.write_u8(*complete as u8);
            }
            GatewayToShell::ApprovalWithdrawn { approval_id, reason } => {
                encoded.write_u8(11);
                encoded.write_field(approval_id.as_bytes());
                encoded.write_u8(reason.to_tag());
            }
            GatewayToShell::Refused { reason, gateway_version } => {
                encoded.write_u8(12);
                encoded.write_u8(reason.to_tag());
                encoded.write_bytes(&gateway_version.to_le_bytes());
            }
        }
        assert_eq!(encoded.len, expected, "preflighted bridge frame length changed during encode");
        encoded
    }

    fn write_u8(&mut self, value: u8) {
        self.write_bytes(&[value]);
    }

    fn write_field(&mut self, value: &[u8]) {
        self.write_bytes(&(value.len() as u32).to_le_bytes());
        self.write_bytes(value);
    }

    fn write_bytes(&mut self, mut value: &[u8]) {
        while !value.is_empty() {
            let page_index = self.len / BRIDGE_OUTBOX_PAGE_BYTES;
            let page_offset = self.len % BRIDGE_OUTBOX_PAGE_BYTES;
            if self.pages[page_index].is_none() {
                self.pages[page_index] = Some(Box::new([0; BRIDGE_OUTBOX_PAGE_BYTES]));
            }
            let bytes = value.len().min(BRIDGE_OUTBOX_PAGE_BYTES - page_offset);
            self.pages[page_index].as_mut().expect("bridge encode page disappeared")[page_offset..page_offset + bytes].copy_from_slice(&value[..bytes]);
            self.len += bytes;
            value = &value[bytes..];
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn copy_into(&self, offset: usize, output: &mut [u8]) -> usize {
        if offset >= self.len {
            return 0;
        }
        let mut source = offset;
        let mut written = 0;
        while written < output.len() && source < self.len {
            let page_index = source / BRIDGE_OUTBOX_PAGE_BYTES;
            let page_offset = source % BRIDGE_OUTBOX_PAGE_BYTES;
            let bytes = (output.len() - written).min(BRIDGE_OUTBOX_PAGE_BYTES - page_offset).min(self.len - source);
            output[written..written + bytes].copy_from_slice(&self.pages[page_index].as_ref().expect("bridge encoded page disappeared")[page_offset..page_offset + bytes]);
            written += bytes;
            source += bytes;
        }
        written
    }

    #[cfg(test)]
    pub(crate) fn page_count(&self) -> usize {
        self.pages.iter().filter(|page| page.is_some()).count()
    }

    #[cfg(test)]
    fn decode(&self) -> GatewayToShell {
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(self.len).expect("bridge test decode reservation failed");
        for offset in (0..self.len).step_by(BRIDGE_OUTBOX_PAGE_BYTES) {
            let bytes_in_page = BRIDGE_OUTBOX_PAGE_BYTES.min(self.len - offset);
            let start = bytes.len();
            bytes.resize(start + bytes_in_page, 0);
            assert_eq!(self.copy_into(offset, &mut bytes[start..]), bytes_in_page);
        }
        GatewayToShell::decode(&bytes).expect("bridge fixed-page encode did not decode")
    }
}

impl Drop for BridgeEncodedFrame {
    fn drop(&mut self) {
        let Some((authority, grant)) = self.retirement.take() else { return };
        let mut pages = std::array::from_fn(|_| None);
        for (target, source) in pages.iter_mut().zip(self.pages.iter_mut()) {
            *target = source.take();
        }
        authority.publish_retirement(grant, pages);
    }
}

#[derive(Clone)]
pub struct BridgeEncodedLease {
    pub generation: u64,
    pub(crate) encoded: Arc<BridgeEncodedFrame>,
}

impl BridgeEncodedLease {
    pub fn len(&self) -> usize {
        self.encoded.len()
    }

    pub fn copy_into(&self, offset: usize, output: &mut [u8]) -> usize {
        self.encoded.copy_into(offset, output)
    }
}

struct BridgeOutboxItem {
    pub lease: BridgeEncodedLease,
    pub bytes: usize,
}

#[derive(Clone, Copy)]
struct BridgeOutboxGrant {
    pub generation: u64,
    pub bytes: usize,
}

struct BridgeRejectedPublish {
    pub grant: BridgeOutboxGrant,
    pub(crate) encoded: Arc<BridgeEncodedFrame>,
}

struct BridgeOutboxState {
    pub slots: [Option<BridgeOutboxItem>; BRIDGE_OUTBOX_MAX_ITEMS],
    pub head: usize,
    pub len: usize,
    pub bytes: usize,
    pub generation: u64,
    pub(crate) reserved_items: usize,
    pub(crate) reserved_bytes: usize,
    pub closed: bool,
    #[cfg(test)]
    pub(crate) encode_count: usize,
    #[cfg(test)]
    waker: Option<std::task::Waker>,
}

impl BridgeOutboxState {
    fn new(generation: u64) -> Self {
        Self {
            slots: std::array::from_fn(|_| None),
            head: 0,
            len: 0,
            bytes: 0,
            generation,
            reserved_items: 0,
            reserved_bytes: 0,
            closed: false,
            #[cfg(test)]
            encode_count: 0,
            #[cfg(test)]
            waker: None,
        }
    }

    pub(crate) fn claim(&mut self, generation: u64, bytes: usize) -> Result<BridgeOutboxGrant, ()> {
        if self.closed || generation != self.generation || self.len.saturating_add(self.reserved_items) == BRIDGE_OUTBOX_MAX_ITEMS || bytes > BRIDGE_OUTBOX_MAX_BYTES.saturating_sub(self.bytes).saturating_sub(self.reserved_bytes) {
            return Err(());
        }
        self.reserved_items += 1;
        self.reserved_bytes += bytes;
        Ok(BridgeOutboxGrant { generation, bytes })
    }

    pub fn cancel(&mut self, grant: BridgeOutboxGrant) {
        if grant.generation == self.generation {
            self.reserved_items = self.reserved_items.saturating_sub(1);
            self.reserved_bytes = self.reserved_bytes.saturating_sub(grant.bytes);
        }
    }

    pub(crate) fn publish(&mut self, grant: BridgeOutboxGrant, encoded: Arc<BridgeEncodedFrame>) -> Result<(), BridgeRejectedPublish> {
        if self.closed || grant.generation != self.generation || self.reserved_items == 0 || grant.bytes > self.reserved_bytes {
            return Err(BridgeRejectedPublish { grant, encoded });
        }
        self.reserved_items -= 1;
        self.reserved_bytes -= grant.bytes;
        let index = (self.head + self.len) % BRIDGE_OUTBOX_MAX_ITEMS;
        self.len += 1;
        self.bytes += grant.bytes;
        #[cfg(test)]
        {
            self.encode_count += 1;
        }
        self.slots[index] = Some(BridgeOutboxItem { lease: BridgeEncodedLease { generation: grant.generation, encoded }, bytes: grant.bytes });
        #[cfg(test)]
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<BridgeOutboxItem> {
        if self.len == 0 {
            return None;
        }
        let item = self.slots[self.head].take().expect("bridge outbox FIFO slot disappeared");
        self.head = (self.head + 1) % BRIDGE_OUTBOX_MAX_ITEMS;
        self.len -= 1;
        self.bytes = self.bytes.saturating_sub(item.bytes);
        Some(item)
    }
}

pub(crate) struct BridgeOutbox {
    pub state: Mutex<BridgeOutboxState>,
}

impl BridgeOutbox {
    pub(crate) fn new(generation: u64) -> Self {
        Self { state: Mutex::new(BridgeOutboxState::new(generation)) }
    }

    pub(crate) fn try_send(&self, frame: GatewayToShell) -> Result<(), GatewayToShell> {
        let Some(bytes) = frame.encoded_len() else { return Err(frame) };
        let grant = match self.claim(bytes) {
            Ok(grant) => grant,
            Err(()) => return Err(frame),
        };
        let encoded = Arc::new(BridgeEncodedFrame::encode(&frame, bytes));
        if self.publish(grant, encoded).is_err() {
            return Err(frame);
        }
        Ok(())
    }

    pub(crate) fn claim(&self, bytes: usize) -> Result<BridgeOutboxGrant, ()> {
        let mut state = self.state.lock().expect("bridge outbox lock poisoned");
        let generation = state.generation;
        state.claim(generation, bytes)
    }

    pub(crate) fn publish(&self, grant: BridgeOutboxGrant, encoded: Arc<BridgeEncodedFrame>) -> Result<(), BridgeRejectedPublish> {
        self.state.lock().expect("bridge outbox lock poisoned").publish(grant, encoded)
    }

    pub fn cancel(&self, grant: BridgeOutboxGrant) {
        self.state.lock().expect("bridge outbox lock poisoned").cancel(grant);
    }

    pub fn try_recv(&self) -> Option<BridgeOutboxItem> {
        let mut state = self.state.lock().expect("bridge outbox lock poisoned");
        let item = state.pop_front()?;
        if item.lease.generation != state.generation {
            return None;
        }
        Some(item)
    }

    pub fn close(&self) {
        let mut state = self.state.lock().expect("bridge outbox lock poisoned");
        state.closed = true;
        state.generation = state.generation.wrapping_add(1);
        state.reserved_items = 0;
        state.reserved_bytes = 0;
        #[cfg(test)]
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

pub struct BridgeOutboxReceiver(pub Arc<BridgeOutbox>);

impl BridgeOutboxReceiver {
    #[cfg(test)]
    pub async fn recv(&mut self) -> Option<GatewayToShell> {
        std::future::poll_fn(|context| {
            let mut state = self.0.state.lock().expect("bridge outbox lock poisoned");
            if let Some(item) = state.pop_front() {
                if item.lease.generation == state.generation {
                    return std::task::Poll::Ready(Some(item.lease.encoded.decode()));
                }
                return std::task::Poll::Ready(None);
            }
            if state.closed {
                return std::task::Poll::Ready(None);
            }
            state.waker = Some(context.waker().clone());
            std::task::Poll::Pending
        })
        .await
    }

    #[cfg(test)]
    pub fn try_recv(&mut self) -> Option<GatewayToShell> {
        self.0.try_recv().map(|item| item.lease.encoded.decode())
    }

    pub fn try_recv_encoded(&mut self) -> Option<BridgeEncodedLease> {
        self.0.try_recv().map(|item| item.lease)
    }
}

struct BridgeInner {
    pub next_id: AtomicU64,
    pub connections: Mutex<HashMap<ShellConnectionId, ConnectionEntry>>,
    pub(crate) asynchronous: Arc<BridgeAsyncAuthority>,
}

/// 🖇️ The seam a later packet reaches live `/bridge` connections through WITHOUT this facet
/// depending on theirs — P6's policy engine routes a parked approval to a connected shell via
/// [`send_to`](Self::send_to)/[`broadcast`](Self::broadcast); a future `ui.*` tool pushes a
/// `ShellCommand` the same way; `semio://ui/shell`/`context_resolve` read
/// [`last_shell_state`](Self::last_shell_state). None of that wiring happens in THIS file
/// (`📓️sol-P1c-packet.md` §3: "do not wire it into P6's files … just publish the API") — obtain a
/// `BridgeHandle` from the live [`crate::HttpTransportRun::bridge`] owner; the Axum router is a
/// test-only differential oracle.
#[derive(Clone)]
pub struct BridgeHandle {
    pub inner: Arc<BridgeInner>,
}

impl Default for BridgeHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl BridgeHandle {
    pub fn new() -> Self {
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        Self::with_pool(semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores)))
    }

    pub fn with_pool(pool: WorkerPool) -> Self {
        Self { inner: Arc::new(BridgeInner { next_id: AtomicU64::new(0), connections: Mutex::new(HashMap::new()), asynchronous: BridgeAsyncAuthority::new(pool) }) }
    }

    pub fn register(&self) -> (ShellConnectionId, BridgeOutboxReceiver) {
        let id = ShellConnectionId(self.inner.next_id.fetch_add(1, Ordering::Relaxed));
        let outbox = Arc::new(BridgeOutbox::new(id.0));
        self.inner
            .connections
            .lock()
            .expect("bridge connections lock poisoned")
            .insert(id, ConnectionEntry { generation: id.0, outbox: Arc::clone(&outbox), flags: BridgeFlags::NONE, last_shell_state: None, last_instances: None, last_command_result: None, last_app_frames: None, last_approval: None, agent_inbox: std::collections::VecDeque::new() });
        (id, BridgeOutboxReceiver(outbox))
    }

    /// 🚩️ Records the flags one shell declared in its `Hello`. `Hello` is handled inline by the
    /// read loop (it is the frame that mints the connection in the first place, so it can never
    /// reach [`BridgeHandle::record`]) — this is the one seam that carries its payload into the
    /// connection state instead of dropping it.
    pub fn record_hello(&self, id: ShellConnectionId, flags: BridgeFlags) {
        if let Some(entry) = self.inner.connections.lock().expect("bridge connections lock poisoned").get_mut(&id) {
            entry.flags = flags;
        }
    }

    pub fn unregister(&self, id: ShellConnectionId) {
        if let Some(entry) = self.inner.connections.lock().expect("bridge connections lock poisoned").remove(&id) {
            entry.outbox.close();
        }
    }

    /// 📝️ Records the effect of one received [`ShellToGateway`] frame against its connection —
    /// `Hello`/`Ping`/`Bye` never reach here (the read loop handles all three inline).
    ///
    /// 🛑️ A cancel is connection-independent and takes no connection state: it flips the SAME
    /// cooperative flag `job_cancel` flips, on the job `AgentConversation::begin_tool_call` opened
    /// under this invocation id. Handled before the connections lock so the two locks never nest.
    /// A `NotFound`/`PreconditionFailed` means the call already finished between the human's click
    /// and this frame — the honest outcome of a cancel arriving late, not an error to report.
    ///
    /// 💬️ Outside the connections lock: the subscription broker takes its own locks, and a human
    /// turn that only becomes visible on the agent's next poll is a channel with a one-poll
    /// latency floor. Subscribed clients are told now; unsubscribed ones still poll as before.
    pub fn record(&self, id: ShellConnectionId, frame: ShellToGateway) {
        if let ShellToGateway::AgentCancel { invocation_id } = &frame {
            let _ = job_registry().request_cancel(invocation_id);
            return;
        }
        let mut inbox_grew = false;
        let mut connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        let Some(entry) = connections.get_mut(&id) else { return };
        match frame {
            ShellToGateway::ShellState { .. } | ShellToGateway::ShellStatePatch { .. } => entry.last_shell_state = Some(frame),
            ShellToGateway::Instances { entries } => entry.last_instances = Some(entries),
            ShellToGateway::ShellCommandResult { in_reply_to, ok, fault } => entry.last_command_result = Some((in_reply_to, ok, fault)),
            ShellToGateway::AppFrames { in_reply_to, instance_id, frames } => entry.last_app_frames = Some((in_reply_to, instance_id, frames)),
            ShellToGateway::Approval { approval_id, decision, note } => entry.last_approval = Some((approval_id, decision, note)),
            ShellToGateway::AgentMessage { message_id, text } => {
                if entry.agent_inbox.len() == BRIDGE_AGENT_INBOX_MAX_ITEMS {
                    entry.agent_inbox.pop_front();
                }
                entry.agent_inbox.push_back(AgentInboxMessage { message_id, text, received_at_ms: bridge_wall_now_ms() });
                inbox_grew = true;
            }
            ShellToGateway::Hello { .. } | ShellToGateway::Ping | ShellToGateway::Bye | ShellToGateway::AgentCancel { .. } => {}
        }
        drop(connections);
        if inbox_grew {
            agent_messages_changed();
        }
    }

    /// 📤️ Pushes one frame to exactly one live connection — `false` if that connection no longer
    /// exists or its outbox is closed (the connection's own task will unregister it shortly after).
    pub fn send_to(&self, id: ShellConnectionId, frame: GatewayToShell) -> bool {
        self.try_send_to(id, frame).is_ok()
    }

    /// 📦️ Fixed-credit bridge admission with exact rejected-frame handback.
    pub fn try_send_to(&self, id: ShellConnectionId, frame: GatewayToShell) -> Result<(), GatewayToShell> {
        let connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        match connections.get(&id) {
            Some(entry) => try_send_bridge_frame(entry, frame),
            None => Err(frame),
        }
    }

    /// 📢️ Atomically admits one shared encoded frame to every live connection.
    pub fn broadcast(&self, frame: GatewayToShell) -> Result<usize, GatewayToShell> {
        let Some(bytes) = frame.encoded_len() else { return Err(frame) };
        let connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        let recipients = connections.len();
        if recipients > BRIDGE_BROADCAST_MAX_RECIPIENTS || bytes.checked_mul(recipients).map_or(true, |total| total > BRIDGE_BROADCAST_MAX_BYTES) {
            return Err(frame);
        }
        if recipients == 0 {
            return Ok(0);
        }
        if self.inner.asynchronous.reserve_broadcast().is_err() {
            return Err(frame);
        }
        let retirement = match self.inner.asynchronous.reserve_retirement() {
            Ok(grant) => grant,
            Err(()) => {
                self.inner.asynchronous.cancel_broadcast_reservation();
                return Err(frame);
            }
        };
        let mut recipient_ids: [Option<ShellConnectionId>; BRIDGE_BROADCAST_MAX_RECIPIENTS] = [None; BRIDGE_BROADCAST_MAX_RECIPIENTS];
        for (index, id) in connections.keys().copied().enumerate() {
            recipient_ids[index] = Some(id);
        }
        recipient_ids[..recipients].sort_unstable();
        let mut recipients_state: [Option<BridgeRecipientState>; BRIDGE_BROADCAST_MAX_RECIPIENTS] = std::array::from_fn(|_| None);
        let mut admitted = 0;
        for id in recipient_ids[..recipients].iter().flatten().copied() {
            let entry = connections.get(&id).expect("stable bridge recipient disappeared under admission lock");
            let grant = match entry.outbox.claim(bytes) {
                Ok(grant) if grant.generation == entry.generation => grant,
                Ok(grant) => {
                    entry.outbox.cancel(grant);
                    for state in recipients_state[..admitted].iter_mut().filter_map(Option::take) {
                        if let BridgeRecipientState::Claimed { outbox, grant, .. } = state {
                            outbox.cancel(grant);
                        }
                    }
                    self.inner.asynchronous.cancel_retirement(retirement);
                    self.inner.asynchronous.cancel_broadcast_reservation();
                    return Err(frame);
                }
                Err(()) => {
                    for state in recipients_state[..admitted].iter_mut().filter_map(Option::take) {
                        if let BridgeRecipientState::Claimed { outbox, grant, .. } = state {
                            outbox.cancel(grant);
                        }
                    }
                    self.inner.asynchronous.cancel_retirement(retirement);
                    self.inner.asynchronous.cancel_broadcast_reservation();
                    return Err(frame);
                }
            };
            recipients_state[admitted] = Some(BridgeRecipientState::Claimed { id, outbox: Arc::clone(&entry.outbox), grant });
            admitted += 1;
        }
        let cursor = BridgeBroadcastCursor {
            frame: Some(frame),
            expected: bytes,
            offset: 0,
            encoded: BridgeEncodedFrame::empty(bytes, Some((Arc::clone(&self.inner.asynchronous), retirement))),
            shared: None,
            recipients_state,
            recipients,
            recipient_cursor: 0,
            delivered: 0,
            recipient_closed: 0,
        };
        drop(connections);
        self.inner.asynchronous.enqueue_broadcast(cursor);
        Ok(recipients)
    }

    pub fn take_broadcast_completion(&self) -> Option<BridgeBroadcastCompletion> {
        self.inner.asynchronous.take_broadcast_completion()
    }

    pub fn close_one_terminal_broadcast_claim(&self) -> bool {
        self.inner.asynchronous.close_one_terminal_broadcast_claim()
    }

    pub fn cancel_broadcasts(&self) {
        self.inner.asynchronous.cancel();
    }

    pub(crate) fn take_terminal_broadcast_job(&self) -> Option<(WorkerSubmitErrorKind, Job)> {
        self.inner.asynchronous.take_terminal_job()
    }

    pub fn close_one_terminal_retired_page(&self) -> bool {
        self.inner.asynchronous.close_one_terminal_retired_page()
    }

    pub fn connections(&self) -> Vec<ShellConnectionId> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").keys().copied().collect()
    }

    /// 🧭️ The last `ShellState`/`ShellStatePatch` frame received on this connection. Applying a patch
    /// onto a base state is `💻️os/🔨️modules/🖥️shell`'s reducer's job, not this facet's — a caller that
    /// needs the merged/canonical state feeds whatever this returns through that reducer.
    pub fn last_shell_state(&self, id: ShellConnectionId) -> Option<ShellToGateway> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_shell_state.clone())
    }

    pub fn last_instances(&self, id: ShellConnectionId) -> Option<Vec<BridgeInstanceRef>> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_instances.clone())
    }

    /// 🚩️ The flags the shell on `id` declared in its `Hello` — `BridgeFlags::NONE` for a
    /// connection that no longer exists, which is the same answer as "claims nothing".
    pub fn shell_flags(&self, id: ShellConnectionId) -> BridgeFlags {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).map_or(BridgeFlags::NONE, |entry| entry.flags)
    }

    /// 🗿️ The shell's most recent `AppFrames` reply on `id`. The caller MUST check `in_reply_to`
    /// against the `seq` it sent — a stale reply from an earlier exchange is otherwise
    /// indistinguishable from its own.
    pub fn last_app_frames(&self, id: ShellConnectionId) -> Option<(u64, String, Vec<Vec<u8>>)> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_app_frames.clone())
    }

    pub fn last_command_result(&self, id: ShellConnectionId) -> Option<(u64, bool, Option<String>)> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_command_result.clone())
    }

    pub fn last_approval(&self, id: ShellConnectionId) -> Option<(String, ApprovalDecision, Option<String>)> {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).and_then(|entry| entry.last_approval.clone())
    }

    /// 📥️ Drains every unread human turn this connection typed at the agent, oldest first. Read-once
    /// by design: the agent polls `semio://ui/agent-messages`, and a turn it has been handed is not
    /// handed to it again.
    pub fn take_agent_messages(&self, id: ShellConnectionId) -> Vec<AgentInboxMessage> {
        let mut connections = self.inner.connections.lock().expect("bridge connections lock poisoned");
        match connections.get_mut(&id) {
            Some(entry) => entry.agent_inbox.drain(..).collect(),
            None => Vec::new(),
        }
    }

    /// 👀️ How many unread human turns this connection is holding — a non-destructive peek, for a
    /// resource listing or a test that must not consume the queue.
    pub fn pending_agent_message_count(&self, id: ShellConnectionId) -> usize {
        self.inner.connections.lock().expect("bridge connections lock poisoned").get(&id).map(|entry| entry.agent_inbox.len()).unwrap_or(0)
    }
}

/// 🕐️ Wall-clock milliseconds — this facet's own single clock read site, same convention every
/// other facet in this crate restates rather than sharing a private helper across module roots.
fn bridge_wall_now_ms() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

fn try_send_bridge_frame(entry: &ConnectionEntry, frame: GatewayToShell) -> Result<(), GatewayToShell> {
    entry.outbox.try_send(frame)
}

//#endregion 🔖️BridgeHandle

//#region 💬️AgentConversation

/// 💬️ The gateway's own emitter for the agent-conversation frames — the ONE place `tools/call`
/// traffic becomes something a shell can render. It is deliberately not a second log: it carries no
/// state beyond the invocation counter, reads nothing, and publishes only what the real dispatch
/// path hands it, so the panel shows the agent's actual calls or nothing at all.
///
/// Every emission is best-effort by construction. A gateway with no `/bridge` (stdio), a bridge with
/// no shell attached, or a shell whose outbox is momentarily saturated must never fail or delay a
/// tool call — the agent's work is the authority, the shell's view of it is a projection.
pub struct AgentConversation {
    pub bridge: BridgeSlot,
    pub label: String,
    next_invocation: AtomicU64,
}

impl AgentConversation {
    /// 🏗️ `label` is the connected client's own name (e.g. `claude-code`) — what
    /// `GatewayToShell::AgentPresence` already publishes and what the panel shows as the speaker.
    pub fn new(bridge: BridgeSlot, label: impl Into<String>) -> Self {
        Self { bridge, label: label.into(), next_invocation: AtomicU64::new(1) }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn publish(&self, frame: GatewayToShell) {
        if let Some(handle) = self.bridge.get() {
            let _ = handle.broadcast(frame);
        }
    }

    /// 🛠️ Announces one tool call and returns its invocation id. Emitted BEFORE the handler runs, so
    /// a long tool is visible in the shell while it is still running rather than only afterwards;
    /// the `AgentPresence{active:true}` frame is what turns the existing presence dot amber.
    ///
    /// 🛑️ Every tool call is a real job in the ONE process-wide registry, keyed by the invocation id
    /// the panel already renders — so the panel's cancel control, `job_cancel` and
    /// `semio://job/{id}` all act on the same record, and a handler that polls
    /// `is_cancel_requested` sees the human's stop. `report_progress` moves it off `Pending`
    /// (a `Pending` job finishes as `Cancelled` immediately; this one is genuinely running).
    pub fn begin_tool_call(&self, tool_name: &str, arguments: &serde_json::Value) -> String {
        let invocation_id = format!("inv_{}", self.next_invocation.fetch_add(1, Ordering::Relaxed));
        job_registry().begin_with_id(invocation_id.clone(), "toolCall");
        job_registry().report_progress(&invocation_id, 0.0, Some(tool_name.to_string()));
        let rendered = if arguments.is_null() { String::new() } else { truncate_conversation_text(&serde_json::to_string(arguments).unwrap_or_default()) };
        self.publish(GatewayToShell::AgentToolCall { invocation_id: invocation_id.clone(), tool_name: tool_name.to_string(), arguments: rendered });
        self.publish(GatewayToShell::AgentPresence { active: true, label: self.label.clone(), invocation_id: Some(invocation_id.clone()) });
        invocation_id
    }

    /// 🧾️ Closes the invocation `begin_tool_call` opened, then returns presence to idle.
    ///
    /// 🛑️ A call the human cancelled settles as `Cancelled`, not as whatever the handler happened to
    /// return; `finish` is a no-op once terminal, so a cancel that already closed the record wins.
    pub fn finish_tool_call(&self, invocation_id: &str, tool_name: &str, ok: bool, summary: &str) {
        let registry = job_registry();
        if registry.is_cancel_requested(invocation_id) {
            registry.mark_cancelled(invocation_id);
        } else if ok {
            registry.succeed(invocation_id, serde_json::json!({ "tool": tool_name }));
        } else {
            registry.fail(invocation_id, GatewayError::new(GatewayErrorCode::Internal, summary.to_string()));
        }
        self.publish(GatewayToShell::AgentToolResult { invocation_id: invocation_id.to_string(), tool_name: tool_name.to_string(), ok, summary: truncate_conversation_text(summary) });
        self.publish(GatewayToShell::AgentPresence { active: false, label: self.label.clone(), invocation_id: None });
    }
}

//#endregion 💬️AgentConversation
