use crate::tui::ansi::AnsiPatch;
use crate::tui::event::Event;
use crate::tui::geometry::Size;
use crate::tui::widget::CursorSpec;
use std::io;

#[derive(Debug)]
pub struct BackendError {
    pub message: String,
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for BackendError {
    fn from(error: io::Error) -> Self {
        Self { message: error.to_string() }
    }
}

//#region 📋️Clipboard
/// 📋️ Completion delivered by a clipboard I/O worker mailbox.
#[derive(Debug)]
pub enum ClipboardResult {
    Copied,
    Pasted(String),
    Failed(BackendError),
}

/// 🗒️ Enqueue-only clipboard access. Event callbacks submit and return; a later tick polls.
pub trait Clipboard {
    fn enqueue_copy(&mut self, text: String);
    fn enqueue_paste(&mut self);
    fn poll(&mut self) -> Option<ClipboardResult>;
}

fn base64_encode(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i];
        let b1 = if i + 1 < data.len() { data[i + 1] } else { 0 };
        let b2 = if i + 2 < data.len() { data[i + 2] } else { 0 };
        out.push(T[(b0 >> 2) as usize] as char);
        out.push(T[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < data.len() {
            out.push(T[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < data.len() {
            out.push(T[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}

/// 📎️ Builds an OSC 52 clipboard-set sequence for selection `c`.
pub fn osc52_copy_sequence(text: &str) -> String {
    let mut out = String::new();
    out.push('\u{1b}');
    out.push_str("]52;c;");
    out.push_str(&base64_encode(text.as_bytes()));
    out.push('\u{07}');
    out
}

fn clip_err(message: impl Into<String>) -> BackendError {
    BackendError { message: message.into() }
}

fn native_copy_worker(text: &str) -> Result<(), BackendError> {
    #[cfg(all(unix, not(target_arch = "wasm32")))]
    {
        use std::io::Write;
        use std::process as system_process;
        let candidates: &[&[&str]] = if cfg!(target_os = "macos") { &[&["pbcopy"]] } else { &[&["wl-copy"], &["xclip", "-selection", "clipboard"], &["xsel", "--clipboard", "--input"]] };
        for argv in candidates {
            let mut child = match system_process::Command::new(argv[0]).args(&argv[1..]).stdin(system_process::Stdio::piped()).stdout(system_process::Stdio::null()).stderr(system_process::Stdio::null()).spawn() {
                Ok(c) => c,
                Err(_) => continue,
            };
            if let Some(mut stdin) = child.stdin.take() {
                if stdin.write_all(text.as_bytes()).is_err() {
                    continue;
                }
            }
            if child.wait().is_ok_and(|s| s.success()) {
                return Ok(());
            }
        }
        Err(clip_err("no native clipboard tool available"))
    }
    #[cfg(all(windows, not(target_arch = "wasm32")))]
    {
        use std::io::Write;
        use std::process as system_process;
        let mut child = system_process::Command::new("clip").stdin(system_process::Stdio::piped()).stdout(system_process::Stdio::null()).stderr(system_process::Stdio::null()).spawn().map_err(|e| clip_err(e.to_string()))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).map_err(|e| clip_err(e.to_string()))?;
        }
        if child.wait().is_ok_and(|s| s.success()) {
            return Ok(());
        }
        Err(clip_err("clip.exe failed"))
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = text;
        Err(clip_err("native clipboard unavailable on wasm"))
    }
}

fn native_paste_worker() -> Result<String, BackendError> {
    #[cfg(all(unix, not(target_arch = "wasm32")))]
    {
        use std::process as system_process;
        let candidates: &[&[&str]] = if cfg!(target_os = "macos") { &[&["pbpaste"]] } else { &[&["wl-paste", "--no-newline"], &["xclip", "-selection", "clipboard", "-o"], &["xsel", "--clipboard", "--output"]] };
        for argv in candidates {
            if let Ok(out) = system_process::Command::new(argv[0]).args(&argv[1..]).output() {
                if out.status.success() {
                    return Ok(String::from_utf8_lossy(&out.stdout).into_owned());
                }
            }
        }
        Err(clip_err("no native clipboard tool available"))
    }
    #[cfg(all(windows, not(target_arch = "wasm32")))]
    {
        use std::process as system_process;
        let out = system_process::Command::new("powershell").args(["-NoProfile", "-Command", "Get-Clipboard"]).output().map_err(|e| clip_err(e.to_string()))?;
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).trim_end_matches(&['\r', '\n'][..]).to_string())
        } else {
            Err(clip_err("Get-Clipboard failed"))
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        Err(clip_err("native clipboard unavailable on wasm"))
    }
}

/// 🗃️ Native clipboard mailbox backed by the process-wide worker pool's I/O lane.
pub struct HostClipboard {
    pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    pending: std::collections::VecDeque<std::sync::mpsc::Receiver<ClipboardResult>>,
}

impl HostClipboard {
    pub fn new(pool: std::sync::Arc<semio_framework_async::WorkerPool>) -> Self {
        Self { pool, pending: std::collections::VecDeque::new() }
    }

    fn submit(&mut self, operation: impl FnOnce() -> ClipboardResult + Send + 'static) {
        let (sender, receiver) = std::sync::mpsc::channel();
        self.pool.submit(
            semio_framework_async::Lane::Io,
            Box::new(move || {
                let _ = sender.send(operation());
            }),
        );
        self.pending.push_back(receiver);
    }
}

impl Clipboard for HostClipboard {
    fn enqueue_copy(&mut self, text: String) {
        self.submit(move || native_copy_worker(&text).map_or_else(ClipboardResult::Failed, |_| ClipboardResult::Copied));
    }

    fn enqueue_paste(&mut self) {
        self.submit(|| native_paste_worker().map_or_else(ClipboardResult::Failed, ClipboardResult::Pasted));
    }

    fn poll(&mut self) -> Option<ClipboardResult> {
        let result = self.pending.front()?.try_recv();
        match result {
            Ok(result) => {
                self.pending.pop_front();
                Some(result)
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.pending.pop_front();
                Some(ClipboardResult::Failed(clip_err("clipboard worker disconnected")))
            }
        }
    }
}

/// 🧠️ In-memory clipboard for tests and headless hosts.
#[derive(Default)]
pub struct MemoryClipboard {
    pub text: String,
    pending: std::collections::VecDeque<ClipboardResult>,
}

impl Clipboard for MemoryClipboard {
    fn enqueue_copy(&mut self, text: String) {
        self.text = text;
        self.pending.push_back(ClipboardResult::Copied);
    }

    fn enqueue_paste(&mut self) {
        self.pending.push_back(ClipboardResult::Pasted(self.text.clone()));
    }

    fn poll(&mut self) -> Option<ClipboardResult> {
        self.pending.pop_front()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
include!("../🧪️tests/🔬️backend-clipboard-mailbox/🦀️.rs");
//#endregion 📋️Clipboard

/// 🌈️ How many colours the attached terminal renders faithfully.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorDepth {
    Monochrome,
    Ansi16,
    Ansi256,
    TrueColor,
}

/// 🔠️ Which glyph repertoire the attached terminal renders at the widths the text model assumes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnicodeLevel {
    Ascii,
    Full,
}

/// 🧰️ What the attached terminal can do; emitters and painters degrade to it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capabilities {
    pub color: ColorDepth,
    pub synchronized_output: bool,
    pub unicode: UnicodeLevel,
}

/// ⏰️ A cloneable, thread-safe handle that makes a pending or the next `TerminalBackend::wait` return `Event::Wake`.
#[derive(Clone)]
pub struct Waker(std::sync::Arc<dyn Fn() + Send + Sync>);

impl Waker {
    pub fn new(wake: impl Fn() + Send + Sync + 'static) -> Self {
        Self(std::sync::Arc::new(wake))
    }

    pub fn wake(&self) {
        (self.0)();
    }
}

/// 🔌️ A platform terminal I/O implementation, kept out of the retained-mode core.
pub trait TerminalBackend {
    fn enter(&mut self) -> io::Result<()>;
    fn leave(&mut self) -> io::Result<()>;
    fn size(&self) -> io::Result<Size>;
    fn capabilities(&self) -> Capabilities;
    fn present(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> io::Result<()>;
    fn wait(&mut self, deadline: Option<std::time::Instant>) -> io::Result<Vec<Event>>;
    fn waker(&self) -> Waker;
    fn copy(&mut self, text: &str) -> io::Result<()>;
}

//#region 🔎️Capabilities
fn lowered(env: &dyn Fn(&str) -> Option<String>, name: &str) -> String {
    env(name).unwrap_or_default().to_ascii_lowercase()
}

/// 🔎️ Reads colour depth, glyph repertoire and synchronized-output support from the terminal environment (`COLORTERM`, `TERM`, `TERM_PROGRAM`, `WT_SESSION`, `NO_COLOR`, locale).
pub fn detect_capabilities(env: &dyn Fn(&str) -> Option<String>, windows: bool) -> Capabilities {
    let term = lowered(env, "TERM");
    let colorterm = lowered(env, "COLORTERM");
    let program = lowered(env, "TERM_PROGRAM");
    let present = |name: &str| env(name).is_some_and(|value| !value.is_empty());
    let dumb = term == "dumb";
    let modern = ["kitty", "alacritty", "foot", "ghostty", "wezterm", "contour"].iter().any(|name| term.contains(name));
    let color = if present("NO_COLOR") || dumb {
        ColorDepth::Monochrome
    } else if windows || modern || colorterm.contains("truecolor") || colorterm.contains("24bit") || term.contains("truecolor") || term.contains("direct") || present("WT_SESSION") || present("KITTY_WINDOW_ID") || matches!(program.as_str(), "iterm.app" | "wezterm" | "ghostty" | "vscode" | "hyper") {
        ColorDepth::TrueColor
    } else if term.contains("256color") || program == "apple_terminal" {
        ColorDepth::Ansi256
    } else {
        ColorDepth::Ansi16
    };
    let locale = ["LC_ALL", "LC_CTYPE", "LANG"].iter().map(|name| lowered(env, name)).find(|value| !value.is_empty());
    let unicode = if dumb || term == "linux" || (!windows && locale.is_some_and(|value| !value.contains("utf"))) { UnicodeLevel::Ascii } else { UnicodeLevel::Full };
    let vte_current = env("VTE_VERSION").and_then(|value| value.parse::<u32>().ok()).is_some_and(|version| version >= 6800);
    let synchronized_output = !dumb
        && (present("WT_SESSION")
            || present("KITTY_WINDOW_ID")
            || vte_current
            || matches!(program.as_str(), "iterm.app" | "wezterm" | "ghostty" | "vscode")
            || modern);
    Capabilities { color, synchronized_output, unicode }
}

#[cfg(all(feature = "tui-terminal", not(target_arch = "wasm32"), any(unix, windows)))]
fn process_environment(name: &str) -> Option<String> {
    std::env::var(name).ok()
}
//#endregion 🔎️Capabilities

//#region 🎞️Presenter
/// 🎞️ Turns an engine patch and a cursor request into the bytes one terminal frame writes, degraded to the capabilities.
pub struct FramePresenter {
    capabilities: Capabilities,
    cursor: crate::tui::ansi::CursorEmitter,
}

impl FramePresenter {
    pub fn new(capabilities: Capabilities) -> Self {
        Self { capabilities, cursor: crate::tui::ansi::CursorEmitter::default() }
    }

    /// 🧾️ What this presenter degrades frames to.
    pub fn capabilities(&self) -> Capabilities {
        self.capabilities
    }

    /// 🧼️ Forgets the cursor state after the terminal was re-entered.
    pub fn reset(&mut self) {
        self.cursor.reset();
    }

    /// ✂️ The bytes of one frame, empty when nothing changes.
    pub fn compose(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> String {
        let colored = crate::tui::ansi::quantize_patch(&patch.0, self.capabilities.color);
        self.cursor.frame(&colored, cursor, self.capabilities.synchronized_output)
    }
}
//#endregion 🎞️Presenter

#[cfg(test)]
include!("../🧪️tests/🔬️backend-capabilities/🦀️.rs");

/// 🪣️ Largest clipboard payload `copy` sends as one OSC 52 sequence.
pub const OSC52_LIMIT: usize = 1 << 20;

#[cfg(any(test, all(feature = "tui-terminal", not(target_arch = "wasm32"), any(unix, windows))))]
fn osc52_payload_check(text: &str) -> io::Result<bool> {
    if text.is_empty() {
        return Ok(false);
    }
    if text.len() > OSC52_LIMIT {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "clipboard text exceeds the OSC 52 limit"));
    }
    Ok(true)
}

#[cfg(all(feature = "tui-terminal", not(target_arch = "wasm32"), any(unix, windows)))]
mod native_common {
    use super::*;
    use crate::tui::ansi::{AnsiParser, ClickCounter};
    use std::time::{Duration, Instant};

    pub fn release_owned_terminal_cleanup(owned: &mut bool, succeeded: bool) -> bool {
        if *owned && succeeded {
            *owned = false;
        }
        !*owned
    }

    pub fn terminal_entry_is_available(entered: bool, ansi_setup_owned: bool, platform_cleanup_empty: bool) -> bool {
        !entered && !ansi_setup_owned && platform_cleanup_empty
    }

    pub fn ceil_millis(duration: Duration) -> u64 {
        (duration.as_micros() as u64).div_ceil(1000)
    }

    /// 📥️ Raw terminal bytes in, stamped events out, with the partial-input clock the parser needs.
    pub struct InputPipeline {
        parser: AnsiParser,
        clicks: ClickCounter,
        epoch: Instant,
        last_input: Instant,
    }

    impl InputPipeline {
        pub fn new() -> Self {
            let now = Instant::now();
            Self { parser: AnsiParser::new(), clicks: ClickCounter::default(), epoch: now, last_input: now }
        }

        pub fn feed(&mut self, bytes: &[u8], events: &mut Vec<Event>) {
            let now = Instant::now();
            self.expire_due(now, events);
            let start = events.len();
            self.parser.feed(bytes, events);
            self.last_input = now;
            let now_ms = now.saturating_duration_since(self.epoch).as_millis() as u64;
            self.clicks.stamp_all(&mut events[start..], now_ms);
        }

        pub fn remaining(&self, now: Instant) -> Option<Duration> {
            self.parser.pending_timeout().map(|timeout| (self.last_input + timeout).saturating_duration_since(now))
        }

        pub fn expire_due(&mut self, now: Instant, events: &mut Vec<Event>) {
            if self.remaining(now).is_some_and(|remaining| remaining.is_zero()) {
                self.parser.expire(events);
            }
        }
    }

    /// ⏲️ How long one platform wait may block: the nearer of the caller's deadline and the parser's partial-input deadline.
    pub fn next_timeout(deadline: Option<Instant>, input: &InputPipeline, now: Instant) -> Option<Duration> {
        let until_deadline = deadline.map(|deadline| deadline.saturating_duration_since(now));
        match (until_deadline, input.remaining(now)) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }

    /// 📐️ Remembers the last reported size so spurious resize signals produce no event.
    #[derive(Default)]
    pub struct ResizeTracker {
        last: Option<Size>,
    }

    impl ResizeTracker {
        pub fn baseline(&mut self, size: Size) {
            self.last = Some(size);
        }

        pub fn observe(&mut self, size: Size) -> Option<Event> {
            if size.width == 0 || size.height == 0 || self.last == Some(size) {
                return None;
            }
            self.last = Some(size);
            Some(Event::Resize(size))
        }
    }

    /// 🛟️ Restores the terminal before a panic message prints, only when the panicking thread entered it.
    pub mod panic_restore {
        use std::sync::{Mutex, Once};
        use std::thread::ThreadId;

        static OWNER: Mutex<Option<ThreadId>> = Mutex::new(None);
        static HOOK: Once = Once::new();

        pub fn claim() {
            *OWNER.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(std::thread::current().id());
        }

        pub fn release() {
            *OWNER.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
        }

        pub fn install(fire: fn()) {
            HOOK.call_once(move || {
                let previous = std::panic::take_hook();
                std::panic::set_hook(Box::new(move |info| {
                    let owned = OWNER.try_lock().is_ok_and(|owner| *owner == Some(std::thread::current().id()));
                    if owned {
                        fire();
                    }
                    previous(info);
                }));
            });
        }
    }

    #[cfg(test)]
    include!("../🧪️tests/🔬️backend-native-common/🦀️.rs");
}

#[cfg(all(feature = "tui-terminal", unix, not(target_arch = "wasm32")))]
mod native_unix {
    use super::native_common::*;
    use super::*;
    use crate::tui::ansi::{setup_sequence, teardown_sequence};
    use std::cell::UnsafeCell;
    use std::io::Write;
    use std::mem::MaybeUninit;
    use std::os::unix::io::RawFd;
    use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    const READ_BURST: usize = 1 << 18;

    fn err(message: impl Into<String>) -> io::Error {
        io::Error::other(message.into())
    }

    fn retry_mode_restoration(raw_mode_entered: &mut bool, restore: impl FnOnce() -> bool) -> bool {
        if !*raw_mode_entered {
            return true;
        }
        if restore() {
            *raw_mode_entered = false;
            true
        } else {
            false
        }
    }

    fn write_byte(fd: RawFd) {
        let byte = 1u8;
        unsafe {
            libc::write(fd, (&byte as *const u8).cast(), 1);
        }
    }

    /// 🪢️ A non-blocking self-pipe: any thread or signal handler writes a byte to make `wait` return.
    struct WakePipe {
        read: RawFd,
        write: RawFd,
    }

    impl WakePipe {
        fn new() -> io::Result<Self> {
            let mut fds = [0 as libc::c_int; 2];
            if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
                return Err(io::Error::last_os_error());
            }
            for fd in fds {
                unsafe {
                    let status = libc::fcntl(fd, libc::F_GETFL);
                    libc::fcntl(fd, libc::F_SETFL, status | libc::O_NONBLOCK);
                    let descriptor = libc::fcntl(fd, libc::F_GETFD);
                    libc::fcntl(fd, libc::F_SETFD, descriptor | libc::FD_CLOEXEC);
                }
            }
            Ok(Self { read: fds[0], write: fds[1] })
        }

        fn drain(&self) {
            let mut buf = [0u8; 64];
            loop {
                let n = unsafe { libc::read(self.read, buf.as_mut_ptr().cast(), buf.len()) };
                if n <= 0 || (n as usize) < buf.len() {
                    break;
                }
            }
        }
    }

    impl Drop for WakePipe {
        fn drop(&mut self) {
            unsafe {
                libc::close(self.read);
                libc::close(self.write);
            }
        }
    }

    struct WakeState {
        pipe: WakePipe,
        flag: AtomicBool,
    }

    static SIGNAL_PIPE: AtomicI32 = AtomicI32::new(-1);
    static PENDING_RESIZE: AtomicBool = AtomicBool::new(false);
    static RESTORE_FD: AtomicI32 = AtomicI32::new(-1);

    struct SavedTermios(UnsafeCell<MaybeUninit<libc::termios>>);

    unsafe impl Sync for SavedTermios {}

    static SAVED_TERMIOS: SavedTermios = SavedTermios(UnsafeCell::new(MaybeUninit::uninit()));

    fn arm_restore(fd: RawFd, original: &libc::termios) {
        teardown_sequence();
        unsafe {
            (*SAVED_TERMIOS.0.get()).write(*original);
        }
        RESTORE_FD.store(fd, Ordering::Release);
    }

    fn disarm_restore() -> bool {
        RESTORE_FD.swap(-1, Ordering::AcqRel) >= 0
    }

    fn restore_now() -> bool {
        let fd = RESTORE_FD.swap(-1, Ordering::AcqRel);
        if fd < 0 {
            return false;
        }
        let bytes = teardown_sequence().as_bytes();
        let mut written = 0;
        while written < bytes.len() {
            let n = unsafe { libc::write(fd, bytes[written..].as_ptr().cast(), bytes.len() - written) };
            if n <= 0 {
                break;
            }
            written += n as usize;
        }
        unsafe {
            libc::tcsetattr(fd, libc::TCSANOW, (*SAVED_TERMIOS.0.get()).as_ptr());
        }
        true
    }

    fn restore_for_panic() {
        restore_now();
    }

    extern "C" fn on_resize(_signal: libc::c_int) {
        PENDING_RESIZE.store(true, Ordering::Release);
        let fd = SIGNAL_PIPE.load(Ordering::Acquire);
        if fd >= 0 {
            write_byte(fd);
        }
    }

    extern "C" fn on_terminate(signal: libc::c_int) {
        restore_now();
        unsafe {
            libc::signal(signal, libc::SIG_DFL);
            libc::raise(signal);
        }
    }

    /// 🚦️ Installs the resize and terminate handlers and puts back the previous dispositions on drop.
    struct SignalGuard {
        previous: Vec<(libc::c_int, libc::sigaction)>,
    }

    impl SignalGuard {
        fn install() -> io::Result<Self> {
            let table: [(libc::c_int, extern "C" fn(libc::c_int)); 5] = [(libc::SIGWINCH, on_resize), (libc::SIGINT, on_terminate), (libc::SIGTERM, on_terminate), (libc::SIGHUP, on_terminate), (libc::SIGQUIT, on_terminate)];
            let mut guard = Self { previous: Vec::new() };
            for (signal, handler) in table {
                unsafe {
                    let mut old: libc::sigaction = std::mem::zeroed();
                    if libc::sigaction(signal, std::ptr::null(), &mut old) != 0 {
                        return Err(io::Error::last_os_error());
                    }
                    if signal != libc::SIGWINCH && old.sa_sigaction == libc::SIG_IGN {
                        continue;
                    }
                    let mut action: libc::sigaction = std::mem::zeroed();
                    action.sa_sigaction = handler as usize as libc::sighandler_t;
                    libc::sigemptyset(&mut action.sa_mask);
                    action.sa_flags = libc::SA_RESTART;
                    if libc::sigaction(signal, &action, std::ptr::null_mut()) != 0 {
                        return Err(io::Error::last_os_error());
                    }
                    guard.previous.push((signal, old));
                }
            }
            Ok(guard)
        }
    }

    impl Drop for SignalGuard {
        fn drop(&mut self) {
            for (signal, old) in &self.previous {
                unsafe {
                    libc::sigaction(*signal, old, std::ptr::null_mut());
                }
            }
        }
    }

    /// 👂️ Blocks until `a` or `b` is readable or the timeout passes; `(false, false)` on timeout or interruption.
    #[cfg(not(target_os = "macos"))]
    pub(super) fn wait_readable(a: RawFd, b: RawFd, timeout: Option<Duration>) -> io::Result<(bool, bool)> {
        let mut fds = [libc::pollfd { fd: a, events: libc::POLLIN, revents: 0 }, libc::pollfd { fd: b, events: libc::POLLIN, revents: 0 }];
        let milliseconds = timeout.map_or(-1, |timeout| i32::try_from(ceil_millis(timeout)).unwrap_or(i32::MAX));
        let ready = unsafe { libc::poll(fds.as_mut_ptr(), 2, milliseconds) };
        if ready < 0 {
            let error = io::Error::last_os_error();
            return if error.kind() == io::ErrorKind::Interrupted { Ok((false, false)) } else { Err(error) };
        }
        let hit = |entry: &libc::pollfd| entry.revents & (libc::POLLIN | libc::POLLHUP | libc::POLLERR) != 0;
        Ok((hit(&fds[0]), hit(&fds[1])))
    }

    /// 🎧️ Blocks until `a` or `b` is readable or the timeout passes; `(false, false)` on timeout or interruption.
    #[cfg(target_os = "macos")]
    pub(super) fn wait_readable(a: RawFd, b: RawFd, timeout: Option<Duration>) -> io::Result<(bool, bool)> {
        let highest = a.max(b);
        if highest as usize >= libc::FD_SETSIZE {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "file descriptor exceeds FD_SETSIZE"));
        }
        let mut set: libc::fd_set = unsafe { std::mem::zeroed() };
        unsafe {
            libc::FD_ZERO(&mut set);
            libc::FD_SET(a, &mut set);
            libc::FD_SET(b, &mut set);
        }
        let mut limit = libc::timeval { tv_sec: 0, tv_usec: 0 };
        let limit_ptr = match timeout {
            Some(timeout) => {
                limit = libc::timeval { tv_sec: timeout.as_secs() as libc::time_t, tv_usec: timeout.subsec_micros() as libc::suseconds_t };
                &mut limit as *mut libc::timeval
            }
            None => std::ptr::null_mut(),
        };
        let ready = unsafe { libc::select(highest + 1, &mut set, std::ptr::null_mut(), std::ptr::null_mut(), limit_ptr) };
        if ready < 0 {
            let error = io::Error::last_os_error();
            return if error.kind() == io::ErrorKind::Interrupted { Ok((false, false)) } else { Err(error) };
        }
        unsafe { Ok((libc::FD_ISSET(a, &set), libc::FD_ISSET(b, &set))) }
    }

