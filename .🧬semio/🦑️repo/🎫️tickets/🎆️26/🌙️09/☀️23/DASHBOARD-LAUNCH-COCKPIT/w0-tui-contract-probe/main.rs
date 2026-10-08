//! 🔬️ Drives `NativeTerminal` through the §4.1 backend contract on a real pseudo-terminal and logs what it saw.

use std::io::Write;
use std::time::{Duration, Instant};
use ui_tui::tui::ansi::AnsiPatch;
use ui_tui::tui::backend::{NativeTerminal, TerminalBackend};
use ui_tui::tui::event::{Event, Key};

fn main() {
    let mut log = std::fs::File::create(std::env::args().nth(1).expect("log path")).expect("log file");
    let mut term = NativeTerminal::new().expect("controlling terminal");
    term.enter().expect("enter");
    let size = term.size().expect("size");
    writeln!(log, "size {}x{} {:?}", size.width, size.height, term.capabilities()).unwrap();

    let started = Instant::now();
    let events = term.wait(Some(started + Duration::from_millis(80))).expect("wait");
    writeln!(log, "idle-80ms events={events:?} elapsed_ms={}", started.elapsed().as_millis()).unwrap();

    let started = Instant::now();
    let events = term.wait(Some(started + Duration::from_millis(400))).expect("wait");
    writeln!(log, "idle-400ms events={events:?} elapsed_ms={}", started.elapsed().as_millis()).unwrap();

    let waker = term.waker();
    waker.wake();
    let started = Instant::now();
    let events = term.wait(Some(started + Duration::from_secs(2))).expect("wait");
    writeln!(log, "wake-before-wait events={events:?} elapsed_ms={}", started.elapsed().as_millis()).unwrap();

    let other = waker.clone();
    let thread = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(150));
        other.wake();
    });
    let started = Instant::now();
    let events = term.wait(None).expect("wait");
    writeln!(log, "wake-from-thread-at-150ms events={events:?} elapsed_ms={}", started.elapsed().as_millis()).unwrap();
    thread.join().unwrap();

    term.present(&AnsiPatch("PROBE-READY".to_string()), None).expect("present");
    term.copy("probe").expect("copy");
    writeln!(log, "ready").unwrap();

    let limit = Instant::now() + Duration::from_secs(10);
    'input: while Instant::now() < limit {
        let started = Instant::now();
        for event in term.wait(Some(started + Duration::from_millis(80))).expect("wait") {
            writeln!(log, "input {event:?} after_ms={}", started.elapsed().as_millis()).unwrap();
            if matches!(event, Event::Key(key) if key.key == Key::Char('q')) {
                break 'input;
            }
        }
    }
    term.leave().expect("leave");
    writeln!(log, "left").unwrap();
}
