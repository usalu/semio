# Retained Child Publication and Atomic Root Exposure Audit

Status: **RED — current uncommitted source review only.** No Cargo, Nx, or runtime gate was run. The patch establishes useful child-lane state and prevents a successful group from exposing intermediate immutable roots, but it is not yet a safely retained or all-or-nothing publication boundary.

## Current positive evidence

- `PendingChildGroupPublication` holds the child emissions, parent mutations, description, bounded receipt/fault, and an explicit phase at [`plugin/🦀️.rs:16070`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16070). Its `close_step` uses the existing `ChildEmit::close_one` and scalar retirement; `MountedTypedCommandFullOperation::retirement_step` checks terminal emptiness before dropping it at [16410](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16410).
- The child result is now ACK-bound: an ACK of the `Child` page only advances a `Committed` pending child publication at [16376](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16376). The committed receipt can therefore be resent without another group dispatch.
- The new root builder in `dispatch_emit_group` starts from one immutable `ChildContentView`, captures every nonempty target into a local `next`, and swaps `child_content_root` once only after every capture succeeds at [21107](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21107). This is a real improvement over the earlier per-child publication loop: a successful N-child group exposes one new root and one generation.
- `ChildContentView::is_empty` is based on a missing root or `root.len == 0` ([8699](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8699)). A locally captured `next` is nonempty after its first successful capture, so the error branch does place any partial candidate root into `ChildContentRetirement`; it does not directly drop that root.
- Existing tests already cover root snapshot isolation and retirement mechanics ([35499](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:35499) onward), and store tests prove phase-1 `dispatch_group` rejects without applying members ([store `🦀️.rs:27047`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:27047)). Neither reaches the new mounted typed `Child` path.

## Material defects

### 1. Extracted retained input is lost on ordinary pre-dispatch faults

`publish_mounted_typed_child_operation_unit` removes `pending` from `mounted` immediately ([22467](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22467)), then takes the cancellation claim and captured root with `?` ([22490–22493](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22490)), and uses `?` again for a captured revision or absent dialect ([22508–22509](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22508)). Each error returns while `pending` is local and nonterminal. `artifact_mutations`, `ChildEmit`s, and the description can then fall out of scope rather than enter the existing bounded close path.

The same issue appears after `take_dispatch_owners` ([22531](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22531)): `dispatch_emit_group` receives the only vectors. A phase-1 validation failure returns an error after the vectors have been consumed, and the caller marks an already-empty pending owner faulted. This is not a retained failure result.

**Required repair:** no fallible branch may own a removed pending owner. Add a private restore/reject helper that first replaces `mounted.pending_child_publication`, then creates the fault/close transition. Replace `take_dispatch_owners` with a private dispatch operation/result whose failure returns the exact three inputs; do not model a failed call as a bare `Fault` once input has crossed the boundary.

### 2. Cancellation does not use the operation's retained cancellation transition

