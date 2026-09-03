# Stdio Native-Codec Receipt And Catalog-Root Post-Implementation Audit

Read-only source audit, 2026-09-03. No production or test source was changed, and no build, test, script, component, or hub was run. Statements below are static-source findings only.

## Decision

**REJECT for trusted-native-codec admission and for treating the owner descriptor pair as completed authority.**

The in-progress code makes a valuable safe change: it refuses to manufacture a receipt from the 26 runtime artifact assemblies. `linked_native_codec_bindings()` in `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` remains `Vec::new()`, which is the correct fail-closed hub state. Do not populate it yet.

It is nevertheless not a native-codec receipt. The descriptor source declares six codec rows, all non-executable, whereas the factory table contains 26 independently selected native functions. `native_codec_factory_receipts()` therefore always returns an error and returns **zero** receipt rows. The current test asserts that withholding behavior. It proves that the unsafe inference is refused; it does not establish the required descriptor-declared executable-codec ↔ exact-factory bijection.

The new raw+core producer fixes the older audit's specific raw-only core-hash design, but its two owner files and the separate fresh-root row have no single crash-safe, reader-atomic commit point. The bounds, cancellation, and cache-isolation mechanisms are useful preconditions, not a trusted publication transaction.

## Exact static census

### Descriptor capability side

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧬️schema/📜️artifact-definition.json` is the only stdio artifact-definition with non-empty `codecs`. It contains these six exact rows:

| Codec id | `from` → `to` | status | executable registration |
| --- | --- | --- | --- |
| `s.stdio.gltf.standard.2.0.codec.inference-binary.v1` | source dialect → source dialect | `unimplemented` | `false` |
| `s.stdio.gltf.standard.2.0.codec.inference-text.v1` | source dialect → source dialect | `unimplemented` | `false` |
| `s.stdio.gltf.standard.2.0.codec.artifact-export.v1` | source dialect → source dialect | `unimplemented` | `false` |
| `s.stdio.gltf.standard.2.0.codec.artifact-import.v1` | source dialect → source dialect | `unimplemented` | `false` |
| `s.stdio.gltf.standard.2.0.codec.mutation-binary.v1` | source dialect → source dialect | `unimplemented` | `false` |
| `s.stdio.gltf.standard.2.0.codec.mutation-text.v1` | source dialect → source dialect | `unimplemented` | `false` |

`Source::Codec` in `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs` has only `id`, `status`, `from`, `to`, and `executable_registration`. It has no artifact-kind, representation/document-schema, pack-schema-hash, extension, stable factory identity, or operation-capability set. The same registry's ledger test encodes `declared={ codecs: 6, mutations: 3, inferences: 67 }`, `registered={ codecs: 0, mutations: 3, inferences: 67 }`, and zero implemented/verified counts. The 70 real executable identities are the three glTF mutations and 67 inferences, not document codecs.

Thus the exact descriptor-declared executable native document-codec set is **empty**. It is not the 26 runtime assemblies, nor six glTF rows, nor an inference/mutation service set.

### Native factory side

`native_codec_factories()` in the same registry source independently names 26 factory symbols:

```text
ply_codec, stl_codec, las_codec, dxf_codec, mp3_codec, xlsx_codec,
tiff_codec, jpg_codec, avi_codec, png_codec, csv_codec, md_codec,
docx_codec, mp4_codec, json_codec, gltf_codec, bcf_codec, zip_codec,
xml_codec, deflate_codec, obj_codec, pdf_codec, pptx_codec, step_codec,
dwg_codec, svg_codec
```

They produce factory candidates for artifact keys `ply`, `stl`, `las`, `dxf`, `mp3`, `xlsx`, `tiff`, `jpg`, `avi`, `png`, `csv`, `md`, `docx`, `mp4`, `json`, `gltf`, `bcf`, `zip`, `xml`, `deflate`, `obj`, `pdf`, `pptx`, `step`, `dwg`, and `svg`. Static source also partitions all 36 artifact roots into 26 `Runtime` assemblies and these ten exact definition-only negatives:

```text
binary, txt, ifc, gif, bmp, semio, wav, epw, tsv, html
```

The definition-only set is correctly excluded from the factory table. That is a necessary structural rule, but it is not descriptor capability authorization. `plugin()` then advertises the 26 factory-selected entries as `PluginManifest.artifact_kinds`; this is a third, independently assembled set. A manifest artifact-kind is not a codec declaration, and the present schema cannot prove it has a read/write codec factory behind it.

### Receipt behavior and identity gap

`NativeCodecFactoryReceipt` has promising output fields: plugin id, package id, artifact kind, codec schema, non-zero pack hash, extension, four capability-id lists, and `fn() -> ArtifactCodec`. `instantiate()` does re-create the codec and compares schema, extension, and pack hash.

But no `NativeCodecFactoryReceipt` is constructed. `native_codec_factory_receipts()` first proves only this weaker relation:

```text
runtime artifact assembly keys == native factory artifact keys == manifest artifact-kind keys
```

It then counts the schema rows and intentionally errors because there are six declared codec rows, zero executable registrations, and no source field binding one row to one factory/document schema. Consequently none of the receipt's capability-id vectors is populated or checked against descriptor evidence.

There is a separate identity problem even if construction were enabled: the receipt hard-codes `package_id == "semio:stdio"`. Cargo metadata in `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` does declare component package `semio:stdio`, but `PackageDescriptor` and `PluginManifest` in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` carry no canonical `packageId` field. The strict registry validator likewise validates `manifest.pluginId`, role, version, and hashes but cannot validate a descriptor package id. A Cargo constant or a plugin id is not a descriptor attestation and must not satisfy the hub bundle's package-id join.

