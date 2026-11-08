STATUS: GREEN native+wasm32-wasip2 for semio-ui-contract-values lib; GREEN native for semio-ui-contract lib. UNVERIFIED: tests, ui-contract wasm, semio-ui-runtime and semio-ui-wgpu-mounted-layout (caller edits made, blocked upstream by semio-framework-job-recorded-clock, 10 errors, not mine: W-uivalues-11/12).

Proofs (C/🗑️generated/1105b/): W-uivalues-5 native rc 0, W-uivalues-6 wasm32-wasip2 rc 0 (values); W-uivalues-10 native rc 0 (semio-ui-contract).

## What changed
- 🔢️fixed-list: try_reserve_one(bytes) = cold page admission (reserve_one_cold); try_reserve_one_funded / try_reserve_capacity_one_funded / release_empty_page take the caller's effect and return the PagedList outcomes (Denied/Ready, typed failures); release_empty_page_leased(bytes) = in-lease page release whose cursor owner pays one unit per progressed step. RetireOwned delegates to PagedList; NativeReceiveMap::from_native; NativeDefaultValue priced size_of::<Option<Self>>().
- value/📋️list (claimed): additive PagedList::release_empty_page_leased; release_empty_page(effect) now runs over module fn release_page with a pay closure (behaviour unchanged).
- 🗺️fixed-map-and-bytes/🛬️receive: RetireOwned on new cursor/seed, admit_storage via native_work_limit, NativeDefault via default_cursor(). Sorted-insert push_step unchanged (keys stay ascending).
- 🛬️receive: UiValueNativeReceiver rewritten on select_advance/advance_nested/advance_local/take(effect,reply)/Poll close (model: NativeTryMappedReceiver + persistent list). Phases Kind, Scalar, Child, CloseChild, Wrap; one accessor or one owner action per local turn (work 1), child take/close priced by the slot. UiNativeRetirement pays one unit only after a progressed arena step.
- 🛬️receive/🏟️collections: UiList/UiMap sequence collections on the new trait (frame-based admission, push work 1).
- 🌱️original (conversion, merge, arguments): forward steps run through one helper `turn` = cx.with_original_effect + single receive_original_retained_work debit; closes through close_original_effect_step; births allocate under the effect (decoder identity from cx.native_receiving_identity). No admit_original_construction left. Unwraps removed.
- arena source: consume_work(u128). typed retire: leased page release.
- semio-ui-contract ♻️retirement/🩹️patch/🛬️receive: UiPatchOps on the new sequence trait (cold page admission under prepare_work/commit_work in reserve_step).
- Callers: present resources (2) and builder original retirement use release_empty_page_leased; wgpu mounted_layout close uses it and fund_list runs try_reserve_one_funded inside with_original_effect with one debit.

## Laws kept
Map keys ascending (UiMapBuilder unchanged), one-page arena rule unchanged, page-exact cold push (reserve_next_slot) unchanged, typed refusals instead of over-admission (zero-progress re-entry is Progress(default) with work 0, errors are typed).

## Open / for owners
- Tests not run: fixed-map receive test and others drive the receiver through value/🧪️testing/📥️admission (Wallet, receive, post_step), which is still on the removed NativeDecodeConstruction*/Work* API (testdebt-b / nativetests). Not ported by me. No new law added because no behaviour changed.
- Question for runtime2 (not asked, decision taken): grant-shaped retirement cursors (resources, builder, mounted_layout close) use release_empty_page_leased instead of an effect-threaded release. Option B: thread an effect through those cursors (large ripple through Mesh3d owners).
- Other callers still on grant forms I did not touch: UiFixedList users whose receiver is a different PagedVec type are not mine.
- Rerun: C/1105b-tools/chk.sh <label> 🧰️framework/Cargo.toml native|wasm -p semio-ui-contract -p semio-ui-runtime -p semio-ui-wgpu-mounted-layout once job-recorded-clock is green; then tst.sh on semio-ui-contract-values.
