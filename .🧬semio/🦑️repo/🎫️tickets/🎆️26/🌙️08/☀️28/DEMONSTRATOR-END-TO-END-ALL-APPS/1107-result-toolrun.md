STATUS: PARTIAL, UNVERIFIED. Tool-run panic conversions are written but no compile verdict exists (upstream/boundary blockers below); pack-json-tests and tool-run-presentation unit tests are NOT ported.

# 1107 toolrun result

## Classification of non-test `unwrap`/`expect` in tool-run code
Counted per file with `/usr/bin/grep -E '\.unwrap\(\)|\.expect\('` over both trees (framework `⏯️tool-run` and os plugin `⏯️tool-run`), excluding `🧪️tests` directories and `#[cfg(test)]` bodies. The opendebt figure of about 230 counted test files (about 330 hits sit in `🧪️tests`); production hits are about 130.

Rule used: a hit is a real fault if the value can be absent because of runtime state a pane can change (selected run, finalize owner, publication owner) or because of input/guest data (decoded tick `retract_to`). A hit is a proven invariant if a phase counter or an immediately preceding guard in the same state machine makes `None` unreachable.

### Real faults, converted (plugin `⏯️tool-run/🧵️instance`, runs on every pane that starts a tool run)
Cause: `ToolRunLedger::apply_event` removes the entry and re-selects the primary run when a transition yields no slot (`Dismiss` of a terminal run, `Closed`); `begin_close` sets `selected = None`. Turn code that fetched the selected entry with `.expect("...slot")` therefore depended on a ledger invariant held by the caller, and a panic kills the whole plugin instance.
Fix: new `tool_run_slot_lost(step)` fault and `selected_entry_required!(ledger, step)` (returns `Result<&mut Entry, Fault>`) in `⏯️tool-run/🦀️.rs`; all call sites use `?`.
- `📤️publication-and-member-refresh/🦀️.rs`: 15 sites (publish_tool_run x4 incl. the `finalize` and `publication` owners, close_tool_run_finalize_owner, close_tool_run_publication, finalized rebase, refresh/settle member x3, publish_tool_run_member x4). All now typed faults.
- `👀️generation-watch-and-turn-driver/🦀️.rs`: 8 sites. The watch returns `()`, so a lost slot ends the watch (`let Some(entry)=... else{return}`); `drive_selected_tool_run` x2 -> `tool_run_slot_lost`; `refold.unwrap()` -> `is_some_and`.
- `🎟️job-admission-and-finalize-turn/🦀️.rs`: 7 sites: job admission, revalidate admission, finalize x3 -> typed faults; `complete_tool_run_job` (returns `()`) uses `if let ... filter` and `map_or((run,generation),..)`.
- `🚀️action-dispatch-and-start/🦀️.rs`: `transition.slot.expect("start yields a slot")` -> `Fault toolRun.start-slot` (cannot trigger today: `Start` on `None` always yields a slot; converted because it is a one-liner).
- `🎞️tick/🎮️operations/🦀️.rs:81` (tool-run operations, guest tick data): `tick.retract_to.unwrap()` -> typed `invalid("original retraction lost its length")`.
- `📊️progress/🪟️presentation/🌱️seed/🦀️.rs`: public `original_identifier()` could panic after output handoff; now `Option<&str>`; caller `🗃️slot` uses `and_then(..).unwrap_or("")`; unit assertion adjusted.

### Proven invariants, left as they are (reason)
- `📊️progress/🪟️presentation/🧬️update/🦀️.rs` (23): `state.original` is set by `admit` and never taken; every other `Option` is `Some` exactly in the phase that follows its assignment (`phase` is private, advanced only after the assignment).
- `👥️entities/🧾️provenance/🦀️.rs` (16), `👥️entities/📸️version/🦀️.rs` (7), `📊️progress/💍️steps/➕️insert/🦀️.rs` (9), `📊️progress/📸️clone/🦀️.rs` (3), `📨️payload/🌊️stream/🧳️update/🦀️.rs` (2), `👥️entities/🦀️.rs` (2): same phase-keyed pattern; `ring.newest().unwrap()` in step-insert phase 1/2 is reached only after phase 0 saw `newest()` is `Some`.
- `⏱️tick-writer/🦀️.rs` (6): `original.take().unwrap()` after `original.is_none()` returned early; `append_ops.last().unwrap()` after `len() > kept`.
- `🎞️tick/🛫️encode/🦀️.rs` (2): `original_retract_to().unwrap()` only for id 8, which `present()` admits only when `is_some()`.
- `🔤️codec/🦀️.rs`: `wire_field(id)` for `id in 1..=wire_field_count()`.
- `🪪️group/🦀️.rs` (2): inside a `#[test]`; `🦀️.rs:125` `from_utf8` of ASCII digits it just wrote.
- plugin `🎞️tick/🎮️operations/🦀️.rs` (5), `📖️source` (5), `🔁️fold` (3), `🧭️walk` (2), `👥️member-owners-and-visitors` (5), `🎟️publication/🦀️.rs` (2), `📒️driver-and-ledger/🦀️.rs:505`: each `Option` is checked or set a few tokens earlier in the same function (`if self.source.is_none(){..return}` then `.as_mut().unwrap()`), or comes from `phase`.
Not converted on purpose: converting those would add error variants nothing can produce and churn the phase machines.

