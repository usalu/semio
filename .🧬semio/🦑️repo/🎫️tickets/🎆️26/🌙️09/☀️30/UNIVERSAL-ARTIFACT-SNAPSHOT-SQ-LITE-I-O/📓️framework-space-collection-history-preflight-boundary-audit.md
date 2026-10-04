# Framework Space, Collection And SpaceHistory Preflight Boundary Audit

Read-only current source audit, 2026-10-03. No Cargo, Source execution or production edits. These are three of the existing seven framework authorities; this does not change the117-plugin denominator. Source-defined selectors are not runtime receipts.

All source locations below are relative to `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules`. The exact owner roots are:

- Space: `🪐️space/🗿️artifacts/🪐️space`
- Collection: `🪐️space/🗿️artifacts/🗂️collection`
- History provider: `🏪️store/📜️space-history`; actual types live in `🏪️store/🦀️.rs`.

## Concrete Current Gaps

All three mounted providers omit `preflight_sqlite_snapshot_encoding`. Store `🦀️.rs:11179–11181` checkpoints EncodeNative(0,1), then returns typed UnsupportedOwner; a refusing callback instead returns Canceled. A direct public preflight law with sufficient limits therefore currently cannot succeed.

The erased codec exposes export/import, not a preflight pointer. Store11220–11224 reconstructs the owner, validates the subset, then directly invokes controlled native encoding at11223. It does not call preflight. The missing hook is a public capability gap, not demonstrated erased import failure. Each owner's subset validator reconstructs a second candidate under the same control; budget assertions must retain that real cost.

**Separate concrete current output-accounting defect:** History's mounted SQL encoder56–60 calls `🧬️schema/📸️snapshot/🪶️sqlite/🚦️native/🦀️.rs:209–231`. This creates a fresh `NativeEncodeControl(limits.max_value_bytes)` at213. The callback212 forwards progress only. No caller allocation stage, remaining physical budget or `owned_bytes` settlement occurs. Thus source bypasses max_allocation_bytes and resets its native allowance on each invocation. This was not executed in this audit. Compare Store `📦️codec/🪶️snapshot-capability/🛫️native-encoding/🦀️.rs:28–56`, used by Space/Collection: caller allocation_stage supplies remaining bytes, and native.owned_bytes is settled at55. The existing History ownership law365–429 tests decode, not output.

## Actual Typed Fields And SQL Authority

