//! 🔭 semio-repo-watch: one std-only file, three native backends (FSEvents, inotify, ReadDirectoryChangesW).
//! Protocol: newline-delimited JSON frames on stdout (schema: resident/schema `WatcherFrame`), commands on stdin.
//! EOF on stdin ends the process, so a watcher can never outlive the service that spawned it.
//! https://developer.apple.com/documentation/coreservices/file_system_events
//! https://man7.org/linux/man-pages/man7/inotify.7.html
//! https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-readdirectorychangesw
#![allow(clippy::missing_safety_doc)]

use std::collections::{BTreeMap, VecDeque};
use std::io::{BufRead, Write};
use std::sync::{Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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

enum Frame {
    Line(String),
    Batch { at: u64, cursor: String, changes: BTreeMap<String, Change> },
}

#[derive(Default)]
struct Queue {
    frames: VecDeque<Frame>,
    writing: bool,
    sequence: u64,
}

/// 📮 The ordered frames waiting for the consumer of stdout. Producers (the native event callbacks) only append under a short lock and never wait for the pipe: while the consumer is slow, adjacent batches merge into one, so a stalled reader cannot hold an event callback and make the operating system drop events.
/// https://developer.apple.com/documentation/coreservices/1455361-fseventstreameventflags
struct Outbox {
    queue: Mutex<Queue>,
    wake: Condvar,
    idle: Condvar,
}

impl Outbox {
    fn new() -> Self {
        Outbox { queue: Mutex::new(Queue::default()), wake: Condvar::new(), idle: Condvar::new() }
    }

    fn line(&self, text: String) {
        let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        queue.frames.push_back(Frame::Line(text));
        self.wake.notify_one();
    }

    fn batch(&self, cursor: &str, changes: &mut BTreeMap<String, Change>) {
        if changes.is_empty() {
            return;
        }
        let incoming = std::mem::take(changes);
        let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        match queue.frames.back_mut() {
            Some(Frame::Batch { cursor: held, changes: pending, .. }) => {
                *held = cursor.to_string();
                for (_, change) in incoming {
                    follow(pending, change.kind, change.dir, change.path);
                }
            }
            _ => queue.frames.push_back(Frame::Batch { at: now_ms(), cursor: cursor.to_string(), changes: incoming }),
        }
        self.wake.notify_one();
    }

    fn serve<W: Write>(&self, sink: &mut W) -> std::io::Result<()> {
        loop {
            let (frame, number) = {
                let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
                let frame = loop {
                    if let Some(frame) = queue.frames.pop_front() {
                        break frame;
                    }
                    queue = self.wake.wait(queue).unwrap_or_else(|e| e.into_inner());
                };
                queue.writing = true;
                if matches!(frame, Frame::Batch { .. }) {
                    queue.sequence += 1;
                }
                (frame, queue.sequence)
            };
            let text = match frame {
                Frame::Line(text) => text,
                Frame::Batch { at, cursor, changes } => {
                    let mut line = format!("{{\"v\":1,\"t\":\"batch\",\"n\":{},\"at\":{},\"cursor\":{},\"c\":[", number, at, quote(&cursor));
                    let mut first = true;
                    for change in changes.values() {
                        if !first {
                            line.push(',');
                        }
                        first = false;
                        line.push_str(&format!("[\"{}\",{},{}]", change.kind, if change.dir { 1 } else { 0 }, quote(&change.path)));
                    }
                    line.push_str("]}");
                    line
                }
            };
            let written = sink.write_all(text.as_bytes()).and_then(|_| sink.write_all(b"\n")).and_then(|_| sink.flush());
            let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
            queue.writing = false;
            self.idle.notify_all();
            drop(queue);
            written?;
        }
    }

    fn drain(&self, limit: Option<Duration>) -> bool {
        let deadline = limit.map(|limit| Instant::now() + limit);
        let mut queue = self.queue.lock().unwrap_or_else(|e| e.into_inner());
        while !queue.frames.is_empty() || queue.writing {
            queue = match deadline {
                None => self.idle.wait(queue).unwrap_or_else(|e| e.into_inner()),
                Some(deadline) => {
                    let Some(left) = deadline.checked_duration_since(Instant::now()) else { return false };
                    self.idle.wait_timeout(queue, left).unwrap_or_else(|e| e.into_inner()).0
                }
            };
        }
        true
    }
}

fn outbox() -> &'static Outbox {
    static BOX: OnceLock<&'static Outbox> = OnceLock::new();
    BOX.get_or_init(|| {
        let outbox: &'static Outbox = Box::leak(Box::new(Outbox::new()));
        std::thread::spawn(move || {
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            let _ = outbox.serve(&mut handle);
            std::process::exit(0);
        });
        outbox
    })
}