    /// 🐧️ Raw-mode terminal backend for unix (macOS/Linux), driven by `libc` alone.
    pub struct NativeTerminal {
        fd: RawFd,
        terminal: std::fs::File,
        original: libc::termios,
        input: InputPipeline,
        presenter: FramePresenter,
        resize: ResizeTracker,
        wake: Arc<WakeState>,
        signals: Option<SignalGuard>,
        armed: bool,
        entered: bool,
        ansi_setup_owned: bool,
        raw_mode_entered: bool,
    }

    impl NativeTerminal {
        pub fn new() -> Result<Self, BackendError> {
            use std::os::fd::AsRawFd;
            let terminal = std::fs::OpenOptions::new().read(true).write(true).open("/dev/tty").or_else(|original| {
                use std::os::unix::ffi::OsStrExt;
                let mut device = [0 as libc::c_char; 4096];
                if unsafe { libc::ttyname_r(libc::STDIN_FILENO, device.as_mut_ptr(), device.len()) } != 0 {
                    return Err(original);
                }
                let name = unsafe { std::ffi::CStr::from_ptr(device.as_ptr()) };
                std::fs::OpenOptions::new().read(true).write(true).open(std::ffi::OsStr::from_bytes(name.to_bytes()))
            }).map_err(|error| err(error.to_string()))?;
            let fd = terminal.as_raw_fd();
            let original = unsafe {
                let mut t: libc::termios = std::mem::zeroed();
                if libc::tcgetattr(fd, &mut t) != 0 {
                    return Err(err("tcgetattr failed").into());
                }
                t
            };
            let wake = Arc::new(WakeState { pipe: WakePipe::new()?, flag: AtomicBool::new(false) });
            let capabilities = detect_capabilities(&process_environment, false);
            Ok(Self { fd, terminal, original, input: InputPipeline::new(), presenter: FramePresenter::new(capabilities), resize: ResizeTracker::default(), wake, signals: None, armed: false, entered: false, ansi_setup_owned: false, raw_mode_entered: false })
        }

