# I1 — DSL value-serde codec fix (`dsl_value_serde.rs`)

Read `📓️h3-db-hub-fix.md` first: it correctly diagnosed and refused (out of lease) the last db
failure, `db_artifact::tests::bridge::envelope_from_operation_uses_operation_and_diff_traits`,
tracing it into `🗣️dsl/🧬️schema/dsl_value_serde.rs`'s `ValueDeserializer`.

## Bug 1 (assigned): `deserialize_newtype_struct` blanket-forwarded to `deserialize_any`

`serialize_newtype_struct` is transparent — it serializes the inner value directly, no wrapper.
But `deserialize_newtype_struct` was in the `forward_to_deserialize_any!` list, so a scalar
newtype (e.g. `struct Counter(i64)` over `DslValue::Number(15.0)`) called `visitor.visit_u64(15)`.
The derived `Visitor` for a newtype struct only implements `visit_newtype_struct`/`visit_seq`, not
scalar `visit_*` methods → `invalid type: integer 15, expected tuple struct Counter`.

Fix: real `deserialize_newtype_struct` impl calling `visitor.visit_newtype_struct(self)` (serde's
contract for a transparent wrapper — hands the visitor the same deserializer, unmodified), removed
`newtype_struct` from the forward list.

## Sibling-method review (as requested)

- `deserialize_option` — **correct, not blanket-forwarded.** Custom impl: `Null` → `visit_none()`,
  else → `visit_some(&mut *self)`. Matches serde's self-describing-format contract exactly.
- `deserialize_enum` — **not blanket-forwarded** (custom `EnumAccessUnit`/`EnumAccessTagged`), but
  review turned up a **second, independent bug** (see below), now also fixed.
- `deserialize_unit_struct` — **correctly forwarded.** `serialize_unit_struct` always emits
  `DslValue::Null`; `deserialize_any` on `Null` calls `visit_unit()`, which is exactly what a unit
  struct's derived `Visitor` implements. No mismatch.
- `deserialize_tuple_struct` — **correctly forwarded.** Only ever reached for tuple structs with
  ≥2 fields (1-field tuple structs route through `deserialize_newtype_struct` instead, per serde's
  own derive routing — confirmed, not assumed). `serialize_tuple_struct` emits `DslValue::Array`;
  `deserialize_any` on `Array` calls `visit_seq`, which is exactly what a multi-field tuple
  struct's derived `Visitor` implements. No mismatch.
- `seq`, `tuple`, `map`, `struct`, scalar/`bytes`/`byte_buf`/`identifier`/`ignored_any` — all
  correctly forwarded; each forwards to a `deserialize_any` branch that dispatches to the exact
  `visit_*` the corresponding derived/std `Visitor` implements.

## Bug 2 (found during the sibling review, same file, same lease): tuple-enum-variant field loss

`TupleVariantSerializer::end()` tagged its payload `{"kind": variant, "fields": [...]}`, but
`EnumAccessTagged::variant_seed` (the deserialize side) only ever looks for a `"value"` key,
defaulting to `DslValue::Null` when absent. Newtype and struct variants both use `"value"`, so only
tuple variants (2+ unnamed fields) hit this — every tuple-variant field was silently dropped on
decode (`VariantAccessNewtype::tuple_variant` fell through to an empty seq instead of erroring).

Verified live before fixing: added a temporary probe test (`enum E { Pair(i64, i64) }`,
round-trip) — failed with `invalid length 0, expected tuple variant E::Pair with 2 elements`.
Confirmed `"fields"` / `TupleVariantSerializer` have no other producers/consumers anywhere in the
repo (`grep -rn '"fields"' 🗣️dsl/` and `grep -rln TupleVariantSerializer` both single-hit this
file), so renaming the key is safe — nothing could have depended on a codepath that never worked.

Fix: `TupleVariantSerializer::end()` now tags with `"value"`, matching newtype/struct variants —
one consistent `{kind, value}` shape for all three tagged-variant kinds.

## Tests added (`dsl_value_serde.rs`'s own new `🧪️Tests` region, file had none before)

- `newtype_struct_wrapping_scalar_round_trips` — `Counter(i64)`, positive and negative.
- `newtype_struct_wrapping_string_round_trips` — `Label(String)`.
- `tuple_enum_variant_round_trips` — `enum E { Pair(i64, i64) }`, proves bug 2's fix.

## VERIFY (real numbers)

- `cargo test -p semio-framework-os-kernel-db --lib` → **424 passed; 0 failed** (was 423/1; the
  target test `db_artifact::tests::bridge::envelope_from_operation_uses_operation_and_diff_traits`
  passes individually too). Log: `🧪️i1-db-test.txt`.
- `cargo test -p semio-framework-os-kernel --lib` → **981 passed; 0 failed** (978 baseline + the 3
  new tests above, no regressions). Log: `🧪️i1-kernel-test.txt`.
- `bun ./📜️script.ts verify mutation-outcome-law` → **passed, 0 breaches**. Log:
  `🧪️i1-mutation-outcome-law.txt`.

## Files touched

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/dsl_value_serde.rs`
  (only file edited — entirely inside this lane's lease)

Ticket left open (not closed by this lane), per the worker brief.
