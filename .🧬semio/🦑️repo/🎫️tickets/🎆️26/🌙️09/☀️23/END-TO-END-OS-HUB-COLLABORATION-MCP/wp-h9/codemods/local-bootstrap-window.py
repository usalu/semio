"""Local-bootstrap pipe: time-windowed replay set, refusals answered on the pipe, a frame pump that observes EOF in every phase."""
p = "/Users/ueli/Documents/semio/🌎️hub/🚀️local-bootstrap/🦀️.rs"
s = open(p, encoding="utf-8").read()

def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:80], s.count(old))
    s = s.replace(old, new)

rep('''pub const LOCAL_BOOTSTRAP_REPLAY_MAX: usize = 64;''', '''/// ⏳️ Exchange ids one validity window (`LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS`) admits. An id leaves
/// the window once its own `expiresAt` has passed, so a long-lived launcher may exchange for ever at
/// up to this many exchanges per 15 s; a burst above it is answered `resource-limit`, never an exit.
pub const LOCAL_BOOTSTRAP_REPLAY_WINDOW_MAX: usize = 64;''')

rep('''struct ExchangeSlots<const N: usize> {
    values: [Option<String>; N],
}
''', '''/// ⏳️ Exchange ids admitted inside their own validity window. An id whose `expiresAt` is past can
/// never validate again — every frame carrying it is refused by its own window — so it is evicted
/// instead of pinning a slot for the rest of the run.
struct ExchangeWindow<const N: usize> {
    values: [Option<(String, i64)>; N],
}

impl<const N: usize> ExchangeWindow<N> {
    fn new() -> Self {
        Self { values: std::array::from_fn(|_| None) }
    }

    fn admit(&mut self, value: &str, expires_at: i64, now: i64) -> Result<(), LocalBootstrapRejectCode> {
        for slot in &mut self.values {
            if slot.as_ref().is_some_and(|(_, expiry)| *expiry < now) {
                *slot = None;
            }
        }
        if self.values.iter().flatten().any(|(existing, _)| existing == value) {
            return Err(LocalBootstrapRejectCode::Denied);
        }
        let slot = self.values.iter_mut().find(|slot| slot.is_none()).ok_or(LocalBootstrapRejectCode::ResourceLimit)?;
        *slot = Some((value.to_owned(), expires_at));
        Ok(())
    }
}

struct ExchangeSlots<const N: usize> {
    values: [Option<String>; N],
}
''')

