use super::*;
use crate::actors::tests::{admission, envelope, id, perfect, ADA, BOB, TENANT};
use crate::actors::{deciders, enrollment, handle_key, learner_key, ProctorDeciders};
use crate::catalog::tests::fixture;
use crate::storage::Database;
use quiz::{Best, Challenge, Command, CrowdView, HandleHolder, LearnerView, RunStatus, RunView, TranscriptBadge, TranscriptRun, CHALLENGES, LEADERBOARD_PERIODS};
use server::authority::{AuthorityDirectory, CommandBus};
use server::contract::{ActorKey, CommandOutcome, EventRecord, HybridLogicalClock, PolicyDecision, TenantId};
use server::storage::AuthorityStore;

/// 🎬️ A bus over `database` with the proctor's deciders, allowing every command.
pub(crate) async fn bus(database: &Database, catalog: &Arc<LoadedCatalog>) -> CommandBus<SqliteAuthorityStore, ProctorDeciders> {
    let mut bus = CommandBus::new(AuthorityDirectory::new(), SqliteAuthorityStore::new(database.clone()), Box::new(|_| PolicyDecision::Allow));
    for decider in deciders(catalog, &admission(0)) {
        bus.register(decider).await;
    }
    bus
}

/// 📨️ Submit one quiz command and relay what a handle decided, like the enrollment saga does.
pub(crate) async fn submit(bus: &mut CommandBus<SqliteAuthorityStore, ProctorDeciders>, command: &Command, millis: u64) -> CommandOutcome {
    let outcome = bus.submit(envelope(command), HybridLogicalClock { millis, counter: 0 }).await;
    if let CommandOutcome::Accepted { events, .. } = &outcome {
        for relay in events.iter().filter_map(enrollment) {
            bus.submit(relay, HybridLogicalClock { millis, counter: 1 }).await;
        }
    }
    outcome
}

fn command_id(number: usize, run: &str) -> String {
    format!("c{number:07x}{}", &run[8..])
}

/// ▶️ Start a run of `quiz` at `challenge` as `learner` and answer every sheet task perfectly — on a
/// timed run each right after opening it —, without submitting.
pub(crate) async fn answer(bus: &mut CommandBus<SqliteAuthorityStore, ProctorDeciders>, catalog: &LoadedCatalog, learner: &str, run: &str, quiz: &str, challenge: Challenge, millis: u64) {
    submit(bus, &Command::StartRun { id: command_id(1, run), learner: learner.into(), run: run.into(), quiz: quiz.into(), challenge, at: millis }, millis).await;
    let document = &catalog.current()[quiz].quiz;
    for (index, task) in quiz::sheet_of(document, quiz::run_seed(run), challenge).tasks.iter().enumerate() {
        if task.seconds().is_some() {
            let opened = submit(bus, &Command::OpenTask { id: command_id(50 + index, run), learner: learner.into(), run: run.into(), task: task.id().clone(), at: millis }, millis).await;
            assert!(matches!(opened, CommandOutcome::Accepted { .. }), "{opened:?}");
        }
        let command = Command::RecordAnswer { id: command_id(2 + index, run), learner: learner.into(), run: run.into(), task: task.id().clone(), answer: perfect(document, task), at: millis + 1 };
        let outcome = submit(bus, &command, millis + 1).await;
        assert!(matches!(outcome, CommandOutcome::Accepted { .. }), "{outcome:?}");
    }
}

/// 🏁️ Play one perfect run of `quiz` at `challenge` as `learner`.
pub(crate) async fn play(bus: &mut CommandBus<SqliteAuthorityStore, ProctorDeciders>, catalog: &LoadedCatalog, learner: &str, run: &str, quiz: &str, challenge: Challenge, millis: u64) {
    answer(bus, catalog, learner, run, quiz, challenge, millis).await;
    assert!(matches!(submit(bus, &Command::SubmitRun { id: command_id(99, run), learner: learner.into(), run: run.into() }, millis + 2).await, CommandOutcome::Accepted { .. }));
}

async fn catch_up(projector: &Projector, database: &Database) -> CatchUp {
    let mut projections = SqliteProjectionStore::new(database.clone());
    projector.catch_up(&SqliteAuthorityStore::new(database.clone()), &mut projections, &CancelToken::root_now(), |_| {}).await.unwrap()
}

async fn failure(projector: &Projector, database: &Database) -> String {
    let mut projections = SqliteProjectionStore::new(database.clone());
    match projector.catch_up(&SqliteAuthorityStore::new(database.clone()), &mut projections, &CancelToken::root_now(), |_| {}).await {
        Err(StorageError::Backend(detail)) => detail,
        other => panic!("the fold must fail, got {other:?}"),
    }
}

