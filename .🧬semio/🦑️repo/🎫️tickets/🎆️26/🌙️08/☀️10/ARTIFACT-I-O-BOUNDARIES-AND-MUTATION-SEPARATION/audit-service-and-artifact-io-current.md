# Current Service and Artifact IO Audit

Read-only bounded source audit, 2026-10-08. Root ticket owns infrastructure. Applicable root and products AGENTS read. No production changes, tests, native builds, Git mutations, or runtime claims. Native floor511 UI/Store remains the supplied limitation. Existing active Directory/Config/Procedural/GIS/PDF fixes intentionally not re-audited.

## Config Cannot Become Standalone by Removing Unused Imports

`🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust/Cargo.toml:31,35` declares kernel as both regular dependency and dev dependency with `protocol-laws` and `mutation-testing`. The aliases at `🎚️config/🦀️.rs:3-4` are used by included schema modules, so a literal search in the crate root is misleading.

Concrete regular dependency uses:

- `🎚️config/🧬️schema/🧬️mutations/🦀️.rs:14,30,53,69,87,104` derives `dsl::Mutations`; line 123 exposes `semio_framework_os_kernel::MutationDescriptorError` in a public registration result.
- `🎚️config/🧬️schema/🦀️.rs:143,166,304,376` implements kernel `MutationDiff` and `DiffAlgebra`; lines 437-580 expose mutation apply/projection functions using `MutationApplyResult` and `apply_diff`.
- `🎚️config/🧬️schema/🧬️mutations/🛡️change-merge-policy/🦀️.rs:12,23,59,63` uses kernel `MergePolicy` in public fields and arguments.
- Mutation leaves such as `🌗️set-appearance/🦀️.rs:5,14-21` derive `MutationLeaf` and implement kernel `MutationKind`, returning `MutationOutcome`.
- Test modules use `protocol::apply_diff`, mutation laws and execution; removing or bypassing them would not demonstrate independence.

Source conclusion: kernel extraction/reownership of mutation protocol contracts, derives, registration errors, and MergePolicy is needed before deleting the regular dependency. No compilation conclusion is made.

## Confirmed Remaining Semantic-to-Physical Codec Calls

All paths below are rooted at `🧰️framework/🛍️products/💻️os/🔨️modules/`.

| Owner and Location | Actual Body and Reachability | Classification |
| --- | --- | --- |
| `🪐️space/🗿️artifacts/🪐️space/🦀️.rs:1208-1225` | Public `space_package_from_schema(source: &str)` calls first-party `pack_json::from_json_str::<SpacePackageSource>`; `package_descriptor()` calls it with included artifact-definition JSON. | Artifact package identity admission retains physical JSON authority in artifact root. Separate typed validation from text codec. |
| `🪐️space/🗿️artifacts/🗂️collection/🦀️.rs:1217-1234` | Public `collection_package_from_schema` follows the same decode/identity-validation path; `package_descriptor()` supplies included JSON. Module root `🪐️space/🦀️.rs:4,11` reexports both schema parsers. | Same concrete artifact breach. |
| `🌿️vcs/🦀️.rs:1887` | Commit identity builder hashes `pack_json::to_json_string(change).as_bytes()`; pending branch hashes `pending_change_ref_json` (line 1861). | Physical JSON bytes are commit identity authority, beyond diagnostics. Ownership and byte semantics need explicit IO/identity contract; merely replacing serde with first-party JSON does not remove the breach. |
| `📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs:810` | Question renderer encodes semantic value into `UiInputNode.value` using `pack_json::to_json_string`. | Artifact UI inference performs text encoding. Typed input/presentation admission should own this separately. |
| `📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs:940-945` | `build_playbook_list_scene` serde-serializes `spec.steps` and palette into `BlockListScene.steps_json` and `palette_json`, falling back to `[]`. `render_playbook_builder` immediately follows this helper. | Artifact scene construction exports serialized public scene contracts and masks serialization failures. |
| `🌊️flow/📔️registry/🦀️.rs:7-10` | `seed_flow_eval_node_cache(..., output_json: &str)` decodes first-party JSON to `Dictionary`, then seeds cache. `🌊️flow/🖥️host/🦀️.rs:3748` delegates to it. | Registry semantic cache ingestion includes text codec. A typed dictionary seed API should sit below explicit response IO decoding. |

Verified callee body: `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1680-1688`: `to_json_string` calls `to_string(&from_dsl_value(&value.to_value()))`; `from_json_str` calls `parse(text, policy)` then `T::from_value(to_dsl_value(&value))`. These are physical text codecs despite first-party naming. Pure `ToValue`/`FromValue` alone is not counted as physical IO.

## Service Boundary Still Owns JSON Normalization

`💡️inference/🔌️service/🟦️.ts:31-44` implements `boundedServicePayloadV1` by recursively checking finite JSON-shaped values, JSON-stringifying to enforce a byte cap, then JSON-parsing to clone. Both semantic `parseInstalledServiceOperationV1` (line 17), status parser (line 51), and registry `dispatch` (line 126) call it. `🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts:99` also uses it. Physical JSON normalization therefore reaches ordinary semantic dispatch through an indirect helper.

At `💡️inference/🔌️service/🟦️.ts:10,76`, public operation declaration `inputSchema`/`outputSchema` are text strings and declaration admission parses them directly, checking `$id` ownership. This is a serialized schema public contract. `documentServiceRequestV1` (lines 83-96) is explicitly a network encoder and can remain physical boundary code if isolated; its request encoder must not become the authority for typed semantic payload admission. `JSON.stringify([owner,serviceId])` at line 141 is an internal key, not physical IO, and is excluded.

## External Serialized Value Public Contracts

`♾️infinite/🎲️board/🦀️.rs:53`, `🎲️board/🔌️ports/🦀️.rs:32`, and `🎲️board/🔌️ports/➡️directed/🦀️.rs:34` publicly expose `Option<serde_json::Value>` user data in descriptor structs. Board visibility helper APIs at `🎲️board/🦀️.rs:177,185,190` accept `serde_json::Map<String, serde_json::Value>` directly. Hand-written `ToValue`/`FromValue` bridges here are pure in-memory conversions and are not themselves IO, but do not cure external serialized types in public APIs.

`🎲️board/🔌️ports/➡️directed/🦀️.rs:750-752` exposes inherent `merge_from_json(&mut self, json: &str)` and calls `serde_json::from_str`; it then applies typed palette defaults and field merges. This is a hidden inherent physical codec in palette semantics; use a typed palette overlay below dedicated text ingress.

## Exclusions and Limits

Diagnostic error formatting/stringification, test serde oracles, compiler build-time descriptor admission, and filesystem activation receipts were excluded from artifact runtime findings. Flow WASM exports intentionally labeled input/output JSON are transport candidates, not automatically semantic breaches. No exhaustive repository closure claim is made; this bounded audit establishes the concrete remaining paths above.
