//! 🔭 semio-repo-watch: one std-only file, three native backends (FSEvents, inotify, ReadDirectoryChangesW).
//! Protocol: newline-delimited JSON frames on stdout (schema: resident/schema `WatcherFrame`), commands on stdin.
//! EOF on stdin ends the process, so a watcher can never outlive the service that spawned it.
//! https://developer.apple.com/documentation/coreservices/file_system_events
//! https://man7.org/linux/man-pages/man7/inotify.7.html
//! https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-readdirectorychangesw
#![allow(clippy::missing_safety_doc)]

use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// 📮 Frames queued for the writer thread but not yet written; batches beyond `QUEUED_LIMIT` are replaced by one `overflow` loss so memory stays bounded when the consumer stalls.
static QUEUED: AtomicUsize = AtomicUsize::new(0);
static OVERFLOWED: AtomicBool = AtomicBool::new(false);
const QUEUED_LIMIT: usize = 64 << 20;

enum Out {
    Line(String),
    Exit(i32),
}

/// 📤 The one writer of stdout: every backend hands frames over and returns at once, so a slow consumer never blocks a native callback (a blocked FSEvents dispatch queue makes fseventsd drop events for this client, `UserDropped`).
fn writer() -> &'static Sender<Out> {
    static SENDER: OnceLock<Sender<Out>> = OnceLock::new();
    SENDER.get_or_init(|| {
        let (sender, receiver) = channel::<Out>();
        std::thread::spawn(move || {
            let stdout = std::io::stdout();
            let mut handle = std::io::BufWriter::with_capacity(1 << 16, stdout.lock());
            loop {
                let item = match receiver.try_recv() {
                    Ok(item) => item,
                    Err(TryRecvError::Empty) => {
                        if handle.flush().is_err() {
                            std::process::exit(0);
                        }
                        receiver.recv().unwrap_or(Out::Exit(0))
                    }
                    Err(TryRecvError::Disconnected) => Out::Exit(0),
                };
                match item {
                    Out::Line(line) => {
                        QUEUED.fetch_sub(line.len(), Ordering::Relaxed);
                        if handle.write_all(line.as_bytes()).is_err() || handle.write_all(b"\n").is_err() {
                            std::process::exit(0);
                        }
                    }
                    Out::Exit(code) => {
                        let _ = handle.flush();
                        std::process::exit(code);
                    }
                }
            }
        });
        sender
    })
}

fn emit(line: &str) {
    QUEUED.fetch_add(line.len(), Ordering::Relaxed);
    let _ = writer().send(Out::Line(line.to_string()));
}

/// 🚦 What to do with a batch of `bytes` while `queued` bytes wait: `(emit an overflow loss first, emit the batch, overflowed afterwards)`. Once over the limit every batch is dropped until the queue has drained to half, and the first batch after that is preceded by the loss, so no later fence can be answered without the loss.
fn admit(queued: usize, overflowed: bool, bytes: usize, limit: usize) -> (bool, bool, bool) {
    if overflowed && queued > limit / 2 {
        return (false, false, true);
    }
    if queued + bytes > limit {
        return (false, false, true);
    }
    (overflowed, true, false)
}

fn emit_rows(line: &str) {
    let (loss, batch, overflowed) = admit(QUEUED.load(Ordering::Relaxed), OVERFLOWED.load(Ordering::Relaxed), line.len(), QUEUED_LIMIT);
    OVERFLOWED.store(overflowed, Ordering::Relaxed);
    if loss {
        lost("overflow", "");
    }
    if batch {
        emit(line);
    }
}

/// ⏲ Whether pending rows leave now: the batch is full or its first row waited the latency (a burst without a quiet gap must not grow one record beyond the journal's row bound).
#[cfg(any(test, target_os = "linux", semio_check_linux))]
fn due(rows: usize, waited: std::time::Duration, latency: std::time::Duration, limit: usize) -> bool {
    rows >= limit || waited >= latency
}

/// 🏁 Ends the process once every queued frame is written.
fn finish(code: i32) -> ! {
    let _ = writer().send(Out::Exit(code));
    loop {
        std::thread::park();
    }
}

fn now_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0)
}

fn esc(text: &str, out: &mut String) {
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
}

fn quote(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    esc(text, &mut out);
    out.push('"');
    out
}

fn fail(code: &str, message: &str) -> ! {
    emit(&format!("{{\"v\":1,\"t\":\"error\",\"code\":{},\"message\":{}}}", quote(code), quote(message)));
    finish(2);
}

#[derive(Clone, Default)]
struct Filter {
    rooted: Vec<String>,
    names: Vec<String>,
}

impl Filter {
    fn excluded(&self, rel: &str) -> bool {
        if rel.is_empty() {
            return false;
        }
        for rooted in &self.rooted {
            if rel == rooted || (rel.len() > rooted.len() && rel.starts_with(rooted.as_str()) && rel.as_bytes()[rooted.len()] == b'/') {
                return true;
            }
        }
        if !self.names.is_empty() {
            for part in rel.split('/') {
                if self.names.iter().any(|name| name == part) {
                    return true;
                }
            }
        }
        false
    }
}

struct Config {
    root: String,
    filter: Filter,
    latency_ms: u64,
    since: Option<String>,
    uuid: Option<String>,
    threads: usize,
}

fn parse(arguments: &[String]) -> (String, Config) {
    let mut config = Config { root: String::new(), filter: Filter::default(), latency_ms: 5, since: None, uuid: None, threads: 8 };
    let mut command = String::new();
    let mut at = 0;
    while at < arguments.len() {
        let item = arguments[at].as_str();
        let mut value = || -> String {
            at += 1;
            if at >= arguments.len() {
                fail("argument", &format!("{item} needs a value"));
            }
            arguments[at].clone()
        };
        match item {
            "--root" => config.root = value(),
            "--exclude" => config.filter.rooted.push(value().trim_matches('/').to_string()),
            "--exclude-name" => config.filter.names.push(value()),
            "--latency-ms" => config.latency_ms = value().parse().unwrap_or_else(|_| fail("argument", "latency")),
            "--since" => config.since = Some(value()),
            "--uuid" => config.uuid = Some(value()),
            "--threads" => config.threads = value().parse().unwrap_or(8).clamp(1, 64),
            other if command.is_empty() && !other.starts_with("--") => command = other.to_string(),
            other => fail("argument", &format!("unknown argument {other}")),
        }
        at += 1;
    }
    (command, config)
}

fn canonical_root(root: &str) -> String {
    if root.is_empty() {
        fail("argument", "--root is required");
    }
    match std::fs::canonicalize(root) {
        Ok(path) => strip_verbatim(&path.to_string_lossy()),
        Err(error) => fail("root", &format!("{root}: {error}")),
    }
}