        fn release_restore(&mut self) -> bool {
            if !self.armed {
                return true;
            }
            self.armed = false;
            disarm_restore()
        }

        fn forget_external_restore(&mut self) {
            self.entered = false;
            self.ansi_setup_owned = false;
            self.raw_mode_entered = false;
            self.signals = None;
            SIGNAL_PIPE.store(-1, Ordering::Release);
            panic_restore::release();
        }

        fn drain_signals(&mut self, events: &mut Vec<Event>) {
            if PENDING_RESIZE.swap(false, Ordering::AcqRel) {
                if let Some(event) = self.size().ok().and_then(|size| self.resize.observe(size)) {
                    events.push(event);
                }
            }
        }

        fn read_available(&mut self, events: &mut Vec<Event>) -> io::Result<()> {
            let mut total = 0usize;
            loop {
                let mut buf = [0u8; 4096];
                let n = unsafe { libc::read(self.fd, buf.as_mut_ptr().cast(), buf.len()) };
                if n > 0 {
                    self.input.feed(&buf[..n as usize], events);
                    total += n as usize;
                    if (n as usize) < buf.len() || total >= READ_BURST {
                        return Ok(());
                    }
                } else if n == 0 {
                    return if total == 0 { Err(io::Error::new(io::ErrorKind::UnexpectedEof, "terminal input closed")) } else { Ok(()) };
                } else {
                    let error = io::Error::last_os_error();
                    match error.kind() {
                        io::ErrorKind::Interrupted => continue,
                        io::ErrorKind::WouldBlock => return Ok(()),
                        _ => return Err(error),
                    }
                }
            }
        }
    }

