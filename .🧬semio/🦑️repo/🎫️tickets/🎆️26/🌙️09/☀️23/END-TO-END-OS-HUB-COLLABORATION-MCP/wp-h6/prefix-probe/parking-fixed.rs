//! 🛌️ Idle native workers park until they are told something changed — never on a poll interval.
//!
//! The pool used to park every idle worker for at most 4 ms and answer every submission with
//! `notify_all`, so an idle pool woke all of its workers 250 times a second and each of them scanned
//! every lane queue plus the maintenance registry under their locks. Measured 2026-09-24 (ticket
//! 26/09/23 slice G6): those scans were the largest non-idle host share in every sample of a busy
//! semio MCP process. Here a worker sleeps on a condition variable until a signal names new work or
//! an earlier timer deadline, and exactly one idle worker (the timer keeper) sleeps with a timeout,
//! which is the earliest wheel deadline and nothing else.
//!
//! The lost-wakeup argument is the classic store/load pairing, all `SeqCst`: a signaller bumps
//! `signal` before it reads `idle`, and a parking worker bumps `idle` before it re-reads `signal`
//! under the state lock. Either the worker sees the new signal and does not sleep, or the signaller
//! sees the worker and notifies it under the same lock the worker is about to wait on.
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, PoisonError};
use std::time::Duration;

struct ParkState {
    sleepers: usize,
    keeper: bool,
    closed: bool,
}

/// 🛌️ Why [`WorkerParking::park`] returned. `Signalled` covers a signal that arrived before the
/// worker slept, so the caller rescans without having waited at all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ParkWake {
    Signalled,
    Woken,
    TimerDue,
    Closed,
}

pub(super) struct WorkerParking {
    signal: AtomicU64,
    idle: AtomicUsize,
    keeper: AtomicBool,
    timer_moved: AtomicBool,
    state: Mutex<ParkState>,
    work: Condvar,
    timer: Condvar,
    #[cfg(test)]
    sleeps: AtomicU64,
}

impl WorkerParking {
    pub(super) fn new() -> Self {
        Self { signal: AtomicU64::new(0), idle: AtomicUsize::new(0), keeper: AtomicBool::new(false), timer_moved: AtomicBool::new(false), state: Mutex::new(ParkState { sleepers: 0, keeper: false, closed: false }), work: Condvar::new(), timer: Condvar::new(), #[cfg(test)] sleeps: AtomicU64::new(0) }
    }

    /// 👁️ The signal generation a worker scans under; pass it back to [`Self::park`].
    pub(super) fn observe(&self) -> u64 {
        self.signal.load(Ordering::SeqCst)
    }

    /// 📣️ New runnable work: wakes one sleeper, or the timer keeper when it is the only idle worker.
    pub(super) fn signal_work(&self) {
        self.signal.fetch_add(1, Ordering::SeqCst);
        if self.idle.load(Ordering::SeqCst) == 0 {
            return;
        }
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.sleepers > 0 {
            self.work.notify_one();
        } else if state.keeper {
            self.timer.notify_one();
        }
    }

    /// ⏰️ The earliest timer deadline moved earlier: the keeper recomputes its timeout, or a sleeper
    /// becomes the keeper when none is armed.
    pub(super) fn signal_timer(&self) {
        self.signal.fetch_add(1, Ordering::SeqCst);
        if self.idle.load(Ordering::SeqCst) == 0 {
            return;
        }
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.keeper {
            self.timer.notify_one();
        } else if state.sleepers > 0 {
            self.work.notify_one();
        }
    }

    /// ⏰️ An earlier deadline registered by a timer callback running on the worker that is firing
    /// the wheel: that worker re-reads the wheel before it parks again, so nobody is woken now. The
    /// move is remembered, because an armed keeper still sleeps on the stale later deadline: the
    /// firing worker hands the new deadline to it before it runs a job or parks as a sleeper.
    pub(super) fn signal_timer_from_firing_worker(&self) {
        self.timer_moved.store(true, Ordering::SeqCst);
        self.signal.fetch_add(1, Ordering::SeqCst);
    }

    /// 🤝️ Called by a worker about to run a job while timers are pending: when no keeper is armed
    /// and someone sleeps, that sleeper takes over the wheel so a long job cannot delay a deadline;
    /// when a keeper sleeps on a deadline a firing callback moved earlier, it recomputes it.
    pub(super) fn hand_off_timers(&self) {
        let moved = self.timer_moved.swap(false, Ordering::SeqCst);
        if !moved && self.keeper.load(Ordering::SeqCst) || self.idle.load(Ordering::SeqCst) == 0 {
            return;
        }
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.keeper {
            if moved {
                self.timer.notify_one();
            }
        } else if state.sleepers > 0 {
            self.work.notify_one();
        }
    }

    /// 🛑️ Wakes every parked worker for shutdown; later parks return immediately.
    pub(super) fn close(&self) {
        self.signal.fetch_add(1, Ordering::SeqCst);
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.closed = true;
        self.work.notify_all();
        self.timer.notify_all();
    }

    /// 🛌️ Parks until a signal newer than `observed`, shutdown, or — for the one timer keeper —
    /// `timer_due` elapses. `timer_due` is `None` when the wheel holds no deadline.
    pub(super) fn park(&self, observed: u64, timer_due: Option<Duration>) -> ParkWake {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if state.keeper && self.timer_moved.swap(false, Ordering::SeqCst) {
            self.timer.notify_one();
        }
        self.idle.fetch_add(1, Ordering::SeqCst);
        let wake = if state.closed {
            ParkWake::Closed
        } else if self.signal.load(Ordering::SeqCst) != observed {
            ParkWake::Signalled
        } else {
            #[cfg(test)]
            self.sleeps.fetch_add(1, Ordering::Relaxed);
            match timer_due {
                Some(due) if !state.keeper => {
                    state.keeper = true;
                    self.keeper.store(true, Ordering::SeqCst);
                    let (guard, timeout) = self.timer.wait_timeout(state, due).unwrap_or_else(PoisonError::into_inner);
                    state = guard;
                    state.keeper = false;
                    self.keeper.store(false, Ordering::SeqCst);
                    if timeout.timed_out() { ParkWake::TimerDue } else { ParkWake::Woken }
                }
                _ => {
                    state.sleepers += 1;
                    state = self.work.wait(state).unwrap_or_else(PoisonError::into_inner);
                    state.sleepers -= 1;
                    ParkWake::Woken
                }
            }
        };
        self.idle.fetch_sub(1, Ordering::SeqCst);
        drop(state);
        wake
    }

    /// 📊️ How many times a worker actually went to sleep — the law's observable: an idle pool
    /// with no timers sleeps once per worker and never again.
    #[cfg(test)]
    pub(super) fn sleeps(&self) -> u64 {
        self.sleeps.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
