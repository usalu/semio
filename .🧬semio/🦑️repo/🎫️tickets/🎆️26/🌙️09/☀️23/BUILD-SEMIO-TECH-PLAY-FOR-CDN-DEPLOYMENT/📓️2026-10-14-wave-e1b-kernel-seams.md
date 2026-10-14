# 🪡️ Wave E1b: os-kernel Store seams, engine codec constructors, owning paths, sealed authority, 4096 bound (2026-10-13/14)

Continues `📓️2026-10-13-wave-e1-os-kernel.md`. Work ran on 10-13; a session restart on 10-14 killed the final verification runs.
On restart the tree had moved about 24 h, and a Codex fleet (ticket `26/10/13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES`) is cutting the
kernel into small crates (`os-store-envelope`, `-command`, `-cursor`, `-projection`, `-message-ledger`, `os-history-*`, `os-vcs`, …).
Every E1b edit below is still in the tree; the fleet built its cuts on top of it (for example `pub mod store_schema`,
`pub mod snapshot_read`, `document_store_owners`, `bare_artifact_codec`).

Rules kept: no git-modifying command, no worktree, no sub-agent, no ticket operation, `🗑️generated` untouched, every cargo call in the
foreground through `🚦️cargo-slot.sh` with private dirs under `⚡️cache/e1b` (deleted at the end). Inputs live in `e1b-kernel-seams/`.

## 1. Outcome

| Task | Result |
|---|---|
| 1. Close the 5 store seams (19 refs) | **Done.** S1 constructors and helpers moved to the engine; S2 `DurableStorePreparedOwnerV1::unbound` replaces the inherent impl on `ArtifactStoreOneItemPrepared`; S3+S4 the owner catalog family (`DocumentStoreOwners`, `MemberStoreOwner`, unscheduled completion, bounded catalog constructors) moved to the engine module `document_store_owners` (`🏗️initialization/🏠️owners/🦀️.rs`); S5 `ArtifactPack::publish_native_snapshot` deleted, `io::publish_native_snapshot::<P>()` publishes the declared `native_snapshot_registration`, 6 duplicated impls deleted. Not re-measured with a new HIR dump (23 min, and the kernel no longer builds today, see §4). |
| 2. `ArtifactCodec::of/bare` → engine fns | **Done.** `store::artifact_codec::<P, M>` / `store::bare_artifact_codec::<P, M>` in the store root's CodecRegistry region (engine crate). 296 call sites in 200 files rewritten, including the stdio and root `📜️script.ts` generators/scanners and the plugin send fixture. The sha-locked `fixture-channel-interfaces` original source is unchanged. |
| 3. Remove interim root re-exports | **Done.** All 592 E1 re-export entries (42 lines, 18 modules + `document_store_owners`) deleted; the modules are `pub mod` (`edit_retirement` `pub(crate)`). Consumers use the owning path: external `store::X` → `store::os_store::<module>::X` (same rule for `semio_framework_os_kernel::`, `directory::`, `protocol::`, `dsl::`, `vcs::`, `kernel::`, `__kernel::`, `fixture_store::`, …), in-kernel `crate::X` → `crate::os_store::<module>::X`. 2,284 Rust files, 12,464 qualified paths, 1,071 use-tree entries; 132 occurrences in 36 TS/JSON scanners, generators and schemas. The root keeps private `use <module>::{..}` imports only. |
| 4. Shrink `pub(crate)` | **Measured, then structural fix for the authority.** Narrowing all 821 E1 widenings and re-widening exactly what rustc demands in 5 configurations returned 820: E1's widenings were already the rustc minimum for this module structure (705 production, ~115 test-only; 395 of them fields). The live authority, seal and prepared candidate now keep **all 19 fields private**: engine and tests read accessors (186 reads rewritten), the Store mints through `ArtifactStoreOneItemLiveAuthority::mint` / `rebind_group` (14 literals), takes candidates apart through `into_parts()`, and tests tamper only through `#[cfg(test)]` seams (`sealed_parts_mut`, `rebind_identity_for_law`). The authority's trait impls moved under its owning module (`🎮️one-item/📬️publication/🪪️authority`). The source law `one_item_publication_source_denies_generic_or_unbounded_shortcuts` still holds. |
| 5. Restore the 4096 history-read bound | **Done in source; law passed on 10-13.** The two oversized releases were the snapshot-registry Arc frame (10,496 B) and the history page stack's last page plus its directory chunk (5,664 B = 1,552 + 4,112). Fixture, Rust test and TS oracle are back to 4096. The census after the change has max demand 4096 (chunk) and 1552 (page). |

