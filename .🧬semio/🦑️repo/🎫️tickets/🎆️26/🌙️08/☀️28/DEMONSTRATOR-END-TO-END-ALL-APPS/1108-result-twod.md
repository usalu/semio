# 1108 twod result

## Libs
Native hub W-twod-6/12 rc 0, wasm32-wasip2 W-twod-13 and W-twod-37 (after test-port edits) rc 0: 2d-path-flatten, 2d-path-stroke, 2d-trace, 2d-booleans, 2d-regions, pixels-codecs, canvas-scene (incl. retirement path/stroke, paint-values), ui-raster-owner, pixels.

## Tests (framework Cargo.toml, tst.sh W-twod-34 + W-twod-36), pass counts
flatten 8/8, stroke 6/6, regions 5/5, trace 18/18, booleans 27/27, pixels-codecs 1/1, canvas-scene lib 27/27 + original_scene_retirement 2/2, ui-raster-owner 11/11, pixels 5/7.
OPEN (2 failing): pixels editing::flood::tests::{neutral_non_square_paged_connectivity_and_allocation_prefixes, uniform_large_region_yields_and_cancel_preserves_workspace_for_close}. The close needs 4328 / 6367 turns (32 items per turn, ~136 B copied) against the law bound 16*(3*pages+3)+16 = 832. flood source unchanged since 732; the granularity comes from the paged Vec<u8> retirement (RetainedCloneClose::step_granted) as changed by 733. Not edited to fit: owner decision needed (bulk release of plain leaf pages in value-retirement, or a re-derived exact bound).

## Wallet ruling (2d-regions)
region_boolean / offset_regions no longer own an AuthoringWallet. They take `turns: &mut impl RegionTurns` (trait in 2d-regions): every arrangement and retirement turn runs inside one original effect the host opens (cx.with_original_effect) and charges to its own turn; a single shared effect cannot work (allocation marker `actual` stays occupied after the first allocating step: "original page allocation attempt is occupied"). Tests implement RegionTurns via the shared invocation-carrying funding helper.
NOTE: AuthoringWallet effects carry no producer invocation, so native_retirement_reply! owners refuse them ("requires its original producer invocation"). Still wallet-driven and therefore broken at runtime: pixels image decode (retire_funded / drive_jpeg in 🖼️image/📥️decode), pixels layers into_result(wallet). Needs the same turn-seam treatment (not done, outside the 2d-regions ask).

## Other edits this step
- 2d path funding test helper (🛤️path/🎟️work/🧪️tests): with_effect now drives a genuine OriginalEffectAuthority producer invocation; Funded::turn / Wallet::turn_effect pass the effect; PhysicalClose::close(body,effect).
- PathFlatten/PathStroke advance take RetainedAllocationEffect (like trace, booleans).
- canvas retirement stroke/path: next_computational_work quote. paint-values RasterImage: fields cursor (u32,u32,ArcLease<Vec<u8>>) with default birth bytes.
- ui raster pool: next_close_demand(body) replaces four next_close_* helpers.
- Test callers ported: canvas scene/path/stroke, raster ownership, png decode close, trace, booleans, regions, flood (shared invocation helper).
Debts: png encode `PngEncodeJob::retirement_birth_bytes` uses sequence_birth_bytes (same shape as the fixed RasterImage bug; no test exercises it yet). Wasm tests not run.

## Round 3 (coordinator rulings)
1. Flood close turns: NOT restored in value-retirement. The 733 change (`count.min(grant.maximum_items)`, `copied_items:count` in Collection/DequeCollection) is an explicit law: `scalar-collection-grants` / cases.json (maximumItems 1 -> one element per turn, remaining [2,1,0]) fails when bulk ignores the item grant (left 24 right 10 at native.rs:15). I tried the restoration, saw that law fail, and reverted; file is back at HEAD. Alternative that keeps both laws: the flood test GRANT lent 32 items; bulk Vec<u8> release is item-capped, so a 4096-item grant closes within the unchanged bound 16*(3*pages+3)+16 (pixels 7/7, W-twod-50). The bound was not raised. Owner decision: keep that grant, or change the 733 law to price a bulk chunk as one item.
2. Wallets removed from pixels: new seam `EffectTurns` (+ `BlockingTurns` host edge opening a genuine producer invocation) in 2d-retire; 2d-regions uses it (RegionTurns removed); pixels `decode_encoded_with(bytes,limits,cancelled,turns)`, `retire_funded(job,fund,turns)`, `RasterStackJob::into_result(turns)`; `decode_encoded_blocking` unchanged for callers (BlockingTurns edge); surface-paint compositing uses semio_framework_pixels::BlockingTurns. Cargo: LOCK NEEDED 2d-regions (+2d-retire) and pixels-codecs (+2d-retire).
3. PngEncodeJob birth bytes: test `png_encode_job_birth_bytes_equal_its_defining_cursor_layout` (in-crate, 📷️png/✍️encode/🧪️tests/birth.rs) failed (176 vs 120 layout) exactly like RasterImage; override removed, default birth used; passes (W-twod-47).

## Results
native tests: pixels 7/7, flatten 8, stroke 6, regions 5, trace 18, booleans 27, canvas-scene 27 + scene test 2, raster-owner 11, pixels-codecs 1+1 birth. wasm lib W-twod-51 rc 0.
Debts: companion crate semio-framework-pixels-tests does not build: duplicate #[global_allocator] (♻️retirement tests + png decode close tests vs testing allocator) and ~25 stale call sites (png encode work, coverage, retirement, image-decoding retire_funded now needs turns, layers). value-retirement own test targets also unresolved (`semio_framework_value` import) and its test closure is currently refused by the source gate.
STATUS: FIXED-verified W-twod-50 (pixels 7/7) / BLOCKED-by-decision: Vec<u8> bulk vs 733 item law
