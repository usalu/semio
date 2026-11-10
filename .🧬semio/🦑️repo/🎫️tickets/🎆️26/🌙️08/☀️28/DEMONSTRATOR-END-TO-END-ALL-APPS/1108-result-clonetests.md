# clonetests: value retained-clone test targets

Rerun (per target, native): `"$C/1105b-tools/runtime/tst.sh" W-clonetests-<n> 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/📦️packages/🦀️rust/Cargo.toml native -p semio-framework-value-retained-clone --test <name>`
Paged engine: same helper on `…/🧬️retained-clone/📋️paged-list/⚙️engine/📦️packages/🦀️rust/Cargo.toml -p semio-framework-value-paged-clone` (lib + `borrowed`).
Derive: `…/✨️derive/📦️packages/🦀️rust/Cargo.toml -p semio-framework-value-derive --test neutral_owner`.

## Green (native)
- original-paged-clone 7/7 (W-clonetests-24), original-collection-work 1/1, alias-work 1/1, array-prefix-selection 2/2, native-sorted-collection 4/4, original-bulk-vector-public 5/5, original-string-work 1/1, take 1/1, original-clone-lifecycle 4/4.
- paged-clone crate: lib 4/4 + borrowed 1/1 (W-clonetests-21, rc 0).
- bulk-logical-body: see final run W-clonetests-31 below.

## Production fixes (source)
- paged engine `finish_child` (📋️paged-list/⚙️engine/🦀️.rs): a child reply beyond its own grant (items/copy/capacity/release/depth) is now refused with `admit_retained_clone_progress` (kind InvariantViolated, borrowed REFUSAL message) before owner placement. Was silently accepted after the redesign.
- paged engine: `#[cfg(test)] placement()` accessor (index, values len, child_value) for the private-state laws.
- vec cursor `copy_count_at_with`: at least one element per turn, so a body narrower than the element width cannot stall a bulk copy (stall had produced zero-quote Progress turns forever).
- string cursor `selected_copy`: when rounding down to a UTF-8 boundary yields nothing, take the first whole char if its width fits the work (a body narrower than a char no longer stalls).
- retirement `Collection`/`DequeCollection` plain-bulk discard: reports one item per page run (`copied_items:1`) and no longer clamps the run to `maximum_items`; the clamp turned a page discard into one element per turn (the plain-elements law is "turns linear in pages, not elements", items:1 grants).

## Test ports / law updates
- Test-local cursors (`ReleaseProbe`, `InsufficientScaffoldChild`, DropProbe, Nonconforming*) implement `begin_close_into`, `native_terminal_release`, work quotes that include binding work (`binding_work_demand`), reply-shaped `advance_into`/`close_into`.
- Grants are widened to the current quote (`funded`) in loops; fail-fast stall asserts replace 100000-turn bounds (2000 turns, plus identical-turn detectors) in paged public/unit, bulk logical-body.
- Law updates (quotes equal actual cost, reply handoff included): spent-close 11 turns / 21 work (fixture + schema `♻️spent-close.json`); release turn copies exactly its reply handoff (`next_close_copy_byte_demand`); collection-work: a PagedList output-ready turn is Progress and the next turn is a zero-work Complete; string-work: when the quote is 0 raise the limit to the minimum affordable (binding check 5 + payload), one-byte payload turns cost exactly their quote; recursive/record neutral-owner turns bounded by `fits(grant)` instead of the old 4096 body.
- bulk-logical-body: copied-bytes law adds the header currencies the quotes now carry (reserve turn Vec header+1, one Vec header per page turn, publication demand); empty original skips the capacity reservation (overheadTurns - 1); owner retirement stepped with the same `body` it was quoted at (helper `step_granted` fixed BODY 4096 vs quote body 1 stalled); `[u8; 257]` uses at least index+element body (portable fixture `staged.json` row `body-refuses-first-byte` makes arrays refuse a smaller body).

