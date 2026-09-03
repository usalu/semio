# 🛑️ Permanent serde exceptions — verified, do not re-attempt

In-component crates with a non-optional serde dep went **14 → 9** today. Three of the remaining 9 are
NOT gaps; they are settled architectural decisions. Re-attempting them wastes a session.

| crate | why serde stays | evidence |
|---|---|---|
| `semio-framework-os-kernel-dsl-derive` | PROC-MACRO crate; serde is expansion-time only and never ships in a component | `proc-macro = true`; all `serde_json` use inside macro expansion logic |
| `semio-framework-ui-contract` | the crate's own docstring (`🦀️.rs:11-15`) names serde a PERMANENT allowed dependency | removing it → **248 errors**; `UiValue`→`DslValue` conversion is deliberately os-kernel's job (`ui_value_to_dsl_retained`, 🔌️plugin/🦀️.rs:6419) |
| `semio-framework-ui-scene` | `🦀️pack.rs` IMPLEMENTS serde's `Serializer`/`Deserializer` as the PRODUCTION byte-exact binary codec (14 wire tags incl. `TAG_CHAR`/`TAG_BYTES`) | protected oracle `owned_scene_neutral_vectors_match_native_serde_packet` round-trips raw `char` + `Bytes` against PINNED HEX bytes |

## Why `DslValue` cannot simply gain Char/Bytes variants
- `DslValue` has exactly 6 variants (Null/Bool/Number/String/Array/Object); bytes are not handled
  anywhere in its codec or the framework JSON writer — not base64, not array-of-numbers.
- Blast radius: **2418** `DslValue::` occurrences; **24** files hold an exhaustive, wildcard-free
  match — including BOTH independent JSON bridges (`🌱️value/🦀️.rs`, `🎒️pack/🔤️json/🦀️.rs`), graph,
  os-kernel, infinite, and 8 plugin artifact schemas.
- Char→String / Bytes→Array<UInt> would keep JSON parity but CANNOT reproduce ui-scene's
  fixture-pinned binary tags.

## The one real remaining gate
`DslValue`'s own ungated `impl serde::Serialize`/`Deserialize` (🌱️value/🦀️.rs:281,288). Its docstring
states the exit criterion for the whole ticket:
    "Remove once no serde-deriving type holds a `DslValue`."
Genuinely actionable crates left: `semio-framework`, `-actor`, `-graph`, `-os-kernel`, `-plugin`,
`-replication`.

## ⚠️ Build-environment contention is now a measurement hazard
Concurrent agents sharing one target dir produced: 5 hash-suffixed `os_kernel` rmeta files written at
once, an actively-modified `Cargo.lock`, and 3 DIFFERENT test-failure signatures with ZERO code
edits (`ui-scene` showed 3 spurious failures, unreproducible on retry; `value-derive` never got a
clean run across 6 attempts). Before believing any red result, check `git diff --stat -- Cargo.lock`
and the target dir's `.fingerprint` for concurrent writers, then `cargo clean -p <pkg>` and re-run.
