# Mounted Caller Contract Decision 1

## Current Source Receipt

Root read the complete GenericPlugin, GeneralJob and Kernel Cargo manifest twice under64MiB/65536 work/60 seconds. The exact observed source pairs are in generated/root-mounted-caller-contract-decision-1.json. Charged bytes6676038. All three bodies were exact between observations. GenericPlugin SHA-25627354ba88158479c071ee4e4330f90ddbc6e34f2a72d661e3470e92f2b4304de; GeneralJob25b25d262403a5f7bff116597392842698c1156a08a40e653f507f286963870d. No source or runtime was changed. Initial guessed top-level Cargo path was absent; the actual owning manifest is OS/packages/rust/Cargo.toml. The actual runtime receiving methods are inside GenericPlugin, so no inferred Kernel facade is needed.

## Actual Scheduling Authority

GenericPlugin RuntimeLiveCleanupJob.step at43212 receives the real StepContext, checks cancellation and contention, consumes fuel, then computes next_maintenance_byte_demand and calls runtime_live_maintenance_step at43237–43238. That helper invokes only maintenance(1, bytes). Its typed progress check at43246 admits only released_items/released_bytes. The mounted grant cannot originate in the nested2D session because it never reaches this host boundary.

Runtime close similarly receives real StepContext, closes the retained maintenance session at43490–43508, then derives artifact_close_release_grant from next_close_byte_demand at43528 and invokes app.close_step at43532. It checks only item/release byte output. Its terminal witness at43555–43559 must remain exact. Both runtime job close methods themselves still have the removed two-scalar InteractiveJob shape at43279 and43577, and the maintenance session driver still destructures removed scalar Pending/Complete forms. A mounted hook signature alone cannot close this real dependency/ownership chain.

The registered typed-operation settle fixture at7952–7968 also drives scalar app maintenance and checks exact progress before publishing/acknowledging all result lanes. Preserve its full original deadlines, ACK/page/completion/revision/effect/event laws when adding the actual mounted owner turn; it must continue using the same host semantics.

## Required Canonical Port

Use the existing first-party RetainedCloneGrant, RetainedCloneProgress and InteractiveJobCloseStep vocabulary. GeneralJob already carries all physical axes, Refused, Blocked and a Complete terminal admission check. Do not introduce a parallel retained budget, scalar adapter, optional grant, demand-funded allowance, default infinite depth or no-op cancellation/progress implementation.

Declare the mounted owner operation schema before bindings: explicit phase/instance identity, mandatory caller-owned physical grant, real operation/cancellation control, exact typed consumed progress, retained refusal/blocked state and terminal witness. Demand is descriptive only. The actual scheduling owner supplies independent item/copy/capacity/release/depth allowances before the turn. Wrappers may legitimately narrow the supplied grant and must return all consumed axes unchanged. They may not reconstruct it from release demand or two existing scalars.

Thread the port through the real runtime maintenance/close jobs, PluginApp, VcsArtifactApp mounted stage/close state, ArtifactApp/Editor/Viewer and both concrete2D/3D sessions. Delete the former scalar mounted API and every mounted scalar forwarding call. A separate mandatory mounted-owner operation can keep unrelated generic owner families on their own unaccepted work frontier, but it must be invoked by the actual host and cannot exist solely as an unused trait. Complete must preserve the current exact nested terminal witness; worker-owned shells remain Blocked with a wake source. Port the real runtime InteractiveJob cleanup receivers to the current canonical typed contract, preserving their owner/control state and all original tests.

The specific sessions must pass the same physical authority into MeshJob.close_step and actual preparation try_reserve_exact ownership transitions. A signature-only3D port does not certify3D runtime. The synchronous nine-case neutral geometry corpus does not certify production allocation, suspension or cleanup. Keep these qualifications until actual receiving runtime proves them.

## Validation Obligation

Retain the complete original geometry, cancellation, revision publication, worker blocking, terminal-empty, no-heap-refusal, page ACK and runtime stall/deadline laws. Add one language-neutral corpus covering zero/short independent copy/capacity/release/depth/items grants, exact receipts, retained refusals and false terminal prevention. Run first-party schema and an existing independent third-party schema/reference oracle. Require actual host-to-session invocation and all original native owning tests; dependency refusal is red and cannot be treated as feature acceptance. No compatibility alias or shortened census is authorized.

## Hand-off

Catalogue owns the concrete FEM mounted chain after the currently admitted neutral oracle interval and strict schema correction close. Native owns the terrain dynamic close caller port. Interface owns fresh41 source joins and deletion/enforcement gates. This decision identifies the real next source work; it does not certify compilation, execution or publication.
