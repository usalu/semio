# Select and Tabs Boundary Adoption

## Result

Only Select and Tabs source/test subtrees changed; shared core CSS and all other UI files untouched. No staging, commits, dependency changes or ticket lifecycle mutation.

- Select trigger and independently portaled listbox register all physical frame edges. Trigger is now positioned relative; content retains its existing measured fixed/absolute positioning.
- Omitted Select offsets resolve to zero inside `[data-window-silhouette]` and four CSS pixels standalone. Explicit values, including zero, remain authoritative. Removed the extra side-dependent translate-y utility, so the measured collision/flipping placement is the actual final position.
- Shell portal precedence, dialog isolation, listbox/combobox semantics, active-descendant navigation, selection and focus restoration remain intact. The portal test includes an overflow-hidden ancestor.
- TabsList is a discovery group with its existing actual orientation. Only the selected TabsTrigger registers its painted full frame and explicit active state; its host is positioned. Existing selected fill, border, shadow, chip sizing and keyboard semantics remain unchanged.
- Tabs intentionally do not become full bordered macro frames or shared cells: the list itself paints no border, inactive chips have transparent borders, and adding cells would invent visible separators absent from the existing design. Frame ownership follows selection rather than introducing an outline on inactive chips.
- Inspected Tree/WindowPaneChromeToggle and left it unchanged. Its border-0 silhouette-owned chip does not warrant a second frame/cell; introducing one could add a redundant handle divider.
- No component emits speculative `data-shared-border-attached`; compositor remains sole geometry authority.

## Changed Files

Relative to `🧰️framework/🔨️modules/🖱️ui/🧱️elements/`:

- `🔽️Select/🟦️.tsx`
- `🔽️Select/🧪️tests/🧩️component/boundary.feature`
- `🔽️Select/🧪️tests/🧩️component/boundary.tsx`
- `📑️Tabs/🟦️.tsx`
- `📑️Tabs/🧪️tests/🧩️component/boundary.feature`
- `📑️Tabs/🧪️tests/🧩️component/boundary.tsx`

Ticket inputs: `select.config.ts` (isolated executable runner), this report.

## Validation

Neutral Gherkin contracts and executable tests were written before source edits. Initial run confirmed the missing frame/group contracts; its temporary presentation-mock setup also exposed an unrelated touch-event polyfill failure. After canonical assets restoration, the final runner uses the repository's real React environment with real UI imports and no presentation mocks.

Command:

```text
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light vitest -- run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/SHARED-UI-BORDERS/select.config.ts
```

**4 suites / 27 tests passed**: existing Select 15, existing Tabs 7, new Select 3, new Tabs 2. Checks include flush/standalone/explicit gaps, flipped no-overlap placement, shell portals, focus, keyboard selection, orientations and selected-frame reassignment. Placement output is cross-checked against independent third-party lodash clamp. Scoped `git diff --check` passed; final diff inspected. Runner cache removed without touching others' generated artifacts.

## Remaining Integration Work

Lead should include these boundary test entries in the integrated runner (current lead config does not discover Select/Tabs). Real-browser clipping/paint/compositor joining remains an integration check, not proven by jsdom. No runtime/compositor implementation changed here. Ticket registration/closure remains with lead; repo MCP tools are not exposed in this execution context.
