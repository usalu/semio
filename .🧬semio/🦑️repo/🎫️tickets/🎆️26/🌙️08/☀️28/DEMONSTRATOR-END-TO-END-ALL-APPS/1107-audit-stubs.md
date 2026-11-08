# 🕳️ 1107 Stubs Audit (lane `stubs`)

Read-only audit. Scope: `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand` Rust, working tree plus `git diff HEAD~3` (added lines). No edits, no cargo, no git modifying commands.

Inputs written to `C/🗑️generated/`:
- `1107-stubs-diff.txt` (full `git diff HEAD~3 --unified=0 -- '*.rs'`, 104,337 lines, 17 MB, 2,929 files)
- `1107-added.tsv` (every added line tagged with its file path)
- `1107-snapclone-hunks.txt` (snapshot-clone hunks)

## Method and coverage

- Pattern sweep over all added lines: `todo!`, `unimplemented!`, `#[ignore]`, `cfg(any())`, `let _ = effect`, `Ok(())` / `Ok(Default::default())` / `work:0` / `Complete(Default)` / `Progress(Default)` bodies, `Pending` arms mapped to silent `Ok`, and refusal messages with "unsupported", "stub", "not yet", "not ported".
- Full reads of the named crates' hot paths: snapshot-clone, operation-wire replacement receiver, tool-run admit/tick, energy close and retirement helpers, plugin exchange.
- Not exhaustive. The diff has about 52k added lines. The 115 added `unreachable!` sites were not audited one by one; spot checks are invariant guards in state machines.

Global counts in added lines: `todo!` 0, `unimplemented!` 0, `#[cfg(any())]` 0, `#[ignore]` 1 (test only), `let _ = effect` 0 direct.

## True stubs (production-reachable or placeholder)

### S1. Plugin exchange `CommandText` arm returns a fixed "unsupported" fault
- File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🦀️.rs:269-271`
- Code: `protocol::AppCommand::CommandText { seq, line: _ } => { push_os_fault(&mut frames, Some(*seq), "unsupported", "CommandText not yet wired".into()).await; }`
- Old code: identical at HEAD~3 (lines 342-343). This is not a lane regression. It was re-reported because the file was reformatted.
- Runtime: any text command sent over the app command channel gets an OS fault reply and does no work. The `line` payload is dropped.
- Severity: high for any pane that sends text commands. Unknown whether any pane UI sends `CommandText`. Check the pane command grep before the browser phase.

### S2. Snapshot-clone "receiving recovery" hooks are no-op trait impls
- File: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:531-534` (`RetainedClonePreparation` impl of `ArtifactStoreOneItemPreparation`)
- Code: `has_pending_receiving_recovery -> false`, `pending_receiving_demand -> None`, `pending_receiving_recovery -> None`, `finish_receiving_recovery -> RetainedWorkOutcome{result: Ok(Default::default()), work: 0}`.
- Old code: these four methods did not exist. `advance` returned `OriginalRetirementFailure` and `close_step` took a grant. Store4 added the recovery stubs with the trait change (`C/1105b-claims.md` line 1328, "recovery stubs").
- Runtime: `finish_receiving_recovery` is reachable only when `has_pending_receiving_recovery()` is true. It is hard-wired false, so the body should be dead. If any caller calls it unconditionally, it reports success with zero work and zero progress.
- Severity: low (dead by contract). Should be an `invariant` error, not `Ok(Default)`.
- Open question (not confirmed): `advance` skips `prepare_work` and `commit_work` in the `Seal` phase (`native` flag). Work observed there is not committed to the receiver. Owner must confirm the native seal accounts for it elsewhere.
- Positive check: the close ladder in the same file is honest. Every progressing branch goes through `RetirementPollExit::settle`, which pays one unit when a progressing turn recorded no work. Verified at lines 614-730.

## Silent drops and idle returns (not stubs, but they behave like stubs)

These compile and run, but they discard work or a failure. Ranked by risk.