/// 🪟 The plain form of a Windows verbatim path: `\\?\C:\dir` is `C:\dir`, and `\\?\UNC\server\share` is the share `\\server\share` (cutting only the verbatim prefix would leave the invalid `UNC\server\share`).
/// https://learn.microsoft.com/windows/win32/fileio/naming-a-file#win32-file-namespaces
fn strip_verbatim(text: &str) -> String {
    if let Some(share) = text.strip_prefix(r"\\?\UNC\") {
        return format!(r"\\{share}");
    }
    text.strip_prefix(r"\\?\").unwrap_or(text).to_string()
}

/// 🪞 The verbatim form of a plain Windows path, the inverse of `strip_verbatim`: a drive path takes `\\?\`, a share takes `\\?\UNC\`.
#[cfg(any(test, target_os = "windows", semio_check_windows))]
fn to_verbatim(path: &str) -> String {
    let backslashed = path.replace('/', "\\");
    match backslashed.strip_prefix(r"\\") {
        Some(share) if !share.starts_with("?\\") => format!(r"\\?\UNC\{share}"),
        Some(_) => backslashed,
        None => format!(r"\\?\{backslashed}"),
    }
}

#[derive(Clone)]
struct Change {
    kind: char,
    dir: bool,
    path: String,
}

fn emit_batch(sequence: &mut u64, cursor: &str, changes: &mut BTreeMap<String, Change>) {
    if changes.is_empty() {
        return;
    }
    *sequence += 1;
    let mut line = format!("{{\"v\":1,\"t\":\"batch\",\"n\":{},\"at\":{},\"cursor\":{},\"c\":[", sequence, now_ms(), quote(cursor));
    let mut first = true;
    for change in changes.values() {
        if !first {
            line.push(',');
        }
        first = false;
        line.push_str(&format!("[\"{}\",{},{}]", change.kind, if change.dir { 1 } else { 0 }, quote(&change.path)));
    }
    line.push_str("]}");
    changes.clear();
    emit_rows(&line);
}

fn merge(changes: &mut BTreeMap<String, Change>, kind: char, dir: bool, path: String) {
    match changes.get_mut(&path) {
        Some(old) => {
            old.dir |= dir;
            if old.kind != kind {
                old.kind = if (old.kind == 'a' && kind == 'r') || (old.kind == 'r' && kind == 'a') { 'm' } else if old.kind == 'a' || old.kind == 'r' { old.kind } else { kind };
            }
        }
        None => {
            changes.insert(path.clone(), Change { kind, dir, path });
        }
    }
}

/// 🍎 FSEvents item flags and the journal kind of one event.
/// https://developer.apple.com/documentation/coreservices/1455361-fseventstreameventflags
#[cfg(any(test, all(target_os = "macos", not(semio_check_linux), not(semio_check_windows))))]
mod fsevent {
    pub const CREATED: u32 = 0x100;
    pub const REMOVED: u32 = 0x200;
    pub const INODE_META: u32 = 0x400;
    pub const RENAMED: u32 = 0x800;
    pub const MODIFIED: u32 = 0x1000;
    pub const FINDER_INFO: u32 = 0x2000;
    pub const CHANGE_OWNER: u32 = 0x4000;
    pub const XATTR: u32 = 0x8000;
    pub const IS_DIR: u32 = 0x20000;
    pub const CLONED: u32 = 0x40_0000;
    const CHANGES: u32 = CREATED | REMOVED | INODE_META | RENAMED | MODIFIED | FINDER_INFO | CHANGE_OWNER | XATTR;
    pub const MUST_SCAN: u32 = 0x1;
    pub const USER_DROPPED: u32 = 0x2;
    pub const KERNEL_DROPPED: u32 = 0x4;
    pub const HISTORY_DONE: u32 = 0x10;
    pub const ROOT_CHANGED: u32 = 0x20;
    pub const MOUNT: u32 = 0x40;
    pub const UNMOUNT: u32 = 0x80;

    /// 🧭 What one FSEvents event means for the journal.
    #[derive(Debug, PartialEq)]
    pub enum Action {
        HistoryDone,
        Resync,
        Lost(&'static str),
        Subtree,
        Row(char),
        Skip,
    }

    /// 🚑 The decision table of one event (`excluded`: its path lies in an excluded region; `root`: its path is the watch root itself).
    /// `UserDropped` alone is a delivery problem between fseventsd and this client: the volume's event history still holds the events, so the stream is replayed from the last delivered event id instead of declaring a loss. `KernelDropped` means fseventsd itself missed kernel events (the history has the same hole): a loss.
    /// `MustScanSubDirs` without a drop flag is hierarchical coalescing: below the root it becomes one structural row for the subtree (the journal's own meaning of a directory row), at the root a loss.
    pub fn action(flag: u32, excluded: bool, root: bool) -> Action {
        if flag & HISTORY_DONE != 0 {
            return Action::HistoryDone;
        }
        if flag & USER_DROPPED != 0 && flag & KERNEL_DROPPED == 0 {
            return Action::Resync;
        }
        let reason = if flag & ROOT_CHANGED != 0 { "root-changed" } else if flag & KERNEL_DROPPED != 0 { "kernel-dropped" } else if flag & (MOUNT | UNMOUNT) != 0 { "mount" } else { "" };
        if excluded {
            return Action::Skip;
        }
        if !reason.is_empty() {
            return Action::Lost(reason);
        }
        if flag & MUST_SCAN != 0 {
            return if root { Action::Lost("must-scan") } else { Action::Subtree };
        }
        kind(flag).map_or(Action::Skip, Action::Row)
    }

    /// ⏪ The event id a replay starts after: the cursor of the oldest of the last delivered callbacks once `trail` is full (a margin for drops that fseventsd reports a little late), else the stream's start id.
    pub fn resume(trail: &std::collections::VecDeque<u64>, capacity: usize, start: u64) -> u64 {
        if trail.len() >= capacity { trail.front().copied().unwrap_or(start).max(start) } else { start }
    }

    /// 🗃 Whether a dropped delivery can be replayed from the volume history: only on a volume that keeps a persistent event history (FSEvents names it by a device UUID; none on a read-only volume or one with `.fseventsd/no_log`) and only while that history is the one the stream started on (a reset history gets a new UUID). Otherwise the drop stays a loss.
    pub fn replayable(started: &str, now: &str) -> bool {
        !started.is_empty() && started == now
    }

    /// 🧬 `None` for an event whose only item change is a clone: the source of an APFS copy-on-write clone (`clonefile(2)`, `cp -c`, every mirror of the checkout) keeps its bytes, entry and metadata, and the destination always arrives with `CREATED` (or `MODIFIED` when it replaced bytes).
    pub fn kind(flag: u32) -> Option<char> {
        if flag & CLONED != 0 && flag & CHANGES == 0 {
            return None;
        }
        Some(if flag & CREATED != 0 && flag & REMOVED != 0 { 'm' } else if flag & REMOVED != 0 { 'r' } else if flag & CREATED != 0 { 'a' } else if flag & RENAMED != 0 { 'n' } else { 'm' })
    }
}

fn lost(reason: &str, scope: &str) {
    emit(&format!("{{\"v\":1,\"t\":\"lost\",\"at\":{},\"reason\":{},\"scope\":{}}}", now_ms(), quote(reason), quote(scope)));
}

fn stdin_guard() {
    std::thread::spawn(|| {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(text) if text.trim() == "stop" => break,
                Ok(_) => continue,
                Err(_) => break,
            }
        }
        emit("{\"v\":1,\"t\":\"bye\"}");
        finish(0);
    });
}

