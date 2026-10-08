# Root Native Terrain Grant Audit 27

Read-only actual caller-chain audit. No runtime gate, source edit, raw40 guard, shared build output, Git mutation, or lifecycle claim. Complete before/after/current text and SHA-256 identities are retained in `🗑️generated/root-native-terrain-grant-audit-27/full-body-receipt.json`.

Bounded receipt: 12819339 bytes, 34 work units, 0.096 seconds; ceilings 64 MiB, 65536 units, 60 seconds.

| Journal | Current SHA-256 | Current equals journal after |
| --- | --- | --- |
| native-world-terrain-production-proposal-1.json | `2cea703502c33ea2d3fe4850140611b74fc65aa05af1864effbfc590bd0fc298` | False |
| native-world-terrain-renderer-receiver-full-pair-1.json | `71b00099f81788f581651de4e9b22c3a2f65d2963f9939e3c70179241fc025f6` | True |
| native-world-terrain-original-receiving-tests-full-pair-1.json | `28f6362879ddee6ca4ca55482444ab223f3cbc2e7cdf1df9101c440b194fd6da` | True |
| native-world-terrain-original-revision-custody-full-pair-1.json | `2cea703502c33ea2d3fe4850140611b74fc65aa05af1864effbfc590bd0fc298` | False |
| native-world-terrain-close-refusal-full-pair-1.json | `2cea703502c33ea2d3fe4850140611b74fc65aa05af1864effbfc590bd0fc298` | True |
| native-world-terrain-independent-release-policy-full-pairs-1.json | `b3cd340effcc252009002169d6a1438f5947e2a55beaf3803242795a2cd0a4d7` | True |
| native-world-terrain-independent-release-policy-full-pairs-1.json | `b4b4d8d43fd2e60f8c8bdd9bf6f6d703f2b68d429935026bd18688aabd290cc5` | True |

## Findings

FrameTransaction World3dSnapshot receiver at Renderer lines15987–15995 supplies separate fixed physical axes (items1, copy4096, capacity65536, release16MiB, depth64), passes actual StepContext, validates ownership.fits, and consumes typed terrain faults. Demand does not mint those grants. Constructor at World9048–9057 returns the original payload on refusal; begin_world_terrain_pending restores all five retained tuple elements at9300–9305. Original unit tests remain in the journal/current receipt, including pointer/capacity refusal and zero grant assertions. No test execution is certified here.

WorldTerrain live cursor distinguishes Busy (Pending, original owner/phase retained) from other mesh faults, which latch a semantic fault and advance controlled retirement. Closing is acknowledged only when initiating abort/close; exact close steps keep receipts. Source ControlledRetirement admission refusal restores id and all original vectors at9226–9230. Writes meter actual12/16/4 copied bytes; mesh begin/allocate return physical receipts; publication accounts key copy/capacity; source close forwards exact retirement progress. Actual StepContext yield/cancellation/stage/fuel is used, rather than no-op scalar job bridges.

Concrete receiver gap: step_world3d_dynamic_retirement at2690 returns bool and constructs its own terrain grant at2703. Renderer close_world_cursor_wake_step at13291 consumes only that bool plus terminal predicate. It neither supplies the physical grant nor receives typed terrain progress/fault; terrain_ownership is stored in state and semantic faults are marked internally. This is demand-independent but does not establish caller-funded typed retirement receiver semantics. Expose caller-owned grant and typed progress/refusal at the actual close receiver while retaining all existing assertions. FrameTransaction's proper typed receiver must not be used to certify this separate close path.

Scope limitation: retained journals and complete current World/Renderer/unit sources were inspected; underlying ControlledRetirement implementation and mesh registry implementation were not independently audited here. Consequently exact lower-layer retirement correctness and runtime green are not certified. Source publication alone does not remove the known Kernel dependency refusal reported by Native.