rep('''pub struct InheritedLocalBootstrapTransport {
    run_id: String,
    channel_key: SecretChannelKey,
    profiles: Box<[LocalBootstrapProfileWire]>,
    reader: tokio::sync::Mutex<LocalBootstrapIo>,
    writer: tokio::sync::Mutex<LocalBootstrapIo>,
    incoming_sequence: AtomicU64,
    outgoing_sequence: AtomicU64,
    consumed: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_REPLAY_MAX>>,
    pending: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>>,
    cancelled: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>>,
    ready: AtomicBool,
    shutdown: AtomicBool,
}

impl InheritedLocalBootstrapTransport {
    pub async fn open_inherited(context: &IdentityVerificationContext<'_>) -> DirectoryResult<Arc<Self>> {
        let file = inherited_bootstrap_file()?;
        let writer = file.try_clone().map_err(|_| unavailable())?;''', '''/// 📥️ One frame the pump read off the pipe, or the failure that ended the pipe.
type LocalBootstrapInbound = DirectoryResult<Box<[u8]>>;

pub struct InheritedLocalBootstrapTransport {
    run_id: String,
    channel_key: SecretChannelKey,
    profiles: Box<[LocalBootstrapProfileWire]>,
    inbound: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<LocalBootstrapInbound>>,
    writer: tokio::sync::Mutex<LocalBootstrapIo>,
    incoming_sequence: AtomicU64,
    outgoing_sequence: AtomicU64,
    consumed: Mutex<ExchangeWindow<LOCAL_BOOTSTRAP_REPLAY_WINDOW_MAX>>,
    pending: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>>,
    cancelled: Mutex<ExchangeSlots<LOCAL_BOOTSTRAP_OUTSTANDING_MAX>>,
    ready: AtomicBool,
    shutdown: Arc<AtomicBool>,
    closed: Arc<LocalBootstrapClosed>,
}

/// 🚪️ Raised once the pipe ended — the launcher closed it, or a frame broke it — whatever phase the
/// hub is in, so a hub still loading its catalog stops with its launcher instead of outliving it.
#[derive(Default)]
struct LocalBootstrapClosed {
    raised: AtomicBool,
    notify: tokio::sync::Notify,
}

impl LocalBootstrapClosed {
    fn raise(&self) {
        self.raised.store(true, Ordering::Release);
        self.notify.notify_waiters();
    }

    async fn wait(&self) {
        loop {
            let notified = self.notify.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            if self.raised.load(Ordering::Acquire) {
                return;
            }
            notified.await;
        }
    }
}

/// 🧭️ The pump's clock and cancellation: the hub's own bootstrap control, or the transport's shutdown.
struct PumpControl {
    base: Arc<dyn IdentityVerificationControl>,
    shutdown: Arc<AtomicBool>,
}

impl IdentityVerificationControl for PumpControl {
    fn now_ms(&self) -> i64 {
        self.base.now_ms()
    }

    fn is_cancelled(&self) -> bool {
        self.base.is_cancelled() || self.shutdown.load(Ordering::Acquire)
    }

    fn report(&self, _progress: crate::directory::IdentityVerificationProgress) {}
}

/// 📥️ Owns the pipe's read half for the whole run: every admitted frame goes to `accept`, bounded by
/// the outstanding-request limit, and the end of the pipe — EOF or a broken frame — raises `closed`.
async fn pump_local_bootstrap_frames(mut reader: LocalBootstrapIo, frames: tokio::sync::mpsc::Sender<LocalBootstrapInbound>, control: PumpControl, closed: Arc<LocalBootstrapClosed>) {
    loop {
        match read_admitted_frame(&mut reader, &control).await {
            Ok(Some((frame, _))) => {
                if frames.send(Ok(frame)).await.is_err() {
                    break;
                }
            }
            Ok(None) => break,
            Err(error) => {
                if !control.is_cancelled() {
                    let _ = frames.send(Err(error)).await;
                }
                break;
            }
        }
    }
    closed.raise();
}

impl InheritedLocalBootstrapTransport {
    /// 🔐️ Opens the launcher's inherited pipe (descriptor 3), completes its hello, and starts the pump.
    pub async fn open_inherited(context: &IdentityVerificationContext<'_>, control: Arc<dyn IdentityVerificationControl>) -> DirectoryResult<Arc<Self>> {
        Self::open_over(inherited_bootstrap_file()?, context, control).await
    }

    async fn open_over(file: std::fs::File, context: &IdentityVerificationContext<'_>, control: Arc<dyn IdentityVerificationControl>) -> DirectoryResult<Arc<Self>> {
        let writer = file.try_clone().map_err(|_| unavailable())?;''')

rep('''        let channel_key = channel_key?;
        let transport = Arc::new(Self {
            run_id: initialize.run_id,
            channel_key: SecretChannelKey(channel_key),
            profiles: initialize.profiles,
            reader: tokio::sync::Mutex::new(reader),
            writer: tokio::sync::Mutex::new(writer),
            incoming_sequence: AtomicU64::new(0),
            outgoing_sequence: AtomicU64::new(0),
            consumed: Mutex::new(ExchangeSlots::new()),
            pending: Mutex::new(ExchangeSlots::new()),
            cancelled: Mutex::new(ExchangeSlots::new()),
            ready: AtomicBool::new(false),
            shutdown: AtomicBool::new(false),
        });
        context.checkpoint(1, 4)?;
        transport.accept_hello(context).await?;
        context.checkpoint(4, 4)?;
        Ok(transport)
    }

    async fn accept_hello(&self, context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
        let bytes = {
            let mut reader = self.reader.lock().await;
            read_frame(&mut *reader, context).await?.ok_or_else(unavailable)?
        };
        let hello: HelloWire = serde_json::from_slice(&bytes).map_err(|_| denied())?;
        let unsigned =
            HelloUnsigned { schema: &hello.schema, kind: &hello.kind, run_id: &hello.run_id, sequence: hello.sequence, exchange_id: &hello.exchange_id, issued_at: hello.issued_at, expires_at: hello.expires_at, launcher_nonce: &hello.launcher_nonce };
        validate_common(&hello.schema, &hello.kind, "hello", &hello.run_id, &self.run_id, hello.sequence, 1, &hello.exchange_id, hello.issued_at, hello.expires_at, context.control.now_ms())?;''', '''        let channel_key = channel_key?;
        let (frames, inbound) = tokio::sync::mpsc::channel(LOCAL_BOOTSTRAP_OUTSTANDING_MAX);
        let transport = Arc::new(Self {
            run_id: initialize.run_id,
            channel_key: SecretChannelKey(channel_key),
            profiles: initialize.profiles,
            inbound: tokio::sync::Mutex::new(inbound),
            writer: tokio::sync::Mutex::new(writer),
            incoming_sequence: AtomicU64::new(0),
            outgoing_sequence: AtomicU64::new(0),
            consumed: Mutex::new(ExchangeWindow::new()),
            pending: Mutex::new(ExchangeSlots::new()),
            cancelled: Mutex::new(ExchangeSlots::new()),
            ready: AtomicBool::new(false),
            shutdown: Arc::new(AtomicBool::new(false)),
            closed: Arc::new(LocalBootstrapClosed::default()),
        });
        context.checkpoint(1, 4)?;
        transport.accept_hello(&mut reader, context).await?;
        tokio::spawn(pump_local_bootstrap_frames(reader, frames, PumpControl { base: control, shutdown: transport.shutdown.clone() }, transport.closed.clone()));
        context.checkpoint(4, 4)?;
        Ok(transport)
    }

    async fn accept_hello(&self, reader: &mut LocalBootstrapIo, context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
        let bytes = read_frame(reader, context).await?.ok_or_else(unavailable)?;
        let hello: HelloWire = serde_json::from_slice(&bytes).map_err(|_| denied())?;
        let unsigned =
            HelloUnsigned { schema: &hello.schema, kind: &hello.kind, run_id: &hello.run_id, sequence: hello.sequence, exchange_id: &hello.exchange_id, issued_at: hello.issued_at, expires_at: hello.expires_at, launcher_nonce: &hello.launcher_nonce };
        validate_frame(&hello.schema, &hello.kind, "hello", &hello.run_id, &self.run_id, hello.sequence, 1, &hello.exchange_id)?;
        validate_window(hello.issued_at, hello.expires_at, context.control.now_ms())?;''')