// ───────────────────────────── scan (portable, parallel) ─────────────────────────────

fn mtime_ms(meta: &std::fs::Metadata) -> f64 {
    match meta.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()) {
        Some(d) => (d.as_secs() as f64) * 1000.0 + (d.subsec_nanos() as f64) / 1e6,
        None => 0.0,
    }
}

#[cfg(unix)]
fn identity(meta: &std::fs::Metadata) -> (u64, u32) {
    use std::os::unix::fs::MetadataExt;
    (meta.ino(), meta.mode())
}

/// 0️⃣ Windows gives `std` neither an inode nor POSIX mode bits without an open handle, and any number made up here would contradict the `lstat` of the consumer (`ino` is the NTFS file id, `mode` is 0o100666 or 0o100444): `0` says "unknown" and the consumers (`frontierFactHoldsV1`) let an unknown fact agree.
#[cfg(not(unix))]
fn identity(_meta: &std::fs::Metadata) -> (u64, u32) {
    (0, 0)
}

fn scan(config: &Config) {
    use std::sync::{Arc, Condvar};
    let root = canonical_root(&config.root);
    let started = now_ms();
    struct Shared {
        queue: Mutex<(Vec<String>, usize)>,
        wake: Condvar,
        directories: Mutex<Vec<(String, f64, u64)>>,
        members: Mutex<Vec<(String, u32)>>,
    }
    let shared = Arc::new(Shared { queue: Mutex::new((vec![String::new()], 1)), wake: Condvar::new(), directories: Mutex::new(Vec::new()), members: Mutex::new(Vec::new()) });
    let filter = Arc::new(config.filter.clone());
    let mut handles = Vec::new();
    for _ in 0..config.threads {
        let shared = shared.clone();
        let filter = filter.clone();
        let root = root.clone();
        handles.push(std::thread::spawn(move || {
            let mut directories = Vec::new();
            let mut members = Vec::new();
            loop {
                let rel = {
                    let mut guard = shared.queue.lock().unwrap();
                    loop {
                        if let Some(item) = guard.0.pop() {
                            break Some(item);
                        }
                        if guard.1 == 0 {
                            break None;
                        }
                        guard = shared.wake.wait(guard).unwrap();
                    }
                };
                let Some(rel) = rel else { break };
                let absolute = if rel.is_empty() { root.clone() } else { format!("{root}/{rel}") };
                if let Ok(meta) = std::fs::symlink_metadata(&absolute) {
                    let (inode, _) = identity(&meta);
                    directories.push((rel.clone(), mtime_ms(&meta), inode));
                }
                let mut subdirectories = Vec::new();
                if let Ok(entries) = std::fs::read_dir(&absolute) {
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        let child = if rel.is_empty() { name } else { format!("{rel}/{name}") };
                        if filter.excluded(&child) {
                            continue;
                        }
                        let Ok(kind) = entry.file_type() else { continue };
                        if kind.is_dir() {
                            subdirectories.push(child);
                        } else if kind.is_file() {
                            if let Ok(meta) = entry.metadata() {
                                members.push((child, identity(&meta).1));
                            }
                        }
                    }
                }
                let mut guard = shared.queue.lock().unwrap();
                guard.1 += subdirectories.len();
                guard.0.extend(subdirectories);
                guard.1 -= 1;
                shared.wake.notify_all();
            }
            shared.directories.lock().unwrap().extend(directories);
            shared.members.lock().unwrap().extend(members);
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    let mut directories = std::mem::take(&mut *shared.directories.lock().unwrap());
    let mut members = std::mem::take(&mut *shared.members.lock().unwrap());
    directories.sort_by(|a, b| a.0.cmp(&b.0));
    members.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = String::with_capacity(members.len() * 64 + directories.len() * 80 + 128);
    out.push_str(&format!("{{\"v\":1,\"t\":\"frontier\",\"capturedAt\":{},\"tookMs\":{},\"directories\":[", started, now_ms() - started));
    for (index, (path, mtime, inode)) in directories.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str(&format!("[{},{},{}]", quote(path), mtime, inode));
    }
    out.push_str("],\"members\":[");
    for (index, (path, mode)) in members.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str(&format!("[{},{}]", quote(path), mode));
    }
    out.push_str("]}");
    emit(&out);
}

// ───────────────────────────── macOS: FSEvents ─────────────────────────────

#[cfg(all(target_os = "macos", not(semio_check_linux), not(semio_check_windows)))]
mod backend {
    use super::*;
    use std::collections::VecDeque;
    use std::ffi::{c_char, c_void, CStr, CString};
    use std::sync::atomic::{AtomicU32, AtomicU64};

    type Ref = *const c_void;
    #[repr(C)]
    struct Context {
        version: isize,
        info: *mut c_void,
        retain: *const c_void,
        release: Option<extern "C" fn(*const c_void)>,
        describe: *const c_void,
    }
    type Callback = extern "C" fn(Ref, *mut c_void, usize, *mut c_void, *const u32, *const u64);

    #[link(name = "CoreServices", kind = "framework")]
    extern "C" {
        fn FSEventStreamCreate(allocator: Ref, callback: Callback, context: *mut Context, paths: Ref, since: u64, latency: f64, flags: u32) -> Ref;
        fn FSEventStreamSetDispatchQueue(stream: Ref, queue: *mut c_void);
        fn FSEventStreamSetExclusionPaths(stream: Ref, paths: Ref) -> u8;
        fn FSEventStreamStart(stream: Ref) -> u8;
        fn FSEventStreamStop(stream: Ref);
        fn FSEventStreamInvalidate(stream: Ref);
        fn FSEventStreamRelease(stream: Ref);
        fn FSEventsGetCurrentEventId() -> u64;
        fn FSEventsCopyUUIDForDevice(device: i32) -> Ref;
    }
    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        static kCFTypeArrayCallBacks: c_void;
        fn CFStringCreateWithBytes(allocator: Ref, bytes: *const u8, length: isize, encoding: u32, external: u8) -> Ref;
        fn CFArrayCreate(allocator: Ref, values: *const Ref, count: isize, callbacks: *const c_void) -> Ref;
        fn CFUUIDCreateString(allocator: Ref, uuid: Ref) -> Ref;
        fn CFStringGetCString(string: Ref, buffer: *mut c_char, size: isize, encoding: u32) -> u8;
        fn CFRelease(value: Ref);
    }
    extern "C" {
        fn dispatch_queue_create(label: *const c_char, attributes: *const c_void) -> *mut c_void;
        fn dispatch_queue_attr_make_with_qos_class(attributes: *const c_void, qos: u32, relative: i32) -> *const c_void;
        fn dispatch_async_f(queue: *mut c_void, context: *mut c_void, work: extern "C" fn(*mut c_void));
    }

    const UTF8: u32 = 0x0800_0100;
    const SINCE_NOW: u64 = u64::MAX;
    const F_FILE_EVENTS: u32 = 0x10;
    const F_NO_DEFER: u32 = 0x2;
    const F_WATCH_ROOT: u32 = 0x4;
    /// 🚀 The callback queue's QoS: without one libdispatch runs it at utility priority (20), below every build process (31), so under fleet load fseventsd's queue for this client overflows during clone bursts (`UserDropped`).
    const QOS_CLASS_USER_INITIATED: u32 = 0x19;
    /// 🔙 Delivered callbacks a replay reaches back behind the last one.
    const TRAIL: usize = 8;
    /// 🔁 Replays in a row without one clean delivery before the watcher declares the loss.
    const RESYNCS_BEFORE_LOSS: u32 = 8;

    /// 🗄 Everything the streams of one watch share. Callbacks and restarts all run on one serial dispatch queue; the generation retires the callbacks of a replaced stream.
    /// `pending` holds the rows of a stream that replays history: fseventsd delivers a replay out of event-id order, so its rows leave as ONE batch when `HistoryDone` arrives (a fence row never precedes an earlier change), and `replayed` is that batch's cursor.
    struct State {
        root: String,
        filter: Filter,
        paths: Ref,
        exclusions: Ref,
        latency: f64,
        queue: *mut c_void,
        start: AtomicU64,
        generation: AtomicU64,
        resyncs: AtomicU32,
        stream: Mutex<usize>,
        trail: Mutex<VecDeque<u64>>,
        sequence: Mutex<u64>,
        pending: Mutex<Option<BTreeMap<String, Change>>>,
        replayed: AtomicU64,
        uuid: String,
    }

    struct Tag {
        state: *const State,
        generation: u64,
    }

    struct Restart {
        state: *const State,
        old: usize,
        since: u64,
        generation: u64,
    }

    fn cf_string(text: &str) -> Ref {
        unsafe { CFStringCreateWithBytes(std::ptr::null(), text.as_ptr(), text.len() as isize, UTF8, 0) }
    }

    fn cf_array(values: &[Ref]) -> Ref {
        unsafe { CFArrayCreate(std::ptr::null(), values.as_ptr(), values.len() as isize, &kCFTypeArrayCallBacks) }
    }

    fn device_uuid(root: &str) -> String {
        use std::os::unix::fs::MetadataExt;
        let Ok(meta) = std::fs::metadata(root) else { return String::new() };
        unsafe {
            let uuid = FSEventsCopyUUIDForDevice(meta.dev() as i32);
            if uuid.is_null() {
                return String::new();
            }
            let text = CFUUIDCreateString(std::ptr::null(), uuid);
            let mut buffer = [0 as c_char; 64];
            let ok = !text.is_null() && CFStringGetCString(text, buffer.as_mut_ptr(), 64, UTF8) != 0;
            let value = if ok { CStr::from_ptr(buffer.as_ptr()).to_string_lossy().to_string() } else { String::new() };
            if !text.is_null() {
                CFRelease(text);
            }
            CFRelease(uuid);
            value
        }
    }

    extern "C" fn release_tag(info: *const c_void) {
        drop(unsafe { Box::from_raw(info as *mut Tag) });
    }

    /// 🌊 Opens and starts one stream after `since` on the shared queue; null when FSEvents refuses.
    fn open(state: &State, since: u64, generation: u64) -> Ref {
        *state.pending.lock().unwrap() = if since == SINCE_NOW { None } else { Some(BTreeMap::new()) };
        state.replayed.store(0, Ordering::SeqCst);
        let tag = Box::into_raw(Box::new(Tag { state, generation }));
        let mut context = Context { version: 0, info: tag as *mut c_void, retain: std::ptr::null(), release: Some(release_tag), describe: std::ptr::null() };
        unsafe {
            let stream = FSEventStreamCreate(std::ptr::null(), callback, &mut context, state.paths, since, state.latency, F_FILE_EVENTS | F_NO_DEFER | F_WATCH_ROOT);
            if stream.is_null() {
                drop(Box::from_raw(tag));
                return stream;
            }
            if !state.exclusions.is_null() {
                FSEventStreamSetExclusionPaths(stream, state.exclusions);
            }
            FSEventStreamSetDispatchQueue(stream, state.queue);
            if FSEventStreamStart(stream) == 0 {
                FSEventStreamInvalidate(stream);
                FSEventStreamRelease(stream);
                return std::ptr::null();
            }
            stream
        }
    }

    /// 🔄 Replaces the stream by one that replays the volume history after the last safely delivered event id (a loss instead when the volume keeps no persistent history or it was reset: `fsevent::replayable`). Runs inside a callback: the old stream is retired at once (its later callbacks are ignored) and torn down on the queue after the callback returns. After `RESYNCS_BEFORE_LOSS` replays in a row without one clean delivery the watcher declares the loss and continues from now.
    fn resync(state: &State) {
        let attempts = state.resyncs.fetch_add(1, Ordering::SeqCst) + 1;
        let since = if attempts > RESYNCS_BEFORE_LOSS || !fsevent::replayable(&state.uuid, &device_uuid(&state.root)) {
            state.resyncs.store(0, Ordering::SeqCst);
            lost("user-dropped", "");
            let now = unsafe { FSEventsGetCurrentEventId() };
            state.trail.lock().unwrap().clear();
            state.start.store(now, Ordering::SeqCst);
            now
        } else {
            fsevent::resume(&state.trail.lock().unwrap(), TRAIL, state.start.load(Ordering::SeqCst))
        };
        let generation = state.generation.fetch_add(1, Ordering::SeqCst) + 1;
        let old = std::mem::replace(&mut *state.stream.lock().unwrap(), 0);
        let job = Box::into_raw(Box::new(Restart { state, old, since, generation }));
        unsafe { dispatch_async_f(state.queue, job as *mut c_void, restart) };
    }

    extern "C" fn restart(context: *mut c_void) {
        let job = unsafe { Box::from_raw(context as *mut Restart) };
        let state = unsafe { &*job.state };
        if job.old != 0 {
            unsafe {
                FSEventStreamStop(job.old as Ref);
                FSEventStreamInvalidate(job.old as Ref);
                FSEventStreamRelease(job.old as Ref);
            }
        }
        if state.generation.load(Ordering::SeqCst) != job.generation {
            return;
        }
        let stream = open(state, job.since, job.generation);
        if stream.is_null() {
            lost("user-dropped", "");
            let now = unsafe { FSEventsGetCurrentEventId() };
            state.start.store(now, Ordering::SeqCst);
            state.trail.lock().unwrap().clear();
            let fresh = open(state, now, job.generation);
            if fresh.is_null() {
                fail("backend", "FSEventStreamCreate failed after a replay");
            }
            *state.stream.lock().unwrap() = fresh as usize;
            return;
        }
        *state.stream.lock().unwrap() = stream as usize;
    }

    extern "C" fn callback(_stream: Ref, info: *mut c_void, count: usize, paths: *mut c_void, flags: *const u32, ids: *const u64) {
        let tag = unsafe { &*(info as *const Tag) };
        let state = unsafe { &*tag.state };
        if state.generation.load(Ordering::SeqCst) != tag.generation {
            return;
        }
        #[cfg(semio_watch_stall)]
        if tag.generation == 0 {
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        let paths = paths as *const *const c_char;
        let mut changes: BTreeMap<String, Change> = BTreeMap::new();
        let mut cursor = 0u64;
        let mut done = None;
        for index in 0..count {
            let (absolute, flag, id) = unsafe { (CStr::from_ptr(*paths.add(index)).to_string_lossy().to_string(), *flags.add(index), *ids.add(index)) };
            cursor = cursor.max(id);
            let rel = match absolute.strip_prefix(state.root.as_str()) {
                Some(rest) => rest.trim_start_matches('/').to_string(),
                None => String::new(),
            };
            match fsevent::action(flag, state.filter.excluded(&rel), rel.is_empty()) {
                fsevent::Action::HistoryDone => done = Some(id),
                fsevent::Action::Resync => {
                    resync(state);
                    return;
                }
                fsevent::Action::Lost(reason) => lost(reason, &rel),
                fsevent::Action::Subtree => merge(&mut changes, 'n', true, rel),
                fsevent::Action::Row(kind) => merge(&mut changes, kind, flag & fsevent::IS_DIR != 0, rel),
                fsevent::Action::Skip => {}
            }
        }
        let mut pending = state.pending.lock().unwrap();
        if let Some(buffer) = pending.as_mut() {
            for (path, change) in std::mem::take(&mut changes) {
                merge(buffer, change.kind, change.dir, path);
            }
            cursor = state.replayed.fetch_max(cursor, Ordering::SeqCst).max(cursor);
            if done.is_none() {
                return;
            }
            changes = pending.take().unwrap_or_default();
        }
        drop(pending);
        let mut sequence = state.sequence.lock().unwrap();
        emit_batch(&mut sequence, &cursor.to_string(), &mut changes);
        if let Some(id) = done {
            emit(&format!("{{\"v\":1,\"t\":\"history-done\",\"cursor\":\"{}\"}}", id));
        }
        let mut trail = state.trail.lock().unwrap();
        if cursor > 0 {
            if trail.len() >= TRAIL {
                trail.pop_front();
            }
            trail.push_back(cursor);
        }
        state.resyncs.store(0, Ordering::SeqCst);
    }

    pub fn run(config: &Config) {
        let root = canonical_root(&config.root);
        let uuid = device_uuid(&root);
        let replay = matches!((&config.since, &config.uuid), (Some(_), Some(expected)) if *expected == uuid && !uuid.is_empty());
        let current = unsafe { FSEventsGetCurrentEventId() };
        let since = if replay { config.since.as_ref().and_then(|s| s.parse::<u64>().ok()).unwrap_or(SINCE_NOW) } else { SINCE_NOW };
        let rooted: Vec<Ref> = config.filter.rooted.iter().take(8).map(|rel| cf_string(&format!("{root}/{rel}"))).collect();
        let label = CString::new("semio.repo.watch").unwrap();
        let queue = unsafe { dispatch_queue_create(label.as_ptr(), dispatch_queue_attr_make_with_qos_class(std::ptr::null(), QOS_CLASS_USER_INITIATED, 0)) };
        let state: &'static State = Box::leak(Box::new(State {
            root: root.clone(),
            filter: config.filter.clone(),
            paths: cf_array(&[cf_string(&root)]),
            exclusions: if rooted.is_empty() { std::ptr::null() } else { cf_array(&rooted) },
            latency: config.latency_ms as f64 / 1000.0,
            queue,
            start: AtomicU64::new(if since == SINCE_NOW { current } else { since }),
            generation: AtomicU64::new(0),
            resyncs: AtomicU32::new(0),
            stream: Mutex::new(0),
            trail: Mutex::new(VecDeque::with_capacity(TRAIL)),
            sequence: Mutex::new(0),
            pending: Mutex::new(None),
            replayed: AtomicU64::new(0),
            uuid: uuid.clone(),
        }));
        {
            let mut slot = state.stream.lock().unwrap();
            let stream = open(state, since, 0);
            if stream.is_null() {
                fail("backend", "FSEventStreamCreate failed");
            }
            *slot = stream as usize;
        }
        emit(&format!("{{\"v\":1,\"t\":\"ready\",\"backend\":\"fsevents\",\"root\":{},\"cursor\":\"{}\",\"uuid\":{},\"replay\":{},\"latencyMs\":{},\"native\":true}}", quote(&root), current, quote(&uuid), replay, config.latency_ms));
        stdin_guard();
        loop {
            std::thread::park();
        }
    }
}

