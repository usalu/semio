# InferredField<P> seam — Serialize/DeserializeOwned → ToValue/FromValue

Scope of this pass: `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️.rs` (the
`InferredField<P>` trait, `~line 83`, and its `Key`/`Value` associated types), plus the four
now-unblocked `🗄️stdio` plugin files named in the assignment. **No `cargo` command was run — by
explicit instruction, a peer session held the exclusive build-directory lock for over an hour with
more queued.** Everything below is edits-only, checked with `rustfmt --check` (real parser, confirms
the file tokenizes/parses as valid Rust and — where it printed no diff — is already
formatting-clean) and manual review, not a compiler run.

## What changed

### `💡️inference/🦀️.rs`

- `use serde::{de::DeserializeOwned, Serialize};` → `use crate::os_dsl::{DslValue, FromValue, ToValue};`
  (same import path `🏪️store/🦀️.rs` already uses for the identical trio — verified by grep, not
  guessed).
- `InferredField<P>::Key` bound: `... + Serialize + DeserializeOwned` → `... + ToValue + FromValue`.
- `InferredField<P>::Value` bound: `Clone + Serialize + DeserializeOwned + Send + Sync` →
  `Clone + ToValue + FromValue + Send + Sync`.
- `encode<T: Serialize>`/`decode<T: DeserializeOwned>` (the per-entity cache-value codec, used only
  for a bare `F::Value`) rebuilt over `crate::os_pack::json::to_json_string`/`from_json_str` (the
  `ToValue`/`FromValue` analog of `serde_json::to_vec`/`from_slice`, already the established
  seam-migration idiom — see `📇️identity/🦀️.rs:108,117` for precedent). Bytes are the JSON text's
  UTF-8 bytes (`.into_bytes()` / `std::str::from_utf8(...).expect(...)`), not a separate binary
  format — `pack::json` is documented byte-identical to `serde_json` for floats, per the assignment.
- **New `encode_map`/`decode_map` pair**, used only for the session's whole-result gate cache
  (`infer_field_after_diff`'s two `BTreeMap<F::Key, F::Value>` sites — the old
  `decode::<BTreeMap<F::Key, F::Value>>` / `encode(&result)` calls). Reason: the value-codec crate's
  `BTreeMap<K, V>: ToValue/FromValue` impl is **only defined for `K = String`**
  (`🌱️value/🔁️codec/🦀️.rs:280-291`, a JSON-object-shaped encoding) — `F::Key` here is any
  `Ord`-bounded type, not necessarily `String` (the trait itself is generic over it). Routing the
  whole map through the `String`-keyed impl would silently require `F::Key: Into<String>` that isn't
  there. Instead `encode_map`/`decode_map` hand-roll the wire shape as a `DslValue::Array` of
  `[key, value]` 2-element pairs (the same shape `serde_json` gives a `Vec<(K, V)>`), reusing
  `DslValue: ToValue/FromValue`'s identity impl to still go through
  `to_json_string`/`from_json_str` rather than inventing a second JSON emitter. **This is new code
  inside the file's own scope, not a codec-crate change** — nothing outside
  `💡️inference/🦀️.rs` was touched to make this compile.
- Updated the one stale docstring (`DagSnapshot`/`WeightSum` test fixture) that named
  `DeserializeOwned` as the reason `Key = String` rather than `&'static str`, to name `FromValue` /
  `decode_map` instead.
- `#[cfg(test)] mod tests` — no serde/serde_json use existed there before (checked by grep), so
  there was no differential-oracle test to preserve via `#[cfg_attr(test, derive(...))]`.

### The four `🗄️stdio` files (payoff half — bound no longer forces serde)

All four had the identical pattern: a cache-`Value` struct dual-deriving
`Serialize, Deserialize, value_derive::ToValue, value_derive::FromValue` with a docstring
explicitly blaming `store::InferredField::Value`'s old serde bound. Removed `Serialize`/
`Deserialize` from the derive list, removed the paired `#[serde(rename_all = "camelCase")]`
attribute (kept `#[value(rename_all = "camelCase")]` — argument parity was already 1:1, both said
`camelCase`), removed the now-unused `use serde::{Deserialize, Serialize};` import, and rewrote each
"Dual-derives serde" docstring to say the bound changed and serde was dropped. No other
`Serialize`/`Deserialize`/`serde_json` reference existed in any of the four files (checked directly,
not inferred) — dep_input encoding in all four already routed through `pack::to_json_string`
(pre-existing, untouched).

