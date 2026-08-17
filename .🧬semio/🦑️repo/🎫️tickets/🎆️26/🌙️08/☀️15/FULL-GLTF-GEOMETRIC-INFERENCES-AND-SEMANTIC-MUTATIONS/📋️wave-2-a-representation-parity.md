# Wave 2-A — Representation Parity

## Frozen representation

The text representation is a deterministic UTF-8, LF-only envelope with four headers followed immediately by the RFC 8785 canonical JSON encoding of the complete `GltfInference { geometry }` value:

```text
schema s.stdio.gltf.inference
version 2
length <canonical-json-utf8-byte-count>
checksum <eight-lowercase-hex CRC-32/ISO-HDLC>
<canonical GltfInference JSON>
```

There is no trailing LF. The checksum covers exactly the canonical JSON bytes. The TypeScript, GraphQL, JSON Schema, and Proto text facets expose the envelope metadata and the complete semantic value; diagnostics, validity, quality, and provenance remain explicit.

The binary representation is a 40-byte prefix followed by the same canonical JSON bytes. Multi-byte integers are little-endian:

| Offset | Width | Field | Constraint |
| ---: | ---: | --- | --- |
| 0 | 8 | magic | `89 53 f8 3f 7d 34 0d 0b` |
| 8 | 2 | format major | `1` |
| 10 | 2 | format minor | `0` |
| 12 | 4 | schema version | `2` |
| 16 | 4 | flags | bit 0 = canonical JSON; other bits zero |
| 20 | 4 | schema CRC-32 | `6b257ae0`, CRC of `s.stdio.gltf.inference` |
| 24 | 8 | payload length | exact payload byte count |
| 32 | 4 | payload CRC-32 | CRC-32/ISO-HDLC of payload |
| 36 | 4 | header CRC-32 | CRC-32/ISO-HDLC of bytes 0–35 |
| 40 | variable | payload | RFC 8785 `GltfInference` JSON |

The prior opaque payload, end-of-stream, declaration-only, and legacy bounds claims are removed.

## Changed representation facets

### Text

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🟦️component.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🔗️component.graphql`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🔣️component.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🛰️component.proto`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🔤️component.ebnf`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🅰️component.g4`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/📖️component.grammar.semio`

### Binary

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🟦️component.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🔠️component.abnf`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🥋️component.ksy`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🌶️component.spicy`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/📡️component.protocol.semio`

No Rust or file outside these existing representation leaves was changed in this wave, apart from this required ticket evidence.

## Validation evidence

- Bun TypeScript transpilation of both representation facets: passed.
- Text JSON Schema parse, external root-schema resolution, and Draft 2020-12 compilation through `ajv/dist/2020`: passed.
- Kaitai YAML parse through the workspace `yaml` parser, including little-endian and payload-length assertions: passed.
- Text GraphQL parity against all 67 concrete root indicator fields: passed.
- Text Proto import resolution to the authoritative root contract: passed.
- EBNF, ANTLR, and Semio text grammar header/root token parity: passed.
- ABNF, Kaitai, Spicy, Semio protocol, and TypeScript binary field parity: passed.
- Independent CRC-32 calculation confirms schema CRC `6b257ae0`: passed.
- In-memory construction and read-back of a 40-byte-prefix frame verifies magic, little-endian fields, payload length, header checksum, and payload checksum: passed.
- `bun nx run '@semio-tech/framework-os-kernel:test-quick'`: 862 tests passed, including handcrafted grammar conformance, handcrafted protocol conformance, and production coverage; one unrelated cross-artifact rejection test failed on pre-existing fixture soft-skips.
- `bun nx run 'breach-_framework_products_repo_script_ts:lint'` could not start because the repository policy script imports a missing framework TypeScript module; no representation file was reached.
- Native GraphQL, Proto, ANTLR, ABNF, Kaitai, and Spicy compilers are not installed, so those facets received the strongest available workspace parser or structural validation stated above.

## Ownership boundary

This wave defines representation contracts only. Rust text/binary codecs must serialize the exact text envelope and binary frame above; semantic inference computation remains owned by the geometry lane.
