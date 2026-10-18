//! 🔔️ Windows console events and spawn: the pseudo-console constants, the console control handler and Ctrl+Break interrupts, the detached spawn and the shared pipe state. See [`super`].

use super::*;

pub(super) const PIPE_BACKLOG_BYTES: usize = 256 * 1024;

pub(super) const CONTROL_C_EXIT: u32 = 0xC000_013A;

pub(super) const WAIT_FOREVER: u32 = u32::MAX;

pub(super) const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: u32 = 0x0000_2000;

pub(super) const CTRL_BREAK_EVENT: u32 = 1;

const ATTACH_PARENT_PROCESS: u32 = u32::MAX;

extern "system" {
    fn SetConsoleCtrlHandler(handler: Option<unsafe extern "system" fn(u32) -> i32>, add: i32) -> i32;
    fn AttachConsole(process: u32) -> i32;
    fn FreeConsole() -> i32;
    fn GenerateConsoleCtrlEvent(event: u32, group: u32) -> i32;
    fn GetConsoleCP() -> u32;
}

/// 🚫 Takes every console control event, so the process that sends one to a console it shares with its
/// target does not receive the default action itself.
unsafe extern "system" fn swallow(_event: u32) -> i32 {
    SWALLOWED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    1
}

static SWALLOWED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🧨️ Sends Ctrl+Break to every process attached to the console of `pid`. A process has one console at
/// a time, so this attaches to the target's pseudo-console for the moment of the event, shielded
/// from the event itself, and returns to the console it had.
///
/// @see https://learn.microsoft.com/windows/console/generateconsolectrlevent
pub(super) fn console_event(pid: u32, event: u32) -> bool {
    static CONSOLE: Mutex<()> = Mutex::new(());
    let Ok(_exclusive) = CONSOLE.lock() else { return false };
    unsafe {
        let attached = GetConsoleCP() != 0;
        FreeConsole();
        let sent = AttachConsole(pid) != 0 && {
            SetConsoleCtrlHandler(Some(swallow), 1);
            let before = SWALLOWED.load(std::sync::atomic::Ordering::SeqCst);
            let sent = GenerateConsoleCtrlEvent(event, 0) != 0;
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
            while sent && SWALLOWED.load(std::sync::atomic::Ordering::SeqCst) == before && std::time::Instant::now() < deadline { std::thread::sleep(std::time::Duration::from_millis(5)); }
            SetConsoleCtrlHandler(Some(swallow), 0);
            sent
        };
        FreeConsole();
        if attached { AttachConsole(ATTACH_PARENT_PROCESS); }
        sent
    }
}

/// 🧮️ Counts console control events instead of ending the process on the first one, so a command can
/// turn them into its documented cancellation steps.
pub fn interrupts() -> &'static std::sync::atomic::AtomicUsize {
    static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    static INSTALL: std::sync::Once = std::sync::Once::new();
    unsafe extern "system" fn count(_event: u32) -> i32 {
        COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        1
    }
    INSTALL.call_once(|| unsafe {
        SetConsoleCtrlHandler(Some(count), 1);
    });
    &COUNT
}

/// 🪟️ Starts a detached process with handle inheritance disabled at the native boundary.
pub fn spawn_detached(cmd: &str, args: &[&str], remove_env: &[&str], cwd: &Path) -> Result<u32, PtyError> {
    use crate::tui::component::windows_abi::STARTUPINFOW;
    let application = to_wide(cmd); let mut command = build_cmdline(cmd, args);
    let directory = to_wide(&cwd.to_string_lossy()); let environment = build_env_block(&[], remove_env).unwrap();
    let mut startup = STARTUPINFOW::default(); startup.cb = size_of::<STARTUPINFOW>() as u32;
    let mut process = PROCESS_INFORMATION::default();
    const DETACHED_PROCESS: u32 = 0x0000_0008;
    const CREATE_BREAKAWAY_FROM_JOB: u32 = 0x0100_0000;
    const ERROR_ACCESS_DENIED: i32 = 5;
    let create = |flags: u32, process: &mut PROCESS_INFORMATION, command: &mut Vec<u16>| unsafe { CreateProcessW(application.as_ptr(), command.as_mut_ptr(), std::ptr::null(), std::ptr::null(), 0, CREATE_UNICODE_ENVIRONMENT | DETACHED_PROCESS | flags, environment.as_ptr().cast(), directory.as_ptr(), &startup, process) };
    let spare = command.clone();
    if create(CREATE_BREAKAWAY_FROM_JOB, &mut process, &mut command) == 0 {
        let refused = std::io::Error::last_os_error();
        command = spare;
        if refused.raw_os_error() != Some(ERROR_ACCESS_DENIED) || create(0, &mut process, &mut command) == 0 {
            return Err(err(format!("CreateProcessW detached failed: {}", std::io::Error::last_os_error())));
        }
    }
    let _process = unsafe { OwnedHandle::from_raw(process.hProcess) };
    let _thread = unsafe { OwnedHandle::from_raw(process.hThread) };
    Ok(process.dwProcessId)
}

/// 📣 What a pseudo-terminal calls when its output, its input room or its exit state changed.
pub type Notifier = Arc<dyn Fn() + Send + Sync>;

/// 🔐 A kernel handle shared with the threads that block on it.
pub(super) struct Shared(pub(super) OwnedHandle);

unsafe impl Send for Shared {}

unsafe impl Sync for Shared {}

impl Shared {
    pub(super) fn raw(&self) -> crate::tui::component::windows_abi::HANDLE { self.0.as_raw() }
}

/// 🚰 A bounded byte queue between the owner of a pseudo-terminal and the thread blocked on its pipe.
#[derive(Default)]
pub(super) struct Pipe {
    pub(super) state: Mutex<PipeState>,
    pub(super) changed: Condvar,
}

#[derive(Default)]
struct PipeState {
    pub(super) bytes: VecDeque<u8>,
    pub(super) closed: bool,
}
