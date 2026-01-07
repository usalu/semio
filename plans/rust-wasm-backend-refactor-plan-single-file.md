# Rust + WebAssembly Backend Refactor Plan

> This is an updated version of the previous refactor plan, adapted to your **single-crate / single-source-file** Rust layout.  
> Previous plan reference: fileciteturn4file0

**Target layout:**

```
├─ rs
│ └─ semio
│   ├─ semio.rs
│   └─ Cargo.toml
```

## 0) Executive summary

### What you’ll build

A single Rust crate named `semio` that can be compiled into:

1. **WASM module** (for your existing TypeScript/React app): `wasm32-unknown-unknown` + `wasm-bindgen`
2. **Native library** (optional, for server-side reuse): `rlib` (and optionally a server binary behind feature flags)

### Key constraints introduced by “single file”

- **All Rust code lives in `semio.rs`**. You’ll use **inline modules** (`mod model { ... }`) instead of a directory tree.
- Tests should live in `#[cfg(test)] mod tests { ... }` inside the same file.
- For large fixtures/golden files: prefer running parity tests on the **TypeScript** side (WASM vs TS), keeping the Rust crate minimal.

---

## 1) Success criteria

- Deterministic, pure core logic (diff apply/inverse, validation, IO transforms).
- Backwards-compatible diff semantics or a well-defined shim.
- Import/export round-trip without data loss (including files).
- TS ↔ WASM API stays stable behind a tiny adapter.

---

## 2) Current-state inventory (do this before porting)

Document these items (1 short markdown note is enough):

- **Domain objects**: Kit, Type, Design, Quality, Author, Tag, Concept, Folder/File, geometry primitives.
- **Diff engine**: `KitDiff`, `apply*Diff`, `inverse*Diff`, collection diff semantics.
- **Commands**: stable command IDs, how they produce diffs/files.
- **Import/export**: archive layout, manifest/schema, file payload rules.
- **Remote hooks** (if any): file provider endpoints, collaboration sync.

This inventory is your checklist for parity.

---

## 3) Single-crate architecture inside `semio.rs`

Even with one file, keep boundaries explicit via inline modules.

### 3.1 Suggested module map (all in `semio.rs`)

```rust
// semio.rs (conceptual structure)

mod error { /* SemioError, ErrorEnvelope */ }

mod model {
  /* Kit, Type, Design, ... + serde */
}

mod validate {
  /* validate_kit(&Kit) -> Result<()> */
}

mod canonical {
  /* normalize_kit(&mut Kit) or -> Kit */
}

mod diff {
  /* KitDiff types + apply/inverse + helpers */
}

mod io {
  /* import/export archive v1/v2 + file index */
}

// WASM bindings should be at the bottom, gated:
#[cfg(target_arch = "wasm32")]
mod wasm_api {
  /* wasm-bindgen exports calling core functions */
}

// Optional server bits, gated by feature:
#[cfg(feature = "server")]
mod server_api {
  /* axum handlers or adapter glue */
}
```

### 3.2 “Core first” rule

- `model`, `diff`, `validate`, `canonical`, `io` must be **pure** and callable from:
  - WASM exports
  - (optional) server handlers
- WASM/server modules must contain **no business rules**.

---

## 4) Cargo.toml plan (single crate, multi-target)

### 4.1 Crate types

Export both WASM and native reuse from the same crate:

- `cdylib` for WASM build output
- `rlib` for native linking/testing

### 4.2 Feature flags (recommended)

- `wasm` (optional; but you can rely on `target_arch = "wasm32"` instead)
- `server` (optional; only if you later build a Rust HTTP backend)
- `binary` (optional; enable MessagePack/CBOR later)

### 4.3 Dependency strategy

Start minimal:

- `serde`, `serde_json`
- `thiserror`
- `wasm-bindgen` + `js-sys` (WASM only)
- `zip` (or similar) for archive handling
- `sha2` (optional) for hashing/checksums
- `proptest` (dev-dependency) for invariants

Keep anything server-related behind `feature = "server"`.

---

## 5) TS ↔ WASM boundary design (unchanged, but implemented inline)

### 5.1 Start with JSON boundary (fastest)

Exports (v1):

- `validate_kit(kit_json: String) -> Result<(), JsValue>`
- `apply_kit_diff(kit_json: String, diff_json: String) -> Result<String, JsValue>`
- `inverse_kit_diff(original_kit_json: String, applied_diff_json: String) -> Result<String, JsValue>`
- `export_kit(kit_json: String, files_index_json: String, files: ... ) -> Result<Vec<u8>, JsValue>`
- `import_kit(zip_bytes: Vec<u8>) -> Result<String, JsValue>` (or a JSON envelope with kit + file index)

You can add binary codecs later.

### 5.2 Stable error envelope

Return errors that TS can display without parsing Rust internals:

