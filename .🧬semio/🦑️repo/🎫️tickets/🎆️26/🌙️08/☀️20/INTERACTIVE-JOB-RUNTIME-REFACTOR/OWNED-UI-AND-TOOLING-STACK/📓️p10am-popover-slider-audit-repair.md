# P10am Popover and Slider Audit Repair

## Verdict: AUDIT-READY

Every P10al finding is repaired and backed by a focused real-DOM interaction proof. P10ak has also been published with the implementation provenance and bounded gate results that were missing at the start of the independent audit.

## Finding Repair Matrix

| P10al finding | Repair | Runtime proof |
| --- | --- | --- |
| P0 Slider crossing redirected later input because tuple index was both identity and React key. | Each logical thumb now owns a stable generated ID. Values and IDs reorder together; thumbs are keyed by ID; keyboard and pointer updates resolve the live tuple position from that ID; cancellation restores both values and identity order. | Starting from `[20, 80]`, the first logical thumb crosses to 100, retains the same DOM node and focus, moves by keyboard to 90 as `[80, 90]`, then moves by pointer across the other thumb to 70 as `[70, 80]`. The same logical node remains focused and reports 70. Exactly three changed gestures commit. |
| Fully clamped pointer/keyboard attempts armed a changed gesture and emitted unchanged change/commit callbacks. | Value publication compares the normalized tuple with the current tuple before changing draft state or `gestureChanged`. No actual tuple change means no `onValueChange` and no later `onValueCommit`. | Exact min and max tests cover Arrow, Home, and out-of-range pointer attempts. All leave both callbacks at zero. The source-inlined ready-ceiling assertion now also requires suppression of the no-op callback. |
| Same-depth sibling Popover Escape selection depended on DOM-last portal order. | Open Popovers have an activity sequence updated by trigger focus/pointer/click and content focus/pointer events. Escape first selects the deepest logical nesting depth, then the most recently active member at that depth. Activity state is removed when the Popover closes/unmounts. | With two controlled open siblings, focusing the first sibling's action and pressing Escape proposes closing only the first. Activating the second trigger and pressing Escape then proposes closing only the second. The existing nested topmost test remains green. |
| P10ak implementation report was absent. | `📓️p10ak-owned-popover-slider.md` is published with surface, consumer, packaging, lock, test, gate, changed-path, and unrun-gate provenance. | Presence and content were checked before this handoff. |

## Final Gate Evidence

| Gate | Result |
| --- | --- |
| Popover/Slider focused formatting write and final formatting check | PASS. |
| UI typecheck | PASS. |
| UI quick suite | PASS — 16 files, 644 tests. |
| UI lint | PASS — only the known Bun color-environment warning. |
| UI primitive policy | PASS — 0 violations, 2 existing allowlisted entries. |
| Renderer quick suite | PASS — 4 files, 438 tests. |
| Frozen lockfile-only reconciliation with scripts ignored | PASS. |
| Dependency freeze | PASS — 238 historical, 146 current, 92 removed, no new dependency. |
| JavaScript list | PASS — 83 rows. |
| JavaScript parity | PASS — 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 workspaces. |
| Manifest/source audit | PASS — 64 manifests, 580 direct rows, 268 external rows, 75 without owned-scope evidence. |
| Exact live source/manifest scan for both retired identities and primitive aliases | PASS — 0 matches. |
| Exact `bun.lock` scan for both retired identities | PASS — 0 matches. |
| Targeted diff whitespace check | PASS. |

The cumulative shared Phase 10 `bun.lock` delta is 51 deletions and the root manifest delta is 2 deletions. The repair introduced no dependency or lock change beyond the already reconciled packet state.

## Residual Risk and Explicitly Unrun Gates

No Cargo/Rust command was run. No browser/Playwright, Storybook, production build, complete monorepo test, or SSR/hydration gate was run. Browser-only residual risk remains around native pointer-capture loss, portal/focus timing, layout/ResizeObserver delivery, and hydration. The focused jsdom matrices and actual UI/renderer consumer suites are the bounded runtime evidence for this repair.

No Git-modifying command, package installation, cache deletion, compose/Dialog/Select edit, or ticket metadata edit was performed.
