# Coordinator Independent P2d Presentation Liveness Final Audit — 2026-08-23

## Verdict

**REJECT — one bounded source blocker remains.**

The mounted-route repair clears the four blockers from the prior independent audit: autonomous
shard jobs now carry separately retained authority by value, accepted overlays remain checked out
through prepared-frame and presenter adoption, fault/app/extension/realm close use retained close
state, and realm release requires both overlay and presentation-bridge terminal witnesses.

The fixed presentation bridge can nevertheless strand a valid ready preview behind a cancelled
hole. That violates continuous preview delivery and can retain the corresponding overlay receipt
indefinitely during normal operation.

No production source was edited by this audit.

## Blocking finding

`JobProgressPresentationBridge::take` inspects only `slots[take_cursor]`. `cancel` makes a reserved
or ready slot vacant without advancing `take_cursor` or otherwise recording the next admitted ready
slot. A concrete live sequence is:

1. actor A and actor B publish into slots 0 and 1;
2. actor A closes before frame checkout, so slot 0 is cancelled to `Vacant`;
3. actor B remains `Ready` in slot 1;
4. every frame calls `take` at cursor 0, receives `None`, and never visits slot 1.

The stalled preview need not be recovered: if B waits for presentation before publishing another
step, no publication advances the ring, while only later realm/app close can retire the retained
owner. The capacity/FIFO fixture fills and drains an unbroken ring and therefore does not exercise
this hole.

Repair the bridge with a fixed, allocation-free admitted-order cursor/sequence that skips vacant
cancelled positions without skipping an older ready position. Add a deterministic fixture for a
cancelled head followed by one or more ready owners, plus a verifier mutation that restores the
single-slot `take` behavior. Preserve exact generation matching, retry handback, and the terminal
witness.

## Cleared prior blockers

- `Effect::SpawnJob` mints and stores `JobTurn` authority before `start_job`; each live
  `ShardOutcome::Job` carries that stable authority separately from its advancing publication.
- WGPU validates authority/publication identity and retains the exact overlay receipt until the
  fixed presentation token is acknowledged after `PreparedRenderGate` presenter adoption.
- A dropped pre-adoption lease returns to its exact generation slot instead of acknowledging.
- `ShardOutcome::Fault`, app/extension destruction, and `CloseRealm` initiate retained owner close.
- `OsHostRetirement` owns the realm close handle, and realm completion requires overlay,
  rejected-owner, pending-presentation, and global bridge terminal emptiness.
- Mounted aggregate fixtures and the permanent predicate now cover the 64/128 slots, 512 items,
  4 MiB, presenter phase ordering, fault close, realm close, and terminal witness claimed by the
  repair report.

## Gates rerun

| Gate | Result |
|---|---|
| Rust-2021 `rustfmt --check` on actor, shard, WGPU glue, frame job, and OS host | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS, 309 |
| `bun ./📜️script.ts verify interactivity --plain --deny` | RED only on the concurrently edited P1 database wait-census predicates; no P2 finding |
| Build/runtime matrix | Not run while overlapping Rust source packets are active |

P2d remains source-rejected until the bounded cancellation-hole repair is independently audited.
Phase 2 remains RED, and runtime acceptance remains mandatory after the shared source tree quiesces.
