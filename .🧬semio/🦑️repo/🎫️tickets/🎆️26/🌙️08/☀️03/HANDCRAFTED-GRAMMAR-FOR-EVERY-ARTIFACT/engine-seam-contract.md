# Engine seam contract (v2)

## Layers

- L0 `dsl_core`: shared lexer; all handcrafted parsers call `dsl_core::lex`.
- L1 `dsl_notation`, `dsl_grammar` (`.semio` grammar + protocol dialects).
- L2 `dsl_family_*`: reusable notation fragments.
- L3 Per-artifact `🦀️component.rs` in facet dirs: handcrafted `DocumentDsl` / `OpText` / `DiffCodec` / pack / spr.
- L4 `#[derive(DslDocument|DslOps|…)]`: semantics only; **no** `DocumentDsl`/`OpText` emission after P5.

## Traits (`store` / `protocol`)

| Surface | Trait | Facet |
|---------|-------|-------|
| Document text | `DocumentDsl` | `🗣️dsl` |
| Document bytes | `DocumentPack` | `🎒️pack` |
| Op text | `OpText` | `🔧️op` |
| Op bytes | `OpBinary` | `📡️spr` |
| Patch text/bytes | `DiffCodec` | `🔺️diff` |

## Laws (per facet)

1. `parse(print(x)) == x` (canonical print is fixpoint).
2. `pack ≡ dsl` on same Rust value.
3. `opText ≡ opBinary`.
4. Grammar: recognizer accepts ⇔ handcrafted parser accepts on fixtures.
5. Protocol: spec-walk decodes bytes from Rust codec.
6. TS facade agrees with Rust on shared fixtures.

## Frozen paths (P1 single-writer)

`🗣️dsl/**`, `🎒️pack/🔢️value/**`, `🏪️store/**`, `📡️protocol/**` — no app edits in P1 except fixture-sweep.

## Wire format

`WireValue` / `EdgeArrow` + pack wire codec bump land atomically in P1.
