# P10an Independent Popover and Slider Final Audit

## Verdict: PASS

Independent read-only implementation audit on 2026-08-22. I read `AGENTS.md`, the full P10ad scout, P10ak implementation report, P10al rejection, and P10am repair report; inspected the current owned Popover/Slider sources, focused tests, consumer/barrel/manifests/lock state; and ran the bounded gates below. P10ak now exists and its stated packet paths, dependency removal, focused test cardinalities, and explicit unrun scope match the observed working-tree state and this audit.

## P10al Repair Challenges

### Slider

`thumbIdsRef` is now a stable logical identity sequence. `publishValues` sorts `{ id, value }` records together, rendering keys/event identities from the logical ID rather than value-tuple position. `updateThumb` resolves the live index from that ID, and cancellation restores both the initial values and ID ordering.

The focused real-DOM test starts `[20, 80]`, holds the first logical thumb, crosses it to `100`, verifies that the same DOM node keeps focus and ID, keyboard-moves that node to `90` as `[80, 90]`, pointer-moves it through the other thumb to `70` as `[70, 80]`, then verifies the same focused node reports `70`. It proves exactly three changed gesture commits. This addresses both keyboard and pointer post-cross continuity, rather than only asserting a sorted tuple.

Controlled lag remains explicit: the pending draft is retained while the parent rerenders the previous controlled tuple and is cleared only after acknowledgement. Pointer cancellation restores the original tuple and emits no commit. `publishValues` now compares normalized output to the live tuple before updating draft state or arming a gesture; Arrow/Home at min and Arrow/out-of-range pointer at max emit neither change nor commit. The ready ceiling uses that same no-op path after hard clamping.

### Popover

The prior document-order sibling Escape ambiguity is repaired. Open content records logical nesting depth, while trigger focus/pointer/click and content focus/pointer update an activity sequence. Escape chooses deepest content first, then the most recently active sibling at that depth. The focused controlled-sibling test proves focus on the first action dismisses only first, then activation of the second trigger dismisses only second; existing nested Escape, outside-boundary, focus lifecycle, slot/ref, collision, and portal cases remain in the matrix.

## Bounded Gate Evidence

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS. |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 16 files, 644 tests. |
| `bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache` | PASS — 4 files, 438 tests. |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS — only the existing Bun `NO_COLOR`/`FORCE_COLOR` warning. |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache` | PASS — 0 violations, 2 allowlisted files. |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile` | PASS — lockfile-only, scripts ignored, no package installation. |
| `bun ./📜️script.ts verify dependencies` | PASS — historical 238, current 146, 92 removed, no new dependency. |
| `bun ./📜️script.ts verify dependencies list js --format json` | PASS — 83 JavaScript identities; neither retired identity is present. |
| `bun ./📜️script.ts verify dependencies parity js --format json` | PASS — 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 lock workspaces. |
| Exact scan for `@radix-ui/react-popover`, `@radix-ui/react-slider`, `PopoverPrimitive`, and `SliderPrimitive`, excluding historical baseline/evidence | PASS — 0 live matches. |
| `git diff --check` across all packet paths | PASS. |
| Exact-file `bun x prettier --check` across owned leaves, focused tests, consumer, target barrel, and manifests | PASS. |

The repository-wide `bun nx format:check` could not be used: it exits before formatting because its configured `⛳️wip` Git revision does not exist in this checkout. This is a repository task-runner precondition failure, not a passing gate. The exact-file non-mutating Prettier check above is the bounded replacement used for this audit.

## Observed Diff/Provenance Match

The working-tree packet contains the two new focused test files plus owned Popover/Slider implementations, ActionGroup integration, public target barrel, UI/root manifests, and `bun.lock`. The observed manifest/lock deltas remove the two retired package identities; `bun.lock` has 51 removed lines. That agrees with P10ak. No production file was changed by this audit.

## Explicitly Unrun / Browser Residuals

No Cargo/Rust command, browser/Playwright run or browser installation, Storybook, production build, full-monorepo test, or SSR/hydration test was run. jsdom cannot prove native pointer-capture loss, real portal/focus timing, ResizeObserver/layout behavior, or hydration. Those remain browser-only residuals; they do not invalidate the bounded packet gates above.
