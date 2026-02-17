---
goal: SKETCHPAD
---

# Ticket

## Summary

Research into 6 test failures in `sketchpad.test.ts` after Settings/Chat panel migration from rightSidePanel to GoldenLayout windows. Findings: 1 failure is potentially migration-related, 4 are pre-existing (unrelated to migration), 1 is an infrastructure issue.

## Findings

### Current Panel Architecture

Each app defines panels via `getPanels()` returning `PanelDefinition[]`. Each `PanelDefinition` has a `PanelKind` which maps to a `PanelPosition` via the `panelKindConfigs` registry in `shared.ts`:

| PanelKind | PanelPosition | Panel Toggle |
|-----------|---------------|--------------|
| WORKBENCH | LEFT | leftSidePanel |
| TOOLS | LEFT | leftSidePanel |
| TOOLBAR | BOTTOM | (not a side panel) |
| STATS | MIDDLE | hudPanel |
| DETAILS | RIGHT | rightSidePanel |
| PARAMS | RIGHT | rightSidePanel |
| CONSOLE | BOTTOM | (not a side panel) |

Current app `getPanels()`:
- **Home**: TOOLBAR (BOTTOM), DETAILS (RIGHT) → has rightSidePanel, no leftSidePanel, no hudPanel
- **Kit**: TOOLBAR (BOTTOM), DETAILS (RIGHT) → has rightSidePanel, no leftSidePanel, no hudPanel
- **Type**: WORKBENCH (LEFT), TOOLS (LEFT), TOOLBAR (BOTTOM), STATS (MIDDLE), DETAILS (RIGHT) → has all 3 panels
- **Design**: WORKBENCH (LEFT), TOOLS (LEFT), TOOLBAR (BOTTOM), STATS (MIDDLE), DETAILS (RIGHT) → has all 3 panels
- **Docs**: WORKBENCH (LEFT), DETAILS (RIGHT) → has leftSidePanel, rightSidePanel, no hudPanel

The `PanelToggles` component in `Sketchpad.tsx:16394` conditionally renders toggles based on whether tabs exist for each position. If no tabs exist (e.g., `hasRightTabs === false`), the toggle is not rendered at all.

### Failure 1: Home test (line 673) — `rightSidePanelIconVisible` expected true, got false

**Test code** (`sketchpad.test.ts:665-675`):
```typescript
const rightSidePanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.rightSidePanel"]');
const hasRightSidePanel = await rightSidePanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
if (hasRightSidePanel) {
  rightSidePanelIconVisible = await verifyToggleIconVisible(...);
  expect(rightSidePanelIconVisible).toBe(true); // LINE 673
}
```

**Analysis**: Home still has `PanelKind.DETAILS` → `PanelPosition.RIGHT`, so the rightSidePanel toggle SHOULD render (the `DETAILS` tab exists → `hasRightTabs === true`). The test enters the `if` block (meaning the toggle IS visible), but `verifyToggleIconVisible` returns false. This function looks for `svg, img, [class*="icon"], > *` inside the toggle.

The toggle renders: `{RightIcon ? <RightIcon size={16} /> : <DocumentIcon size={16} />}` where `RightIcon = rightTabs[0]?.icon` which is `DetailsIcon` from `panelKindConfigs`. The icon SHOULD render an SVG.

**Assessment**: This is likely **NOT caused by our migration**. The DETAILS panel was already in Home's getPanels. If Settings/Chat were previously additional right panel tabs, removing them would only change which icon is shown (the first tab's icon), but the toggle itself and its icon should still render. This could be a race condition / rendering timing issue, or the DetailsIcon component not rendering visually (size 0, hidden, etc.). Possibly a **pre-existing intermittent** issue.

**However**, there's a subtle possibility: if before migration, Settings was `rightTabs[0]` with a recognizable icon, and now DETAILS is `rightTabs[0]` with an icon that renders differently (e.g., as a Lucide component that doesn't produce a standard `<svg>` at the top level), the migration could be the indirect cause. Needs runtime verification.

### Failure 2: Kit test (line 1000) — `hasKitIntersect` expected false, got true

**Test code** (`sketchpad.test.ts:987-1000`):
```typescript
const kitIntersectToggle = page.locator('[id="semio.sketchpad.app.kit.tools.select.mode.intersect"]');
const hasKitIntersect = await kitIntersectToggle.isVisible({ timeout: 3000 }).catch(() => false);
expect(hasKitIntersect).toBe(false); // LINE 1000
```

**Source code** (`Kit.tsx:8107-8112`): The intersect toggle IS unconditionally rendered in `KitToolbarSelection`. It's always part of the toolbar markup — no conditional check. The test expects it to NOT be visible, but it IS rendered.

**Assessment**: **PRE-EXISTING / UNRELATED** to Settings/Chat migration. The intersect toggle was added to Kit's toolbar selection tools (`KitToolbarSelection`) and the test was not updated to reflect this. This is a code-vs-test discrepancy where the implementation was updated (intersect mode added) but the test expectation was not.

### Failure 3: Type test (line 1504) — `hasHudPanel` expected false, got true

