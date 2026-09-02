# value-derive: `#[value(crate = "…")]` + enum-variant field attribute fixes (2026-09-02)

File owned/edited: `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs` (the `#[derive(ToValue, FromValue)]`
proc macro implementation). Tests added to
`🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/tests/🌾flatten-with-skip.rs`.

## Defect 1 — HARD BLOCKER: crate path override (fixed)

Added `#[value(crate = "path::to::value_root")]` as a container attribute. Implementation:

- `ContainerAttrs` gained a `crate_path: Option<String>` field, parsed from the `"crate"` key in
  `parse_container_attrs`.
- New `container_crate_path(&ContainerAttrs) -> syn::Path` resolves it, defaulting to
  `::semio_framework_os_kernel` when absent (`syn::parse_str`).
- `expand_to_value`/`expand_from_value` each compute `let value_crate = container_crate_path(&container);`
  immediately after parsing container attrs, then pass `&value_crate` through every helper
  (`to_value_object_entries`, `deny_unknown_keys`, `from_value_struct_fields`, the new
  `variant_field_to_value_push`/`variant_field_from_value_read`/`variant_destructure_patterns`) and
  interpolate it as `#value_crate::Type` everywhere a literal `::semio_framework_os_kernel::` used to
  appear. All 73 occurrences of the old hardcoded literal were replaced (verified via
  `grep -c '::semio_framework_os_kernel::'` returning 0 post-edit); `generics_with_bound`'s
  `trait_path` argument is now `quote!{ #value_crate::ToValue }` / `#value_crate::FromValue}` too.
- Default behavior is byte-identical to before: a container with no `#[value(crate = …)]` still
  resolves to `::semio_framework_os_kernel`.

### How `semio-framework-actor` (or any sub-kernel crate) should invoke the derive

```rust
// inside semio-framework-actor, wherever it re-exports the value-runtime types it needs
// (it cannot depend on semio-framework-os-kernel directly — os-kernel already depends on
// semio-framework-actor, so a back-dependency would be a Cargo cycle):
pub use some_lower_crate::{DslValue, ToValue, FromValue, ValueError}; // or hand-rolled equivalents

#[derive(ToValue, FromValue)]
#[value(crate = "crate::value")]   // or "crate", or any path visible from the annotated type
struct ActorMessage { ... }
```

The path is resolved as a plain `syn::Path` and spliced in front of `ToValue`/`FromValue`/`DslValue`/
`ValueError` at every call site, so it just needs to be a valid path (from the derived type's own
crate root) that has all four names available (re-exported or defined locally).

## Defect 2 — SILENT WIRE-SHAPE BUG: enum-variant field attributes (fixed by implementing, not erroring)

`skip_serializing_if` on a NAMED field of an enum variant was parsed into `FieldAttrs` and then never
consulted by the three `Fields::Named` codegen sites (externally-tagged, adjacently-tagged
`content`, internally-tagged) — the field was always emitted via the default `ToValue::to_value`.

**While fixing this, `cargo check -p semio-framework` immediately turned up a REAL production
instance of the same bug class being silently wrong**, not merely theoretical:
`ArtifactActorMsg::LocalMutations`, `ArtifactEvent::RemoteMutations` (🏪️store's
`🔄️sync/🦀️.rs`), and `ArtifactMutationsSaved.envelope` (🏪️store's `🦀️.rs:2556`) all put
`#[value(serialize_with = "…", deserialize_with = "…")]` on a NAMED field of an internally-tagged
enum variant, relying on it to route `MutationEnvelope` through hand-written byte-framing bridges
(`operation_envelope_serde`/`envelope_serde`). That attribute was ALSO silently dropped for
enum-variant fields, meaning these three call sites were silently falling back to the default
`ToValue`/`FromValue` impls instead of the intended byte-framing bridge — a genuine, live,
previously-undetected wire-shape bug (the type still compiled and its own test suite still passed,
exactly the "silent is the dangerous part" scenario the ticket called out for `CapabilityOwner`/
`ContentBlock`).

**Decision: implemented (preferred, matching serde) rather than erroring**, for `skip`,
`skip_serializing_if`, `serialize_with`/`deserialize_with`/`with` on an enum variant's own named
field — all four now behave identically to their plain-struct-field counterparts:

- New `variant_field_to_value_push` / `variant_field_from_value_read` helpers (mirroring
  `to_value_object_entries`/`from_value_struct_fields`) handle `skip` (omit unconditionally on
  serialize; always `default`/`Default::default()` on deserialize, no wire lookup), `skip_serializing_if`
  (conditional omit), and `effective_serialize_with()`/`effective_deserialize_with()` (the existing
  `with`-shorthand-resolution methods on `FieldAttrs`, reused as-is).
- Wired into all 3 ToValue named-variant-field call sites and 2 FromValue named-variant-field call
  sites in `expand_to_value`/`expand_from_value`.
- `default` on enum-variant fields already worked correctly before this change (decode-only, no
  change needed) — confirmed by reading the pre-existing code, not assumed.
- `flatten` on an enum-variant's own named field remains **unsupported**, but is now a loud
  `compile_error!` (`check_variant_field_attrs_supported`) instead of a silent no-op — no live usage
  found repo-wide (`grep -rn 'value(flatten'` found only one hit, on a plain STRUCT field in
  `🌉️mcp/🧭️protocol/🦀️.rs` — `JsonRpcResponse.outcome`, unaffected). `with`/`serialize_with`/
  `deserialize_with` were REMOVED from that reject-list once the real 🏪️store usage was found — they
  are now implemented, not rejected.
- Also fixed a codegen-quality side effect: since `skip` fields no longer get pushed, the ToValue
  match-arm destructure pattern (`Self::Variant { field1, field2, .. }`) would otherwise bind an
  unused local for a skipped field. New `variant_destructure_patterns` emits `ident: _` for a skipped
  field's destructure slot instead of the shorthand `ident`, avoiding a rustc "unused variable"
  warning that a correct `skip` implementation would otherwise introduce.

### Report: same silent-drop class checked for `default` and `skip`

- `default` on an enum-variant named field: **already correct** before this pass (both externally-
  tagged and tag-based FromValue read sites already special-cased `field_attrs.default` when building
  the "missing key" fallback). No bug found.
- `skip` on an enum-variant named field: **was silently dropped**, same as `skip_serializing_if` — a
  `#[value(skip)]` field on a variant was still pushed on encode and still looked up (and required, if
  no default) on decode. Fixed alongside `skip_serializing_if` above.
- `serialize_with`/`deserialize_with`/`with`: **also silently dropped**, confirmed live-broken in
  🏪️store as described above. Fixed (implemented), not merely errored.

## Nice-to-have: `rename_all_fields`-style `SCREAMING_SNAKE_CASE`

Not implemented. `apply_case` only handles `camelCase`/`kebab-case`/`lowercase`/`snake_case`; adding
`SCREAMING_SNAKE_CASE` would be a small, mechanical addition (one more `match` arm building
upper-snake from the existing `split_words_snake`/`split_words_pascal` word lists), but was skipped
this pass to keep the change surface focused on the two required defects — the ticket said
nice-to-have/report-only. Flagged as a small standalone follow-up if the mcp agent's rename pain
recurs.

## Tests added (both in `🌾flatten-with-skip.rs`, registered `[[test]] name = "flatten_with_skip"` in
Cargo.toml already covers this file)

