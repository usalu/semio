# `#[value(flatten)]` / `with` / `skip` added to the ToValue/FromValue derive

File: `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs`

## What changed

- `FieldAttrs` gained `flatten: bool`, `with: Option<String>`, `skip: bool`, plus
  `effective_serialize_with()`/`effective_deserialize_with()` helpers that resolve `with = "path"`
  to `path::to_value` / `path::from_value` (an explicit `serialize_with`/`deserialize_with` wins
  over `with` for that one direction — mirrors serde).
- `parse_field_attrs` accepts the three new keys.
- `named_fields()` now rejects `#[value(flatten)]` combined with the container's
  `#[value(deny_unknown_fields)]` via `syn::Error` → `compile_error!`, matching serde's own
  restriction (checked once, covers both `ToValue`/`FromValue` expansion since both call
  `named_fields`).
- `to_value_object_entries`: `skip` fields emit nothing; `flatten` fields splice their own
  `DslValue::Object` entries into the parent's `entries` Vec instead of nesting under the field's
  wire name.
- `from_value_struct_fields`: `skip` fields never look up `__entries`, going straight to
  `Default::default()` or `default = "path"`; `flatten` fields are fed a `DslValue::Object` built
  from every `__entries` key NOT owned by a sibling (non-flatten) field, via `FromValue::from_value`
  or the resolved `deserialize_with`.
- Scope: like the pre-existing `serialize_with`/`deserialize_with`/`skip_serializing_if`, the three
  new attributes are wired only for plain struct fields (`to_value_object_entries` /
  `from_value_struct_fields`), not for an enum variant's own named fields — module docs updated to
  say so explicitly.
- Module docstring (top of file) rewritten to document all three new attributes and drop `flatten`
  from the "deliberately not supported" list.

Validated real-world usage shape against the actual `#[serde(flatten)]` / `#[serde(with = "…")]` /
`#[serde(skip)]` call sites already in the repo (e.g.
`✏️s/🔌️plugins/🏛️architect/…/🗄️registers/🦀️.rs` bare `#[serde(flatten)] pub header: EntityHeader`,
`✏️s/🔌️plugins/🖨️raster/…/🦀️.rs` `#[serde(with = "asset_data_base64")]`,
`✏️s/🔌️plugins/🖍️draw/…/🦀️.rs` bare `#[serde(skip)]`) — all bare/simple forms, matching what this
derive now supports.

## Test additions

New file: `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/tests/🌾flatten-with-skip.rs`
(emoji filename — declared via an explicit `[[test]] name = "flatten_with_skip"` entry in this
crate's `Cargo.toml`, same pattern already used for `🛡️deny-unknown-fields-enums.rs`).

Covers:
- `flatten` on a nested-struct field: wire-key splice order, round-trip, and a `serde`/`serde_json`
  dev-dependency oracle comparison (`serde_json::Value` equality AND `serde_json::to_string`
  equality) proving byte-identical output to `#[derive(serde::Serialize)]` + `#[serde(flatten)]`.
- `flatten` on a `BTreeMap<String, String>` catch-all field: round-trip, byte-identical oracle
  comparison, and absorption of an otherwise-unknown wire key.
- `with = "path"` shorthand wiring both directions through a hand-written `hex_u32` module.
- `skip` (bare, and combined with `default = "path"`): omitted on serialize, defaulted on
  deserialize even when the wire key is present.
- A commented-out (not `trybuild`) struct documents the `flatten` + `deny_unknown_fields`
  `compile_error!` combination, since there's no runtime artifact to assert on for a
  compile-time rejection and `trybuild` isn't a dependency here.

Cargo.toml: added `serde = { version = "1.0", features = ["derive"] }` and `serde_json = "1.0"` to
`[dev-dependencies]` only (never a production dependency of this proc-macro crate) — the sanctioned
third-party oracle pattern — plus the new `[[test]]` entry.

## Verification (real run, foreground)

Command:
```
cd /Users/ueli/Documents/semio && cargo test -p semio-framework-value-derive --tests
```

Environment note: this repo currently has severe concurrent-build contention (dozens of other
agents' `cargo`/`rustc` processes sharing the same `target/` dir; swap usage peaked around
55-58GB/59GB, load average briefly 40-50). Multiple attempts blocked for 10-50+ minutes each on
`Blocking waiting for file lock on build directory` before the lock cleared. This is environmental,
not caused by this change — confirmed separately: an unrelated concurrent agent's build picked up
and successfully recompiled this crate's edited source within 4 minutes of the edit landing
(fresh `libsemio_framework_value_derive-*.dylib`), before this session's own `cargo test` run ever
got past the lock, proving the macro code itself compiled cleanly.

Real result once the lock cleared (`Finished \`test\` profile [unoptimized] target(s) in 26m 12s`,
all-in build+run time dominated by lock contention, not actual compile work):

```
     Running unittests 🦀️.rs (target/debug/deps/semio_framework_value_derive-05fb07210124573a)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/🛡️deny-unknown-fields-enums.rs (target/debug/deps/deny_unknown_fields_enums-78b01dc8138c7f50)
running 14 tests
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/🌾flatten-with-skip.rs (target/debug/deps/flatten_with_skip-16bd1de5a9904ea1)
running 9 tests
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

**Total: 23 passed, 0 failed, 0 ignored across both integration test binaries.** The pre-existing
`deny_unknown_fields_enums` suite (unrelated to this change) still passes in full, confirming no
regression.