| Owner | Actual typed fields | Mounted authority and tables |
|---|---|---|
| Space | Root103–116: schema/name; kind Atelier/Studio/Archive; visibility Private/Public; ordered users(id/name/avatar Option/role Author or Spectator); ordered CollectionRef(id/name/document_id); programs strings; InstalledExtension(extension_id/version/source_uri/package_hash/enabled)120–129. | Root11–13 mounts provider. Provider11 registers bare SpaceSnapshot/SpaceMutation at os.space@1/*. SQL five tables: space_document, space_user, space_collection, space_program, space_extension. |
| Collection | Root190–200: schema/name, folders(id/parent_id Option/name), entries(id/folder_id Option/name/kind_id/body)178–185. Box body is Document(schema/document_id) or Blob(BlobRef hash/u64 size/media_type)41–44. | Root11–13 mounts provider; Pack228 opts in. os.collection@1/* bare CollectionSnapshot/CollectionMutation. SQL five tables: collection_document, collection_folder, collection_entry, collection_document_body, collection_blob_body. |
| SpaceHistory | Store25851–25862: checkpoints, alternatives, active_alternative_id Option. Checkpoint25814–25821: id/parent_id Option/message/authors/timestamp/members. Author id/name/avatar Option. Member25801–25807: document_id/checkpoint_id/alternative_id. Alternative25829–25832: id/name/checkpoint_ids. Timestamp actor/physical_ms/logical are u64. | Store25842–25843 mounts provider; provider19–21 explicitly registers bare SpaceHistorySnapshot/SpaceHistoryMutation at os.space.history@1/*. Seven SQL tables: space_history_document, space_history_checkpoint, space_history_timestamp, space_history_author, space_history_member_pin, space_history_alternative, space_history_alternative_checkpoint. |

Each SQL schema is adjacent at `🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql`: five/five/seven CREATE TABLE lines. Space row census provider12 is 1+users+collections+programs+extensions. Collection is 1+folders+2*entries. History25–46 is 1+sum(checkpoints:2+authors+members)+sum(alternatives:1+checkpoint_ids), with checked additions and256-item progress checkpoints.

Space provider controlled encode16–19/decode20–24 uses the canonical derived record producer, projection26–34 and paid reconstruction35–44. Exact coordinate/state validation46–50 remains mounted. Collection's controlled manual body lowering root89–106 preserves u64 as UInt; SQL high/low u32 words preserve its full domain, and reconstruction rejects invalid word ranges and missing/doubled/wrong-kind bodies. History provider encode56–60/decode61–64 is mounted; timestamp words are normalized high/low pairs. History native Text is its existing JSON text codec, Binary its existing Value-Pack codec; persisted SQLite remains seven semantic tables. Native JSON is not a hidden persisted SQLite snapshot carrier.

Ordinals preserve occurrence order. Duplicate logical ids, unresolved semantic references, NUL/non-ASCII literals and None versus empty string are explicitly present in current fixtures; do not replace them with integrity reconciliation. History alternatives have a declared ascending-id convention in Store25853–25858, while SQL preserves supplied ordinal order. These owners do not contain arbitrary IEEE fields or octet arrays. Collection references do not own the addressed document/blob payload, nor do Space collection refs own complete collections. No invented all-nine wrapper belongs in these real owner laws.

## Existing Executable Gates And Coverage

| Owner | Existing Bun/Nx Native gate | Source-defined selected laws | .vscode/launch.json |
|---|---|---|---|
| Space | `bun nx run @semio-tech/framework-space-space-rs:test-snapshot-sqlite-native --skip-nx-cache` | 11 |193|
| Collection | `bun nx run @semio-tech/framework-space-collection-rs:test-snapshot-sqlite-native --skip-nx-cache` |11|196|
| History | `bun nx run @semio-tech/framework-os-kernel:test-space-history-sqlite-native --skip-nx-cache` |12|198|

Space/Collection package `📋️project.json` routes to package `📜️script.ts test-snapshot-sqlite native`; script4 supplies its actual Cargo package and Source test file to runArtifactRustPackageMain. Shared router `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts:77` selects --lib sqlite_snapshot_ --no-fail-fast. Kernel package `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:2212` selects --lib sqlite_snapshot_framework_space_history_ --no-fail-fast; commands2738–2739 mount Native/Source. Launch192–199 supplies all/native/source gates. Source names are the corresponding Space/Collection -source target and framework-os-kernel:test-space-history-sqlite-source. None was executed here.

Each actual test module is owner `🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs`. Existing laws cover factory capability, ordinary native equality, both erased directions, independent edit/renumber, public file metadata/route, row/schema admission, cancellation and cumulative reconstruction/native-input ownership. No test in these three modules calls preflight. Space/Collection native-input ledger test87–93 checks semantic cumulative retention; History365–429 additionally compares allocation/semantic debit. These are not output or allocator proofs.

## Concrete Meaningful Laws

1. Extend each existing language-neutral SQLite fixture with both-encoding preflight cases: sufficient bounds, actual output bytes minus one, row census minus one, schema length minus one, early and known interior cancellation. Call the actual public owner trait. Adequate limits must succeed; insufficient cases assert typed kinds. Compute ordinary comparison bytes outside measured intervals. A conservative preflight need not equal exact output length; avoid imposing an invented exactness contract.

2. Retain the existing independent Bun SQLite oracle: execute handwritten schema, insert every authored row, integrity/foreign-key check, exact UTF-8 BLOB lengths, independently edit the declared literal, and reconstruct the actual full owner. Preserve both native encodings and erased/public file equality. Oracle `🪐️space/🧪️tests/🪶️sqlite/🔬️oracle/🟦️.ts:17–20` supplies all-cell, independent-writer, edits and BigInt high/low word laws; Rust oracle4–13/40 supplies real independent renumber/edit. Do not replace complete state equality with category flags or source prose.

3. For borrowed preflight, prepare source outside measurement, run with zero physical allowance under a genuine existing System allocation observer, and assert zero new backing if borrowed admission is the intended contract. Assert source unchanged afterward outside measurement. Confirm observer mount first; counters alone are not allocator evidence. No ABI sizes should be assumed.

4. Add output budget law at actual owner encoder, especially History: both encodings with max_allocation_bytes=0 must refuse before output construction. With sufficient allowance, retain first payload and measure actual backing plus caller allocation debit. Reuse the exact same controller with a ceiling between one and two measured operations, keeping first payload live: second must refuse, first still decodes to the complete original owner. Assert monotonic debit, real progress and typed cancellation/ownership kinds. Pair erased codec import separately with its reconstruction and subset-validation costs genuinely included; never reset the caller ledger to make the test pass.

Current source already accounts Space/Collection output through shared allocation_stage. That does not grant allocator/drop coverage. Preserve History's genuine input ownership laws while repairing output. Missing public preflight does not imply inserting a new erased dispatcher call without an owning assertion. No universal runtime status is asserted.