// ───────────────────────────── Linux: inotify ─────────────────────────────

#[cfg(any(target_os = "linux", semio_check_linux))]
mod backend {
    use super::*;
    use std::collections::HashMap;
    use std::ffi::{c_char, c_int, c_void, CString};

    #[repr(C)]
    struct PollFd {
        fd: c_int,
        events: i16,
        revents: i16,
    }
    extern "C" {
        fn inotify_init1(flags: c_int) -> c_int;
        fn inotify_add_watch(fd: c_int, path: *const c_char, mask: u32) -> c_int;
        fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
        fn read(fd: c_int, buffer: *mut c_void, count: usize) -> isize;
        fn poll(fds: *mut PollFd, count: u64, timeout: c_int) -> c_int;
    }

    const MASK: u32 = 0x2 | 0x4 | 0x8 | 0x40 | 0x80 | 0x100 | 0x200 | 0x400 | 0x800 | 0x0100_0000 | 0x0200_0000 | 0x0400_0000;
    const IN_CREATE: u32 = 0x100;
    const IN_DELETE: u32 = 0x200;
    const IN_MOVED_FROM: u32 = 0x40;
    const IN_MOVED_TO: u32 = 0x80;
    const IN_DELETE_SELF: u32 = 0x400;
    const IN_MOVE_SELF: u32 = 0x800;
    const IN_Q_OVERFLOW: u32 = 0x4000;
    const IN_IGNORED: u32 = 0x8000;
    const IN_ISDIR: u32 = 0x4000_0000;
    const IN_NONBLOCK: c_int = 0o4000;
    /// 📦 Rows per batch during a burst that leaves no quiet gap of the latency.
    const BATCH_ROWS: usize = 8192;
    const IN_CLOEXEC: c_int = 0o2000000;
    const POLLIN: i16 = 1;

