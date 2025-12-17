---
slug: UI-ID-SYSTEM-ANALYSIS
summary: ""
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.713Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# UI ID System Analysis

**Date:** 2025-11-24  
**Status:** Analysis Complete

## Overview

The Semio Sketchpad UI system uses a comprehensive ID-based architecture where every interactive UI component has a unique identifier following the pattern `semio.sketchpad.*`. This ID serves as the central integration point for multiple subsystems.

## ID Pattern

### Convention

```
semio.sketchpad.<context>.<feature>.<component>
```

**Examples:**

- `semio.sketchpad.navbar.panelToggle.workbench`
- `semio.sketchpad.app.quality.panel.details.key`
- `semio.sketchpad.app.kit.kitApp.createType`
- `semio.sketchpad.app.home.createTemporary`

### Rules

1. **All IDs start with `semio.sketchpad.`**
2. **Kebab-case** for multi-word segments
3. **Hierarchical structure** reflecting UI containment
4. **Only the final DOM element** receives the `id` attribute

## Integration Points

The ID system integrates with **7 major subsystems**:

### 1. Internationalization (i18n)

**Location:** `js/js/sketchpad/locales/{lang}.json`

**Mechanism:**

- Every ID has automatic i18n key resolution
- Standard suffixes define content types:
  - `.label` - Main display text
  - `.beginner` - Beginner-friendly description
  - `.manual` - Path to manual page
  - `.tutorial` - Path to tutorial
  - `.hotkey` - Keyboard shortcut display

**Example:**

```json
{
  "semio.sketchpad.navbar.panelToggle.workbench": {
    "label": "Workbench",
    "beginner": "Open the workbench panel to see available types",
    "manual": "panels/workbench",
    "hotkey": "Ctrl+J"
  }
}
```

**Validation:**

- Script: `scripts/i18n.ts`
- Validates all IDs have translations
- Checks for missing/unused keys
- Generates report: `reports/i18n.md`

**Components Using i18n:**

- `useLabel(id)` - Hook to get translated label
- `DescriptionTooltipContent` - Auto-resolves content from ID
- All input components (`Input`, `Textarea`, `Select`, etc.) with `showLabel` prop

### 2. Tooltips

**Location:** `js/js/sketchpad/elements.tsx`

**Components:**

- `DescriptionTooltipContent` - Automatic tooltip from ID
- `IdSemioTooltip` - Wrapper providing ID-based tooltip
- `EnhancedTooltipContent` - Manual tooltip configuration

**Mechanism:**

```tsx
function DescriptionTooltipContent({ id }: { id: string }) {
  const { t } = useTranslation();
  const mode = useTooltipMode();

  // Resolves:
  // - Label from `${id}.label` or `${id}.beginner`
  // - Manual link from `${id}.manual`
  // - Tutorial link from `${id}.tutorial`
  // - Hotkey from `${id}.hotkey`
}
```

**Expertise Adaptation:**

- `EXPERT` - No tooltips shown
- `NORMAL` - Standard labels, manual links, hotkeys
- `BEGINNER` - Beginner descriptions, manual links, tutorial links, hotkeys

**Usage in Components:**

```tsx
// Automatic via wrapper
<Input id="semio.sketchpad.app.quality.name" showLabel />

// Manual tooltip
<Tooltip>
  <TooltipTrigger>...</TooltipTrigger>
  <TooltipContent>
    <DescriptionTooltipContent id="semio.sketchpad.navbar.back" />
  </TooltipContent>
</Tooltip>
```

### 3. Hotkeys

**Location:** `js/js/sketchpad/App.tsx` (SketchpadStore)

**Mechanism:**

- Hotkeys stored in `hotkeyOverrides: Map<HotkeyPath, HotkeyValue>`
- Path = UI element ID
- Value = `react-hotkeys-hook` format (`ctrl+k`, `mod+j`, etc.)

**Integration:**

```tsx
// Register hotkey
useHotkeys("ctrl+j", () => togglePanel("workbench"));

// Display in tooltip
// Automatically shows hotkey from i18n `${id}.hotkey`
```

**Hotkey Override System:**

- Users can customize hotkeys
- Stored in SketchpadStore
- Persisted with Y.js
- Accessible via settings UI

**Hotkey Navigation:**

```tsx
// Click hotkey in tooltip to navigate to settings
const handleHotkeyClick = () => {
  window.dispatchEvent(
    new CustomEvent("navigate-to-hotkey", {
      detail: { path: id },
    }),
  );
};
```

### 4. Command System (Origin Tracking)

