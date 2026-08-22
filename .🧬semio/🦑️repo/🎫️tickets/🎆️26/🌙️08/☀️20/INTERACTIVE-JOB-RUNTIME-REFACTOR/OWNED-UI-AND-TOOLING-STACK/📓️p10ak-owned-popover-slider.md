# P10ak Owned Popover and Slider

## Verdict: PASS

Phase 10 replaced the live `@radix-ui/react-popover` and `@radix-ui/react-slider` facades with repository-owned React primitives. The independent P10al audit found three interaction defects; all three are repaired in the final implementation described here and in P10am. The retired package identities are absent from live source, manifests, and `bun.lock`. The JavaScript dependency ratchet remains at the required 146 current dependencies from the historical 238 baseline.

## Owned Surface

### Popover

The owned Popover provides controlled, uncontrolled, and default-open state; trigger and anchor `asChild` composition; composed refs and preventable caller events; repository-owned portal creation; nonmodal dialog semantics; state-owned `aria-expanded`, `aria-controls`, and `data-state`; disabled-trigger handling; measured side/alignment/offset positioning; viewport flip and clamp; resize/scroll/anchor remeasurement; outside-pointer and outside-focus dismissal; Escape dismissal; preventable open/close autofocus; trigger focus return; logical nested-popover boundaries; and deepest-then-most-recently-active Escape ordering. Closed panels are not mounted and therefore do not run panel effects.

The trigger path remains a single native interactive element. `ActionGroup` was adjusted so the Popover trigger does not wrap a nested button. Search's prevented autofocus contract, Toggle's owned trigger behavior, and ShellSync's anchor/portal path remain covered by the focused UI and renderer suites.

### Slider

The owned Slider exports only repository-defined value, orientation, and callback contracts. It provides controlled draft behavior during parent lag; uncontrolled/default values; valid normalization for invalid ranges and steps; min/max/step snapping; ready extent display and hard ready clamping; one or multiple thumbs; crossing with sorted values and stable logical thumb identities; pointer capture, move, release, and cancel rollback; horizontal, vertical, and RTL keyboard behavior; disabled/read-only behavior; native slider ARIA values and labels; and one commit per changed gesture.

Logical thumb IDs, not tuple positions, are React keys and event identities. When values cross and reorder, focus and pointer interaction remain attached to the same logical thumb. Fully clamped keyboard and pointer attempts are no-ops: they neither publish an unchanged `onValueChange` tuple nor arm `onValueCommit`.

## Consumer and Packaging Changes

- The public React target barrel now exports the owned Popover and Slider contracts without Radix-derived public types.
- The UI package manifest and root manifest no longer declare either retired package; the root Slider override was removed because no other live user remained.
- `bun.lock` removed 51 lines in the cumulative Phase 10 lock delta and contains neither retired package identity.
- The UI source manifest and styling discovery include the owned leaves.
- The existing ActionGroup consumer uses the owned trigger without nested interactive controls.
- A real renderer `SyncAttachCard`/ShellSync test exercises the portal/anchor consumer path; focused UI tests also exercise Search, Toggle, and ActionGroup-relevant behavior.

## Focused Runtime Evidence

- Popover matrix: 11 real-DOM tests covering controlled/uncontrolled/default-open state, trigger/anchor composition, portal ownership, placement and collision behavior, outside dismissal, nested boundaries, Escape, focus return, preventable autofocus, disabled triggers, current consumer combinations, and same-depth sibling focus/activation ordering.
- Slider matrix: 11 real-DOM tests covering helpers, controlled lag, normalization, stable crossing identity with continued keyboard and pointer movement, minimum gaps, pointer capture, cancellation, orientation/RTL keyboard behavior, disabled/read-only state, ARIA, ready extent, commits, and exact min/max no-op attempts.
- UI quick suite: PASS, 16 files and 644 tests.
- Renderer quick suite: PASS, 4 files and 438 tests.

An intermediate repaired UI run correctly failed one stale source-inlined consumer assertion that expected `clampToReady` to emit an unchanged value at the ready ceiling. That assertion encoded the P10al no-op defect. It was changed to require no callback, and the final 644-test run passed.

## Final Bounded Gates

| Gate | Result |
| --- | --- |
| Focused `bun nx format:write` for Popover/Slider leaves and tests | PASS. |
| Focused `bun nx format:check` including the React target barrel | PASS. |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS. |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 16 files, 644 tests. |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS — only the existing Bun `NO_COLOR`/`FORCE_COLOR` warning. |
| UI primitive policy gate | PASS — 0 violations and 2 existing allowlisted entries. |
| `bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache` | PASS — 4 files, 438 tests. |
| `df -h .` before lock reconciliation | PASS — 2.3 GiB available. |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile` | PASS — lockfile-only; no lifecycle scripts or package installation. |
| `bun ./📜️script.ts verify dependencies` | PASS — historical 238, current 146, removed 92, no new dependency. |
| `bun ./📜️script.ts verify dependencies list js --format json` | PASS — 83 manifest rows; neither retired identity listed. |
| `bun ./📜️script.ts verify dependencies parity js --format json` | PASS — 83 manifests, 268 external rows, 119 evidenced rows, 149 unowned rows, 0 undeclared imports, 0 lock mismatches, 5 lock fixtures, 44 lock workspaces. |
| Manifest/source audit | PASS — 64 manifests, 580 direct rows, 268 external rows, 75 rows without owned-scope evidence. |
| Exact Popover/Slider executable-source, manifest, and lock scans | PASS — 0 matches. |
| Targeted `git diff --check` | PASS. |

## Changed Paths

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎚️Slider/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/⚡️ActionGroup/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts`
- `package.json`
- `bun.lock`
- this report and `📓️p10am-popover-slider-audit-repair.md`

## Explicitly Unrun

No Cargo/Rust gate was run. No browser/Playwright, Storybook, production build, complete monorepo test, or SSR/hydration gate was run. The earlier broad renderer typecheck remains outside this packet because it reports the repository's existing cross-package TypeScript failures; the bounded renderer runtime suite is green. Remaining browser-only risk is real pointer-capture loss, native portal focus transitions, layout/ResizeObserver timing, and hydration behavior.

No Git-modifying command, cache deletion, package install, compose/Dialog/Select edit, or ticket metadata edit was performed.