## Derive neutral-owner (🧪️tests/🧩️neutral-owner/🦀️.rs)
- Ported to the reply-shaped API (effect helper via `#[path]`, `RetainedCloneBorrowAuthority::from_admitted_source` through `effect::admit`, `driven` for authority/owner close). Derive Cargo.toml dev-deps added (LOCK NEEDED logged).
- 2/4 green (neutral_record_vectors_match_independent_serde, borrowed_record_clone_matches_neutral_serde_and_exact_cancellation; W-clonetests-20).
- OPEN (production, not test): `paged_native_recursive_clone_uses_admitted_lazy_cursor_owners` and `paged_native_record_derives_preserve_neutral_serde_and_dsl_shapes` fail with "clone cursor lacks an original work quote": `PagedBytesCursor`, `PagedUtf8Cursor`, `PagedMapCursor` in `🧬️retained-clone/📦️paged/🦀️.rs` implement `advance_into/take/close_into` but none of `advance_work_demand`, `take_work_demand`, `close_work_demand` (only `begin_close_work_demand`). Quotes to add: bind = `source.binding_work_demand`; Bytes phase 0 reserve = `bytes.reserve_capacity_one_work_demand(len)`, copy chunk 1, publish 1; Utf8/Map delegate to the inner PagedList child (`nested` work) plus take/publish 1; close = `close.next_owner_work::<…>` mirroring each `close_demands`. Utf8/Map still use sync `take_child_output`/`begin_close_paid` paths (half-ported file), so an owner (paged/instance-b) should port it as one unit.

## Final run W-clonetests-31 (native, rc 0): bulk-logical-body 5/5, original-string-work, original-paged-clone 7/7, original-collection-work, array-prefix-selection, native-sorted-collection, alias-work, take, original-clone-lifecycle, original-bulk-vector-public all green.
STATUS: PARTIAL FIXED-verified W-clonetests-31 (all retained-clone value targets green); neutral-owner 2/4 BLOCKED on missing work quotes in 🧬️retained-clone/📦️paged cursors (see above)

## Paged cursors (coordinator follow-up, 11/09)
- `🧬️retained-clone/📦️paged/🦀️.rs`: PagedBytes/PagedUtf8/PagedMap cursors gained `advance_work_demand` (bind work; Bytes reserve via `reserve_capacity_one_work_demand`, copy 1, publish 1; Utf8/Map delegate to the inner PagedList child quote, take, child close start/close, publish), `take_work_demand`, `close_work_demand` (mirrors `close_demands` through `next_owner_work`) and `native_terminal_release`; `advance_demands` now treats a ready inner child as the take phase. The file was already reply-shaped (routed child calls); no sync path remained beyond the immediate-complete `take_child_output`.
- Performance fault found by the 17-level tree: quote recursion was exponential (engine `child_quote` called the child three times per level, nested derive/field/engine layers each doubled). Fixed with one-call selection: `PagedCloneElement::advance_selection`, engine `advance_selection` (advance_demands/advance_work_demand project it), `PagedListCursor` selection overrides, and generated derive cursors implement `advance_selection`/`captured_advance_selection` (derive `🧬️retained-clone/🦀️.rs`). Depth 17 now runs in under 2 s.
- Law update: native-owner fixture `cloneFrames` perLevel 5 -> 1, base 3 -> 4 (lazy cursors quote one frame per tree level; observed 5,6,7,21 for depths 1,2,3,17).
- Results: derive neutral_owner 4/4 (W-clonetests-42); retained-clone 10 value targets rc 0 (W-clonetests-43); paged-clone crate rc 0 (W-clonetests-44). `generic_retained_owner` in the derive crate is still on the old sync API (not part of this lane).
STATUS: FIXED-verified W-clonetests-43/44/42
- generic_retained_owner ported (shared drain helper over the effect helper, shared_original_demands); derive generic_retained_owner 3/3 + neutral_owner 4/4 W-clonetests-45 rc 0 (after sync-133 rc 0).
STATUS: FIXED-verified W-clonetests-45
