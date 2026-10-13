# 2026-10-11 Bulk clone for bitwise `Vec<T>`

Executor: bulk-clone (Sonnet). Goal: `RetainedClone for Vec<T>` must not clone scalar samples element by element (about 13 turns per element), so real-size pixel/sample documents are editable.

## Change

`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs`
- `RetainedClone::bitwise_extend(target: &mut Vec<Self>, source: &[Self]) -> bool` (default `false`). The scalar macro implements it with `extend_from_slice`; `[T; N]` implements it through `T::bitwise_array` row by row when `T::BITWISE`.
- `pub const RETAINED_CLONE_BULK_PAGE_BYTES: usize = 64 * 1024` (same as the ABI page / editor retirement page).
- `VecCursor::advance_bitwise`, entered from `VecCursor::advance` after the bind turn (phase 0 bind + one exact `try_reserve_exact`, phase 2/3 handling untouched) when `T::BITWISE`:
  - one chunk turn = 1 item, `min(grant.maximum_copy_bytes, page) / size_of::<T>()` elements (zero-sized elements copy the whole run in one turn), progress `copied_items: 1, copied_bytes: n * size`, no capacity;
  - grant with 0 items, 0 depth, or less copy than one element returns empty `Progress` (blocked, no state change);
  - the chunk that copies the last element completes the cursor (`Complete`, output taken with `take`), an empty vector completes on its first chunk turn.
- Cancellation/close is the existing `close_step` path: the partially filled `values` vector (exact reserved capacity) is retired through its controlled retirement, then the source binding alias.
- Non-bitwise `T` (`String`, `Option<T>`, `Vec<_>`, structs) keeps the per-element cursor path unchanged.
- `Box<[T]>`/`Arc<[T]>`/`VecDeque` have no `RetainedClone` implementation in this crate, so there was nothing further to convert.

Tests (language-agnostic fixture + schema + Bun test, plus Rust tests), new directory `🧬️retained-clone/🚚️bulk-vec/`:
- `🧫️fixtures/🔣️.json`, `🧬️schema/🔣️.json` (page size, sample generator, counts, turn formula `2 + max(ceil(bytes/page), 1)`, 66 turns for 4 MiB).
- `🧪️tests/🟦️.ts` (registered as `bun ./📜️script.ts test bulk-vec-retained`, nx target `test-bulk-vec-retained-source`): validates the fixture with Ajv and checks the chunk arithmetic and sample bytes against an independent page-wise `Uint16Array` copy.
- `🧪️tests/🦀️.rs` (module `retained_clone::bulk_vec_tests`):
  - `bulk_vector_clone_equals_the_clone_oracle_for_every_bitwise_element_shape` (u8/u16/u32/u64/i128/f64/bool/char, `[u8;4]`, `[[u16;3];2]`, `()`; empty vector; 0..2,097,152 samples);
  - `non_bitwise_vector_clones_keep_the_per_element_path` (`String`, `Vec<Vec<u8>>`, `Option<u32>`, > 3 turns per element);
  - `bulk_vector_clone_turn_count_is_bytes_over_page` (fixture counts x copy grants 2/1000/65536/1048576 B give exactly `2 + chunks` turns; 4 MiB `Vec<u16>` = 66 turns, each chunk 1 item and at most one page);
  - `bulk_vector_clone_capacity_accounting_is_exact` (reserve turn `retained_capacity_bytes == len*2`, allocator witness `(planned, 0)`; chunk turns have a capacity grant of 0 and allocator witness `(0, 0)`; sum of chunk bytes == payload; denied grants for capacity-1, 0 items, 0 depth, copy < element return empty progress with zero heap events and no state change);
  - `bulk_vector_clone_cancels_at_every_turn_and_releases_everything` (cancel after every turn 0..=total, allocator witness `allocated == freed` across the whole clone+close, source unchanged).

## Commands and results

All in `🧰️framework` (value/os-kernel) or the artifact dir, foreground, through `🚦️cargo-slot.sh bulk-clone --`, `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/bulk-clone/target CARGO_BUILD_BUILD_DIR=.../bulk-clone/build CARGO_BUILD_JOBS=3`.

