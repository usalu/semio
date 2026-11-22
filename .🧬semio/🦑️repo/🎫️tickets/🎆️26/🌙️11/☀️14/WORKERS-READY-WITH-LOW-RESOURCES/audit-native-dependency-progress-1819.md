# Native Dependency Build and Publication Progress

The 2026-11-21 18:20–18:23 UTC mirrored Ready receipts put m02 and m10 in dependencies, with `ready=false`. m02 was in attempt 3 under supervisor 1684. m10 was in attempt 1 under supervisor 37199. These observations came from each origin's current `ready.json`, `ready-log-dependencies.txt` and previous failed log under `/Users/Shared/semio-store/v1/origins`.

| Worker | End of first producer pass | Next producer pass in the same attempt |
| --- | --- | --- |
| m02 | Nine full fences took 42 seconds. Ownership reached `frontier:1371136` at 465.46 seconds. | Ownership restarted at `list:64` and 1.42 seconds. A later sample reached Source preservation and reported four more full fences taking 27 seconds. |
| m10 | Six full fences took 183 seconds. Ownership reached `frontier:1373568` at 1,128.98 seconds. | Ownership restarted at `list:64` and 1.07 seconds and reached `list:445824` by 45.82 seconds. A later sample reached `dependencies:current-input`. |

Both current attempts progressed through this sequence without a new terminal receipt. m10's earlier failure recorded a changed exact relative descriptor. m02's earlier failure recorded repeated implementation restart around a missing schema-validator package. The current counter reset follows the successful build-to-publication handoff in the producer rather than the prior descriptor failure. The counters count physical operations and continuations, not unique paths. Installed-path counters are also distinct. For example, m02 progressed through a separate fleet extraction of 547,459,019 bytes before the publication pass.

## Custody and repeated work

The Store generation coordinator calls the producer for build, then calls it again for publication after taking the exclusive bindings gate. The gate remains held through publication and preservation retirement. Both calls run `withPublishedBunWorkspaceOwnershipV1`, compute the input projection and validate it. The first producer writes its settled listing index. The second loads that index, and the process already retains live listing cache entries. Those entries cover settled directory listings paired with physical stamps. They contain no authenticated complete ownership result or manifest proof. The second producer still creates a fresh ownership context and walks states and manifests under the bindings gate. The physical log labels ownership progress exclusive in both phases. The source identifies where the actual bindings gate is acquired.

The current publish path also calls fleet seeding for existing ready scopes before source bindings switch. This best-effort path checks for an existing pack first. An eligible miss can traverse installed paths, fingerprint files and pack units while the bindings gate is held. The retained log establishes such scans occurred during the first producer, where m10 reported a 66,249-entry installed-path scan and a 14.47-second sample. It does not establish an equivalent packing miss after the gate was acquired. Source preservation retirement can take another whole ownership observation under its own required exclusive custody.

## Bounded handoff

Keep the first authenticated, invocation-bound ownership callback live across the build and its transition to the bindings gate. Recheck that same custody immediately after acquiring the exclusive gate and retain the existing terminal recheck. The current frontier replay checks physical state for each observed path, every consumed manifest and the policy. Settled directories with identical stamps reuse their observations. Changed or unsettled directories get listed and compared. When a replay detects drift, existing rederivation accepts only unchanged ownership and consumed manifest bytes after the asynchronous and synchronous checks. A concurrent relevant edit must release the gate and re-enter the existing bounded restart path when its conditions allow.

Keep exact generation pins, current ready receipts and the existing descriptor, Source endpoint, preservation and final fences. The terminal caller rechecks its live ownership callback before releasing it. Moving optional fleet seeding outside the bindings gate is independent. Perform it under the original read and generation custody with its existing authentication. An optimization that just reloads or trusts the persisted listing index would neither eliminate the second derivation nor establish current ownership.

- Store coordinator and bindings gate: `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:733–759, 825–868`
- Bun ownership cache and replay: `🗂️workspaces/🟦️bun/🟨️.cjs:333–378, 446–547, 566–622`
- Fleet seed and publication: `⚡️caching/🚀️bootstrap/🛰️fleet-fetch/📦️install/🟦️.ts:248–267, 426–435`
