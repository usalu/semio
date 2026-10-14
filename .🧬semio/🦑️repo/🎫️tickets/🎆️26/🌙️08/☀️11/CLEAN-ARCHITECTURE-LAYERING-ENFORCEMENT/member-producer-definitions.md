# Actual Member Producer Definitions

Moved original factory method bodies from snapshot implementations to selected actual producer classes. This is an in-progress source roster; no compile/acceptance.

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs`: `DagSnapshot` / `DagMutation` now owns actual producer `DagMemberStoreProducer`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️replay-retirement/🦀️.rs`: `DemoSnapshot` / `FailClosedOp` now owns actual producer `crate::os_store::tests::DemoMemberStoreProducer<FailClosedOp>`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️deferred-reprojection/🦀️.rs`: `DemoSnapshot` / `CountedOp` now owns actual producer `crate::os_store::tests::DemoMemberStoreProducer<CountedOp>`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`: `DemoSnapshot` / `DemoMutation` now owns actual producer `crate::os_store::tests::DemoMemberStoreProducer<DemoMutation>`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`: `DemoSnapshot` / `SeverityMutation` now owns actual producer `crate::os_store::tests::DemoMemberStoreProducer<SeverityMutation>`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`: `DemoSnapshot` / `ValidatedMutation` now owns actual producer `crate::os_store::tests::DemoMemberStoreProducer<ValidatedMutation>`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`: `DemoSnapshot` / `TimestampedMutation` now owns actual producer `crate::os_store::tests::DemoMemberStoreProducer<TimestampedMutation>`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-law/🦀️.rs`: `DemoSnapshot` / `LawOp` now owns actual producer `crate::os_store::tests::DemoMemberStoreProducer<LawOp>`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🦀️.rs`: `$snapshot` / `$mutation` now owns actual producer `$crate::SemioMemberStoreProducer<$snapshot,$mutation>`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`: `HashProjection` / `HashMutation` now owns actual producer `HashMemberStoreProducer`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs`: `FlowHostSnapshot` / `FlowMutation` now owns actual producer `crate::io::binary::snapshot::FlowSnapshotPackProducer`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs`: `DummySnapshot` / `DummyMutation` now owns actual producer `DummyMemberStoreProducer`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`: `TestSnapshot` / `TestMutation` now owns actual producer `TestMemberStoreProducer`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`: `$snapshot` / `RecursiveFixtureMutation` now owns actual producer `RecursiveMemberStoreProducer<$snapshot>`.