**Location:** `js/js/sketchpad/App.tsx` (AppStore, KitDiffAppStore)

**Mechanism:**

- Every command execution accepts an `origin` parameter
- Origin = ID of the UI element that triggered the command
- Used for logging, debugging, and undo/redo context

**Pattern:**

```tsx
// Command execution with origin
executeCommand("semio.kitApp.addType",
  "semio.sketchpad.app.kit.toolbar.addType", // origin
  typeData
);

// Origin extraction in executeCommand
async executeCommand<T>(command: string, ...rest: any[]): Promise<T> {
  let origin: string | undefined;

  // First arg is origin if it's a semio.sketchpad.* string
  if (rest.length > 0 &&
      typeof rest[0] === "string" &&
      rest[0].startsWith("semio.sketchpad.")) {
    origin = rest[0];
    rest = rest.slice(1);
  }

  // Execute command with context
  const result = callback(context, ...rest);

  // Log with origin for debugging
  console.log(`[${origin || "unknown"}] ${command}`, result);

  return result;
}
```

**Usage in Components:**

```tsx
// Button triggers command with origin
<Button
  id="semio.sketchpad.app.kit.createType"
  onClick={() =>
    executeCommand(
      "semio.kitApp.addType",
      "semio.sketchpad.app.kit.createType", // origin matches id
      newTypeData,
    )
  }
>
  Create Type
</Button>
```

### 5. Tutorial Recording

**Location:** `js/js/sketchpad/tutorials/`

**Mechanism:**

- Records user interactions for tutorial playback
- Uses command origins to identify which UI elements were used
- Stores sequence of commands with origins and parameters

**Recording Structure:**

```typescript
interface TutorialRecordingEvent {
  timestamp: number;
  command: string;
  origin: string; // UI element ID
  parameters: any[];
}

interface TutorialRecording {
  events: TutorialRecordingEvent[];
  milestones: TutorialMilestone[];
}
```

**Playback:**

- Highlights UI elements based on `origin` field
- Shows tooltips explaining what to do
- Validates user actions match expected origins

**Tutorial Overlay:**

```tsx
// Highlights element during tutorial
<TutorialOverlay highlightedElementId={currentEvent.origin} description={t(`${currentEvent.origin}.beginner`)} />
```

**Command Interception:**

```tsx
const useTutorialCommandInterceptor = () => {
  // Intercept executeCommand calls
  // Record origin and parameters
  // Validate against tutorial sequence
};
```

### 6. Testing (E2E)

**Location:** `js/js/e2e/**/*.spec.ts`

**Mechanism:**

- Playwright tests select elements by ID
- IDs provide stable selectors that don't break with style changes
- Test readability improved with semantic IDs

**Example:**

```typescript
test("drag type from workbench to canvas", async ({ page }) => {
  // Create temporary kit
  await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();

  // Create type
  await page.locator('[id="semio.sketchpad.app.kit.kitApp.createType"]').click();

  // Navigate back
  await page.locator('[id="semio.sketchpad.navbar.back"]').click();

  // Create design
  await page.locator('[id="semio.sketchpad.app.kit.kitApp.createDesign"]').click();

  // Toggle workbench
  await page.locator('[id="semio.sketchpad.navbar.panelToggle.workbench"]').click();

  // ... drag and drop test
});
```

**Benefits:**

- **Stable selectors** - IDs don't change with styling
- **Semantic** - Test reads like documentation
- **Debuggable** - Clear which UI element failed
- **Refactor-safe** - CSS changes don't break tests

### 7. Analytics & Logging

**Mechanism:**

- Command origins provide analytics data
- Track which features are used
- Identify UI/UX issues

**Logging Pattern:**

```typescript
// All commands logged with origin
console.log(`[DEBUG] [${origin}] Command: ${command}`, parameters);

// Analytics event
analytics.track("command_executed", {
  command,
  origin,
  timestamp: Date.now(),
});
```

## Component Integration Patterns

### Input Components

All input components support ID-based integration:

```tsx
interface BaseInputProps {
  id: string; // Required ID for i18n/hotkey/tooltip
  showLabel?: boolean; // Auto-show label from i18n
  // ... other props
}

// Usage
<Input
  id="semio.sketchpad.app.quality.name"
  value={quality.name}
  onChange={handleChange}
  showLabel // Shows translated label
/>;
```

**Supported Components:**

- `Input`
- `Textarea`
- `Select`
- `Combobox`
- `Toggle`
- `Slider`
- `Stepper`
- `Button`
- `Action`

