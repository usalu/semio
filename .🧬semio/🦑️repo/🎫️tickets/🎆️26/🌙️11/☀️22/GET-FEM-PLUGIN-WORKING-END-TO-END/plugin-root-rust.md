# FEM plugin root Rust workspaces: status (update 2)

`repo cargo --store=shared` hung for more than 15 minutes, so every run used native cargo with `build.rustc=rustc` and no wrapper. The later runs used the coordinator's non-C:-default target/build/TMP dirs and CARGO_INCREMENTAL=0. No cargo process of mine is running.

## Result per workspace
| Workspace | check --all-targets | test |
|---|---|---|
| root (app-surface, sqlite, receiving) | PASS | PASS (all ok) |
| `🏭️bridge` | PASS | PASS (0 tests) |
| `⚙️engine/📍node` | PASS | PASS, 2 of 2 (planar and volumetric `native_node_original_receiving_projection_and_every_cancelled_frontier`) |
| `⚙️engine/🧱️material` | PASS | 12 of 14 pass; 2 fail (planar and volumetric `native_material_sparse_patch_...`) |

The `🧪️tests` folder has no Rust crates wired into these workspaces. I did not run the 2d/3d artifact workspaces (other agents own them).

## Fixes (files)
1. `🧩️sqlite/🧮️projection/🧪️tests/🦀️.rs`: work-loan test migrated to `OriginalWorkRecipient`, `OriginalWorkAuthority::with_loan` and `u128` actual work.
2. Framework harness `🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/receiver-driver.rs`:
   - Nested turns now get `maximum_items = NATIVE_SCHEMA_METADATA_ITEMS` (3, the quote of the schema-metadata turn) instead of a hard-coded 1. This fixed the "receiver stalled ... no work" panics; the framework's own `native_shared` test failed the same way.
   - The wallet stock stays at the original `1<<20`. I raised it briefly, then reverted it per the coordinator and used a fresh bank per cut instead.
