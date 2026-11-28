# Progress under a single-item wallet: a real invariant, but the floor is 9 items, not 1 and not 96

## Verdict
- The framework does not promise progress under a one-item wallet for an original construction. It promises progress at the quoted floor of the step, one operation per turn, and a yield that spends nothing below it.
- The previous fix (a 96-item wallet) was nevertheless wrong. It rested on a misread receipt. The true floor of a whole replay is **9 items**, pinned exactly by a new law.
- Product fixes: a real cancellation crash in the history ledger, and the floor is now a documented, exported number. The laws are renamed and say what they assert.

## Contract evidence (framework docs, code and history)
- `OriginalReturnScopePlan::quote` (`🧵️job/📬️outcome/🪙️budget/📦️stock/🎟️scope/🦀️.rs`): "A scope claims exactly its quoted need in one turn and is ready in that same call, or claims nothing: a turn that cannot afford the whole need is denied and charges nothing."
- `original_construction_reserve` (`🧵️job/🏭️construction/🦀️.rs`): "the complete same-turn constructor, candidate, capture and Source result plan". `admit_original_construction` returns `Pending` unless `complete.fits(retained_grant)`. `admit_original_construction_settled`: "A turn commits only when it funds the full settled quote before charging anything"; a quote above the ceiling is a typed `OwnershipLimit` refusal. `original_construction_settled_turn_quote`: "the whole grant ... a caller funds one turn with".
- `ArtifactStoreInitializationAuthority::next_step_demands`: "Quotes an indivisible next business step". `step` returning `None` means "could not afford an outcome". Callers size the wallet from quotes.
- Splitting the reserve (capture, candidate, Source outcome placement) across turns would leave a born candidate whose metadata is not yet charged. That is what the same-turn plan exists to prevent. One-item progress holds for divisible work (retirement, copy, paging) and is exercised there.
- History: the two laws asserted wallets `1` and `7` since commit 03c902fa4886 (2026-10-11). The construction admission entered the initializer on 2026-10-28 / 11-01 (`1e27248a0baa`, `15d755f053c3`). The bin-05 initializer laws were not re-measured afterwards. The 08/28 `1105b-result-hostcheck.md` shows the single-item law green, but that was before any construction admission existed. So the laws were stale, not a regression of a kept guarantee.

## What was wrong with 96
The harness `OriginalJobFixtureCaller::turn_with_decoder` widens the turn by `original_receiving_service_scope_headroom()` (18 items) beside the wallet and the test asserted `receipt.fits(wallet)`. At a 48-item wallet the receipt was 66 = 48 + 18, which the previous agent read as "the first turn claims 58 to 66 items". Nothing demanded about 70 items. The scope claim is capped by the wallet and refused whole when it does not fit.
Measured by bisection over complete replays (3 edits, one supersession): a wallet of 8 never completes, a wallet of 9 does.

## Changes
Product
- `🏪️store/🏗️initialization/🔁️replay/🦀️.rs`: `artifact_store_replay_step_items_floor()` (9), computed from the framework quote, see the follow-up below.
- `🌱️value/🎟️control/🧬️contract/📏️physical/🦀️.rs`: doc on `RetainedCloneGrant`: divisible work progresses under any non-zero wallet; an indivisible step names its floor by quote, and a wallet below it yields without spending.
- `🌿️vcs/🔗️read/🦀️.rs`: `original_read_slot` called `get_key`, which dereferences the slot value. While a history read is in flight or returning, the value has moved into the read and `RetainedOriginalSlot::get` panics ("original read retains its actual authority"). Cancelling the replay during `Fold` hit it in `OriginalHistorySource::close_demands`, so `cancellation_at_every_turn_boundary_closes_with_exact_retirement` crashed. New `is_live_key` checks generation, committed visibility and occupancy without borrowing the value.

Laws (`🏪️store/🏗️initialization/🔁️replay/🧪️tests/🔬️unit/🦀️.rs`, bin-05)
- `TIGHT_WALLET_ITEMS = 96` removed; laws use the product floor.
- Renamed: `replay_equals_direct_application_under_every_grant` to `..._under_every_funded_grant` (wallets: floor, 2x floor, 1<<20); `cancellation_under_the_tightest_wallet_makes_progress_and_closes` to `cancellation_at_the_step_floor_wallet_makes_progress_and_closes`. The suite header says what it covers.
- New `a_wallet_below_the_step_floor_yields_without_advancing_and_still_closes`: a one-item wallet for 256 turns only yields, never leaves the first edit, folds nothing, then cancels and closes exactly.
- New `the_step_items_floor_is_exactly_the_minimum_wallet_that_completes`: floor to floor+3 complete; floor-1 and 1 never do.
- The per-turn wallet assertion now allows the receiving service the caller funds beside the wallet and names the phase in its message.

