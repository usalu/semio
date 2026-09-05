# Procedural Test Blockers Inventory — 2026-09-05

## Survey Scope
- Crate: `semio-s-plugin-procedural` (`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/🦀️.rs`)
- Test targets: All `#[cfg(test)]` and `#[test]` code across three artifact subsets
- Artifacts scanned:
  - `🌀️generation2d` — fixture mutations, editors, UI code
  - `🧊️generation3d` — fixture mutations, editors, 3D-specific code, examples
  - `🧩️assembly` — WFC engine, constraint satisfaction, solver tests

## Compilation Blocker Classes Found

### Class 1: Un-awaited Async Testkit Calls
Calls to `protocol::testkit` law helpers (`assert_mutation_inverse_law`, `assert_mutation_diff_absorb_law`) that are async functions but invoked without `.await` in non-async `#[test]` functions.

**Correct attribute:** `#[semio_framework_async_macros::async_test]` instead of `#[test]`

| Subset | Count | Representative Examples |
|--------|-------|-------------------------|
| 🌀️generation2d | 14 | `🧬️mutations/🦀️.rs:255`, `🧬️mutations/🦀️.rs:263`, `🧬️mutations/🦀️.rs:278` |
| 🧊️generation3d | 0 | (Uses `.await` in async_test fns) |
| 🧩️assembly | 0 | (No law helper calls found) |

---

### Class 2: serde_json / serde:: on Migrated Types
Direct calls to `serde_json::from_str()`, `serde_json::to_value()`, `serde_json::Value` that should use framework's `ToValue`/`FromValue` trait instead (repo migrated to `dsl::json` wrapper).

**Pattern observed:** Every mutation fixture test file includes 11–18 `serde_json::` calls.

| Subset | Count | Representative Examples |
|--------|-------|-------------------------|
| 🌀️generation2d | ~180 (18 per fixture test, 10 tests) | `🧬️mutations/🧹clear-widget-layout/🧪️tests/🦀️.rs:21`, `🧪️tests/🦀️.rs:56–62`, `🧪️tests/🦀️.rs:70–115` |
| 🧊️generation3d | ~110 (11 per fixture test, 10 tests) | `🧬️mutations/🔗️connect-synapse/🧪️tests/🦀️.rs:21`, similar pattern across 10 mutations |
| 🧩️assembly | ~180 (18 per fixture test, 10 tests) | `🧬️mutations/🔗️connect-slots/🧪️tests/🦀️.rs:21`, similar pattern across 10 mutations |

---

### Class 3: Mutation Constructors → Enum Variants
Calls to `SomeMutation::new(...)` or struct literals `MutationType { ... }` where the type became an enum variant (declared as `MutationType(PayloadTuple)` in the derive macro).

| Subset | Count | Representative Examples |
|--------|-------|-------------------------|
| 🌀️generation2d | 0 (not found in manual scan) | — |
| 🧊️generation3d | 0 (not found in manual scan) | — |
| 🧩️assembly | 0 (not found in manual scan) | — |

**Note:** Mutations appear to use builder functions (e.g., `create_widget(...)`) not direct constructors.

---

### Class 4: Renamed protocol::testkit Law Helpers
Uses of old names for law helpers (e.g., `assert_mutation_absorb_law` vs. current `assert_mutation_diff_absorb_law`).

**Current names in framework (from `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️.rs`):**
- `pub async fn assert_mutation_diff_absorb_law<P, D>(...)`
- `pub async fn assert_mutation_inverse_law<P, Op>(...)`

| Subset | Count | Representative Examples |
|--------|-------|-------------------------|
| 🌀️generation2d | 0 (uses current `assert_mutation_inverse_law`) | — |
| 🧊️generation3d | 0 (uses current names with `.await`) | — |
| 🧩️assembly | 0 (no law helper calls) | — |

---

### Class 5: VcsArtifactApp Construction Mismatches
Wrong arity or generic arguments to `VcsArtifactApp::new()` or similar factory calls.

| Subset | Count | Representative Examples |
|--------|-------|-------------------------|
| 🌀️generation2d | 0 (not found) | — |
| 🧊️generation3d | 0 (not found) | — |
| 🧩️assembly | 0 (not found) | — |

---

### Class 6: Out-of-Scope Types
References to `Widget`, `Mesh3d`, `FormGeneration`, `Generation3dPreviewCamera` without resolving path (when they are not imported/re-exported).

| Subset | Count | Representative Examples |
|--------|-------|-------------------------|
| 🌀️generation2d | 0 (uses `Widget` from `flow::playbook`, imported inline) | — |
| 🧊️generation3d | 0 (uses `Widget` from `flow::playbook`) | — |
| 🧩️assembly | 0 (no UI types) | — |

---

## Summary by Subset

| Subset | Class 1 | Class 2 | Class 3 | Class 4 | Class 5 | Class 6 | **Total** |
|--------|--------|--------|--------|--------|---------|---------|----------|
| 🌀️generation2d | 14 | 180 | 0 | 0 | 0 | 0 | **194** |
| 🧊️generation3d | 0 | 110 | 0 | 0 | 0 | 0 | **110** |
| 🧩️assembly | 0 | 180 | 0 | 0 | 0 | 0 | **180** |

### Biggest Blocker
**🌀️generation2d** with 194 total blockers, dominated by:
- 14 un-awaited async law helper calls (blocking entire test suite on first failure)
- 180 serde_json usage violations (widespread pattern across all mutation fixture tests)

---

## Integration Test Configuration

### `[[test]]` Declarations in Cargo.toml
**None found.** The crate contains only a library target (`[lib]` with `crate-type = ["cdylib", "rlib"]`), no separate `[[test]]` integration test target declared.

### Participation of `🧪️tests/🦀️mutate-procedural-3d-1/🦀️.rs`
Located at: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🦀️mutate-procedural-3d-1/🦀️.rs`

**Target:** Participates in the **library `--lib` test target** (via conditional inclusion in the module tree), NOT a separate integration test target.  
- Compiled as part of `cargo test --lib`
- Any blocker in generation2d or assembly blocks this file's compilation

---

## Repair Priority

1. **Immediate (blocks lib compilation):**
   - Fix 14 Class 1 calls in generation2d mutations tests → add `#[async_test]` and `.await`
   
2. **High (pervasive pattern):**
   - Replace ~470 total `serde_json::` calls across all subsets with `dsl::json` / `ToValue`/`FromValue` wrappers
   - This is a mechanical codemod across ~30 fixture test files (10 per subset)

3. **Verification:**
   - After fixes, run `cargo test --lib` in the procedural crate root
   - Confirm all three subsets compile and tests execute