### Button/Action Pattern

```tsx
<Button
  id="semio.sketchpad.navbar.back"
  onClick={() =>
    executeCommand(
      "semio.sketchpad.navigate",
      "semio.sketchpad.navbar.back", // origin = id
      -1,
    )
  }
>
  {/* Icon/label auto-resolved from i18n */}
</Button>
```

### Panel Toggle Pattern

```tsx
<Action id={`semio.sketchpad.navbar.panelToggle.${panelKey}`} onClick={() => executeCommand("semio.sketchpad.togglePanel", `semio.sketchpad.navbar.panelToggle.${panelKey}`, panelKey)} pressed={isPanelVisible} />
```

## Current Implementation Status

### ✅ Fully Implemented

1. **i18n System**
   - Translation files: `js/js/sketchpad/locales/{en,de}.json`
   - Validation script: `scripts/i18n.ts`
   - Hook: `useLabel(id)`
   - Auto-resolution in components

2. **Tooltip System**
   - `DescriptionTooltipContent` component
   - Expertise-aware display
   - Manual/tutorial links
   - Hotkey display

3. **Command Origin Tracking**
   - Origin extraction in `executeCommand`
   - Logging with origin
   - Undo/redo with context

4. **E2E Testing**
   - ID-based selectors
   - Stable test suite
   - Example: `js/js/e2e/kit/design/drag-and-drop.spec.ts`

### ⚠️ Partially Implemented

1. **Tutorial Recording**
   - Infrastructure exists: `js/js/sketchpad/tutorials/`
   - Recording events with origins
   - Playback mechanism
   - ⚠️ Not all apps integrate recording yet

2. **Hotkey System**
   - Override storage exists
   - Display in tooltips works
   - ⚠️ Settings UI for customization incomplete

3. **Analytics**
   - Origin logging works
   - ⚠️ No centralized analytics service yet

### ❌ Not Implemented

1. **Tutorial Highlight System**
   - Infrastructure exists
   - ❌ Visual highlighting not fully implemented

2. **Comprehensive Testing**
   - ❌ Most apps lack E2E tests
   - Only kit/design have examples

## Documentation Status

### ✅ Documented in AGENTS.md

1. ID convention for commands
2. Tooltip integration
3. i18n integration
4. Tutorial system architecture

### ❌ Missing Documentation

1. **Complete ID naming conventions**
   - When to use which context
   - How to structure nested IDs
   - Examples for each app type

2. **Component authoring guide**
   - How to add ID to new components
   - When to use `showLabel`
   - Tooltip best practices

3. **Testing guide**
   - How to write ID-based tests
   - Selector patterns
   - Common test scenarios

4. **i18n guide**
   - Translation file structure
   - When to add new keys
   - How to run validation

## Inconsistencies Found

### 1. Incomplete ID Coverage

**Issue:** Not all interactive elements have IDs

**Examples:**

- Some buttons in `Navbar` lack IDs
- Dynamically generated items may not have IDs
- Internal sub-components often missing IDs

**Impact:**

- Can't test these elements
- No tooltips available
- Analytics incomplete

### 2. Inconsistent Origin Passing

**Issue:** Commands called without origin parameter

**Pattern Found:**

```tsx
// ❌ Bad - no origin
executeCommand("semio.kitApp.addType", typeData);

// ✅ Good - with origin
executeCommand("semio.kitApp.addType", "semio.sketchpad.app.kit.toolbar.addType", typeData);
```

**Impact:**

- Can't trace which UI triggered command
- Recording incomplete
- Debugging harder

### 3. Label vs ID Mismatch

**Issue:** Component has `id` but doesn't use i18n

**Pattern Found:**

```tsx
// ❌ Bad - hardcoded label
<Button id="semio.sketchpad.navbar.back">
  Back
</Button>

// ✅ Good - i18n label
<Button id="semio.sketchpad.navbar.back">
  {useLabel("semio.sketchpad.navbar.back")}
</Button>

// ✅ Better - auto-label
<Button id="semio.sketchpad.navbar.back" showLabel />
```

### 4. Tooltip Inconsistency

**Issue:** Mixed use of manual tooltips vs ID-based tooltips

**Found:**

- Some components use `tooltip="..."` prop (string)
- Others use `DescriptionTooltipContent` (ID-based)
- No clear pattern when to use which

**Recommendation:**

- Deprecate `tooltip` string prop
- Always use ID-based tooltips
- Allow override for special cases

### 5. Missing i18n Entries

**Issue:** IDs used in code but missing in `locales/*.json`