fn view<T: serde::de::DeserializeOwned>(bytes: Option<Vec<u8>>) -> T {
    serde_json::from_slice(&bytes.expect("a projected view")).unwrap()
}

async fn transcripts(projections: &SqliteProjectionStore) -> Vec<Transcript> {
    projections.list(LEADERBOARD, "").await.iter().map(|(_, bytes)| serde_json::from_slice(bytes).unwrap()).collect()
}

const DAY: Timestamp = 86_400_000;
/// 🗓️ Tuesday 2026-09-29 00:00 UTC: two days before a month turns, one after a week began.
const TUESDAY: Timestamp = 20_725 * DAY;
const QUIZZES: [&str; 2] = ["power", "homes"];

/// 🏟️ The all-time leaderboard of every quiz as `caller` is answered.
fn all_time(board: &Board, caller: Option<&str>) -> Leaderboard {
    board.view(LeaderboardPeriod::AllTime, None, 0, caller).expect("the board of every quiz")
}

/// 🎲️ One more run of a random quiz at a random challenge and a random instant of the four days from
/// Monday, with a dyadic score and its points, and — one time in three — a badge it earned.
fn played(transcript: &mut Transcript, random: &mut quiz::Mt19937) {
    let (quiz, challenge, score) = (QUIZZES[(random.next_u32() % 2) as usize].to_string(), CHALLENGES[(random.next_u32() % 4) as usize], f64::from(random.next_u32() % 9) / 8.0);
    let run = TranscriptRun { quiz, challenge, score, points: quiz::points(score, challenge), at: TUESDAY - DAY + u64::from(random.next_u32() % 16) * (DAY / 4) + u64::from(random.next_u32() % 3) };
    if random.next_u32().is_multiple_of(3) {
        transcript.badges.push(TranscriptBadge { badge: format!("badge-{}", transcript.badges.len()), quiz: run.quiz.clone(), at: run.at });
    }
    transcript.runs.push(run);
    transcript.runs.sort_by_key(|run| run.at);
}

/// 📜️ The transcript of learner `seed` with one to three random runs.
fn record(seed: u32, random: &mut quiz::Mt19937) -> Transcript {
    let learner = format!("{seed:032x}");
    let mut transcript = Transcript { tag: quiz::learner_tag(&learner), learner, identity: Identity::Anonymous, runs: Vec::new(), badges: Vec::new() };
    for _ in 0..=random.next_u32() % 3 {
        played(&mut transcript, random);
    }
    transcript
}

/// ⚖️ Hold every leaderboard of `board` — each period, of every quiz and of each one, at instants on
/// both sides of a day, a week and a month — to the core's over `transcripts`, for every caller.
fn held_to_the_core(board: &Board, catalog: &LoadedCatalog, transcripts: &[Transcript], callers: &[Option<String>]) {
    for at in [TUESDAY - DAY, TUESDAY - 1, TUESDAY, TUESDAY + DAY, TUESDAY + 2 * DAY - 1, TUESDAY + 2 * DAY, TUESDAY + 40 * DAY] {
        for period in LEADERBOARD_PERIODS {
            for quiz in [None, Some("power"), Some("homes")] {
                for caller in callers {
                    assert_eq!(board.view(period, quiz, at, caller.as_deref()), Some(quiz::leaderboard(transcripts, catalog.view(), period, quiz, at, caller.as_deref())), "{period:?} of {quiz:?} at {at} for {caller:?}");
                }
            }
        }
    }
}