old_accept = s[s.index("    fn consume_request(&self, exchange_id: &str) -> DirectoryResult<()> {"):s.index("    async fn issue_response(")]
new_accept = '''    /// 🧾️ The per-request admission of one authentic, in-sequence issue frame. Every refusal here is
    /// the launcher's answer (`reject` with its code); none of them ends the pipe or the hub.
    fn admit_issue(&self, wire: &IssueWire, now: i64) -> Result<&LocalBootstrapProfileWire, LocalBootstrapRejectCode> {
        validate_window(wire.issued_at, wire.expires_at, now).map_err(|_| LocalBootstrapRejectCode::Expired)?;
        validate_bounded_auth_text(&wire.device_instance_id, "device instance", DEVICE_INSTANCE_MAX_BYTES).map_err(|_| LocalBootstrapRejectCode::Denied)?;
        let profile = self.profile(&wire.profile_id, wire.client_class).map_err(|_| LocalBootstrapRejectCode::Denied)?;
        self.consumed.lock().map_err(|_| LocalBootstrapRejectCode::Unavailable)?.admit(&wire.exchange_id, wire.expires_at, now)?;
        if !self.pending.lock().map_err(|_| LocalBootstrapRejectCode::Unavailable)?.insert(&wire.exchange_id) {
            return Err(LocalBootstrapRejectCode::ResourceLimit);
        }
        Ok(profile)
    }

    fn finish_request(&self, exchange_id: &str) {
        if let Ok(mut pending) = self.pending.lock() {
            pending.remove(exchange_id);
        }
        if let Ok(mut cancelled) = self.cancelled.lock() {
            cancelled.remove(exchange_id);
        }
    }

    /// 🚫️ Marks a still-pending exchange cancelled; an exchange that already finished has nothing to
    /// cancel, so the cancelled set never outgrows the pending one.
    fn cancel_pending(&self, exchange_id: &str) {
        let Ok(pending) = self.pending.lock() else { return };
        if !pending.contains(exchange_id) {
            return;
        }
        if let Ok(mut cancelled) = self.cancelled.lock() {
            cancelled.insert(exchange_id);
        }
    }

    /// 💥️ A frame that is not an authentic, in-sequence frame of this run: the pipe itself is broken,
    /// so it stops — the only way a pipe refusal ends the service.
    fn broken(&self) -> DirectoryError {
        self.shutdown.store(true, Ordering::Release);
        self.ready.store(false, Ordering::Release);
        denied()
    }

    async fn accept_next(&self, control: &dyn IdentityVerificationControl) -> DirectoryResult<Option<VerifiedLocalBootstrapRequest>> {
        let transport_control = TransportControl { base: control, shutdown: &self.shutdown };
        let mut inbound = self.inbound.lock().await;
        loop {
            if transport_control.is_cancelled() {
                return Ok(None);
            }
            let bytes = match tokio::time::timeout(std::time::Duration::from_millis(LOCAL_BOOTSTRAP_IDLE_POLL_MS), inbound.recv()).await {
                Err(_) => continue,
                Ok(None) => {
                    self.shutdown.store(true, Ordering::Release);
                    self.ready.store(false, Ordering::Release);
                    return Ok(None);
                }
                Ok(Some(Err(error))) => {
                    self.broken();
                    return Err(error);
                }
                Ok(Some(Ok(bytes))) => bytes,
            };
            let now = control.now_ms();
            let context = IdentityVerificationContext { deadline_ms: checked_deadline(now)?, control: &transport_control };
            let kind: KindWire = serde_json::from_slice(&bytes).map_err(|_| self.broken())?;
            let expected_sequence = self.incoming_sequence.load(Ordering::Acquire).checked_add(1).ok_or_else(resource_limit)?;
            if kind.kind == "issue" {
                let wire: IssueWire = serde_json::from_slice(&bytes).map_err(|_| self.broken())?;
                let unsigned = IssueUnsigned {
                    schema: &wire.schema,
                    kind: &wire.kind,
                    run_id: &wire.run_id,
                    sequence: wire.sequence,
                    exchange_id: &wire.exchange_id,
                    issued_at: wire.issued_at,
                    expires_at: wire.expires_at,
                    profile_id: &wire.profile_id,
                    device_instance_id: &wire.device_instance_id,
                    client_class: wire.client_class,
                };
                validate_frame(&wire.schema, &wire.kind, "issue", &wire.run_id, &self.run_id, wire.sequence, expected_sequence, &wire.exchange_id).map_err(|_| self.broken())?;
                self.verify_proof(&unsigned, &wire.proof).map_err(|_| self.broken())?;
                self.incoming_sequence.store(wire.sequence, Ordering::Release);
                match self.admit_issue(&wire, now) {
                    Ok(profile) => {
                        context.checkpoint(4, 4)?;
                        return Ok(Some(VerifiedLocalBootstrapRequest {
                            request_id: wire.exchange_id.clone(),
                            run_id: self.run_id.clone(),
                            profile_id: profile.profile_id.clone(),
                            identity_provider: LOCAL_BOOTSTRAP_IDENTITY_PROVIDER.to_string(),
                            identity_subject: profile.subject.clone(),
                            display_name: profile.display_name.clone(),
                            device_instance_id: wire.device_instance_id,
                            client_class: wire.client_class,
                        }));
                    }
                    Err(code) => {
                        self.reject_response(&wire.exchange_id, code, &context).await?;
                        continue;
                    }
                }
            }
            if matches!(kind.kind.as_str(), "cancel" | "shutdown") {
                let wire: TerminalInputWire = serde_json::from_slice(&bytes).map_err(|_| self.broken())?;
                let unsigned = TerminalUnsigned { schema: &wire.schema, kind: &wire.kind, run_id: &wire.run_id, sequence: wire.sequence, exchange_id: &wire.exchange_id, issued_at: wire.issued_at, expires_at: wire.expires_at };
                validate_frame(&wire.schema, &wire.kind, &kind.kind, &wire.run_id, &self.run_id, wire.sequence, expected_sequence, &wire.exchange_id).map_err(|_| self.broken())?;
                self.verify_proof(&unsigned, &wire.proof).map_err(|_| self.broken())?;
                self.incoming_sequence.store(wire.sequence, Ordering::Release);
                if validate_window(wire.issued_at, wire.expires_at, now).is_err() {
                    continue;
                }
                if kind.kind == "shutdown" {
                    self.shutdown.store(true, Ordering::Release);
                    self.ready.store(false, Ordering::Release);
                    return Ok(None);
                }
                self.cancel_pending(&wire.exchange_id);
                continue;
            }
            return Err(self.broken());
        }
    }

'''
s = s.replace(old_accept, new_accept)

