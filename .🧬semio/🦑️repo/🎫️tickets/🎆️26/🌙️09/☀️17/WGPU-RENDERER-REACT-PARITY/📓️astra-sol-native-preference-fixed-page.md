# Native Preference Retained Document

## Corrected capacity diagnosis

The native `semio.os.config` path selects `OS_SHELL_CONFIG_MAX_BYTES` (64 KiB) and passes that value to `storage_worker_read_fixed_file_page` and `storage_worker_write_fixed_file_page`. The host-storage service owns one 16 KiB worker step and rejects any requested maximum above `STORAGE_FIXED_FILE_PAGE_BYTES` before opening the file. Native config reads therefore return `None` and writes return `false` for every document, including `{}`.

The correct repair preserves the 64 KiB logical document contract and the separate 4 KiB flat-field contract. The existing 16 KiB service bound governs work per step; it is not the public capacity of the complete `semio.os.config` value. The earlier ticket draft incorrectly reduced the document maximum to 16 KiB and has been replaced.

## Existing storage authority

The current service stores each fixed page as raw bytes at one caller-owned path. It uses ordinary `std::fs::File`/`OpenOptions` and has no page-set manifest, generation directory, or multi-file document encoding. Elsewhere in the repository, the rendezvous and pack I/O code already use the same single-value publication shape: write a sibling temporary file and rename it into place.

The narrow extension is therefore one retained single-file document service, rather than four physical page files:

- One logical document remains the raw value at the existing preference path.
- A retained cursor owns a unique sibling temporary file and an exact `(owner, generation)` token.
- Each `write_step` appends at most `STORAGE_FIXED_FILE_PAGE_BYTES` (16 KiB), so exactly 65,536 bytes require four steps.
- The service admits at most `STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES` (64 KiB) and refuses 65,537 bytes before creating or truncating anything.
- One destination has one active write generation. A newer generation supersedes the prior token; stale step, publish, cancel, and cleanup calls refuse.
- Cancellation marks the exact token non-publishable and removes only that owner's inactive file through bounded cleanup. It never modifies the committed destination.
- Publication first verifies the inactive file's exact expected length and completed-step state, flushes it, then performs a platform atomic replacement. Readers address only the committed destination, so they see the prior document throughout a partial write and the complete replacement after publication.
- Publication uses `std::fs::rename` directly after the inactive file is flushed and synced. The installed Rust 1.99 nightly's Windows backend implements this as `MoveFileExW(..., MOVEFILE_REPLACE_EXISTING)` and falls back on access-denied to `SetFileInformationByHandle` with `FILE_RENAME_FLAG_REPLACE_IF_EXISTS`; Unix uses the platform rename primitive. No new platform adapter is needed. Plain delete-then-rename would create a visible gap and remains forbidden.
- The 4 KiB standalone preference keys continue to use the existing one-page read/write functions unchanged.

This preserves the existing file encoding and avoids a parallel generation/manifest format. The four-page wording in the capacity audit is satisfied as four bounded 16 KiB worker steps over one inactive file. A future multi-file encoding is neither required nor specified by this contract.

## Neutral contract

`🔬️native-preference-page/🧫️fixtures/🔣️.json` is now schema version 2. Its strict schema pins:

- 16 KiB per write step, 64 KiB per config document, four maximum steps, and 4 KiB per flat field;
- exact owner/generation tokens, stale-token refusal, cancellation non-publication, and supersession;
- inactive temporary writes, exact-length validation, atomic replacement, and last-committed reads during a write;
- exact-boundary, plus-one, cancel-after-two-step, and supersede-after-one-step cases;
- truncated temporary, trailing temporary, and non-owner publication refusal.

The schema deliberately pins service properties rather than temporary filenames or a manifest encoding. The scoped Bun/Nx neutral law passed:

```text
5 pass
0 fail
28 expect() calls
Ran 5 tests across 1 file. [215.00ms]
```

The exact command was:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️native-preference-page/🧪️draft/🧭️neutral.🟦️.ts'
```

The fifth law is the independent Node filesystem oracle. It writes a nontrivial 65,536-byte buffer to an inactive file in four 16 KiB appends, verifies the prior destination after every append, replaces the destination with `node:fs.renameSync`, and compares the reopened bytes exactly. Its 65,537-byte attempt refuses before creating an inactive file and leaves the exact committed bytes unchanged.

## Registered native laws

The service law is registered in `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs` as `retained_fixed_file_document_preserves_exact_owner_cancellation_supersession_and_atomic_publication`. It requires:

1. one prior raw config document to publish and remain readable during every inactive step;
2. exactly 65,536 bytes to advance in four service-sized steps, publish atomically, and equal the system-file oracle;
3. 65,537 bytes to refuse before an inactive owner exists, preserving the prior committed bytes;
4. cancellation after two steps to refuse publication, drain to terminal, leave the exact committed bytes readable, and keep the cancelled token stale;
5. a newer generation after one stale step to invalidate the old token and publish only the replacement;
6. a different owner cannot reuse the current generation, and only a destination-wide strictly newer generation can supersede it;
7. truncated and trailing temporary files and non-owner publication all refuse while preserving the committed bytes.

The last review added two intentional runtime-RED assertions before Native68: the published generation remains stale for every owner, and the cancelled generation remains stale for both its original and a different owner. The current manager removes its only destination/generation row on publication and cancellation, so it can accept a delayed older token after a newer write completes. The repair must retain a per-destination terminal generation floor without scanning the owner table or changing any byte capacity.

The exact separate service gate for the RED and subsequent repair is:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/os-services-rs:test-quick -- retained_fixed_file_document_preserves_exact_owner_cancellation_supersession_and_atomic_publication
```

