# GeneralUI Owning Red5

Actual Nx/Bun1 Cargo101, 1222 sources exact=true, producer exact=true. Original all-target controls unchanged; no whole closure.

test wgpu::host::physical_job_close_tests::original_clipboard_backing_refusal_cancel_and_terminal_receipts_are_physical ... ok
test wgpu::prepared::tests::pending_presenter_witness_rejects_superseding_packet_with_exact_owner ... FAILED
test wgpu::prepared::tests::prepared_render_common_close_funds_declared_frontiers_without_losing_original_owners ... FAILED
test wgpu::prepared::tests::physical_job_close_laws::prepared_original_atlas_owner_preserves_denials_and_reports_every_physical_release ... FAILED
test wgpu::prepared::tests::physical_job_close_laws::prepared_original_owner_closes_through_the_actual_job_full_grant_contract ... ok
    wgpu::prepared::tests::physical_job_close_laws::prepared_original_atlas_owner_preserves_denials_and_reports_every_physical_release
test result: FAILED. 789 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 8.44s

retained-hit-targets/🦀️.rs:456:17
    |
456 |                 single_resolved += 1;
    |                 ^^^^^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: associated constant `RETIRE_PAGE_BYTES` is never used
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:1819:11
     |
1747 | impl PreparedRenderPacket {
     | ------------------------- associated constant in this implementation
...
1819 |     const RETIRE_PAGE_BYTES: usize = 16 * 1024;
     |           ^^^^^^^^^^^^^^^^^
     |
     = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: method `close_step` is never used
    --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:2432:8
     |
2331 | impl PreparedRenderReceiver {
     | --------------------------- method in this implementation
...
2432 |     fn close_step(&self) -> bool {
     |        ^^^^^^^^^^

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

warning: `semio-framework-ui` (lib test) generated 164 warnings (run `cargo fix --lib -p semio-framework-ui --tests` to apply 153 suggestions)
    Finished `test` profile [unoptimized] target(s) in 23.47s
warning: the following packages contain code that will be rejected by a future version of Rust: naga v29.0.4, wgpu v29.0.4, wgpu-core v29.0.4, wgpu-hal v29.0.4, winit v0.30.13
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
     Running unittests 🦀️.rs (.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-canonical-6/target/debug/build/semio-framework-ui/8d5e0e87e4e81833/out/semio_framework_ui-8d5e0e87e4e81833)
[DEBUG] authored mip upload revision=1 slices=17 residentBytes=0 terminal=true 
[DEBUG] authored mip upload revision=2 slices=23 residentBytes=0 terminal=true 
[DEBUG] native GPU created authored pipelines with five texture roles and four alpha/culling combinations
[DEBUG] originalZeroIndexGpu actualDevice=true uploaded=4 submittedPaths=24 residentPaths=12 absentVersionPaths=12 readbackTinySkia=true noFabricatedTriangles=true exactResidencyMarkers=true exactOwnersClosed=true

thread 'wgpu::prepared::tests::pending_presenter_witness_rejects_superseding_packet_with_exact_owner' (9032996) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:890:5:
assertion failed: matches!(returned.uploads.get(0),
    Some(PreparedRenderUpload::GlyphAtlas { pixels, .. }) if pixels.len() ==
    1)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'wgpu::prepared::tests::prepared_render_common_close_funds_declared_frontiers_without_losing_original_owners' (9033007) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:1153:9:
assertion `left == right` failed
  left: false
 right: true

thread 'wgpu::prepared::tests::physical_job_close_laws::prepared_original_atlas_owner_preserves_denials_and_reports_every_physical_release' (9032997) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../♻️physical-job-close/🎟️prepared/🦀️.rs:86:9:
assertion `left == right` failed
  left: 121968
 right: 121928

thread 'wgpu::prepared::tests::worker_panic_hands_back_the_exact_job_and_mailbox_owners' (9033023) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:975:9:
hostile worker interruption
error: test failed, to rerun pass `-p semio-framework-ui --lib`
error: 1 target failed:
    `-p semio-framework-ui --lib`