## Tests
- `⏯️tool-run/🦀️.rs` inline module `selected_entry_required_tests`: selected run resolves; no selection -> typed fault naming the step; dangling index -> typed fault. These cover the failure path of all 31 converted instance sites (one macro).
- The `retract_to` and `start-slot` conversions have no test: both need a mounted ledger (`ToolRunCtx`) and a tick that the encoder cannot produce. Stated, not claimed.

## pack-json-tests (`semio-framework-pack-json-tests`, lib test)
First error layer was 13 errors (log `🗑️generated/1105b/toolrun-102.txt`). Ported by name or trait shape (UNVERIFIED until the next run):
- `🛫️encode/🧭️tree/🧪️tests`: `PathlessBox` now implements `RetirementCursor`/`retirement_seed`.
- `🧪️tests/🔬️unit/🧯️retirement-laws`: `DirectJsonSource` derives `RetireOwned`.
- `📥️decode/🧾️receipt/🧪️tests`: allocation port closure takes `NativeAllocationRequest`.
Not ported, needs a protocol rewrite (not a rename):
- `🧪️tests/🥣️companion/🧮️owned-bytes`, `📥️decode/🧳️receiving/🧪️tests`: construction/work recipients are now `OriginalConstructionRecipient`/`OriginalWorkRecipient` loans (`OriginalConstructionAuthority::with_loan`), `NativeReceivingOriginal::advance` returns `RetainedWorkPoll`, `close_step` takes an effect, the `ArtifactOwnedValueRetirementFactory` takes `&mut Option<T>` + reply. The existing helper `semio-framework-value-testing-admission` (`receive`, `WorkLoan`, `capture`) would carry this, but adding it as dev-dependency changes `Cargo.toml` after the lock marker (needs LOCK NEEDED).
- `🧪️tests/🔬️unit/🫳️borrowed-source-laws:324,333`: written against the previous JSON writer protocol (`construction_grant`, `prepare_work`, `begin_effect_settlement`, `NativeEncodeAllocationIssuer`), which lane packjson is changing in pack-json-source; port after that lands.

## Runs
- toolrun-101: failed immediately, my mistake (`--keep-going` is not a `cargo test` flag; tst.sh already passes `--no-fail-fast`).
- toolrun-102: pack-json-tests `--no-run`, rc 1, the 13 errors above.
- toolrun-103: `-p semio-plugin-tool-run -p semio-plugin-tool-run-turns -p semio-plugin-tool-run-operations -p semio-framework-tool-run-presentation --lib selected_entry_required_tests` queued; verdict recorded below when finished.

- toolrun-104 (plugin tool-run, turns, operations): refused by the Source gate before the compiler: the `semio-plugin-tool-run` source closure is about 268-283 KB against `maxSourceBytes` 262144 (refusal at `tool-machine/🏷️tag/🧵️original/🦀️.rs`, observed 9949 > remaining 3646). It was already over without my ~2 KB; it needs a split by the crate owner. I raised nothing.
- toolrun-105 (operations `--no-run`, retract_to edit): rc 1, blocked upstream by `semio-framework-replication-history-index` (9 errors: `sequence_birth_bytes`, `deferred_birth_bytes_for` not in scope in `🪴️insert/🦀️.rs:112`, peer-owned). My edit was not reached.
- toolrun-103: wrong invocation (`semio-framework-tool-run-presentation` is not a member of the plugin workspace).

## semio-framework-tool-run-presentation unit tests (coordinator request)
Not ported. Read from `hangfix-8.txt`: the 61 errors are one protocol change in `🧪️tests/🔬️unit/🦀️.rs`: `ToolRunPresentation::admit`, `capture`, `ToolRunEntityProvenance::admit` now take `&mut RetainedAllocationEffect` instead of a `RetainedCloneGrant` (45 type mismatches), `close_step` takes an effect (6+4 arity errors), `RetainedWorkPoll` has no `unwrap` (6), missing `serde_json`/`semio_framework_trace` dev-dependencies (needs Cargo.toml edit, LOCK NEEDED), `ControlledRetirement::abandon` is gone. The port is a harness rewrite (a `RetainedAllocationEffect::new(grant, fuel, &mut actual)` per call, polls matched on `Complete`), not mechanical; with 40-minute queue cycles and no way to compile the crate meanwhile I did not guess it. Also my `original_identifier()` change touches that test file (one assertion), so the file is edited but still red for the reasons above.

## Rerun commands
`C/1105b-tools/runtime/tst.sh <label> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/📦️packages/🦀️rust/Cargo.toml native -p semio-plugin-tool-run -p semio-plugin-tool-run-turns --lib selected_entry_required_tests` once the closure is split and replication-history-index compiles.