#[tokio::test]
async fn played_runs_fold_into_learner_run_handle_and_leaderboard_views() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Pseudonym { handle: " Ada  Lovelace ".into() } }, 100).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(2), learner: BOB.into(), identity: Identity::Anonymous }, 101).await;
    play(&mut bus, &catalog, ADA, &id(10), "power", Challenge::Medium, 200).await;
    let projector = Projector::new(Arc::clone(&catalog));
    let CatchUp::Current(at) = catch_up(&projector, &database).await else { panic!("not cancelled") };
    assert_eq!(at.position, SqliteAuthorityStore::new(database.clone()).log_head().unwrap());
    let projections = SqliteProjectionStore::new(database.clone());
    let ada: LearnerView = view(projections.get(LEARNERS, ADA).await);
    assert_eq!(ada.identity, Identity::Pseudonym { handle: "Ada Lovelace".into() });
    assert_eq!(ada.best.get("power"), Some(&Best { challenge: Challenge::Medium, score: 1.0, points: 200.0 }));
    assert_eq!((ada.total, ada.runs[0].challenge, ada.runs[0].points), (200.0, Challenge::Medium, Some(200.0)));
    assert_eq!(ada.badges.iter().map(|award| award.badge.as_str()).collect::<Vec<_>>(), ["perfect-power", "sorter"]);
    let run: RunView = view(projections.get(RUNS, &id(10)).await);
    assert_eq!((run.status, run.result.map(|result| result.score)), (RunStatus::Submitted, Some(1.0)));
    let bob: LearnerView = view(projections.get(LEARNERS, BOB).await);
    assert!(bob.runs.is_empty());
    assert_eq!(view::<HandleHolder>(projections.get(HANDLES, "ada lovelace").await), HandleHolder { learner: ADA.into(), identity: Identity::Pseudonym { handle: "Ada Lovelace".into() } });
    assert_eq!(projections.list(HANDLES, "").await.len(), 1, "an anonymous learner holds no handle");
    assert_eq!((projector.learners().load(Ordering::Acquire), view::<u64>(projections.get(META, LEARNER_COUNT_KEY).await)), (2, 2));
    let board = all_time(&projector.board(), Some(ADA));
    assert_eq!(board.rows.iter().map(|row| (row.rank, row.tag.clone(), row.total)).collect::<Vec<_>>(), [(1, quiz::learner_tag(ADA), 200.0)]);
    assert_eq!((board.learners, board.submissions, board.own.as_ref()), (1, 1, board.rows.first()));
    assert_eq!(all_time(&projector.board(), Some(BOB)).own, None, "a learner without a submitted run has no row");
    let stored = transcripts(&projections).await;
    assert_eq!(board, quiz::leaderboard(&stored, catalog.view(), LeaderboardPeriod::AllTime, None, 0, Some(ADA)));
    assert!(!serde_json::to_string(&board).unwrap().contains(ADA), "no learner id leaves the board");
    let today = projector.board().view(LeaderboardPeriod::Daily, Some("power"), 202, Some(ADA)).expect("a quiz of the catalog");
    assert_eq!((today.rows.clone(), today.period, today.quiz.as_deref(), today.window), (board.rows.clone(), LeaderboardPeriod::Daily, Some("power"), quiz::period_window(LeaderboardPeriod::Daily, 0)));
    for (period, quiz, at) in [(LeaderboardPeriod::Daily, None, DAY), (LeaderboardPeriod::Weekly, None, 4 * DAY), (LeaderboardPeriod::Monthly, None, 31 * DAY), (LeaderboardPeriod::AllTime, Some("homes"), 0)] {
        let other = projector.board().view(period, quiz, at, Some(ADA)).expect("a quiz of the catalog");
        assert_eq!((other.rows.len(), other.learners, other.submissions, other.own.clone()), (0, 0, 1, None), "{period:?} of {quiz:?} at {at}");
        assert_eq!(other, quiz::leaderboard(&stored, catalog.view(), period, quiz, at, Some(ADA)));
    }
    assert_eq!(projector.board().view(LeaderboardPeriod::AllTime, Some("cooling"), 0, None), None, "a quiz the catalog does not list has no leaderboard");
}

#[tokio::test]
async fn the_best_of_a_quiz_is_its_run_with_the_most_points_and_another_challenge_voids_the_open_run() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Anonymous }, 100).await;
    play(&mut bus, &catalog, ADA, &id(10), "power", Challenge::Hard, 200).await;
    play(&mut bus, &catalog, ADA, &id(11), "power", Challenge::Easy, 300).await;
    submit(&mut bus, &Command::StartRun { id: id(2), learner: ADA.into(), run: id(12), quiz: "power".into(), challenge: Challenge::Easy, at: 400 }, 400).await;
    let switched = submit(&mut bus, &Command::StartRun { id: id(3), learner: ADA.into(), run: id(13), quiz: "power".into(), challenge: Challenge::Expert, at: 401 }, 401).await;
    let CommandOutcome::Accepted { events, .. } = switched else { panic!("another challenge starts: {switched:?}") };
    assert_eq!(events.iter().map(|event| event.kind.as_str()).collect::<Vec<_>>(), ["quiz.run-voided", "quiz.run-started"]);
    let projector = Projector::new(Arc::clone(&catalog));
    catch_up(&projector, &database).await;
    let projections = SqliteProjectionStore::new(database.clone());
    let ada: LearnerView = view(projections.get(LEARNERS, ADA).await);
    let hard = Best { challenge: Challenge::Hard, score: 1.0, points: 300.0 };
    assert_eq!((ada.best.get("power"), ada.total), (Some(&hard), 300.0), "a perfect easy run earns fewer points than a perfect hard one");
    assert_eq!(ada.runs.iter().map(|run| (run.challenge, run.status, run.points)).collect::<Vec<_>>(), [(Challenge::Expert, RunStatus::Open, None), (Challenge::Easy, RunStatus::Voided, None), (Challenge::Easy, RunStatus::Submitted, Some(100.0)), (Challenge::Hard, RunStatus::Submitted, Some(300.0))]);
    let run: RunView = view(projections.get(RUNS, &id(13)).await);
    assert_eq!((run.sheet.challenge, run.opened, run.hints), (Challenge::Expert, Some(BTreeMap::new()), None), "an expert run shows which tasks are opened, none yet");
    let board = all_time(&projector.board(), Some(ADA));
    assert_eq!((board.rows[0].total, board.rows[0].best.get("power")), (300.0, Some(&hard)));
    assert_eq!(board, quiz::leaderboard(&transcripts(&projections).await, catalog.view(), LeaderboardPeriod::AllTime, None, 0, Some(ADA)));
}

