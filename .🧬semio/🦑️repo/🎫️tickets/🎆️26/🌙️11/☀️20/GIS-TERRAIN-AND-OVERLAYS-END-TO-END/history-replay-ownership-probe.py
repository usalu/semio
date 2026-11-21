from pathlib import Path

patch = Path(__file__).with_name("history-replay-close.patch").read_text()
start = patch.index("+/// 🧹️ Keeps the replay")
end = patch.index("\n ", start)
print("\n".join(line[1:] for line in patch[start:end].splitlines()))
print(r"""
struct Replay([usize; 5320], &'static std::sync::atomic::AtomicUsize);

impl Drop for Replay {
    fn drop(&mut self) {
        assert_eq!(self.0[0], 42);
        self.1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

#[no_mangle]
#[inline(never)]
fn clear_paid_replay(slot: &mut HistoryReadReplaySlot<Replay>) {
    slot.destroy_terminal();
}

fn main() {
    static DROPS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let mut slot = HistoryReadReplaySlot::default();
    assert!(!slot.is_some());
    assert!(std::mem::size_of::<Replay>() > 32_768);
    for expected in 1..=2 {
        slot.install_original(Replay([42; 5320], &DROPS));
        assert!(slot.as_ref().is_some() && slot.as_mut().is_some());
        clear_paid_replay(&mut slot);
        assert!(slot.is_none() && slot.as_ref().is_none());
        assert_eq!(DROPS.load(std::sync::atomic::Ordering::SeqCst), expected);
    }
    drop(slot);
    assert_eq!(DROPS.load(std::sync::atomic::Ordering::SeqCst), 2);
}
""")