    impl TerminalBackend for NativeTerminal {
        fn size(&self) -> io::Result<Size> {
            unsafe {
                let mut ws: libc::winsize = std::mem::zeroed();
                if libc::ioctl(self.fd, libc::TIOCGWINSZ, &mut ws) != 0 {
                    return Err(err("TIOCGWINSZ failed"));
                }
                Ok(Size { width: ws.ws_col, height: ws.ws_row })
            }
        }

        fn enter(&mut self) -> io::Result<()> {
            if !terminal_entry_is_available(self.entered, self.ansi_setup_owned, !self.raw_mode_entered) {
                return Err(err("Terminal entry or cleanup is already active"));
            }
            let mut raw = self.original;
            raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG | libc::IEXTEN);
            raw.c_iflag &= !(libc::IXON | libc::ICRNL | libc::BRKINT | libc::INPCK | libc::ISTRIP);
            raw.c_oflag &= !libc::OPOST;
            raw.c_cc[libc::VMIN] = 0;
            raw.c_cc[libc::VTIME] = 0;
            setup_sequence();
            arm_restore(self.fd, &self.original);
            self.armed = true;
            unsafe {
                if libc::tcsetattr(self.fd, libc::TCSANOW, &raw) != 0 {
                    self.release_restore();
                    return Err(err("tcsetattr failed"));
                }
            }
            self.raw_mode_entered = true;
            self.ansi_setup_owned = true;
            if let Err(error) = self.terminal.write_all(setup_sequence().as_bytes()).and_then(|()| self.terminal.flush()).map_err(|e| err(e.to_string())) {
                self.release_restore();
                let teardown = self.teardown_ansi();
                let restoration = self.restore_mode();
                self.entered = false;
                return match (teardown, restoration) {
                    (Ok(()), Ok(())) => Err(error),
                    _ => Err(err("Terminal setup write failed and terminal rollback is pending")),
                };
            }
            self.presenter.reset();
            if let Ok(size) = self.size() {
                self.resize.baseline(size);
            }
            PENDING_RESIZE.store(false, Ordering::Release);
            SIGNAL_PIPE.store(self.wake.pipe.write, Ordering::Release);
            self.signals = SignalGuard::install().ok();
            panic_restore::claim();
            panic_restore::install(restore_for_panic);
            self.entered = true;
            Ok(())
        }