The current shared descriptor emitter also calls `dsl::from_dsl_value` / `to_dsl_value` on `PackageDescriptor`, while that type still derives only serde in the current source. This is the known structural value-codec blocker, not a runtime finding. Until the package-descriptor codec work lands, the described producer path has no static basis for a successful compilation claim.

## Hub boundary

`🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs` expects an explicit `NativeCodecBinding` keyed by plugin id, package id, artifact kind, and native codec schema. It checks bundle component/descriptor bytes and hashes, the descriptor's plugin id/version/raw hash, manifest artifact kinds, bundle native-codec rows, codec schema, and pack-schema hash before it stages global codec registration.

That loader cannot repair the missing provenance:

* it does not receive a codec-row id or factory identity;
* its `validate_descriptor` cannot compare `record.package_id` to a descriptor field because none exists;
* it treats `manifest.artifact_kinds` as the declared native-codec set, even though stdio currently fills that list from the factory table; and
* a binding derived from that table would turn the exact unsafe inference that the receipt function rejects into authority.

Therefore the empty hub binding is correct. Any non-empty stdio binding today would either have no descriptor-declared executable codec counterpart, or would falsely promote an unimplemented/false glTF row or an assembly-only kind.

## Catalog-root producer reinspection

### What changed relative to the two earlier audits

The earlier catalog-root audit correctly rejected a one-input raw component emitter that assigned the core digest from raw bytes and reported no root producer. Current source has materially improved that boundary:

* `describe_component` now requires `raw` and independently supplied `core` inputs; `artifact_hashes` rejects equal SHA-256 values.
* `CatalogRootScript` creates a caller-supplied empty root, uses a private `CARGO_TARGET_DIR`, builds raw stdio, extracts an explicit core with JCO, calls the two-input emitter, stages raw/core/descriptor, and runs an independent WebCrypto/Pack/WASM validation before its local receipt comparison.
* Static limits are 64 MiB per artifact, 64 KiB copy/hash chunks, and a 1,200,000 ms deadline. The copy and verification loops check cancellation/deadline between chunks. The script rejects a non-empty/ambient target/dev-cache root, symlink/non-regular artifacts, missing core, oversize input, and a size-changing source. Its fixture includes distinct raw/core and rejected raw-only-substitution vectors.

