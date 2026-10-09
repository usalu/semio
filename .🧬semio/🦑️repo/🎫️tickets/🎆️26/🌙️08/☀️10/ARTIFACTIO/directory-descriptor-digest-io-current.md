# Directory Descriptor Digest IO Ownership

The five Rust descriptor binary bodies and five TypeScript bodies now live under `directory/io/binary/descriptor-digest`; the domain constant also lives in IO. Schema and Directory/OS semantic facades no longer export the physical encoding/digest functions. Native production consumers use the explicit IO owner; OS in-source test dependency typing names the IO contract directly. Hub creation schema admission remains root-owned and was excluded from the broad consumer edit.

Pure metadata validation is `schema::validate_document_descriptor_v1(&DocumentDescriptor) -> Result<(), DescriptorDigestError>`, exported by Directory root. It checks positive bootstrap version, commit≤head, seven nonempty text leaves, and three nonzero lowercase64 hash shapes without byte conversion or hashing. TypeScript `validateDocumentDescriptorV1` additionally validates safe unsigned integer bounds. Native IO encoding calls this validator before framing. Descriptor metadata and the refusal type remain semantic.

## Actual validation

- SourceRED session84596: strict source execution reached the independent crypto law; one law passed and the actual owner law failed on `schema::decode_descriptor_hash`. This was the intended architecture RED.
- Source2 session89226: terminal exit0, strict TypeScript and two Bun laws,27 assertions. Native relocation and neutral authored byte/digest vector match Node crypto and WebCrypto with DEBUG receipts.
- Source3 session41237: terminal exit0, strict TypeScript +4 Bun laws/43 assertions, including five neutral metadata refusals and Tree-sitter witnesses for current Rust schema/root/IO assemblies/codec/native laws.
- Native target registered as `test-directory-descriptor-digest-native`, filters actual `os_directory::io::binary::descriptor_digest::tests::` and captures DEBUG. Native law was moved to the actual physical owner; metadata refusal and module-owner laws were added. These native laws have not run because the shared OS receiving-floor compilation remains unresolved. No native GREEN claim.

## Remaining verified Directory binary boundaries

`schema::ArtifactHash::parse_hex` decodes a lowercase64 external hexadecimal string using `u8::from_str_radix`; `ArtifactHash::hex` calls schema `hex_lower`, which emits physical hexadecimal. `EditedArtifactFrontierV1::public_frontier` decodes `chain_sha256` through that schema helper. Pure canonical checkpoint pair `admit` currently formats the typed hash via `.hex()` to compare an externally textual descriptor digest. These are real remaining boundaries, not closed by the descriptor/pair extraction. A typed authority hash in the relevant semantic contract and explicit IO text admission are required to avoid merely redirecting schema to IO. Root is informed. TypeScript schema still uses TextEncoder to enforce UTF8 byte limits in semantic text validators; this scoped extraction did not alter root-owned JSON/service paths.

## Scoped edited manifest

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧫️fixtures/🔣️.json`
- `🌎️hub/🛰️lag-rebootstrap/🦀️.rs`
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`
- `🌎️hub/💡️inference/📇️catalog/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/📌️check-in/🦀️.rs`
- `🌎️hub/🏗️bootstrap/🦀️.rs`
- `🌎️hub/📇️directory/🦀️.rs`
- `🌎️hub/📇️directory/🌐️neo4j/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs`
- `🌎️hub/📇️directory/🪶️sqlite/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs`
- `🌎️hub/📇️directory/🐘️postgres/🌱️creation-v1/🧪️tests/🔬️standalone/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🟦️.ts`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/project.json`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/package.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`

The separately authorized remaining hash/frontier/pair bodies are now extracted and typed; `directory-hash-typed-admission-current.md` supersedes the former remaining-boundary list. Current Hash Source6 session59966 strictTS8tests124 assertions is GREEN, Pair Source4 session17630 strictTS33tests58 assertions is GREEN. Native combined IO binary laws remain pending.
