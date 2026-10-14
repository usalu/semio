# Remaining Original Member Producer Ownership

Read-only continuation after the settled explicit constructor/genesis/reset frontier. This report is no source or runtime qualification. No writes to held Store/media production input were performed after releasing the bounded source frontier.

The current snapshot-owned MemberStoreOwner trait requires Self:Clone+ToValue+FromValue and an associated SnapshotOpen whose Snapshot=Self. Its two original methods quote source birth and admit the actual DocumentStoreOwners. InitialMemberStoreOpen<F,P,M> still stores P::SnapshotOpen and invokes P::member_store_owners_birth_demand / P::member_store_owners at the original request/context/SourceLoan checks. It also statically requires snapshot ArtifactPack/ArtifactStoreSnapshot. Moving FlowHostSnapshot to a parent owner leaves the child unable to implement this foreign trait on that foreign type.

The next real source step must make the exact existing owner factory an explicit type capability parameter of initial member opening, not introduce a forwarding wrapper around P. Its retained snapshot operation, source-demand and source-admission methods must be defined on the child factory. Original request custody, native body extent size_of<InitialMemberStoreOpen<F,P,M,C>>, snapshot buffer/decode demand, original caller minimum, source loan validation and complete paid retirement ladder must all remain actual. Every authored factory declaration and every real typed consumer must select C; no generic default or compatibility owner trait should remain.

The Flow factory's current original source birth adds DocumentStoreOwners::source_birth_bytes<FlowSnapshotRetirementFactory,FlowOwnedHostSnapshotRetirementFactory,FlowMutationRetirementFactory,ArtifactStoreCursorDisposer<FlowHostSnapshot,FlowMutation>> to FlowAuthoringFactory's exact Arc birth extent. Actual admission uses admit_source_constructor_with_one_item_preparation, unchanged original retirement factories and actual authoring factory. FlowSnapshotPackProducer already defines actual Genesis/member snapshot decoder/publication; its child owner declaration should retain these original catalog capabilities rather than move them upstream.

Fourteen lexical impl declarations remain, including two authored macro declarations that instantiate additional real types. They are not fourteen runtime owner instances and are not counted as quota/source witnesses. The fixture wrapper generics and actual macros also name P::member_store_owners; those consumers must be canonicalized together with the defining trait and retained operation. A new neutral snapshot package alone is not deletion proof while these or the parent concrete child tests/Cargo dependencies remain.

Original declarations observed:

```text
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:181:impl MemberStoreOwner<FlowMutation> for FlowHostSnapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs:973:        impl store::os_store::document_store_owners::MemberStoreOwner<RecursiveFixtureMutation> for $snapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-dummy/🦀️.rs:506:impl store::os_store::document_store_owners::MemberStoreOwner<DummyMutation> for DummySnapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:5019:    impl store::os_store::document_store_owners::MemberStoreOwner<TestMutation> for TestSnapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:1831:impl store::os_store::document_store_owners::MemberStoreOwner<HashMutation> for HashProjection {
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs:26:impl MemberStoreOwner<DagMutation> for DagSnapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️replay-retirement/🦀️.rs:223:impl MemberStoreOwner<FailClosedOp> for DemoSnapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️deferred-reprojection/🦀️.rs:330:impl MemberStoreOwner<CountedOp> for DemoSnapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:2285:impl MemberStoreOwner<DemoMutation> for DemoSnapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:8631:impl MemberStoreOwner<SeverityMutation> for DemoSnapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:10021:impl MemberStoreOwner<ValidatedMutation> for DemoSnapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:10033:impl MemberStoreOwner<TimestampedMutation> for DemoSnapshot {
🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-law/🦀️.rs:93:impl MemberStoreOwner<LawOp> for DemoSnapshot {
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🦀️.rs:111:        impl ::semio_framework_os_kernel::os_store::document_store_owners::MemberStoreOwner<$mutation> for $snapshot {
```