These are source-level improvements only; this audit ran none of them.

### Remaining authority defects

1. **No atomic observable pair.** Both `atomicDescriptorPair` in stdio's `📜️script.ts` and `write_descriptor_pair_atomic` in the shared Rust emitter rename pack and JSON separately. A reader or a process crash can observe new pack/old JSON, old pack/new JSON, only one file, or cleanup/rollback residue. `fsync` is performed on individual new files but not on the containing directory. The name "atomic descriptor pair" is therefore stronger than the filesystem transaction actually supplied.

2. **No atomic relation between owner and fresh row.** The script first renames `stageRoot` to `<fresh-root>/stdio`, then writes the owner pair, then regenerates and validates the repository registry. A crash/cancellation/failure boundary can leave a staged raw/core/pack row unrelated to the currently readable owner pair. Those trees may live on different filesystems, so cross-root rename cannot make them one transaction. The outer rollback is best-effort, not durable recovery.

3. **Owner-pair snapshot accepts pre-existing invalid states.** `snapshotDescriptor` reads pack and JSON independently and `restoreDescriptor` preserves a pack-only or JSON-only snapshot. The authoritative producer should refuse to start unless the old owner state is a valid canonical pair or both files are absent; it must never restore an incomplete pair as a valid prior state.

4. **TOCTOU remains in copy/read paths.** The script checks metadata/path containment and subsequently opens/reads the source by pathname, rechecks size only, and does not pin a file identity across the read. This is not safe against a concurrent same-size path replacement. A private fresh root narrows the expected actor set, but it is not a proof. The shared emitter has the equivalent metadata-then-open sequence.

5. **Cancellation has one synchronous post-cancel gap.** Cancellation is polled while children wait/copies run, but the later `atomicDescriptorPair`, registry generation, audit, and strict verifier use no common durable commit token. `runControlled` observes cancellation around spawned work, while synchronous owner publication itself cannot be interrupted or proven all-or-nothing. It should be legal to cancel only before an immutable evidence commit, never after exposing a partly updated owner pair.

6. **Fresh cache isolation is good but not a release receipt.** The dedicated target/work/stage folders and ambient-cache rejection are appropriate. They do not create a verified hub bundle or solve the missing package-id/factory relation. The printed JSON is a CLI result, not a signed or immutably addressed catalog record.

## Smallest safe remediation packet

Land this before any stdio hub binding or trusted-bundle row.

1. **Make one schema-owned codec contract.** Extend the artifact-definition schema (and `Source::Codec`) with a native document-codec row containing at least:

   ```text
   codecId, executableRegistration=true, artifactKind,
   representationId, artifactSchema, extension,
   factoryId, readCapabilityIds, writeCapabilityIds,
   mutationCapabilityIds, inferenceCapabilityIds
   ```

   `artifactSchema` and capability ids must use existing canonical identities; `factoryId` is a stable non-public source identity, not a Rust symbol exposed in descriptor JSON. Require every listed representation/artifact/capability to exist and every list to be sorted, bounded, and duplicate-free. Do not retroactively declare all 26 candidates: begin with only factories whose document codec semantics and exact schema are intentionally modeled. The desired initial set may be empty; zero is safer than a guessed migration.

2. **Generate the receipt from one exact join.** In `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs`, make a private table map `factoryId` to its sole `fn() -> ArtifactCodec`. Construct a receipt only after exact equality of all of:

   ```text
   executable schema codec ids
   == native factory ids
   == descriptor native-codec declarations
   == bundle native-codec declarations
   ```

   Instantiate every factory during preflight and compare artifact kind, schema, extension, non-zero pack hash, and all capability ids. Reject duplicates, missing rows, declaration-only rows, runtime-assembly-only rows, and definition-only artifacts. Remove the factory-derived `manifest.artifact_kinds` authority shortcut or replace it with a distinct descriptor-native-codec projection; artifact-kind availability alone must remain non-executable.