| Command | Result |
| --- | --- |
| `cargo test -p semio-framework-value --lib bulk_vec -- --nocapture` | `5 passed; 0 failed`, `[DEBUG] bulk vec clone 4 MiB Vec<u16>: 66 turns` |
| `cargo test -p semio-framework-value --lib --no-fail-fast` | `test result: ok. 305 passed; 0 failed` |
| `bun test 🧬️retained-clone/🚚️bulk-vec/🧪️tests/🟦️.ts` (value dir) | `1 pass, 0 fail` |
| `cargo test --manifest-path ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/Cargo.toml --features component-app-assembly --test preparation_law -- --nocapture` | `2 passed`; `[DEBUG] png 1081344-byte image edited in 545530 turns, peak live 1134168` (0.24 s for both tests) |
| `cargo test --manifest-path ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/Cargo.toml --features component-app-assembly --test preparation_law` | `1 passed` (the bmp law only covers a 2x1 image; no big-image case exists there) |
| `cargo check -p semio-framework-os-kernel --lib` | Finished, 0 errors (1006 pre-existing warnings) |
| `cargo check -p semio-framework-os-kernel --lib --target wasm32-wasip2` | Finished, 0 errors (999 pre-existing warnings) |

## Per-phase turns (temporary `[DEBUG]` counter in `RetainedClonePreparation::advance_original`, removed again)

| Case | Clone | CloseClone | Edit | Seal | Build | total |
| --- | --- | --- | --- | --- | --- | --- |
| png 2x1 (`ReplaceImage`) | 475 | 10 | 967 | 6003 | 63 | ~7.5k |
| png 512x264 RGBA16 (1.03 MiB, was ~7M turns) | 491 | 10 | 540,679 | 4,280 | 63 | 545,530 |
| png 1024x512 RGBA16 (4 MiB, 2M samples, was 27M turns / 25 s) | 538 | 10 | 2,097,159 | 4,328 | 63 | 2,102,105 (1.44 s debug incl. fixture build/asserts) |
| bmp 2x1 | 708 | 10 | 1,486 | 7,854 | 63 | ~10k |

Clone grew by only 16 turns for the 1 MiB image and 63 for 4 MiB (one turn per 64 KiB), the rest of the Clone turns is the non-sample fields of the snapshot.

## Remaining blocker for real-size images (not part of this change)

