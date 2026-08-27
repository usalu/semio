# PDF 1.7/UA Direct Mutation Cutover

## Scope and Result

The exact mutation root is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️ua/🧬️schema/🧬️mutations`.

The root now contains **95 files**: eleven direct semantic owners with eight files each, plus seven transparent root assembly files. `NoMutation`, generic `SetSnapshot`, and the whole-class stamping helper are removed. The original semantic graph transforms and conformance vocabulary are retained; empty inverse plans replace removed identity-sentinel values.

| Tag | Semantic Owner | Aggregate Variant |
| --- | --- | --- |
| 0 | `🏷️set-mark-info` | `SetMarkInfo` |
| 1 | `🗑️remove-mark-info` | `RemoveMarkInfo` |
| 2 | `🌲️set-struct-tree-root` | `SetStructTreeRoot` |
| 3 | `🪓️remove-struct-tree-root` | `RemoveStructTreeRoot` |
| 4 | `🗣️set-lang` | `SetLang` |
| 5 | `🤐️remove-lang` | `RemoveLang` |
| 6 | `🪧️set-display-doc-title` | `SetDisplayDocTitle` |
| 7 | `🚫️remove-display-doc-title` | `RemoveDisplayDocTitle` |
| 8 | `🏷️set-info-title` | `SetInfoTitle` |
| 9 | `🔤️embed-font-file` | `EmbedFontFile` |
| 10 | `🧺️remove-font-file` | `RemoveFontFile` |

Every owner contains `🦀️component.rs`, `🔣️component.json`, `🔣️payload.schema.json`, `🟦️component.ts`, `🔗️component.graphql`, `🛰️component.proto`, `📝️text/🦀️component.rs`, and `💾️binary/🦀️component.rs`. Payload schemas use the required JSON filename, never the JSON Lines emoji. All descriptors explicitly classify invertibility, diff participation, outcomes, composition, and seven required language surfaces.

The root's seven files are Rust, TypeScript, GraphQL, protobuf, JSON Schema, text registry/framing, and binary registry/framing. Both canonical root codecs are mounted locally. The root Rust component owns only mounts/re-exports, wrapped aggregate variants, derived delegation/catalog assembly, and structural correspondence tests.

The oracle Rust/catalog, exhaustive case adapter, and language-neutral feature were updated to exactly eleven kinds. The feature retains 22 concrete mutation/inverse rows. The direct text/binary facets own their operation-specific codec functions and tests; the UA root codec facets also test canonical framing and malformed identity rejection.

## Executed Evidence

- TDD red probe: direct owner contract `expected=11 actual=0`, exit `1`, before extraction.
- Ajv: `descriptors=11 payloads=11 surfaces=77 payloadCases=44 rootSchemaCompiled=true errors=[]`.
- Dependency-free internal validator versus Ajv: `internalAjvAgreementCases=66 errors=[]`, including valid descriptors and rejected negative tags, empty outcomes, both unclassified values, and extra properties.
- Exact Rust/TypeScript/GraphQL/protobuf/JSON/text/binary/catalog/adapter parity: eleven identities each, tags 0–10, `featureRows=22 featureKinds=11 errors=[]`.
- Bun TypeScript imports: root plus eleven direct components, `typescriptImports=12 errors=0`.
- Independent nightly Rust parser: `rustc 1.99.0-nightly (c4af71034 2026-07-06)`; all 45 UA Rust files plus the one exhaustive adapter parsed, `errors=[]`.
- Scoped `git diff --check`: exit `0`.
- Scoped sentinel/fallback/nested-owner/unclassified/source-`[DEBUG]` scan: zero matches (ripgrep exit `1`).
- No Cargo or registered Nx runtime command was launched in this lane. Runtime behavior and Rust test execution remain coordinator-serialized follow-ups; parser success is not presented as a runtime test pass.
- The coordinator's exact existence-checked UA all-17 structural policy query passed with zero violations. Transcript: `🧪️pdf-ua-independent-policy.log`.

## Authorized E/H Codec Reachability Closure

The E and H roots each now explicitly mount `📝️text/🦀️component.rs` and `💾️binary/🦀️component.rs` exactly once. All four physical targets exist. Independent nightly parse checked both roots plus all four canonical codecs: `roots=2 canonicalMounts=4 parserFiles=6 errors=[]`. Only the two root Rust mount sections changed in this follow-up.

A remains Rust-only: its root has no declared codec files and its descriptors require only Rust with null text/binary identities. An attempted mount-only addition was immediately removed when the physical-target check established this; A is unchanged by this follow-up.

Exact implementation paths and command transcripts are recorded in `🔣️pdf-1-7-ua-cutover-files.json` and `📓️pdf-1-7-ua-validation-commands.md`.