## 2. Paging (task 5)

- **Registry** (`🔗️read/🦀️.rs`): the lease slot table is 8 pages of 128 slots (`SnapshotReadSlotPages`, 3,072 B each). The generation words are 2 pages of 512 (`SnapshotReadGenerationPages`, 4,096 B). The Arc frame drops to about 2.4 KB. Every page size is a const-asserted ≤ `SNAPSHOT_READ_RELEASE_PAGE_BYTES`.
  - A shared registry releases slot pages one per step (`close_empty_backing_step`).
  - Generation pages are read lock-free, so they leave only after the registry is exclusively owned (`owned_backing_demands` / `close_owned_backing_step`, used by both last-custody retirements).
  - The Store's frame phase now quotes `custody_byte_demand` / `close_custody_step`: the frame first, then each generation page.
- **History page stack** (`🌿️vcs/🦀️.rs`): `ARTIFACT_HISTORY_DIRECTORY_PAGES` is derived from `ARTIFACT_HISTORY_RELEASE_PAGE_BYTES = 4096`, which gives 510 on 64-bit and 1022 on wasm32. One chunk is exactly 4096 B (const-asserted).
  - When the last page of a chunk is released, the page is detached first: the chunk leaves in one step, the page in the next. `capacity()`-based emptiness checks stay exact.
- **Tests adapted to multi-step release:** backing tests and registry-return tests loop over pages, step bounds are derived from the page constants, and the independent-Arc oracle drives the owned pages.

## 3. Verification (all on 10-13, after the last source edit of that day)

| Check | Result |
|---|---|
| kernel `--lib` native, `--features sync,ureq`, wasm32-wasip2, wasm32-unknown-unknown | green, 8 kernel warnings each (= baseline) |
| kernel `--profile test`, `--tests` (incl. both integration tests) | green |
| targeted lib tests (registry, backing, return, history read/plan, alias, bounded history read) | all pass, `bounded_history_read_cursors_obey_the_neutral_law` at 4096 |
| root workspace `check --workspace --lib --keep-going` (after the owning-path rewrite) | only `semio-framework-plugin` (peer in-flight edits) and `semio-framework-os-kernel-db` (pre-existing `SharedUtf8`/`PackError` drift) fail; no path or re-export error |
| full lib suite one-process-per-test (1470 tests listed), hub wasm workspace check, `🔎️manifest-sweep.sh` | **not run**: the test binary was built and the suite was about to start when the restart killed the session |

Not compilable on 10-14: `semio-framework-job` and `semio-framework-ui-theme` (fleet edits 15:23) fail upstream of the kernel. So nothing could be re-verified on the moved tree.

## 4. Reconciliation with the Codex fleet (10-14)

