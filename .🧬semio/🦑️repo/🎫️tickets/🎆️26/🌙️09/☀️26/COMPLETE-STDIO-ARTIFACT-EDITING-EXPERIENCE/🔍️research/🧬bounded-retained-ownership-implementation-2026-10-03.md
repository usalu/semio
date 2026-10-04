# Bounded Retained Ownership Implementation — 2026-10-03

## Implemented cut

This cut establishes a surviving Store close owner for interrupted retained-clone publications, mounts borrowed DOCX native preflight into the production SQLite snapshot trait, and adds page-owned octet and OPC carriers without changing the existing `OpcPackage` contract used by XLSX.

## Production wiring

### Store driver retirement handoff

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🚚️handoff/🦀️.rs` owns retained cursors during scheduler transfer and checks the terminal witness before taking the cursor.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` exposes `handoff_batch_publication_close`, reserves the Store maintenance queue before taking the scheduler owner, and drains the transferred retirement through Store maintenance.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` calls the handoff on mounted durable cancellation, stale completion, and terminal error paths.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏯️tool-run/🦀️.rs` transfers replaced and aborted publications to Store maintenance.
- `interrupted_retained_clone_publication_handoffs_to_store_maintenance` is a real destructor probe over the production Store path.

### DOCX native SQLite preflight

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧩️native/🦀️.rs` routes encode, input, and preflight through the shared controlled backing.
- `...📸️snapshot/🧩️native/💰️backing/🦀️.rs` measures with borrowed `OpcNativeWriter`, creates no encoded output during preflight, and pays allocation, row, physical-output, progress, and cancellation frontiers through `SqliteSnapshotControl`.
- `...📸️snapshot/🪶️sqlite/🦀️.rs` implements `ArtifactSqliteSnapshot::preflight_sqlite_snapshot_encoding`, so the production trait no longer returns the default `UnsupportedOwner` refusal.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🧩️native/🦀️.rs` now classifies both encoded and decoded OPC row-ceiling exhaustion as `WorkLimit`, matching `SqliteSnapshotControl::check_rows`. Physical file and value ceilings remain `OwnershipLimit`.

### Page-owned Value and OPC infrastructure

- `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs` adds non-contiguous and fallible cold constructors plus logical Debug/Eq laws for `PagedList`.
- `🧰️framework/🔨️modules/🌱️value/📦️paged/🦀️.rs` adds `PagedBytes`, controlled contiguous codec materialization, and fallible ordered-map construction. Octet ownership grows in separately admitted 4 KiB pages.
- `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️paged/🦀️.rs` adds bounded retained clone and explicit retirement for `PagedBytes`.
- The paged owner fixture/schema now covers a 6,145-byte octet owner, structured-clone equality, multi-turn copying, interruption, and terminal retirement.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🧬️retained/🦀️.rs` adds `RetainedOpcPackage`, page-owned parts, content-type entries, relationship lists, owner directory, paged text, and paged part bytes. Its controlled materialization pays collection, string, byte, relationship-directory, progress, and cancellation frontiers.
- Existing `OpcPackage`, `OpcPart`, `OpcContentTypes`, relationship ordering, helpers, and encoders keep their semantic contract. The foreign SQLite ownership ticket replaced the conventional package's guessed `BTreeMap` backing with the explicit ordered `OpcRelationshipOwners` authority; retained OPC consumes that public owner representation.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust/📜️script.ts` and `📋️project.json` register the independent `retained-opc-check` Nx target. The neutral schema and fixture are under `📦️opc/🧬️retained/🧫️fixtures`.

## Validation evidence

### Fresh green Store destructor law

Command:

```text
NX_DAEMON=false CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE/🗑️generated/retained-cursor-handoff-target" CARGO_BUILD_BUILD_DIR=same bun nx run @semio-tech/framework-os-kernel:test --skip-nx-cache -- interrupted_retained_clone_publication_handoffs_to_store_maintenance -- --nocapture
```

Result: 1 passed, 0 failed, 1,255 filtered; Nx cache explicitly skipped. Log: `🗑️generated/store-driver-handoff-native-fresh.log`.

The earlier `store-driver-handoff-native-green.log` was an Nx cache replay and is not used as runtime evidence.

### Fresh green paged-owner neutral oracle

Command:

```text
SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE/🗑️generated" bun nx run @semio-tech/framework-os-kernel:retained-clone-check --skip-nx-cache
```

Result: success, cache skipped, `pagedBytes=6145`. Log: `🗑️generated/paged-bytes-neutral-oracle.log`.

### Fresh green retained OPC neutral oracle

Command:

```text
SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE/🗑️generated" bun nx run @semio-tech/stdio-zip-rs:retained-opc-check --skip-nx-cache
```

Result: success, cache skipped, `parts=2 payload=6145 owners=2`. Log: `🗑️generated/retained-opc-neutral-oracle-green-2.log`.

### Native retained OPC law

Command:

```text
NX_DAEMON=false CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE/🗑️generated/retained-opc-target" CARGO_BUILD_BUILD_DIR=same bun nx run @semio-tech/stdio-zip-rs:test --skip-nx-cache -- retained_opc_copy_and_materialization_preserve_package_authority -- --nocapture
```

