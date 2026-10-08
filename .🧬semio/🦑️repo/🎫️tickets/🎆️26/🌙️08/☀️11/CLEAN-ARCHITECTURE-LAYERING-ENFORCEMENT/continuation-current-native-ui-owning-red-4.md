# GeneralUI Owning Red4

Actual Nx/Bun1 Cargo101, 1219 sources exact=true, producer exact=true; original all targets and controls unchanged.

test wgpu::host::physical_job_close_tests::original_clipboard_backing_refusal_cancel_and_terminal_receipts_are_physical ... ok
test wgpu::prepared::tests::cancellation_retires_large_upload_incrementally_before_terminal_empty ... FAILED
test wgpu::prepared::tests::draw_item_cap_faults_before_packet_publication ... FAILED
test wgpu::prepared::tests::engine_tests::original_zero_index_geometry_submits_without_fabricated_surface_or_missing_mesh ... FAILED
test wgpu::prepared::tests::eviction_byte_cap_faults_before_packet_publication ... FAILED
test wgpu::prepared::tests::preparation_completes_across_bounded_steps ... FAILED
test wgpu::prepared::tests::physical_job_close_laws::prepared_original_owner_closes_through_the_actual_job_full_grant_contract ... ok
test wgpu::prepared::tests::prepared_render_common_close_refuses_undeclared_frontiers_without_touching_original_owners ... FAILED
test wgpu::prepared::tests::retained_codec_source_moves_once_and_retires_one_page_per_governed_step ... FAILED
test wgpu::prepared::tests::upload_byte_cap_faults_before_packet_publication ... FAILED
test wgpu::prepared::tests::zero_fuel_and_expired_deadline_advance_no_raster_page_or_allocation ... FAILED
test result: FAILED. 782 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.36s

        node_len: usize,
    |         ^^^^^^^^
334 |         byte_len: usize,
    |         ^^^^^^^^
335 |         root: Option<u16>,
    |         ^^^^
336 |         receipt: Option<ActionQueueReceipt>,
    |         ^^^^^^^
    |
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: function `hit_test_subtree` is never used
   --> 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:273:15
    |
273 | pub(crate) fn hit_test_subtree(tree: &UiTree, subtree_root: NodeId, x: f32, y: f32) -> Option<NodeId> {
    |               ^^^^^^^^^^^^^^^^

warning: `semio-framework-ui` (lib test) generated 162 warnings (run `cargo fix --lib -p semio-framework-ui --tests` to apply 153 suggestions)
    Finished `test` profile [unoptimized] target(s) in 26.29s
warning: the following packages contain code that will be rejected by a future version of Rust: naga v29.0.4, wgpu v29.0.4, wgpu-core v29.0.4, wgpu-hal v29.0.4, winit v0.30.13
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
     Running unittests 🦀️.rs (.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/current-native-canonical-6/target/debug/build/semio-framework-ui/8d5e0e87e4e81833/out/semio_framework_ui-8d5e0e87e4e81833)
[DEBUG] authored mip upload revision=1 slices=17 residentBytes=0 terminal=true 
[DEBUG] authored mip upload revision=2 slices=23 residentBytes=0 terminal=true 

thread 'wgpu::prepared::tests::cancellation_retires_large_upload_incrementally_before_terminal_empty' (8996795) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:679:5:
assertion failed: job.terminal_is_empty()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread 'wgpu::prepared::tests::draw_item_cap_faults_before_packet_publication' (8996797) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:925:5:
assertion failed: job.terminal_is_empty()
[DEBUG] native GPU created authored pipelines with five texture roles and four alpha/culling combinations

thread 'wgpu::prepared::tests::engine_tests::original_zero_index_geometry_submits_without_fabricated_surface_or_missing_mesh' (8996801) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/../🔬️targets-wgpu-prepared-engine-unit/🦀️.rs:101:13:
assertion failed: job.terminal_is_empty()

thread 'wgpu::prepared::tests::eviction_byte_cap_faults_before_packet_publication' (8996804) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:910:5:
assertion failed: job.terminal_is_empty()

thread 'wgpu::prepared::tests::preparation_completes_across_bounded_steps' (8996865) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:747:5:
assertion failed: job.terminal_is_empty()

thread 'wgpu::prepared::tests::prepared_render_common_close_refuses_undeclared_frontiers_without_touching_original_owners' (8996873) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:1119:9:
assertion `left == right` failed
  left: Pending { progress: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: 0 } }
 right: Refused(UnsupportedOwner)

thread 'wgpu::prepared::tests::retained_codec_source_moves_once_and_retires_one_page_per_governed_step' (8996883) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:410:5:
assertion failed: job.terminal_is_empty()

thread 'wgpu::prepared::tests::upload_byte_cap_faults_before_packet_publication' (8996887) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:896:5:
assertion failed: job.terminal_is_empty()

thread 'wgpu::prepared::tests::worker_panic_hands_back_the_exact_job_and_mailbox_owners' (8996888) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:952:9:
hostile worker interruption

thread 'wgpu::prepared::tests::zero_fuel_and_expired_deadline_advance_no_raster_page_or_allocation' (8996889) panicked at 🔨️modules/🖱️ui/📦️packages/🦀️rust/../../🎯️targets/🧊️wgpu/🎟️prepared/../../../🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:662:5:
assertion failed: job.terminal_is_empty()
error: test failed, to rerun pass `-p semio-framework-ui --lib`
error: 1 target failed:
    `-p semio-framework-ui --lib`