rep('''    fn cancel<'a>(&'a self, request_id: &'a str) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move {
            if let Ok(mut cancelled) = self.cancelled.lock() {
                if !cancelled.contains(request_id) && !cancelled.insert(request_id) {
                    return Err(resource_limit());
                }
            }
            Ok(())
        })
    }
''', '''    fn cancel<'a>(&'a self, request_id: &'a str) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move {
            self.cancel_pending(request_id);
            Ok(())
        })
    }

    fn closed<'a>(&'a self) -> LocalBootstrapTerminalFuture<'a> {
        Box::pin(async move {
            self.closed.wait().await;
            Ok(())
        })
    }
''')

rep('''pub async fn serve_local_bootstrap(transport: Arc<dyn LocalBootstrapTransport>, directory: Arc<HubDirectories>, control: Arc<dyn IdentityVerificationControl>) -> DirectoryResult<()> {
    let mut tasks = tokio::task::JoinSet::new();
    loop {
        while tasks.len() >= LOCAL_BOOTSTRAP_OUTSTANDING_MAX {
            if let Some(result) = tasks.join_next().await {
                result.map_err(|_| unavailable())??;
            }
        }''', '''/// 🎫️ Serves the pipe until the launcher ends it. A request's own failure is answered on the pipe
/// (`reject`) or retired (an undelivered session is revoked); it never ends the service. Only the end
/// of the pipe, a broken frame, or a panicking request task does.
pub async fn serve_local_bootstrap(transport: Arc<dyn LocalBootstrapTransport>, directory: Arc<HubDirectories>, control: Arc<dyn IdentityVerificationControl>) -> DirectoryResult<()> {
    let mut tasks = tokio::task::JoinSet::new();
    loop {
        while tasks.len() >= LOCAL_BOOTSTRAP_OUTSTANDING_MAX {
            if let Some(Err(join_error)) = tasks.join_next().await {
                if join_error.is_panic() {
                    return Err(unavailable());
                }
            }
        }''')