- `✳️brep/…/💡️inferences/✅validation-report/🦀️.rs` — `BrepValidationDiagnostic`
  (`Value = Vec<BrepValidationDiagnostic>` on `BrepValidationReport`'s `InferredField` impl; `Vec<T:
  ToValue>` is a generic codec impl, so this composes without further changes).
- `✳️graph/…/💡️inferences/🔗connectivity/🦀️.rs` — `SemioGraphNodeConnectivity`.
- `✳️table/…/💡️inferences/🎲entropy/🦀️.rs` — `SemioColumnEntropy`.
- `✳️table/…/💡️inferences/📊moments/🦀️.rs` — `SemioColumnMoments`.

All four `type Key = String` (has a hand-written `ToValue`/`FromValue` impl in the codec) and
`type Value = <the now-single-derive struct above>` — bound is satisfied conceptually.

## Explicitly NOT touched

- `🏪️store/🦀️.rs`'s `pack_rt` bridge (`dsl_value_to_json`/`json_values_equal`/
  `encode_json_value`) — per instruction, separate queued wave, 8 cross-crate consumers.
- No `Cargo.toml` edited, no dependency added or removed.
- No `serde_json::Value` routing introduced anywhere (checked: every new call goes through
  `DslValue`/`ToValue`/`FromValue`/`pack::json`, never `serde_json`).
- `🌉️mcp/💡️inference/🦀️.rs` is a **different file** (mounted separately, at
  `🌉️mcp/📦️packages/🦀️rust/🦀️.rs:70` → `../../💡️inference/🦀️.rs` resolves inside the `🌉️mcp`
  tree, not `🔨️modules/💡️inference`) — confirmed via grep of every `#[path=...]` mount pointing at
  a file literally named `💡️inference/🦀️.rs`; not in scope, not touched.

## Verification performed (non-compiling only, per instruction)

- `rustfmt --edition 2021 --check` on all 5 edited files: `💡️inference/🦀️.rs` and 3 of the 4
  stdio files → clean, no diff (parses AND already formatted). `✅validation-report/🦀️.rs` →
  rustfmt printed a diff, but `git diff` confirms it is **entirely outside** the lines this session
  touched (a pre-existing single-line-`DepInput`-literal formatting deviation around its
  `dep_input` function, ~60 lines below this session's edit) — parsing still succeeded (a real
  syntax error produces an error, not a diff), so this is a pre-existing style debt, not a defect
  introduced here.
- Manual bound-satisfaction trace for every `type Key`/`type Value` in all 5 files against the
  codec crate's actual impl list (read directly:
  `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs`) — `String`, `i64`, `Vec<T: ToValue>`, and each
  `#[derive(ToValue, FromValue)]` struct all resolve.
- Grep-confirmed zero remaining `Serialize`/`Deserialize`/`serde_json` identifiers in all 5 files
  except: one doc-comment line in `💡️inference/🦀️.rs` describing the `[[key,value],…]` wire shape
  as "the same shape `serde_json` would give" (prose analogy, not a use/derive), and the
  intentional "no longer dual-derives serde" explanatory docstrings in the 4 stdio files.

## Uncertain / worth a second look at central verification

1. **`decode_map`'s panic-based error handling** (`let DslValue::Array(...) = ... else { panic!(...) }`)
   mirrors the file's existing style (`encode`/`decode` already used `.expect(...)` on decode
   failure, treating a corrupt cache entry as a bug, not a recoverable error) — consistent, but
   flagging since it's new code, not a mechanical rename.
2. **`encode_map`/`decode_map`'s `[[k,v],…]` wire shape is NEW on-wire format** for the
   `InferenceSession.roots` gate cache — this cache is process-local/ephemeral (never persisted per
   the module's own doc comment structure), so there is no on-disk fixture to have drifted, but a
   central verification pass should confirm nothing external depends on the old
   `serde_json`-object-shaped encoding of this specific cache (unlikely — `roots` is private to
   `InferenceSession`, never serialized elsewhere per grep).
3. **`let-else` syntax** (`let DslValue::Array(items) = parsed else { panic!(...) };`) requires Rust
   ≥1.65; crate's `rust-version = "1.88"` (checked in
   `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`), so this is not a version risk.
4. Did not attempt to build/check anything — cannot confirm the crate actually compiles end to end;
   this is a structural/textual review only, per the explicit no-cargo instruction.

**Verification was not attempted. No `cargo` command (check, build, test, fmt --check via cargo) was
run at any point in this pass, per the explicit instruction that a peer session held the exclusive
build-directory lock.**
