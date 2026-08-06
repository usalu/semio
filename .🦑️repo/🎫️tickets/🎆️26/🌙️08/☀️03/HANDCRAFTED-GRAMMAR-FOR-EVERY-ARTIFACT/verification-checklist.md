# Verification checklist (per artifact facet)

- [ ] `📖️component.grammar.semio` or `📡️component.protocol.semio` committed
- [ ] `🦀️component.rs` handcrafted parse/print or encode/decode
- [ ] `🟦️component.ts` WASM facade + vitest agrees with Rust
- [ ] Fixture in `📚️examples` or `🧫️fixtures` round-trips
- [ ] `store::test_support` / `dsl::test_support` laws in facet tests
- [ ] Grammar or protocol conformance test in fixture-sweep entry
- [ ] `LanguageSpec` registered; writer opens extension with diagnostics (`[DEBUG]` log once)

## Wave gate

`cargo check -p <crate>`, nx test for touched packages, `bun 📜️script.ts policy` allowlist shrinks, collision map re-run.

## Program done

All three allowlists empty: `POLICY_GRAMMAR_FILE_ALLOWLIST`, `POLICY_PROTOCOL_FILE_ALLOWLIST`, `POLICY_TS_FACADE_ALLOWLIST`. `dsl_derive` no longer emits text traits.
