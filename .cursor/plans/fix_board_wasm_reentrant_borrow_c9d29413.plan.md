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
    status: cancelled
  - id: verify
    content: Run board vitest + playwright + manual play app sanity check (pan/zoom/select).
    status: pending
  - id: e2e-honest
    content: Document why Vitest does not cover main-thread WebGPU; add/adjust real-browser checks (Playwright+Chrome or manual script).
    status: pending
isProject: false
---

## 0 Why “all tests pass” but the app is still broken

- **Vitest (`elements/client/lib/board`)** almost exclusively constructs `BoardRenderer` with **`renderMode: "headless-test"`** (see `index.ts` Vitest region). That path **never** runs `initGpuSurfaceOnce` / `attach_canvas` / `syncGpuFrame` / the real WebGPU present loop.
- **`initSync` WASM** in Vitest loads the wasm bytes from disk; it does **not** reproduce Chromium’s WebGPU adapter, `ResizeObserver` + flex layout from `@elements/ui` windows, multi-pane focus, or `device.poll` re-entry.
- **Playwright GPU specs** often **skip** on bundled Chromium without a WebGPU adapter (`board-play-gpu.spec.ts`); unless CI runs **`BOARD_PLAYWRIGHT_CHANNEL=chrome`**, the “GPU” tests are frequently no-ops.
- **Conclusion:** Passing Vitest proves **headless WASM + scene graph invariants**, not **board play triptych + WebGPU + DOM layout**. Treat Vitest green as **necessary, not sufficient**.

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
- Run `bunx playwright test --config play/playwright.config.ts` in `elements/client/lib/board`. With bundled Chromium (no adapter), GPU + stress tests **skip**; on a WebGPU machine run **`BOARD_PLAYWRIGHT_CHANNEL=chrome bunx playwright test …`** so both run and the stress test asserts **no** `borrow_fail` / `unsafe aliasing` / `recursive use of an object` in console after viewport resizes.

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
- Removing synchronous **`renderer.render()`** from **`applySize`** was tried to reduce nested render pressure; **superseded** by **368 parity** (§9) — `applySize` again calls **`renderer.render()`** after **`setSize`**.

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
| **`setPointerCapture` before WASM re-entry guard** | **Likely root of “no mouse after init”**: if `pointerdown` ran while GPU attach borrowed the session, we returned early **after** capturing the pointer, so the canvas kept capture while WASM never saw `pointerDown` — **permanent broken input**. **Fix:** `releasePointerCapture` on the blocked early-return path. |
| Flex `h-full` in nested flex (Golden Layout) | **Hypothesis for “canvas cut at bottom”**: inner stack used `h-full` without establishing a flex column chain; **Fix:** `flex flex-1 min-h-0 flex-col` on container + inner + `flex-1 min-h-0` on canvases. |
| **Commit `369` (`c3889ca27`) dual canvas in `BoardCanvas`** | **First DOM divergence after `368`:** second stacked canvas (`textOverlayRef`) + inner wrapper around the WebGPU canvas (`elements/client/lib/board/index.tsx`). **`368`** used a **single** canvas as the flex child measured by `ResizeObserver`. Dual layer risks flex height, compositor quirks, and hit-testing. **Mitigation (2026-05-16):** revert to **one canvas** (GPU captions skipped until overlay is reintroduced safely). |
| `BoardRenderer.activeRenderer` removed from constructor (`369` `index.ts`) | **Low** for pan/zoom; affects **Delete/Backspace** (`handleWindowKeyDown` checks `activeRenderer`). |

## 7 Hypotheses (ordered — update as disproven)

1. **H1 — Dual canvas / wrapper (`369` `index.tsx`)**  
   Stacked overlay + inner `div` broke layout or targeting vs **`containerRef`** sizing. **368 = one canvas.**  
   **Status:** **Mitigated** by single-canvas revert (2026-05-16).

2. **H2 — WASM `borrow_fail` / half-present**  
   `attach_canvas` + overlapping `setSize` / `renderFrame` / `device.poll`.  
   **Status:** **Partially addressed** (Rust `Rc<RefCell<…>>` + `future_to_promise`; JS fences; render schedule closer to `368`).

3. **H3 — `pushSceneToWasmDriver` before `gpuReady`**  
   **368** only pushed inside `syncGpuFrame` after `gpuReady`.  
   **Status:** **Restored** 368-style flow + `pushScene` immediately before `renderFrame` inside `syncGpuFrame`.

4. **H4 — `wasmSessionCallBlockedForReentry` / `readGpuReady` stuck**  
   Could block all pointer entry while a partial frame shows. Needs **`[DEBUG]`** logs on attach, `gpuReady`, blocked handlers.

5. **H5 — UI layer above canvas**  
   Toolbar/modal/invisible Radix layer. **368 play** already had `BoardPaneChrome` capture — lower probability unless later UI commits add a full-screen veil.

6. **H6 — Stale wasm under Vite**  
   `play/vite.config.ts` ignores `../rs/**` — Rust edits may not rebuild `rs/pkg`. **Verify:** wasm rebuild + hard reload.

## 8 Next discovery steps (real browser)

1. Rebuild wasm (`bun ./rs/scripts/build-wasm.script.ts`), hard reload.
2. On each `[data-testid=board-canvas]`, confirm `data-board-surface-state=ready`.
3. If broken: copy **first** console error + stack (`borrow_fail`, `GPU`, `Validation`, etc.).
4. Run **`BOARD_PLAYWRIGHT_CHANNEL=chrome bunx playwright test --config play/playwright.config.ts`** so WebGPU tests do not skip on stock Chromium.
5. If still ambiguous: **`git bisect`** `a01093653..HEAD` with a short manual script (pan, wheel, select).

## 9 Plan / code drift notes

- Section **2.3** (“drop `renderer.render()` from `applySize`”) was implemented then **superseded**: **`368` always called `render()` after `setSize` in `applySize`**; that was restored for 368 parity.
- **`verify` todo** in this plan is **not sufficient** if Playwright GPU tests skip; treat **Vitest green** as **headless-only** signal (see §0).
