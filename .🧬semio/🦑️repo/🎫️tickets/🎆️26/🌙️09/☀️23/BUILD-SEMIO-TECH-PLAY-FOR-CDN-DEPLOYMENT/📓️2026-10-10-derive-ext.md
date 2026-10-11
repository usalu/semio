# 2026-10-10 derive-ext: CanonicalJsonTree field roles

Owner: executor `derive-ext`. Corrections entry: `📓️2026-10-10-migration-corrections.md` #44.

## Files

- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🦀️.rs`: `wire` (role classification), `octets_kind`, `canonical_field_decimal`, general `skip_serializing_if` path predicate, `named_roles` takes the owner path.
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🧪️tests/🦀️.rs` (new): derive unit tests for accepted and refused roles and predicate expansion.
- `🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🧫️fixtures/🔣️.json`, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`: language-agnostic `roles` fixture (Ajv schema, JSON.stringify and SQLite oracles).
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs`: `ArtifactCanonicalDecimalU64` is `#[repr(transparent)]` and gains `from_ref(&u64)`. The `U64Text` node, its writer and the leaf already existed (core-tests).
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/🛫️encode/🧭️tree/🧪️tests/🦀️.rs`: tests `canonical_native_field_roles_match_serde_json_and_to_value_wire`, `..._project_original_fields_in_place`, `canonical_native_variant_field_roles_match_serde_json`.

## Rules

- Predicate: any path, `!path(&field)`. Previously only five fixed paths.
- Octets: `with`/`serialize_with` equal to `{pack::value::bytes|semio_framework_value::bytes}[::optional]::to_value` and a controlled serializer that is absent or the matching `to_value_controlled`. Projection equals `Vec<u8>` array of numbers (`Option` -> `null`).
- Decimal: `#[canonical_json(decimal_string)]` requires a value serializer on the field (the attribute is the canonical declaration, the serializer path is never guessed). Without the attribute any other custom serializer is refused.
- A `serialize_controlled_with` without a matching plain serializer is now refused (was silently ignored).
- Not covered: `ToValue` cannot be derived for named variant fields with `with`/`serialize_with`; the enum test is serde_json only. `Option<u64>` decimal is not supported.

## Verification (slot-gated, private dirs under `play-fleet/derive-ext`)

- `cargo test --manifest-path <value/derive>/Cargo.toml --lib`: 3 passed.
- `cargo test --manifest-path <pack/json>/Cargo.toml --lib canonical_native`: 11 passed (incl. the three new tests and the decimal u64 test).
- `cargo check --manifest-path <pack/json>/Cargo.toml --target wasm32-wasip2 --lib`: Finished.
- `bun test ./🧪️tests/🟦️.ts` in the derive canonical folder: 2 pass.

## Follow-up: hex-word binary32 role (corrections #48)

- `pack-json`: `ArtifactCanonicalJsonNode::F32HexWord`, leaves `ArtifactCanonicalHexWordF32`, `ArtifactCanonicalHexWordArray<N>`, `ArtifactCanonicalHexWordList` (`🛫️encode/🔣️scalar/🦀️.rs`, `🛫️encode/🧭️tree/🦀️.rs`, root re-export in `🔤️json/🦀️.rs`).
- Derive: `#[canonical_json(hex_word)]` for `f32`, `[f32; N]`, `Vec<f32>` and `Option` of them; `Wire`/`Declared` classification replaces `canonical_field_decimal` (now `canonical_field_role`). Decimal i64 selection by type (stdio B) is untouched.
- Proof: wire = lowercase `{:08x}` of the IEEE bits inside a string, same formatting as lowpoly `managed_mesh::json::words`; tests compare to a serde_json oracle with hand-written word serializers (finite, subnormal, max, -0.0, NaN payload, infinities) plus the language-agnostic `hexWords` fixture (Ajv schema, DataView bit oracle, SQLite word shape).
- Tests: `canonical_native_hex_words_match_the_managed_mesh_word_wire_and_serde_json`, `canonical_native_hex_words_project_original_fields_in_place`, derive unit tests `hex_word_role_is_explicit_and_independent_of_the_value_serializer`, `hex_word_references_cover_scalar_array_list_and_optional_owners`, bun `canonical hex word roles ...`.
- The lowpoly crate itself was not compiled here (fleet owner); the oracle replicates `words()` formatting rather than calling it.

## Follow-up: `#[canonical_json(tree)]` (corrections #49)

Derive `Declared::Tree` maps to `Wire::Direct` irrespective of value serializers (`T` and `Option<T>` already implement the tree trait). Tests: derive unit `tree_role_projects_through_the_field_tree_beside_any_value_serializer`; pack-json `canonical_native_tree_role_matches_the_custom_value_serializer_and_serde_json` (serde_json plus `ToValue` wire, pointer identity of the projected child). Files: derive `🧵️canonical/🦀️.rs`, `🧵️canonical/🧪️tests/🦀️.rs`, pack-json `🛫️encode/🧭️tree/🧪️tests/🦀️.rs`.

## Follow-up: `#[value(flatten)]` (corrections #50)

`FieldRole.flatten` splices the flattened record's members at the field position (node length, child and key delegation). Derive: `Wire::Flatten`; refusals for `skip_serializing_if`, decimal/hex roles and un-declared custom serializers. Tests: derive unit `flatten_role_requires_the_field_tree_and_no_presence_predicate`; pack-json `canonical_native_flatten_splices_at_field_position_like_serde_json_and_to_value`; bun `canonical flatten splices ...`; fixture/schema key `flatten`. Old unit assertions that flatten is refused were removed.

## Follow-up: externally tagged enums with named variants (corrections #51)

Derive generates a hidden transparent view per named variant (`__<Enum><Variant>CanonicalView`), cast from `&Self`, so the enum node is `Object(1){variant -> view}` and the view yields the fields. Tests: pack-json `canonical_native_externally_tagged_named_variants_match_serde_json_and_to_value`, `canonical_native_externally_tagged_variants_support_every_field_role_and_generics`; bun `canonical externally tagged named variants ...`; fixture/schema key `externalVariants`.
