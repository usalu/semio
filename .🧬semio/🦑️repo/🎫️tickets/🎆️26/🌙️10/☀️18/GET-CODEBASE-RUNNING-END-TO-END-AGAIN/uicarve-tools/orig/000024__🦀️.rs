mod native_common_tests {
    use super::*;
    use crate::tui::event::{Key, KeyEvent, MouseEvent};
    use std::time::Duration;

    #[test]
    fn input_pipeline_counts_clicks_from_the_wall_clock_of_arrival() {
        let mut input = InputPipeline::new();
        let mut events = Vec::new();
        input.feed(b"\x1b[<0;3;3M", &mut events);
        input.feed(b"\x1b[<0;3;3m", &mut events);
        input.feed(b"\x1b[<0;3;3M", &mut events);
        let clicks: Vec<u8> = events.iter().map(|event| if let Event::Mouse(MouseEvent { clicks, .. }) = event { *clicks } else { 0 }).collect();
        assert_eq!(clicks, vec![1, 1, 2]);
    }

    #[test]
    fn input_pipeline_reports_partial_input_deadlines_and_expires_them() {
        let mut input = InputPipeline::new();
        let mut events = Vec::new();
        assert_eq!(input.remaining(Instant::now()), None);
        input.feed(b"\x1b", &mut events);
        let remaining = input.remaining(Instant::now()).expect("a lone escape is pending");
        assert!(remaining <= crate::tui::ansi::ESCAPE_TIMEOUT);
        input.expire_due(Instant::now(), &mut events);
        assert!(events.is_empty(), "not due yet");
        input.expire_due(Instant::now() + crate::tui::ansi::ESCAPE_TIMEOUT + Duration::from_millis(1), &mut events);
        assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Esc, mods: 0 })]);
        assert_eq!(input.remaining(Instant::now()), None);
    }

    #[test]
    fn a_byte_arriving_after_the_escape_timeout_is_not_an_alt_chord() {
        let mut input = InputPipeline::new();
        let mut events = Vec::new();
        input.feed(b"", &mut events);
        std::thread::sleep(crate::tui::ansi::ESCAPE_TIMEOUT + Duration::from_millis(20));
        input.feed(b"j", &mut events);
        assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Esc, mods: 0 }), Event::Key(KeyEvent { key: Key::Char('j'), mods: 0 })]);
    }

    #[test]
    fn next_timeout_picks_the_nearer_of_deadline_and_partial_input() {
        let mut input = InputPipeline::new();
        let now = Instant::now();
        assert_eq!(next_timeout(None, &input, now), None);
        assert_eq!(next_timeout(Some(now + Duration::from_secs(5)), &input, now), Some(Duration::from_secs(5)));
        let mut events = Vec::new();
        input.feed(b"\x1b", &mut events);
        let nearer = next_timeout(Some(Instant::now() + Duration::from_secs(5)), &input, Instant::now()).expect("bounded");
        assert!(nearer <= crate::tui::ansi::ESCAPE_TIMEOUT);
        assert_eq!(next_timeout(Some(now), &input, now + Duration::from_secs(1)), Some(Duration::ZERO), "a passed deadline never waits");
    }

    #[test]
    fn resize_tracker_reports_only_real_changes() {
        let mut tracker = ResizeTracker::default();
        let first = Size { width: 80, height: 24 };
        tracker.baseline(first);
        assert_eq!(tracker.observe(first), None);
        assert_eq!(tracker.observe(Size { width: 0, height: 24 }), None);
        let second = Size { width: 100, height: 30 };
        assert_eq!(tracker.observe(second), Some(Event::Resize(second)));
        assert_eq!(tracker.observe(second), None);
    }

    #[test]
    fn ceil_millis_never_rounds_a_wait_down_to_a_spin() {
        assert_eq!(ceil_millis(Duration::ZERO), 0);
        assert_eq!(ceil_millis(Duration::from_micros(1)), 1);
        assert_eq!(ceil_millis(Duration::from_micros(1001)), 2);
        assert_eq!(ceil_millis(Duration::from_millis(80)), 80);
    }

    #[test]
    fn panic_restore_fires_only_for_the_thread_that_entered_the_terminal() {
        static FIRED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        fn fire() {
            FIRED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        panic_restore::install(fire);
        panic_restore::claim();
        let other = std::thread::spawn(|| std::panic::catch_unwind(|| panic!("a panic on a thread that never entered the terminal")).is_err());
        assert!(other.join().unwrap(), "the foreign panic really unwound");
        assert_eq!(FIRED.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert!(std::panic::catch_unwind(|| panic!("a panic on the thread that owns the terminal")).is_err());
        assert_eq!(FIRED.load(std::sync::atomic::Ordering::SeqCst), 1);
        panic_restore::release();
        assert!(std::panic::catch_unwind(|| panic!("a panic after the terminal was released")).is_err());
        assert_eq!(FIRED.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