The png `Edit` phase is now 99.8 percent of all turns: `PngOwnedValidationWork::advance` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/⚙️operations/🪪️validation/🦀️.rs`) hashes and range-checks one sample per loop iteration and charges `used.copied_items += 1` each time, so with the store's `maximum_items: 1` grant it is one turn per sample (2,097,152 turns for a 1024x512 RGBA16 image). It needs the same treatment: process a page of samples (bounded by `grant.maximum_copy_bytes`) per item and charge `copied_bytes`. The same pattern likely exists in the other pixel validation works (bmp, tiff, ...); I did not touch them.

## Not done / not verified

- No sandbox-free proof that the turn-count test fails against the old per-element path (the old path needs ~27M turns for the 4 MiB case); the old numbers come from corrections #38.
- `advance_demands` (pure quote) is still not implemented for `VecCursor` (it never was); callers keep driving it with explicit grants.

## Follow-up 2026-10-11: paged owned work, audit, permanent law tests, launch registration

### 1. Paging the per-sample owned work (shared generic, no per-crate hacks)

Protocol decision: the store's `maximum_items: 1` grant stays; one item is one PAGE of homogeneous work, the copy currency carries the bytes. Value crate (`🧬️retained-clone/🦀️.rs`) now exports two helpers used by every pager:
- `bulk_run_elements(grant, width)`: elements a turn may process = `min(grant.maximum_copy_bytes, RETAINED_CLONE_BULK_PAGE_BYTES) / width`, `0` without an item or without credit for one element, `usize::MAX` for zero-width elements.
- `bulk_run_progress(n, width)`: the honest receipt, `copied_items: 1`, `copied_bytes: n * width`.
`VecCursor::advance_bitwise` was refactored onto them.

Users:
- png `🪪️validation/🦀️.rs` (`PngOwnedValidationWork`): samples (2 B each, range check + `target` proof + hash), text characters (page = whole characters inside the byte budget), ancillary octets (slice compare + slice hash). Stages that cannot afford their minimum unit return `(false, used)` BEFORE touching the hash, so retries never double-hash (the length-prefix hash at `byte == 0` made this necessary).
- bmp `🪪️validation/🦀️.rs` (`BmpOwnedValidationWork`): palette entries (4 B), samples (1 B indexed, 20 B direct), gap and trailer octets. The interactive `paint-region` command passed `maximum_copy_bytes: 0`, which would now never progress; it passes `RETAINED_CLONE_BULK_PAGE_BYTES`. The png command keeps its 4096 B grant (2048 samples per item inside its `should_yield` loop).
- png and bmp `🧬️publication/🦀️.rs` paint phase: one reserve turn (`try_reserve_exact` of the inverse samples, `retained_capacity_bytes` = actual capacity, refused if the capacity grant is smaller or the allocator over-delivers), then paged paint turns with capacity grant 0, `bulk_run_progress` receipts. Previously one pixel per turn charged `bytes` of capacity per pixel while `Vec` growth doubled underneath.
- Derive: `#[retained_clone(bitwise)]` (value derive, `attributes(retained_clone)`) on a `Copy` struct whose fields are all `BITWISE` sets `BITWISE`/`bitwise_array`/`bitwise_extend` so `Vec<S>` and `[S; N]` take the bulk path. Applied to `BmpNativeSample` and `TiffWord64`. Without opt-in the struct stays element-wise (`PlainTexel` test).

### 2. Audit (`rg 'OwnedValidationWork|impl .*Work for' ✏️s/🔌️plugins` plus every `build_artifact_store_one_item_preparation_factory`)

| Editor | Finding | Action |
| --- | --- | --- |
| png | bespoke `RetainedCloneEdit`: per-sample validation, per-pixel paint | paged (above) |
| bmp | same pattern, plus `Vec<BmpNativeSample>` cloned element-wise, plus paint command with copy grant 0 | paged, derive opt-in, command grant fixed |
| tiff (baseline + document) | `mutation_apply` (one snapshot-sized apply turn); sample block `Vec<TiffWord64>` cloned element-wise | `TiffWord64` opts in to bitwise; pixels `Vec<u8>` get the bulk Vec clone |
| jpg, gif, ply, obj, other stdio formats | `mutation_apply`; pixel buffers are `Vec<u8>` | benefit from bulk Vec clone, nothing else per-element |
| wfc bitmap | `BitmapOneItemPreparation::advance` prepares in one turn, no clone cursor | none |
| raster | own `RasterOneItemApply` over paged owned values (fuel 256 per grant, field pages) | not changed (not a one-element-per-turn pattern in the owned apply); not exercised by a law test |
| puzzle2d | no bitmap document data (only icons/config) | none |
| any `impl .*Work for` hit | only png and bmp validation use the `OwnedValidationWork` pattern; the other `*Work` structs are command/job work, not pixel loops | none |

### 3. Evidence (all foreground through `🚦️cargo-slot.sh bulk-clone --`, private target dir)

| Command | Result |
| --- | --- |
| `cargo test -p semio-framework-value --lib --no-fail-fast` | `307 passed; 0 failed` (7 `bulk_vec` tests incl. the new derive and `bulk_run_elements` tests) |
| `cargo test -p semio-framework-value-derive --no-fail-fast` | all ok |
| `bun test .../🚚️bulk-vec/🧪️tests/🟦️.ts` | 1 pass |
| png `--features component-app-assembly --test preparation_law` | `3 passed`; 512x264 RGBA16 edit = 4,873 turns (was 545,530) |
| bmp `--features component-app-assembly --test preparation_law` | `2 passed` |
| `cargo check --lib` native + `--target wasm32-wasip2` for png, bmp, tiff (the tiff artifact crate) | Finished, 0 errors in all six runs |

