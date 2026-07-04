# S Studio Behavior Checklist (from s/core/js/index.ts + f8376e848)

## Home commands
- createStudio, importStudio, openStudio, navigateVirtualFileSystemNode, goHome

## Studio commands (SPlayController.run)
- mediaGraphEngagementInput, mediaGraphEngagementSubmit
- compiledDagEngagementInput, compiledDagEngagementSubmit
- setMediaNodeSelection, setAppInstanceSelection
- patchMediaNodes (position x/y), patchAppInstances (label)
- selectInstance, spawnApp, openInstance, closeFocusedInstance
- undo, redo, commitCheckpoint, checkoutCheckpoint
- setActiveExample, addParameter, removeParameter, patchParameter
- bindParameterField, unbindParameterField
- goHome, applySOsUri (URI routing)

## Panel tabs (right)
- Catalogue (spawn programs), Parameters (numeric/categorical/toggle/text), Inspection (batch label, position, bindings)

## Windows (studio)
- Media Graph (node-graph, 40% golden), Media VFS (30%), Compiled DAG (30%, text-editor)

## Keybindings
- mod+z undo, mod+shift+z redo, mod+n createStudio (home)

## Vitest parity (s/core/js)
- demo projection (5 instances, edge, 2 params)
- checkpoint round-trip, spawn draw, puzzle5d/shooting multi-port
- catalogue tree, inspector label field, patchAppInstances batch
- openInstance/closeFocusedInstance, registry completeness
- checkoutCheckpoint after spawn

## Shell (platform renderer f8376e848)
- ProductShell: navbar (SemioLogo, app/example select, mode, PanelToggleGroup, theme/compact/expertise)
- windowMeasuresToGolden, tab stacks, engagement rails
- useUIHistory (back/forward/up/navigate)
- useCommandHotkey / keybinding dispatch
- renderUiControl (all control kinds)
- Tree panels with drag-and-drop, VFS surfaces

## LOC targets (old → new)
| Old file | LOC | New file |
|----------|-----|----------|
| platform/renderer/react/index.tsx | 5880 | framework/renderer/react/os-shell.tsx + ui-interpreter.tsx |
| playground/renderer/react/index.tsx | 2207 | framework/renderer/react/os-shell.tsx |
| platform/core/js/index.ts | 3804 | framework/core/rs/ui.rs + layout.rs |
| playground/core/js/index.ts | 1566 | framework/core/rs/layout.rs |
| os/core/js/index.ts | 3095 | framework/product/os/core/rs/*.rs |
| s/core/js/index.ts | 1579 | s/plugin/rs/lib.rs |
| s/react/index.tsx | 521 | s/plugin/rs/lib.rs |
| each tech core+react | varies | tech/plugin/rs/lib.rs |
