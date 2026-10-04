use crate::publication::{PublicationBound, PublicationControl, PublicationError, PublicationLimits, PublicationObserver, PublicationProgress, PublicationTransaction};
use crate::{poll::resolve_ready, CancelToken};
use serde_json::{json, Value};
use std::cell::Cell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::time::{Duration, Instant};

fn isolated_observation(name: &str, variable: &str, value: &str) -> (bool, std::process::Output) {
    let namespace = module_path!().split_once("::").unwrap().1;
    let law = format!("{namespace}::{name}");
    let mut child = Command::new(std::env::current_exe().unwrap()).args(["--exact", &law, "--nocapture"]).env(variable, value).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    let started = Instant::now();
    let mut timed_out = false;
    loop {
        if child.try_wait().unwrap().is_some() { break; }
        if started.elapsed() >= Duration::from_secs(3) {
            child.kill().unwrap();
            timed_out = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    (timed_out, child.wait_with_output().unwrap())
}

fn corpus() -> Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}

struct Observer {
    token: CancelToken,
    armed: Rc<Cell<&'static str>>,
    progress: Rc<Cell<u32>>,
    max_attempts: u32,
}

impl PublicationObserver for Observer {
    fn on_attempt(&mut self, progress: PublicationProgress) {
        let count = self.progress.get() + 1;
        self.progress.set(count);
        assert_eq!(progress.attempt, count);
        assert_eq!(progress.max_attempts, self.max_attempts);
        match self.armed.replace("none") {
            "cancel" => self.token.cancel_now(),
            "park" => resolve_ready(self.token.park()),
            "panic" => panic!("observer witness before gate ownership"),
            "none" => (),
            action => panic!("unknown observer action: {action}"),
        }
    }
}

struct HeldObserver;

impl PublicationObserver for HeldObserver {
    fn on_attempt(&mut self, progress: PublicationProgress) {
        assert_eq!(progress, PublicationProgress { attempt: 1, max_attempts: 1 });
    }
}

fn held_transaction() -> PublicationTransaction {
    let token = CancelToken::root_now();
    let mut observer = HeldObserver;
    let mut control = PublicationControl::new(PublicationLimits { max_attempts: 1, max_token_nodes: 1 }, &token, &mut observer).unwrap();
    control.try_begin().unwrap()
}

fn run_case(row: &Value) -> Value {
    let depth = usize::try_from(row["tokenChain"]["depth"].as_u64().unwrap()).unwrap();
    let mut chain = vec![CancelToken::root_now()];
    for index in 1..depth {
        let child = chain[index - 1].child_now();
        chain.push(child);
    }
    for entry in row["tokenChain"]["overrides"].as_array().unwrap() {
        let index = usize::try_from(entry["index"].as_u64().unwrap()).unwrap();
        let token = &chain[depth - 1 - index];
        match entry["state"].as_str().unwrap() {
            "cancelled" => token.cancel_now(),
            "parked" => resolve_ready(token.park()),
            state => panic!("unknown token state: {state}"),
        }
    }
    let token = chain.last().unwrap().clone();
    let limits = PublicationLimits {
        max_attempts: u32::try_from(row["limits"]["maxAttempts"].as_u64().unwrap()).unwrap(),
        max_token_nodes: u32::try_from(row["limits"]["maxTokenNodes"].as_u64().unwrap()).unwrap(),
    };
    let armed = Rc::new(Cell::new("none"));
    let progress = Rc::new(Cell::new(0));
    let mut observer = Observer { token: token.clone(), armed: armed.clone(), progress: progress.clone(), max_attempts: limits.max_attempts };
    let actual = match PublicationControl::new(limits, &token, &mut observer) {
        Err(PublicationError::InvalidLimits) => json!({ "construction": "invalid-limits", "results": [] }),
        Err(error) => panic!("unexpected constructor refusal: {error}"),
        Ok(mut control) => {
            let mut current = None;
            let mut other = None;
            let mut results = Vec::new();
            for action in row["actions"].as_array().unwrap() {
                match action.as_str().unwrap() {
                    "hold-other" => { assert!(other.is_none()); other = Some(held_transaction()); }
                    "release-other" => { assert!(other.take().is_some()); }
                    "release-current" => { assert!(current.take().is_some()); }
                    "poison" => { assert!(std::thread::spawn(|| { let _transaction = held_transaction(); panic!("writer poison witness"); }).join().is_err()); }
                    "cancel-current" => token.cancel_now(),
                    "park-current" => resolve_ready(token.park()),
                    "unpark-current" => resolve_ready(token.unpark()),
                    "arm-cancel-on-progress" => armed.set("cancel"),
                    "arm-park-on-progress" => armed.set("park"),
                    "arm-panic-on-progress" => armed.set("panic"),
                    "try" => {
                        let outcome = match catch_unwind(AssertUnwindSafe(|| control.try_begin())) {
                            Ok(Ok(transaction)) => { assert!(current.is_none()); current = Some(transaction); "acquired" }
                            Ok(Err(PublicationError::Busy)) => "busy",
                            Ok(Err(PublicationError::Cancelled)) => "cancelled",
                            Ok(Err(PublicationError::Parked)) => "parked",
                            Ok(Err(PublicationError::BoundExceeded(PublicationBound::Attempts))) => "bound-attempts",
                            Ok(Err(PublicationError::BoundExceeded(PublicationBound::TokenNodes))) => "bound-token-nodes",
                            Ok(Err(PublicationError::Unavailable)) => "unavailable",
                            Ok(Err(PublicationError::InvalidLimits)) => panic!("constructor authority was not retained"),
                            Err(_) => "observer-panicked",
                        };
                        results.push(json!({ "outcome": outcome, "attempts": control.attempts(), "progressEvents": progress.get() }));
                    }
                    action => panic!("unknown fixture action: {action}"),
                }
            }
            drop(other);
            drop(current);
            json!({ "construction": "ready", "results": results })
        }
    };
    drop(observer);
    drop(token);
    while chain.pop().is_some() {}
    actual
}

#[test]
fn fixture_case() {
    let corpus = corpus();
    let id = std::env::var("SEMIO_PUBLICATION_CASE_ID").unwrap_or_else(|_| "free-gate".to_owned());
    let row = corpus["cases"].as_array().unwrap().iter().find(|row| row["id"] == id).unwrap();
    let actual = run_case(row);
    println!("[DEBUG] Publication admission case {id}: {actual}");
    assert_eq!(actual, row["expected"], "{id}");
}

#[test]
fn all_rows_execute_in_distinct_process_gates() {
    let corpus = corpus();
    let rows = corpus["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 30);
    let mut failures = Vec::new();
    for row in rows {
        let id = row["id"].as_str().unwrap();
        let (timed_out, output) = isolated_observation("fixture_case", "SEMIO_PUBLICATION_CASE_ID", id);
        let observed = String::from_utf8(output.stdout).unwrap();
        let witness = format!("[DEBUG] Publication admission case {id}:");
        if timed_out || !output.status.success() || !observed.contains(&witness) {
            failures.push(format!("{id}: timedOut={timed_out}, status={}, stdout={observed}, stderr={}", output.status, String::from_utf8_lossy(&output.stderr)));
        }
        println!("[DEBUG] Publication subprocess {id}: status={}, timedOut={timed_out}", output.status);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn cross_thread_contention_releases_turn_before_retry() {
    if std::env::var("SEMIO_PUBLICATION_CONTENTION_CHILD").as_deref() != Ok("1") {
        let (timed_out, output) = isolated_observation("cross_thread_contention_releases_turn_before_retry", "SEMIO_PUBLICATION_CONTENTION_CHILD", "1");
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(!timed_out && output.status.success(), "timedOut={timed_out}, stdout={stdout}, stderr={}", String::from_utf8_lossy(&output.stderr));
        assert!(stdout.contains("[DEBUG] Publication contention: busy,acquired; attempts=2"));
        return;
    }
    let (held_sender, held_receiver) = std::sync::mpsc::sync_channel(0);
    let (release_sender, release_receiver) = std::sync::mpsc::sync_channel(0);
    let owner = std::thread::spawn(move || {
        let transaction = held_transaction();
        held_sender.send(()).unwrap();
        release_receiver.recv().unwrap();
        drop(transaction);
    });
    held_receiver.recv().unwrap();
    let token = CancelToken::root_now();
    let armed = Rc::new(Cell::new("none"));
    let progress = Rc::new(Cell::new(0));
    let mut observer = Observer { token: token.clone(), armed, progress: progress.clone(), max_attempts: 2 };
    let mut control = PublicationControl::new(PublicationLimits { max_attempts: 2, max_token_nodes: 1 }, &token, &mut observer).unwrap();
    assert!(matches!(control.try_begin(), Err(PublicationError::Busy)));
    assert_eq!(control.attempts(), 1);
    release_sender.send(()).unwrap();
    owner.join().unwrap();
    let transaction = control.try_begin().unwrap();
    assert_eq!(control.attempts(), 2);
    assert_eq!(progress.get(), 2);
    drop(transaction);
    println!("[DEBUG] Publication contention: busy,acquired; attempts=2");
}

#[test]
fn independent_mutex_observes_the_same_admission_subtraces() {
    let corpus = corpus();
    let rows = corpus["cases"].as_array().unwrap();
    for id in ["free-gate", "held-gate-returns-busy", "held-gate-resumes-after-release", "recursive-admission-is-finite", "released-owner-can-reacquire"] {
        let row = rows.iter().find(|row| row["id"] == id).unwrap();
        let mutex = parking_lot::Mutex::new(());
        let mut current = None;
        let mut other = None;
        let mut observed = Vec::new();
        for action in row["actions"].as_array().unwrap() {
            match action.as_str().unwrap() {
                "hold-other" => { assert!(other.is_none()); other = Some(mutex.try_lock().unwrap()); }
                "release-other" => { assert!(other.take().is_some()); }
                "release-current" => { assert!(current.take().is_some()); }
                "try" => match mutex.try_lock() {
                    Some(guard) => { assert!(current.is_none()); current = Some(guard); observed.push("acquired"); }
                    None => observed.push("busy"),
                },
                action => panic!("non-admission action in independent subtrace: {action}"),
            }
        }
        let expected = row["expected"]["results"].as_array().unwrap().iter().map(|result| result["outcome"].as_str().unwrap()).collect::<Vec<_>>();
        println!("[DEBUG] Independent publication admission {id}: {observed:?}");
        assert_eq!(observed, expected, "{id}");
    }
}