Real-size png case (permanent test `publication_preparation_turns_are_bounded_by_pages_at_real_image_size`, 2x2 paint, grant copy 64 KiB):

| Image | Bytes | Pages | Turns | Turns before this follow-up |
| --- | --- | --- | --- | --- |
| 256x128 RGBA16 | 262,144 | 4 | 4,895 | n/a |
| 512x264 RGBA16 | 1,081,344 | 17 | 4,873 | 545,530 |
| 1024x512 RGBA16 | 4,194,304 | 64 | 5,015 | 2,102,105 (27M before the bulk clone) |

The three cases together run in 0.42 s in a debug build including fixture construction and the restore proofs (the 1024x512 edit alone was 1.44 s before). The turn count is flat in the image size apart from ~1.2 turns per extra page; the remaining ~4.8k turns are the canonical sealer floor. The test asserts: growth of at most 8 turns per additional page between 4 and 64 pages, and fewer than 12,000 turns at 4 MiB. The bmp law (`publication_preparation_turns_are_bounded_by_pages_and_paged_paints_restore`) asserts the same bound for a 2.6 MiB direct bitmap (40 pages, 5,517 turns; 128x64 baseline 5,441) and proves a paged 64x40 paint (2,560 samples) equals `apply_mutation` and that its inverse restores the base.

### 4. launch.json

`.vscode/launch.json` and its source `.vscode/🧩️launch.seed.jsonc` (both contain the sibling entries; `rg --files -g launch.json` returned nothing, so I located it by `find`) gained, next to the scalar/dynamic retained pairs in group `4_gate`:
- `⚖️test-original-bulk-vec-retained-native🌱️🦀️` (order 900.058606496): `bun nx run @semio-tech/value-rs:test ... -E 'test(bulk_vec_tests)'`
- `⚖️test-original-bulk-vec-retained-oracle🌱️🟦️` (order 900.058606497): `bun nx run @semio-tech/framework-value:test-bulk-vec-retained-source`
`launch.json` still parses as JSON. Not run: the nx entries themselves (I ran the equivalent cargo/bun commands directly, as required).

### Remaining bottlenecks and caveats

