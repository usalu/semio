# Puzzle 3D Example Snapshot Preparation

The example worker emits one mutation per deleted or created object, attraction, target volume, reference and compatibility row, plus the domain and catalog changes. The existing worker law requires that exact sequence. Store folds the rows into one `Edit`, retains each mutation's inverse and metadata, and commits one history entry.

Puzzle 3D `Puzzle3dArtifactStorePreparation::advance_prepared` computes each inverse against that item's prebase and runs `apply_diff_cold`. This copies the Play header, which shares its typed document through an `Arc`. The previous implementation then deep-cloned that typed document through `Arc::make_mut`. For ordinary sparse diffs, `Puzzle3dDiff::apply` deep-cloned it again under `PreparedField`. It also cloned every vector before rebuilding each changed list. The semantic phase returns default retained progress and records one work unit, so these clones are not a direct debit against the copy stock. The observed copy-stock exhaustion at artifact item 155 establishes a fixture allowance mismatch. Snapshot read return, displaced snapshot retirement and the other staged batch receipts still draw on the allowance.

## Scoped Change

The typed diff and its Arc-backed Play bridge now share preparation of one validated candidate. The Play bridge calls the sealed `apply_shared` entry point. A unique Arc keeps the typed `PreparedField` install path. A shared Arc retains the original pointer through all fallible work and installs its prepared replacement once. Disposing a displaced Arc uses `Arc::into_inner` so the typed retirement hook receives the value only when there is no other strong owner. `MutationDiff<Puzzle3dSnapshot>` has no `retire_projection` override, so its cold hook and ordinary drop of a refused candidate are equivalent. Play invalidates its projected value cache only after a successful apply.

The candidate applies changed lists directly to borrowed source fields and copies unchanged lists once. An artifact replacement still supplies the base for its overlays. The first fallible field order remains meta, objects, attractions, target volumes and references. `list-delta::Parts::commit_onto` still clones the rows of a changed Vec and rebuilds the list, so this reduces redundant allocation while leaving the old cold algorithm's per-row cost.

## Store Boundaries for One Batch Candidate

| Interface | Implication |
| --- | --- |
| `StoreLive::batch_item_base` and `SnapshotRead<P>` | Every item after the first takes the staged `Arc<P>` into a new immutable read and registry lease. The factory receives only that lease. Neither exclusive mutation nor consuming the staged root is available there. |
| `ArtifactStoreBatchItemRequest`, `ArtifactStoreBatchSource::begin_next`, `ArtifactStoreOneItemPreparationRequest` and `OneItemOwners` | Each item gets its own mutation, read, Store authority and retirement issuers. Closure and refusal paths preserve and retire those owners. These APIs have no private mutable batch slot. |
| `ArtifactStoreOneItemLiveAuthority::next_edit` and `prepare_one_item` | They mint one forward and its inverse rows under authority. `ArtifactStoreOneItemPrepared` seals the item edit and snapshot addresses. Reusing a mutable post behind a prior seal would violate its immutable candidate contract. |
| `StoreLive::original_proposal_forward` and `advance_prepared_foreign_steps` | A foreign proposal can still need the exact pre-item snapshot after preparation and before folding. Generic reuse must complete that read before writes or retain the existing path for any mutation whose capability allows it. |
| `StoreOwners::fold_batch_item` and `ArtifactStoreBatchStage` | The fold transfers original forward and inverse rows, stamps per-row metadata and replaces `stage.post` with the new `Arc`. It retires the old staged post before starting the next item. Final commit validates the total rows and one aggregate digest. |
| `protocol::apply_diff_cold_into`, `Store::fold_operation_into`, `ColdReplayProjection` and `ColdReplayEdit` | The central cold applier can target an exclusive projection, and cold replay provides a precedent for in-place applying and typed retirement. Their contracts do not admit a mounted publication, preserve one item's prebase after applying, or bound the Vec work. Puzzle's own typed diff can still allocate a whole candidate. |
| `commit_retained_diff` and the paged Store semantic edit traits | The funded path takes an actual retained diff and original source plus an unpublished destination. Puzzle 3D has no retained diff fragment for these dense vectors. Merely swapping preparation factories would repeat the same per-item owner model. |

A full one-scratch route needs Store ownership changes. A batch stage would fund creation of one unpublished candidate from its original read, then retain it exclusively. Each operation would compute its diff, inverse and any foreign outputs from the current candidate before mutation. It would apply one funded typed patch at a time, preserve the original rows and metadata in the existing aggregate edit, and seal one final post root for the existing revision-validated commit. Fault and cancellation would close the candidate, unapplied source, original read and staged edit through their original issuers. Success would preserve the existing one-entry Undo and Redo law. The candidate must never be exposed as a registered snapshot read while mutable. A bounded Puzzle Vec editor or a retained paged domain representation is also needed to remove row-wide allocation and shifts. The scoped change does not implement that route.

## Focused Regression

`play_pack_tests::play_snapshot_diff_keeps_shared_source_and_cache_until_success` uses the existing JSON fixture with a `serde_json` oracle. It tests a successful sparse edit, exact first error target when two fields would refuse, unchanged original pointer and cache on refusal, unchanged source after success, and replacement overlays through the unique Arc path. The snapshot package has no allocation observer dependency, so this regression checks ownership and applicability. The removed allocation paths were verified by source inspection.

The focused native law passed against the freshly built snapshot and diff libraries. `verify-3d-snapshot-warm.py` resolves the exact dependency fingerprints from snapshot metadata and compiles a small Rust harness using the existing test source. It ran one test successfully with two filtered out and no plugin recompilation. The scheduled Cargo test exited before launching when the shared editor-gesture build failed on a missing test-module import. Both production crates had compiled by then.

The standard Cargo command uses the shared 3D cache after the build completes and with at least 8 GiB of free storage.

```bash
puzzle_ticket="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/GET-PUZZLE-ARTIFACTS-WORKING-END-TO-END/🗑️generated"
CARGO_HOME="$puzzle_ticket/cargo-home" \
CARGO_TARGET_DIR="$puzzle_ticket/cargo-puzzle-clipboard" \
CARGO_BUILD_BUILD_DIR="$puzzle_ticket/cargo-puzzle-clipboard-build" \
ZIG_GLOBAL_CACHE_DIR="$puzzle_ticket/zig-clipboard/global" \
ZIG_LOCAL_CACHE_DIR="$puzzle_ticket/zig-clipboard/local" \
RUSTC=rustc RUSTC_WRAPPER='' RUST_MIN_STACK=134217728 CARGO_PROFILE_DEV_DEBUG=0 \
cargo test --locked --offline -j1 \
  --manifest-path '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.toml' \
  -p semio-s-artifact-puzzle-3d-editor-snapshot --lib -- \
  play_pack_tests::play_snapshot_diff_keeps_shared_source_and_cache_until_success --exact
```