        fn leave(&mut self) -> io::Result<()> {
            if !self.entered && !self.ansi_setup_owned && !self.raw_mode_entered {
                return Ok(());
            }
            if !self.release_restore() {
                self.forget_external_restore();
                return Ok(());
            }
            SIGNAL_PIPE.store(-1, Ordering::Release);
            self.signals = None;
            panic_restore::release();
            let teardown = self.teardown_ansi();
            let restoration = self.restore_mode();
            self.entered = false;
            teardown.and(restoration)
        }

        fn capabilities(&self) -> Capabilities {
            self.presenter.capabilities()
        }

        fn present(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> io::Result<()> {
            let frame = self.presenter.compose(patch, cursor);
            if frame.is_empty() {
                return Ok(());
            }
            self.terminal.write_all(frame.as_bytes()).map_err(|e| err(e.to_string()))?;
            self.terminal.flush().map_err(|e| err(e.to_string()))
        }

        fn wait(&mut self, deadline: Option<Instant>) -> io::Result<Vec<Event>> {
            let mut events = Vec::new();
            loop {
                let now = Instant::now();
                let timeout = next_timeout(deadline, &self.input, now);
                let (tty_ready, wake_ready) = wait_readable(self.fd, self.wake.pipe.read, timeout)?;
                if wake_ready {
                    self.wake.pipe.drain();
                }
                self.drain_signals(&mut events);
                if self.wake.flag.swap(false, Ordering::AcqRel) {
                    events.push(Event::Wake);
                }
                if tty_ready {
                    self.read_available(&mut events)?;
                }
                let after = Instant::now();
                self.input.expire_due(after, &mut events);
                if !events.is_empty() || deadline.is_some_and(|deadline| after >= deadline) {
                    return Ok(events);
                }
            }
        }

        fn waker(&self) -> Waker {
            let wake = Arc::clone(&self.wake);
            Waker::new(move || {
                wake.flag.store(true, Ordering::Release);
                write_byte(wake.pipe.write);
            })
        }

        fn copy(&mut self, text: &str) -> io::Result<()> {
            if !osc52_payload_check(text)? {
                return Ok(());
            }
            self.terminal.write_all(osc52_copy_sequence(text).as_bytes())?;
            self.terminal.flush()
        }
    }

    impl NativeTerminal {
        fn teardown_ansi(&mut self) -> io::Result<()> {
            if !self.ansi_setup_owned {
                return Ok(());
            }
            let result = self.terminal.write_all(teardown_sequence().as_bytes()).and_then(|()| self.terminal.flush()).map_err(|e| err(e.to_string()));
            release_owned_terminal_cleanup(&mut self.ansi_setup_owned, result.is_ok());
            result
        }

