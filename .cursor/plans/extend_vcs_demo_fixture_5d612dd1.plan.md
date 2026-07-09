---
name: Extend VCS Demo Fixture
overview: Extend `DocumentVcsStore` with a small, backward-compatible "checkout" mechanism so alternatives can genuinely fork from any earlier checkpoint (not just the latest), then rewrite the `vcs/play` demo fixture into a rich ~15-checkpoint, 5-alternative history exercising real branch forks, batched multi-edit commits, 3 authors, and 2 new operation kinds.
todos:
 - id: store-checkout
   content: Add checkoutCheckpoint command, currentCheckpointId tracking, and checkoutCheckpointInternal helper to DocumentVcsStore in vcs/core/index.ts
   status: completed
 - id: store-fork-fix
   content: Rewrite commitCheckpoint/createAlternative/switchAlternative to use currentCheckpointId (not always-last-checkpoint) and grow the active alternative's checkpointIds on each commit
   status: completed
 - id: store-tests
   content: Extend vcs/core/index.ts test block with fork + checkout round-trip cases
   status: completed
 - id: fixture-ops
   content: Add status/tags fields to VcsDemoProjection and setStatus/addTag/removeTag operations in vcs/play/index.ts
   status: completed
 - id: fixture-authors
   content: Add third author (Carol) to VCS_DEMO_AUTHORS
   status: completed
 - id: fixture-seed
   content: Rewrite seedVcsDemoHistory into ~15-checkpoint/5-alternative history with real forks (checkoutCheckpoint) and batched multi-edit commits
   status: completed
 - id: fixture-tests
   content: Extend seedVcsDemoHistory test assertions for new scale and real-fork proof
   status: completed
 - id: verify-fixture
   content: Run vcs-core/vcs-play tests and bun run dev:vcs, screenshot the branching History graph
   status: completed
isProject: false
---

## Current limitation

`DocumentVcsStore.dispatch` in [vcs/core/index.ts](vcs/core/index.ts) always resolves the parent of a new checkpoint as `this.envelope.vcs.checkpoints.at(-1)` (line 270) and `createAlternative` always tags `this.envelope.vcs.checkpoints.at(-1)?.id` (line 295) — regardless of what is actually "checked out". Also, an `Alternative`'s `checkpointIds` is set once at creation (`[checkpointId]`) and never grows on subsequent commits (lines 297-308). Net effect: the store can only produce a single linear checkpoint chain with alternatives acting as static tags — no real forking, so the History graph always renders as a straight line no matter how the fixture is written.

Since `vcs/play`, `draw/play`, `forms/play`, etc. all share this same `vcs/core` store, this is the correct place to fix it once at the root.

## Part 1 — Add checkout/fork support to `DocumentVcsStore`

In [vcs/core/index.ts](vcs/core/index.ts):

1. Add `checkoutCheckpoint` to `DocumentVcsCommand`:

```typescript
| { readonly kind: "checkoutCheckpoint"; readonly checkpointId: string };
```

2. Add a private `currentCheckpointId: string | undefined` field to `DocumentVcsStore`, initialized in the constructor and in `setEnvelope` from `envelope.vcs.checkpoints.at(-1)?.id` (preserves today's behavior when nothing ever checks out explicitly).

3. Add a private helper:

```typescript
private checkoutCheckpointInternal(checkpointId: string): void {
	const checkpoint = this.envelope.vcs.checkpoints.find((entry) => entry.id === checkpointId);
	this.appliedEditIds = checkpoint ? editIdsForChanges(this.envelope, checkpoint.changeIds) : [];
	this.redoEditIds = [];
	this.currentCheckpointId = checkpointId;
}
```

4. Rewrite `commitCheckpoint` to resolve `parent` via `this.currentCheckpointId` (instead of `.at(-1)`), and after building the checkpoint: set `this.currentCheckpointId = checkpoint.id`, and if `this.envelope.activeAlternativeId` is set, append the new checkpoint id to that alternative's `checkpointIds` (so a branch's tag keeps following its own tip):

