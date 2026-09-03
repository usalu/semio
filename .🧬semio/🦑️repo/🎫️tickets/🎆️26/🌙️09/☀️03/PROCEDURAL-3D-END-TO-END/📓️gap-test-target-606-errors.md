# 606 Compiler Error Analysis: gen3d, generation2d, assembly

## Summary

The `cargo test --package semio-s-plugin-procedural --lib` target has 606 errors across three artifact types. **The LIB target is affected**: errors occur in non-`#[cfg(test)]` code (lines 1385–1860 in gen3d editor/commands before the test region at 1951).

### Root Causes: gen3d Artifact

**1. E0277 (167 errors) — Serde trait bounds not satisfied**
- **Issue**: `Serialize`/`Deserialize` traits removed after serde → `ToValue`/`FromValue` migration (26/09/01–02)
- **Affected types**: `Generation3dSnapshot`, `Generation3dDiff`, `Generation3dMutation`, `Severity`, `MeshData`
- **Current API**: Derive `semio_framework_value_derive::{ToValue, FromValue}` instead of serde traits
- **Examples** (12–52 occurrences each):
  - `Generation3dSnapshot: serde::Deserialize<'de>` ×42 → needs `#[derive(FromValue)]`
  - `Generation3dSnapshot: serde::Serialize` ×15 → needs `#[derive(ToValue)]`
  - `Generation3dDiff: serde::Serialize/Deserialize<'de>` ×28 each → needs both derives

**2. E0423 (14 errors) — Expected function, found module**
- **Issue**: Mutation operations (`create_widget`, `delete_widget`, `connect_synapse`, etc.) are now modules with no exported function
- **Affected lines**: 1444–1457 in editor/🦀️.rs
- **Current API**: Access the mutation payload via `module::ComponentType`, call it as a struct variant `Generation3dMutation::CreateWidget(create_widget::CreateWidget { ... })`
- **Examples**: `create_widget`, `delete_widget`, `connect_synapse`, `disconnect_synapse`, `move_widget`, `update_camera`, `change_schema`, `update_widget`, `update_synapse`, `delete_widget_position`, `create_generation`, `delete_generation`, `rename_generation`, `change_generation_value`

**3. E0599 (6 errors) — No method on opaque Future type**
- **Issue**: Calling sync methods (`.expect()`, `.iter()`, `.is_empty()`, `.len()`) on unawaited futures
- **Affected lines**: 1749, 1760, 1762, 1393, 1397, 1405 in editor/🦀️.rs
- **Current API**: Await futures or use `.await.method()` syntax; if using result futures, prefer `.await?` error propagation over `.expect()`

**4. E0308 (16 errors) — Mismatched types**
- **Issue**: Return type mismatches, likely `VcsArtifactApp<EditorApp<...>>` vs. `EditorApp<...>` or async/await wrapping
- **Affected lines**: 1385, 1389, 1393, 1397, 1405, 1432, 1454 (and command files)
- **Current API**: Check expected return type on app constructor; ensure async context wrapping is consistent

**5. E0425 (6 errors) — Cannot find function in module**
- **Issue**: `assert_mutation_inverse_law` and `assert_mutation_diff_absorb_law` are in testkit but not in `protocol::testkit` path
- **Affected file**: 🧬️schema/🧬️mutations/🦀️.rs:504, 507, 514, 517, 524, 527
- **Current API**: Functions exist in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️.rs`; check import path or re-export visibility

**6. E0433 + E0432 (3 errors) — Cannot find type; unresolved import**
- **Issue**: `Widget`, `Generation3dPreviewCamera` types missing; `ui_wgpu::wgpu::kernel_3d_scene::Mesh3d` unresolved
- **Examples**: Widget (editor/commands/remove-widget/🦀️.rs:54), Generation3dPreviewCamera (schema/diff/text/🦀️.rs:222), Mesh3d (editor/🦀️.rs:1860)
- **Current API**: Check if imports moved (ui_wgpu module structure); `Widget` may be scoped differently post-migration

---

### Root Causes: Non-gen3d Artifacts (generation2d, assembly)

- **E0277 (232 errors)** — Same serde → ToValue/FromValue migration; affects `Generation2dSnapshot`, `Generation2dDiff`, `Generation2dMutation`, `AssemblySnapshot`, `AssemblyDiff`, `AssemblyMutation`, `Severity`, `CheckpointDoc`, `WfcPreview`, `AssemblyInferenceCommit`
- **E0308 (34 errors)** — Type/return mismatches (shared pattern with gen3d)
- **E0433 (19 errors)** — Widget type scoping (shared across all three)
- **E0277 + E0599 (others)** — Async awaiting and trait bounds (Future doesn't implement Debug, dsl::Fault doesn't implement Display)

---

### LIB Target Verdict

**YES — LIB is affected.** Errors occur in:
- gen3d editor main file (lines 1385–1860, before `#[cfg(test)]` at 1951)
- gen3d commands (remove-widget, set-active-example, set-lod-mode)
- gen3d schema files (mutations/🦀️.rs, diff/text/🦀️.rs)

Non-test code cannot compile; test-only gate doesn't help since the item fails to parse in all contexts.

---

## Immediate Actions

1. **Serde migration (E0277)**: Add `#[derive(semio_framework_value_derive::{ToValue, FromValue})]` to snapshot/diff/mutation types where `Serialize`/`Deserialize` was removed; verify feature flag is off (no `serde` dep for plugins)
2. **Module-as-function (E0423)**: Update test calls from `create_widget(...)` to `Generation3dMutation::CreateWidget(create_widget::CreateWidget { ... })`; check glue.rs if command dispatch changed
3. **Future awaiting (E0599)**: Prefix calls with `.await` or use `.await?` for Result types
4. **Testkit imports (E0425)**: Verify `protocol::testkit` re-exports `assert_mutation_*` functions or use full path from spr module