```json
{
  "code": "VALIDATION_ERROR",
  "message": "Human readable",
  "path": ["designs", 3, "pieces", 12, "plane"],
  "details": { "expected": "...", "actual": "..." }
}
```

Implementation tip: define `ErrorEnvelope` in `error` module and serialize it to JSON even for WASM errors.

---

## 6) Rust model port plan (inside `model` module)

### 6.1 Data representation

- All types derive `Serialize, Deserialize`.
- Use **newtype IDs** (`struct FileId(String);`) to reduce accidental mixing.
- Use `Option<T>` for optional fields.
- In **diff types**, allow representing “explicit null/clear” if TS depends on it:
  - `Option<Option<T>>` for “no change vs clear vs set”.

### 6.2 Validation

In `validate` module:

- Structural checks (required fields/formats)
- Referential checks (IDs exist, connections point to existing pieces)
- Business constraints (uniqueness, invariants)

Do not rely on TS to validate.

### 6.3 Canonicalization

In `canonical` module:

- Stable ordering for arrays/maps (for deterministic hashing and minimal diff churn)
- Decide whether to:
  - keep empty collections, or
  - drop them (but do it consistently)

---

## 7) Diff engine port plan (highest leverage)

### 7.1 Diff types

Inside `diff` module:

- `KitDiff` plus nested `*Diff`.
- `CollectionDiff` helper:
  - `added`, `updated`, `removed`
- Apply helpers:
  - “only changes what is present in diff”
  - no accidental deletes

### 7.2 Core algorithms

- `apply_kit_diff(base: &Kit, diff: &KitDiff) -> Result<Kit, SemioError>`
- `inverse_kit_diff(original: &Kit, applied: &KitDiff) -> Result<KitDiff, SemioError>`

### 7.3 Tests inside the same file

Use:

- `#[cfg(test)] mod tests { ... }`
- `proptest` for invariants:
  - apply + inverse restores base (with canonicalization)
  - diff omission preserves untouched fields

Golden fixtures: run them primarily in TS parity tests (WASM vs TS) to avoid requiring Rust-side fixture files.

---

## 8) Import/Export plan (single file implementation)

You still have two strategies:

### Strategy A: v1 compatibility (keep current archive)

Pros: zero UX changes  
Cons: SQLite-in-WASM can be painful (if your v1 format depends on SQLite)

### Strategy B: v2 archive format (recommended long-term)

Proposed v2 layout:

```
.semio/
  manifest.json
  kit.json
files/
  <fileId>/...
```

Plan:

- Import supports v1 + v2 (v1 as best-effort)
- Export defaults to v2 behind a feature flag at first
- File index includes `sha256`, sizes, mime

**Single-file caveat:** keep the IO code tidy with submodules (`mod io { mod v1 { ... } mod v2 { ... } }`) inside `semio.rs`.

---

## 9) Optional: “real server backend” from the same crate

If you later want a server:

- Gate it behind `feature = "server"` inside `server_api` module.
- Implement minimal endpoints:
  - `POST /kits/{kitId}/diff`
  - `GET /kits/{kitId}/export`
  - file upload/download/delete endpoints

But do not block the WASM migration on this.

---

## 10) Incremental migration path (adapted to single file)

### Phase 0 — Lock parity & fixtures (TS)

- Add canonicalizer + golden tests in TS (baseline).
- Create dual-runtime harness: old TS vs new WASM outputs.

### Phase 1 — Create `rs/semio` crate

- Write Cargo.toml (crate types for wasm + rlib).
- Add `semio.rs` with stub modules and a single `validate_kit` export.

### Phase 2 — Port model + validation

- Implement `model` and `validate`.
- Wire WASM `validate_kit`.

### Phase 3 — Port apply diff

- Implement `diff::apply_kit_diff`.
- Add TS parity tests for apply.

### Phase 4 — Port inverse diff

- Implement `diff::inverse_kit_diff`.
- Add property tests inside `semio.rs`.
- Add TS parity tests for inverse.

### Phase 5 — Port import/export

- Implement `io::import_kit` / `io::export_kit`.
- Prefer v2 export (optionally keep v1 import).

### Phase 6 — Flip UI to use WASM by default

- Keep TS API stable behind a wrapper:
  - if WASM fails to load, fallback to TS implementation.

---

## 11) Practical single-file conventions (to keep it maintainable)

- Put **public API** at the top (types, function signatures), and detailed impl below.
- Use long-ish comment headers between inline modules.
- Keep `wasm_api` at the bottom with minimal glue.
- Make canonicalization and validation explicit and callable (don’t “hide” them inside apply).

---

## 12) Definition of done (WASM milestone)

- `validate_kit`, `apply_kit_diff`, `inverse_kit_diff` callable from TS.
- TS parity tests pass on representative fixtures.
- Import/export round-trips a kit + files without loss (at least in v2).
- Determinism: canonical output stable across runs.

---

**End of single-file layout plan.**
