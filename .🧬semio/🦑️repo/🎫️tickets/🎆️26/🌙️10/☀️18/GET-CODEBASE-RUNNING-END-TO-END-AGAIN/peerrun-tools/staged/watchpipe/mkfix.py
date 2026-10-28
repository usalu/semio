import sys,re
src=open(sys.argv[1],encoding='utf-8').read()
def sub(old,new,count=1):
    global src
    assert src.count(old)==count,(old,src.count(old))
    src=src.replace(old,new)

sub('''use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static OUT: Mutex<()> = Mutex::new(());
''','''use std::collections::{BTreeMap, VecDeque};
use std::io::{BufRead, Write};
use std::sync::{Condvar, Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
''')

sub('''fn emit(line: &str) {
    let _guard = OUT.lock().unwrap_or_else(|e| e.into_inner());
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    if handle.write_all(line.as_bytes()).is_err() || handle.write_all(b"\\n").is_err() || handle.flush().is_err() {
        std::process::exit(0);
    }
}

fn fail(code: &str, message: &str) -> ! {
    emit(&format!("{{\\"v\\":1,\\"t\\":\\"error\\",\\"code\\":{},\\"message\\":{}}}", quote(code), quote(message)));
    std::process::exit(2);
}
''','''enum Frame {
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
                    let mut line = format!("{{\\"v\\":1,\\"t\\":\\"batch\\",\\"n\\":{},\\"at\\":{},\\"cursor\\":{},\\"c\\":[", number, at, quote(&cursor));
                    let mut first = true;
                    for change in changes.values() {
                        if !first {
                            line.push(',');
                        }
                        first = false;
                        line.push_str(&format!("[\\"{}\\",{},{}]", change.kind, if change.dir { 1 } else { 0 }, quote(&change.path)));
                    }
                    line.push_str("]}");
                    line
                }
            };
            let written = sink.write_all(text.as_bytes()).and_then(|_| sink.write_all(b"\\n")).and_then(|_| sink.flush());
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
    emit(&format!("{{\\"v\\":1,\\"t\\":\\"error\\",\\"code\\":{},\\"message\\":{}}}", quote(code), quote(message)));
    finish(2);
}
''')

sub('''fn emit_batch(sequence: &mut u64, cursor: &str, changes: &mut BTreeMap<String, Change>) {
    if changes.is_empty() {
        return;
    }
    *sequence += 1;
    let mut line = format!("{{\\"v\\":1,\\"t\\":\\"batch\\",\\"n\\":{},\\"at\\":{},\\"cursor\\":{},\\"c\\":[", sequence, now_ms(), quote(cursor));
    let mut first = true;
    for change in changes.values() {
        if !first {
            line.push(',');
        }
        first = false;
        line.push_str(&format!("[\\"{}\\",{},{}]", change.kind, if change.dir { 1 } else { 0 }, quote(&change.path)));
    }
    line.push_str("]}");
    changes.clear();
    emit(&line);
}
''','''fn emit_batch(cursor: &str, changes: &mut BTreeMap<String, Change>) {
    outbox().batch(cursor, changes);
}
''')

sub('''fn merge(changes: &mut BTreeMap<String, Change>, kind: char, dir: bool, path: String) {''','''/// 🧵 Appends one change that happened strictly after the pending one of the same path: the pending kind and the new kind fold into the net kind, `m` (look at it again) whenever the order alone cannot tell whether the path exists.
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

fn merge(changes: &mut BTreeMap<String, Change>, kind: char, dir: bool, path: String) {''')

sub('''        emit("{\\"v\\":1,\\"t\\":\\"bye\\"}");
        std::process::exit(0);''','''        emit("{\\"v\\":1,\\"t\\":\\"bye\\"}");
        finish(0);''')

sub('''        let mut sequence = state.sequence.lock().unwrap();
        emit_batch(&mut sequence, &cursor.to_string(), &mut changes);''','''        emit_batch(&cursor.to_string(), &mut changes);''')
sub('''        sequence: Mutex<u64>,
''','')
sub(''', sequence: Mutex::new(0) }''',''' }''')
sub('''        let mut sequence = 0u64;
''','',2)
sub('''emit_batch(&mut sequence, "", &mut changes);''','''emit_batch("", &mut changes);''',2)

sub('''        _ => fail("argument", "expected one of: watch, scan, version"),
    }
}''','''        _ => fail("argument", "expected one of: watch, scan, version"),
    }
    finish(0);
}''')

sub('''        fn dispatch_queue_create(label: *const c_char, attributes: *const c_void) -> *mut c_void;
''','''        fn dispatch_queue_create(label: *const c_char, attributes: *const c_void) -> *mut c_void;
        fn dispatch_queue_attr_make_with_qos_class(attribute: *const c_void, class: u32, relative: i32) -> *const c_void;
''')
sub('''let queue = dispatch_queue_create(label.as_ptr(), std::ptr::null());''','''let queue = dispatch_queue_create(label.as_ptr(), dispatch_queue_attr_make_with_qos_class(std::ptr::null(), QOS_USER_INTERACTIVE, 0));''')
sub('''    const UTF8: u32 = 0x0800_0100;''','''    /// 🚀 `QOS_CLASS_USER_INTERACTIVE`: the event queue keeps its priority on a loaded host (a service started from a niced job inherits that nice value), because a callback that runs late makes the operating system drop events for good.
    /// https://developer.apple.com/documentation/dispatch/1453030-dispatch_queue_attr_make_with_qos
    const QOS_USER_INTERACTIVE: u32 = 0x21;
    const UTF8: u32 = 0x0800_0100;''')

tests='''
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
            let kind = if line.contains("\\"t\\":\\"batch\\"") { "batch" } else { "line" };
            let mut rows = Vec::new();
            if kind == "batch" {
                let body = &line[line.find("\\"c\\":[").unwrap() + 5..line.len() - 1];
                let mut rest = body;
                while let Some(start) = rest.find("[\\"") {
                    let item = &rest[start + 2..];
                    let kind = item.chars().next().unwrap();
                    let quote_at = item.find(",\\"").unwrap();
                    let end = item[quote_at + 2..].find("\\"]").unwrap() + quote_at + 2;
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
        outbox.line("{\\"t\\":\\"lost\\"}".into());
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
            outbox.line("{\\"t\\":\\"lost\\"}".into());
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
        let numbers: Vec<&str> = text.lines().filter(|line| line.contains("\\"batch\\"")).map(|line| line.split("\\"n\\":").nth(1).unwrap().split(',').next().unwrap()).collect();
        assert_eq!(numbers, vec!["1", "2"]);
    }
'''
idx=src.rindex('}')
src=src[:idx]+tests.lstrip('\n').join(['','']) if False else src[:idx].rstrip('\n')+'\n'+tests+'}\n'
open(sys.argv[2],'w',encoding='utf-8').write(src)
