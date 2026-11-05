//! 🧪️ The solve inference — determinism, the contradiction verdict and the entropy map.

use crate::host::inferences::{Wfc2dInferenceJob, Wfc2dInferenceRequest};
use crate::standards::v1::subsets::any::schema::inferences::Wfc2dInferenceCommit;
use crate::schema::inferences::{solve_relations };


use crate::host::inferences::{solve_with_clock};
use crate::schema::snapshot::{Wfc2dRule, Wfc2dSnapshot};

/// 🎲 One seed, one answer: two solves of the same document agree exactly. This is what makes the
/// `DepHash` caching over the inferred fields sound.
fn resumable_result(snapshot: &Wfc2dSnapshot) -> Result<Wfc2dInferenceCommit, String> {
    let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(0), semio_framework_job::Generation(1), snapshot.seed);
    let mut job = Wfc2dInferenceJob::new(operation, Wfc2dInferenceRequest { snapshot: Some(snapshot.clone()), document: None, checkpoint: None })?;
    let mut driver = semio_s_plugin_wfc_engine_oracle::step_context::TestJobDriver::new(operation.operation, operation.generation);
    let bytes = driver.completed_output(&mut job, 64, 2_000_000)?;
    let text = std::str::from_utf8(&bytes).map_err(|error| error.to_string())?;
    crate::standards::v1::subsets::any::io::text::inferences::decode_inference_value::<Wfc2dInferenceCommit>(text).map_err(|error| error.to_string())
}

#[test]
fn a_seed_pins_the_solve() {
    let document = crate::examples::two_room_corridor::document();
    let first = solve_with_clock(&document, semio_framework_job::logical_now_us).expect("the corridor spec solves");
    let second = solve_with_clock(&document, semio_framework_job::logical_now_us).expect("the corridor spec solves again");
    assert_eq!(first, second, "the same seed produced two different solves");
    assert!(!first.contradiction);
    assert_eq!(first.assignments.len(), document.slots.len());
}

/// 🚪️ The corridor spec FORCES an alternation: both same-tile adjacencies are denied, so a path of
/// three slots must two-colour — the middle differs from both ends and the two ends agree. Which of
/// the two colourings the seed picks is the solver's business; that it alternates is the law. (An
/// earlier version of this test asserted one specific colouring and was simply wrong: a path graph
/// under "no two neighbours alike" has two valid answers, not one.)
#[test]
fn the_forced_corridor_alternates() {
    let commit = solve_with_clock(&crate::examples::two_room_corridor::document(), semio_framework_job::logical_now_us).expect("solves");
    let middle = commit.assignments.get("corridor").expect("the middle slot is assigned");
    let left = commit.assignments.get("room-a").expect("the left slot is assigned");
    let right = commit.assignments.get("room-b").expect("the right slot is assigned");
    assert_ne!(middle, left, "two neighbours may not carry the same tile");
    assert_ne!(middle, right, "two neighbours may not carry the same tile");
    assert_eq!(left, right, "both ends sit at the same distance from the middle, so they agree");
}

/// 🩺 Two adjacent slots pinned to tiles whose pairing is forbidden is UNSATISFIABLE — the honest
/// answer is the contradiction verdict, never a fault and never a half-filled assignment. The engine
/// itself reports this as a `wfc-unsatisfiable` FAULT; turning that into a verdict is the inference's
/// own job, and this is the test that pins it.
#[test]
fn two_adjacent_pins_of_a_forbidden_pair_contradict() {
    let mut document = crate::examples::two_room_corridor::document();
    document.slots[1].pinned_tile_id = Some("room".into());
    document.slots[0].pinned_tile_id = Some("room".into());
    document.edges.retain(|edge| edge.id == "edge-a-corridor");
    let commit = solve_with_clock(&document, semio_framework_job::logical_now_us).expect("an unsatisfiable spec still commits a verdict");
    assert!(commit.contradiction, "pinning room beside room must contradict rule-room-room");
    assert!(commit.assignments.is_empty(), "a contradiction assigns nothing");
    assert_eq!(commit.entropy.len(), document.slots.len(), "the entropy map still covers every slot");
}

/// 🎲 A pinned slot has zero entropy; an open one has the prior over the tile weights.
#[test]
fn the_entropy_map_zeroes_pinned_slots() {
    let commit = solve_with_clock(&crate::examples::wall_roof_facade_strip::document(), semio_framework_job::logical_now_us).expect("solves");
    assert_eq!(commit.entropy.get("bay-1-top"), Some(&0.0));
    assert!(commit.entropy.get("bay-0-top").is_some_and(|entropy| *entropy > 0.0));
}

/// 🔗 A rule scoped to one relation only constrains that relation's edges.
#[test]
fn a_relation_scoped_rule_binds_only_its_own_class() {
    let document = crate::examples::wall_roof_facade_strip::document();
    assert_eq!(solve_relations(&document), vec!["above".to_string(), "beside".to_string()]);
    let commit = solve_with_clock(&document, semio_framework_job::logical_now_us).expect("the facade spec solves");
    assert!(!commit.contradiction);
    assert_eq!(commit.assignments.get("bay-1-top").map(String::as_str), Some("roof"));
    assert_eq!(commit.assignments.get("bay-0-top").map(String::as_str), Some("wall"), "roof beside roof is denied on the `beside` relation");
}

/// ⬡️ The non-rectangular ring solves, which is the whole reason this artifact is not `grid2d`.
#[test]
fn the_hex_ring_solves() {
    let commit = solve_with_clock(&crate::examples::hex_ring::document(), semio_framework_job::logical_now_us).expect("the ring solves");
    assert!(!commit.contradiction);
    assert_eq!(commit.assignments.len(), 6);
}

/// 💡️ The `InferredField` façades agree with the job they both drive.
#[test]
fn resumable_job_and_headless_batch_publish_the_same_answer() {
    let document = crate::examples::two_room_corridor::document();
    let stepped = resumable_result(&document).expect("resumable corridor completes");
    let batch = solve_with_clock(&document, semio_framework_job::logical_now_us).expect("headless corridor completes");
    assert_eq!(stepped, batch);
    assert!(!stepped.contradiction);
}

/// 🕳️ A document with no slots is trivially satisfiable and commits an empty assignment.
#[test]
fn an_empty_document_commits_nothing() {
    let commit = solve_with_clock(&Wfc2dSnapshot::default(), semio_framework_job::logical_now_us).expect("an empty spec still commits");
    assert!(commit.assignments.is_empty());
    assert!(!commit.contradiction);
}

/// ⛔️ A deny always beats an allow of the same pair, whichever order the rules are authored in: the
/// alternation the `room`/`room` deny forces survives a later `room`/`room` allow.
#[test]
fn a_deny_beats_an_allow_of_the_same_pair() {
    let mut document = crate::examples::two_room_corridor::document();
    document.rules.push(Wfc2dRule { id: "zz-rule-room-room-allow".into(), tile_a_id: "room".into(), tile_b_id: "room".into(), relation: None, allowed: true });
    let commit = solve_with_clock(&document, semio_framework_job::logical_now_us).expect("solves");
    assert_ne!(commit.assignments.get("corridor"), commit.assignments.get("room-a"), "the deny still forbids two like neighbours");
    assert_ne!(commit.assignments.get("corridor"), commit.assignments.get("room-b"), "the deny still forbids two like neighbours");
}