- `crate_path_override_compiles_and_round_trips_from_a_non_default_path` — a struct derived with
  `#[value(crate = "crate::value_root", rename_all = "camelCase")]` where `value_root` is a local
  `mod` re-exporting `DslValue`/`FromValue`/`ToValue`/`ValueError` from `semio_framework_os_kernel`
  under a NON-default path; compiles and round-trips.
- `variant_field_serialize_with_routes_through_the_named_bridge_not_the_default_impl` — an
  internally-tagged enum (`#[value(tag = "kind")]`) whose one named-field variant combines
  `serialize_with`/`deserialize_with` (routes a `Vec<u8>` through a byte-LENGTH bridge instead of the
  default `Vec<u8>` → `DslValue::Array` codec — trivially observable if it silently fell back),
  `skip_serializing_if = "Option::is_none"` + `default`, and `skip` + `default = "…"` all on the same
  variant's fields — asserts all three take effect together.
- `variant_field_skip_serializing_if_includes_the_field_when_the_predicate_is_false` — the
  complementary case (field present when the predicate is false).
- A comment-only `FlattenOnVariantFieldCompileError` block (same pattern as the pre-existing
  `FlattenDenyUnknownFieldsCompileError`) documents that `flatten` on an enum variant field is now a
  `compile_error!`.

## Verification (REAL, run — not inferred)

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/iso3
export RUSTC_WRAPPER=""
cargo test -p semio-framework-value-derive -j 1   # 26/26 passed, 0 failed (14 + 12; was 14 + 9 = 23 baseline)
cargo check -p semio-framework --message-format short   # 0 errors (same as baseline)
```

Proof-of-baseline pattern followed exactly as instructed: `git diff HEAD` on both edited files saved
to `.diff` files, `git apply -R` both, re-ran `cargo test -p semio-framework-value-derive -j 1` →
confirmed baseline 14 + 9 = 23/23 passed with the reverted (original) source, then `git apply` both
diffs back and re-ran → 14 + 12 = 26/26 passed again.

## Environment note (cost significant time — future agents on this crate should know this)

Building `-p semio-framework-value-derive` in the shared `iso3` `CARGO_TARGET_DIR` was extremely
flaky: repeated `E0464 multiple candidates for rlib dependency semio_framework_os_kernel` (a stale
unhashed `.rmeta`/`.rlib` pair with mismatched hashes for `libsemio_framework_os_kernel`) and,
alternately, `E0277`/"multiple different versions of crate serde_json in the dependency graph" on the
`flatten_with_skip` test binary specifically. Root cause (confirmed via `.fingerprint` inspection):
MULTIPLE distinct fingerprint directories existed for the `semio-framework-os-kernel` package itself
(4 different hashes, not just its `dsl-derive` proc-macro sub-crate) — the deliberately-cyclic
dev-dependency shape documented in this crate's own `Cargo.toml` comment (os-kernel depends on
value-derive normally; value-derive depends back on os-kernel only in `[dev-dependencies]`) appears to
cause Cargo to plan more than one unit for os-kernel in some builds, and — since it's a path
dependency built with a `[profile.dev.package.semio-framework-os-kernel] codegen-units = 1` override
at the workspace root — those units contend for the SAME unhashed output filename, and whichever
finishes last can leave a `.rmeta`/`.rlib` pair from two different compiles. Confirmed this was NOT
caused by my changes (reverted to the pre-existing, untouched `flatten-with-skip.rs` + baseline
`🦀️.rs` and reproduced the identical flakiness). What actually worked: `cargo clean -p
semio-framework-os-kernel -p semio-framework-value-derive -p semio-framework-os-kernel-dsl-derive`
(full fingerprint wipe for exactly these 3 packages, ~1.2GiB, cheap) followed by ONE
`cargo test -p semio-framework-value-derive -j 1` run. Plain retries without the full `-p`-scoped
clean did NOT reliably converge even after 8+ attempts.