    struct Watches {
        fd: c_int,
        by_wd: HashMap<c_int, String>,
        filter: Filter,
        root: String,
        refused: bool,
    }

    impl Watches {
        fn add(&mut self, rel: &str) -> bool {
            let absolute = if rel.is_empty() { self.root.clone() } else { format!("{}/{}", self.root, rel) };
            let Ok(path) = CString::new(absolute) else { return false };
            let wd = unsafe { inotify_add_watch(self.fd, path.as_ptr(), MASK) };
            if wd < 0 {
                let error = std::io::Error::last_os_error();
                if error.raw_os_error() == Some(28) {
                    if !self.refused {
                        self.refused = true;
                    }
                    lost("watch-limit", rel);
                }
                return false;
            }
            self.by_wd.insert(wd, rel.to_string());
            true
        }

        fn tree(&mut self, rel: &str) {
            if self.filter.excluded(rel) || !self.add(rel) {
                return;
            }
            let absolute = if rel.is_empty() { self.root.clone() } else { format!("{}/{}", self.root, rel) };
            let Ok(entries) = std::fs::read_dir(&absolute) else { return };
            let mut children = Vec::new();
            for entry in entries.flatten() {
                if entry.file_type().map(|k| k.is_dir()).unwrap_or(false) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    children.push(if rel.is_empty() { name } else { format!("{rel}/{name}") });
                }
            }
            for child in children {
                self.tree(&child);
            }
        }

