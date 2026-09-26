import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
pairs = [
("""                let outcomes = self.runtime.wait_for_outcomes(expected, RUN_TURN_OUTCOME_TIMEOUT);
                if outcomes.len() < granted.len() {""",
"""                let outcomes = self.runtime.wait_for_outcomes(expected, RUN_TURN_OUTCOME_TIMEOUT);
                eprintln!("[DEBUG] wg8 dispatch actor={} granted={granted:?} expected={expected} outcomes={:?}", actor.0, outcomes.iter().map(|outcome| match outcome { ShardOutcome::Turn { actor, result, jobs } => format!("Turn({actor},{:?},jobs={})", result.status, jobs.len()), ShardOutcome::Preempted { actor } => format!("Preempted({actor})"), ShardOutcome::Job { actor, .. } => format!("Job({actor})"), ShardOutcome::Fault { actor, message } => format!("Fault({actor},{message})"), other => format!("Other({})", other.actor()) }).collect::<Vec<_>>());
                if outcomes.len() < granted.len() {"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
