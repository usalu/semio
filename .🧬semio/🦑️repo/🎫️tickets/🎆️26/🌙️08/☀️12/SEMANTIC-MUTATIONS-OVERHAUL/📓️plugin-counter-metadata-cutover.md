# Plugin Counter Metadata Cutover

This bounded group moves only the testkit dummy, transaction, and surface count mutations from manual `DslOps` aggregates to five direct `MutationLeaf` sources plus three transparent `Mutations` aggregates under `🧪️tests/🧬️mutation-fixtures`.

The notification operation remains its own leaf and preserves its foreign step. The no-preflight operation remains its own leaf and retains the deliberate panic if a caller violates the `may_emit_foreign_steps == false` contract. All leaves carry direct descriptor JSON, payload schema, text, binary, semantic, diff, and inverse behavior.

The remaining E0046 adoption owners are unchanged: macro-generated channel fixture, contribution wire fixture, TestApp document/config fixtures, and children fixture. The builder correction is now resolved: `🏗️builder/🦀️component.rs:769` owns `DependencyTestOp::Add`, an ordinary manual test operation passed to `ArtifactContribution::mutation` with `DependencyTestMutationKind`; it is a separate future adoption owner, not `component.rs:769` and not part of this group.

No Rust compiler or Cargo command has run for this packet; runtime remains pending root serialization.

## Late Fixture Audit

The next packet's initial first-five source fingerprint is retained at `🧪️plugin-counter-metadata/🔐️source-before-next.sha256`. `TestMutation` has independent count and label leaves. `TestConfigMutation` must become a named selection change with a nested optional diff (`Option<Option<String>>`): `None` is identity and `Some(None)` is the genuine clear-selection change; a snapshot pseudo-operation would conflate those states. `ChildrenTestMutation` is now an uninhabited aggregate with an empty descriptor roster and total text/binary rejection. `🏗️builder/🦀️component.rs:769` is not a simple set operation: `DependencyTestOp::Add` drives `DependencyTestMutationKind` composite contribution planning and therefore remains a separate direct-leaf/composite adoption.