        fn restore_mode(&mut self) -> io::Result<()> {
            if !self.raw_mode_entered {
                return Ok(());
            }
            if !retry_mode_restoration(&mut self.raw_mode_entered, || unsafe { libc::tcsetattr(self.fd, libc::TCSANOW, &self.original) } == 0) {
                return Err(err("tcsetattr restore failed"));
            }
            Ok(())
        }
    }

    #[cfg(test)]
    include!("../🧪️tests/🔬️backend-native-unix-unit/🦀️.rs");

    impl Drop for NativeTerminal {
        fn drop(&mut self) {
            let _ = self.leave();
        }
    }
}
#[cfg(all(feature = "tui-terminal", unix, not(target_arch = "wasm32")))]
pub use native_unix::NativeTerminal;

#[cfg(all(feature = "tui-terminal", windows))]
mod native_windows {
    use super::native_common::*;
    use super::*;
    use crate::tui::ansi::{setup_sequence, teardown_sequence};
    use crate::tui::component::windows_abi::{
        CreateEventW, GetConsoleCP, GetConsoleMode, GetConsoleOutputCP, GetConsoleScreenBufferInfo, GetNumberOfConsoleInputEvents, OwnedHandle, ReadConsoleInputW, SetConsoleCP, SetConsoleCtrlHandler, SetConsoleMode, SetConsoleOutputCP, SetEvent, WaitForMultipleObjects, WriteFile,
        CONSOLE_SCREEN_BUFFER_INFO, CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT, DISABLE_NEWLINE_AUTO_RETURN, ENABLE_ECHO_INPUT, ENABLE_EXTENDED_FLAGS, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT, ENABLE_QUICK_EDIT_MODE, ENABLE_VIRTUAL_TERMINAL_INPUT,
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, ENABLE_WINDOW_INPUT, HANDLE, INFINITE, INPUT_RECORD, KEY_EVENT, VK_MENU, VK_SPACE, VK_2, CONTROL_KEYS, WAIT_OBJECT_0, WAIT_TIMEOUT, WINDOW_BUFFER_SIZE_EVENT,
    };
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    const RECORD_BATCH: usize = 128;
    const SIZE_POLL: Duration = Duration::from_millis(250);

    fn err(message: impl Into<String>) -> io::Error {
        io::Error::other(message.into())
    }

    /// 🔔️ An auto-reset Win32 event other threads set to make `wait` return.
    struct WakeEvent(OwnedHandle);

    unsafe impl Send for WakeEvent {}
    unsafe impl Sync for WakeEvent {}

    impl WakeEvent {
        fn new() -> io::Result<Self> {
            let handle = unsafe { CreateEventW(std::ptr::null(), 0, 0, std::ptr::null()) };
            unsafe { OwnedHandle::from_raw(handle) }.map(Self).ok_or_else(io::Error::last_os_error)
        }

        fn set(&self) {
            unsafe {
                SetEvent(self.0.as_raw());
            }
        }
    }

    struct WakeState {
        event: WakeEvent,
        flag: AtomicBool,
    }

