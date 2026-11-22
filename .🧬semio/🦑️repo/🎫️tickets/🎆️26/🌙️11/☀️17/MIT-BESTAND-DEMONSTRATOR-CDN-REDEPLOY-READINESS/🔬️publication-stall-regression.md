# Typed-operation publication stall (wd3qizX, wRATjNi)

Status: the `boxclose#` emitter is identified and its premise (an unfundable whole-close quote) is refuted. A silent unfunded-unit skip in the maintenance ladder is the surviving candidate; a peer's corrective edit (`covering_loan`) landed after both builds and is covered by a new law. The stall itself is not yet reproduced; the rebuild below is the proof step.

## 1. The `boxclose#` emitter

The compiler provenance of each run (`.cache/b/r/m/components/runs/<run>/r/provenance/cargo-unit-provenance-*.json`) records every source file the build read with its size. Comparing those sizes with the live tree lists exactly the files edited after the build started (wRATjNi started 03:42, built 03:43):

| file | build | now | meaning |
|---|---|---|---|
| `♻️retirement/🔗️arc-custody/🏭️factory/🌳️root/🦀️.rs` | 22,524 B | 22,210 B (mtime 03:50, no diff against HEAD) | the peer's temporary print was removed: this is the `boxclose#` site |
| `♻️retirement/🏭️factory/🦀️.rs` | 59,432 B | 59,594 B (04:23) | new `[DEBUG] native header unsafe type` print |
| `🪜️maintenance-ladder/📥️staging` (instance) | 6,898 B | 8,181 B (04:12) | `covering_loan` |
| `🪜️maintenance-ladder` (plugin) | 83,718 B | 87,190 B (04:33) | `CloseEmissionConstruction`, `document_load_diagnostic` |

The only function in that root file that holds a `grant`, a work ceiling, `box_depth(owner)` (a `Result`, printed as `depth=Ok(2)`) and `owner.terminal_is_empty()` (`term=false`) is `box_close_step`. So `boxclose#` printed `effect.grant()` and `effect.maximum_work_units()` of the effect handed to `box_close_step`.

## 2. The printed grant is not a quote

`RetainedCloneGrant{5,652,011 items; 2,094,757,945 copy; 263,862,856 capacity; 467,039,831 release; depth 64}` and 47,727,873 work are the budgets of the effect, i.e. the actor turn's own issued ceiling (`StepContext::original_turn_ceiling`), as the `generator-mounted-law.rs` harness also issues (8 M items, 1 GiB copy). A close that runs under that grant is never starved: `box_close_step` admits it. The close was funded, not stalled. No retained Box or Arc-root demand is quoted as a whole: `ArcRootRetirement::demands` quotes one stage at a time (alias, elect, convert, `issuer.retire_demands`, child), and every `retire_demands` implementation in the tree returns a constant-size frame.

## 3. What can stall forever instead

The mounted per-unit loan (64 items, 32 KiB copy, 16 MiB capacity, 1 MiB release) is smaller than the turn ceiling. `maintenance_ladder_unit_step` skipped any unit whose declared indivisible demand exceeded the loan with `cx.consume_fuel(1); continue`, with no progress, no wait and no fault. Such a unit is never served, so everything that waits on it waits forever: the archive-load units (stage 24 reports `Progress(empty)`), store replacement (stage 14), and the typed-operation publication behind them, which the watchdog then terminates with `interactive-job.publication-stalled` after 4096 units. Known demands above the loan: the 33,976 B history-replay preparation step (copy above 32 KiB) and the paged archive receiver, whose `PagedArchiveParent::advance` (`demand` vs `issued`) and `receiver_close_demand` silently return when unfunded.

`covering_loan` (instance `🪜️maintenance-ladder`, 04:12) derives the unit grant from the unit's declared demand: copy, capacity, release and depth are raised to the covering power of two, never beyond the caller's actual turn ceiling, so the shortfall becomes bounded and funded. The stall limits are untouched. Together with `MountedOwnerPolicyV1::INTERACTIVE_COMPONENT` now deriving its copy grant from `LARGEST_INDIVISIBLE_STEP_COPY_BYTES` (03:22, included in wRATjNi) this is the proper repair: page within the grant when the grant can cover, clamp at the real ceiling, otherwise stay refused.

## 4. Test evidence

- Law: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🪜️maintenance-ladder/🧪️tests/🦀️.rs` with the language-agnostic fixture `🧪️tests/🧫️fixtures/covering-loan.json` (six rows: funded demand keeps the loan; 33,976 B copy covered to 65,536; 20 MiB capacity covered to 32 MiB; release covered; a 1 GiB demand clamps at the 263,862,856 B ceiling and stays unfunded; a ceiling below the loan never narrows it).
- Run: see the result recorded below.

## 5. Not proven

The Generator and Demonstrator stalls themselves were not reproduced natively (the native laws run with an 8 M-item grant and do not meet the per-unit loan). Next evidence: the queued rebuilds carry `covering_loan`, the stage-24 `document_load_diagnostic` witness and the factory header print; rerun `probe-linked.sh` on both. If a stall persists, its `archive-load:` line names the owner and its `funded` flag.