#[tokio::test]
async fn every_claimed_handle_counts_as_a_registration_even_when_one_learner_claims_them_all() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    for (seed, handle) in [(1, "Ada"), (2, "Countess"), (3, "Lovelace")] {
        let claimed = submit(&mut bus, &Command::IdentifyLearner { id: id(seed), learner: ADA.into(), identity: Identity::Pseudonym { handle: handle.into() } }, 100 + u64::from(seed)).await;
        assert!(matches!(claimed, CommandOutcome::Accepted { .. }), "{claimed:?}");
    }
    submit(&mut bus, &Command::IdentifyLearner { id: id(4), learner: BOB.into(), identity: Identity::Anonymous }, 200).await;
    let projector = Projector::new(Arc::clone(&catalog));
    catch_up(&projector, &database).await;
    let projections = SqliteProjectionStore::new(database.clone());
    assert_eq!((projector.learners().load(Ordering::Acquire), view::<u64>(projections.get(META, LEARNER_COUNT_KEY).await)), (4, 4), "three handles and one anonymous learner are four registrations against the cap");
    for key in ["ada", "countess", "lovelace"] {
        assert_eq!(view::<HandleHolder>(projections.get(HANDLES, key).await).learner, ADA, "{key}");
    }
    assert_eq!(view::<LearnerView>(projections.get(LEARNERS, ADA).await).identity, Identity::Pseudonym { handle: "Ada".into() }, "a learner keeps the identity it registered first");
    assert_eq!(SqliteAuthorityStore::new(database.clone()).events_since(&learner_key(TENANT, ADA), 0).await.unwrap().len(), 1, "a learner is enrolled once");
    let mut rewound = SqliteProjectionStore::new(database.clone());
    rewound.set_checkpoint(CHECKPOINT, 0).await.unwrap();
    catch_up(&Projector::new(Arc::clone(&catalog)), &database).await;
    assert_eq!(view::<u64>(projections.get(META, LEARNER_COUNT_KEY).await), 4, "a replayed claim is counted once");
}

