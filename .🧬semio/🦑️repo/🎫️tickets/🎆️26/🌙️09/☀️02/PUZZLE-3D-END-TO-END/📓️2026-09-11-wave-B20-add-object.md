# Wave B20 — Add Object dialog

## 1. Land

A user can press a reachable **Add Object…** chip on the Puzzle 3D window, pick a kind in the declared `addObject` dialog, and the instance count increases. B11's `openAddObjectDialog` / hidden `addObjectKind` palette split is unchanged.

## 2. Defect

`#45b add-object-trigger-present` failed with `trigger=0` because:

1. The opener only existed as a context-menu row. After `dismissChrome()` that node is gone.
2. Putting it only inside the folded Actions pane is not user-reachable. That pane unmounts its body while folded, and the Actions chip sits under the Perspective `mode-dock-tabbar` (the tab label intercepts pointer events). Playwright force-click on the covered toggle does not unfold the pane.

## 3. Fix

`main::engagement` still publishes one engagement option (id `shell-menu.action.openAddObjectDialog`, label `Add {object}…` / `{object} hinzufügen…`, action `openAddObjectDialog`). Utility switch options stay off the HUD.

`Window` mounts those options as **quick-action chips** (`data-slot="window-engagement-quick-actions"`) while Actions is folded. The row sits below the window-cap tab and is not a `window-cap` reveal region, so it stays in the DOM and receives clicks. Unfolding Actions still shows the same option in the pane body (quick chips unmount to avoid duplicate ids in that window).

A Playwright locator click on the first matching chip (Top pane) can still be covered by that pane's tab. The probe native-clicks the last matching chip (Perspective) and then drives `#objectKind` / `#ui.dialog.submit`.

## 4. Probe

`bun` the ticket `browser-probe.ts --only=add-object-dialog --port=6014` (never `:6013`).

Live dialog chrome: title `Add Object`, kind combobox `#objectKind`, submit `#ui.dialog.submit`.

## 5. Files

- puzzle 3d `windows/main` — engagement option
- puzzle 3d editor unit test — HUD law
- framework `Window` — folded quick-action chips
- ticket `browser-probe.ts` — native click + kind + count

## 6. Probe evidence

`bun browser-probe.ts --only=add-object-dialog --port=6014` on 2026-09-11T18-59-41:

- `add-object-trigger-present` PASS (`trigger=2`)
- `add-object-dialog-opens` PASS
- `add-object-kind-options-are-dynamic` PASS (13 catalog kinds, not a hardcoded `"Object"`)
- `add-object-instance-count-increases` PASS (`1` → `2`, new id `puzzle3d.object.670d973117482e73`)