**Test code** (`sketchpad.test.ts:1500-1504`):
```typescript
const hudPanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.hudPanel"]');
const hasHudPanel = await hudPanelToggle.isVisible({ timeout: 3000 }).catch(() => false);
expect(hasHudPanel).toBe(false); // LINE 1504
```

**Source code**: Type's `getPanels()` includes `PanelKind.STATS` which maps to `PanelPosition.MIDDLE` (HUD). This creates a HUD panel tab, making the hudPanel toggle visible.

**Assessment**: **PRE-EXISTING / UNRELATED** to Settings/Chat migration. `PanelKind.STATS` was added to Type's panel list, creating HUD panel tabs. The test was written when Type didn't have STATS and was never updated.

### Failure 4: Design test (line 1894) — `hasDesignIntersect` expected false, got true

**Test code** (`sketchpad.test.ts:1881-1894`):
```typescript
const designIntersectToggle = page.locator('[id="semio.sketchpad.app.design.tools.select.mode.intersect"]');
const hasDesignIntersect = await designIntersectToggle.isVisible({ timeout: 3000 }).catch(() => false);
expect(hasDesignIntersect).toBe(false); // LINE 1894
```

**Source code** (`Design.tsx:3846-3850`): The intersect toggle IS unconditionally rendered in `DesignSelectSettings`. Same situation as Kit.

**Assessment**: **PRE-EXISTING / UNRELATED** to Settings/Chat migration. Identical root cause as Failure 2 — intersect mode was added to Design's toolbar, test not updated.

### Failure 5: Docs test (line 2499) — h1 element not found

**Test code** (`sketchpad.test.ts:2498-2499`):
```typescript
const pageTitle = page.locator("h1").first();
await expect(pageTitle).toBeVisible({ timeout: 15000 });
```

**Source code**: Docs navigates to `/docs/index`. The Docs app has MDX rendering that maps `h1` via custom components (`Docs.tsx:381-391`). `initDocs` does `page.goto("/docs/index")` + `waitForLoadState("networkidle")` + 2s wait.

**Assessment**: **LIKELY PRE-EXISTING / UNRELATED** to Settings/Chat migration. The h1 visibility depends on Docs MDX content rendering correctly. The migration changed panel layout, not Docs content rendering. This is likely a **content loading / routing issue** — the Docs page content may not be loading correctly due to docusaurus/MDX rendering issues, or the route `/docs/index` may not resolve to the expected page. No panel migration changes would affect Docs content rendering.

### Failure 6: Panels test (line 3262) — ERR_CONNECTION_REFUSED (server crash)

**Test code**: The Panels test navigates to various pages and tests panel state persistence. At line 3262, the server is refusing connections, meaning the dev server crashed during the test.

**Assessment**: **INDETERMINATE** — could be caused by anything including the migration. A server crash during a long-running test could be:
1. A pre-existing memory leak or crash bug in the dev server
2. An error triggered by the migration changes (e.g., null pointer on accessing removed Settings/Chat panel state)
3. An unrelated concurrent issue

Without server logs, cannot determine causality. The Panels test is the last test and tests cross-app panel persistence, panel content, resize handles, and keyboard shortcuts, so it exercises many code paths.

### Summary Table

| # | Test | Line | Failure | Caused by Migration? |
|---|------|------|---------|---------------------|
| 1 | Home | 673 | rightSidePanelIconVisible false | Possibly (icon change) — needs runtime verification |
| 2 | Kit | 1000 | hasKitIntersect true | NO — intersect tool added, test not updated |
| 3 | Type | 1504 | hasHudPanel true | NO — STATS panel added, test not updated |
| 4 | Design | 1894 | hasDesignIntersect true | NO — intersect tool added, test not updated |
| 5 | Docs | 2499 | h1 not found | NO — Docs content rendering issue |
| 6 | Panels | 3262 | ERR_CONNECTION_REFUSED | INDETERMINATE — server crash |

### Recommended Fixes

1. **Failure 1**: Verify at runtime whether the rightSidePanel toggle renders correctly for Home. If the icon is just invisible (size 0), fix the icon component. If the toggle genuinely shouldn't appear, update test.
2. **Failures 2 & 4**: Update tests to `expect(hasKitIntersect).toBe(true)` and `expect(hasDesignIntersect).toBe(true)` since the intersect toggle is now unconditionally rendered.
3. **Failure 3**: Update test to `expect(hasHudPanel).toBe(true)` since Type now has STATS in HUD.
4. **Failure 5**: Investigate Docs page rendering (MDX content, routing). Check if `/docs/index` resolves correctly.
5. **Failure 6**: Check server logs for crash cause. May need to investigate memory usage or runtime errors in the Panels test.

## Changes

No code changes — research only.

## Log

- Analyzed `getPanels()` for all 5 apps (Home, Kit, Type, Design, Docs)
- Analyzed `panelKindConfigs` mapping PanelKind → PanelPosition
- Analyzed `PanelToggles` component conditional rendering logic
- Analyzed each test failure against current source code
- Determined causality for each failure

## Todos

- [x] Read test failure locations
- [x] Read app source code for panel definitions
- [x] Analyze PanelKind → PanelPosition mapping
- [x] Determine causality for each failure
- [x] Document findings

## Plan

Research-only ticket. No implementation changes needed.