        fn rename_prefix(&mut self, from: &str, to: &str) {
            let from_slash = format!("{from}/");
            for path in self.by_wd.values_mut() {
                if path == from {
                    *path = to.to_string();
                } else if path.starts_with(&from_slash) {
                    *path = format!("{to}/{}", &path[from_slash.len()..]);
                }
            }
        }

        fn drop_prefix(&mut self, from: &str) {
            let from_slash = format!("{from}/");
            let gone: Vec<c_int> = self.by_wd.iter().filter(|(_, p)| p.as_str() == from || p.starts_with(&from_slash)).map(|(wd, _)| *wd).collect();
            for wd in gone {
                unsafe { inotify_rm_watch(self.fd, wd) };
                self.by_wd.remove(&wd);
            }
        }
    }

    pub fn run(config: &Config) {
        let root = canonical_root(&config.root);
        let fd = unsafe { inotify_init1(IN_NONBLOCK | IN_CLOEXEC) };
        if fd < 0 {
            fail("backend", "inotify_init1 failed");
        }
        let mut watches = Watches { fd, by_wd: HashMap::new(), filter: config.filter.clone(), root: root.clone(), refused: false };
        watches.tree("");
        emit(&format!("{{\"v\":1,\"t\":\"ready\",\"backend\":\"inotify\",\"root\":{},\"cursor\":\"\",\"uuid\":\"\",\"replay\":false,\"latencyMs\":{},\"native\":true,\"watches\":{}}}", quote(&root), config.latency_ms, watches.by_wd.len()));
        stdin_guard();
        let mut buffer = vec![0u8; 1 << 16];
        let mut sequence = 0u64;
        let mut changes: BTreeMap<String, Change> = BTreeMap::new();
        let mut moves: HashMap<u32, (String, bool)> = HashMap::new();
        let latency = std::time::Duration::from_millis(config.latency_ms.max(1));
        let mut pending: Option<std::time::Instant> = None;
        loop {
            if let Some(since) = pending {
                if due(changes.len(), since.elapsed(), latency, BATCH_ROWS) {
                    emit_batch(&mut sequence, "", &mut changes);
                    pending = None;
                }
            }
            let mut poller = PollFd { fd, events: POLLIN, revents: 0 };
            let timeout = if changes.is_empty() { -1 } else { config.latency_ms.max(1) as c_int };
            let ready = unsafe { poll(&mut poller, 1, timeout) };
            if ready == 0 {
                for (_, (path, _)) in moves.drain() {
                    watches.drop_prefix(&path);
                }
                emit_batch(&mut sequence, "", &mut changes);
                pending = None;
                continue;
            }
            let count = unsafe { read(fd, buffer.as_mut_ptr() as *mut c_void, buffer.len()) };
            if count <= 0 {
                continue;
            }
            pending.get_or_insert_with(std::time::Instant::now);
            let mut offset = 0usize;
            while offset + 16 <= count as usize {
                let wd = i32::from_ne_bytes(buffer[offset..offset + 4].try_into().unwrap());
                let mask = u32::from_ne_bytes(buffer[offset + 4..offset + 8].try_into().unwrap());
                let cookie = u32::from_ne_bytes(buffer[offset + 8..offset + 12].try_into().unwrap());
                let length = u32::from_ne_bytes(buffer[offset + 12..offset + 16].try_into().unwrap()) as usize;
                let name_bytes = &buffer[offset + 16..offset + 16 + length];
                let name = String::from_utf8_lossy(&name_bytes[..name_bytes.iter().position(|b| *b == 0).unwrap_or(length)]).to_string();
                offset += 16 + length;
                if mask & IN_Q_OVERFLOW != 0 {
                    lost("overflow", "");
                    continue;
                }
                let Some(parent) = watches.by_wd.get(&wd).cloned() else { continue };
                if mask & IN_IGNORED != 0 {
                    watches.by_wd.remove(&wd);
                    continue;
                }
                let rel = if name.is_empty() { parent.clone() } else if parent.is_empty() { name.clone() } else { format!("{parent}/{name}") };
                if watches.filter.excluded(&rel) {
                    continue;
                }
                let dir = mask & IN_ISDIR != 0;
                if mask & (IN_DELETE_SELF | IN_MOVE_SELF) != 0 {
                    merge(&mut changes, if mask & IN_DELETE_SELF != 0 { 'r' } else { 'n' }, true, rel);
                    continue;
                }
                if mask & IN_CREATE != 0 {
                    merge(&mut changes, 'a', dir, rel.clone());
                    if dir {
                        watches.tree(&rel);
                    }
                } else if mask & IN_DELETE != 0 {
                    merge(&mut changes, 'r', dir, rel.clone());
                    if dir {
                        watches.drop_prefix(&rel);
                    }
                } else if mask & IN_MOVED_FROM != 0 {
                    merge(&mut changes, 'n', dir, rel.clone());
                    moves.insert(cookie, (rel, dir));
                } else if mask & IN_MOVED_TO != 0 {
                    merge(&mut changes, 'n', dir, rel.clone());
                    if let Some((old, was_dir)) = moves.remove(&cookie) {
                        if was_dir {
                            watches.rename_prefix(&old, &rel);
                        }
                    } else if dir {
                        watches.tree(&rel);
                    }
                } else {
                    merge(&mut changes, 'm', dir, rel);
                }
            }
        }
    }
}

