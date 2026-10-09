# Directory Hash Transport and Typed Admission

Semantic `ArtifactHash` no longer parses or emits physical hexadecimal. Real implementations live in `os_directory::io::binary::artifact_hash`: `parse_artifact_hash_hex`, `artifact_hash_hex`, `hex_lower`, `decode_edited_artifact_frontier_v1`, and `encode_edited_artifact_frontier_v1`. Directory root no longer exports the physical hex helper. The textual edited-frontier transport retains its existing shape validator; IO converts it to/from the typed semantic `ArtifactFrontier`. Native callers select explicit IO free functions.

Pure `AdmittedCheckpointSelectionV1` has typed checkpoint, descriptor, aggregate hashes and a typed baseline frontier. Its explicit semantic JSON schema, Rust struct and TS interface agree. Native checkpoint IO `admit_checkpoint_selection_v1` / `admitCheckpointSelectionV1` admits textual transport identities before `CanonicalCheckpointPairV1::admit` / `admitCanonicalCheckpointPairV1`. Pure open/rebootstrap gates compare hash values, without rendering text or calling IO. Original transport field grammar is preserved; no compatibility API or codec facade was added.

## Actual receipts

- Architecture RED52892: exit1,4pass/1fail, correctly named schema `pub fn parse_hex` as the violation.
- Hash Source2 session4687: terminal exit0, strictTS5tests56 assertions after actual body extraction/typed gate.
- Pair Source4 session17630: terminal exit0, strictTS33tests58 assertions. Authored corpus and independent Node digest/framing/shape witnesses still pass after explicit IO selection admission.
- Expanded Source3 session21701 and Source4 session76024 failed the new grammar census; the latter also exposed an incorrect double-prefix in the new fixture witness assertion. Source5 session33608:7pass/1fail, exact remaining cause the installed Tree-sitter grammar rejects valid Rust2024 `unsafe extern` in Hub inference runtime3033/3040. This was outside changed hash calls. The production source was not altered to accommodate this third-party grammar.
- **Current Hash Source6 session59966 terminal exit0: strictTS8tests124 assertions.** Actual DEBUG vector receipts, four neutral hash IO vectors versus independent Buffer, five metadata refusals, Ajv typed selection schema, every authored pair identity/refusal decision, descriptor Node/WebCrypto vector, IO owner laws, and native receiving grammar census. The census normalizes only the Rust2024 unsafe extern modifier for the older grammar and explicitly logs that limitation; all actual hash receiving bodies remain unchanged.
- Native descriptor target now filters actual `os_directory::io::binary::` laws, covering descriptor, hash/frontier and checkpoint native witnesses. Four neutral native hash vectors compare independent Rust formatting; typed frontier roundtrip/genesis refusal law is authored. No native runtime pass is claimed while the shared OS receiving-floor compilation remains unresolved.

## Remaining boundaries

Current production Directory schema census has no hash decoding/encoding functions, endian framing or crypto body. UTF8 byte-length shape validators remain in TS, in root-owned service/JSON paths; no actual native byte generation or interpretation is added here. Whole artifact IO goal remains open, including fresh native receiving and retained multi-item retirement proofs.

## Scoped edited manifest

- `🌎️hub/🛰️lag-rebootstrap/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🧪️tests/🔏️trusted-catalog-profile/🪶️count/🦀️.rs`
- `🌎️hub/🧪️tests/🔏️trusted-catalog-profile/🦀️.rs`
- `🌎️hub/🧪️tests/🔬️bin-unit/🪶️count-lease/🦀️.rs`
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`
- `🌎️hub/💡️inference/🏃️runtime/🦀️.rs`
- `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/💡️inference/📇️catalog/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🔌️adapters/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🌱️creation/🧑‍🏭️service-v1/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🌱️creation/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🌱️creation/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🌱️creation/🚪️io/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/📌️check-in/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/📌️check-in/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🌐️browser-actor/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧩️plugin-module/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧩️plugin-module/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/📤️publication/🦀️.rs`
- `🌎️hub/🏗️bootstrap/🦀️.rs`
- `🌎️hub/📇️directory/🦀️.rs`
- `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️quick/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📌️document-check-in-v1/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🪢️checkpoint-pair/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🪪️artifact-hash/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🪪️artifact-hash/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🪪️artifact-hash/🧪️tests/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🪢️canonical-checkpoint-pair-v1/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🪢️canonical-checkpoint-pair-v1/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🪢️checkpoint-pair/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🪢️checkpoint-pair/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🪢️canonical-checkpoint-pair/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🚪️io/🧱️binary/🔐️descriptor-digest/🧫️fixtures/🔣️.json`
- `🌎️hub/📇️directory/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🪢️canonical-checkpoint-pair/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`