### D1. Replacement receiver maps `Pending` to silent zero progress
- File: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🧳️replacement/🦀️.rs:40`
- Code: `return match receiver.advance(control, grant.maximum_copy_bytes, maximum_work, grant) { RetainedWorkPoll::Pending{..} => Ok(Default::default()), RetainedWorkPoll::Complete(turn) => ... }`
- Runtime: a pending child returns `Ok(RetainedCloneProgress::default())`. No refusal, and the `work` in `Pending{work}` is dropped. The job caller sees success with no progress and repolls. If the child never completes, the turn loops with no visible cause (W4 says this must be a typed refusal).
- Severity: medium-high. The runtime addendum says host recipients are synchronous, so `Pending` should be unreachable here and must be a typed refusal.

### D2. Tool-run `admit_sealed` drops surplus work when unfunded
- File: `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs:48`
- Code: `let surplus = turn.work.saturating_sub(quote); if surplus>0 { let _ = record_work(effect, surplus); }`
- Doc comment on the same function says "surplus beyond the quote is recorded best effort".
- Runtime: a refused debit for surplus work is silently discarded, so the turn's `performed` is short by `surplus`. This is under-charge of fuel. Nothing else debits it.
- Same pattern in `🧰️framework/🔨️modules/⏯️tool-run/👥️entities/🦀️.rs:24` and `:31`.
- Severity: medium. Fuel accounting is exact by rule. Either make the refusal typed or prove the surplus cannot happen.

### D3. Tool-run tick native receive maps zero-work pending to progress
- File: `🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🦀️.rs:101`
- Code: `Ok(NativeReceivingEffectPoll::Pending{work:0}) => Ok(RetainedCloneStep::Progress(Default::default()))`
- This is the backpressure path. It is probably intended idle, not a stub. The `_ =>` arm above it calls `control.finish_receiving(...)` and maps `Ok(_) => Ok(())`, which drops `turn.work`. Not verified. Owner should check that finish work is recorded.
- Severity: low to medium.

### D4. Energy close returns idle Pending with zero work when the grant cannot fund
- Files: `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧪️sim/♻️close/🦀️.rs:51` (`retire_retained_tail`: `if !energy_grant_covers(grant,demand){return Ok(energy_idle())}`), `…/🔚️close/🦀️.rs:116, 143, 208`, and the helpers in `…/🪜️retirement/🦀️.rs:38` (`energy_builder_close`, unfunded returns `Progress(default)`), `:95` (`energy_guarded`, unfunded returns `Progress(default)`), `:122` (`energy_idle`).
- Runtime: an idle Pending is correct if the job driver backs off until the grant grows. It is a freeze if the grant can never cover the demand (for example a zero depth or item grant). W4 requires a typed refusal or `Blocked{reason}` for a close that cannot progress.
- Work accounting: `energy_outcome` (`:51`) charges 1 unit iff the step made progress, and `energy_construct` charges 1 iff the construction succeeded. Neither measures the work actually performed. Under the W1/W2 rules this is a fixed-work heuristic.
- Severity: medium.

### D5. Test-only `#[ignore]` leaves a law unrun
- File: `🧰️framework/🔨️modules/🌱️value/🛫️encode/♻️retirement/🧪️tests/🦀️.rs:105`
- `#[ignore = "recorded Source quotes run through prepare_source_allocation, which this law does not drive yet: see C/1107-result-testdebt-f.md"]`
- Test only. Mentioned because the lane rules forbid skipped tests. Severity: low for runtime, medium for coverage.

## Legitimate refusals and naming (not stubs)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️.rs` ("original contributed stub handoff requires depth"): a depth refusal. The word "stub" is a name for a handoff type, not a placeholder.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📥️lifted/🏭️operation/🦀️.rs` ("unsupported factory request"): a label for `admit_retained_clone_close`. Refusal path, real work.
- `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/📬️typed-operation/📥️receiving/🦀️.rs` ("native byte receiving retirement is unsupported"): a typed refusal when `ControlledRetirement::new` cannot take the wire. Real refusal.
- `✏️s/🔌️plugins/🧩️puzzle/…/🧱️board-events-and-fasteners/🦀️.rs` (`Fault::from("puzzle5d-board-brush-download-unsupported")`): an explicit refusal for brush download kinds. Real refusal, but a user-visible gap. Owner should confirm it is planned.
- `✏️s/🔌️plugins/🔋️energy/…/🧮️reduce-and-helpers/🦀️.rs` ("temporary document retirement stalled under its own quote"): a `WorkLimit` refusal. Real refusal.
- `unreachable!` sites in state machines (115 added, not audited one by one). The ones on paths I read are guarded by prior state checks. Sample only.

## Checked and clean

- `store-space-history` sqlite snapshot: work-producing encode/validate paths in `🚪️io/🪶️sqlite/📸️snapshot/…` do real work (`port.work`, `encode_effect`). `validate_context -> Ok(())` is the trait default for a context with nothing to validate. Not a stub.
- `store-operation-wire` close paths (`📤️edit`, `🧳️original`, `📦️output`): close steps call `retained_clone_grant_admits` and return `Progress(default)` only when unfunded.
- `store-one-item-publication` (`🎮️one-item/📬️publication/🪪️authority`): real `ControlledRetirement` ownership, no constant returns.
- Energy admission (`🎟️admission`) and close (`🔚️close`): real steps, work via `energy_effect_step`.

## Severity summary

| ID | Where | Kind | Severity |
|---|---|---|---|
| S1 | plugin-exchange `CommandText` arm | true stub, pre-existing | high (if pane sends text commands) |
| S2 | snapshot-clone recovery hooks | no-op placeholder, dead by contract | low |
| D1 | operation-wire replacement receiver `Pending => Ok(default)` | silent idle, dropped work | medium-high |
| D2 | tool-run `admit_sealed` surplus `let _ =` (and entities 24, 31) | dropped debit, under-charge | medium |
| D3 | tool-run tick native `Pending{work:0}` and finish `Ok(_)` | idle / unverified work | low-medium |
| D4 | energy idle and fixed-work helpers | freeze risk, heuristic work | medium |
| D5 | value encode retirement law `#[ignore]` | test-only skip | low / coverage |

## Summary

- True stubs: 2 (S1 plugin `CommandText` fault, S2 snapshot-clone recovery no-ops). Plus 5 silent-drop items (D1 to D5) that are not stubs but behave like them.
- Top risk: D1 and D2. A pending receive silently reports success with zero progress, and a surplus work debit is dropped. Both break "no pending or no-work success" rules, and both sit on pane store paths.
