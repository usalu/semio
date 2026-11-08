STATUS: WRAPPED UP (quota). Nothing verified green. Canvas-scene lib tests do not compile for pre-existing stale-API reasons (unwork-102 rc 1, 26 errors in path/stroke tests: close_factory_ticket now returns RetainedWorkPoll). unwork-104 (brep validation+representation) killed while queued.

## Class-d sites (8)
- canvas path test `➰️path/🧪️tests:63` `== size_of::<PathEl>()`: path retirement is a hand-written owner (not ControlledRetirement), assert stays correct. No edit.
- canvas stroke test `🖊️stroke/🧪️tests:50`: the `== size_of::<f64>()` gate would silently skip the underfund denials now that active is a ControlledRetirement. EDITED to gate on any non-zero quote and deny `[0, quote-1]`. Run: unwork-101.
- value retirement tests `copy == 0` hints (`🌱️value/♻️retirement/🧪️tests/🔬️unit`): value's own retirement files, left to rootfix.
- hub drawing `retire_snapshot` and CAD `📎️references close` (same body hint): EDITED to `next_demand(0)` then `next_demand(probe.copy_bytes)`; one quote feeds all four grant currencies. Not compiled yet.
- store-read `snapshot-alias` and replay `derived` hard-coded 4096: EDITED copy/release/depth to quote at body 0 (matches ControlledRetirement piecewise). Not compiled yet.
- pack document cursor:86 (`deflate.copy + compressor_bookkeeping_copy`): deflate is not a ControlledRetirement and bookkeeping is the document's own bool/Option<u8>; reading says correct. No edit.
- raster base64:59: `size_of::<Base64DecodeCursor>() + cursor.copy`: cursor is a plain io cursor, not controlled; correct. No edit.

## Workarounds removed
- selected_demand two-pass rebuilds (5 + brep tessellation job): `copy=next_copy; selected_demand(copy)[; selected_demand(first.copy.max(copy))]` replaced by `next_demand(0).and_then(|first| next_demand(first.copy_bytes))` in brep arena unit test:21, brep validation tests 442/469/494, spatial-kernel session tessellation test:99, tessellation-job:283 (rename only). Not compiled yet.

## Skipped (recorded)
- generation2d/3d `.max(OWNER_BYTES)`, playbook 4096 floor, plugin lifecycle:94, typed-input-law:37, store test-support:617, neural 211/463: these call the old `close_step(grant)` / `admit_owned_retirement(value, birth)` API (pane/engine lanes are porting them) so no test can run; receivers are domain wrappers, not provably ControlledRetirement.
- 35 piecewise `RetirementDemand{..}` rebuilds: receivers are mostly non-controlled cursors/wrappers (replication, flow, store, remodel, plugin); replacing blindly would drop their own depth/body semantics. Not touched.
- brep retention test `narrow_job` floor: deliberate widened retry helper, left.

## Wrap-up (final)
REVERTED (unproven): store snapshot-alias and replay derived back to body 4096; hub drawing retire_snapshot and CAD references close back to the original copy/release/capacity/depth form.
KEPT, UNVERIFIED (never compiled/ran; behaviour-equivalent two-pass quote): selected_demand rebuilds -> next_demand(0) then next_demand(first.copy_bytes) in brep arena unit test, brep validation tests (3), spatial-kernel session tessellation test, tessellation-job rename. Also KEPT: canvas stroke test denial gate (non-zero quote instead of == size_of::<f64>()), blocked by the stale-API compile errors above.
Removed and verified: none. Everything else remains (floors, piecewise rebuilds, d sites value tests/pack/base64 judged correct by reading only).
Claims released.

## Round 2
Step 1: ported canvas path (5 sites) and stroke (8 sites) tests: close_factory_ticket(&mut slot, grant).unwrap() -> local helper close() funding an AuthoringWallet(grant) and settling the OriginalRetirementPoll (pattern of brep arena funded_step). UNVERIFIED until unwork-105 (canvas-scene --lib) finishes.
Step 2 (final stop): unwork-105 was still queued behind the fleet and was killed before compiling. The path/stroke test port, the brep/spatial-kernel two-pass edits and the stroke denial gate are all UNVERIFIED (never compiled). No removals proven. Rerun: C/1105b-tools/runtime/tst.sh unwork-106 🧰️framework/Cargo.toml native -p semio-framework-canvas-scene --lib ; then -p semio-framework-3d-brep-validation -p semio-framework-3d-brep-representation. Claims released.
