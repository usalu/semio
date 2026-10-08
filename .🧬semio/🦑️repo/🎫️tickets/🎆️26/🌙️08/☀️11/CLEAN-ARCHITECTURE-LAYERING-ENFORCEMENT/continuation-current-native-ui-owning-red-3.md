# GeneralUI Owning Red3

Actual Nx/Bun1, Cargo=101, reason=exit. 1219 selected sources; exact=false; producer exact=true. No whole acceptance.

Original unfiltered output:

test wgpu::flex::tests::the_flex_tree_releases_exactly_one_node_per_close_grant ... FAILED
test wgpu::host::physical_job_close_tests::original_clipboard_backing_refusal_cancel_and_terminal_receipts_are_physical ... ok
test wgpu::mounted_layout::tests::mounted_layout_deadline_and_partial_close_each_advance_at_most_one_owner ... FAILED
test wgpu::prepared::tests::engine_tests::original_zero_index_geometry_submits_without_fabricated_surface_or_missing_mesh ... FAILED
test wgpu::prepared::tests::physical_job_close_laws::prepared_original_owner_closes_through_the_actual_job_full_grant_contract ... FAILED
    wgpu::prepared::tests::physical_job_close_laws::prepared_original_owner_closes_through_the_actual_job_full_grant_contract
test result: FAILED. 787 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.62s

er {
     | --------------------------- methods in this implementation
1218 |     fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
1233 |     fn close_granted(&mut self, grant: RetainedCloneGrant) -> CloseStep {
     |        ^^^^^^^^^^^^^

warning: methods `next_close_release_byte_demand` and `close_granted` are never used
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1637:8
     |
1636 | impl PreparedRenderCommandPages {
     | ------------------------------- methods in this implementation
1637 |     fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
1641 |     fn close_granted(&mut self, grant: RetainedCloneGrant) -> CloseStep {
     |        ^^^^^^^^^^^^^

warning: multiple fields are never read
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎬️action/../../../🧪️tests/🔬️targets-wgpu-action-unit/🦀️.rs:329:9
    |
328 |     struct BeforeBatchMarker {
    |            ----------------- fields in this struct
329 |         controller_id: TextSpan,
    |         ^^^^^^^^^^^^^
330 |         action: TextSpan,
    |         ^^^^^^
331 |         nodes: Box<[Option<FlatNode>; ACTION_NODE_CAPACITY]>,
    |         ^^^^^
332 |         bytes: Box<[u8; ACTION_ITEM_BYTE_CAPACITY]>,
    |         ^^^^^
333 |         node_len: usize,
    |         ^^^^^^^^
334 |         byte_len: usize,
    |         ^^^^^^^^
335 |         root: Option<u16>,
    |         ^^^^
336 |         receipt: Option<ActionQueueReceipt>,
    |         ^^^^^^^

warning: function `hit_test_subtree` is never used
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:273:15
    |
273 | pub(crate) fn hit_test_subtree(tree: &UiTree, subtree_root: NodeId, x: f32, y: f32) -> Option<NodeId> {
    |               ^^^^^^^^^^^^^^^^

warning: `semio-framework-ui` (lib test) generated 174 warnings (run `cargo fix --lib -p semio-framework-ui --tests` to apply 155 suggestions)
    Finished `test` profile [unoptimized] target(s) in 28.45s
warning: the following packages contain code that will be rejected by a future version of Rust: naga v29.0.4, wgpu v29.0.4, wgpu-core v29.0.4, wgpu-hal v29.0.4, winit v0.30.13
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
     Running unittests 🦀️.rs (.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-canonical-6/target/debug/build/semio-framework-ui/8d5e0e87e4e81833/out/semio_framework_ui-8d5e0e87e4e81833)
[DEBUG] authored mip upload revision=1 slices=17 residentBytes=0 terminal=true 
[DEBUG] authored mip upload revision=2 slices=23 residentBytes=0 terminal=true 
[DEBUG] native GPU created authored pipelines with five texture roles and four alpha/culling combinations

thread 'wgpu::flex::tests::the_flex_tree_releases_exactly_one_node_per_close_grant' (8974249) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📐️flex/../../../🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs:484:5:
an already-empty tree reports terminal without releasing anything
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'wgpu::mounted_layout::tests::mounted_layout_deadline_and_partial_close_each_advance_at_most_one_owner' (8974328) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/📌️mounted_layout/../../../🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs:236:5:
assertion `left == right` failed
  left: 0
 right: 1

thread 'wgpu::prepared::tests::engine_tests::original_zero_index_geometry_submits_without_fabricated_surface_or_missing_mesh' (8974452) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../🔬️targets-wgpu-prepared-engine-unit/🦀️.rs:59:13:
zeroVertexWire: original GPU upload failed after exact owner closure: Some("mesh upload schema was empty")

thread 'wgpu::prepared::tests::physical_job_close_laws::prepared_original_owner_closes_through_the_actual_job_full_grant_contract' (8974515) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:20:83:
called `Result::unwrap()` on an `Err` value: ValueError { kind: UnsupportedOwner, message: "interactive job retained close demand is undeclared" }

thread 'wgpu::prepared::tests::worker_panic_hands_back_the_exact_job_and_mailbox_owners' (8974541) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:952:9:
hostile worker interruption
error: test failed, to rerun pass `-p semio-framework-ui --lib`
error: 1 target failed:
    `-p semio-framework-ui --lib`

