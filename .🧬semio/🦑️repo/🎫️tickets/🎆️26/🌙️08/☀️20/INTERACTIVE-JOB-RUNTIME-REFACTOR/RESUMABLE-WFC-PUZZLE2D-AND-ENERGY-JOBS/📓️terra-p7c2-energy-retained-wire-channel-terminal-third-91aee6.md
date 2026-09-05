# P7c2 Energy Retained-Wire Channel — Third Independent Terra Re-Audit

Date: 2026-08-25  
Auditor: Terra (independent, source-static)  
Tree: `18adc8cce3`, inspected as the current shared dirty working tree  
Verdict: **GREEN — source/static acceptance only**

This re-audit preserves the first and second Terra RED reports. It reread the second RED and the updated Sol implementation report, then inspected the current production implementation rather than verifier predicates. No source was edited, and no build, Cargo, Nx, Wasm, browser, shared-script, or runtime test was run.

## Resolved Second-RED Findings

### 1. Retained decoded series, meter, and history identity

`EnergyRestoreJob` now owns the three decoded count fields, and `EnergyRestoreAbandoned` carries the same fields through loss recovery. The field-three decoder reads the three 64-bit values from checkpoint bytes 132–156 and retains them after cap validation. It no longer merely validates and discards them.

At replay readiness, the rebuilt authority must match all three exact logical lengths and have backing slots at least as large as each corresponding retained count. `finish` repeats those exact logical-length and admitted-backing comparisons before it installs the original checkpoint packet as `restore_input`, and it still requires the retained numerical digest. Thus a cap-valid but shortened replay cannot become ready or install.

Evidence: `sim/component.rs` `EnergyRestoreJob` fields and abandonment transfer (556–624), replay comparison (842–887), and pre-install check (933–955). The live mutation law also targets each of offsets 132, 140, and 148 before restore mount.

### 2. Checkpoint recovery remains a provenance-bound lease until close and ACK

The checkpoint queue has no public `adopt` API and no raw packet insertion path. `take` reserves a fixed recovery slot and records the exact generation, pages, and bytes while retaining the original `head`/`len`; `retry` restores the owned packet to that same head; only `ack` of an empty, matching lease advances `head` and decrements `len`. `Drop` stores the exact packet in the fixed recovery registry and `recover_lost` restores it to the unchanged head. The panic-law is a static test declaration; executable confirmation remains deferred.

During restore replay, a nonempty checkpoint notification is retired one payload page per turn. The job then holds an `EnergyWireLease`, ACKs one payload page per turn, and calls `ack_checkpoint_packet` only after that exact lease is terminally empty. An ACK refusal returns the same lease to the restore job. Both the pending flag and the lease are retained through restore abandonment, so recovery cannot substitute a raw packet or skip the acknowledgement.

Evidence: queue take/retry/ack/recovery (186–249); `EnergyRestoreJob` replay lease fields (581–584), per-page retirement and lease ACK (800–824), and checkpoint replay routing (898–906). Scoped census found no `adopt_checkpoint_packet`, `EnergyWireQueue::adopt`, or `fn adopt(` occurrence in the production simulation source.

### 3. One canonical 100-byte preview, decoded before installation

Preview wire publication uses the canonical `SMENERGY` header plus exactly 20 payload bytes, making an exact 100-byte single-page packet. The payload encodes warmup hour, timestep, total timesteps, and `facility_electricity_j / 3_600_000.0`; stage and tier are the typed header codes, and sequence plus all identity fields are taken from the exact packet header. `decode_preview_packet` requires the exact kind, length, magic, version, and all operation/base-revision/generation/seed/sequence fields before producing the sole `EnergyJobPreview` projection from bytes 80–100. The installed packet owns that decoded typed view.

`facility_electricity_j` is a live authority scalar: zone heating, zone cooling, fan, and facility heating delivery contributions accumulate energy in joules and it is included in the numerical digest. The old pending/last preview stores, legacy `ENERGYP1`, alternate encoder, and zero-placeholder projection are absent from the production simulation source.

Evidence: `decode_preview_packet` (3810–3834), source census described below, and the declared live preview law at 4770. The law requires 100 bytes, compares the installed typed projection with a decode of those packet bytes, and observes a positive running facility total; it was inspected but not executed.

## Restore, Commit, and Close Recheck

- Restore still gates each step and installation on matching operation and generation, retains original packet/model/config on rejection, and routes replay fault/completion outputs through page-wise retirement.
- Commit census remains incremental. It accounts for one resident field/channel position at a time, derives all aggregate queue/item/byte/page demand before writer/copy, reserves the commit slot first, and precreates one fixed page source per output page. The final packet is accepted only when payload pages, bytes, mounted pages, and encoded items exactly equal the retained reservation.
- `close_step` remains incremental and begins by closing the writer and staged output page-by-page. Queue close first invokes lost-lease recovery and refuses to retire an active lease; terminal payloads close in bounded steps.

Evidence: commit reserve/page-source/final accounting (`sim/component.rs` 2864–2897 and 3439–3501) and beginning of bounded close (3517–3550).

## Static Law and Mutation Inventory

- Eight live P7c2 laws are declared in `sim/component.rs` at 4610, 4660, 4724, 4748, 4770, 4807, 4843, and 4861.
- The P7c2 mutation fixture contains exactly 35 `id` entries. Its coverage includes the three decoded count fields/backing, public adoption, unchanged-head retry/ACK, loss recovery, preview size and typed projection, the old split/zero/legacy preview forms, wire identity, restore and commit cases.
- The law and mutation JSON files parse successfully with `jq empty`.

## Scoped Non-Executable Gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` over the ten modified Energy engine Rust files | GREEN |
| `git diff --check` over those files plus the two P7c2 JSON fixtures | GREEN |
| `jq empty` over both P7c2 fixtures | GREEN |
| Production-source negative census for adoption, legacy/split/placeholder preview, JSON/raw-payload helpers, and the prohibited dynamic collection helpers | GREEN — no matches |

## Explicitly Deferred Executable Evidence

This is not a runtime sign-off. The following were deliberately not executed in this audit: Rust/Cargo type and test gates; all eight live P7c2 laws; all 35 fixture mutations; 1/2/4/default-worker chronology; MAX/backpressure; panic injection; Wasm/browser parity; and the 8 ms scheduling/fuel timing measurement. Those gates remain required for executable acceptance, but do not block this source-static GREEN result.
