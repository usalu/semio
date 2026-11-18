# 🧩 Nested bordered controls audit (box-in-box)

Read-only audit. No source file was edited. Base: `🧰️framework/🔨️modules/🖱️ui`, `🧰️framework/🛍️products` (excluding 📚️course), `✏️s/🔌️plugins`.

Pattern under audit: a bordered control (Select trigger, Input, Textarea, Button, Toggle; Checkbox excluded) sits inside an already-bordered row, cell, card or list item with padding or gap, so its own frame floats inside the parent frame. Target state: the control is a cell of the row, its outer edges coincide with the row edges, and there is one divider to its neighbour.

Related: `audit-products.md` (L90-91) already notes `SpaceBrowser` L268 and L172. `audit-ui-elements.md` covers other element-level edges and does not cover the self-framing primitives below.

## Summary of root cause

1. The primitives draw their own frame unconditionally. `ButtonGroup`, `Input`, `Textarea`, `Select` (trigger), and `Toggle` each render `data-shared-border-frame` and ignore whether a parent already frames them. Every `Button` is a `ButtonGroup`, so every `Button` self-frames.
2. The compositor (`🗂️WindowChrome/shared-border.ts`) paints each `[data-shared-border-frame]` on all four sides independently. The only nested-frame CSS rule is for navbar and footer (`🎨️styling/🖌️ui/🎨️.css` L6587-6590). Nothing suppresses a frame nested in another frame.
3. Bordered containers are ad hoc. About 100 non-story UI lines use `rounded(-sm|-md) border` as a row or card (for example `rounded-sm border px-single py-1`), and they have no shared "row frame" or "group" primitive. The Stepper (`🪜️Stepper/🟦️.tsx` L257, L321, L343) is the correct reference: one frame, inner input `border-0`, children are `data-shared-border-cell`.

## Ranked findings

### F1. Self-framing primitives ignore their container (shared helper, highest reach)

Fixes every call site in F2-F6 and most unlisted ones at once.

| Primitive | File:line | Offending classes / attributes |
|---|---|---|
| ButtonGroup (base of Button, ActionGroup, DialogFooter items) | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔳️ButtonGroup/🟦️.tsx` L63-77 | `data-shared-border-frame data-shared-border-edges="top right bottom left"`, `chromeControlGroupClass` |
| Button | `🔘️Button/🟦️.tsx` (wraps ButtonGroup, no own border) | inherits the ButtonGroup frame |
| Input | `✏️Input/🟦️.tsx` L382, L489, L510 | root `data-slot="input-root" data-shared-border-frame`; inner `border ... ${borderElementClass}` (native `border` on the inner `<input>`, L510) |
| Textarea | `🔤️Textarea/🟦️.tsx` L48, L111, L119 | root `data-slot="textarea-root" data-shared-border-frame`; inner `border ... ${borderElementClass}` |
| Select trigger | `🔽️Select/🟦️.tsx` L401-430 | `data-shared-border-frame`; trigger class includes `border` (L430) |
| Toggle / ToggleGroup | `🔀️Toggle/🟦️.tsx` L136, L184, L209 | `data-shared-border-frame` on the group, `data-shared-border-cell` on items |

Why it matters: a nested frame draws four edges plus a second inner edge, so the parent's edge and the child's edge sit a padding apart (the box-in-box look). The inner native `border` classes (Input L510, Select L430) add a second CSS border on top of the compositor line even when the frame is the only intended line.

Suggested fix (one mechanism, not per-site overrides):
- Add a context, for example `SharedBorderOwnerContext`, that bordered containers provide (row/list-item/card/Popover body/Dialog body wrappers, and `data-shared-border-frame` or `-group` owners). ButtonGroup, Input, Textarea, Select, and Toggle read it. When an owner is present they render `data-shared-border-cell` and no `data-shared-border-frame`, drop their native `border`, and keep `border-0`. When absent (top level), behaviour is unchanged.
- Alternative (DOM-level, no React context): in `shared-border.ts` `selector` handling, skip registering a frame whose nearest `[data-shared-border-frame], [data-shared-border-group]` ancestor exists, and add a CSS rule that strips the native `border` from `[data-shared-border-cell]` inside an owner. This needs a render check.
- Test: extend `🗂️WindowChrome/shared-border.test.ts` and `compositor.cases.json` with a nested-control case (Button inside a bordered row yields one cell, zero nested frames).

Confidence: high for the mechanism and call-site reach (read in source). Medium for the exact fix shape (not rendered; the alternative needs a visual check).

### F2. Ad hoc bordered row/card helper missing (shared helper, second highest)

Repeated pattern: `rounded-sm border px-single py-1` used as a list row or confirm box, with buttons inside. Propose a shared `BorderedRow` / `ControlRow` (group frame, children cells) so these rows use the same mechanism as `ToggleGroup`, `Tabs`, and `Stepper`. `Field` (`📝️Field/🟦️.tsx`) is clean and should stay border-free. Use it as the layout root and put the controls inside it as cells.

### F3. Product OS: `SpaceBrowser` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏘️SpaceBrowser/🟦️.tsx`)

