# Editor Rust Cache Overrides — Current Inventory

Read-only JSONC-aware scan on 2026-10-15; no source changes, builds, lock probes or runtime writer claims. Existing ignored comments and trailing commas were parsed while preserving string literals.

.vscode/launch.json: 494 entries; keys {'CARGO_TARGET_DIR': 260, 'CARGO_BUILD_BUILD_DIR': 79, 'CARGO_BUILD_JOBS': 347, 'CARGO_PROFILE_DEV_DEBUG': 104, 'CARGO_INCREMENTAL': 105, 'CARGO_PROFILE_TEST_DEBUG': 23, 'CARGO_PROFILE_DEV_CODEGEN_UNITS': 24, 'CARGO_PROFILE_TEST_CODEGEN_UNITS': 24}; routes {'repo run': 361, 'repo exec': 129, 'other': 4}.

.vscode/🧩️launch.seed.jsonc: 419 entries; keys {'CARGO_TARGET_DIR': 198, 'CARGO_BUILD_BUILD_DIR': 38, 'CARGO_PROFILE_DEV_DEBUG': 103, 'CARGO_INCREMENTAL': 104, 'CARGO_BUILD_JOBS': 285, 'CARGO_PROFILE_TEST_DEBUG': 22, 'CARGO_PROFILE_DEV_CODEGEN_UNITS': 22, 'CARGO_PROFILE_TEST_CODEGEN_UNITS': 22}; routes {'repo run': 284, 'repo exec': 130, 'other': 5}.

These are authored editor ingress overrides, not proof that all paths currently produce output. Canonical current Cargo drivers may reject them before writing. Paths/jobs/profile/incremental settings belong to central prototype/release policy, but removing a target override blindly from an unqualified raw Cargo/ticket caller could instead restore an unbounded default target. Therefore classify current actual permanent entry routes and ensure genuine managed admission before removing obsolete overrides. Historical Original capability blobs also need separate validation; replacing them with path-only policy is not custody. The fixed managed cache envelope does not establish this remaining writer/config closure.
