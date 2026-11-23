# Bearbeiten `original-turn.native-return-refused: semantic` (demo-2326)

Status: guest check NOT yet identified; one known cause ruled out; the diagnostic is in source and needs one rebuild. No product fix made (a fix without the cause would be a guess).

## Evidence

- `🗑️generated/pw-demo/*bearbeiten-boot-en/error-context.md`: the last guest prints before the fault are `[DEBUG] bundle reported 0 observed 10 coord 0`, then `[DEBUG] semantic semio_framework_job_failure::component::OriginalTurnFailure`, then `setContributions command failed demonstrator original-turn.native-return-refused: semantic`. All earlier `bundle` lines are `reported 0 observed N coord 1` (accepted).
- The `semantic` print is `dispatch_value_step` (`poll_wit` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs:87`, helper in `🌉️call/🦀️.rs:74`). That helper printed only the Rust type name, so the failing cause text was lost. The whole `wit_bridge::poll` turn is inside it, so any `OriginalTurnFailure` of the turn lands here.

## Ruled out: the work-report law (`original_one_item_turn_work`)

The staged demonstrator module (built 23:26) is newer than `🏪️store/🎮️one-item/🦀️.rs` (law, 19 Nov) and `⚙️engine/🧳️bundle/🦀️.rs` (22:08). The law returns `Ok(observed)` when `reported==0` (`reported!=0 && observed>reported` is the only refusal), so `reported 0 observed 10` cannot be the refusal that was fixed earlier. Coordination 0 means the advance ended `Blocked`, `AwaitingAck`, `Complete`, or `Err(_)` (`coordination` maps `Err` to 0). With 10 native units spent and no ownership progress, the likely shapes are the PreflightingCommit digest step (`owner.advance_digest`, returns `Blocked` when `ownership==Default`) or an `Err(VcsError)` in PreparingCursor/Preflighting whose message is dropped by `.map_err(|error| error.to_string())` in `🧩️space-member/.../🏪️space-member-for-artifact-store/🦀️.rs:128`. Not distinguishable from the log.

## Diagnostics added (temporary, `[DEBUG]` prefix)

1. `🏪️store/⚙️engine/🧳️bundle/🦀️.rs` `advance_apply_batch`: the existing `bundle reported` print now also shows `err {:?}` (the `VcsError` message of the inner advance).
2. `🌉️call/🦀️.rs:74` and `:79` (`dispatch_value_step`, `dispatch_cleanup_value_step`): print `dbg_cause(&error)` (primary, receipt, context and work refusal messages) instead of the type name.

## Next step

Rebuild `demonstrator` (links `semio-os-store-engine-bundle` and the reactor call crate) and rerun `bearbeiten boot`. The new `[DEBUG] semantic turn <message> | receipt=.. ctx=.. work=..` line names the check, and the `err` field of the last `bundle` line says whether it was a store `Err`. Then fix at that site with a focused law test; remove both diagnostics afterwards.
