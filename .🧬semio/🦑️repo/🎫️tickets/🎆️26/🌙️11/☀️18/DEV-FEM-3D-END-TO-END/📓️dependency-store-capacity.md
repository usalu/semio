# Dependency store capacity refusal (fem-3d-rs check, 2026-11-18)

## Symptom
`repo run @semio-tech/fem-3d-rs:check` ended after 102 min with `The dependencies image cannot hold one more generation: 3805 MiB needed, 1430 MiB free after removing every unpinned superseded generation.`

## Measurements (allocated bytes, `blocks*512`, the unit the partition meter and admission use; script `deps-measure.ts`)
- Partition `.cache/b/d`: 36.00 GiB declared, 34.60 GiB used (usage.json 37153374720) at failure time; `.cache/b/d` walk 35431 MiB allocated, 1.53 M files.
- Current generations (the six pointers, all published 11-13, epoch 1): 1715 / 1523 / 1523 / 1525 / 1432 / 1523 MiB allocated, about 9.0 GiB together. Logical size is 6 percent smaller. Scope `.` is 1715 MiB allocated, not 3.3 GiB; the pointer `bytes` of 3.47 GB for `.` is the logical sum of its members and does not equal the disk footprint (du: 1734 MiB).
- Package cache `bun-cache`: 3359 MiB allocated (budget declares 4 GiB). `tmp`, `bootstrap`, `pointer`, `v`: 0 MiB. `generation` receipts: 73 MiB.
- Superseded generations that were present (35ae 1551 MiB; 751b, 8c3b, c1db, c642 about 1-2 MiB empty shells) were this very run's own planned generations, pinned by the failing process itself (`pinJavascriptDependencyGenerationsV1` runs before admission), hence "kept 6 pinned". They were not the problem and are partly gone now.
- Preservation capsules `m/j/.p<8hex>`: 60 directories (hidden from a plain `ls`), created 11-16 18:39 through 11-18 07:07. Twelve of them hold a whole replaced Source copy: six at 1715 MiB (scope `.`) and six at 1532 MiB; the others are 1 MiB. At least 19.5 GiB allocated, which is the missing 20 GiB.
- Budget model check: copiedGenerationBytes 1.75 GiB versus measured 1.43-1.72 GiB per generation. The constant is right; the Windows generations are not 3.3 GiB. C: had 67 GB free at the start and 26 GB later (other work).

## Root cause
Capsules keep a full copy of a replaced Source (`node_modules`) until a publication retires them (`retireJavascriptDependencyPreservationsInStoreV1`, after the publish phase only). Every synchronization that was interrupted or failed after creating capsules, and every retry, leaves more of them. The budget counts capsules as 84 MiB (measured 2026-10-20 on a healthy store), the collector `collectJavascriptDependencyGenerationsV1` only knows generation names and tombstones, and `repo doctor` has no rule for them. Admission refuses the build whose publication would free them: a deadlock that filled 19.5 GiB of the 36 GiB image. Neither the budget constants nor generation retention (pressure grace is already 0) is at fault.

## Fix (source)
`📦️dependencies/💾️storage/🟦️.ts`: new `reclaimJavascriptDependencyPreservationsUnderPressureV1`, called first in `runJavascriptDependencyStoreGenerationsV1`. When the free bytes do not hold the largest recorded generation plus the 115 percent margin (`concurrentAdmissionV1`), it reports a `reclaiming` event and retires the completed preservation capsules through the existing retirement under the exclusive bindings gate. The gate handling is factored into `underJavascriptDependencyBindingsGateV1`, shared with the publish phase. No schema change, no new constants; a healthy store pays one free-space read.
Test: `🚦️concurrent/🧪️tests/🟦️.ts` gains `concurrent reclaim: ...` (passes). Pre-existing failures in that file, unrelated to this change: the Python oracle tests (no python on this host), two timing tests, and the wiring-law test that counts `acquireConcurrentLeaseV1("bindings"` as 1 (it is 3 in HEAD too).

## Not changed, and why
- Budget constants stay: re-measurement confirms them.
- Making the dependencies partition elastic would only postpone the leak and cannot grow the existing 36 GiB image without a reset that would pull every `node_modules` symlink out from under the other agents.
- Follow-up worth doing: a `repo doctor` rule that flags capsules older than the newest publication, and a startup retirement for the `built.state==="current"` early return (it never retires leftovers).