3. `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🦀️.rs`: re-export `NATIVE_SCHEMA_METADATA_ITEMS` (path `native_decoding::owned`).
4. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🎟️natural-publication/🦀️.rs`: `reserved_birth.as_ref().is_none_or(OriginalConstructionAdmission::terminal_is_empty)`.
5. `⚙️engine/📍node/🧪️tests/🦀️.rs`: rows the derived receiver accepts but `validate()` must refuse (the empty id) are now checked through `FemNode::validate()`. The refusal was never part of receiving; the `validate` doc says it is a separate turn.
6. `⚙️engine/🧱️material/🧪️tests/🦀️.rs`: same for refused rows (`FemMaterial::validate()`); `fresh_bank()` added.
7. `⚙️engine/🧱️material/📋️table/🧪️tests/🦀️.rs`:
   - Unbox the `Box<FemMaterial>` projection output.
   - `sealed_original` uses `SealedShared::source_admission_demand()` and `source_admission_work_demand()` (quote-derived).
   - `fresh_bank()` per cut.
8. `⚙️engine/🧱️material/🩹️patch/🦀️.rs`: the patch cursor is split into a local `advance(row,patch,body,effect)` and a nested `advance_nested(row,patch,control,body,maximum,grant)`, selected by `nested(patch)`.
   - Reason: `control.receive_shared` and the name child prepare Work, which is forbidden inside `with_effect`.
   - `primary()` now uses `OriginalReceivingPrimary::refusal()`; the old version dropped work_refusal and the other new causes.
   - Phase 6 quotes `self.admission.shared_demand()` (stage-aware).
9. `⚙️engine/🧱️material/🩹️patch/🧪️tests/🦀️.rs`: drives local turns through `session_step`, nested turns through `session_nested`, and uses `fresh_bank()` per cut.

All `[DEBUG]` prints were removed.

## Open: material patch test (2 failures)
- The test now fails with `assert!(normal > 0)`: the cursor never becomes `ready()`.
- Debug runs showed the sealed-admission birth succeeds for exactly one turn per cut (work=4, 1 item). The next exact-grant `receive_shared` call then returns zero work with `Ok(Progress(default))` indefinitely.
- That is the publication stage of `NativeSharedAdmission` (demand `size_of::<T>()`, depth 1, work `publication_work_demand`). I suspect the receiving frame is not back at `Empty` or `Granted`, or its work quote is unresolved; `can_advance()` and `phase()` on the admission should show which.
- Next step: print `self.admission.phase()` and `can_advance()` in `advance_nested`, and compare with how the sealed-shared receiver drives a second `receive_shared`, in `🛬️decode/🧳️owned/🔗️sealed-shared-receiver`.
- The cursor is only used by this test (no other consumer).

## Notes for others
- Anything that includes `receiver-driver.rs` (the 2d/3d artifacts) now gets the nested items grant.
- Windows native cargo needs the env from the rules file; `cargo`'s default `.cargo/config.toml` points rustc at a repo-only launcher.

## Material sparse patch (round 2)

Both `native_material_sparse_patch_...` tests (planar, volumetric) now pass. Three independent root causes, all in the framework field/admission protocol, not in the test driving.

1. `NativeSharedAdmission::shared_demand()` quoted the wrong publication stage. After the sealed birth it returned `size_of::<T>()` copy bytes, but `SealedSharedBirth::publish` requires `SealedSharedBirth::<T>::publication_demand()` (that plus the `SealedShared` and `Option<SealedShared>` headers). The grant derived from the quote was therefore too small; the publish step silently returned zero work with `Progress(default)` forever. Fix: `shared_demand` returns `SealedSharedBirth::<T>::publication_demand()` when a birth exists (`🛬️decode/🏭️construction/🦀️.rs`). Found by printing `receive_effect_typed` stages ([DEBUG], removed): work quote admitted, effect ran, publish refused the grant.
2. `NativeIntrinsicSharedField` had no step that advances or closes its clone cursor; `advance` returned "requires its borrowed child producer" in phase 0 and phase 2 and `close_step` "needs its borrowed close producer". Added in `🛬️decode/🧳️owned/🪡️field/🦀️.rs` and `🎛️producer.rs`:
   - `Operation::AdvanceCursor` (tag 12) and `Operation::CloseCursor` (tag 13) with receive and recover arms, mirroring `TakeCursor`; reply copy added to the phase 0 and phase 2 demands and +1 work to their quotes.
   - `relay_close` target 4 (cursor `close_into`, `recover_close_into`, `close_original_into`, `recover_close_original_into`) replacing the two UnsupportedOwner refusals in `close_step` / `close_original_step`; cursor `close_demands` / `close_work_demand` now include the reply.
   - `advance_pending()` also reports `slots.advance_result_ready()`, as `sealed-shared-receiver` already does, so owners settle the outcome before close.
3. Patch cursor and test: `NativeMaterialPatchCursor::advance_pending()` added (`🩹️patch/🦀️.rs`); the test keeps advancing while pending after its cut. The accounting assertion is exact and per turn, with no lag: measured, a slot birth allocates in its own turn and the construction recipient accepts the same physical progress in that same turn (verified on every turn of a cut), while the field's `Ready` turn returns no items because "the original semantic receipt has already settled" (`acknowledge_success_into`). So each funded turn asserts `heap == recipient delta` and `heap == reported progress || reported items == 0`. The recipient delta needs the new harness reader `session_submitted(bank)` (`🧪️testing/📥️admission/driven.rs`, re-exported). Grants all come from `cursor.demand()` quotes; no budget changes. The earlier `>=` relaxation is gone.

Framework files touched: `🛬️decode/🏭️construction/🦀️.rs`, `🛬️decode/🧳️owned/🪡️field/🦀️.rs`, `🛬️decode/🧳️owned/🪡️field/🎛️producer.rs`, plus the harness addition `session_submitted` in `🧪️testing/📥️admission/driven.rs` and `🦀️.rs`.

### Framework suites (target `C:	wroot`, `-j 6`, debug info off)
- Pass: `native-shared-field` (0 tests), `native-owned` (all ok, 15 tests across 7 binaries), `testing-admission`, `control-contract`.
- Do not compile, unrelated to this change (test files unmodified vs HEAD): `native-decoding`, `native-outcome` (`decode(...)` called with 2 args, API takes 3; a `u64` vs `u128` work assert in `♻️retirement/🎟️continuation`), `native-encoding` (close_step arity, missing `loan` / `close_retirement_recipient`). I did not fix these: the drift is in other agents' API work, not a regression of mine; they are open and need an owner.
- Material: 14 of 14 pass; root, node, material, bridge `cargo check --all-targets` pass.
