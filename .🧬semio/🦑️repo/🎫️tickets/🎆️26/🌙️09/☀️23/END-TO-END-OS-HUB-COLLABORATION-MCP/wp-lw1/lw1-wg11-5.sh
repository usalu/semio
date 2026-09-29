#!/bin/zsh
# ⚖️ LW1 → renderer-wgpu full lib triage run (default stack) with the four process-aborting laws skipped, so libtest prints every
# failure message (the owner's triage input).
unset RUST_MIN_STACK
cargo test --offline --no-fail-fast -p semio-framework-os-renderer-wgpu --lib -- --skip async_boundary_tests::an_independent_decoder_job_recovers_its_response_from_an_exact_rejected_session --skip kernel_runtime::semantic_document_tests::product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder --skip shell::settings_general_layout_tests::a_framework_setting_dispatch_completes_on_a_one_mebibyte_thread --skip async_boundary_tests::dropped_ordinary_frame_deferred_jobs_recover_the_exact_pair_and_close_remaining_actions_one_per_step; echo "LW1-STEP renderer-wgpu-lib-skip4 rc=$?"