fn emit(line: &str) {
    outbox().line(line.to_string());
}

/// 🏁 Hands the frames still queued to the consumer (waiting at most two seconds for a consumer that has stopped reading), then ends the process.
fn finish(code: i32) -> ! {
    outbox().drain(Some(Duration::from_secs(2)));
    std::process::exit(code);
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

/// 🪟 The verbatim form of a plain Windows path, the inverse of `strip_verbatim`: a drive path takes `\\?\`, a share takes `\\?\UNC\`.
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

fn emit_batch(cursor: &str, changes: &mut BTreeMap<String, Change>) {
    outbox().batch(cursor, changes);
}

/// 🧵 Appends one change that happened strictly after the pending one of the same path: the pending kind and the new kind fold into the net kind, `m` (look at it again) whenever the order alone cannot tell whether the path exists.
fn follow(changes: &mut BTreeMap<String, Change>, kind: char, dir: bool, path: String) {
    match changes.get_mut(&path) {
        Some(old) => {
            old.dir |= dir;
            old.kind = match (old.kind, kind) {
                ('a', 'a') | ('a', 'm') => 'a',
                ('m', 'r') | ('r', 'r') => 'r',
                _ => 'm',
            };
        }
        None => {
            changes.insert(path.clone(), Change { kind, dir, path });
        }
    }
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
    use std::ffi::{c_char, c_void, CStr, CString};

    type Ref = *const c_void;
    #[repr(C)]
    struct Context {
        version: isize,
        info: *mut c_void,
        retain: *const c_void,
        release: *const c_void,
        describe: *const c_void,
    }
    type Callback = extern "C" fn(Ref, *mut c_void, usize, *mut c_void, *const u32, *const u64);

    #[link(name = "CoreServices", kind = "framework")]
    extern "C" {
        fn FSEventStreamCreate(allocator: Ref, callback: Callback, context: *mut Context, paths: Ref, since: u64, latency: f64, flags: u32) -> Ref;
        fn FSEventStreamSetDispatchQueue(stream: Ref, queue: *mut c_void);
        fn FSEventStreamSetExclusionPaths(stream: Ref, paths: Ref) -> u8;
        fn FSEventStreamStart(stream: Ref) -> u8;
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
        fn dispatch_queue_attr_make_with_qos_class(attribute: *const c_void, class: u32, relative: i32) -> *const c_void;
    }

    /// 🚀 `QOS_CLASS_USER_INTERACTIVE`: the event queue keeps its priority on a loaded host (a service started from a niced job inherits that nice value), because a callback that runs late makes the operating system drop events for good.
    /// https://developer.apple.com/documentation/dispatch/1453030-dispatch_queue_attr_make_with_qos
    const QOS_USER_INTERACTIVE: u32 = 0x21;
    const UTF8: u32 = 0x0800_0100;
    const SINCE_NOW: u64 = u64::MAX;
    const F_FILE_EVENTS: u32 = 0x10;
    const F_NO_DEFER: u32 = 0x2;
    const F_WATCH_ROOT: u32 = 0x4;
    const E_MUST_SCAN: u32 = 0x1;
    const E_USER_DROPPED: u32 = 0x2;
    const E_KERNEL_DROPPED: u32 = 0x4;
    const E_HISTORY_DONE: u32 = 0x10;
    const E_ROOT_CHANGED: u32 = 0x20;
    const E_MOUNT: u32 = 0x40;
    const E_UNMOUNT: u32 = 0x80;

    struct State {
        root: String,
        filter: Filter,
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

    extern "C" fn callback(_stream: Ref, info: *mut c_void, count: usize, paths: *mut c_void, flags: *const u32, ids: *const u64) {
        let state = unsafe { &*(info as *const State) };
        let paths = paths as *const *const c_char;
        let mut changes: BTreeMap<String, Change> = BTreeMap::new();
        let mut cursor = 0u64;
        for index in 0..count {
            let (absolute, flag, id) = unsafe { (CStr::from_ptr(*paths.add(index)).to_string_lossy().to_string(), *flags.add(index), *ids.add(index)) };
            cursor = cursor.max(id);
            let rel = match absolute.strip_prefix(state.root.as_str()) {
                Some(rest) => rest.trim_start_matches('/').to_string(),
                None => String::new(),
            };
            if flag & E_HISTORY_DONE != 0 {
                emit(&format!("{{\"v\":1,\"t\":\"history-done\",\"cursor\":\"{}\"}}", id));
                continue;
            }
            if flag & (E_MUST_SCAN | E_USER_DROPPED | E_KERNEL_DROPPED | E_ROOT_CHANGED | E_MOUNT | E_UNMOUNT) != 0 {
                let reason = if flag & E_ROOT_CHANGED != 0 { "root-changed" } else if flag & E_KERNEL_DROPPED != 0 { "kernel-dropped" } else if flag & E_USER_DROPPED != 0 { "user-dropped" } else if flag & (E_MOUNT | E_UNMOUNT) != 0 { "mount" } else { "must-scan" };
                if !state.filter.excluded(&rel) {
                    lost(reason, &rel);
                }
                continue;
            }
            if state.filter.excluded(&rel) {
                continue;
            }
            if let Some(kind) = fsevent::kind(flag) {
                merge(&mut changes, kind, flag & fsevent::IS_DIR != 0, rel);
            }
        }
        emit_batch(&cursor.to_string(), &mut changes);
    }

    pub fn run(config: &Config) {
        let root = canonical_root(&config.root);
        let uuid = device_uuid(&root);
        let replay = matches!((&config.since, &config.uuid), (Some(_), Some(expected)) if *expected == uuid && !uuid.is_empty());
        let since = if replay { config.since.as_ref().and_then(|s| s.parse::<u64>().ok()).unwrap_or(SINCE_NOW) } else { SINCE_NOW };
        let state = Box::into_raw(Box::new(State { root: root.clone(), filter: config.filter.clone() }));
        let mut context = Context { version: 0, info: state as *mut c_void, retain: std::ptr::null(), release: std::ptr::null(), describe: std::ptr::null() };
        let path = cf_string(&root);
        let paths = cf_array(&[path]);
        let stream = unsafe { FSEventStreamCreate(std::ptr::null(), callback, &mut context, paths, since, config.latency_ms as f64 / 1000.0, F_FILE_EVENTS | F_NO_DEFER | F_WATCH_ROOT) };
        if stream.is_null() {
            fail("backend", "FSEventStreamCreate failed");
        }
        let rooted: Vec<Ref> = config.filter.rooted.iter().take(8).map(|rel| cf_string(&format!("{root}/{rel}"))).collect();
        if !rooted.is_empty() {
            unsafe { FSEventStreamSetExclusionPaths(stream, cf_array(&rooted)) };
        }
        let label = CString::new("semio.repo.watch").unwrap();
        unsafe {
            let queue = dispatch_queue_create(label.as_ptr(), dispatch_queue_attr_make_with_qos_class(std::ptr::null(), QOS_USER_INTERACTIVE, 0));
            FSEventStreamSetDispatchQueue(stream, queue);
            if FSEventStreamStart(stream) == 0 {
                fail("backend", "FSEventStreamStart failed");
            }
        }
        emit(&format!("{{\"v\":1,\"t\":\"ready\",\"backend\":\"fsevents\",\"root\":{},\"cursor\":\"{}\",\"uuid\":{},\"replay\":{},\"latencyMs\":{},\"native\":true}}", quote(&root), unsafe { FSEventsGetCurrentEventId() }, quote(&uuid), replay, config.latency_ms));
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
        let mut changes: BTreeMap<String, Change> = BTreeMap::new();
        let mut moves: HashMap<u32, (String, bool)> = HashMap::new();
        loop {
            let mut poller = PollFd { fd, events: POLLIN, revents: 0 };
            let timeout = if changes.is_empty() { -1 } else { config.latency_ms.max(1) as c_int };
            let ready = unsafe { poll(&mut poller, 1, timeout) };
            if ready == 0 {
                for (_, (path, _)) in moves.drain() {
                    watches.drop_prefix(&path);
                }
                emit_batch("", &mut changes);
                continue;
            }
            let count = unsafe { read(fd, buffer.as_mut_ptr() as *mut c_void, buffer.len()) };
            if count <= 0 {
                continue;
            }
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
        let mut buffer = vec![0u32; 1 << 14];
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
            emit_batch("", &mut changes);
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

    struct Gate {
        open: std::sync::Arc<(Mutex<bool>, Condvar)>,
        bytes: std::sync::Arc<Mutex<Vec<u8>>>,
    }

    impl Write for Gate {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let (flag, cv) = &*self.open;
            let mut open = flag.lock().unwrap();
            while !*open {
                open = cv.wait(open).unwrap();
            }
            self.bytes.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn frames_of(bytes: &[u8]) -> Vec<(String, Vec<(char, String)>)> {
        let mut out = Vec::new();
        for line in String::from_utf8(bytes.to_vec()).unwrap().lines() {
            let kind = if line.contains("\"t\":\"batch\"") { "batch" } else { "line" };
            let mut rows = Vec::new();
            if kind == "batch" {
                let body = &line[line.find("\"c\":[").unwrap() + 5..line.len() - 1];
                let mut rest = body;
                while let Some(start) = rest.find("[\"") {
                    let item = &rest[start + 2..];
                    let kind = item.chars().next().unwrap();
                    let quote_at = item.find(",\"").unwrap();
                    let end = item[quote_at + 2..].find("\"]").unwrap() + quote_at + 2;
                    rows.push((kind, item[quote_at + 2..end].to_string()));
                    rest = &item[end..];
                }
            }
            out.push((kind.to_string(), rows));
        }
        out
    }

    #[test]
    fn a_stalled_consumer_never_blocks_the_event_callback() {
        let outbox: &'static Outbox = Box::leak(Box::new(Outbox::new()));
        let open = std::sync::Arc::new((Mutex::new(false), Condvar::new()));
        let bytes = std::sync::Arc::new(Mutex::new(Vec::new()));
        let mut gate = Gate { open: open.clone(), bytes: bytes.clone() };
        let server = std::thread::spawn(move || {
            let _ = outbox.serve(&mut gate);
        });
        let released = std::sync::Arc::new(Mutex::new(false));
        let watchdog = {
            let (open, released) = (open.clone(), released.clone());
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_secs(20));
                let (flag, cv) = &*open;
                let mut open = flag.lock().unwrap();
                if !*open {
                    *released.lock().unwrap() = true;
                }
                *open = true;
                cv.notify_all();
            })
        };
        let total = 4000usize;
        for index in 0..total {
            let mut changes = BTreeMap::new();
            merge(&mut changes, 'a', false, format!("d{}/f{}", index % 40, index));
            outbox.batch(&index.to_string(), &mut changes);
        }
        assert!(!*released.lock().unwrap(), "the producer waited for the stalled consumer");
        {
            let (flag, cv) = &*open;
            *flag.lock().unwrap() = true;
            cv.notify_all();
        }
        assert!(outbox.drain(Some(Duration::from_secs(20))));
        let frames = frames_of(&bytes.lock().unwrap());
        let mut seen = std::collections::BTreeSet::new();
        for (_, rows) in &frames {
            for (kind, path) in rows {
                assert_eq!(*kind, 'a');
                seen.insert(path.clone());
            }
        }
        assert_eq!(seen.len(), total, "every path survives the coalescing");
        assert!(frames.len() <= 3, "{} frames for a stalled consumer", frames.len());
        drop(watchdog);
        drop(server);
    }

    #[test]
    fn coalesced_batches_keep_the_final_existence_of_every_path() {
        let kinds = ['a', 'm', 'r'];
        let mut sequences: Vec<Vec<char>> = vec![vec![]];
        for _ in 0..5 {
            let mut next = Vec::new();
            for sequence in &sequences {
                for kind in kinds {
                    let mut longer = sequence.clone();
                    longer.push(kind);
                    next.push(longer);
                }
            }
            sequences.extend(next.clone());
            sequences.sort();
            sequences.dedup();
        }
        for sequence in sequences.iter().filter(|s| !s.is_empty()) {
            let outbox = Outbox::new();
            for (index, kind) in sequence.iter().enumerate() {
                let mut changes = BTreeMap::new();
                merge(&mut changes, *kind, false, "p".into());
                outbox.batch(&index.to_string(), &mut changes);
            }
            let drained = {
                let mut queue = outbox.queue.lock().unwrap();
                let mut collected = Vec::new();
                while let Some(Frame::Batch { changes, .. }) = queue.frames.pop_front() {
                    collected.push(changes["p"].kind);
                }
                collected
            };
            assert_eq!(drained.len(), 1, "adjacent batches merge into one frame: {sequence:?}");
            let last = *sequence.last().unwrap();
            let merged = drained[0];
            if last == 'r' {
                assert!(merged == 'r' || merged == 'm', "{sequence:?} -> {merged}");
            } else {
                assert!(merged == 'a' || merged == 'm', "{sequence:?} -> {merged}");
            }
            if sequence.iter().all(|kind| *kind != 'r') && sequence[0] == 'a' {
                assert_eq!(merged, 'a', "{sequence:?}");
            }
        }
    }

    #[test]
    fn a_line_is_an_ordering_barrier_between_batches() {
        let outbox = Outbox::new();
        let mut one = BTreeMap::new();
        merge(&mut one, 'a', false, "x".into());
        outbox.batch("1", &mut one);
        outbox.line("{\"t\":\"lost\"}".into());
        let mut two = BTreeMap::new();
        merge(&mut two, 'a', false, "y".into());
        outbox.batch("2", &mut two);
        let order: Vec<&str> = outbox.queue.lock().unwrap().frames.iter().map(|frame| match frame { Frame::Line(_) => "line", Frame::Batch { .. } => "batch" }).collect();
        assert_eq!(order, vec!["batch", "line", "batch"]);
    }

    #[test]
    fn sequence_numbers_follow_the_written_frames() {
        let outbox = Outbox::new();
        for name in ["x", "y"] {
            let mut changes = BTreeMap::new();
            merge(&mut changes, 'a', false, name.into());
            outbox.batch("c", &mut changes);
            outbox.line("{\"t\":\"lost\"}".into());
        }
        let bytes = std::sync::Arc::new(Mutex::new(Vec::new()));
        let open = std::sync::Arc::new((Mutex::new(true), Condvar::new()));
        let mut gate = Gate { open, bytes: bytes.clone() };
        let outbox: &'static Outbox = Box::leak(Box::new(outbox));
        std::thread::spawn(move || {
            let _ = outbox.serve(&mut gate);
        });
        assert!(outbox.drain(Some(Duration::from_secs(10))));
        let text = String::from_utf8(bytes.lock().unwrap().clone()).unwrap();
        let numbers: Vec<&str> = text.lines().filter(|line| line.contains("\"batch\"")).map(|line| line.split("\"n\":").nth(1).unwrap().split(',').next().unwrap()).collect();
        assert_eq!(numbers, vec!["1", "2"]);
    }
}