3. **Close package identity before joining the hub.** Add one required canonical `packageId` to the packed `PackageDescriptor`; derive it from the guest/component contract, then compare it against Cargo component metadata during strict source generation, the factory receipt, the bundle record, and `PackageRef`. Do not infer it from `pluginId`, Cargo package name, crate path, or a hard-coded receipt literal. Complete the owned `PackageDescriptor` value codec first, including a neutral pack/JSON round-trip oracle, rather than retaining a serde bridge.

4. **Publish immutable evidence at one location.** Make the fresh row the only authoritative completion object, e.g. an immutable `<fresh-root>/stdio/<generation-digest>/` containing raw, core, canonical descriptor pack, canonical descriptor JSON, and an exact receipt; fsync files and directories, then create one final completion marker containing all hashes. Readers accept only a fully verified immutable generation with that marker. The owner-root pair is a checked-in derivative generated only after that evidence is complete and must be non-authoritative while publishing. This avoids pretending that two owner filenames plus a separate root can be an atomic cross-filesystem transaction. Add durable recovery that removes/ignores unmarked generations.

5. **Keep hub binding empty until the proof exists.** Only a generated static composition projection may turn verified receipt rows into `NativeCodecBinding`s. It must preflight the exact package id, component digest, descriptor byte digest/self-hash, artifact/schema/pack hash and capability relation before the trusted loader's assembly transaction. No lookup by file extension, app, assembly type, runtime registry, Cargo filename, or client input is permitted.

## Required verification assets (for the implementation owner)

Use one language-neutral fixture/oracle and Rust-only factory assertions:

* A fixture with one intentionally executable codec and one declaration-only codec. Node/built-in crypto plus the canonical Pack implementation recomputes the descriptor self-hash and generation digest, verifies sorted duplicate-free rows, and rejects package/factory/schema/pack-hash/capability mismatches.
* Rust builds the same fixture receipt, invokes the one authorized factory, and proves the `ArtifactCodec` schema, extension, and pack hash agree. A definition-only `binary` or `txt` fixture, a 26-runtime-assembly-only candidate, an unregistered glTF codec row, and an undeclared factory must each produce no receipt/binding.
* Publication tests must inject failure before marker, after raw, after core, after descriptor pack, after descriptor JSON, and during owner mirroring; independent readers may accept only the completed immutable generation. Exercise cancellation/deadline/oversize/path replacement and prove neither a hub binding nor a completion marker is published.
* The hub fixture must prove that an empty receipt leaves `linked_native_codec_bindings` empty and a non-empty bundle cannot load; then prove one valid generated binding loads and every package-id/descriptor-byte/schema/pack-hash mismatch fails before global registration. Do not make a runtime claim until this is executed.

## Blocker order

1. **Critical:** zero descriptor-declared executable codecs versus 26 factory candidates; no bijective receipt exists.
2. **Critical:** descriptor has no canonical package id, while the intended hub/factory join needs one; current receipt hard-codes it.
3. **High:** `PackageDescriptor` has no owned value codec despite the two-input emitter requiring it, so no successful producer compilation is established by source inspection.
4. **High:** owner JSON/Pack and fresh row are independently published, not one crash-safe reader-visible authority record.
5. **Medium:** pathname metadata/read/copy TOCTOU and cancellation/commit gaps remain; private roots reduce exposure but do not prove authority.
6. **Correct current boundary:** retain the empty hub binding and the receipt function's explicit failure until blockers 1–4 are resolved.

## Files inspected

* `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs`
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🧬️schema/📜️artifact-definition.json`
* `✏️s/🔌️plugins/🗄️stdio/🦀️.rs`
* `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` and `📋️project.json`
* `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🧪️fixtures/catalog-root/{🧬️.schema.json,🔣️.json}`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/{🦀️.rs,📜️script.ts}`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
* `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`
* `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs` and `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
* Earlier ticket audits `📓️terra-stdio-catalog-root-completion-audit.md` and `📓️terra-trusted-native-codec-openable-catalog-audit.md`.
