//! 🔬 jlprobe: a read-only FSEvents observer that classifies every event the watcher callback would see (rooted-excluded, name-excluded, clone-only, emitted, drop markers) and prints one JSON line per second.
//! Used to find why the change journal records `user-dropped` under fleet load. Writes nothing below the watched root.
use std::ffi::{c_char, c_void, CStr, CString};
use std::io::Write;
use std::sync::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

type Ref = *const c_void;
#[repr(C)]
struct Context { version: isize, info: *mut c_void, retain: *const c_void, release: *const c_void, describe: *const c_void }
type Callback = extern "C" fn(Ref, *mut c_void, usize, *mut c_void, *const u32, *const u64);
#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn FSEventStreamCreate(a: Ref, cb: Callback, ctx: *mut Context, paths: Ref, since: u64, latency: f64, flags: u32) -> Ref;
    fn FSEventStreamSetDispatchQueue(s: Ref, q: *mut c_void);
    fn FSEventStreamSetExclusionPaths(s: Ref, p: Ref) -> u8;
    fn FSEventStreamStart(s: Ref) -> u8;
}
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFTypeArrayCallBacks: c_void;
    fn CFStringCreateWithBytes(a: Ref, b: *const u8, l: isize, e: u32, x: u8) -> Ref;
    fn CFArrayCreate(a: Ref, v: *const Ref, n: isize, cb: *const c_void) -> Ref;
}
extern "C" {
    fn dispatch_queue_create(label: *const c_char, attr: *const c_void) -> *mut c_void;
    fn dispatch_queue_attr_make_with_qos_class(attr: *const c_void, qos: u32, rel: i32) -> *const c_void;
}

#[derive(Default, Clone)]
struct Counts { callbacks: u64, events: u64, rooted: u64, named: u64, cloned: u64, emitted: u64, user: u64, kernel: u64, scan: u64, maxcb_us: u64, maxgap_ms: u64 }

struct State { root: String, rooted: Vec<String>, names: Vec<String>, counts: Mutex<(Counts, Option<Instant>)>, out: Mutex<std::fs::File>, label: String, started: Instant }

fn now_ms() -> u128 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() }

extern "C" fn callback(_s: Ref, info: *mut c_void, n: usize, paths: *mut c_void, flags: *const u32, _ids: *const u64) {
    let t0 = Instant::now();
    let st = unsafe { &*(info as *const State) };
    let paths = paths as *const *const c_char;
    let mut c = Counts::default();
    let mut markers = Vec::new();
    for i in 0..n {
        let (p, f) = unsafe { (CStr::from_ptr(*paths.add(i)).to_string_lossy().to_string(), *flags.add(i)) };
        let rel = p.strip_prefix(st.root.as_str()).map(|r| r.trim_start_matches('/').to_string()).unwrap_or_default();
        c.events += 1;
        if f & 0x6 != 0 || f & 0x1 != 0 {
            if f & 0x2 != 0 { c.user += 1 } else if f & 0x4 != 0 { c.kernel += 1 } else { c.scan += 1 }
            markers.push(format!("[{},\"{}\"]", f, rel.replace('"', "'")));
            continue;
        }
        if st.rooted.iter().any(|r| rel == *r || rel.starts_with(&format!("{r}/"))) { c.rooted += 1; continue; }
        if rel.split('/').any(|part| st.names.iter().any(|x| x == part)) { c.named += 1; continue; }
        if f & 0x40_0000 != 0 && f & 0xFF00 == 0 { c.cloned += 1; continue; }
        c.emitted += 1;
    }
    let took = t0.elapsed().as_micros() as u64;
    let mut g = st.counts.lock().unwrap();
    let gap = g.1.map(|t| t.elapsed().as_millis() as u64).unwrap_or(0);
    g.1 = Some(Instant::now());
    let a = &mut g.0;
    a.callbacks += 1; a.events += c.events; a.rooted += c.rooted; a.named += c.named; a.cloned += c.cloned; a.emitted += c.emitted; a.user += c.user; a.kernel += c.kernel; a.scan += c.scan;
    a.maxcb_us = a.maxcb_us.max(took); a.maxgap_ms = a.maxgap_ms.max(gap);
    if !markers.is_empty() {
        let mut o = st.out.lock().unwrap();
        let _ = writeln!(o, "{{\"label\":\"{}\",\"at\":{},\"t\":{:.1},\"drop\":[{}],\"batch\":{},\"gapMs\":{}}}", st.label, now_ms(), st.started.elapsed().as_secs_f64(), markers.join(","), n, gap);
    }
}