// ───────────────────────────── Windows: ReadDirectoryChangesW ─────────────────────────────

#[cfg(any(target_os = "windows", semio_check_windows))]
mod backend {
    use super::*;
    use std::ffi::c_void;

    extern "system" {
        fn CreateFileW(name: *const u16, access: u32, share: u32, security: *mut c_void, disposition: u32, flags: u32, template: isize) -> isize;
        fn ReadDirectoryChangesW(handle: isize, buffer: *mut c_void, length: u32, subtree: i32, filter: u32, returned: *mut u32, overlapped: *mut c_void, completion: *mut c_void) -> i32;
        fn GetLastError() -> u32;
    }

    const FILE_LIST_DIRECTORY: u32 = 0x1;
    const SHARE_ALL: u32 = 0x7;
    const OPEN_EXISTING: u32 = 3;
    const BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const NOTIFY_ALL: u32 = 0x1 | 0x2 | 0x4 | 0x8 | 0x10 | 0x40;
    const ERROR_NOTIFY_ENUM_DIR: u32 = 1022;
    const ERROR_INVALID_PARAMETER: u32 = 87;
    /// 🪣 1 MiB of notification buffer on local volumes (a build burst in a directory the kernel cannot exclude overflowed 64 KiB); 64 KiB, the documented maximum for network shares, after `ERROR_INVALID_PARAMETER`.
    const LOCAL_BUFFER_WORDS: usize = 1 << 18;
    const REMOTE_BUFFER_WORDS: usize = 1 << 14;
    const INVALID_HANDLE: isize = -1;

