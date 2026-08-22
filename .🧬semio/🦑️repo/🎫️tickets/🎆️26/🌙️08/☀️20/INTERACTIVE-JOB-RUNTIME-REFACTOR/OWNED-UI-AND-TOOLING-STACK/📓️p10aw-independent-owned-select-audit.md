# P10aw Independent Owned Select Audit

## Verdict

**PASS — scoped source, DOM, consumer, lock, and dependency gates.** The current Select leaf is repository-owned and has no Radix-derived public type contract. The direct dependency ratchet is **80 JavaScript identities**, with no `@radix-ui/react-select` entry. This acceptance does not certify a real-browser or hydration run.

## Independent Findings

- The actually exported facade is the scout's ten-symbol surface: `Select`, `SelectContent`, `SelectGroup`, `SelectItem`, `SelectLabel`, both scroll controls, `SelectSeparator`, `SelectTrigger`, and `SelectValue`. The P10av prose additionally names Viewport/ItemText/ItemIndicator, but those are internal slots rather than exported API; this is a report-inventory inconsistency, not a source contract leak.
- `🧱️elements/☑️Select/🟦️component.tsx` owns all contracts, controlled/uncontrolled value and open proposals, fallback projection, stable `useId`-based IDs, listbox/option/group semantics, disabled skipping, active-versus-selected state, NFKD/mark-stripped locale-independent typeahead, pointer/touch selection, focus policy, logical nested/sibling portal ordering, placement/clamping/RTL, and document guards.
- The 10-case real-DOM matrix exercises controlled lag, keyboard/typeahead, disabled rows, groups/labels, preventable dismissal, sibling/nested ordering, touch exact-once behavior, custom portals, RTL placement, and scrolling. Admin exercises a live locale Select; renderer exercises declarative Select dispatch.
- Live source/manifests and `bun.lock` have zero `@radix-ui/react-select`, `SelectPrimitive`, or `--radix-select-` matches. Historical ticket records and ignored/generated `dist`/cache outputs still contain old strings; they are not live source, manifests, or the current lock and were not treated as evidence of the shipped runtime.

## Independently Executed Gates

| Gate | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick -- --run ../../../../🧱️elements/☑️Select/🧪️component.test.tsx` | PASS — 10 tests |
| `bun nx run @semio-tech/ui-react:test-quick` | PASS — 19 files, 672 tests |
| `bun nx run @semio-tech/framework-renderer-react:test-quick` | PASS — 4 files, 439 tests |
| `bun nx run os-hub-admin:test` | PASS — 2 files, 8 tests |
| `bun nx run @semio-tech/ui-react:typecheck` | PASS |
| `bun nx run @semio-tech/ui-react:lint` | PASS (Bun emitted only the existing `NO_COLOR`/`FORCE_COLOR` warning) |
| `bun nx run @semio-tech/ui-react:check-ui-primitives` | PASS — 0 violations |
| `bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary` | PASS |
| JS dependency census | PASS — 80 identities; Select identity absent |
| JS dependency parity | PASS — no undeclared imports or lock mismatches; 5 fixtures, 44 workspaces |
| Targeted `git diff --check` | PASS |

No Cargo command, production build, modifying Git command, or implementation-source edit was made by this audit.

## Explicit Residuals

JSDOM cannot establish native portal pointer/focus ordering, screen-reader announcements, physical collision/scroll behavior during resize, or real-browser hydration. The server guard is source-reviewed (`document` is only accessed after the SSR null return), and existing static-render tests cover the closed Select path, but no hydrate run was performed. A later browser/SSR gate must cover those environmental behaviors before claiming them closed.
