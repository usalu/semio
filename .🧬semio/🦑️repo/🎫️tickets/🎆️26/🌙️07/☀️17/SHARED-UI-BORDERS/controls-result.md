# Controls and Bars Result

## Implemented

Paths relative to `🧰️framework/🔨️modules/🖱️ui/🧱️elements/`:

- `🔳️ButtonGroup/🟦️.tsx`: positioned shared frame, physical painted edges, horizontal orientation, explicit direct cells. Outline variant no longer paints a redundant full rectangle inside the group's outer outline.
- `🎛️ToggleGroup/🟦️.tsx`: positioned shared frame and uniform direct shell cells; each secondary action remains a separate sibling of the native toggle, never nested inside its button. Divider moves to shell so action and no-action cases share one model. Labeled no-action shells preserve intrinsic non-growing width. Vertical groups explicitly stack; keyboard/radio/roving-focus code unchanged.
- `🔀️Toggle/🟦️.tsx`: standalone toggle registers outer positioned frame and native cell. Compound variants inherit ToggleGroup registration.
- `⚡️ActionGroup/🟦️.tsx`: positioned group frame/cells; standalone actions register full positioned frames and loading/waiting state.
- `🔝️Navbar/🟦️.tsx`: only bottom edge registered; flow and centered layout regions marked as attachment groups. Centering measurement, gaps and landmark semantics unchanged.
- `🔚️Footer/🟦️.tsx`: only top edge registered.
- `🎀️Ribbon/🟦️.tsx`: layout-only attachment groups, not invented painted frames across gaps.
- `🧭️PanelTabBar/🟦️.tsx`: tab rows marked as attachment groups and existing tab buttons as cells; preserves original chip content, drag handles and aria-pressed.

Inspected `🔘️Button/🟦️.tsx` and `🧱️DragHandle/🟦️.tsx` without source edits. Button already delegates through ButtonGroup; DragHandle paints no border and must not become its own divided cell.

No speculative attached-side attributes are emitted. No CSS, silhouette, or menu source edited.

## Tests Added

- `🎛️ToggleGroup/🧪️tests/🔣️.json`: six language-neutral ownership/physical-edge/direct-cell expectation cases.
- `🎛️ToggleGroup/🧪️tests/🧩️component/🟦️.tsx`: same cases validated against both React DOM server markup and testing-library client rendering through DOM queries; checks positioned frame hosts, no premature attachments, single outline, standalone disabled button behavior, panel tab ownership/pressed semantics, action sibling semantics, vertical shell topology and borderless drag handles/ribbon layout groups.

Existing accessibility/interaction tests retained unchanged.

## Validation

- TypeScript `transpileModule` syntax diagnostics: **9 modified TSX files passed** (eight components plus test suite). This is not a complete type check.
- Scoped `repo tools exec --light git -- diff --check`: passed.
- Final component source diff inspected.
- Standard UI Vitest config cannot bootstrap without explicit repo Vitest policy. Lead supplied ticket-local config.
- Focused ToggleGroup + Toggle test command with ticket config reached suite collection but **both suites blocked before tests** by missing canonical `🎨️styling/🤖️generated/🔤️tokens/🟦️.ts`. Setup owns restoring that prerequisite. No tokens stubbed; no successful runtime tests claimed.

Rerun after setup:

```sh
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light vitest -- run --config '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/SHARED-UI-BORDERS/vitest.config.ts' '🎛️ToggleGroup/🧪️tests' '🔀️Toggle/🧪️tests'
```

## Integration Risks / Remaining Work

- Browser layout, actual frame coincidence, navbar/name-chip joins, hover/focus and vertical sizing require integrated runtime verification. Registration is declarative, geometric joining remains silhouette ownership.
- Core cell selectors currently use `:last-child`; if compositor appends its SVG inside a frame after cells, last-cell divider suppression will fail. Ensure compositor SVG belongs to scope overlay (not after frame cells), or adjust core selectors to identify the last cell independently of non-cell decoration.
- Lead owns ticket lifecycle and integrated validation. This agent retained no temporary generated outputs.
