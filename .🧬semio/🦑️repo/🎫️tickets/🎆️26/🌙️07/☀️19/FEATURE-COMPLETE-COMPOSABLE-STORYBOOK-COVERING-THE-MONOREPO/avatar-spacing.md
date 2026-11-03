# Avatar Spacing

## Implemented

Paths relative to `🧰️framework/🔨️modules/🖱️ui/🧱️elements/`:

- `📻️TableAvatar/🟦️.tsx`: owns shared `tableAvatarSizeClassName = size-workbench` and `tableAvatarTextClassName = text-xs`. Default border box is 24px at compact spacing; table interior is 22px, PresenceBar's 2px-border interior is 20px. Existing 16px text line box has 3px/2px vertical clearance respectively. SVG icons get a centered 16px wrapper with direct-child and nested SVG normalization. Images use object-cover with a 1px inset inside the border interior. Explicit caller sizing, ref, image lifecycle, selection and hover semantics retained. No cn workaround or global scale changes.
- `👥️PresenceBar/🟦️.tsx`: replaces negative overlap with semantic `gap-single` (3.2px compact); consumes the avatar default and shares dimension/typography with overflow chip. Agent/activity markers, keyboard focusability, translated titles and accessible names preserved.
- `📻️TableAvatar/🧪️tests/🧫️fixtures/📐️spacing/🔣️.json`: language-neutral sizing/clearance contract written before implementation.
- `📻️TableAvatar/🧪️tests/🧩️component/🟦️.tsx`: extends actual existing tests for contract classes, neutral geometry against canonical styling metrics plus independent `@csstools/css-calc`, state typography, image inset, raw/nested SVG normalization; retains lifecycle/ref/style tests and adds explicit cleanup.
- `👥️PresenceBar/🧪️tests/🧩️component/🟦️.tsx`: roster gap, peer/overflow sizing, border, agent/activity and focus regressions. Label translation and appearance boundaries mocked; avatar and class merger are real.

Ticket input added: `vitest.avatar-spacing.config.ts` (focused jsdom runner).

## Validation

All commands used installed repo route: prepend `export PATH="$HOME/.local/share/semio/launcher/bin:$PATH"`, then:

```sh
repo tools exec --light bun -- node_modules/vitest/vitest.mjs run --config '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FEATURE-COMPLETE-COMPOSABLE-STORYBOOK-COVERING-THE-MONOREPO/vitest.avatar-spacing.config.ts'
```

Test-first red: 3 failed / 5 passed, specifically default dimension, image containment, roster spacing. Green after implementation: 2 files / 8 tests passed. Inspected scoped final git diff without modifying git. No server/generator/install operations. No generated ticket output created or removed. Initial bare `repo` lookup failed before owner provided installed absolute route.

## Lead Browser Acceptance

- `[data-slot="avatar"]`: default width/height 24px; PresenceBar border 2px and TableAvatar border 1px.
- `[data-slot="avatar-fallback"]`: selected/default/hover font size 11.2px and line-height 16px; interiors 20px/22px; text vertical centers agree. This relies on lead-owned shared cn fix (present during passing test run).
- `[data-slot="avatar-icon"]`: compact 16px box and descendants contained; image `[data-slot="avatar-image"]` content inset 1px and object-cover.
- PresenceBar `[role="list"]`: computed gap 3.2px, no negative margins; adjacent `[data-row-id^="peer:"]` bounds separated, peer and overflow centers aligned.
- `[data-row-id="peer:overflow"]`: 24px square and text-xs. Agent/activity badge selectors remain unchanged; focus each peer and overflow.

Real browser dimensions, loaded image rendering, icon-catalog rendering, extreme-width initials/large overflow counts and final visuals remain lead integration checks; jsdom does not perform layout. Caller-specified smaller dimensions intentionally remain caller-controlled. No work performed in other owners' scopes; ticket lifecycle remains lead-owned because MCP is unavailable.