The generic publisher calls `reject_cancelled_publication()` before claiming publication ([22600](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22600). The child publisher instead attempts a claim first ([22489](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22489), subject to defect 1) and only later checks `permit.is_cancelled()` ([22523](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22523)). A cancellation already observable before the claim can therefore lose the pending owner; it also does not produce the normal bounded fault-page/retirement transition.

**Required repair:** retain/restore `pending` first, call the same cancellation transition used by scalar publication, and have it place the pending child owner in `Closing`. A post-claim cancellation remains a pre-linearization refusal and must not invoke group dispatch.

### 3. Root exposure is atomic only after durable member dispatch, not atomic composition publication

`CompositionCoordinator::dispatch_group` commits child and parent member edits before the new root capture starts ([store `🦀️.rs:19361`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19361); call at [21104](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21104)). A capture error then asks `undo_group` to compensate. That is an asynchronous best-effort recovery after durable member mutation, not one linearized all-or-nothing publication.

The compensation call also passes every live `self.children` member rather than the receipt's exact target set ([21119–21121](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:21119)). `undo_group` skips foreign tails, but unrelated members affect work/diagnostics and cannot establish that only the originally touched coordinate set was considered. If a touched tail has changed or undo fails, the source returns a fault with durable group edits still present and no retained reconciliation owner.

**Required repair:** first packet should be honest: call this **atomic root exposure**, not atomic composition publication. For an all-or-nothing commit, add one retained prepared group owner that (a) captures all candidate roots and reserves retirement capacity before `dispatch_group`, (b) retains the exact receipt target set for compensation/reconciliation, and (c) keeps the recovery state until every exact member has either been undone or a bounded retry/reconciliation terminal record exists. Do not pass all map members to this path.

### 4. Child-only typed groups lack a parent history tail and need their route selected explicitly

`dispatch_emit_group` correctly permits no parent operations and returns child `member_edits`; however the current generic history routing remains parent-tail based outside the new group function. This is the previously filed global history-routing RED, not repaired by the root swap. The Child receipt contains only an invocation id, count, and generation ([16061](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16061)), insufficient to route a child-only undo/replay by exact member edits.

**Required repair:** integrate the existing planned applied/redo route owner before treating a child-only typed operation as a complete user-facing mutation path. The pending child group must retain or hand its exact member edit route to that owner after commit; a parent-only history lookup is not a fallback.

## Compile/API risks to resolve before a cold native run

1. `pending` must be restored before every `?` mentioned above; compiler acceptance would not make the implicit strict-owner drop sound.
2. `pending.commit(...)?` at [22550](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22550) has the same structural rule. It is logically expected to succeed, but should use an infallible state transition after the receipt is known or restore `pending` before returning a state fault.
3. The existing `ArtifactFixedRegistry::insert_admitted` asserts prior capacity ([15578](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:15578)). This is valid under the exclusive app actor only; do not describe `admit_child_content_publication()` as a cross-await reservation. The pre-dispatch root candidate path must keep all allocation/fallibility before the non-await exposure block.
4. The present success receipt is an acknowledgment token, not authority to reconstruct group membership. Do not broaden it with raw typed mutations; use the internal exact group receipt/history route.

## Smallest exact RED-to-GREEN proof

Add a schema-first `retained-child-group-publication-v1` corpus and an independent Bun state oracle. It models operation state, exact target coordinates/revisions/dialects, a one-slot root retirement ledger, a result token/ACK, and bounded close counts—never Rust `dispatch_group` internals.

| Case | Required observation |
| --- | --- |
| One accepted mixed parent + child group | One `dispatch_group`, one root generation advance, one root replacement after both captures, one Child result token; ACK only retires the same pending owner. |
| Child-only group | One exact child member-edit route is retained; no invented parent history tail. |
| Cancel before claim / after claim-before-dispatch | Zero member edits and root change; original child emits/mutations remain owned, then reach terminal through incremental close. |
| Stale parent root, stale child root/revision/dialect, missing target, duplicate target, saturated retirement slot | No dispatch and no direct drop; state returns a bounded fault and retains/retirements drain under grants `0`, `1`, and `4096`. |
| Store phase-1 rejection | Inputs return to the retained group owner; zero edits/root change. |
| Capture failure after member group commit | No partial root is exposed; exact receipt targets—not unrelated children—are used for compensation; fixture distinguishes complete compensation from retained reconciliation. |
| ACK retry / cancel after commit | Same receipt is retransmitted; it never re-dispatches. Cancellation after commit affects only delivery/retirement. |

Native integration belongs beside the existing composed-app tests at [`plugin/🦀️.rs:35499`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:35499), using a real `VcsArtifactApp<TestApp, TestMembers>` plus its registered typed command factory—not `dispatch_emit_group` or a registry-less action helper. The focused law should drive the actual mounted operation, expose/take/ACK the Child page under small grants, inspect the child view/history route, and then drive app maintenance plus operation retirement to terminal-empty. It must use an instrumented capture failure and a strict-drop sentinel on `A::Mutation`/`ChildEmit` inputs to prove defect 1 is fixed.

Register the source oracle and focused native selector in the existing framework OS Rust script/launch seed; generate launch artifacts from the seed. No browser, socket, Flow `addWidget`, genesis, or process claim follows until this kernel law and the separate Flow factory/history-route packets execute.

## Verdict

The root-swap edit is a sound *successful-path* exposure improvement. The retained typed publication is **not ready to mount or claim**: it has input-loss paths before dispatch and a post-dispatch compensation window. Fix retained handback/cancellation first, then prove the narrow lifecycle above. The broader durable atomic-composition and child-only history routing packets remain independent REDs.