The coordinator-owned actual RED receipt was Nextest run `f2a0b9b0-dc26-409c-8579-73776b8a9740`: 0 pass / 1 fail / 44 outside the filter in 28 ms. It failed at the first post-publication replay assertion, `the published generation remains stale for every owner`. The initial redirected log was removed by concurrent generated-tree cleanup; this receipt comes from the coordinator's rerun on direct stdout.

The repair adds one exact `BTreeMap<PathBuf, u64>` generation floor to the retained manager. `begin_write` performs one destination lookup and refuses any generation less than or equal to the retained floor. The floor advances only after byte/page/path admission and inactive-file creation succeed, and it is deliberately retained when publication or cancellation removes the current owner. Active ownership and write registries remain separate and no table scan, capacity change, or file-format change was introduced.

The renderer source-route law is registered in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs` as `native_config_uses_the_document_lane_and_flat_fields_keep_the_fixed_page_lane`. It requires the Shell config branch to select the retained document lane while every non-config field retains the 4 KiB fixed-page lane.

The earlier held Rust files remain ticket inputs recording the fail-first design. The production names were aligned when the laws were promoted. No Cargo command was run locally; the root task owns all native compilation.

## Production integration

`🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs` now exposes a native retained document owner. `begin_write` admits the complete logical value before file creation, splits it into at most four 16 KiB pages, and assigns an exact owner/generation token. Each `write_step` writes one page. A destination accepts only a strictly newer generation, retires the superseded inactive file, and makes stale tokens terminal. Cancellation closes and removes only the inactive file. Publication requires all pages, exact expected length, flush and sync, then uses `std::fs::rename` to replace the committed raw file. The read lane opens only the committed destination and reads it in 16 KiB chunks up to 64 KiB.

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` routes `semio.os.config` through that document owner. A retained mutex serializes current ownership and an atomic counter assigns exact generations. The 4 KiB flat fields still use `storage_worker_read_fixed_file_page` and `storage_worker_write_fixed_file_page` unchanged.

Native67 did not provide a production-absent RED. The preceding coordinator instruction had already authorized production implementation before the later request to hold it. Native67 then encountered a source-timing seam: the service component was written at 02:58:56 while the run ended at 03:00:31, so the already-built service dependency omitted the new public APIs while the later renderer compile saw their callers. The package glue already exports `component::*`; no additional export layer was missing. Native68 is the first run able to compile the coherent service and renderer snapshot.

Native68 compiled that coherent boundary and executed the renderer census. Its coordinator-captured 1,247 pass / 10 fail / 0 skip receipt contains no preference failure; the registered renderer source-route law passed. The ten failures belong to the concurrently owned engine canvas, scene routing, pane ownership, and command-palette packets. A concurrent generated-tree cleanup removed the raw Native68 log after the coordinator captured the complete summary, so this report does not claim a surviving raw artifact. The os-services component law is not executed by the renderer target and remains a separate gate.

## Browser storage boundary oracle

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🗄️wgpu-host-storage-door/🟦️.ts` now proves the page host accepts, reads, and snapshot-carries exactly 65,536 bytes. A following 65,537-byte write refuses and leaves the exact committed value unchanged. The scoped Bun/Nx Vitest receipt is:

```text
Test Files  1 passed (1)
Tests       9 passed (9)
Duration    377ms
```

The exact command was:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bunx vitest run '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🗄️wgpu-host-storage-door/🟦️.ts' --config '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts'
```

All modified Rust sources parse through `rustfmt --edition 2021 --emit stdout`, and `git diff --check` is clean. The coordinator-owned full services gate verified the retained-document repair together with the native-I/O owner-lifecycle correction: Nextest run `4df2f746-2248-48d3-a42c-64163cfe6bc3` completed 45/45 PASS, 0 skipped, in 261 ms; Nx completed in 17.8 seconds. Receipt metadata is retained at `🗑️generated/astra-runtime/services-green-5/metadata/semio-nextest-YCOXQq`.
