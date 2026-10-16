# Two Intent Cargo Profile Integration

2026-10-16. Read-only inspection; no profile or Cargo configuration edits.

The current actual root Cargo manifest still authors `dev`, `host-tool`, `wasm-release`, `wasm-prototype` and many per-package overrides. `.cargo/config.toml` adds prototype/release, but global profile precedence and role-specific callers remain distributed. The existing `planCargoProfilePolicyV1` is pure planning DATA with `requiresConsumedView:true`; its own passing laws do not establish that actual Cargo consumes the plan. It reads existing authored native/WASM hot-package constraints and preserves measured optimization requirements, so blindly deleting old profile tables would lose behavior without wiring the real consumed configuration.

The remaining integration must route every actual native/descriptor/component/browser operation to prototype or release and materialize its exact selected role settings/host settings/package constraints in the owned compiler workspace before Cargo planning and compilation. Its actual consumed view, source provenance, full Nx inputs and configuration receipts must agree. Only after that integration can old authored profiles and callers be removed atomically. No compatibility aliases, deprecated profiles or migration script should remain. Current review is recorded here because the broader goal is not complete; no new runtime/performance claim follows from this inspection.


## Later Source Integration And Residual Reader Correction

The earlier root-profile-table observation above is historical. cargo-profile-consumption-implementation-current.md and its Source snapshot record the subsequent consumed-view integration and authored table removal (Source BOTH44/2009 plus strict at that epoch). Root later found three remaining wasm-prefixed receipt comparisons and a shipped-only binder that denied valid prototype models. component-selected-mode-binding-current.md records genuine Source REDs, exact selected-mode binding in both implementations, all actual callers migrated, existing component-profile launch Source BOTH9/186, and the current strict epoch. Those Source results do not qualify actual compiler emission, Native model/runtime/SDK/cache performance or cross-platform behavior.