- Their crates (`os-store-envelope`, `-command`, `-cursor`, `-projection`, `-message-ledger`, history, directory, vcs, command, pack, inference, identity, channel) supersede my 20-crate plan for those layers. E1's `🎯️target-crates.json` is now only valid above them.
- Store files edited by the fleet in the last 3 h: the root, `🔗️read`, `🔁️replay`, `📒️ledger`, `🧬️schema`, `📦️codec`, `🚪️io`, `🧩️composition`, `🧾️document`, `🎮️one-item`, `👥️presence`. Their audit names "exact original read/retirement ports" as the next Store cut. So I made **no** further Store edit today.
- Skipped and recorded:
  - The remaining `pub(crate)` field exposure (395 fields): it needs accessors in modules the fleet is actively cutting.
  - The fresh seam re-measure.
  - The stale TS oracle path: `🧪️tests/🧪️supersede-replay/🟦️.ts` reads `../../🧫️fixtures/…`, but the fleet moved the corpus to `🧪️tests/🧫️fixtures/` (TS file touched 14:34).
- **Open decision for the cut:** `ArtifactStoreOneItemLiveAuthority::mint` is `pub(crate)`. If the authority stays in a base crate while the engine mints in another crate, `mint` must become reachable across crates. That needs a capability (engine-owned mint token), or the authority, seal and prepared types must move into the engine's crate. Do not make `mint` plain `pub`.

## 5. Pre-existing failures seen (not caused here)

- Kernel `--profile test --features sync`: 58× `DemoMutation: ArtifactCanonicalJsonTree` unsatisfied, plus a stale `crate::os_store::pack_rt` import in `🔄️sync/🧪️tests/🔬️unit`.
- wasm32 `--features worker`: `👷️worker/🦀️.rs:43`, `decode_request` became async.
- The `🪪️authority` TS law expected `crate::os_spr::` while the source has said `crate::command::` since HEAD. The token was fixed in the TS while moving the file.

## 6. Files

- **Kernel:**
  - store root `🦀️.rs`
  - `🏗️initialization/🏠️owners/🦀️.rs` (new)
  - `🔗️read/🦀️.rs`, `🔗️read/🔒️registry/{🦀️.rs,♻️retirement/🦀️.rs}`, `🔗️read/🧾️return/{🦀️.rs,🧪️tests/🦀️.rs}`
  - `📦️codec/🦀️.rs`, `🚪️io/🦀️.rs`
  - `🎮️one-item/📬️publication/🦀️.rs`, `🎮️one-item/📬️publication/🪪️authority/` (moved from `🧵️canonical-edit/🪪️authority/`), `🧵️canonical-edit/🦀️.rs`
  - `🧩️composition/🗄️durable-group/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`, `📦️prepared/♻️retirement/{🦀️.rs,🧪️tests/🦀️.rs}`
  - `🧾️document/📜️history/💧️hydration/🦀️.rs`, `🧬️schema/🧬️mutations/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
  - `♻️retirement/📦️backing/🧪️tests/🦀️.rs`, the supersede-replay fixture/test/oracle
  - `📦️codec/🪶️snapshot-capability/{🪶️native-decoding,🪶️native-encoding,🪶️native-retirement}/🧪️tests/🦀️.rs`
  - the canonical-edit, snapshot-clone, paged-one-item and unit test files
  - `🌿️vcs/🦀️.rs`, os `🚪️io/🦀️.rs`
- **Consumers:**
  - 200 files for the codec constructors.
  - 2,284 Rust files for owning paths: hub, ✏️s plugins, framework crates, os products.
  - 36 TS/JSON files, including root, stdio, space and puzzle `📜️script.ts`.
  - 6 snapshot impls (workflow run, dag, collection, space, flow, wgpu renderer + its `🪶️sqlite`).
- **Inputs (kept):** `e1b-kernel-seams/` holds `🏗️cargo.sh`, `🔍️item-graph.py`, `🚚️relocate.py`, `🧱️rust_items.py`, `🔀️reexport-move.py`, `🔁️fix-loop.py`, `🔁️owning-paths.py`, `🗒️owning-modules.json`, `🗒️kernel-files.txt`, `🔽️narrow-visibility.py`, `🔑️authority-accessors.py`, `🪙️authority-mint.py`, `🗒️headers/`. `e1-os-kernel/🎯️target-crates.json` gained the `document_store_owners` engine rule.