#[tokio::test]
async fn only_a_submission_a_badge_or_a_registration_rewrites_a_transcript_and_only_other_facts_than_answers_a_learner_view() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    let projector = Projector::new(Arc::clone(&catalog));
    let projections = SqliteProjectionStore::new(database.clone());
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Anonymous }, 100).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(2), learner: BOB.into(), identity: Identity::Anonymous }, 101).await;
    play(&mut bus, &catalog, ADA, &id(10), "power", Challenge::Medium, 200).await;
    play(&mut bus, &catalog, BOB, &id(11), "power", Challenge::Medium, 300).await;
    catch_up(&projector, &database).await;
    let before = (projections.get(LEADERBOARD, ADA).await, projections.get(LEADERBOARD, BOB).await, projections.get(LEARNERS, BOB).await, all_time(&projector.board(), Some(BOB)));
    assert_eq!(before.3.own.as_ref().map(|row| row.rank), Some(2));
    let homes = projector.board().view(LeaderboardPeriod::Weekly, Some("homes"), 400, Some(BOB)).expect("a quiz of the catalog");
    assert_eq!((homes.learners, homes.submissions), (0, 2), "nobody submitted this quiz yet");

    submit(&mut bus, &Command::StartRun { id: command_id(1, &id(12)), learner: BOB.into(), run: id(12), quiz: "homes".into(), challenge: Challenge::Expert, at: 400 }, 400).await;
    catch_up(&projector, &database).await;
    let started = projections.get(LEARNERS, BOB).await;
    assert_ne!(started, before.2, "a started run is in the learner view");
    assert_eq!((projections.get(LEADERBOARD, ADA).await, projections.get(LEADERBOARD, BOB).await), (before.0.clone(), before.1.clone()));

    let document = &catalog.current()["homes"].quiz;
    let sheet = quiz::sheet_of(document, quiz::run_seed(&id(12)), Challenge::Expert);
    for (index, task) in sheet.tasks.iter().enumerate() {
        submit(&mut bus, &Command::OpenTask { id: command_id(50 + index, &id(12)), learner: BOB.into(), run: id(12), task: task.id().clone(), at: 401 }, 401).await;
    }
    catch_up(&projector, &database).await;
    assert_eq!(projections.get(LEARNERS, BOB).await, started, "an opened task leaves the learner view alone");
    assert_eq!(view::<RunView>(projections.get(RUNS, &id(12)).await).opened, Some(sheet.tasks.iter().map(|task| (task.id().clone(), 401)).collect()), "an opened task is in the run view");
    for (index, task) in sheet.tasks.iter().enumerate() {
        submit(&mut bus, &Command::RecordAnswer { id: command_id(2 + index, &id(12)), learner: BOB.into(), run: id(12), task: task.id().clone(), answer: perfect(document, task), at: 402 }, 402).await;
    }
    catch_up(&projector, &database).await;
    assert_eq!(projections.get(LEARNERS, BOB).await, started, "answers leave the learner view alone");
    assert_eq!((projections.get(LEADERBOARD, ADA).await, projections.get(LEADERBOARD, BOB).await, all_time(&projector.board(), Some(BOB))), (before.0.clone(), before.1.clone(), before.3.clone()), "opened tasks and answers leave every transcript and the board alone");
    assert!(!view::<RunView>(projections.get(RUNS, &id(12)).await).answers.is_empty(), "answers are in the run view");

    submit(&mut bus, &Command::SubmitRun { id: command_id(99, &id(12)), learner: BOB.into(), run: id(12) }, 402).await;
    catch_up(&projector, &database).await;
    assert_eq!(projections.get(LEADERBOARD, ADA).await, before.0, "another learner's transcript is not rewritten");
    assert_ne!(projections.get(LEADERBOARD, BOB).await, before.1);
    let after = all_time(&projector.board(), Some(ADA));
    assert_eq!((after.rows.iter().map(|row| row.tag.clone()).collect::<Vec<_>>(), after.own.as_ref().map(|row| row.rank), after.learners, after.submissions), (vec![quiz::learner_tag(BOB), quiz::learner_tag(ADA)], Some(2), 2, 3));
    let stored = transcripts(&projections).await;
    assert_eq!(after, quiz::leaderboard(&stored, catalog.view(), LeaderboardPeriod::AllTime, None, 0, Some(ADA)));
    let homes = projector.board().view(LeaderboardPeriod::Weekly, Some("homes"), 400, Some(BOB)).expect("a quiz of the catalog");
    assert_eq!((homes.rows.iter().map(|row| (row.tag.clone(), row.runs)).collect::<Vec<_>>(), homes.own.as_ref().map(|row| row.rank)), (vec![(quiz::learner_tag(BOB), 1)], Some(1)), "the index that was asked for before the submission follows it");
    assert_eq!(homes, quiz::leaderboard(&stored, catalog.view(), LeaderboardPeriod::Weekly, Some("homes"), 400, Some(BOB)));
}

#[tokio::test]
async fn a_refold_and_a_restarted_projector_reach_the_same_views() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Name { handle: "Ada".into() } }, 100).await;
    play(&mut bus, &catalog, ADA, &id(10), "power", Challenge::Expert, 200).await;
    let first = Projector::new(Arc::clone(&catalog));
    catch_up(&first, &database).await;
    let projections = SqliteProjectionStore::new(database.clone());
    let read = || async { (projections.get(LEARNERS, ADA).await, projections.get(RUNS, &id(10)).await, projections.get(LEADERBOARD, ADA).await, projections.get(HANDLES, "ada").await, projections.get(META, LEARNER_COUNT_KEY).await) };
    let before = read().await;
    let board = all_time(&first.board(), Some(ADA));
    let mut rewound = SqliteProjectionStore::new(database.clone());
    rewound.set_checkpoint(CHECKPOINT, 0).await.unwrap();
    let restarted = Projector::new(Arc::clone(&catalog));
    assert!(!restarted.board().loaded());
    catch_up(&restarted, &database).await;
    assert_eq!((read().await, all_time(&restarted.board(), Some(ADA)), restarted.learners().load(Ordering::Acquire)), (before.clone(), board.clone(), 1), "a replayed batch changes nothing and the board is loaded from its rows");
    let mut cleared = SqliteProjectionStore::new(database.clone());
    restarted.reset(&mut cleared).await.unwrap();
    assert_eq!((cleared.get(LEARNERS, ADA).await, cleared.get(HANDLES, "ada").await, restarted.board().learners(), restarted.learners().load(Ordering::Acquire)), (None, None, 0, 0));
    catch_up(&restarted, &database).await;
    assert_eq!((read().await, all_time(&restarted.board(), Some(ADA))), (before, board));
}