rep('''        tasks.spawn(async move {
            let deadline_ms = checked_deadline(request_control.now_ms())?;
            let context = IdentityVerificationContext { deadline_ms, control: request_control.as_ref() };
            let result = issue_local_session(request_directory.clone(), &request, &context).await;
            match result {
                Ok(session) => match request_transport.issue(&request, &session, &context).await {
                    Ok(()) => Ok(()),
                    Err(delivery_error) => {
                        request_directory.revoke_auth_session(&session.record.id, "local-bootstrap-delivery-failed", None, &request.request_id).await?.ok_or_else(unavailable)?;
                        Err(delivery_error)
                    }
                },
                Err(_) => {
                    let code = if request_control.is_cancelled() { LocalBootstrapRejectCode::Cancelled } else { LocalBootstrapRejectCode::Unavailable };
                    request_transport.reject(&request.request_id, code, &context).await
                }
            }
        });''', '''        tasks.spawn(async move {
            let Ok(deadline_ms) = checked_deadline(request_control.now_ms()) else { return };
            let context = IdentityVerificationContext { deadline_ms, control: request_control.as_ref() };
            match issue_local_session(request_directory.clone(), &request, &context).await {
                Ok(session) => {
                    if request_transport.issue(&request, &session, &context).await.is_err() {
                        let _ = request_directory.revoke_auth_session(&session.record.id, "local-bootstrap-delivery-failed", None, &request.request_id).await;
                    }
                }
                Err(_) => {
                    let code = if request_control.is_cancelled() { LocalBootstrapRejectCode::Cancelled } else { LocalBootstrapRejectCode::Unavailable };
                    let _ = request_transport.reject(&request.request_id, code, &context).await;
                }
            }
        });''')

rep('''fn validate_common(schema: &str, kind: &str, expected_kind: &str, run_id: &str, expected_run_id: &str, sequence: u64, expected_sequence: u64, exchange_id: &str, issued_at: i64, expires_at: i64, now: i64) -> DirectoryResult<()> {
    if schema != LOCAL_BOOTSTRAP_SCHEMA || kind != expected_kind || run_id != expected_run_id || sequence != expected_sequence {
        return Err(denied());
    }
    decode_hex::<16>(run_id)?;
    decode_hex::<16>(exchange_id)?;
    if issued_at < 0''', '''/// 🔏️ The frame belongs to this run, at the next sequence, with well-formed ids — or the pipe is broken.
fn validate_frame(schema: &str, kind: &str, expected_kind: &str, run_id: &str, expected_run_id: &str, sequence: u64, expected_sequence: u64, exchange_id: &str) -> DirectoryResult<()> {
    if schema != LOCAL_BOOTSTRAP_SCHEMA || kind != expected_kind || run_id != expected_run_id || sequence != expected_sequence {
        return Err(denied());
    }
    decode_hex::<16>(run_id)?;
    decode_hex::<16>(exchange_id)?;
    Ok(())
}

/// ⏱️ The exchange's own validity window holds `now` and is no longer than one exchange deadline.
fn validate_window(issued_at: i64, expires_at: i64, now: i64) -> DirectoryResult<()> {
    if issued_at < 0''')

open(p, "w", encoding="utf-8").write(s)
print("ok")