- L268: `<li className="... rounded-sm border px-single py-1 ...">` row contains `<Button ... variant="outline">` (archive, L280). Offending: row `border`, button frame. Fix: row as group, archive as a trailing cell. Confidence: high.
- L206: `role="status" ... rounded-sm border px-single py-1` invite box contains copy (L210) and dismiss (L211) buttons. Fix: group, buttons as cells. Confidence: high.
- L88-106, L187-201, L237-239: native `<input>`/`<select>` with `rounded-sm border` and `<Button type=submit>`. These sit in unbordered sections (`border-t` section at L172 only), so no box-in-box here. Native controls bypass the primitives (inconsistency, not nesting). Confidence: high.

### F4. Product OS: `AgentDelegations` (`🤖️AgentDelegations/🟦️.tsx`)

- L141-146: credential box `rounded-sm border` with download (L145) and dismiss (L146) buttons. Confidence: high.
- L184-196: MCP client group `rounded-sm border` with install (L187) and copy (L196) buttons. Confidence: high.
- L234-243: delegation row `li rounded-sm border` with revoke button (L243). Confidence: high.
- L247-264: revoke confirm `rounded-sm border border-destructive` with cancel button (L264) and confirm button. Confidence: high.

### F5. Product OS: `SpaceAdministration` (`🛂️SpaceAdministration/🟦️.tsx`)

- L366: `role="alertdialog" ... border p-2` contains delete-confirm (L370) and cancel (L371) buttons. Fix: a group or a dialog footer (`DialogFooter` pattern, `💬️Dialog/🟦️.tsx` L596 uses `data-shared-border-group`). Confidence: high.

### F6. Product OS: `BlockListHost` (`🧩️BlockListHost/🟦️.tsx`)

- L83-89: block card `rounded border border-border` with delete `<Button>` (L89). Confidence: high.
- L115-133: step card `rounded border border-border` with delete `<Button>` (L133). Confidence: high.
- L197: palette group `data-shared-border-group` `rounded border border-border` containing "add step" `<Button>` (L271). Here the group border and the group attribute are both set. Remove the native `rounded border border-border` and keep the group. Confidence: medium-high.

### F7. Product OS: `TextEditor` (`✏️TextEditor/🟦️.tsx`)

- L661-672: `border-t` footer strips with raw `<button className="rounded border border-border px-3 py-1 ...">` (L663, L670-672). Raw buttons bypass the `Button` primitive, so they cannot be cells. Fix: footer as group, buttons via `Button`. Confidence: medium (footer is a bar with a top border only, so the inner borders are the box-in-box part).

### F8. Product OS: `IconSelector` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎴️IconSelector/🟦️.tsx`)

- L199: `<div className="flex ... gap-2 rounded-md border p-2">` container with `<Select>` (L200-201), `<Textarea>` (about L204-214), preview box at L240 (`rounded-sm border px-1 py-2`, non-control nested box), and two `<Button>`s (L242-243), all with padding and gap. The outer container is a frame-like card and every child is a framed control. Fix: remove the outer `border p-2` and make it a vertical group (`data-shared-border-group`, `data-orientation="vertical"`), with the preview as a cell. The `OverviewCard` `flush` prop (`🃏️OverviewCard/🟦️.tsx` L29-31) is the in-repo precedent. Confidence: high. This is in the elements scope and is a reusable component, so it is the most valuable element-level fix after F1.

### F9. Plugin CAD REPL (`✏️s/🔌️plugins/📐️cad/.../✏️editor/⚙️engine/📺️renderer/🎛️repl/🟦️.tsx`, and sibling `📋️repl-panes/🟦️.tsx`)

