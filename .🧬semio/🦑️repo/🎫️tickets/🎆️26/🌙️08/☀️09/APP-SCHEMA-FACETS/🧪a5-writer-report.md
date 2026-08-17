# 🧪 A5 — ✒️writer

## Summary

Wave A5 delivered app schema facets for owner `✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/🎚️config`: ten handcrafted leaves for `WriterConfig` (all fields `local-ui`, matching runtime `WriterConfig` including nested `WriterEditorSelection`, `WriterEditorSettings`, and `WriterCamera`). Added sibling `👥️presence` with runtime `WriterPresence` / `WriterPresenceMutation` (DSL + pack, Snapshot mutation) and five schema leaves for shareable live state: AST selection, editor selection, tree/editor hover, and viewport camera. Wired `📦️glue.rs` (`config` + `presence` modules with `schema` children) and set `WriterPlayApp::Presence` / `PresenceMutation` to the typed pair.

## Files touched

- `✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/🎚️config/🧬️schema/` — five leaves (new)
- `✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/👥️presence/🦀️component.rs` (new)
- `✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/👥️presence/🧬️schema/` — five leaves (new)
- `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/🦀️component.rs`

## Gate tails

### Scoped `policyAppSchemaBreaches` (writer filter)

```
0
```

### `cargo check -p semio-s-plugin-writer`

```
Finished `dev` profile [unoptimized + debuginfo] target(s) in …
```

(exit code 0)

### `cargo test -p semio-s-plugin-writer --lib`

```
test result: ok. 91 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

## Unverified

- Runtime wiring of `presence_pack` / peer JSON serialization in the hub UI (kernel A3 scope); only crate types and policy facets were validated here.
- No dedicated presence DSL/pack round-trip unit test (lowpoly pilot likewise relies on derive + existing config tests).