The initial run found an incorrect fixture-relative path, test-only imports, and eleven ZIP tests that referenced the removed `dsl::json` module. Those sites now use `semio_framework_pack_json` with `JsonMemberPolicy::Reject`.

A fresh rerun compiled the production and lib-test targets and started exactly one law. The law failed after 99,999 retained-clone turns at `retained OPC copy must terminate`; it did not reach copied-authority or retirement assertions. Result: 0 passed, 1 failed, 81 filtered; Nx cache skipped. No retained-copy runtime pass is claimed. Log: `🗑️generated/retained-opc-native-current.log`.

### DOCX native preflight

The isolated command was blocked after compiling the new preflight source by concurrent DOCX mutation-oracle compile drift. Log: `🗑️generated/docx-borrowed-preflight-native.log`. Later DOCX18 reached runtime with 154 of 156 laws passing. The borrowed-preflight law exposed the OPC row classification mismatch (`OwnershipLimit` returned for `max_rows: 0`); the writer and reader now return `WorkLimit`. This correction has not been rerun. The other DOCX18 failure concerns an independently owned missing-styles fixture. Log: `🗑️generated/docx-native-current-18.log`.

## Exact remaining production limits

1. `DocxSnapshot` still stores `OpcPackage` plus `Vec<DocxXmlPart>`. The production DOCX preparation still measures against the roughly 1 MiB one-item envelope and executes `let mut post = base.clone()` in phase zero.
2. DOCX Store import, mutation, save, and retirement do not use `RetainedOpcPackage`. Therefore the additive OPC owner is infrastructure, not an end-to-end DOCX scalability completion.
3. XML documents still use recursive `Vec` and `String` owners. A canonical paged XML owner and addressed retained mutation path are still required before the DOCX preparation cap and whole-snapshot clone can be removed honestly.
4. `RetainedOpcPackage::try_from_package` is a cold consuming transition from the current codec package and does not expose progress or cancellation. Production codecs should construct the retained owner directly; this transition must not become the Store hot path.
5. Controlled retained-to-codec materialization supports progress and cancellation, but the current OPC encoder still accepts `OpcPackage`; no retained streaming encoder is wired.
6. The retained OPC native law reaches runtime but its copy cursor does not terminate within 99,999 admitted turns. The copy driver needs a state-progress audit before this owner can be used in DOCX Store paths.
7. The DOCX borrowed-preflight row classification correction is source-coherent but has no post-correction runtime result in this lane.
8. The ticket and associated goal remain open as required.

## Post-audit retained-owner checkpoint

This section supersedes the earlier DOCX authority and nonterminating-copy statements. `DocxSnapshot` and `DocxArtifact` now store `RetainedOpcPackage`; DOCX preparation clones that retained OPC owner incrementally, and the registered editor installs a DOCX-specific `DocumentStoreOwners` catalog. XML parts remain recursive `Vec`/`String` owners and retain their bounded whole-clone path.

### Generic owner placement and retirement

- `PagedList::adopt_reserved` moves an already-copied `T` into an admitted page slot. It charges one item and zero additional copied bytes; physical page reservation remains a separate capacity operation.
- The derived retained-clone scaffold refuses a zero-item/zero-capacity grant before beginning a copy that could never retire its scaffold. Successful turns cannot repeat zero progress indefinitely.
- `PagedBytesCursor` reserves its destination page separately, then copies octets in bounded byte batches under `maximum_copy_bytes`. A 64-byte grant remains viable and does not require one 4 KiB page-sized copy grant.
- Retirement cursors expose the next indivisible physical release demand. A wholly zero-progress turn whose complete byte grant is smaller returns deterministic `WorkLimit`; a partially productive outer turn returns its credited progress and retries the physical release with the next full grant.

Fresh uncached evidence:

- `🗑️generated/paged-owner-liveness-native-2.log`: nine selected laws passed, 122 skipped, Nx cache skipped. The set includes the generic `sizeof(T) > copy budget` owner-adoption law.
- `🗑️generated/derived-scaffold-refusal-native.log`: one selected derived-scaffold refusal law passed, 130 skipped, Nx cache skipped.
- `🗑️generated/paged-octet-single-item-native-6.log`: one selected 6,145-byte copy/cancel/materialize/retire law passed, 131 skipped, Nx cache skipped.

Final byte law command:

```text
NX_DAEMON=false CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE/🗑️generated/retained-byte-target" CARGO_BUILD_BUILD_DIR=same bun nx run @semio-tech/value-rs:test --skip-nx-cache -- --lib paged_octets_copy_and_retire_by_credited_bytes_under_single_item_grants -- --nocapture
```

### Retained OPC and DOCX production wiring

