# Startup Carried Phase Caller Closure — Source Checkpoint

Current Source was inspected before edits against `🗑️generated/print-gallery-pdf-types-prototype1.log`. Its three service missing-phase calls were already migrated by a coeditor from the former graph-native API to `withRepositoryNxStorageV1`, carrying `storePhase` into storage and receiving `nxPhase`. The bootstrap injection was already corrected to `typeof import("./🛠️tools/📜️script.ts")`; existing preparation, tooling, Rust/Next graph and graph-artifact calls already matched the required live signatures. Those fixes were preserved, rather than replaying stale diagnostics or introducing an optional compatibility interface.

Two remaining actual phase handoffs were corrected. `NxScript.runInStorage` now opens its tools store under the actual phase returned by `withNxToolingHeadroomRecoveryV1`, then forwards the returned tools-store phase into its work. `withNxDaemonServiceBirthV1` now supplies its actual tooling-use phase as the required second callback argument; `withNxDaemonStartupV1` passes that phase to its child callback rather than the earlier tools-store phase, and guards its enclosing lease with the enclosing actual phase.

## Actual custody callgraph

- Bootstrap: existing Script Original → incoming `phaseControl` → headroom recovery `actualPhase` → tools store `storePhase` → `runWithToolingStore`. Existing Rust, Next, graph-artifact and selected-snapshot callback controls remain carried as authored.
- Finite daemon birth: Original/incoming phase authentication → tools store `storePhase` → Nx store `nxPhase` → tooling-use `usePhase` → environment/body callback `(environment, usePhase)`.
- Startup: actual inherited receiver → existing startup admission phase → tools callback `actualPhase` → startup resource lease → birth callback `birthPhase` → actual child callback `(birthPhase, environment)`.
- Service startup authentication: existing parent-envelope receiver → tools callback `storePhase` → required authentication control. Service acquisition's later authentication uses its actual `usePhase`.
- Journal receiver authentication: existing carried service envelope → tools callback `storePhase` → required physical authentication control, followed by Nx `storePhase` and tooling `nxPhase`. Existing journal lifetime/body contract was preserved.

`authenticateNxDaemonServiceHandoffV1` and private `authenticatePhysicalWitnessV1` now require an explicit incoming control. Bounded witness descriptor reads use that control, rather than the reservation's broad `store.control`. The reservation checker still authenticates the actual reservation base as required by its contract. Checks surround rechecks and the network await; handshake duration is the minimum of its existing 10-second ceiling, incoming phase remaining time, and the original parent absolute deadline. No Original clock is renewed. This is Source evidence; no live socket/service execution was performed.

## Schema, genuine RED and final gates

New closed language-neutral fixture/schema describe eleven exact carried-phase edges across the three actual Source modules. The test reads Source without importing/executing these modules, uses TypeScript's AST to locate real function/method calls, checks exact argument bindings, and compares the observed edge multiset with independent SQLite set difference. It also preserves all three coeditor-corrected service→Nx store edges. It does not fabricate storage, controls, authority, observations, or successful bootstrap execution.

The genuine first RED, handle 30750, exited 1 with 0 pass / 1 fail / 2 assertions: recovery's nested store received `phaseControl` instead of returned `actualPhase`. The narrow implementation then passed both Source and selected actual-root strict gates; three test-only fixture rows subsequently extended the census to the existing corrected Nx edges.

| Final gate | Handle | Result |
| --- | --- | --- |
| Source prototype | 19806 | exit 0; 1 test / 13 assertions; 1.344 s |
| Source release | 27585 | exit 0; 1 test / 13 assertions; 1.56 s |
| Selected actual-root strict prototype | 88430 | exit 0 |
| Selected actual-root strict release | 92744 | exit 0 |

All handles, including RED and intermediate successors, are terminal. Logs are `🗑️generated/startup-carried-phase-red.log`, `startup-carried-phase-{test,types-owned}-{prototype,release}1.log`, and `startup-carried-phase-{test,types-owned}-{prototype,release}-final.log`. Root added the narrow `SEMIO_STARTUP_PHASE_ONLY=1` selector to the existing registered `typescript-artifact/📜️script.ts` test/types-owned route, retaining the actual startup, service and bootstrap Source roots. That route uses its preauthored finite Original and existing owned child capture; no executable/launch command was added.

## Source freeze and limits

Paths have maximum absolute length 140, below 256. The three production bodies and three schema/test files are frozen at this checkpoint; other coedited generic store, Nx graph, bootstrap tooling and selected-snapshot owners were not altered by this task.

| Path below caching | Bytes | SHA-256 |
| --- | ---: | --- |
| `🚀️bootstrap/📜️script.ts` | 57553 | `1694a56435e1bb6391bf3212b86aa47bc359599c95ffea8ee32e215703b12418` |
| `🕸️graph/🏁️startup/🟦️.ts` | 2472 | `654def429ad0c8d7de49738f061f56e8746c77a9e8f95f09149461ffacfeef04` |
| `🕸️graph/🏁️startup/🚦️service/🟦️.ts` | 24190 | `b85c9bb33aa8aa10511e30f046e6e21eba1b9ae0f2f92bd69b22fac87c9611b5` |
| `🕸️graph/🏁️startup/📥️phase/🧬️schema/🔣️.json` | 1247 | `7992ec0add9cb63a0cdd346b132b06b88b3aa0d7e250f6a8ca94945f98220ca2` |
| `🕸️graph/🏁️startup/📥️phase/🧪️tests/🟦️.ts` | 2443 | `39292f385ec30be0506faddb520f2d240780290900e4f315b3953cd8355d8796` |
| `🕸️graph/🏁️startup/📥️phase/🧪️tests/🧫️fixtures/🔣️.json` | 2231 | `7624e1a14a17f30f7471f280c793f2660838176d10f57f6ee24b421b84ebca8e` |

The earlier Bun readonly membership checkpoint remains independently frozen in `javascript-dependency-readonly-membership-current.md` (final BOTH 27/242, owned strict BOTH 0). No physical bootstrap, Native cache/image, daemon, SDK, dependency installation, compiler, build, or network operation was run in this task. AST/type evidence does not establish actual service lifetime or process-tree behavior. Existing authored service issuance policy and journal worker lifetime were not replaced or qualified here.
