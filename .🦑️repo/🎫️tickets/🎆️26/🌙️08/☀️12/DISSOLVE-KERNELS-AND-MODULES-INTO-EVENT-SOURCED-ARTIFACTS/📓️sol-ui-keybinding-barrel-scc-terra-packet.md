# UI Keybinding Barrel SCC Terra Packet

## Lease Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- React barrel SHA-256 after ButtonCycle registrar: `8494f5da41ac9bcde40169278f6ad9a2749167b72ceef703b2eb31a6f606c906`
- ButtonGroup: `acef2a6a27df2373cf607a002751fa61a213f9854202f2da3fe57efe3a5cabd0`
- ToggleGroup: `7097f9ded7e6945b1be691640215a91836177da341f405962aa732c27951194f`
- UIDialog: `afa983f9bc1887c4c501594ca9e2ec42e520b6027995bcddc9343a69fe78268c`
- ContextMenu: `6d986501d5d68c440fc7da15ac9c3d0b3b40abf1d031631db36b84613f919d34`
- UiDriver dependency: `9baffb7eb658388153924a625bb1f8229b9f7f197f680803e290d5b7d9f78e16`
- Ports dependency: `dbfe7968b1322633ba5db67f86a74e9f88b529d2558cae76044e68102fd347ad`
- ElementId dependency: `308951b26486abda4a67e5adda3273ac8eff260e924b3f57a0728ed110cfc38d`

Read root/UI/React-package AGENTS before editing. Use `apply_patch` only and no modifying Git commands. Preserve accepted dirty content exactly outside the leased regions.

## Writable Source Scope

Create these exact specific UI-owner modules:

- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/⌨️keybinding-text-interpretation/🟦️component.ts`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/⌨️control-keybinding-context/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/⌨️control-hotkey-presentation/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/💾️keybinding-persistence/🟦️component.ts`

Update only:

- `🧱️elements/🎛️ButtonGroup/🟦️component.tsx`
- `🧱️elements/🎛️ToggleGroup/🟦️component.tsx`
- `🧱️elements/💬️UIDialog/🟦️component.tsx`
- `🧱️elements/🖱️ContextMenu/🟦️component.tsx`
- unique ticket acceptance Markdown

Do not edit the protected React barrel, UiDriver, Ports, ElementId, product renderer, Storybook, package manifests, locks, generated files, or any other path.

## Semantic Split

### Keybinding Text Interpretation

Move `parseKeybindingChords`, `formatKeybindingShortcut`, and `isAppleUiPlatform` here. Platform classification becomes private unless a real production caller besides formatting remains. Preserve glyphs, normalization, alternative-chord selection, and platform behavior exactly.

### Control-Keybinding Context

Move `buildKeysByActionId`, `SHELL_KEYBINDINGS`, `composeControlKeybindings`, the context/provider, binding lookup/resolution hooks, `useControlHotkey`, and `useControlKeybinding` here. Import `reactHostPort`, `resolveControlLabelId`, text formatting, `ephemeralMap`, and `useHotkeys` directly; never import the barrel. Define repository-owned input, provider, and hook-option contracts. Do not expose `Parameters<typeof useHotkeys>`, anonymous React props, or an external dependency type in the public API. Keep lookup helpers public only when required by authored barrel shell hooks; otherwise private.

Do not move `SHELL_PANEL_ANCHOR_KEY_IDS`, `useActionHotkey`, `PANEL_TOGGLE_HOTKEYS`, or `usePanelChromeHotkeys`; the coordinator will leave/import their dependencies until their separate protected ShellHost/private-Anchor lease.

### Control-Hotkey Presentation

Move `ControlHotkeyBadge`, its shortcut class, and inline visibility here. Use a named repository-owned props contract, import `useUiDriver` and `useControlHotkey` directly, and never import the barrel. Delete `useControlHotkeyTooltipVisible`; it has zero active consumers.

### Keybinding Persistence

Move the override storage key, parser, read, and write behavior here. Import repository `StoragePort`, `isElementId`, and keybinding parsing directly. Preserve storage key/wire values and invalid-entry filtering exactly. Keep parser/key private unless required by another production component.

## Consumer Rewiring

- ButtonGroup and ToggleGroup import chrome classes directly from `🎛️chrome-control-presentation` and `ControlHotkeyBadge` directly from its new module.
- UIDialog imports `useControlKeybinding` directly from control-keybinding context.
- ContextMenu imports formatting from keybinding-text interpretation and removes its local `isAppleUiPlatform` definition.
- Preserve all other source bodies and accepted ClassNames changes.

## Registrar Handshake

Stop after new modules and consumer imports land. Send final hashes, a symbol-to-module map, the exact definitions that the coordinator must remove/replace in the barrel, and confirmation the barrel remains at the supplied hash. Do not run Nx yet.

The coordinator will replace the in-barrel definitions with explicit imports/re-exports, preserve shell-only Anchor/panel binding code, remove newly unused barrel external imports, and return the new barrel hash.

## Final Validation

- No keybinding-specific reverse import from ButtonGroup, ToggleGroup, UIDialog, or ContextMenu to the React barrel.
- No duplicate in-barrel definitions, zero-consumer tooltip hook, external-library public type, wildcard export, or compatibility wrapper.
- Each new shared module has at least two independent terminal production consumers through its reverse closure.
- Scoped ordinary/cached `git diff --check` and dependency-cycle scan.
- Run UI React lint, typecheck, test-quick, and build once through uncached Nx after registrar signal; classify unrelated failures without repairs.
- Write unique acceptance Markdown with inventory, hashes, consumer evidence, commands, outcomes, and blockers.