- `RetainedOpcPackage` owns paged parts, metadata, relationship groups, text, and octets. Its conventional materialization now adopts a first-party `OpcRelationshipOwners` group buffer directly rather than estimating or copying standard-library B-tree nodes.
- `DocxSnapshot.opc` and `DocxArtifact.opc` are retained owners. Import/native/SQLite boundaries consume or reconstruct that owner, while conventional codec/export boundaries materialize an `OpcPackage` under a caller-owned control.
- DOCX preparation carries a `RetainedOpcPackage` cursor and an explicit owner catalog. The shared editor macro calls `preparation::document_store_owners`, so the production Store receives the DOCX preparation and retirement owners.
- `retained_opc_store_route_cancels_publishes_saves_and_retires_large_owner` enters through the registered editor factory, installs its owner catalog, cancels an in-flight publication, drains the Store close path, publishes and saves a 1.25 MiB part, validates the saved archive with third-party `zip::ZipArchive`, reopens it, retires the reopened owner catalog, and closes the Store.
- `🗑️generated/docx-retained-check-2.log` is an uncached successful DOCX production check from the retained-field checkpoint.

`🗑️generated/retained-opc-native-green-10.log` is the fresh current-source OPC result after the explicit `OpcRelationshipOwners` cutover and final physical-release demand changes. Nextest run `0290c3c0-dc8d-45ba-9caa-c8c05a25e7c4` passed the selected law 1/1 with 81 skipped and Nx cache explicitly skipped. The unchanged 64-byte grant covers multi-turn copy, interior cancellation, controlled conventional materialization, interrupted cursor close, and final retained-owner retirement.

The three DOCX lifecycle attempts in `docx-retained-opc-store-lifecycle-{2,3,4}.log` reached the registered production Store route but exceeded 99,999 preparation turns under the former one-octet-per-turn cursor. Attempt 5 reached final Store disposal and exposed a nested-grant boundary: after spending part of a 1 MiB outer grant, the Store passed the remainder even when it was smaller than the retained owner's next indivisible physical release. The generic Store cursor now ends that productive outer turn before the next exact demand; a complete insufficient grant still receives the retirement cursor's typed `WorkLimit`. `DocxSnapshotRetirement` also advertises its active OPC demand before its later XML remainder.

`🗑️generated/docx-retained-opc-store-lifecycle-6.log` is the fresh current-source production result. Nextest run `a7760374-e86f-40db-b4d3-507847707f09` passed the selected law 1/1 with 157 skipped in 4.38 seconds; Nx cache was explicitly skipped. The law proves registered Store cancellation and drain, retained publication and mutation, save through the current codec, third-party ZIP validation of the 1.25 MiB owner, reopen, standalone owner-catalog retirement, and final Store disposer terminal emptiness.

### Remaining scalable-path limits

1. XML authority remains recursive `Vec`/`String`; preparation clones XML parts as capped whole owners. Large XML needs a paged canonical owner and addressed mutation route.
2. Cold `OpcPackage` to `RetainedOpcPackage` import is consuming and controlled at reconstruction boundaries but is not a resumable Store cursor. Production import should construct retained fields directly when its codec becomes streaming.
3. DOCX save, native, SQLite, and diff boundaries still materialize a complete conventional OPC package. They support bounded refusal/progress/cancellation but are not retained streaming encoders.
4. Store initialization still passes through generic whole snapshot encode/fold paths outside the retained preparation cursor.
5. DOCX XML parts remain the next production authority cut: a shared flat retained XML owner must replace the recursive clone and one-turn remainder retirement while preserving the existing thirteen-table semantic vocabulary.
6. The ticket and associated goal remain open.


### Shared flat retained XML foundation

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧬️retained/🦀️.rs` adds a shared flat XML owner with paged text, nodes, attributes, boundary nodes, DTD declarations, and entities. Elements retain authored attribute order plus `first_child`/`next_sibling` links over postorder node ordinals.
- `validate` rejects out-of-range ordinals, malformed attribute ranges, invalid forward sibling links, and multiply owned nodes before publication. `materialize` is explicitly controlled and remains a conventional compatibility boundary rather than a Store hot path.
- The neutral fixture/schema and native law live beside the retained snapshot module. The law enters through the shared parser, compares controlled materialization with the source document, validates the emission independently with `quick_xml`, copies under the same one-item/64-byte grant, interrupts and closes an interior cursor, and retires the copied owner to terminal emptiness.
- `🗑️generated/retained-xml-native-2.log` is the fresh current-source result. Nextest run `71d5f26a-67c3-417e-be9d-a0bbac0e12b9` passed 1/1 with 95 skipped; Nx cache was explicitly skipped.

Command:

```text
NX_DAEMON=false CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE/🗑️generated/retained-xml-target" CARGO_BUILD_BUILD_DIR=same bun nx run @semio-tech/stdio-xml-rs:test --excludeTaskDependencies --skip-nx-cache -- --lib flat_retained_xml_copy_materialize_and_retire_preserve_shared_vocabulary -- --nocapture
```

This foundation is additive. `DocxXmlPart.document` and `DocxSnapshot.xml_parts` still own recursive `XmlDocument`/`Vec` values, and DOCX preparation still clones that remainder as one bounded whole owner. DOCX save/native/SQLite also still materialize conventional XML/OPC structures. The next authority cut must move DOCX XML parts to the retained graph and route preparation, mutation, save traversal, and retirement through it; this result does not claim that end-to-end XML migration.
