# Marker-Only Root Activation Result

## Implementation

Completed the scoped lifecycle fix. Marker-only bar/control roots now activate shared-boundary paint without a WindowChrome.

Internal API in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts`:

- `retainSharedBorderRoot(root)` returns `{ compositor, release }`. Every window and mounted adopted frame holds an independent lease. Release is idempotent, and only the last lease disposes the root compositor.
- `useSharedBorderFrameRef<T>(forwardedRef?)` composes a stable host ref with lifecycle activation. Object refs, ordinary callback refs, and callback cleanup refs are preserved. It uses the same ShellScope/closest scope/body fallback as windows.
- `useSharedWindowBorder` now retains/releases through the common lifecycle, rather than disposing whenever its window map becomes empty.
- Disposal is idempotent, cancels queued animation frames, disconnects observers, restores attached paint/position, removes listeners, and prevents later scheduling on a disposed compositor.

No import-time registration, permanent body observer, perpetual idle RAF, public barrel dependency, geometry change, CSS change, Canvas/Resizable/native change, or generated-manifest edit was introduced. Existing chip paths and effect rendering remain unchanged.

## Changed Files

Under `🧰️framework/🔨️modules/🖱️ui/🧱️elements/`:

- `🗂️WindowChrome/shared-border.ts`
- `🗂️WindowChrome/shared-border.test.ts`
- `🗂️WindowChrome/compositor.schema.json`
- `🗂️WindowChrome/compositor.cases.json`
- `🔳️ButtonGroup/🟦️.tsx`
- `🎛️ToggleGroup/🟦️.tsx`
- `🔀️Toggle/🟦️.tsx`
- `⚡️ActionGroup/🟦️.tsx` (ActionGroup and standalone Action)
- `🔝️Navbar/🟦️.tsx`
- `🔚️Footer/🟦️.tsx`
- `🔽️Select/🟦️.tsx` (trigger and portaled content)
- `📑️Tabs/🟦️.tsx` (trigger)
- `🗨️Popover/🟦️.tsx` (portaled content)
- `🗨️Popover/🧪️tests/🧩️component/🔸️routes-sibling-escape-focus/🟦️.tsx`

Control edits are limited to internal hook imports and host-ref composition. Existing adoption markers and other agents' changes were preserved. The Popover observer test previously modeled only one ResizeObserver; it now notifies all live observers and returns a complete content rectangle, because compositor activation legitimately introduces an independent observer.

Ticket artifacts: `root-activation-api.md` and this report. No generated files created by this assignment were retained; the shared generated folder belongs to integrated lead validation and was not removed.

## Validation

- TDD: initial marker-only test failed because `retainSharedBorderRoot` did not exist; implementation then passed it.
- Expanded existing language-neutral compositor schema/fixtures for marker-only seam expectations and both window/frame release orders. AJV validates the fixture; JSDOM and React DOM independently exercise resulting ownership and lifecycle behavior.
- Existing permanent config already includes `🗂️WindowChrome/shared-border.test.ts`; no config change is needed.
- Final focused permanent-config run: **3 files, 29 tests passed** (WindowChrome compositor, WindowSilhouette helpers, Popover component suite).
- Lifecycle coverage includes marker-only sibling ownership, hover transfer, separate roots, shared instance identity, idempotent releases, replacement after last release, StrictMode, both window/frame teardown orders, forwarded object/callback-cleanup refs, restored attributes/styles, RAF cancellation and idle draining, and no reaction to mutation/pointer/resize after last participant leaves.
- Focused TypeScript check passed for every changed source component and the compositor test, using ES2022/bundler/react-jsx/Vitest types.
- Final tracked source diff reviewed; shared-border source/test/fixtures are already untracked in this worktree and were inspected directly. Scoped `git diff --check` passed. No modifying git commands were used.

Commands used the absolute repo launcher and ticket `run-ui-tests.mjs` normal permanent configuration.

## Remaining Validation / Risks

Full permanent UI suite was run twice. Final result: **53 files passed, 1 failed; 1279 tests passed, 1 failed, no unhandled errors**. The sole remaining failure is outside this assignment: Diagram's `cursorizes live controlled 20,000-node and edge setup before cooperative projection` exceeds its 5000ms timeout (about 10s in final run). No Diagram files were changed. The first full run exposed the now-fixed Popover test observer assumption; its final suite passes.

Lead retains browser paint verification and ticket lifecycle. No browser verification was duplicated. Existing axis-aligned geometry limits and body fallback policy remain unchanged.
