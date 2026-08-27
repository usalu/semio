# PDF 1.7/X Direct Mutation Cutover

## Scope and Result

Exact root: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️x/🧬️schema/🧬️mutations`.

The root now has **119 files**: fourteen direct semantic owners with eight files each, plus seven transparent root assembly files. The old sixteen-case aggregate loses `NoMutation`, generic `SetSnapshot`, and the class-stamp helper. Four local oracle/catalog/adapter/feature files are aligned, for **123 implementation paths** total.

| Tag | Direct Owner | Aggregate Variant |
| --- | --- | --- |
| 0 | `🔒️insert-encryption-dictionary` | `InsertEncryptionDictionary` |
| 1 | `🔓️remove-encryption-dictionary` | `RemoveEncryptionDictionary` |
| 2 | `🏳️set-output-intent` | `SetOutputIntent` |
| 3 | `🧽️remove-output-intent` | `RemoveOutputIntent` |
| 4 | `📐️set-trim-box` | `SetTrimBox` |
| 5 | `🧽️remove-trim-box` | `RemoveTrimBox` |
| 6 | `🔤️embed-font-file` | `EmbedFontFile` |
| 7 | `🧺️remove-font-file` | `RemoveFontFile` |
| 8 | `📜️insert-javascript-action` | `InsertJavascriptAction` |
| 9 | `🚫️remove-javascript-action` | `RemoveJavascriptAction` |
| 10 | `🚀️insert-launch-action` | `InsertLaunchAction` |
| 11 | `🛬️remove-launch-action` | `RemoveLaunchAction` |
| 12 | `🎬️insert-media-annotation` | `InsertMediaAnnotation` |
| 13 | `⏹️remove-media-annotation` | `RemoveMediaAnnotation` |

Each owner contains Rust behavior/tests, a completed descriptor, `🔣️payload.schema.json`, TypeScript, GraphQL, protobuf, direct text codec/tests, and direct binary codec/tests. Root text/binary components are visible registries plus framing and are mounted locally. JavaScript identities use canonical `Javascript` spelling.

X preserves `GTS_PDFX` with `OUTPUT_INTENT_DEST_PROFILE = true`, its page trim-box conformance operations, and its other print-conformance axes. Concrete diff/inverse semantics remain in direct leaves; no shared operations switch was introduced. Existing PDF object-graph primitives remain in the established conformance-support module.

## Executed Evidence

- TDD red probe before extraction: `expected=14 actual=0`, exit `1`.
- Ajv: `descriptors=14 payloads=14 surfaces=98 payloadCases=56 rootSchemaCompiled=true errors=[]`.
- Dependency-free internal validator/Ajv agreement: `84` valid/invalid descriptor cases, errors `0`.
- Exact enum/descriptor/TypeScript/GraphQL/protobuf/JSON/text/binary/oracle/adapter parity: fourteen identities each; tags `0–13`; feature rows `28`; feature kinds `14`; errors `0`.
- Bun TypeScript import parse: root plus fourteen direct components, `15` imports; errors `0`.
- Independent nightly Rust parser: `54` X Rust files plus one exhaustive adapter; errors `0`.
- Scoped `git diff --check`: exit `0`.
- Scoped sentinel, snapshot fallback, stamp helper, nested owner, unclassified value, old aggregate type/JavaScript spelling, and source-`[DEBUG]` scan: zero matches.
- No Cargo or registered Nx runtime was launched. The Rust behavior and codec tests are present but runtime execution remains a coordinator-serialized follow-up; parse-only validation is not a test-pass claim.
- The coordinator's exact existence-checked hardened all-17 X policy query passed with zero violations. Transcript: `🧪️pdf-x-independent-policy.log`.

Exact paths: `🔣️pdf-1-7-x-cutover-files.json`. Exact executed commands: `📓️pdf-1-7-x-validation-commands.md`.