## Results (native, private `C:\t\sw`, `-j 3`)
- bin-05: 86 of 86 after the follow-up (83 plus three new laws). This includes `cancellation_at_every_turn_boundary_closes_with_exact_retirement`, which was red in this tree before the ledger fix.
- `scope-quote`: 4 of 4 (re-run after the follow-up), including `a_loaned_scope_never_clamps_a_quote_below_its_indivisible_floor`.
- 2D `fem2d_replacement_revokes_the_read_leases_of_the_displaced_store`: PASS. 3D `fem3d_replacement_revokes_the_read_leases_of_the_displaced_store`: PASS.

## Follow-up: no more yielding forever below the floor
- **One pricing source.** `original_construction_step_floor::<O>(demand)` (`🧵️job/🏭️construction/🦀️.rs`, exported from `semio_framework_job`) is the reserve (`original_construction_reserve`) plus the receiving scope's control rows (outcome placement items + 2): 9 items for a one-item demand. `ARTIFACT_STORE_REPLAY_STEP_ITEMS_FLOOR` is gone; `artifact_store_replay_step_items_floor()` computes the number from that quote, so the constant can no longer drift from the pricing.
- **Quotes.** New trait method `ArtifactStoreInitializationAuthority::next_step_floor` (grant with items; default is the physical quote with one item). The replay overrides it: exact (demand-derived) floor for `ValidateEdit` and an unbuilt candidate, the item floor for every other constructing phase (`FindApplied`, `FindRedo`, `BindGenesis`, `SeedHistory`, `CommitApplied`, `CommitRedo`, `BuildCandidate`). `next_step_demands` for `ValidateEdit` now returns the floor's physical currencies instead of zero (the other phases keep their existing quotes; their bytes are not derivable before entering the phase).
- **Refusal.** Every replay construction goes through `built`, which now calls `refuse_below_ceiling` first: if any currency of the floor exceeds `cx.original_turn_ceiling()` it fails with `OwnershipLimit`, code `replay.step-floor-above-ceiling` (`ARTIFACT_STORE_REPLAY_STEP_FLOOR_REFUSAL`), message `"<code>: maximumItems floor 9 above ceiling 8"`. It runs before any charge. It only refuses what could never run: the reserve currencies already had to fit the wallet, and only the 4 control items are new.
- **Fixture audit.** Callers of the replay construction: bin-05 (stock 1<<24 items), plugin genesis handoff (1<<28), and the lifecycle host (ceiling = the step grant, which the 2D and 3D replacement laws exercise). The other single-item fixtures (`bounded-reload`, recursive replacement, window config) drive retirement or a custom authority, never a replay construction. No fixture needed raising.
- **Law.** `a_ceiling_below_the_step_floor_is_refused_at_admission_with_the_floor_quoted`: a caller whose stock is `floor - 1` items is refused with the code, the currency and the floor quoted; a second initializer at `ValidateEdit { index: 0 }` quotes the same floor through `next_step_floor` and the same physical currencies through `next_step_demands`.
- **Still true.** The fixture caller's ceiling is its stock, so a small per-turn wallet under a large ceiling still just yields (pinned by `a_wallet_below_the_step_floor_...`). Only a host whose ceiling is below the floor is refused; hosts should size wallets from `next_step_floor`.

## Files
`🏪️store/🏗️initialization/🔁️replay/🦀️.rs`, `🏪️store/🏗️initialization/🔁️replay/🧪️tests/🔬️unit/🦀️.rs`, `🌱️value/🎟️control/🧬️contract/📏️physical/🦀️.rs`, `🌿️vcs/🔗️read/🦀️.rs`, `🧵️job/🏭️construction/🦀️.rs`, `🧵️job/🦀️.rs`, `🏪️store/🏗️initialization/🤝️authority/🦀️.rs`. Scripts here: `sw-b5.sh`, `sw-law.sh`. `C:\t\sw*` deleted.