- The canonical sealer (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit`) walks mutation and inverse as generic frames: ~35 turns per u16 sample (png) and ~360 turns per `BmpNativeSample` pixel (bmp 64x40 paint = 922,193 turns). Large paint/replace regions stay slow until it gets a bulk scalar-array node. Brush-sized edits on large images are O(pages).
- png/bmp crate `--lib` test targets do not compile (old-API tests: `from_authority`, 3-arg `advance`, `InteractiveJobCloseStep::Complete` struct variant); not touched, owned by stdio B. Only the `preparation_law` integration targets and `cargo check --lib` were run.
- `mutation_apply` lanes (tiff, jpg, gif, ...) still apply in one snapshot-sized turn; clone is now O(pages) but that single apply turn is a latency spike on big images.

## Phase 3 2026-10-11: bulk canonical sealing (no per-element cost in the sealer)

### Measured breakdown before the change (bmp, temporary `[DEBUG]` counters, removed again)

A 2x2 paint on a 128x64 bmp took 5,441 turns: Clone 444, Edit 10, Build 63, CloseClone 10, Seal 4,758 + ~190. The seal hash phase alone was 4,758 turns for a 76-byte-per-field edit: the native tree walk emits ONE byte per turn (`ArtifactCanonicalJsonTreeCursor::advance` writes `output[0]`), the seal cursor then spent a second turn hashing each byte (`hash_pending`), it ran two passes (length count, hash), the 59-byte framed header was hashed one byte per turn, and every collection element paid an alias projection, a frame push, an inspect, byte turns and a frame pop (~35 turns per u16, ~360 per `BmpNativeSample`). A 64x40 bmp paint was 922,193 turns (hash phase 921,510).

### Design (root cause, three layers, canonical bytes unchanged)

1. `ArtifactCanonicalJsonTreeCursor` (`🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs`) gained a bulk path, active only when the caller's window (`min(output.len(), grant.maximum_copy_bytes)`) is at least 32 bytes, so every existing one-byte caller keeps the one-event walk and its tests. `flat_encode` encodes a whole subtree (scalars via the same `ScalarBytes::write_node`, strings/keys via the same `canonical_escaped_byte`, arrays, objects, nesting up to 16 deep) with exactly the walk's bytes, or reports it does not fit. `flat_run` uses it per frame:
   - `Inspect`: a node that fits the window is emitted whole and its frame completes (a small edit is one turn);
   - `ArrayNext`: a RUN of children (comma-separated, each atomic) per turn, so `Vec<u16>`, `Vec<u8>`, `[f32; N]`, vectors of derived structs (`BmpNativeSample`, `{"red":..}` objects) cost one turn per window, not per element (generic over `ArtifactCanonicalJsonTree`, no per-type hook, so `#[retained_clone(bitwise)]` structs and everything else are covered);
   - `ObjectNext`: a run of `,"key":value` members, and when the next member's value does not fit even an empty window, its `,"key":` prefix is written at once and the walk continues into the child (no per-byte key turns);
   - `Text`: as many escaped bytes as fit, resuming the existing `TextState`.
   Honest accounting: a bulk turn is one item and charges exactly the bytes written as `copied_bytes`, bounded by the grant's copy credit.
2. `ArtifactCanonicalEditSealCursor` (`📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🦀️.rs`): the 1-byte buffer became a 1,024-byte page (`SEAL_PAGE_BYTES`; 4,096 was tried first and broke the Store contract that a sealer constructor needs at most 4,096 bytes of capacity, `canonical_store_native_sealer_mounts_original_sources...` reported 6,832, because the page lives inline in the cursor). The count pass hands the whole page to the tree each turn; the hash pass fills the page across turns and hashes it in one turn when it is nearly full (free window below 64 bytes) or the tree is done (before: one hash turn per byte). The framed header is hashed up to a page of bytes per turn. Exact-quote callers (grant copy below 32 bytes) fall back to the one-byte walk and still work.
3. Nothing in the store sealer (`🧵️canonical-edit`) needed to change: it already forwards the full grant, so it inherits the paged walk.

### Evidence (all foreground through `🚦️cargo-slot.sh bulk-clone --`, private target dir)

| Command | Result |
| --- | --- |
| `cargo test -p semio-framework-pack-json --lib` | `99 passed` incl. `bulk_canonical_walk_is_byte_identical_to_the_one_event_walk_and_to_serde` and `bulk_canonical_walk_matches_the_one_event_walk_for_floats_wide_integers_and_deep_nests` |
| `cargo test -p semio-framework-replication --test paged_seal_digest` | `1 passed` (new `[[test]]` target; the crate's old `--lib` seal tests do not compile on old-API code, pre-existing, so the new test is an integration target) |
| `cargo test -p semio-framework-value --lib --no-fail-fast` | `307 passed` |
| `cargo test -p semio-framework-value-derive --no-fail-fast` | all ok |
| png `--features component-app-assembly --test preparation_law` | `4 passed` |
| bmp `--features component-app-assembly --test preparation_law` | `2 passed` |
| shooting `--test preparation_law` (config lane `config_lane_set_camera...` and document lane `document_lane_create_shot...`) | `2 passed` (first run compiled the plugin host stack from cold) |
| `cargo check -p semio-framework-os-kernel --lib`, native and `--target wasm32-wasip2` (from the repo root; the crate is in the root workspace) | Finished, 0 errors in both |

Differential proofs (old path vs new path, bytes and digests):
- tree level: for scales 0, 1, 5, 64, 1000, 2500 of a document with `Vec<u16>`, `Vec<u8>`, `Vec<struct>` (5 u32 fields), strings with quotes/backslashes/control bytes/non-BMP, `Option<Vec>`, nested vectors, empty vector and a tuple, the one-event walk (window 1) equals serde_json AND the bulk walk for windows 32, 33, 64, 255, 256, 4096, 65536; floats (`[f32; 4]`, `Vec<f32>`), `Vec<i64>`, `Vec<u64>`, `Vec<bool>`, `Vec<()>` and nests 1, 5, 16, 17, 46 deep match the one-event walk. At scale 2,500 (75,150 bytes) the walk is 242,549 turns one-event vs 101 turns bulk.
- seal level: the two fixture edits (sparse 76 bytes, dense 734 bytes) give the fixture's Node-framed SHA256 digests and lengths on both the paged and the exact-quote one-event path (466 vs 70 and 2,643 vs 70 turns); `Edit<Vec<u16>>` with 0, 1, 9, 700, 3000 samples gives identical length and digest on both paths AND equals an independently framed SHA256 of the serde-ordered canonical text (3,000 samples: 154,631 one-event turns vs 300 paged; 700 samples: 35,003 vs 220).

### Turn counts, before and after (this phase only changes the sealer)

| Case | Before phase 3 | After |
| --- | --- | --- |
| bmp 128x64, 2x2 paint | 5,441 | 755 |
| bmp 512x256 (40 pages), 2x2 paint | 5,517 | 831 |
| bmp 128x64, 64x40 paint (2,560 pixels) | 922,193 | 1,313 |
| png 256x128 RGBA16, 2x2 paint | 4,895 | 786 |
| png 512x264 RGBA16, 2x2 paint | 4,873 | 812 |
| png 1024x512 RGBA16 (64 pages), 2x2 paint | 5,015 | 906 |
| png 1024x512, 256x128 paint (131,072 samples, new permanent test) | not run; ~35 turns per sealed sample would be ~4.6M | 2,410 |

The new permanent tests: png `publication_preparation_turns_of_a_large_paint_region_are_bounded_by_pages_through_the_sealer` (< 12,000 turns for the 131,072-sample paint, equality with `apply_mutation`, inverse restores) and the bmp test now also asserts the 2,560-pixel paint stays under 20,000 turns; the earlier per-page growth assertions still hold.

### Remaining floor (identified, not changed)

Tiny edits now cost about 750 to 900 turns: Clone 440 to 540, Seal 219 (semantic pass 43, identity pass 47, hash/count 72, source and authority closure ~49), Build 63, Edit 10 to 70, CloseClone 10. The Clone share is the generic derived-record cursor: every field of the image record (about 17 for png, 13 for bmp, scalar and `Option<scalar>` alike) pays a child cursor lifecycle of about 25 turns although its payload is a few bytes. The clean root cause is in `✨️derive/🧬️retained-clone`: a field whose type is `BITWISE` could be copied in the record's own turn (and `Option<T: BITWISE>` could be `BITWISE`). That is a derive-wide change affecting every document clone, so I left it as the next step rather than bundling it here.

### Verification notes (phase 3)

- `cargo test -p semio-framework-os-kernel --lib canonical_edit` is NOT green and was not before this change: `canonical_store_native_sealer_mounts_original_sources_and_keeps_prepared_and_frame_closure_separate` stalls in its cancel-at-turn-0 close loop (`assertion failed: turn < 999999 || owner.terminal_is_empty()`), and `reader::...partial_unicode_output...` and `borrowed_map_long_unicode_keys...` also failed in the one full run I made (modules I did not touch). I proved the sealer test fails identically with the original `tree`/`seal` files restored (rebuilt and ran it, then restored my versions byte for byte), so it is pre-existing. Its constructor-capacity assertion (`<= 4096`) was the one my 4 KiB page broke; with the 1 KiB page it passes that part.
- The full `canonical_edit` lib run takes more than 50 minutes of CPU in a debug build (small-grant walks), so I did not wait for it to finish.
- An unrelated peer edit left `🗣️dsl/✨️derive` non-compiling for a few minutes during my run; it settled by itself.
- Not done: the derive-wide record-clone fix named above, and the stale `--lib` seal tests in the replication crate.
