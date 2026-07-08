# Command Kernel Verify Log

## Unit tests
- `cargo test -p semio-framework-core --lib` — 16 passed
- `cargo test -p semio-framework-sync --lib` — 1 passed
- `cargo test -p vcs --lib` — 10 passed
- `cargo test -p semio-framework-os invoke_command` — 1 passed
- `cargo test -p semio-framework-os hot_swap` — 2 passed
- `cargo check -p semio-framework-os -p semio-framework-plugin -p semio-framework-renderer-wgpu -p semio-framework-plugin-host` — pass

## End-to-end
- `bun run dev:lowpoly` — exit 0, dev server at http://127.0.0.1:6078/
- lowpoly plugin manifest callable via C-ABI wrapper (wasm-bindgen path)

## Architecture delivered
- Phase 1: kernel contracts in `framework/core/rs` (`kernel` module)
- Phase 2: `framework/hash`, `framework/hlc`, `framework/schema` crates
- Phase 3: vcs causal metadata, undo policies, merge strategies on existing Edit/Checkpoint/Alternative
- Phase 4: `framework/sync` OpDag + os-hub OpEnvelope transport
- Phase 5: `framework/wit/world.wit` rewritten
- Phase 6: wasm32-wasip2 toolchain, `component_plugin_exports!`, fuel/epoch host sandbox (C-ABI default retained)
- Phase 7: `plugin-worker.js` + `SEMIO_PLUGIN_WORKERS` in boot.ts
- Phase 8: `PluginHost::invoke_command` command router
- Phase 9: `validate_ui_node` render-plan validator in wgpu renderer
- Phase 10: `PluginSupervisorState` on host runtime + PluginHost supervisor map
- Phase 11: transactional `hot_swap_plugin` with rollback
- Phase 12: all 21 plugins retain `plugin_exports!()`; `component_plugin_exports!()` available for wasip2 builds