    /// 🕰️ What ended one multi-object wait.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(super) enum WaitOutcome {
        Input,
        Wake,
        Timeout,
    }

    /// ⏱️ Waits for the console input handle or the wake event, whichever is signalled first.
    pub(super) fn wait_signalled(input: HANDLE, wake: HANDLE, timeout: Option<Duration>) -> io::Result<WaitOutcome> {
        let handles = [input, wake];
        let milliseconds = timeout.map_or(INFINITE, |timeout| ceil_millis(timeout).min(u64::from(INFINITE - 1)) as u32);
        let status = unsafe { WaitForMultipleObjects(2, handles.as_ptr(), 0, milliseconds) };
        match status {
            WAIT_OBJECT_0 => Ok(WaitOutcome::Input),
            status if status == WAIT_OBJECT_0 + 1 => Ok(WaitOutcome::Wake),
            WAIT_TIMEOUT => Ok(WaitOutcome::Timeout),
            _ => Err(io::Error::last_os_error()),
        }
    }

    /// 🔤️ Reassembles UTF-16 console key characters, including surrogate pairs, into UTF-8 bytes.
    #[derive(Default)]
    pub(super) struct Utf16Decoder {
        high: Option<u16>,
    }

    impl Utf16Decoder {
        pub(super) fn push(&mut self, unit: u16, out: &mut Vec<u8>) {
            let scalar = match (self.high.take(), unit) {
                (Some(high), 0xdc00..=0xdfff) => char::from_u32(0x10000 + ((u32::from(high) - 0xd800) << 10) + (u32::from(unit) - 0xdc00)),
                (_, 0xd800..=0xdbff) => {
                    self.high = Some(unit);
                    None
                }
                (_, 0xdc00..=0xdfff) => None,
                (_, unit) => char::from_u32(u32::from(unit)),
            };
            if let Some(scalar) = scalar {
                out.extend_from_slice(scalar.encode_utf8(&mut [0u8; 4]).as_bytes());
            }
        }
    }

    /// 📜️ Turns console input records into the VT byte stream the parser decodes; reports whether the window buffer was resized.
    pub(super) fn records_to_bytes(records: &[INPUT_RECORD], decoder: &mut Utf16Decoder, bytes: &mut Vec<u8>) -> bool {
        let mut resized = false;
        for record in records {
            match record.EventType {
                KEY_EVENT => {
                    let key = unsafe { record.Event.KeyEvent };
                    let control_space = key.bKeyDown != 0 && matches!(key.wVirtualKeyCode, VK_SPACE | VK_2) && key.dwControlKeyState & CONTROL_KEYS != 0;
                    if ((key.bKeyDown != 0 || key.wVirtualKeyCode == VK_MENU) && key.uChar != 0) || control_space {
                        for _ in 0..key.wRepeatCount.clamp(1, 256) {
                            decoder.push(key.uChar, bytes);
                        }
                    }
                }
                WINDOW_BUFFER_SIZE_EVENT => resized = true,
                _ => {}
            }
        }
        resized
    }

    struct SavedConsole {
        stdin: usize,
        stdout: usize,
        original_in: u32,
        original_out: u32,
        input_cp: u32,
        output_cp: u32,
    }

    static SAVED: Mutex<Option<SavedConsole>> = Mutex::new(None);

    fn lock_saved() -> std::sync::MutexGuard<'static, Option<SavedConsole>> {
        SAVED.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn arm_restore(saved: SavedConsole) {
        teardown_sequence();
        *lock_saved() = Some(saved);
    }

    fn disarm_restore() -> bool {
        lock_saved().take().is_some()
    }

    fn restore_saved(saved: &SavedConsole) {
        let bytes = teardown_sequence().as_bytes();
        let mut written = 0u32;
        unsafe {
            WriteFile(saved.stdout as HANDLE, bytes.as_ptr(), bytes.len() as u32, &mut written, std::ptr::null_mut());
            SetConsoleMode(saved.stdin as HANDLE, saved.original_in);
            SetConsoleMode(saved.stdout as HANDLE, saved.original_out);
            SetConsoleCP(saved.input_cp);
            SetConsoleOutputCP(saved.output_cp);
        }
    }

    fn restore_for_panic() {
        if let Ok(mut slot) = SAVED.try_lock() {
            if let Some(saved) = slot.take() {
                restore_saved(&saved);
            }
        }
    }

    unsafe extern "system" fn on_console_control(kind: u32) -> i32 {
        if matches!(kind, CTRL_BREAK_EVENT | CTRL_CLOSE_EVENT | CTRL_LOGOFF_EVENT | CTRL_SHUTDOWN_EVENT) {
            if let Some(saved) = lock_saved().take() {
                restore_saved(&saved);
            }
        }
        0
    }

    /// 🪟️ VT-mode terminal backend using the console input buffer, a wake event and the private first-party Win32 ABI.
    ///
    /// The dashboard main loop owns this backend. `wait` blocks on the console input handle and the wake event together,
    /// reads input records (never a blocking byte read), and turns window-buffer-size records into `Event::Resize`.
    pub struct NativeTerminal {
        stdin: HANDLE,
        stdout: HANDLE,
        _console_input: std::fs::File,
        _console_output: std::fs::File,
        original_in: u32,
        original_out: u32,
        original_input_cp: u32,
        original_output_cp: u32,
        input: InputPipeline,
        presenter: FramePresenter,
        resize: ResizeTracker,
        utf16: Utf16Decoder,
        wake: Arc<WakeState>,
        armed: bool,
        entered: bool,
        ansi_setup_owned: bool,
        modes: ConsoleModeOwnership,
    }

    #[derive(Default)]
    struct ConsoleModeOwnership {
        stdin_changed: bool,
        stdout_changed: bool,
        input_cp_changed: bool,
        output_cp_changed: bool,
    }

    impl ConsoleModeOwnership {
        fn restore_with(&mut self, mut set_mode: impl FnMut(HANDLE, u32) -> bool, stdin: HANDLE, original_in: u32, stdout: HANDLE, original_out: u32) -> bool {
            let mut restored = true;
            if self.stdin_changed && set_mode(stdin, original_in) {
                self.stdin_changed = false;
            } else if self.stdin_changed {
                restored = false;
            }
            if self.stdout_changed && set_mode(stdout, original_out) {
                self.stdout_changed = false;
            } else if self.stdout_changed {
                restored = false;
            }
            restored
        }

        fn restore_code_pages_with(&mut self, mut input: impl FnMut(u32) -> bool, mut output: impl FnMut(u32) -> bool, original_input: u32, original_output: u32) {
            if self.input_cp_changed && input(original_input) {
                self.input_cp_changed = false;
            }
            if self.output_cp_changed && output(original_output) {
                self.output_cp_changed = false;
            }
        }

        fn is_empty(&self) -> bool {
            !self.stdin_changed && !self.stdout_changed && !self.input_cp_changed && !self.output_cp_changed
        }
    }

    impl NativeTerminal {
        pub fn new() -> Result<Self, BackendError> {
            unsafe {
                use std::os::windows::io::AsRawHandle;
                let console_input = std::fs::OpenOptions::new().read(true).write(true).open("CONIN$").map_err(|error| err(error.to_string()))?;
                let console_output = std::fs::OpenOptions::new().read(true).write(true).open("CONOUT$").map_err(|error| err(error.to_string()))?;
                let stdin = console_input.as_raw_handle();
                let stdout = console_output.as_raw_handle();
                let mut original_in = 0u32;
                let mut original_out = 0u32;
                if GetConsoleMode(stdin, &mut original_in) == 0 || GetConsoleMode(stdout, &mut original_out) == 0 {
                    return Err(err("GetConsoleMode failed").into());
                }
                let original_input_cp = GetConsoleCP();
                let original_output_cp = GetConsoleOutputCP();
                if original_input_cp == 0 || original_output_cp == 0 {
                    return Err(err("GetConsoleCP failed").into());
                }
                let wake = Arc::new(WakeState { event: WakeEvent::new()?, flag: AtomicBool::new(false) });
                let capabilities = detect_capabilities(&process_environment, true);
                Ok(Self {
                    stdin,
                    stdout,
                    original_input_cp,
                    original_output_cp,
                    _console_input: console_input,
                    _console_output: console_output,
                    original_in,
                    original_out,
                    input: InputPipeline::new(),
                    presenter: FramePresenter::new(capabilities),
                    resize: ResizeTracker::default(),
                    utf16: Utf16Decoder::default(),
                    wake,
                    armed: false,
                    entered: false,
                    ansi_setup_owned: false,
                    modes: ConsoleModeOwnership::default(),
                })
            }
        }

        fn release_restore(&mut self) -> bool {
            if !self.armed {
                return true;
            }
            self.armed = false;
            disarm_restore()
        }

        fn forget_external_restore(&mut self) {
            self.entered = false;
            self.ansi_setup_owned = false;
            self.modes = ConsoleModeOwnership::default();
            panic_restore::release();
        }

        fn poll_size(&mut self, events: &mut Vec<Event>) {
            if let Some(event) = self.size().ok().and_then(|size| self.resize.observe(size)) {
                events.push(event);
            }
        }

        fn read_records(&mut self, events: &mut Vec<Event>) -> io::Result<()> {
            let mut pending = 0u32;
            if unsafe { GetNumberOfConsoleInputEvents(self.stdin, &mut pending) } == 0 {
                return Err(io::Error::last_os_error());
            }
            if pending == 0 {
                return Ok(());
            }
            let mut records = [INPUT_RECORD::default(); RECORD_BATCH];
            let mut read = 0u32;
            if unsafe { ReadConsoleInputW(self.stdin, records.as_mut_ptr(), pending.min(RECORD_BATCH as u32), &mut read) } == 0 {
                return Err(io::Error::last_os_error());
            }
            let mut bytes = Vec::new();
            let resized = records_to_bytes(&records[..read as usize], &mut self.utf16, &mut bytes);
            if !bytes.is_empty() {
                self.input.feed(&bytes, events);
            }
            if resized {
                self.poll_size(events);
            }
            Ok(())
        }
    }

    impl TerminalBackend for NativeTerminal {
        fn size(&self) -> io::Result<Size> {
            unsafe {
                let mut info: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();
                if GetConsoleScreenBufferInfo(self.stdout, &mut info) == 0 {
                    return Err(err("GetConsoleScreenBufferInfo failed"));
                }
                let width = (info.srWindow.Right - info.srWindow.Left + 1).max(0) as u16;
                let height = (info.srWindow.Bottom - info.srWindow.Top + 1).max(0) as u16;
                Ok(Size { width, height })
            }
        }

        fn enter(&mut self) -> io::Result<()> {
            if !terminal_entry_is_available(self.entered, self.ansi_setup_owned, self.modes.is_empty()) {
                return Err(err("Terminal entry or cleanup is already active"));
            }
            setup_sequence();
            arm_restore(SavedConsole { stdin: self.stdin as usize, stdout: self.stdout as usize, original_in: self.original_in, original_out: self.original_out, input_cp: self.original_input_cp, output_cp: self.original_output_cp });
            self.armed = true;
            unsafe {
                if SetConsoleOutputCP(65001) == 0 {
                    self.release_restore();
                    return Err(err("SetConsoleOutputCP failed"));
                }
                self.modes.output_cp_changed = true;
                if SetConsoleCP(65001) == 0 {
                    self.release_restore();
                    let _ = self.restore_modes();
                    return Err(err("SetConsoleCP failed"));
                }
                self.modes.input_cp_changed = true;
                let out_mode = self.original_out | ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN;
                let in_mode = (self.original_in | ENABLE_VIRTUAL_TERMINAL_INPUT | ENABLE_WINDOW_INPUT | ENABLE_EXTENDED_FLAGS) & !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT | ENABLE_QUICK_EDIT_MODE);
                if SetConsoleMode(self.stdout, out_mode) == 0 {
                    self.release_restore();
                    let _ = self.restore_modes();
                    return Err(err("SetConsoleMode failed"));
                }
                self.modes.stdout_changed = true;
                if SetConsoleMode(self.stdin, in_mode) == 0 {
                    self.release_restore();
                    return match self.restore_modes() {
                        Ok(()) => Err(err("SetConsoleMode failed")),
                        Err(_) => Err(err("SetConsoleMode failed and rollback is pending")),
                    };
                }
                self.modes.stdin_changed = true;
            }
            self.ansi_setup_owned = true;
            if let Err(error) = self.write_raw(setup_sequence().as_bytes()) {
                self.release_restore();
                let teardown = self.teardown_ansi();
                let restoration = self.restore_modes();
                self.entered = false;
                return match (teardown, restoration) {
                    (Ok(()), Ok(())) => Err(error),
                    _ => Err(err("Terminal setup write failed and rollback is pending")),
                };
            }
            self.presenter.reset();
            if let Ok(size) = self.size() {
                self.resize.baseline(size);
            }
            unsafe {
                SetConsoleCtrlHandler(Some(on_console_control), 1);
            }
            panic_restore::claim();
            panic_restore::install(restore_for_panic);
            self.entered = true;
            Ok(())
        }

        fn leave(&mut self) -> io::Result<()> {
            if !self.entered && !self.ansi_setup_owned && self.modes.is_empty() {
                return Ok(());
            }
            unsafe {
                SetConsoleCtrlHandler(Some(on_console_control), 0);
            }
            if !self.release_restore() {
                self.forget_external_restore();
                return Ok(());
            }
            panic_restore::release();
            let teardown = self.teardown_ansi();
            let restoration = self.restore_modes();
            self.entered = false;
            teardown.and(restoration)
        }

        fn capabilities(&self) -> Capabilities {
            self.presenter.capabilities()
        }

        fn present(&mut self, patch: &AnsiPatch, cursor: Option<CursorSpec>) -> io::Result<()> {
            let frame = self.presenter.compose(patch, cursor);
            if frame.is_empty() {
                return Ok(());
            }
            self.write_raw(frame.as_bytes())
        }

        fn wait(&mut self, deadline: Option<Instant>) -> io::Result<Vec<Event>> {
            let mut events = Vec::new();
            loop {
                let now = Instant::now();
                let timeout = Some(next_timeout(deadline, &self.input, now).map_or(SIZE_POLL, |timeout| timeout.min(SIZE_POLL)));
                match wait_signalled(self.stdin, self.wake.event.0.as_raw(), timeout)? {
                    WaitOutcome::Input => self.read_records(&mut events)?,
                    WaitOutcome::Wake => {
                        if self.wake.flag.swap(false, Ordering::AcqRel) {
                            events.push(Event::Wake);
                        }
                    }
                    WaitOutcome::Timeout => {}
                }
                self.poll_size(&mut events);
                let after = Instant::now();
                self.input.expire_due(after, &mut events);
                if !events.is_empty() || deadline.is_some_and(|deadline| after >= deadline) {
                    return Ok(events);
                }
            }
        }

        fn waker(&self) -> Waker {
            let wake = Arc::clone(&self.wake);
            Waker::new(move || {
                wake.flag.store(true, Ordering::Release);
                wake.event.set();
            })
        }

        fn copy(&mut self, text: &str) -> io::Result<()> {
            if !osc52_payload_check(text)? {
                return Ok(());
            }
            self.write_raw(osc52_copy_sequence(text).as_bytes())
        }
    }

    impl NativeTerminal {
        fn teardown_ansi(&mut self) -> io::Result<()> {
            if !self.ansi_setup_owned {
                return Ok(());
            }
            let result = self.write_raw(teardown_sequence().as_bytes());
            release_owned_terminal_cleanup(&mut self.ansi_setup_owned, result.is_ok());
            result
        }

        fn restore_modes(&mut self) -> io::Result<()> {
            unsafe {
                let restored = self.modes.restore_with(|handle, mode| SetConsoleMode(handle, mode) != 0, self.stdin, self.original_in, self.stdout, self.original_out);
                self.modes.restore_code_pages_with(|page| SetConsoleCP(page) != 0, |page| SetConsoleOutputCP(page) != 0, self.original_input_cp, self.original_output_cp);
                if restored && self.modes.is_empty() {
                    Ok(())
                } else {
                    Err(err("SetConsoleMode restore failed"))
                }
            }
        }

        fn write_raw(&self, bytes: &[u8]) -> io::Result<()> {
            let mut offset = 0usize;
            while offset < bytes.len() {
                let mut written = 0u32;
                let chunk = (bytes.len() - offset).min(1 << 20) as u32;
                unsafe {
                    if WriteFile(self.stdout, bytes[offset..].as_ptr(), chunk, &mut written, std::ptr::null_mut()) == 0 {
                        return Err(err("WriteFile failed"));
                    }
                }
                if written == 0 {
                    return Err(err("WriteFile wrote nothing"));
                }
                offset += written as usize;
            }
            Ok(())
        }
    }

    #[cfg(test)]
    include!("../🧪️tests/🔬️backend-native-windows-unit/🦀️.rs");

    impl Drop for NativeTerminal {
        fn drop(&mut self) {
            let _ = self.leave();
        }
    }
}
#[cfg(all(feature = "tui-terminal", windows))]
pub use native_windows::NativeTerminal;
