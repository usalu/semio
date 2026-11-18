# os rows, first half (SpaceBrowser, AgentDelegations, ShellHost)

Files done (23 of the 47 os items): checklist sections 1 SpaceBrowser (9), 2 AgentDelegations (7), 3 ShellHost (7). Second half starts at section 4 (Paint2dHost). HubSignIn (section 7) is untouched and can reuse the new helper below.

Base path: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/`.

## What changed

- New shared helper `🔗️HubConnection/🪢️hub-rows/🟦️.tsx`: `HubFieldRow` (label cell + control cell in one horizontal row), `HubSelectField` (HubFieldRow around the framework `Select`), `HubRowList` / `HubRowItem` (vertical owner with `role="list"`, nested rows as `role="listitem"`).
- SpaceBrowser: native `input`/`select` replaced by framework `Input` / `Select`; create form, invite form and redeem form are each one vertical `SharedBorderRow` (field rows + submit Button as cells); search is a field row; invite card is a framed vertical row (text cells, `output` cell, nested copy/dismiss row, status cells); the space list is one `HubRowList` and each row a `HubRowItem` (open button cell + archive Button cell), no gap, no per-row border.
- AgentDelegations: same for the create form; credential card and its nested MCP block are vertical rows (MCP block is a nested row, so no box-in-box; `code`/`pre` tokens sit in cells without own radius/padding); delegation list is one `HubRowList`, each item a vertical row = [info cell + revoke Button] plus the revoke confirmation as a nested vertical row (section of the item, buttons as cells, no own frame).
- ShellHost: new `🏛️ShellHost/🧱️notice-bands/🟦️.tsx` (`NoticeStack`, `NoticeCell`, `FloatingPane`). The bootstrap, execution-target and artifact-creation notice stacks and the command stall list are now one frame with notices as cells (no gap, no native border). The InstalledService, HubWorkspace and SpaceAdministration wrappers dropped their native `rounded-sm border`: `FloatingPane` is a framed `SharedBorderRow` (compositor paints the single hairline on the fixed outer box) with the scroller inside.
- Left alone (checklist says fine): directory bootstrap, route admission, plugin install band, checkin band, corner trigger. The plugin install band and the stall band can sit stacked at the same spot (stall uses `translate-y-full`); they still draw two native borders there. Not in the 47, flagged only.

## Tests

- Ticket-local config `vitest-os.config.ts` (root = os product). Run from the repo root: `npx vitest run --config <ticket>/vitest-os.config.ts <filter>`.
- SpaceBrowser component suite: 35 pass (new consolidation test; selects now driven through the combobox/listbox; `li[data-space-id]` selectors became `[data-space-id]`).
- AgentDelegations: 31 pass (new consolidation test incl. confirmation as nested row).
- New `🏛️ShellHost/🧱️notice-bands/🧪️tests/🧩️component/🟦️.tsx`: 2 pass.
- HubConnection suites still pass. Three ShellHost `.ts` tests fail to collect under vitest because they import `bun:test` (pre-existing, unrelated).
- Not verified visually (no render check of the compositor seams in a browser).

## Gaps in the `SharedBorderRow` primitive

- No `as` / element override: lists need `ul`/`li` semantics; I used `role="list"` / `role="listitem"` on divs and changed test selectors. An `as` prop (or an exported `useSharedBorderFrameRef` from `@semio-tech/ui-react`, which is not exported today) would keep real `ul`/`li`.
- No label-cell variant: field rows use `SharedBorderRowCell` + a native `label htmlFor`; a `SharedBorderRowLabel` would remove the repeated `w-1/3 shrink-0` styling in `HubFieldRow`.
- No tone for the destructive confirmation (the old box had `border-destructive`); it is now a plain cell section.
- Compositor overlay vs scroll: a framed row with `overflow-auto` on itself would scroll its overlay, so `FloatingPane` keeps the scroller as an inner element. Worth documenting in the API section.
