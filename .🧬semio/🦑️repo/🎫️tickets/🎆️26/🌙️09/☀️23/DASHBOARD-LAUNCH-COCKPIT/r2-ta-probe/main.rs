//! Drives the Windows `NativeTerminal` on the real console and logs what it saw (round 2, slice T-A).

use std::io::Write;
use std::os::windows::io::AsRawHandle;
use std::time::{Duration, Instant};
use ui_tui::tui::ansi::AnsiPatch;
use ui_tui::tui::backend::{NativeTerminal, TerminalBackend};
use ui_tui::tui::event::Event;
use ui_tui::tui::geometry::Pos;
use ui_tui::tui::widget::{CursorShape, CursorSpec};

#[repr(C)]
#[derive(Clone, Copy)]
struct KeyRecord {
    key_down: i32,
    repeat: u16,
    vk: u16,
    scan: u16,
    ch: u16,
    state: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct InputRecord {
    event_type: u16,
    key: KeyRecord,
}

extern "system" {
    fn WriteConsoleInputW(console: *mut core::ffi::c_void, buffer: *const InputRecord, length: u32, written: *mut u32) -> i32;
    fn GetConsoleMode(console: *mut core::ffi::c_void, mode: *mut u32) -> i32;
}

fn input_handle() -> std::fs::File {
    std::fs::OpenOptions::new().read(true).write(true).open("CONIN$").expect("CONIN$")
}

fn console_mode() -> u32 {
    let file = input_handle();
    let mut mode = 0u32;
    unsafe { GetConsoleMode(file.as_raw_handle(), &mut mode) };
    mode
}

fn inject(text: &str) {
    let file = input_handle();
    let mut records = Vec::new();
    for unit in text.encode_utf16() {
        for down in [1, 0] {
            records.push(InputRecord { event_type: 1, key: KeyRecord { key_down: down, repeat: 1, vk: 0, scan: 0, ch: unit, state: 0 } });
        }
    }
    let mut written = 0u32;
    unsafe { WriteConsoleInputW(file.as_raw_handle(), records.as_ptr(), records.len() as u32, &mut written) };
}

fn collect(term: &mut NativeTerminal, want: usize, limit: Duration) -> (Vec<Event>, u128) {
    let started = Instant::now();
    let mut all = Vec::new();
    while all.len() < want && started.elapsed() < limit {
        all.extend(term.wait(Some(started + limit)).expect("wait"));
    }
    (all, started.elapsed().as_millis())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("panic-child") {
        let mut term = NativeTerminal::new().expect("console");
        term.enter().expect("enter");
        panic!("[DEBUG] deliberate panic after enter");
    }
    let mut log = std::fs::File::create(&args[1]).expect("log file");
    let before = console_mode();
    let mut term = NativeTerminal::new().expect("console");
    term.enter().expect("enter");
    let during = console_mode();
    let size = term.size().expect("size");
    writeln!(log, "console mode before={before:#x} during={during:#x}").unwrap();
    writeln!(log, "size {}x{} {:?}", size.width, size.height, term.capabilities()).unwrap();

    let started = Instant::now();
    let events = term.wait(Some(started + Duration::from_millis(300))).expect("wait");
    writeln!(log, "idle-300ms events={events:?} elapsed_ms={}", started.elapsed().as_millis()).unwrap();

    let waker = term.waker();
    let other = waker.clone();
    let thread = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(300));
        other.wake();
    });
    let started = Instant::now();
    let events = term.wait(Some(started + Duration::from_secs(5))).expect("wait");
    thread.join().unwrap();
    writeln!(log, "wake-from-thread (deadline 5s) events={events:?} elapsed_ms={}", started.elapsed().as_millis()).unwrap();

    waker.wake();
    let started = Instant::now();
    let events = term.wait(Some(started + Duration::from_secs(5))).expect("wait");
    writeln!(log, "wake-before-wait events={events:?} elapsed_ms={}", started.elapsed().as_millis()).unwrap();

    inject("a\u{1b}[1;5Cü");
    let (events, ms) = collect(&mut term, 3, Duration::from_secs(3));
    writeln!(log, "keys events={events:?} elapsed_ms={ms}").unwrap();

    inject("\u{1b}[<0;10;5M\u{1b}[<0;10;5m\u{1b}[<0;10;5M\u{1b}[<35;11;5M\u{1b}[<65;11;5M\u{1b}[<66;11;5M");
    let (events, ms) = collect(&mut term, 6, Duration::from_secs(3));
    writeln!(log, "mouse events={events:?} elapsed_ms={ms}").unwrap();

    inject("\u{1b}[200~ä€😀\r\nx\u{1b}[201~");
    let (events, ms) = collect(&mut term, 1, Duration::from_secs(3));
    writeln!(log, "paste events={events:?} elapsed_ms={ms}").unwrap();

    inject("\u{1b}[I\u{1b}[O");
    let (events, ms) = collect(&mut term, 2, Duration::from_secs(3));
    writeln!(log, "focus events={events:?} elapsed_ms={ms}").unwrap();

    inject("\u{1b}");
    let started = Instant::now();
    let events = term.wait(None).expect("wait");
    writeln!(log, "lone-escape (deadline None) events={events:?} elapsed_ms={}", started.elapsed().as_millis()).unwrap();

    let patch = AnsiPatch("\x1b[6;6H\x1b[0;38;2;255;0;0;48;2;0;0;40mhello\x1b[0m".to_string());
    let cursor = Some(CursorSpec { pos: Pos { x: 12, y: 5 }, shape: CursorShape::Bar, blink: false });
    writeln!(log, "present-with-cursor {:?}", term.present(&patch, cursor)).unwrap();
    writeln!(log, "present-unchanged {:?}", term.present(&AnsiPatch::default(), cursor)).unwrap();
    writeln!(log, "copy {:?}", term.copy("probe copy ä")).unwrap();

    let status = std::process::Command::new("cmd").args(["/c", "mode", "con:", "cols=91", "lines=27"]).stdout(std::process::Stdio::null()).status();
    writeln!(log, "mode-con {status:?}").unwrap();
    let (events, ms) = collect(&mut term, 1, Duration::from_secs(3));
    writeln!(log, "resize events={events:?} elapsed_ms={ms} size_now={:?}", term.size()).unwrap();

    term.leave().expect("leave");
    writeln!(log, "console mode after-leave={:#x} restored={}", console_mode(), console_mode() == before).unwrap();
    drop(term);

    let exe = std::env::current_exe().unwrap();
    let child = std::process::Command::new(exe).arg("panic-child").stderr(std::process::Stdio::null()).status();
    writeln!(log, "panic-child {child:?} console mode after panic={:#x} restored={}", console_mode(), console_mode() == before).unwrap();
}