#[tokio::test]
async fn a_cancelled_fold_stops_between_batches_and_resumes() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Name { handle: "Ada".into() } }, 100).await;
    let projector = Projector::new(Arc::clone(&catalog));
    let cancel = CancelToken::root_now();
    cancel.cancel_now();
    let mut projections = SqliteProjectionStore::new(database.clone());
    let log = SqliteAuthorityStore::new(database.clone());
    assert!(matches!(projector.catch_up(&log, &mut projections, &cancel, |_| {}).await.unwrap(), CatchUp::Cancelled(Progress { position: 0, folded: 0, .. })));
    let mut reported = Vec::new();
    let CatchUp::Current(at) = projector.catch_up(&log, &mut projections, &CancelToken::root_now(), |progress| reported.push(progress)).await.unwrap() else { panic!("not cancelled") };
    assert_eq!((at.position, at.folded, reported.last().copied()), (log.log_head().unwrap(), 2, Some(at)));
}

#[tokio::test]
async fn projections_of_another_catalog_are_reset() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut projections = SqliteProjectionStore::new(database.clone());
    let projector = Projector::new(Arc::clone(&catalog));
    assert!(!projector.prepare(&mut projections).await.unwrap(), "fresh read models are stamped, not reset");
    assert_eq!(projections.get(META, FINGERPRINT_KEY).await, Some(projector.stamp()));
    assert_eq!(projector.stamp(), format!("{PROJECTOR_REVISION}:{}", catalog.fingerprint).into_bytes());
    assert!(!projector.prepare(&mut projections).await.unwrap());
    projections.put(META, FINGERPRINT_KEY, b"another catalog".to_vec()).await.unwrap();
    projections.put(LEARNERS, ADA, b"stale".to_vec()).await.unwrap();
    assert!(projector.prepare(&mut projections).await.unwrap());
    assert_eq!(projections.get(LEARNERS, ADA).await, None);
    assert_eq!(projections.get(META, FINGERPRINT_KEY).await, Some(projector.stamp()));
    projections.put(META, FINGERPRINT_KEY, format!("{}:{}", PROJECTOR_REVISION - 1, catalog.fingerprint).into_bytes()).await.unwrap();
    assert!(projector.prepare(&mut projections).await.unwrap(), "an earlier projector's read models are rebuilt");
}

#[tokio::test]
async fn an_event_the_fold_cannot_read_stops_it_with_its_position_and_cause() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Anonymous }, 100).await;
    let mut log = SqliteAuthorityStore::new(database.clone());
    let recalled = EventRecord { stream: learner_key(TENANT, ADA), seq: 2, hlc: HybridLogicalClock::default(), kind: "quiz.learner-recalled".into(), payload: format!(r#"{{"type":"learner-recalled","learner":"{ADA}","at":5}}"#).into_bytes() };
    log.append_events(&learner_key(TENANT, ADA), std::slice::from_ref(&recalled), &[]).await.unwrap();
    submit(&mut bus, &Command::IdentifyLearner { id: id(2), learner: BOB.into(), identity: Identity::Anonymous }, 101).await;
    let projector = Projector::new(Arc::clone(&catalog));
    let detail = failure(&projector, &database).await;
    assert!(detail.contains("event at position 2") && detail.contains(&format!("{TENANT}/quiz-learner/{ADA} seq 2")) && detail.contains("quiz.learner-recalled") && detail.contains("unknown variant"), "{detail}");
    let projections = SqliteProjectionStore::new(database.clone());
    assert_eq!((projections.checkpoint(CHECKPOINT).await, projections.get(LEARNERS, ADA).await, projections.get(LEARNERS, BOB).await), (0, None, None), "the batch is not written and the checkpoint does not pass the event");
    assert_eq!(failure(&projector, &database).await, detail, "the next catch-up fails the same way instead of skipping");
}

#[tokio::test]
async fn a_stream_the_proctor_does_not_know_and_a_misplaced_fact_stop_the_fold() {
    let catalog = Arc::new(fixture());
    let registered = |learner: &str, identity: Identity| serde_json::to_vec(&Event::LearnerRegistered { learner: learner.into(), identity, at: 1 }).unwrap();
    let record = |stream: &ActorKey, payload: Vec<u8>| EventRecord { stream: stream.clone(), seq: 1, hlc: HybridLogicalClock::default(), kind: "quiz.learner-registered".into(), payload };
    let cases = [
        (ActorKey { tenant: TenantId(TENANT.into()), kind: "quiz-roster".into(), id: "roster".into() }, registered(ADA, Identity::Anonymous), "the stream kind \"quiz-roster\" is none this proctor folds"),
        (handle_key(TENANT, "ada"), registered(ADA, Identity::Name { handle: "Bob".into() }), "the registered handle is not the one this stream is named after"),
        (handle_key(TENANT, "ada"), registered(ADA, Identity::Anonymous), "a handle stream holds registrations under a handle"),
        (learner_key(TENANT, ADA), registered(BOB, Identity::Anonymous), "it is a fact of learner bbbb"),
        (learner_key(TENANT, ADA), b"\xff".to_vec(), "cannot be folded"),
    ];
    for (stream, payload, cause) in cases {
        let database = Database::memory().unwrap();
        SqliteAuthorityStore::new(database.clone()).append_events(&stream, &[record(&stream, payload)], &[]).await.unwrap();
        let detail = failure(&Projector::new(Arc::clone(&catalog)), &database).await;
        assert!(detail.contains("event at position 1") && detail.contains(cause), "{detail}");
    }
}

#[tokio::test]
async fn a_stored_state_or_transcript_that_does_not_decode_is_an_error_not_a_fresh_start() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let mut bus = bus(&database, &catalog).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Anonymous }, 100).await;
    let projector = Projector::new(Arc::clone(&catalog));
    catch_up(&projector, &database).await;
    let mut projections = SqliteProjectionStore::new(database.clone());
    projections.put(STATES, ADA, b"{\"seq\":1}".to_vec()).await.unwrap();
    submit(&mut bus, &Command::StartRun { id: id(3), learner: ADA.into(), run: id(10), quiz: "power".into(), challenge: Challenge::Medium, at: 200 }, 200).await;
    let detail = failure(&projector, &database).await;
    assert!(detail.contains(&format!("projection {STATES}/{ADA} does not decode")), "{detail}");
    projections.put(LEADERBOARD, ADA, b"not a transcript".to_vec()).await.unwrap();
    let restarted = failure(&Projector::new(Arc::clone(&catalog)), &database).await;
    assert!(restarted.contains(&format!("projection {LEADERBOARD}/{ADA} does not decode")), "{restarted}");
}

