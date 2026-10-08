# Retirement Granted Port Patch Roster Audit 6

Read-only current repository scans, excluding ticket/generated trees by limiting to framework and S source roots. No source edits or runtime runs. Definitions include tests; string-generated or multiline impls require compiler confirmation. Distinguish Generic RetirementStep from SnapshotRetirementStep and separate ordered/flow enums.

## Actual Cursor Definitions

```text
✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:148:impl semio_framework_value::retirement::RetirementCursor for SharedTextRetirement {
✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:189:impl semio_framework_value::retirement::RetirementCursor for SharedChunksRetirement {
✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🫧️transient/🦀️.rs:240:impl semio_framework_value::retirement::RetirementCursor for SharedTryValuesRetirement {
🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs:169:impl semio_framework_value::retirement::RetirementCursor for ValueRetirement {
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs:439:impl semio_framework_value::retirement::RetirementCursor for DagOwnedSnapshotCursor {
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️mutation-fixtures-surface/🦀️.rs:1258:impl semio_framework_value::retirement::RetirementCursor for SurfaceMutationRetirement {
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs:298:impl semio_framework_value::retirement::RetirementCursor for EvaluationInputRetirement {
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs:365:impl semio_framework_value::retirement::RetirementCursor for FlowOwnedSnapshotCursor {
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🦀️.rs:159:impl RetirementCursor for CatalogRoot {
✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/♻️retirement/🦀️.rs:175:impl RetirementCursor for SnapshotRetirement {
🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1006:impl<T:semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetirementCursor for JsonProjectionIteratorRetirement<T> {
🧰️framework/🔨️modules/🕸️graph/🛂️manifest/♻️retirement/🦀️.rs:7:impl RetirementCursor for PropertyRetirement {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:49:impl<T: Copy + Send + 'static> RetirementCursor for Leaf<T> {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:110:impl RetirementCursor for Bytes {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:148:impl<T: RetireOwned> RetirementCursor for Collection<T> {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:196:impl<T: RetireOwned> RetirementCursor for DequeCollection<T> {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:221:impl<T:RetireOwned> RetirementCursor for VectorIterator<T> {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:229:impl<T: RetireOwned> RetirementCursor for UnorderedSet<T> {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:241:impl<K: RetireOwned,V: RetireOwned> RetirementCursor for UnorderedMap<K,V> {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:252:impl<T: RetireOwned + Ord> RetirementCursor for OrderedSet<T> {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:273:impl<K: RetireOwned + Ord, V: RetireOwned> RetirementCursor for OrderedMap<K, V> {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:303:impl<T: RetireOwned> RetirementCursor for BoxedOwner<T> {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:353:impl RetirementCursor for Sequence {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:374:impl<T: RetireOwned> RetirementCursor for Deferred<T> {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:394:impl RetirementCursor for ValueRetirement {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:542:impl RetirementCursor for ErasedCursor {
🧰️framework/🔨️modules/🌱️value/♻️retirement/🧪️tests/🔬️unit/🦀️.rs:591:    impl RetirementCursor for Hostile {
🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🦀️.rs:19:impl<T: RetireOwned, const N: usize> RetirementCursor for PagedListRetirement<T, N> {
🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:26:impl RetirementCursor for DropProbeRetirement {
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs:681:impl semio_framework_value::retirement::RetirementCursor for ZipRetirementCursor {
```

## Exact Generic Child Consumer Arms

```text
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:464:                RetirementStep::Child(child) => self.0.push(child),
🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🦀️.rs:82:            RetirementStep::Child(child) => { self.cursors.push_reserved(child).map_err(|_| refusal("retirement child lost its admitted frontier slot"))?; (0, 0, true) }
✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🧪️tests/🔬️unit/🦀️.rs:13:            semio_framework_value::retirement::RetirementStep::Child(child) => stack.push(child),
```

## Trait Declaration

```text
🧰️framework/🔨️modules/🌱️value/♻️retirement/🦀️.rs:21:    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep;
```

Core patches: change RetirementCursor trait to the full five-dimension grant, migrate all listed close implementations, both exhaustive consumers (CursorStack and ControlledRetirement) and any direct typed callers. Propagate Failure(ValueError) exactly; preserve owner/frontier state on failure. Value derive-generated cursors and RetireOwned factory returns also reference this trait but are not themselves necessarily implementations; inspect generated quote bodies rather than modifying unrelated factories.

Concurrent native paths: Infinite DAG retained/🦀️.rs439, Flow artifact vcs/🦀️.rs365 and Neural engine retirement/🦀️.rs169. Native agent was notified. DAG currently maps Err to BudgetExhausted and clamps receipt bytes with min; Flow uses maximum_bytes.max(demand), clamps released bytes back with min and swallows errors. These are actual admission violations to remove, not behavior to preserve in a compatibility bridge. Full grant direct ports must refuse undergrant and report real release/error. Other concrete wrappers include Flow wasm evaluation input, Forms shared text/chunks/try-values, Block catalog/snapshot, Zip preparation, Generic graph properties, Generic Pack JSON iterator and Generic Value paged-list.

Definition signature migration alone is insufficient. CursorStack old aggregate byte budget must explicitly assign caller copy/capacity/release/depth dimensions and validate returned categories; ControlledRetirement must pass only caller-authorized remaining depth to its inner cursor, not mint unlimited capacity or release. Existing collection/leaf copy loops use body bytes, while Box/backing releases use physical release bytes and child births use capacity bytes. Enforce maximum_items zero before any cursor mutation.

Nested ControlledRetirement cursor can now forward an actual full grant and separate copy/release categories without a hidden credit adapter. One-transition capacity-only progress must be represented explicitly and charged once; typed constructor failure cannot become a zero-progress retry. Root/inner frontier Box/page allocations and their terminal releases remain distinct. Parent depth must include nested retained frontier membership under the declared depth policy.

This roster is source evidence, not successful global compilation. Compile diagnostics remain the authority for alias/multiline/generated definitions and direct consumers outside these literal matches. No legacy byte-only overload or default bridge should survive the canonical cut.