- `🎛️repl/🟦️.tsx` L148: floating `<aside data-shared-border-frame>` contains `<label className="flex flex-col gap-half">` rows with `<SelectTrigger>` at L251, L286, L311.
- Each trigger carries `cadFieldClass, "rounded-md border px-single py-half", borderNormalClass` (L251, L286, L311). That is a Select frame plus an explicit native border plus a normal-border token. Triple border on one control.
- `📋️repl-panes/🟦️.tsx` L110: same trigger class inside attribute rows.
- Fix: remove the explicit `rounded-md border ... borderNormalClass` (the primitive frames it), and let the aside frame be the only card edge. Confidence: high for the duplicate border; medium for the nesting (the label rows are unframed, so the box-in-box is the aside frame around controls with padding).
- Plugins: only these two files in `✏️s/🔌️plugins` have border classes. The other 24 tsx files in plugins have no border usage.

### F10. Bordered floating boxes hosting bordered forms (`ShellHost`)

- `🏛️ShellHost/🟦️.tsx` L217-264, L295, L317: `rounded-sm border bg-base` popovers. L249 (`data-elevation-root max-h-[80vh] overflow-auto`) and L264 can host `SpaceBrowser`/`HubSignIn` forms whose controls are bordered (F3, F4). Box-in-box is inherited from F3/F4. The ShellHost boxes themselves contain only underlined text buttons (L281, L307, L322, L385, L391), which are not bordered. Confidence: medium (depends on the rendered children, not read in detail).

### F11. `ChromePanels` theme paint row (`📌️ChromePanels/🎨️settings-theme/🟦️.tsx`)

- L175: `input type="color"` with `cn(borderElementClass, "... rounded border ...")`; L19 uses the same double-border class set. The alpha input follows. The row is a `control` slot, not framed, so this is not box-in-box. It is a double-border (native `border` plus `border-element` token on one element). Confidence: medium. Low priority.

### F12. Table cells with framed buttons (low)

- `🧰️framework/🛍️products/💻️os/.../📊️Table/🟦️.tsx` L230 (cell action `<Button variant=...>`) and `🧵️TaskManager/🟦️.tsx` L452-454, L717-722 place framed buttons inside table cells or list rows. The row carries only a bottom divider (`borderNormalBottomClass`, shared `Table` L258, L559), so this is a control in a cell, not a box in a frame. Consider a grouped action cell (`data-shared-border-group`) once F1 lands. Confidence: medium-low.

## Verified clean (no nested bordered control found)

- `📝️Field`: no border, no frame, label and wrapper only. Keep as the shared layout helper.
- KeyValue: no such component exists. `🔑️KeyValue/🎯️targets/🧊️wgpu/🦀️.rs` is the only file and draws its own control border through `push_control_border`. Nothing to fix in TSX.
- `📝️Form`, `📃️List`, `📋️MenuItem` (story only), `🔍️WindowSearch`, `🧙️Wizard`: no border classes in production code.
- `⌨️Command`: `command-input-wrapper` carries the bottom divider and the input is unframed (`⌨️Command/🟦️.tsx` L286-298). Correct.
- `🔚️Footer`, `💬️Dialog` footers (`💬️Dialog/🟦️.tsx` L596), `🗨️Popover`: footers are shared-border groups, so buttons are cells. Correct.
- `🪜️Stepper`: correct reference (single frame, inner `border-0`, cells). Use as the model.
- `🌳️Tree` rows and `🕹️control-tree` (`🌳️Tree/🕹️control-tree/🟦️.tsx` L126-154, L172-176): rows are unframed grid rows, controls are not nested in a bordered parent. Becomes an issue only if a consumer adds a border class to `classNames.panel`/`controlRow`; none found.
- `🔀️Toggle`/`🎛️ToggleGroup`, `⚡️ActionGroup`, `📑️Tabs`: groups with cells, correct.
- `🌳️Tree/🟦️.tsx` L638 `TreeFiles` card: contains links only, no controls.
- `📊️Table` (element): row borders only, no controls in the element itself.

## Suggested order of work

1. F1: nesting-aware primitives (context or compositor rule), plus a test case in `shared-border.test.ts` and `compositor.cases.json`. Fixes every nested Button, Input, Textarea, Select, and Toggle at once.
2. F2: shared bordered row/group helper (`ControlRow` or `BorderedRow`) used by F3-F6.
3. F8 (IconSelector), F9 (CAD REPL triple border), F7 (TextEditor raw buttons): local fixes after 1 and 2.
4. F10-F12: follow-up after the render check.

## Confidence key

- High: read the file and the class or attribute on the element, and the containing element carries a border or frame at that point in the tree.
- Medium: mechanism is clear from source, but the rendered result or a child component was not read in full.
- Low: inferred from class names only.

Not run: no tests, no build, no render. Read-only per the brief.