#[test]
fn every_board_is_the_core_leaderboard_over_every_transcript_whatever_order_they_arrive_in_and_whenever_it_is_asked() {
    let catalog = Arc::new(fixture());
    let mut random = quiz::Mt19937::new(7);
    let mut transcripts: Vec<Transcript> = (0..333u32).map(|seed| record(seed, &mut random)).collect();
    let board = Board::new(Arc::clone(&catalog));
    assert!(!board.loaded());
    board.fill(Vec::new());
    let callers = [None, Some(transcripts[0].learner.clone()), Some(transcripts[332].learner.clone()), Some("nobody".to_string())];
    for entry in &transcripts[..111] {
        board.set(entry.clone());
    }
    held_to_the_core(&board, &catalog, &transcripts[..111], &callers);
    for entry in &transcripts[111..] {
        board.set(entry.clone());
    }
    held_to_the_core(&board, &catalog, &transcripts, &callers);
    for (index, entry) in transcripts.iter_mut().enumerate().filter(|(index, _)| index % 7 == 0) {
        played(entry, &mut random);
        if index % 3 == 0 {
            entry.identity = Identity::Pseudonym { handle: format!("Learner {index}") };
        }
        board.set(entry.clone());
        board.set(entry.clone());
    }
    assert_eq!(board.learners(), 333);
    held_to_the_core(&board, &catalog, &transcripts, &callers);
    let view = all_time(&board, Some(&transcripts[5].learner));
    assert_eq!((view.rows.len(), view.learners, view.own.is_some(), view.submissions), (LEADERBOARD_TOP, 333, true, transcripts.iter().map(|entry| entry.runs.len()).sum()));
    let filled = Board::new(Arc::clone(&catalog));
    filled.fill(transcripts.clone());
    assert!(filled.loaded());
    held_to_the_core(&filled, &catalog, &transcripts, &callers);
    assert_eq!(filled.transcript(&transcripts[1].learner).as_deref(), Some(&transcripts[1]));
    filled.clear();
    assert_eq!((filled.loaded(), filled.learners()), (false, 0));
}

