# Mounted Locale Retained Refresh

## Contract

The neutral `🌐️settings-locale-panel-refresh` contract names four simultaneously mounted shell-owned leaves: General, Theme, Hotkeys, and Default Apps. Conflicts remains unmounted. Locale dispatch must snapshot exactly those currently retained shell leaves, publish no successor in the dispatch call, and replace at most one owner per frame-maintenance step. A refused retirement admission keeps both the readable prior lease and the same cursor head. A newer locale generation replaces the old pending roster. No step requests guest refresh and no document or retirement capacity changes.

The React oracle mounts the shared `Tree` with labels from the real `uiI18n`/`shellLabel` authority. English `General`, `Theme`, and `Hotkeys` become German `Allgemein`, `Thema`, and `Tastenkürzel` after the real language change and rerender. The schema and independent one-owner cursor reducer run in the same suite.

Focused React result:

```text
Test Files  1 passed (1)
Tests       2 passed (2)
Duration    13.13s
```

Command:

```text
SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌐️settings-locale-panel-refresh/🟦️.tsx --config /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts
```

## WGPU ownership

`setLocale` now compares the accepted locale, reserves the next checked locale-refresh generation, updates the host locale, synchronizes localized dock tabs, and arms one `ShellLocalizedPanelRefreshCursor`. The cursor inventory is derived from `shell_owned_panel_leaves` intersected with `panel_documents`, so arbitrary guest documents and absent shell panels cannot enter it. Its boxed inline owner adds no new fixed-capacity arena and its pending queue is bounded by the already admitted shell leaf inventory.

The existing frame-maintenance lane owns progress. One call attempts one cursor head through the transactional `republish_shell_panel_document`. Success pops that one id after exact prior-lease retirement; refusal retains it for retry. Locale mismatch cannot advance an old cursor, and a newer dispatch replaces the cursor with a fresh mounted snapshot and higher generation. Existing preference-load, persistence, presence, and layout work retain their prior priority/order except that the localized roster runs immediately after preference load. No guest body refresh, `refresh_ui`, capacity increase, or synchronous whole-roster loop was added.

The WGPU catalog strings for this exact neutral contract now match React: English `Hotkeys` and German `Thema`. The prior WGPU strings `Keybindings` and `Design` disagreed with the real shared React translation authority.

## Native laws and source boundary

Two laws were added to the existing General settings corpus:

- `locale_refresh_republishes_one_exact_mounted_shell_owner_per_maintenance_step` requires the exact four-id roster, unchanged headers immediately after dispatch, exactly one additional successor per step, German retained section labels, absent Conflicts, terminal prior-owner retirement, and an empty guest refresh scope.
- `locale_refresh_refusal_and_supersession_preserve_exact_owner_and_generation` exhausts the exact retirement admission, requires the unchanged readable header and unchanged cursor head, resumes after admission returns, then supersedes the partial German generation with a complete newer English roster and requires canonical English retained labels.

No Cargo/native command was run in this lane. These laws and production sources are ready for the root-owned Native58 gate.
