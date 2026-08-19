# ArtifactStore::new Result Fallout — Scope and Fix

**Ticket:** `26/08/19/ARTIFACT-STORE-NEW-RESULT-FALLOUT`  
**Trigger:** `ArtifactStore::<P, Mutation>::new(envelope)` now returns `Result<Self, VcsError>` (`🏪️store/🦀️component.rs:4350`).

## Scope (verified 2026-08-19)

### Root cause

`new` validates durable history (`validate_durable_history`, `validate_history_lanes`, `fold_history`) and returns `Err` on malformed envelopes. Call sites that treated `new` as infallible now get `Result` and fail with `E0599` (`dispatch` / other methods not on `Result`).

### What actually needed fixing

| Area | Broken before fix? | Action |
|------|-------------------|--------|
| `🏪️store` `mod tests` wrapper (`struct ArtifactStore(super::…)`) | **No** — wrapper `new` already calls `super::ArtifactStore::new(envelope).expect("test fixture history is valid")` | **No change** (bulk `.expect` was attempted and reverted) |
| `✏️s/🔌️plugins` native tests using `store::ArtifactStore` directly | **Yes** — 5 files | `.expect("valid artifact store fixture")` |
| `📡️replication/📡️wire` | **No** — no `ArtifactStore::new` call sites | N/A |
| Production paths (`plugin`, `host`, `db`, `space`, store `?` paths) | **Already handled** with `?` / `map_err` | N/A |

### Plugin files fixed

1. `🌀️procedural/…/procedural2d/…/🧬️mutations/💾️binary/🦀️component.rs` (test `document_text_round_trip_with_operation_applied`)
2. `🌀️procedural/…/procedural3d/…/🧬️mutations/💾️binary/🦀️component.rs` (same)
3. `🌀️procedural/…/procedural3d/…/🧬️mutations/🦀️component.rs` (test `store_applies_widget_create`)
4. `🎥️shooting/…/🧬️mutations/💾️binary/🦀️component.rs`
5. `🎪️demonstrator/…/playground/…/🧬️mutations/🦀️component.rs` (multiline `new`)

### Pattern

Non-`JsValue` tests:

```rust
store::ArtifactStore::<P, M>::new(envelope).expect("valid artifact store fixture");
```

Wasm / `JsValue` returns (see animate D1):

```rust
.map_err(|e| JsValue::from_str(&e.to_string()))?
```

### Verification

```bash
cargo test -p semio-s-plugin-procedural --lib --no-run
cargo test -p semio-framework-os-kernel --lib --no-run
```

After fix: **zero** `ArtifactStore` / `E0599` / `dispatch found for enum Result` errors.

Remaining compile failures (unrelated, peer in-flight `OperationContext` on directory transport):

- `semio-framework-os-kernel` lib: 2× `E0061` (`directory/🪪️identity`)
- `semio-framework-os-kernel` lib tests: + `E0050` on `DirectoryTransport` trait impl

### Repo-wide grep (remaining intentional non-expect `new`)

Only `?`-handled production paths and `match`/`assert!(matches!` error-path tests in `🏪️store`.