#[test]
fn a_board_asked_in_the_next_window_forgets_the_runs_of_the_last_one() {
    let catalog = Arc::new(fixture());
    let board = Board::new(Arc::clone(&catalog));
    let learner = format!("{:032x}", 1);
    let run = |at: Timestamp| TranscriptRun { quiz: "power".to_string(), challenge: Challenge::Hard, score: 0.5, points: 150.0, at };
    let mut transcript = Transcript { tag: quiz::learner_tag(&learner), learner: learner.clone(), identity: Identity::Anonymous, runs: vec![run(TUESDAY + 5)], badges: Vec::new() };
    board.fill(vec![transcript.clone()]);
    let ranked = |at: Timestamp| board.view(LeaderboardPeriod::Daily, None, at, Some(&learner)).map(|answer| (answer.learners, answer.own.map(|row| row.runs), answer.window));
    assert_eq!(ranked(TUESDAY + 9), Some((1, Some(1), quiz::period_window(LeaderboardPeriod::Daily, TUESDAY))));
    assert_eq!(ranked(TUESDAY + DAY), Some((0, None, quiz::period_window(LeaderboardPeriod::Daily, TUESDAY + DAY))), "the next day begins empty");
    transcript.runs.push(run(TUESDAY + DAY + 1));
    board.set(transcript.clone());
    assert_eq!(ranked(TUESDAY + DAY + 2), Some((1, Some(1), quiz::period_window(LeaderboardPeriod::Daily, TUESDAY + DAY))), "and counts only its own runs");
    assert_eq!(ranked(TUESDAY + 9), Some((1, Some(1), quiz::period_window(LeaderboardPeriod::Daily, TUESDAY))), "a clock that steps back is answered for its window");
    assert_eq!(board.view(LeaderboardPeriod::Weekly, None, TUESDAY + DAY, Some(&learner)).and_then(|answer| answer.own).map(|row| row.runs), Some(2));
}

#[tokio::test]
async fn submitted_runs_fold_into_the_crowd_view_of_their_quiz() {
    let catalog = Arc::new(fixture());
    let database = Database::memory().unwrap();
    let projector = Projector::new(Arc::clone(&catalog));
    let mut projections = SqliteProjectionStore::new(database.clone());
    projector.prepare(&mut projections).await.unwrap();
    let empty: CrowdView = view(projections.get(CROWDS, "homes").await);
    assert_eq!(empty, quiz::crowd_view::<quiz::RunResult>(&catalog.current()["homes"].quiz, &[]), "every quiz has its crowd view before anybody submitted");

    let mut bus = bus(&database, &catalog).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(1), learner: ADA.into(), identity: Identity::Anonymous }, 100).await;
    submit(&mut bus, &Command::IdentifyLearner { id: id(2), learner: BOB.into(), identity: Identity::Anonymous }, 101).await;
    play(&mut bus, &catalog, ADA, &id(10), "power", Challenge::Medium, 200).await;
    catch_up(&projector, &database).await;
    play(&mut bus, &catalog, BOB, &id(11), "power", Challenge::Hard, 300).await;
    catch_up(&projector, &database).await;

    let mut results: Vec<quiz::RunResult> = Vec::new();
    for run in [id(10), id(11)] {
        results.push(view::<RunView>(projections.get(RUNS, &run).await).result.expect("submitted"));
    }
    assert!(serde_json::to_string(&results[1]).unwrap().contains("\"miss\":false"), "the hard run was guessed");
    let crowd: CrowdView = view(projections.get(CROWDS, "power").await);
    assert_eq!(crowd, quiz::crowd_view(&catalog.current()["power"].quiz, &results));
    assert_eq!((crowd.runs, crowd.tasks.iter().map(|task| task.items.iter().map(|item| item.answers).max().unwrap_or(0)).collect::<Vec<_>>()), (2, vec![2, 2, 2]));
    assert_eq!((crowd.scores.iter().sum::<usize>(), crowd.tasks.iter().map(|task| task.scores.iter().sum::<usize>()).collect::<Vec<_>>()), (2, vec![2, 2, 2]), "both runs fall into the score bins of the quiz and of every task");
    assert!(crowd.tasks[0].items.iter().all(|item| item.places.as_ref().is_some_and(|places| places.iter().sum::<usize>() == item.answers)), "a sorting item's places hold every answer");
    assert_eq!(view::<CrowdView>(projections.get(CROWDS, "homes").await), empty, "another quiz's crowd is untouched");

    let before = projections.get(CROWDS, "power").await;
    let restarted = Projector::new(Arc::clone(&catalog));
    let mut cleared = SqliteProjectionStore::new(database.clone());
    restarted.reset(&mut cleared).await.unwrap();
    assert_eq!(view::<CrowdView>(cleared.get(CROWDS, "power").await).runs, 0);
    catch_up(&restarted, &database).await;
    assert_eq!(projections.get(CROWDS, "power").await, before, "the crowd is rebuilt from the log");
    let resumed = Projector::new(Arc::clone(&catalog));
    play(&mut bus, &catalog, ADA, &id(12), "power", Challenge::Expert, 400).await;
    catch_up(&resumed, &database).await;
    assert_eq!(view::<CrowdView>(projections.get(CROWDS, "power").await).runs, 3, "a restarted projector resumes from the stored tally");
}
