# S Shell UI Acceptance Spec Exploration

## 1. FrameworkOsShell / ShellHost Components

**Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`

**Export:** `export function FrameworkOsShell(props: FrameworkOsShellProps)` (line 1043)

**Inner implementation:** `function FrameworkOsShellInner(...)` (line 1083)

### Top-level DOM landmarks rendered by the booted shell:

1. **Root scope container** (line 1073):
   - `<div class="semio-scope" data-shell-id={scope.shellId}>`
   - Full-height, full-width flex container with `position: relative`

2. **Main layout structure** (line 7050):
   - `<div class="flex h-screen min-h-0 w-screen flex-col bg-transparent" data-level="base">`

3. **Navbar** (line 7071):
   - Component: `<Navbar items={navbarItems} showFullscreenToggle={!mobile} />`
   - Contains logo/title, mode switcher, example selector

4. **Optional sub-navbar for tutorials** (line 7072-7095):
   - Component: `<TutorialBar ... />` (conditionally rendered if `activeTutorial` is set)

5. **Panel dock system** (line 7067, 7097):
   - Provider: `<PanelDockProvider dock={dock} ...>`
   - Panels at anchors: `top-left`, `top-right`, `bottom-left`, `bottom-middle`, `bottom-right`

6. **Layout container** (line 7068):
   - Component: `<Layout mobile={mobile} mobilePanel={mobilePanel} navbar={...} panels={{...}} canvas={...} footer={...} />`

7. **Canvas area** (line 7100-7105):
   - Fault-bounded canvas rendering the app windows via `Mode` component

8. **Footer** (line 7096):
   - Component: `<Footer items={footerItems} />`

9. **Portal layer** (line 1076):
   - `<div data-semio-portal-layer ref={setPortalLayer}>`
   - For portaled content (dialogs, introductions, context menus)

### Key selectors NOT text-content dependent:

- `[class="semio-scope"]` - Root shell container
- `[data-shell-id]` - Scope identification
- `[data-semio-portal-layer]` - Portal host
- `[data-level="base"]` - Main layout wrapper
- `[role="status"]` - Transient notices / alerts
- `[data-semio-transient-notice]` - Notice container

## 2. Readiness Beacon

**Location:** Lines 6896-6940 in `ShellHost/🟦️component.tsx`

**Dataset keys on `document.documentElement`:**
- `dataset.semioOsReady` - Set when shell reaches ready state
- `dataset.semioOsError` - Set when an error occurs
- `dataset.semioOsNotFound` - Set when app not found (host mode only)

**Dataset keys on shell's own root element** (mirrored):
- `dataset.shellReady`
- `dataset.shellError`
- `dataset.shellNotFound`

**Value for the `s` plugin specifically:**
When the `s` plugin boots successfully, the beacon value is set to the `pluginId`:
```javascript
root.dataset.semioOsReady = "s"  // for the s plugin
```

**Readiness conditions:**
- `"ready"` - Session exists and no errors
- `"error"` - `error` state is truthy
- `"not-found"` - Host mode with route kind "notFound"

## 3. What S Plugin Boots: Desktop/Workspace/Window Manager

The `s` plugin is the **semio Studio Host** - it boots a **workspace/studio management interface**, not a launcher or desktop per se.

### Declared windows (from descriptor):

**App ID:** `s.space.home@1/*#editor`

**Window kinds:**
1. **`s-home-main`** - "Studios" table
   - Surface kind: `table`
   - Icon: `table-2`
   - Engagement: `none` (read-only)

### Commands reachable without opening a document:

- `createStudio` - Create a new studio
- `openSpace` - Open an existing studio
- `importSpace` - Import studio from file
- `bindSpaceFile` - Bind to a studio file
- `navigateVirtualFileSystemNode` - Navigate file system
- `deleteVirtualFileSystemNode` - Delete file system node
- `goHome` - Navigate home
- `undo` / `redo` - Undo/redo commands
- `commitCheckpoint` / `checkoutCheckpoint` - Checkpoint management
- `setClient` / `presenceHeartbeat` - Presence/identity
- `noteShellCommand` - Record shell commands
- `recordTutorial` / `setHistoryCommandFilter` - Tutorial recording

### Panel tabs:
- `framework.panel.history` - History panel

## 4. Concrete, Deterministic Interactions for Testing

### Interaction 1: Verify navbar renders with app label

**Selector:** `[data-slot="app-name"]` (line 6329)

**Test:** 
- Navigate to story `🛠️framework🖥️os-plugins--S`
- Assert element is visible
- Assert it contains text matching "s" or "S Studio"

**Post-condition:** Navbar displays active app identifier

### Interaction 2: Trigger command palette with keyboard shortcut

**Selector:** None directly (keyboard-driven)

**Test:**
- After shell boots, press `Ctrl/Cmd+K` (command palette shortcut)
- Assert `[role="dialog"]` or search input becomes visible

**Post-condition:** Command palette opens, showing available commands

### Interaction 3: Click example selector (if examples exist)

**Selector:** `#playground.navbar.fixture` (line 5683)

**Test:**
- Navigate to story with fixtures/examples available
- Click on the example selector dropdown
- Verify it renders with options

**Post-condition:** Example list is visible and selectable

### Interaction 4: Toggle mobile panel visibility

**Selector:** `#ui.mobilePanel.toggle` (line 6349)

**Test:** (Mobile viewport only)
- Resize to mobile breakpoint
- Click panel toggle button
- Assert panel visibility state changes in DOM

**Post-condition:** Mobile panel appears/disappears

### Interaction 5: Right-click to open shell context menu

**Selector:** None (contextmenu event)

**Test:**
- Right-click on empty canvas area
- Assert context menu appears with undo/redo/command palette options
- Verify it has proper menu structure (`[role="menu"]`, menu items)

**Post-condition:** Fallback context menu shows available actions

## 5. Existing Stories for S/Space Plugin

**Story location:** `.storybook/stories/framework/os/plugins.stories.tsx`

**Story export** (line 52):
```typescript
export const S: Story = { args: { plugin: "s" } satisfies OsBootHostProps };
```

**Story ID:** `🛠️framework🖥️os-plugins--S`

**Story path for iframe testing:**
```
iframe.html?id=🛠️framework🖥️os-plugins--S&viewMode=story
```

**Meta title:** `"🛠️framework🖥️os/Plugins"` (line 13)

This story uses the `OsBootHost` component from `.storybook/framework/os/index.tsx` and automatically probes for the plugin's WASM artifact at `/plugin-modules/s/*.wasm`.

## 6. Storybook Suite Invocation & SEMIO_TEST_LEVEL Convention

### Package.json scripts:

- **Dev:** `bun run dev:storybook:framework:os`
- **Build:** `bun run build:storybook`
- **Test:** `bun run test:storybook`

### Nx targets:

- `workspace:dev-storybook-framework-os` - Dev server
- `workspace:test-storybook` - Run Playwright specs

### SEMIO_TEST_LEVEL Environment Variable:

**Location:** `nx.json` declares it as a build input

**Levels** (from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts`, line 1115-1126):

```typescript
export const TEST_LEVELS = ["fundamental", "quick", "long", "exhaustive"] as const;

export const TEST_LEVEL_BUDGET_MS: Record<TestLevel, number> = {
  fundamental: 15_000,   // Default if unset
  quick: 30_000,
  long: 300_000,
  exhaustive: 900_000,
};
```

**Convention:**
- Tests inherit the level from the environment variable
- Child processes (vitest, cargo, etc.) inherit it automatically
- If not set, defaults to `"fundamental"`
- Exhaustive level auto-enables coverage (`SEMIO_COVERAGE=1`)

**Usage:**
```bash
SEMIO_TEST_LEVEL=fundamental bun run test:storybook
SEMIO_TEST_LEVEL=quick bun run test:storybook
SEMIO_TEST_LEVEL=long bun run test:storybook
```

### Reference test patterns:

See `.storybook/os-plugins.spec.ts` (per-plugin boot matrix):
- Uses `pluginArtifactAvailable()` to HEAD-probe WASM artifact
- Waits for readiness beacon: `document.documentElement.dataset.semioOsReady === pluginId`
- Asserts zero significant console errors
- Timeout: 60 seconds

See `.storybook/puzzle-2d.spec.ts` (interaction model):
- Navigates to `iframe.html?id=<storyId>&viewMode=story`
- Reads debug output from `[data-testid="puzzle2d-board-debug"]`
- Uses `worldToClientPoint()` transforms for deterministic clicks
- Polls state with `expect.poll()`