```typescript
const activeAltId = this.envelope.activeAlternativeId;
const alternatives = activeAltId ? this.envelope.vcs.alternatives.map((alt) => (alt.id === activeAltId ? { ...alt, checkpointIds: [...alt.checkpointIds, checkpoint.id] } : alt)) : this.envelope.vcs.alternatives;
```

5. Rewrite `createAlternative` to branch from `this.currentCheckpointId` instead of `checkpoints.at(-1)?.id`.

6. Rewrite `switchAlternative` and the new `checkoutCheckpoint` to both go through `checkoutCheckpointInternal`; `checkoutCheckpoint` additionally auto-detects `activeAlternativeId` by finding the alternative (if any) whose `checkpointIds.at(-1) === checkpointId`, else clears it (so checking out a non-tip ancestor and then calling `createAlternative` creates a genuine new fork).

7. Extend the existing `DocumentVcsStore` test block (do not add a new test file) with cases for: a `checkoutCheckpoint` + `createAlternative` sequence producing two checkpoints with the same `parentId` (real fork), and `checkoutCheckpoint` restoring `projection()` to the checked-out state.

This is backward compatible: any usage that never calls `checkoutCheckpoint`/`switchAlternative` behaves exactly as before (`currentCheckpointId` always advances to the last commit).

## Part 2 — Rich fixture in `vcs/play`

In [vcs/play/index.ts](vcs/play/index.ts), inside the `//#region 🔖demo` block:

1. **Widen the projection** — add `status: string` and `tags: readonly string[]` to `VcsDemoProjection`; update `emptyVcsDemoProjection()`.
2. **Add 2 new operation kinds** to `VcsDemoOp`/`applyVcsDemoOp`/`backwardsVcsDemoOp`: `setStatus`, and `addTag`/`removeTag` (inverse of each other), alongside the existing `setCounter`/`setTitle`/`setNotes` — 5 operation kinds total.
3. **Add a third author** (`Carol`) to `VCS_DEMO_AUTHORS`.
4. **Rewrite `seedVcsDemoHistory`** into a ~15-checkpoint, 5-alternative history with real branching, using the new `checkoutCheckpoint` command:
   - Main line: C1 → C2 (2 batched edits) → C3, later resumed as C8 → C12 (3 batched edits) → C13.
   - `feature-a` forks from C3: C4 (2 edits) → C5, later resumed as C9 (2 edits).
   - `feature-b` forks from **the same C3** (real fork — `checkoutCheckpoint(C3)` before `createAlternative`): C6 (2 edits) → C7, later resumed as C11.
   - `feature-a-hotfix` forks from **C4** (mid-branch, non-tip fork): C10.
   - `docs` forks from C2 (near root): C14.
   - `spike` forks from C1 (root): C15.
   - Net: 15 checkpoints, 5 alternatives, C3 becomes a genuine 3-way fork point (main/feature-a/feature-b), C4 becomes a 2-way fork point (feature-a/feature-a-hotfix); 5 checkpoints batch 2-3 edits before committing; 3 authors used in varying combinations (solo/pair/all-three) per checkpoint.
5. **Extend the existing test block** (`seedVcsDemoHistory` describe in `vcs/play/index.ts`): assert `checkpoints.length >= 15`, `alternatives.length >= 5`, and that at least one pair of checkpoints share the same `parentId` (proving a real fork exists), plus keep the existing `VcsPlayController` test.

No changes needed to [vcs/react/index.tsx](vcs/react/index.tsx) — its existing per-checkpoint "line to `parentCheckpointId`" rendering already supports multiple children sharing one parent row; it just wasn't previously exercised with real fork data.

## Verification

- `bun nx run @semio-tech/vcs-core:test`, `@semio-tech/vcs-play:test`.
- `bun run dev:vcs` — confirm the History window shows a branching graph (C3 splitting into 3 lanes, C4 splitting into 2), not a straight line; screenshot to confirm visually.
