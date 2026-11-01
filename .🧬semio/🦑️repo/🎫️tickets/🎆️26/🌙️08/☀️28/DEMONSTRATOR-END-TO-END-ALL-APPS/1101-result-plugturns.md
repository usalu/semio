STATUS: GREEN (wasm32-wasip2 --lib, hub manifest): `C/🗑️generated/1101/plugturns-1.rc`=0, zero `error` lines in `plugturns-1.txt`; "Finished `prototype`" after Checking time-travel-turns, instance-typed-ops, tool-run-turns (and tool-run).

Command: `C/1101-tools/chk.sh plugturns-1 "🌎️hub/Cargo.toml" wasm --lib -p semio-framework-plugin-instance-typed-ops -p semio-framework-plugin-time-travel-turns -p semio-framework-plugin-tool-run-turns -p semio-framework-plugin-tool-run`

## Finding
No edits by this lane were needed. The three crates and the `tool-run` `⏱️limits-and-job-slot` borrow error were already ported by a peer (typing `🎟️original/🦀️.rs` last modified 08:15, already using `RetainedWorkOutcome{result,work}` closures and `Send+Sync` on the Drop impl) after the stale `stdio-2` log. The error counts in the brief came from that stale log.

## Not verified
Tests (`--tests`) of these crates and native target were not checked. Warnings (5590 total in the log) not addressed.
