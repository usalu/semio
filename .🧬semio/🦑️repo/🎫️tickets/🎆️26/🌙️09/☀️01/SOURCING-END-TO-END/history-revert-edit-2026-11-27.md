# History revert and edit withdraw/restore, 2026-11-27

## Result
Final PASS (hub wasm 20:37, en and de, run 21:2x against 6081): `history panel lists the actions`, `revert to command takes back only the newer commands`, `history edit withdraw removes a mutation from the replay and restore returns it`. Log: `🗑️generated/hre-final-history.log`.
The `history edit accept ... discard` step and the steps blocked behind it are owned by the discard agent (`history-discard-2026-11-27.md`); not touched here.

## Root causes
1. Revert fault `selected original birth must run before the general retirement effect`: `revertToCommand` aborts a replay in `🏪️store/🕰️history/🏁️replay`. `retire_controlled` opened the general allocation effect first and closed the owner (a `dyn ErasedSnapshotRetirement` ticket) inside it. The original recipient had no failure spare, and a selected child birth was pending; both must run on their own turn before a general effect (`close_factory_ticket_controlled` refuses otherwise). Found by temporarily tagging the fault with the owner type name.
2. Restore never appeared: the journey was wrong, not the product. Restore is offered only on a mutation holding an accepted draft (design 22.1, Rust law `a_mutation_row_offers_withdraw_and_restore_takes_an_accepted_draft_back`). Withdraw opens a withdrawn draft; Accept and the replay must finish first.

## Fixes
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/🏁️replay/🦀️.rs`: `retire_controlled` funds the failure spare outside the effect (`born`); new `retire_ticket_controlled` also runs the selected child birth through the decoder loan; shared `retire_born` keeps the old effect path.
- `.../🏁️replay/abort.rs`: the abort ticket close uses `retire_ticket_controlled`.
- `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/🦀️.rs`: re-exports `retirement_birth` and `close_retirement_birth`.
- `journey-history.ts` (ticket): withdraw step clicks Accept and waits for Restore (90 s) before clicking it.

## Wrong turns (all removed)
Two attempts in the time-travel retirement path (spare funding, child birth via the ledger) did not touch the failing path and are removed. Temporary `[DEBUG]` type tag in `🎟️tickets/🦀️.rs` and journey debug lines removed.

## Tests
- No Rust unit test for the replay fix: a faithful one needs a StepContext with an original decoder; my attempted helper test (never compiled) was removed with the helper. The regression is covered by the browser journey step only. Open.
- Open File probe (coordinator request): `probe-open-file.ts` against the then-served hub produced no `[DEBUG] natural-` lines (wasm predates the prints); only the `os-shell ... command stalled ... media-in` line. Saved in `🗑️generated/open-file-debug.txt`.

## Incident
A boot failure ("Retirement receipt smaller than already received effects") during this work was not from my change; the coordinator traced it to a peer edit (`boot-retirement-invariant-2026-11-27.md`).
