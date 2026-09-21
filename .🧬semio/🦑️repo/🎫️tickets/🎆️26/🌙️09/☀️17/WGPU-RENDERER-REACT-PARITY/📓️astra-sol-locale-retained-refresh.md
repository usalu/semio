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

## Native 59 failure correction

Native59 proved the bounded production cursor coherent and exposed three stale law assumptions:

- `setLocale` deliberately retains the exact General owner during dispatch and publishes it from the next frame-maintenance step. The General publication law now encodes `publicationLane: maintenance` for locale and `publicationLane: dispatch` for the five single-panel mutations, requires the unchanged dispatch header and exact General cursor head, and removes its test-only continuation republication.
- `Thema` is the localized dock-tab label installed synchronously by `sync_dock_tabs`. The retained Theme body's real localized first section is the selector label `Theme/Design`.
- Hotkeys is a real `UiNode::Section`; retained publication stores its label on a Section-role `Component::Container`. The law now reads the record label rather than accepting only `Component::TreeSection`.

The locale laws separately assert localized dock labels (`General/Allgemein`, `Theme/Thema`, `Hotkeys/Tastenkürzel`) and localized retained-body labels (`General/Allgemein`, `Theme/Design`, `Hotkeys/Tastenkürzel`). Exact refusal ownership, same-head retry, generation supersession, one owner per maintenance step, terminal prior-owner retirement, absent-panel refusal, and empty guest refresh scope remain asserted without a cap change.

Focused React/schema receipt:

```text
Test Files  2 passed (2)
Tests       12 passed (12)
Duration    7.57s
```

Command:

```text
SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bun x vitest run /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚙️settings-general-layout/🟦️.ts /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌐️settings-locale-panel-refresh/🟦️.tsx --config /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts
```

No Cargo/native command was run in this correction lane. The root-owned gate should execute the three exact Native59 settings laws.

Native60 stopped during compilation before executing any law because the retained-label helper read a removed `UiNodeRecord::label` field. The helper now reads the actual typed owners: `Component::Container(ContainerProps::label)` for Section-role settings bodies and `Component::TreeSection(TreeSectionProps::label)` for tree sections. The assertions remain exact; no label or gate was weakened.

Native61 reached the laws and showed their manually mounted retained documents had never initialized the separate dock-tab catalog before asserting its synchronous labels. `ShellState::new` deliberately starts that catalog empty; production initializes it before panels mount. The shared fixture mount now calls the real `sync_dock_tabs` first, then publishes the exact four retained documents. This preserves the distinction between dock presence and retained-body presence: Conflicts can exist in the dock catalog while remaining absent from the mounted document roster and locale cursor.

Native62 proved that diagnosis was incomplete: `default_dock` deliberately returns an empty dock until an `ActiveSession` exists, so calling `sync_dock_tabs` on a sessionless fixture correctly leaves the catalog empty. Both locale laws failed before observing refresh behavior, at their first General dock lookup. The fixture now mounts the same `ActiveSession` shape used by the registered command test app, calls the production `sync_dock_tabs`, records the complete recursive `(id, label)` roster, and requires all four fixture leaves to be present before publishing their retained documents. A missing leaf reports the complete actual roster. No dock nodes are manually injected. Locale dispatch therefore exercises the production path: a live session owns the dock source, `setLocale` rebuilds that source, and only the four explicitly published retained bodies enter the bounded refresh cursor.

Native62 receipt before this correction: `1244 tests run: 1242 passed, 2 failed, 0 skipped`; the Ink lifecycle law passed and the two locale laws alone failed at the sessionless dock lookup. Native63 should run those two exact locale filters.

Native63 verified the correction: **2 selected locale tests passed, 1242 outside the filter, 0 failures**. Nextest test time was **0.056s**; the root-owned uncached Nx gate completed in **4m45s**. This is the first native receipt in which the locale laws observe the real session-derived dock roster and complete their bounded refresh/refusal/supersession assertions.
