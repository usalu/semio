# Phase 10 Owned Class Composition

<!-- #region Outcome -->

## Outcome

The repository-owned `cn` gateway now performs recursive class token flattening and last-winner conflict resolution without importing `clsx` or `tailwind-merge`. Its deliberately finite conflict catalogue is derived from utility families used by the UI source and explicitly includes the repository fill family `ui-surface` / `ui-glass` / `ui-veil`. This implementation does not claim general Tailwind compatibility.

The `@semio-tech/ui-react` manifest no longer declares either implementation identity. Removal of the stale public `clsx` re-export was explicitly handed to and completed by the concurrently active UI-foundation owner so that this packet did not overlap its barrel write; dependency installation and the broad gates remain held behind the active Puzzle/Animate lanes.

<!-- #endregion Outcome -->

<!-- #region Contract -->

## Contract

- Strings are split into class tokens; finite non-zero numbers retain the previous stringification behavior.
- Arrays recurse; falsey array values are suppressed; truthy object keys become class tokens.
- Unknown application-specific classes remain ordered and are not deduplicated.
- Known utility conflicts are scoped by modifier and importance; the last conflicting utility wins.
- Directional conflicts are asymmetric where ordering matters, including padding, margin, inset, overflow, gap, size, rounded corners, borders, and scale.
- Repository fills share the owned `bg-color` group with `bg-*` utilities.
- The live `aspect-square` / `aspect-auto` pair shares one finite `aspect-ratio` group, so inline-label controls keep only the last requested geometry.

<!-- #endregion Contract -->

<!-- #region Evidence -->

## Evidence

Focused Vitest:

```text
Test Files  1 passed (1)
Tests       20 passed (20)
Duration    518ms
```

The later broad quick-suite run exposed one missing live family: `aspect-square` survived beside a later `aspect-auto` on Engagement option buttons. The owned catalogue now groups exactly the two aspect utilities used by UI source, and focused fixtures prove last-winner behavior in both directions. The assertion was not weakened.

Focused UI regression:

```text
Test Files  1 passed (1)
Tests       1 passed | 537 skipped (538)
Duration    2.10s
```

Before removing the two manifest rows, an in-memory differential corpus compared the owned gateway with `clsx` plus the existing extended `tailwind-merge` adapter. It covered recursive inputs, falsey suppression, repository fills, standard utility winners, modifiers, arbitrary data modifiers, importance, directional conflicts, background subgroups, and unknown classes:

```text
[DEBUG] owned class composition parity: 22/22
```

The source-derived owned fixtures additionally lock the repository theme-token behavior (`px-single` / `px-tiny`, `h-medium` / `h-large`, `border-normal` / `border-accent`) that the generic adapter did not merge reliably.

A TypeScript AST scan collected string and static-template tokens passed directly to all `cn(...)` calls under the UI module, then checked every utility-shaped candidate against the owned catalogue:

```text
[DEBUG] cn static tokens: 428; utility candidates: 370; unclassified: 0
```

<!-- #endregion Evidence -->

<!-- #region DeferredGates -->

## Deferred Gates

These gates were intentionally not run while the exclusive hard-gate lanes were active:

- `bun install --ignore-scripts`
- UI quick suite
- UI typecheck
- UI lint
- UI primitive check
- demonstrator production build
- dependency freeze and JavaScript parity

The coordinated UI barrel change is complete and the packet is ready when the parent releases the shared gate lane.

<!-- #endregion DeferredGates -->
