# Framework instance/job callers of decl-app APIs

## Migrated
- `ArtifactOwnedContextHandle::admit(..,grant,cx)`: 🧵️instance typed-command and actions pass `cx`. Snapshot `try_duplicate()` calls there are another type and untouched.
- `ReservedJobOutcomes::close_step(effect)`: 👷️job `🏛️reserved` (2 sites), `📤️outcomes` (`retire` now uses `close_original_retained_effect_step`), `🛬️lifted/🔄️full-operation`, `📜️declaration/📥️natural-file` (its local `retired` is now effect-shaped), fixtures `agent-lane-preview` and `clipboard-and-retained-command-fixtures`.
- `ArtifactMediaExportRequestRetirement::close_step(effect)`: `📄️docs/📥️staging/🧵️instance-root` via `close_original_effect_step`.
- `🌱️capture:28 advance_original_index`: now `close_original_retained_effect_step(cx,d,|effect|registry.advance_original_index(effect))`; unused third birth unit removed.
- decl-app `📸️fields` test: `drain` runs through `EffectBank` (new dev-dependency `semio-framework-value-testing-admission`); `drain(window)` became `drain(ContextViewLease::new(window))`. decl-app `--tests` no longer fails.
- `retired` (effect closure) re-exporters: `👷️job/🦀️.rs` also re-exports `retired_poll`; `🏛️reserved/📌️checkpoint` and `📨️emission/🎟️capture` migrated to `retired_poll(step_granted)` and effect closures.
- `🪆️child-vcs` (main, `📌️checkpoint`, `🪆️cascade`): `NativeAllocationIdentity` decoder, `funded`/`replied` for owner `advance_into`/`take`/`take_authority_into`, `close_into`, admission effects. `check -p semio-plugin-job-child-vcs --lib` passes (0 errors).

## Round 2
- host-configuration: local helpers replaced by `semio-plugin-job-receiving::original_steps` (new dependency); parser phase runs through `with_native_receiving` + `NativeDecodeControl::with_effect` + `receive_original_receiving_work`; encode-continuation tuple errors keep the saved continuation; close via `step_granted`/`begin_granted(effect)`. Lib compiles.
- retained-work: dependency key renamed to `semio-os-kernel` (code uses `semio_os_kernel`). Compiles.
- child: `drive` takes an effect; metadata `step(body,effect)`. Compiles. 🧵️instance docs `ingress` caller of `drive` and `advance_snapshot_read_returns_one(grant)` plus the 🪜️maintenance-ladder and 🔒️close-ladder callers are still grant-shaped (not yet migrated).
- peer: entry custody via new `PeerPresenceEntryLease` (`ArcLeaseRetirement`) in `ControlledRetirement`; rejected-pack close through `NativeDecodeControl::with_effect`; typed publication close handles `RetirementPollExit`. Lib compiles; its test module still uses `SharedControlledRetirement::new(Arc)` and grant `step`.
- job: 🏛️registration moved from `Arc` to `SharedOriginalStorage` (`from_cold`, `alias`, `physically_unique`), `RetireOwned` seed plus inherent `close_step(body,effect)` cursor with `native_retirement_reply!`; `QualifiedToolProof` cursor wrapper; 🏛️reserved/checkpoint/capture decoder identities, effect steps for typed owners and lease close. `check -p semio-plugin-job --lib` passes.

## Remaining (full `check --all-targets --keep-going`)
composition-publication (46), document-media (6), tool-run (56), tool-machine (38, other agent). The 🧵️instance crates are blocked behind these and have not been checked.