fn cfs(t: &str) -> Ref { unsafe { CFStringCreateWithBytes(std::ptr::null(), t.as_ptr(), t.len() as isize, 0x0800_0100, 0) } }

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (mut root, mut rooted, mut names, mut latency, mut qos, mut label, mut seconds, mut out) = (String::new(), vec![], vec![], 5u64, 0u32, String::from("p"), 60u64, String::from("/dev/stdout"));
    let mut i = 0;
    while i < args.len() {
        let v = args.get(i + 1).cloned().unwrap_or_default();
        match args[i].as_str() {
            "--root" => root = std::fs::canonicalize(&v).unwrap().to_string_lossy().to_string(),
            "--exclude" => rooted.push(v.trim_matches('/').to_string()),
            "--exclude-name" => names.push(v),
            "--latency-ms" => latency = v.parse().unwrap(),
            "--qos" => qos = match v.as_str() { "ui" => 0x21, "in" => 0x19, "default" => 0x15, "utility" => 0x11, _ => 0 },
            "--label" => label = v,
            "--seconds" => seconds = v.parse().unwrap(),
            "--out" => out = v,
            _ => panic!("argument {}", args[i]),
        }
        i += 2;
    }
    let file = std::fs::OpenOptions::new().create(true).append(true).open(&out).unwrap();
    let st = Box::into_raw(Box::new(State { root: root.clone(), rooted: rooted.clone(), names, counts: Mutex::new((Counts::default(), None)), out: Mutex::new(file), label: label.clone(), started: Instant::now() }));
    let mut ctx = Context { version: 0, info: st as *mut c_void, retain: std::ptr::null(), release: std::ptr::null(), describe: std::ptr::null() };
    let rp = cfs(&root);
    let paths = unsafe { CFArrayCreate(std::ptr::null(), &rp, 1, &kCFTypeArrayCallBacks) };
    let s = unsafe { FSEventStreamCreate(std::ptr::null(), callback, &mut ctx, paths, u64::MAX, latency as f64 / 1000.0, 0x10 | 0x2 | 0x4) };
    let ex: Vec<Ref> = rooted.iter().take(8).map(|r| cfs(&format!("{root}/{r}"))).collect();
    let exok = if ex.is_empty() { 1 } else { unsafe { FSEventStreamSetExclusionPaths(s, CFArrayCreate(std::ptr::null(), ex.as_ptr(), ex.len() as isize, &kCFTypeArrayCallBacks)) } };
    let l = CString::new("jlprobe").unwrap();
    unsafe {
        let attr = if qos == 0 { std::ptr::null() } else { dispatch_queue_attr_make_with_qos_class(std::ptr::null(), qos, 0) };
        FSEventStreamSetDispatchQueue(s, dispatch_queue_create(l.as_ptr(), attr));
        FSEventStreamStart(s);
    }
    let st = unsafe { &*st };
    { let mut o = st.out.lock().unwrap(); let _ = writeln!(o, "{{\"label\":\"{}\",\"at\":{},\"start\":true,\"pid\":{},\"exclusionOk\":{},\"qos\":{},\"latencyMs\":{}}}", label, now_ms(), std::process::id(), exok, qos, latency); }
    for second in 1..=seconds {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let c = { let mut g = st.counts.lock().unwrap(); std::mem::take(&mut g.0) };
        if c.callbacks == 0 { continue; }
        let mut o = st.out.lock().unwrap();
        let _ = writeln!(o, "{{\"label\":\"{}\",\"at\":{},\"s\":{},\"cb\":{},\"ev\":{},\"rooted\":{},\"named\":{},\"cloned\":{},\"emitted\":{},\"user\":{},\"kernel\":{},\"scan\":{},\"maxCbUs\":{},\"maxGapMs\":{}}}", label, now_ms(), second, c.callbacks, c.events, c.rooted, c.named, c.cloned, c.emitted, c.user, c.kernel, c.scan, c.maxcb_us, c.maxgap_ms);
    }
}
