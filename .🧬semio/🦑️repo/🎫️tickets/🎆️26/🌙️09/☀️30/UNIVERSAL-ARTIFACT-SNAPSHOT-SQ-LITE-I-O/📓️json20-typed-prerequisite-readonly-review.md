# JSON20 Typed Prerequisite Read-only Review

Inspected current mounted source and Root's existing Source logs, 2026-10-03. No source changes or Cargo execution in this lane.

## Preserved Baseline Defect

JSON SQL direct native input remains `NativeDecodeControl::new(limits.max_value_bytes,...)` at current line54 and contains no `allocation_stage`. The ValueError prerequisite port therefore preserves the actual semantic/native allocation defect for a genuine native assertion RED. Its five owner hooks now return ValueError. Binary input maps PackError with its typed enum method; text input retains TextError kind/message. Capability guards preserve causes through `IoError::from_value_error`; named-profile diagnostics remain successful IoOutcome diagnostics.

JSON pack root preflight/encode/reconstruct_record and pack encode project now return ValueError. Pack encode's callback converts typed project/refusal errors using `TextError::from_value_error(error,TextSpan::at(1,1))`; this preserves typed refusal identity, while the synthetic span denotes a nontext projection origin. No message classifier was found in these inspected ports.

## Mounted New Law and Neutral Fixture

The SQLite tests mount `💰️allocation/🦀️.rs` through an explicit child module, bringing `sqlite_snapshot_json` to20. New law exercises both logical Binary and Text: schema contains Unicode and NUL repeated10000, object retains duplicate Unicode/NUL keys, a nonempty string and an empty string. Its first call allows semantic SQL bytes1 independently of backing16MiB. It measures settled caller allocation, asserts at least schema-byte ownership, reruns with exact measured backing and checks zero remainder, then proves retirement does not reset cumulative admission by rejecting a second decode. Tiny1 and measured-minus-one backing refuse. Unexpected successful candidates are explicitly retired before panic; successful results retire after equality. The fixture is shallow, so assertion failure before explicit retirement does not expose deep recursive-drop hazards.

The new native refusal checks test failure and backing remainder but do not assert `ValueRefusalKind` for insufficient ownership. They establish allocation semantics once executed, not exact typed kind preservation for those branches. Existing Source fixture schema and independent Ajv checks reject missing allocation, extra carrier and conflated semantic allowance; Bun SQLite independently confirms exact Unicode/NUL schema byte count, duplicate ordered keys and empty string, integrity and foreign keys.

## Actual Source Receipt

`🗑️generated/root-json-owned-input-allocation-source-staging.log` failed its original Source attempt. The newer `root-json-owned-input-allocation-source-typed-prerequisites.log` reports15 pass,0 fail,1250 expect calls across3 files,1.64s Bun,10.8s uncached Nx, and successful registered target. Its Nx flaky annotation reflects prior task history and does not negate this executed result. This proves Source15 only; Native20 has no runtime verdict in this audit.

## CommonMark Current Typed Port

The mounted CommonMark SQL file now returns ValueError in all five hooks and private hook forwarding functions. Final reconstruct checkpoint still retains the complete snapshot in OwnedSnapshot until success, then takes ownership once. Subset guard uses UnsupportedOwner for wrong coordinate, typed closure for comparison/work, and `IoError::from_value_error` at terminal. The earlier final-cancellation retirement guard is preserved. No new semantic cause or successful-diagnostic loss concern was established in these inspected paths; complete native compiler/runtime authority remains Root's fresh run.

CommonMark pack controlled functions are likewise typed; authored SQL schema-byte refusal explicitly uses OwnershipLimit, and actual controlled envelope/pack errors preserve typed mappings. Ordinary unbounded `children` and `TryFrom<Snapshot>` still return String, but the controlled owner path has separate binding/reconstruction and does not establish their use at the five controlled hooks.

One precise error-taxonomy concern remains: CommonMark `add_rows` at pack line36 now emits OwnershipLimit when domain rows exceed max_rows. Shared `SqliteSnapshotControl::check_rows` and JSON's corresponding checked row growth classify this as WorkLimit. Allocation bytes and schema bytes do classify OwnershipLimit. This difference is assigned at the producer, not inferred from message text; current tests using only is_err do not establish matching row-refusal kind.

## Current Strengthening Follow-up

Root's subsequent mounted source corrects CommonMark `add_rows` to WorkLimit. Its preflight law asserts exact OwnershipLimit and keeps the original message check; deep duplicate-root law asserts InvalidValue and keeps its multiple-owner message check. JSON20 new allocation law now asserts exact OwnershipLimit for retired repeated decode, tiny backing and measured-minus-one backing, explicitly retiring any unexpected successful candidates. These revisions supersede the earlier source-strength/taxonomy concerns above. Root's parser-only receipt reports exit0; Native20 remains unexecuted here. Root's maintained staging Markdown is the execution/status authority.
