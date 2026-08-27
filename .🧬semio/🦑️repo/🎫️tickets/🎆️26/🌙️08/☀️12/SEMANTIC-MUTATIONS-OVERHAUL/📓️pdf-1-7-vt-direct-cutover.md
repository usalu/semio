# PDF 1.7/VT Direct Mutation Cutover

## Scope and Result

Exact root: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🧬️mutations`.

The root now has **151 files**: eighteen direct semantic owners with eight files each, plus seven transparent root assembly files. The old twenty-case aggregate loses `NoMutation`, generic `SetSnapshot`, and the class-stamp helper. Four local oracle/catalog/adapter/feature files are aligned, for **155 implementation paths** total.

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
| 14 | `🗂️set-dpart-root` | `SetDpartRoot` |
| 15 | `🧹️remove-dpart-root` | `RemoveDpartRoot` |
| 16 | `🏷️set-dpart-metadata` | `SetDpartMetadata` |
| 17 | `🗑️remove-dpart-metadata` | `RemoveDpartMetadata` |

Each owner contains Rust behavior/tests, a completed descriptor, `🔣️payload.schema.json`, TypeScript, GraphQL, protobuf, direct text codec/tests, and direct binary codec/tests. Root text/binary components are visible registries plus framing and are mounted locally. JavaScript identities use canonical `Javascript` spelling.

VT preserves `GTS_PDFX` with `OUTPUT_INTENT_DEST_PROFILE = true`. Trim-box and all four document-part operations own their concrete diff/inverse code in their direct leaves; no behavior was moved into a shared operations switch. Existing PDF object-graph primitives remain in the established conformance-support module.

## Executed Evidence

- TDD red probe before extraction: `expected=18 actual=0`, exit `1`.
- Ajv: `descriptors=18 payloads=18 surfaces=126 payloadCases=72 rootSchemaCompiled=true errors=[]`.
- Dependency-free internal validator/Ajv agreement: `108` valid/invalid descriptor cases, errors `0`.
- Exact enum/descriptor/TypeScript/GraphQL/protobuf/JSON/text/binary/oracle/adapter parity: eighteen identities each; tags `0–17`; feature rows `36`; feature kinds `18`; errors `0`.
- Bun TypeScript import parse: root plus eighteen direct components, `19` imports; errors `0`.
- Independent nightly Rust parser: `66` VT Rust files plus one exhaustive adapter; errors `0`.
- Scoped `git diff --check`: exit `0`.
- Scoped sentinel, snapshot fallback, stamp helper, nested owner, unclassified value, old aggregate type/JavaScript spelling, and source-`[DEBUG]` scan: zero matches.
- No Cargo or registered Nx runtime was launched. The Rust behavior and codec tests are present but their runtime execution remains a coordinator-serialized follow-up; parse-only validation is not a test-pass claim.
- The coordinator's exact existence-checked all-17 VT policy query passed with zero violations under the hardened verifier. Transcript: `🧪️png-pdf-vt-independent-policy.log`, second record.

Exact paths: `🔣️pdf-1-7-vt-cutover-files.json`. Exact executed commands: `📓️pdf-1-7-vt-validation-commands.md`.
