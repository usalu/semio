# Summary

Sketchpad app modules, state machine wiring, and shared app surfaces for Home, Kit, Design, Type, Quality, Docs, and Feedback.

# Docs

## elements.tsx

`Table` supports row-level hover callbacks for app hover state dispatch.

## Home.tsx

Home app hover state is stored in the Sketchpad state machine and updated via hover commands for table rows.

## Kit.tsx

Kit app hover state covers all artifact kinds and is updated via table and diagram hover dispatch.

## Sketchpad.tsx

Home command hooks forward hover events, including clear, into the Sketchpad state machine.

# Specs

## State Management

App hover and selection state MUST be managed by the Sketchpad state machine.

## Toolbar

The toolbar is a floating panel positioned at the bottom center of the canvas. Each app registers toolbar sections.

- **Home app**: Filter toggles for kit kinds (temporary, local, remote) with action buttons to create new kits
- **Kit app**: Filter toggles for artifact kinds (designs, types, qualities, ports, tags, concepts, files, folders, authors) with action buttons to create new artifacts
- **Design app**: Selection tools (normal, additive, subtractive) and lasso tools (rectangular, freeform)
- **Type app**: Selection tools (normal, additive, subtractive, intersect) and connector creation tool
- **Feedback app**: Send button to submit feedback form
- Toolbar tool definitions MUST use a shared hierarchical tool tree contract so each top-level tool can declare subtools; Settings tool subtools are `App Settings`, `Command`, and `Tools`.
- Subtool dropdown rendering MUST be group-agnostic infrastructure while preserving current behavior: existing selection dropdown behavior remains unchanged unless additional groups explicitly define multiple subtools.

Toolbar panel visibility defaults to true for all apps in default state creation.

### Toolbar Sizing Normalization

All toolbar interactive elements (Toggle, Button, ActionGroup) MUST derive their height from the `ToolbarZone` container, not from internal hardcoded values. This is enforced via CSS normalization rules in `globals.css` that target `data-slot` attributes:

- `[data-slot="toolbar-zone"] [data-slot="toggle-group"]` and `[data-slot="button-group"]`: `border-width: 0; height: 100%`
- `[data-slot="toolbar-zone"] [data-slot="toggle-group-item"]` and `[data-slot="button-group-item"]`: `height: 100%`

This ensures that `ToolbarZone` is the single source of truth for element height (via `--toolbar-item-height`), inner group borders are stripped (the zone provides the outer border), and all elements render at identical heights regardless of component family (Toggle, Button, ActionGroup).

## Interaction State

Hover and selection feedback across Home, Kit, Design, Type, Quality, Docs, and Feedback is driven by the app state machine.

Hover and selection highlights MUST be consistent across tables, lists, and diagrams.

Design Details routing MUST normalize connector selection from both legacy singular `connector` and canonical array `connectors` selection shapes before rendering inspector sections.

Design selection invariants MUST hold at every read/write boundary: `selection.pieces` and `selection.connections` are always arrays, and active port selection is mutually exclusive with piece/connection selection.

Design mixed selection behavior MUST render the mixed-selection warning section and MUST NOT render conflicting piece/connection inspector editors simultaneously.

Design pane-click clear behavior MUST clear selection through design commands/hooks and preserve canonical empty selection shape.

Design replacement dropdowns in Details MUST resolve replacement candidates from design context + selected piece ids, using normalized piece identity (`guid` and `type.guid`) so object/string type encodings do not break option resolution.

Design piece replacement updates MUST keep design references coherent: direct design pieces update `piece.design.guid`, while included design pieces update encoded `piece.type.variant` only when allowed by included-design kind constraints.

Design selector reads for selection and entity-bound piece/connection checks MUST validate GUID-shaped ids and fail closed in dev mode with `[DEBUG]` diagnostics instead of throwing.

Design write actions for piece/connection updates MUST reject invalid or missing entity ids to prevent undefined writes from mutating app state.

Design selection and transaction debug logging MUST be disabled by default and only enabled in dev via `window.localStorage['semio.sketchpad.debug']` (channels: `selection`, `transactions`, `all`) or `globalThis.__SEMIO_SKETCHPAD_DEBUG__`.

Design drag interactions MUST start transaction lifecycle on drag start, finalize on drag stop, and abort on escape cancel so undo grouping remains one gesture per history step.

Design diagram selection synchronization MUST normalize selection ids (non-empty GUID strings, de-duplicated) and compare set-membership before dispatching selection updates to avoid feedback loops during React Flow selection events.

Selection composition across Design/Kit/Type MUST use shared composition semantics (`replace`, `additive`, `subtractive`, `intersect`) and shared helper utilities from `shared.ts`.

Selection modifier interpretation across Design/Kit/Type MUST be resolved centrally (`Shift => additive`, `Alt/Ctrl/Meta => subtractive`, combined modifiers => intersect) and then mapped to selection tool kinds.

Design selection setters MUST no-op when incoming selection is semantically equivalent (pieces/connections set-equal and same primary connector) to avoid repeated selection dispatch loops under high-frequency UI selection events.

Plain app-store transactions MUST snapshot baseline state at transaction start, push exactly one history entry only when state changes at finalize, and restore baseline state on abort.

## Borders

- Element border kind (hover color)
- Window border kind (normal border color)
- Window spacing: 1-unit gap between windows and 1-unit margin to canvas edge
- Base canvas uses the base background surface; windows, panels, and temporary UI surfaces use their respective background levels
- Exactly one window is active in a multi-window layout; the active window surface uses an active background tint
- Table views use the active window surface background
- Global Sketchpad shell is wrapped in base level so Navbar/Footer resolve base background
- Panels are rendered under panel level so panel surfaces resolve panel background
- Window chrome controls MUST be rendered as Action UI elements
- Window frames use inset overlay strokes so all four edges remain visible with clipped layouts

## Windows

Sketchpad apps MUST render inside a multi-window workspace.

Each app MUST define a set of window kinds and a default window layout.

Window layouts MUST be persisted per app as JSON strings.

The active window MUST be tracked for focus-sensitive UI.

Window chrome MUST expose action controls for open-in-new-window, maximize/minimize, and close.