**Detection:** Run `tsx scripts/i18n.ts` to generate report

**Common Missing:**

- `.beginner` descriptions
- `.manual` paths
- `.tutorial` paths

## Recommendations

### 1. Enforce ID Requirement

**Rule:** Every interactive component MUST have an `id` prop

**Implementation:**

- TypeScript: Make `id` required in component props
- Lint rule: Warn on missing `id` for interactive components
- Code review: Check for ID in new components

```tsx
// Before
interface ButtonProps {
  id?: string;
  // ...
}

// After
interface ButtonProps {
  id: string; // Required
  // ...
}
```

### 2. Standardize Origin Passing

**Rule:** All `executeCommand` calls MUST include origin as first argument

**Pattern:**

```tsx
// Standard pattern
<Button
  id="semio.sketchpad.app.kit.createType"
  onClick={() => {
    const origin = "semio.sketchpad.app.kit.createType";
    executeCommand("semio.kitApp.addType", origin, typeData);
  }}
/>;

// Helper hook
const useCommandExecutor = (id: string) => {
  return (command: string, ...args: any[]) => {
    executeCommand(command, id, ...args);
  };
};

// Usage
const execute = useCommandExecutor("semio.sketchpad.app.kit.createType");
<Button onClick={() => execute("semio.kitApp.addType", typeData)} />;
```

### 3. Auto-Generate i18n Skeleton

**Tool:** Extend `scripts/i18n.ts` to auto-generate missing entries

```typescript
// Auto-generate skeleton
{
  "semio.sketchpad.navbar.back": {
    "label": "[TODO: Translate] Back",
    "beginner": "[TODO: Add beginner description]",
    "manual": "",  // Optional
    "tutorial": "",  // Optional
    "hotkey": ""  // Optional
  }
}
```

### 4. Component Template

**Create:** Standard component template with ID integration

```tsx
interface MyComponentProps {
  id: string; // Required
  // ... other props
}

export function MyComponent({ id, ...props }: MyComponentProps) {
  const label = useLabel(id);
  const execute = useCommandExecutor(id);

  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button id={id} onClick={() => execute("my.command", data)}>
          {label}
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <DescriptionTooltipContent id={id} />
      </TooltipContent>
    </Tooltip>
  );
}
```

### 5. Documentation Update

**Add to AGENTS.md:**

#### Section: "UI Component ID System"

1. **ID Convention**
   - Pattern: `semio.sketchpad.<context>.<feature>.<component>`
   - Examples for each app type
   - Naming guidelines

2. **Integration Points**
   - i18n (with examples)
   - Tooltips (with examples)
   - Hotkeys (with examples)
   - Command origins (with examples)
   - Testing (with examples)
   - Recording (with examples)

3. **Component Authoring**
   - Required: ID prop
   - Required: Origin in commands
   - Optional: showLabel for auto-label
   - Optional: Tooltip override

4. **i18n Management**
   - File structure
   - Key suffixes (.label, .beginner, etc.)
   - Validation script
   - Adding new keys

5. **Testing Guide**
   - Selector pattern: `[id="semio.sketchpad.X"]`
   - Example test structure
   - Common scenarios

### 6. Migration Plan

**Phase 1:** Add IDs to all components

- Audit all interactive components
- Add missing IDs following convention
- Generate i18n skeleton entries

**Phase 2:** Standardize origin passing

- Add origin to all `executeCommand` calls
- Create `useCommandExecutor` hook
- Refactor existing calls

**Phase 3:** Complete i18n

- Translate all `[TODO]` entries
- Add beginner descriptions
- Add manual/tutorial paths

**Phase 4:** Testing coverage

- Write E2E tests for each app
- Use ID-based selectors
- Cover major workflows

**Phase 5:** Tutorial system

- Record tutorials for key workflows
- Test playback system
- Add tutorial links to tooltips

## Conclusion

The Semio Sketchpad UI ID system is a well-architected solution for integrating multiple subsystems (i18n, tooltips, hotkeys, commands, recording, testing, analytics) through a single identifier.

**Strengths:**

- Centralized integration point
- Consistent pattern across codebase
- Enables powerful features (recording, testing, i18n)

**Weaknesses:**

- Incomplete implementation (not all components have IDs)
- Inconsistent usage (some commands lack origins)
- Under-documented (missing authoring guide)

**Next Steps:**

1. Document the system in AGENTS.md (this analysis)
2. Create component authoring template
3. Add lint rules for ID requirement
4. Complete i18n coverage
5. Expand E2E test coverage