    pub fn run(config: &Config) {
        let root = canonical_root(&config.root);
        let wide: Vec<u16> = to_verbatim(&root).encode_utf16().chain(std::iter::once(0)).collect();
        let handle = unsafe { CreateFileW(wide.as_ptr(), FILE_LIST_DIRECTORY, SHARE_ALL, std::ptr::null_mut(), OPEN_EXISTING, BACKUP_SEMANTICS, 0) };
        if handle == INVALID_HANDLE {
            fail("backend", "CreateFileW failed for the watch root");
        }
        emit(&format!("{{\"v\":1,\"t\":\"ready\",\"backend\":\"rdcw\",\"root\":{},\"cursor\":\"\",\"uuid\":\"\",\"replay\":false,\"latencyMs\":0,\"native\":true}}", quote(&root)));
        stdin_guard();
        let mut buffer = vec![0u32; LOCAL_BUFFER_WORDS];
        let mut sequence = 0u64;
        let mut changes: BTreeMap<String, Change> = BTreeMap::new();
        loop {
            let mut returned = 0u32;
            let ok = unsafe { ReadDirectoryChangesW(handle, buffer.as_mut_ptr() as *mut c_void, (buffer.len() * 4) as u32, 1, NOTIFY_ALL, &mut returned, std::ptr::null_mut(), std::ptr::null_mut()) };
            if ok == 0 {
                let code = unsafe { GetLastError() };
                if code == ERROR_NOTIFY_ENUM_DIR {
                    lost("overflow", "");
                    continue;
                }
                if code == ERROR_INVALID_PARAMETER && buffer.len() > REMOTE_BUFFER_WORDS {
                    buffer = vec![0u32; REMOTE_BUFFER_WORDS];
                    continue;
                }
                fail("backend", &format!("ReadDirectoryChangesW failed ({code})"));
            }
            if returned == 0 {
                lost("overflow", "");
                continue;
            }
            let bytes = unsafe { std::slice::from_raw_parts(buffer.as_ptr() as *const u8, returned as usize) };
            let mut offset = 0usize;
            loop {
                if offset + 12 > bytes.len() {
                    break;
                }
                let next = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
                let action = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap());
                let length = u32::from_le_bytes(bytes[offset + 8..offset + 12].try_into().unwrap()) as usize;
                let units: Vec<u16> = bytes[offset + 12..offset + 12 + length].chunks_exact(2).map(|pair| u16::from_le_bytes([pair[0], pair[1]])).collect();
                let rel = String::from_utf16_lossy(&units).replace('\\', "/");
                if !config.filter.excluded(&rel) {
                    let kind = match action {
                        1 => 'a',
                        2 => 'r',
                        4 | 5 => 'n',
                        _ => 'm',
                    };
                    merge(&mut changes, kind, false, rel);
                }
                if next == 0 {
                    break;
                }
                offset += next;
            }
            emit_batch(&mut sequence, "", &mut changes);
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows", semio_check_linux, semio_check_windows)))]
mod backend {
    use super::*;
    pub fn run(_config: &Config) {
        fail("backend", "no native watcher for this platform");
    }
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let (command, config) = parse(&arguments);
    match command.as_str() {
        "watch" => backend::run(&config),
        "scan" => scan(&config),
        "version" => emit(&format!("{{\"v\":1,\"t\":\"version\",\"name\":\"semio-repo-watch\",\"protocol\":1,\"os\":{}}}", quote(std::env::consts::OS))),
        _ => fail("argument", "expected one of: watch, scan, version"),
    }
    finish(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filter() -> Filter {
        Filter { rooted: vec![".git".into(), "a/b".into()], names: vec!["node_modules".into()] }
    }

    #[test]
    fn exclusion_has_path_boundaries() {
        let f = filter();
        assert!(f.excluded(".git") && f.excluded(".git/x") && !f.excluded(".github/x"));
        assert!(f.excluded("a/b/c") && !f.excluded("a/bc") && !f.excluded("a"));
        assert!(f.excluded("x/node_modules/y") && !f.excluded("x/node_modules2") && !f.excluded(""));
    }

    #[test]
    fn json_escaping_round_trips_through_quote() {
        assert_eq!(quote("a\"b\\c\n\t\u{1}é😀"), "\"a\\\"b\\\\c\\n\\t\\u0001é😀\"");
    }

    #[test]
    fn merge_keeps_created_then_removed_as_modified_and_remembers_directories() {
        let mut changes = BTreeMap::new();
        merge(&mut changes, 'a', false, "x".into());
        merge(&mut changes, 'r', false, "x".into());
        assert_eq!(changes["x"].kind, 'm');
        merge(&mut changes, 'm', false, "y".into());
        merge(&mut changes, 'm', true, "y".into());
        assert!(changes["y"].dir);
        merge(&mut changes, 'a', false, "z".into());
        merge(&mut changes, 'm', false, "z".into());
        assert_eq!(changes["z"].kind, 'a');
    }

    #[test]
    fn fsevent_kinds_ignore_only_the_source_of_a_clone() {
        let measured = [(0x0041_0000, None), (0x0042_0000, None), (0x0041_0100, Some('a')), (0x0041_1000, Some('m')), (0x0041_4000, Some('m')), (0x0041_0200, Some('r')), (0x0041_0800, Some('n')), (0x0001_0100, Some('a')), (0x0001_1000, Some('m')), (0x0001_0300, Some('m')), (0x0002_0000, Some('m'))];
        for (flag, kind) in measured {
            assert_eq!(fsevent::kind(flag), kind, "flag {flag:#x}");
        }
    }

    #[test]
    fn fsevent_actions_replay_user_drops_and_bound_subtree_scans() {
        use fsevent::Action::*;
        let rows = [
            (0x0000_0003, false, true, Resync),
            (0x0000_0003, true, false, Resync),
            (0x0000_0007, false, true, Lost("kernel-dropped")),
            (0x0000_0005, true, false, Skip),
            (0x0000_0001, false, false, Subtree),
            (0x0000_0001, false, true, Lost("must-scan")),
            (0x0000_0001, true, false, Skip),
            (0x0000_0010, false, true, HistoryDone),
            (0x0000_0020, false, true, Lost("root-changed")),
            (0x0000_0040, false, false, Lost("mount")),
            (0x0001_0100, false, false, Row('a')),
            (0x0001_0100, true, false, Skip),
            (0x0041_0000, false, false, Skip),
        ];
        for (flag, excluded, root, action) in rows {
            assert_eq!(fsevent::action(flag, excluded, root), action, "flag {flag:#x} excluded {excluded} root {root}");
        }
    }

    #[test]
    fn replays_resume_behind_the_trail_and_never_before_the_start() {
        let trail: std::collections::VecDeque<u64> = (10..14).collect();
        assert_eq!(fsevent::resume(&trail, 4, 5), 10);
        assert_eq!(fsevent::resume(&trail, 8, 5), 5);
        assert_eq!(fsevent::resume(&trail, 4, 12), 12);
        assert_eq!(fsevent::resume(&std::collections::VecDeque::new(), 4, 7), 7);
    }

    #[test]
    fn drops_replay_only_from_the_persistent_history_the_stream_started_on() {
        assert!(fsevent::replayable("A3E1", "A3E1"));
        assert!(!fsevent::replayable("", ""));
        assert!(!fsevent::replayable("A3E1", ""));
        assert!(!fsevent::replayable("A3E1", "77B0"));
    }

    #[test]
    fn bursts_leave_in_bounded_batches_and_quiet_rows_after_the_latency() {
        let ms = std::time::Duration::from_millis;
        assert!(due(8192, ms(0), ms(5), 8192));
        assert!(due(1, ms(5), ms(5), 8192));
        assert!(!due(8191, ms(4), ms(5), 8192));
    }

    #[test]
    fn a_stalled_consumer_turns_batches_into_one_overflow_loss() {
        assert_eq!(admit(0, false, 10, 100), (false, true, false));
        assert_eq!(admit(95, false, 10, 100), (false, false, true));
        assert_eq!(admit(60, true, 10, 100), (false, false, true));
        assert_eq!(admit(40, true, 10, 100), (true, true, false));
        assert_eq!(admit(40, false, 10, 100), (false, true, false));
    }

    #[test]
    fn windows_verbatim_paths_round_trip_for_drives_and_shares() {
        let rows = [
            (r"\\?\C:\Users\dev\repo", r"C:\Users\dev\repo"),
            (r"\\?\UNC\server\share\repo", r"\\server\share\repo"),
            (r"C:\Users\dev\repo", r"C:\Users\dev\repo"),
            (r"\\server\share", r"\\server\share"),
            ("/unix/path", "/unix/path"),
        ];
        for (verbatim, plain) in rows {
            assert_eq!(strip_verbatim(verbatim), plain, "strip {verbatim}");
        }
        assert_eq!(to_verbatim(r"C:\Users\dev"), r"\\?\C:\Users\dev");
        assert_eq!(to_verbatim("C:/Users/dev"), r"\\?\C:\Users\dev");
        assert_eq!(to_verbatim(r"\\server\share\repo"), r"\\?\UNC\server\share\repo");
        assert_eq!(to_verbatim(r"\\?\C:\Users"), r"\\?\C:\Users");
        for plain in [r"C:\a\b", r"\\h\s\x"] {
            assert_eq!(strip_verbatim(&to_verbatim(plain)), plain);
        }
    }

    #[cfg(not(unix))]
    #[test]
    fn without_a_handle_windows_reports_unknown_inode_and_mode() {
        let meta = std::fs::metadata(std::env::current_dir().unwrap()).unwrap();
        assert_eq!(identity(&meta), (0, 0));
    }

    #[test]
    fn arguments_are_parsed_strictly() {
        let args: Vec<String> = ["watch", "--root", "/r", "--exclude", "/.git/", "--exclude-name", "node_modules", "--latency-ms", "7", "--threads", "3"].iter().map(|s| s.to_string()).collect();
        let (command, config) = parse(&args);
        assert_eq!(command, "watch");
        assert_eq!((config.root.as_str(), config.latency_ms, config.threads), ("/r", 7, 3));
        assert_eq!(config.filter.rooted, vec![".git".to_string()]);
    }
}
