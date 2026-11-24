# Boot Matrix 02:00 Causes (Overlay, en)

Probe: overlay port 8775, modules demonstrator (01:55) and procedural (02:3x) built with all fixes through 00:25. Result 2/8 (energie, statik pass).

| Pane | Root error |
| --- | --- |
| generator | `member.publication: original page allocation attempt is occupied` on `toolRunStart`, contributions re-arm, typed-operation progress refresh and completion effects. Shell reaches `error`. |
| koordinator | `[DEBUG] one-item publication blocked` then `interactive-job.publication-stalled` on `setContributions` and `setActiveExample` (`pending_artifact=true publication=true`). |
| aussuchen | `setContributions` publication-stalled plus other sessions' `[DEBUG] session advance`/`page admit`/`recv refuse` noise. |
| aggregator | Only other sessions' `[DEBUG] session advance`/`page admit`/`recv refuse` noise. |
| bearbeiten | `member.publication: seal canonical second pass length changed` (bundle `err Some("validation failed: …")`) on `setContributions`. |
| verfolgen | One-item publication preparation refused with `InvariantViolated: canonical original child receipt exceeds caller grant` (canon kind 3 phase 4, demand copy_bytes 3336 depth 3, progress copied_bytes 3975), silently retried until `setActiveExample` stalls. |

Raw log: `🗑️generated/ov-0200.log` (deleted at close).

## 04:50 Re-Probe (Demonstrator 04:47, Procedural 03:50)

3/8 en: Aggregator, Energie, Statik pass. Bearbeiten's seal error and Generator's page-occupied error are gone. Every remaining pane fails on `[DEBUG] one-item publication blocked` then `publication-stalled`:

| Pane | Extra evidence |
| --- | --- |
| verfolgen | `sealer blocked: zero progress demand RetirementDemand { copy_bytes: 0, capacity_bytes: 0, release_bytes: 0, depth: 3 } grant ArtifactStoreOneItemGrant { maximum_items: 1, maximum_depth: 64, .. } phase 0` |
| koordinator, bearbeiten | `poll_wit authority refusal kind=OwnershipLimit "original return source receipt exceeds spent and held authority" spent=0` then `native-return-refused: authority` on progress refresh, completion effects, refresh-owed host effects |
| aussuchen, generator | no sealer or batch site printed; blocked elsewhere |

No `batch blocked at` line appeared, so `advance_apply_batch` is not the Blocked source.

## 06:45 Re-Probe (Demonstrator 06:38)

All four demonstrator panes still fail; the diagnostics now name the sites:

- **koordinator, aussuchen, bearbeiten (lane `Config`):** `batch blocked at` bundle `🦀️.rs:301:567`, which is the `else` of `receiving.with_effect(grant.retained_grant(), work, |_, effect| owner.advance_digest(effect))` returning `None`: the batch digest step is never admitted. `work` = `owner.next_digest_work_demand()`.
- **verfolgen (lane `WindowConfig`):** `canonical source zero progress phase 4 sub 0 traversal hash demand depth 2 grant {items 1, copy 0, capacity 0, depth 2} work Ok(2) remaining Ok(1)`: the step needs 2 work units, the effect has 1 left, so it makes zero progress forever. Then `sealer blocked: zero progress`, `prep step blocked`, `batch blocked at …:301:567`.
- **Common hypothesis:** each publication unit funds fewer work units than the owner's minimum next-step work demand, so the step is never admitted and the unit answers Blocked every time.
- **Authority refusal confirmed secondary:** it now reads `WorkExceeded` / `original return source CPU exceeds spent and held authority` with `claims: 2`, the drain caused by the stall.
