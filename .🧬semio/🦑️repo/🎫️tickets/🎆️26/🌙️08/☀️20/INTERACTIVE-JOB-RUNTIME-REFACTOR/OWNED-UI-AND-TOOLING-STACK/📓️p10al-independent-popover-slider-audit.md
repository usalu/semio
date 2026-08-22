# P10al Independent Popover and Slider Audit

## Verdict: REJECT

Independent read-only audit on 2026-08-22. I read `AGENTS.md` and the complete P10ad wave scout, inspected the live Popover/Slider implementations, their focused matrices, real ActionGroup/Toggle/Search/ShellSync consumers, the public barrel, manifests, and `bun.lock`, and ran the bounded JavaScript gates below. The requested P10ak implementation report was absent at audit start and remains absent; that is a provenance defect, but not the reason for this rejection.

## P0: Crossing a Thumb Loses Its Identity

`Slider` promises an operable tuple, including crossing. Its value normalization sorts the tuple (`🎚️Slider/🟦️component.tsx:90-100`), while every rendered thumb is keyed by its current positional index (`:455-474`). In `updateThumb` (`:240-249`), moving index 0 from `[20, 80]` to `100` produces `[80, 100]` and merely updates `activeThumbRef` to index 1. The focused DOM element that received the keyboard event is still keyed/indexed as thumb 0 and is now value 80. A subsequent Arrow key applies `handleKeyDown`'s closed-over index 0 (`:360-379`), therefore changes the *other* thumb, not the thumb the user is still focused on. Pointer identity has the same positional model.

The existing crossing test only checks the sorted proposal `[80, 100]` (`🎚️Slider/🧪️component.test.tsx:49-61`); it never proves focus/drag identity after the crossing. This violates the requested multi-thumb crossing and identity contract and makes continued keyboard/pointer interaction act on an unexpected control. Repair requires stable thumb identities (not positional React keys), preserving the active identity through reorder, and an interaction test that crosses then continues from the same focused/dragged thumb.

## Popover Assessment

The owned contract replaces the facade and covers controlled/uncontrolled proposals, exact-one-child slotting, composed refs, nonmodal portal ownership, side/alignment/offset measurement, resize/scroll remeasurement, logical nested boundaries, preventable open/close focus, pointer/focus outside handling, and one-level-at-a-time nested Escape. The focused matrix exercises all current ActionDropdown, Toggle, Search prevented-autofocus, and ShellSync anchor-relevant paths. The ActionGroup trigger is a native button and the inspected current DOM path has no nested interactive descendant.

One remaining risk is not a separate release blocker: `isTopmostPopover` chooses the last deepest portal in document order (`🗨️Popover/🟦️component.tsx:254-258`) rather than the focused/last-activated member. Two same-depth controlled sibling popovers could consequently send Escape to the later portal even when the earlier one owns focus. The matrix proves nesting but not this sibling ordering case. Add a same-depth controlled-sibling Escape test and an activation/focus-based stack if that composition is supported.

## Slider Assessment Beyond the P0

The implementation does correctly normalize invalid ranges and step values, clamps `min`/`max`, observes disabled/read-only, handles horizontal RTL and vertical keyboard mapping, applies pointer capture/release, rolls `pointercancel` back without committing, carries a controlled draft until matching external acknowledgement, separates ready display from hard ready clamping, and exposes native slider ARIA values. The 641-test UI gate includes the inspected pointer, cancellation, controlled-lag, RTL/vertical, disabled/read-only, ready, and nominal single-commit tests. Those results cannot demonstrate the missing post-cross identity behaviour above.

There is also a lower-priority exactness gap: `publishValues` deliberately treats an attempted but fully clamped/no-op input as a changed gesture (`🎚️Slider/🟦️component.tsx:221-236`), so an Arrow/End attempt at a ready/bound/min-gap ceiling can emit `onValueChange` and subsequently `onValueCommit` with an unchanged tuple. The requested change-versus-commit contract should explicitly decide and test whether no-op proposals may produce a commit; suppression is the safer interaction semantics.

## Dependency and Gate Evidence

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 16 files, 641 tests. |
| `bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache` | PASS — 4 files, 438 tests. |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile` | PASS — lockfile-only, no lifecycle scripts. |
| `bun ./📜️script.ts verify dependencies list js` | PASS — neither retired identity is listed. |
| `bun ./📜️script.ts verify dependencies` | PASS — command exited zero. |
| `bun ./📜️script.ts verify dependencies parity js --format json` | PASS — 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 lock workspaces. |
| Exact executable source/manifest/lock scans | PASS — no live manifest or `bun.lock` occurrence of either retired package. The only matches are the deliberate historical baseline rows in `🔒️dependencies.json`. |

The bounded gates emitted the known `NO_COLOR`/`FORCE_COLOR` warning only. No Cargo command, cache deletion, Git mutation, package installation, browser/Playwright run, Storybook run, production build, or production-source change was performed.

## Required Repair and Re-audit Gate

1. Represent thumbs with stable identities and preserve the active identity while tuple values reorder; verify keyboard focus and pointer capture continue controlling that same identity after an actual crossing.
2. Ensure a clamped/no-op gesture does not spuriously commit, or codify and test a different exact contract.
3. Add the same-depth sibling Popover Escape ordering proof or make the stack focus/activation aware.
4. Publish the missing P10ak implementation report, then rerun the two focused test commands, lock freeze/parity, and exact identity scans before a fresh independent audit.

## Unrun Browser Risks

No production browser/Playwright verification covered real pointer-capture loss, portal stacking, focus transitions, native layout/ResizeObserver behaviour, or SSR/hydration. jsdom and the two focused suites provide bounded runtime evidence only.
