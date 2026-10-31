# Native Fixture API Review

Read-only preflight, 2026-10-29. No source edits, builds, tests, native requests, cache operations or active snapshot changes. Native84839 remains with its existing controller. Read native-layout-typing-selection-review.md, native-fold-selection-review.md, host-native-doors.md, typing-original.md and host-policy.md; inspected actual selected fixtures and defining APIs.

## Concrete Finding: Writer Editor Gate Compiles An Unconverted ActorId

`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️unit/🦀️.rs:236` constructs `protocol::ActorId(text(&value["actor"]))`. The local text closure at line 230 explicitly returns String using to_string(). The actual protocol ActorId tuple field is SharedUtf8, defined at `🧰️framework/🔨️modules/📡️replication/🆔️ids/🦀️.rs:23`. A tuple constructor does not apply Into implicitly. This is the same concrete String-versus-SharedUtf8 compile mismatch as the preceding companion failure.

This source is compiled by the required `semio-s-artifact-writer-writer-tests-2 --lib writer_editor_forwards_its_canonical_original_typing_factory` gate even though the runtime selector names another test. Its library mounts 🪞️mirror/🦀️.rs; that mirror mounts `host::owned` from the actual owned source, whose cfg(test) module mounts this host unit-test file. Function-name filtering happens after compilation, so it cannot avoid this mismatch.

Precise correction for the assigned owner: explicitly convert only this ActorId payload to its actual SharedUtf8 type. Do not change the shared text closure globally: neighboring ArtifactId/MutationId/SchemaId constructors intentionally receive String. No correction was made by this reviewer.

## Other Reviewed Boundaries

No additional clear instance of the two requested API failures was established in these inspected selected fixtures:

- Layout selected 15-library plan and nineteen-test transform root/mounted typing child: current ActorId uses found in the reachable common app fixture and Layout behavior cases convert through .into(); the transform press-settlement law uses OriginalPressSettlement/OriginalScrubLedgerClose rather than obsolete cold ScrubLedger.send calls. Its production ToolMachineRunner.send is a different valid API and must not be mechanically rewritten.
- Standalone Writer fold two laws use canonical WriterMutation/SpliceText and OriginalJobFixtureCaller/decoder interfaces. The real editor forward law uses the canonical factory; its additional compile closure contains the concrete host finding above.
- instance-typed-ops original typing two laws: fixture seed uses TypingLedger.send, whose actual signature still takes owned TypingInput; ActorId is already converted. TypingState still declares states, net, transaction and deadline_ms used by these tests. It is not ScrubLedger.send and should not be changed to the scrub Option receiver shape.
- original-job-ownership two laws: inspected defining owner mounts and included fixture sources; no ActorId construction or ScrubLedger.send mismatch found.
- Store bin02 checkpoint law: inspected current source-frame quote/birth, selected-preview/read calls and current receiving fixture path. No ActorId/String or cold scrub-send use exists in this selected law.
- Generic host-configuration fixture plus Process/CAD/Sourcing focused original_host_configuration targets: arguments use DslValue::String(String), consistent with current intrinsic encoding/value constructors, and the testkit signature takes the declared static policy and owned Option<DslValue>. No ActorId construction or scrub-send call occurs in those focused host tests.

An initial repository search also found an old two-argument new_app_with_registry_and_members call in Layout intrinsic test source. It was not reported as a selected compile defect: Layout editor unit root currently mounts only context.rs, and no selected module mounting that intrinsic file was established. Likewise unrelated database fixture ActorId hits were excluded. This keeps the finding tied to a demonstrated required gate rather than every stale file in the repository.

## Limits

This preflight checks concrete constructor/signature drift and selected source reachability; it is not a Rust typechecker or a pass of any native target. No claim is made that every transitive dependency or all fifteen Layout source closures compile. Native execution remains necessary after the single identified Writer fixture correction and the ongoing shared native lane settle. The review did not alter the pure/core/composer/companion snapshots.

## Root Repair

The exact selected Writer fixture now moves its existing String into canonical SharedUtf8 with .into() at the ActorId boundary. SharedUtf8 implements From<String>; the sibling String fields and every assertion remain unchanged. No production or current framework native snapshot source changed. The future Writer native gate must still execute.
