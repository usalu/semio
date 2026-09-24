import pathlib, sys

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs")
text = path.read_text()
pairs = [
    ("""            let mut owed = run_turn_continuation_turns(continuation);
            let mut quiet = 0usize;
            while let Some(events) = owed.pop_front() {
                if std::time::Instant::now() >= deadline || quiet >= run_turn_quiescent_continuations() {
                    break;
                }
                let (later, next) = self.run_turn_once(actor, instance, events, deadline).await?;
                quiet = if later.carries_nothing() { quiet + 1 } else { 0 };
                outcome.absorb(later)?;
                owed.extend(run_turn_continuation_turns(next));
            }
            Ok(outcome)
""", """            let mut owed = run_turn_continuation_turns(continuation);
            let mut quiet = 0usize;
            let debug_started = std::time::Instant::now();
            let mut debug_turns = 0usize;
            while let Some(events) = owed.pop_front() {
                if std::time::Instant::now() >= deadline || quiet >= run_turn_quiescent_continuations() {
                    eprintln!("[DEBUG] wg8 settle stop actor={} deadline={} quiet={quiet} turns={debug_turns} elapsed={:?} next={:?}", actor.0, std::time::Instant::now() >= deadline, debug_started.elapsed(), events.iter().map(|event| format!("{event:?}").chars().take(60).collect::<String>()).collect::<Vec<_>>());
                    break;
                }
                let (later, next) = self.run_turn_once(actor, instance, events, deadline).await?;
                debug_turns += 1;
                quiet = if later.carries_nothing() { quiet + 1 } else { 0 };
                outcome.absorb(later)?;
                owed.extend(run_turn_continuation_turns(next));
            }
            if debug_started.elapsed() > std::time::Duration::from_secs(2) {
                eprintln!("[DEBUG] wg8 settle slow actor={} turns={debug_turns} quiet={quiet} elapsed={:?}", actor.0, debug_started.elapsed());
            }
            Ok(outcome)
"""),
    ("""                let mut granted: Vec<u64> = decision.run.iter().map(|grant| grant.actor.0).collect();
""", """                let mut granted: Vec<u64> = decision.run.iter().map(|grant| grant.actor.0).collect();
                if granted.iter().any(|granted| *granted != actor.0) {
                    eprintln!("[DEBUG] wg8 foreign grants actor={} granted={granted:?}", actor.0);
                }
"""),
]
if sys.argv[1:] == ["revert"]:
    pairs = [(new, old) for old, new in pairs]
for old, new in pairs:
    assert text.count(old) == 1, old[:80]
    text = text.replace(old, new)
path.write_text(text)
print("ok")
