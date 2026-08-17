# UI Keybinding Registrar Integration

## Baseline

- Protected React barrel baseline SHA-256: `8494f5da41ac9bcde40169278f6ad9a2749167b72ceef703b2eb31a6f606c906`.
- Terra source handshake confirmed four specific UI-owner modules and direct consumer rewrites before the registrar edit.
- The React barrel was the only serialized registrar in this integration.

## Registrar Changes

- Removed the stale `ContextMenu` `isAppleUiPlatform` import and re-export.
- Replaced authored keybinding parsing, formatting, registry context, badge presentation, and persistence behavior in the list root with mechanical imports and explicit exports from:
  - `⌨️keybinding-text-interpretation`;
  - `⌨️control-keybinding-context`;
  - `⌨️control-hotkey-presentation`;
  - `💾️keybinding-persistence`.
- Retained list-root/package-specific assembly only: `SHELL_PANEL_ANCHOR_KEY_IDS`, `useActionHotkey`, `PANEL_TOGGLE_HOTKEYS`, and `usePanelChromeHotkeys`.
- Did not re-export the private Apple classifier, tooltip/inline visibility helpers, shortcut class, storage key, or persistence parser.
- Exported repository-owned keybinding contracts explicitly; no new external-library type is exported by the modules.
- Replaced retained `useActionHotkey`'s `react-hotkeys-hook` and React public types with the repository-owned callback, options, and dependency contracts after the lease's static audit identified that pre-existing leak.

## Static Evidence

- React barrel final post-registrar SHA-256: `de3c18afdb4a6cb03ef35814457c139547b268d7ba960748ff5bc4c652a52f99`.
- Removed-symbol scan in the barrel returned zero matches.
- Scoped `git diff --check` over the barrel and four modules passed.
- Full UI validation is delegated back to the Terra semantic lease after this serialized registrar handshake.
