---
name: fix board wasm reentrant borrow
overview: Stop `pushSceneToWasmDriver` from being called while another `&mut BoardSession` borrow is outstanding (during async `attach_canvas`), which throws `borrow_fail` and freezes the canvas. Guard the unconditional push commit 379 introduced, and re-invalidate when the borrow is released so the canvas paints again.
todos:
  - id: guard-push
    content: Guard pushSceneToWasmDriver against wasmSessionBorrowDepth/wasmGpuFrameDepth and mark invalidated for retry.
    status: completed
  - id: reinvalidate-attach
    content: Re-invalidate from initGpuSurfaceOnce finally block when borrow depth returns to zero.
    status: completed
  - id: drop-sync-render
    content: Drop synchronous renderer.render() from applySize in index.tsx.
    status: completed
  - id: verify
    content: Run board vitest + playwright + manual play app sanity check (pan/zoom/select).
    status: completed
isProject: false
---

## 1 Problem

After commit `379`, `BoardRenderer.render()` calls `pushSceneToWasmDriver()` every frame:

```1815:1818:elements/client/lib/board/index.ts
} else if (this.canvas) {
    if (!this.suppressSceneToWasmPush) {
        this.pushSceneToWasmDriver();
```

`pushSceneToWasmDriver` immediately mutably borrows the wasm session:

```1981:1984:elements/client/lib/board/index.ts
const o = this.selectionOptions;
this.session.setSize(this.width, this.height, this.dpr);
this.session.setSelectionOptions(o.method, o.mode, o.target);
this.session.setWorldRasterTiling(this.worldRasterTiling);
```

`initGpuSurfaceOnce()` holds a `&mut self` borrow on the session across `await this.session.attach_canvas(...)`. Commit 378 tracks this with `wasmSessionBorrowDepth`, but only `readGpuReady` / `syncGpuReadyCacheFromSession` honour it. `pushSceneToWasmDriver` does not — so any `render()` that fires while `attach_canvas` is still awaiting throws `recursive use of an object detected which would lead to unsafe aliasing in rust` at `BoardSession.setSize`.

Side effect: the first throw escapes `render()` before `paintTextOverlays` or `applyCanvasDebugAttributes` runs, the dirty flag stays set, every following `invalidate()` re-throws, and pointer/wheel handlers' `invalidate()` calls also re-throw. Net result: canvas stuck on the first frame ("cut board") and every interaction "frozen". Drag-and-drop survives because it doesn't call the wasm session.

## 2 Fix

### 2.1 Guard `pushSceneToWasmDriver` against the active borrow

In [`elements/client/lib/board/index.ts`](elements/client/lib/board/index.ts) `pushSceneToWasmDriver` (line ~1977), bail out when another borrow is live, and re-mark dirty so a subsequent frame retries:

```ts
private pushSceneToWasmDriver(): void {
    if (this.suppressSceneToWasmPush) return;
    if (this.wasmSessionBorrowDepth > 0 || this.wasmGpuFrameDepth > 0) {
        this.invalidated = true;
        return;
    }
    // ... existing body
}
```

### 2.2 Re-invalidate when the borrow is released

In [`elements/client/lib/board/index.ts`](elements/client/lib/board/index.ts) `initGpuSurfaceOnce()` (around line 1867), schedule another frame after `wasmSessionBorrowDepth` returns to zero so the deferred push actually runs:

```ts
this.wasmSessionBorrowDepth += 1;
this.cachedWasmGpuReady = false;
try {
    await this.session.attach_canvas(this.canvas, lw, lh, dpr);
} finally {
    this.wasmSessionBorrowDepth = Math.max(0, this.wasmSessionBorrowDepth - 1);
    if (this.wasmSessionBorrowDepth === 0) {
        this.invalidate();
    }
}
```

### 2.3 Don't synchronously re-enter `render()` from `applySize`

In [`elements/client/lib/board/index.tsx`](elements/client/lib/board/index.tsx) `applySize` (line ~756), drop the eager `renderer.render()` and rely on `setSize` → `markDirty` → `invalidate` (RAF). This avoids stacking a synchronous render on top of the just-scheduled RAF render and makes any future re-entrancy regression behave gracefully via `renderPipelineDepth` alone:

```tsx
const applySize = (): void => {
    const nextWidth = width ?? container.clientWidth ?? 1;
    const nextHeight = height ?? container.clientHeight ?? 1;
    renderer.setSize(nextWidth, nextHeight, globalThis.devicePixelRatio || 1);
};
```

This reverts the half of commit 378 that was kept "to match prior behavior" but is no longer needed once 2.1/2.2 land. The Playwright spec timing it was meant to help is independently fixed by the explicit `data-board-surface-state !== "init"` poll already in [`board-play-gpu.spec.ts`](elements/client/lib/board/play/e2e/board-play-gpu.spec.ts) lines 17–19.

