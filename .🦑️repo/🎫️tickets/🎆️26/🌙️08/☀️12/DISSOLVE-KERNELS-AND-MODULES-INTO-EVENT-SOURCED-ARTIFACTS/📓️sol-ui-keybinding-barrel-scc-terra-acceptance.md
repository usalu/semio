# UI Keybinding Barrel SCC Acceptance

## Lease Outcome

Keybinding behavior no longer originates in the React barrel. Four UI-owned modules hold text interpretation, control-binding context, hotkey presentation, and persistence; the barrel explicitly imports and re-exports only the required public contracts while retaining its Shell anchor and panel code.

## File Inventory and Hashes

| Status | Path | SHA-256 |
| --- | --- | --- |
| Added | 🧰️framework/🔨️modules/🖱️ui/🔨️modules/⌨️keybinding-text-interpretation/🟦️component.ts | 62fbece748274716a6b850127ab66cb522b2cb4ef4966f4901dfcd52c8dd04c0 |
| Added | 🧰️framework/🔨️modules/🖱️ui/🔨️modules/⌨️control-keybinding-context/🟦️component.tsx | 9b739a01047cae47601b696a4b21594d44fd8a57a4bef4cc36f439cb263a2d3b |
| Added | 🧰️framework/🔨️modules/🖱️ui/🔨️modules/⌨️control-hotkey-presentation/🟦️component.tsx | 32684070b5460ddc1e737d732902045f583df368e3b571a6cbfa9bc6eb05c147 |
| Added | 🧰️framework/🔨️modules/🖱️ui/🔨️modules/💾️keybinding-persistence/🟦️component.ts | 4ea12dce6fc25e18d17d1aa53227cd5ba4390029a6a9da23021cb58521208312 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ButtonGroup/🟦️component.tsx | 0a5fd2b80f479b00961df9b3361f0e514476d4a3c217b17b1e283cb459e26d28 |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎛️ToggleGroup/🟦️component.tsx | 3a43381979d734a3934bd7c4e26475c242eaa62e5221334e6e4c2056984fb41b |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️UIDialog/🟦️component.tsx | 1802bab543d7360bcc90fc6bb68192099078170d5f762626d8cc254ae3f3d09f |
| Updated | 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️component.tsx | e6d0b25fdb3637f21a65d46b65fa0603fd2bb72cc8fa358c6061445942799446 |
| Registrar-owned update | 🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx | de3c18afdb4a6cb03ef35814457c139547b268d7ba960748ff5bc4c652a52f99 |

## Static Acceptance

- Keybinding text interpretation exports normalized chord parsing and formatted first-chord display. Its Apple-platform classifier is private; ContextMenu now imports formatting directly and no longer defines or exports the classifier.
- Control-keybinding context owns the shell defaults, merge, context/provider, lookup, resolution, hotkey, and binding hook. The action, definition, callback, option, and dependency contracts are repository-owned. Provider props are private and its children are opaque at the public boundary.
- Hotkey presentation exports only its named Badge props and Badge. The inline visibility helper and shortcut class are private; the zero-consumer tooltip helper was removed.
- Persistence retains the exact `ui.keybindings.overrides` wire key and invalid-entry filtering while keeping both the key and parser private. Its read/write functions are the sole public persistence API.
- ButtonGroup and ToggleGroup directly consume chrome presentation and the Badge; UIDialog directly consumes useControlKeybinding; ContextMenu directly consumes shortcut formatting. No keybinding-specific reverse import from those four consumers to the React barrel remains.
- The barrel contains explicit imports/re-exports rather than duplicate authored definitions. It retains SHELL_PANEL_ANCHOR_KEY_IDS, useActionHotkey, PANEL_TOGGLE_HOTKEYS, and usePanelChromeHotkeys; useActionHotkey now also uses the repository-owned callback, options, and dependency contracts.
- The static stale scan found no barrel use of isAppleUiPlatform, useControlHotkeyTooltipVisible, useControlHotkeyInlineVisible, controlHotkeyShortcutClassName, UI_KEYBINDING_OVERRIDES_STORAGE_KEY, parseUiKeybindingOverrides, Parameters<typeof useHotkeys>, or React.DependencyList.
- Shared-module terminal evidence: text reaches ContextMenu, persistence, and control context; control context reaches UIDialog and Badge, then ButtonGroup and ToggleGroup; Badge has ButtonGroup and ToggleGroup as direct terminal consumers; persistence reaches renderer Shell and ShellHost through the public package. The directed new-module edges are persistence → text, context → text, and Badge → context, so the SCC dependency-cycle scan is clean.
- Scoped ordinary and cached git diff --check commands completed cleanly.

## Nx Validation

Each required target ran once through uncached Nx with --skip-nx-cache.

| Command | Outcome |
| --- | --- |
| bun nx run @semio-tech/ui-react:lint --skip-nx-cache | Passed. |
| bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache | Failed on broad existing framework and UI diagnostics, including plugin/event contracts, generated manifest symbols, styling readonly writes, translations, XYFlow, Table, Icons, Tree, and product fetch typing. |
| bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache | Ran 520 tests: 510 passed, 10 failed, and 2 jsdom unhandled errors. Existing failures cover Scene camera mocks, icon hover CSS, CanvasPickMenu, shell measurement, Tree, VirtualFileSystem, and pointer-event handling. |
| bun nx run @semio-tech/ui-react:build --skip-nx-cache | Failed independently because Storybook cannot resolve @semio-tech/coda-desktop/renderer from .storybook/stories/ui/🌳OntologyTree.stories.tsx. |

## Blockers

No Keybinding SCC source or registrar blocker remains. The non-passing gates are blocked by unrelated workspace type errors, the existing quick-test failures and jsdom errors, and the unresolved Storybook Coda renderer dependency.
