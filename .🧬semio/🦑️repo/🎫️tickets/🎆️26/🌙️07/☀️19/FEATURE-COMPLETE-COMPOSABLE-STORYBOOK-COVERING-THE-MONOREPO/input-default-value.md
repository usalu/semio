# Input Default Value Fix

## Changes

Only Input implementation and its directly associated tests/fixture were changed; Label, stories, configuration, package manifests and other components were not edited by this assignment. Existing unrelated work was preserved.

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🟦️.tsx`: consumes `defaultValue` instead of forwarding it alongside a controlled DOM value. Tracks an uncontrolled committed value initialized from the default, while defined external `value` remains authoritative. Immediate edits persist and still notify `onChange`. Lazy edits commit once on Enter/blur, persist without a parent callback, and Escape restores the latest committed/external value without committing.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧪️tests/🧩️component/🟦️.tsx`: expanded the existing React component oracle.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🧪️tests/🧫️fixtures/⌨️default-value/🔣️.json`: language-neutral cases, including Label's numeric defaults 240 and 4, zero, text, omitted defaults and lazy commit/cancellation outcomes.
- `test-input.ts` beside this report: reproducible focused runner using the existing owner config and explicit Vitest policy; its owned generated cache is removed afterward.

The component keeps its internal DOM input controlled so collapse/remount and lazy editing retain their existing presentation behavior. Uncontrolled consumer semantics are implemented in component state. Form reset restores the current default without emitting a change/commit, including ancestor or explicitly associated forms and collapsed fields. Reset handling waits until cancellation is known and respects `preventDefault`.

## Validation

Test-first: added fixture and tests before editing Input. Initial run: **9 failed, 5 passed**, reproducing blank numeric/text defaults, rejected immediate edits, lost lazy commits and incorrect reset. After the implementation and additional regressions: **18 passed** using the existing React/Vitest/jsdom setup. Five fixture cases are executed against both our Input and React's native `<input>` as the independent third-party oracle; their initial/edit/rerender/reset snapshots match. React console-error spies assert no warnings, including when controlled `value` and `defaultValue` are both supplied to Input.

Additional regressions exercise controlled rerenders and rejected edits, lazy external updates, Escape, Enter/blur deduplication, IME composition, canceled reset, focused numeric reset, associated forms and collapsed labeled remounts. Existing shared Input/Textarea lifecycle tests remain green.

```sh
PATH=$HOME/.local/share/semio/launcher/bin:$PATH repo tools exec --light bun -- '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FEATURE-COMPLETE-COMPOSABLE-STORYBOOK-COVERING-THE-MONOREPO/test-input.ts'
```

Focused `git diff --check` passed; final implementation/test diff and new fixture were inspected. No new runtime dependencies.

## Typecheck Limitation

Ran:

```sh
PATH=$HOME/.local/share/semio/launcher/bin:$PATH repo tools exec --light bun -- node_modules/typescript/bin/tsc --project '🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json' --incremental false --pretty false
```

Exit **2**, **17 diagnostics**, none in Input or its tests. Existing generated declaration gaps include these exact TS2307 locations:

- `🧰️framework/🔨️modules/🎓️onboarding/🏷️declaration/🟦️.ts(3,1674)`: `../../🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts`.
- `🧰️framework/🔨️modules/🛂️manifest/🎬️action-types-and-semantics/🟦️.ts(3,1025)`: `../🤖️generated/🪪️manifest/🟦️.ts`.
- `🧰️framework/🔨️modules/🛂️manifest/🏷️ui-contract-and-canvas-types/🟦️.ts(4,163)`: `../🤖️generated/🪪️manifest/🟦️.ts`.
- `🧰️framework/🔨️modules/🛂️manifest/🏷️ui-contract-and-canvas-types/🟦️.ts(5,3443)`: `../🤖️generated/📜️ui-contract/🟦️.ts`.
- `🧰️framework/🔨️modules/🛂️manifest/🔘️definitions-action-ids-and-shell-brand/🟦️.ts(3,1548)`: `../🤖️generated/🪪️manifest/🟦️.ts`.

Other diagnostics outside this ownership boundary: implicit-any TS7006 in React introduction (120:35, 120:38, 272:111, 332:36, 332:49), tutorial engine (98:79), UIDialog story (37:228), UIDialog component (136:27), UIDialog test (233:60), action-argument-resolution (43:68); missing-return TS2366 in introduction-demo (30:78) and Tree virtual-window (20:69). Full typecheck is therefore not claimed green.

## Handoff

Implementation is ready for integration. Remaining owner work: rerun the unchanged Label Storybook preview as part of catalog validation; this assignment validated equivalent defaults and actual edits in the root Input tests, not a live Storybook browser session. Full native MCP remains unavailable as supplied; no ticket state was fabricated or changed.