## 3 Verification

- Reload `bun ./script.ts dev` for the board play. Confirm no `borrow_fail` in console, all three panes reach `data-board-surface-state="ready"`, panning with middle-click (or `Shift+left-drag` on empty area), wheel zoom, and node click selection all work.
- Run `bunx vitest run --config vitest.config.ts` in `elements/client/lib/board` — all existing tests must still pass.
- Run `bun ./script.ts e2e` for the board play Playwright spec.

## 4 Ticket

Open / reopen a repo MCP ticket under goal that includes `BOARD-WASM-GPUREADY-REENTRANCY` (this is a follow-up to that exact ticket). Add log notes referencing commit `379` as the regression source.

## 5 Out of scope (separate follow-ups)

- "Pan with left-drag empty" UX. Today pan is only middle-button or `Shift+left-drag on empty`; left-drag empty starts marquee. Not a freeze.
- The `delete (this.canvas).__boardRenderer` happening in deferred `R1.dispose()` after R2 already set the expando. Currently harmless in production but worth fixing alongside (just check `this.canvas.__boardRenderer === this` before deleting).
- Discarded `drainEventsJson()` at the tail of `pushSceneToWasmDriver`. Latent footgun, separate cleanup.

## 6 Follow-up (why the first fix was incomplete)

### 6.1 What worked (first wave)

- Guarding **`pushSceneToWasmDriver`** when `wasmSessionBorrowDepth > 0` or `wasmGpuFrameDepth > 0` stopped **`setSize`** from racing **`attach_canvas`** during the same `render()` path.
- **`invalidate()`** after `attach_canvas` `finally` ensured a frame ran once the borrow was released.
- Removing synchronous **`renderer.render()`** from **`applySize`** reduced nested render pressure.

### 6.2 What did not work / user still saw `borrow_fail`

Guarding **only** `pushSceneToWasmDriver` was **insufficient**: many other call sites call **`this.session.*`** while `attach_canvas` still holds `&mut BoardSession` across its `await`. Any of these can throw the same wasm-bindgen error (often still reported at `setSize` because that was the first session call in `push`, or at another entry point):

- **`setCamera`** (React `camera` prop / `BoardHostSubtree` layout effect) — very likely with **three canvases** mounting and props updating during GPU init.
- **`setSelectionOptions`**, **`setWorldRasterTilingOption`**
- **`setSelectionIds`** (after `push` returned early, **`setSelectionIdsJson` still ran** → crash)
- **Pointer / wheel / contextmenu** handlers and **`deleteSelection`**

### 6.3 Current hypothesis (second wave)

Introduce **`wasmSessionCallBlockedForReentry()`** (`wasmSessionBorrowDepth > 0 || wasmGpuFrameDepth > 0`) and:

- **Defer all `BoardSession` mutations** on those paths: update JS-side state where possible (`setCamera` updates `camera` + store + emit, then returns; next successful `pushSceneToWasmDriver` syncs WASM via trailing `setCamera` in push).
- **Skip pointer/wheel/context** during the block (set `invalidated` / `invalidate` for retry) so we never call `pointerDownScreen` / `wheelScreen` / … concurrently with `attach_canvas`.
- **`setSelectionIds`** when blocked: use **`updateSelection`** (scene + store) and skip WASM until unblocked.

### 6.4 How to test (WebGPU on developer machine)

- **Playwright:** new test **`no wasm borrow_fail during load and viewport resize stress`** in [`elements/client/lib/board/play/e2e/board-play-gpu.spec.ts`](elements/client/lib/board/play/e2e/board-play-gpu.spec.ts) — collects `console` errors + `pageerror`, waits for all three canvases `data-board-surface-state=ready`, then **resizes the viewport** several times (ResizeObserver → `setSize` → `invalidate` → `render` stress).
- **Bundled Chromium often has no WebGPU adapter** on Windows; use **installed Chrome**: `BOARD_PLAYWRIGHT_CHANNEL=chrome` (documented in [`play/playwright.config.ts`](elements/client/lib/board/play/playwright.config.ts)).

### 6.5 Outcomes table (update as you verify)

| Approach | Result |
|----------|--------|
| Guard `pushSceneToWasmDriver` only | Reduced `setSize` races; **user still hit `borrow_fail`** |
| Fence **all** `session` entry points during attach / `renderFrame` | **Implemented** in `index.ts` (`wasmSessionCallBlockedForReentry`); verify locally with WebGPU + optional `BOARD_PLAYWRIGHT_CHANNEL=chrome` |
| Playwright resize-stress + console capture | **Automated regression signal** |
