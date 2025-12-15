---
slug: SETTINGS_PANELS
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Implement settings panel hierarchy
model: claude-opus-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---

# Settings Panel Hierarchy

## App Hierarchy

The Semio Sketchpad application has a hierarchical structure where child apps inherit and extend functionality from their parent apps:

```
Sketchpad (root)
├── Home
└── Kit
    ├── Design
    └── Type
```

### App Levels:

- **Sketchpad**: Root application container providing global settings (theme, language, layout, expertise, mode)
- **Home**: Displays list of kits and kit management
- **Kit**: Shows types and designs within a kit
- **Design**: Specific design editor for creating and editing designs
- **Type**: Specific type editor for creating and editing types

## Panel System

Panels from the same kind have different sections ordered from most specific (top) to least specific (bottom).

### Specificity Levels:

- **30**: App-specific settings (Design, Type)
- **20**: App-specific settings (Home)
- **10**: Kit-level settings
- **0**: Global Sketchpad settings

### Settings Panel Sections by App:

#### Home App

1. **Home Settings** (specificity: 20)
   - Theme (System, Light, Dark)
   - Language (English, German)
   - Layout (Desktop, Tablet)
   - Expertise (Beginner, Normal, Expert)
   - Mode (User, Dev)

2. **Sketchpad Settings** (specificity: 0)
   - Same global settings as above

#### Kit App

1. **Kit Settings** (specificity: 10)
   - Theme, Language, Layout, Expertise, Mode

2. **Sketchpad Settings** (specificity: 0)
   - Global settings

#### Design App

1. **Design Settings** (specificity: 30)
   - Proximity Connect Distance
   - Grid Size

2. **Kit Settings** (specificity: 10)
   - Theme, Language, Layout, Expertise, Mode

3. **Sketchpad Settings** (specificity: 0)
   - Global settings

#### Type App

1. **Type Settings** (specificity: 30)
   - (Currently placeholder, can be extended with type-specific settings)

2. **Kit Settings** (specificity: 10)
   - Theme, Language, Layout, Expertise, Mode

3. **Sketchpad Settings** (specificity: 0)
   - Global settings

## Implementation Details

### Panel Section Structure

Each panel section has the following properties:

- `id`: Unique identifier (e.g., "semio.sketchpad.app.home.settings")
- `specificity`: Number indicating hierarchy level (higher = more specific)
- `order`: Number for ordering within same specificity level
- `content`: React component or function returning the section content

### Sorting Logic

Sections are sorted by:

1. **Specificity** (descending): Higher specificity appears first
2. **Order** (ascending): Lower order appears first within same specificity

Example from `Sketchpad.tsx`:

```typescript
const addSection = useCallback((panelKey: PanelKey, section: PanelSection) => {
  setSections((prev) => {
    const updated = {
      ...prev,
      [panelKey]: [...prev[panelKey].filter((s) => s.id !== section.id), section].sort((a, b) => {
        const specificityDiff = b.specificity - a.specificity; // Higher specificity first
        if (specificityDiff !== 0) return specificityDiff;
        return (a.order || 0) - (b.order || 0); // Lower order first
      }),
    };
    return updated;
  });
}, []);
```

### Adding Settings Sections

To add settings sections in your app component:

```typescript
useEffect(() => {
  if (appType !== "your-app") return;

  // Add app-specific settings (most specific)
  addSection("settings", {
    id: "semio.sketchpad.app.yourapp.settings",
    specificity: 30,  // Adjust based on app level
    order: 0,
    content: () => <YourAppSettings />,
  });

  // Add inherited settings (less specific)
  addSection("settings", {
    id: "semio.sketchpad.settings",
    specificity: 0,
    order: 0,
    content: () => <SketchpadSettings />,
  });

  return () => {
    removeSection("settings", "semio.sketchpad.app.yourapp.settings");
    removeSection("settings", "semio.sketchpad.settings");
  };
}, [appType, addSection, removeSection]);
```

## Testing

The settings panel hierarchy is tested using Playwright end-to-end tests in `sketchpad.test.ts`. The tests verify:

1. Each app shows the correct settings sections
2. Sections appear in the correct order (most specific to least specific)
3. All apps have access to global Sketchpad settings

Example test structure:

```typescript
test("Home app shows correct settings sections in order", async ({ page }) => {
  await page.goto("/");
  await openSettingsPanel(page);

  const sections = await getSettingsSections(page);

  // Verify sections exist
  expect(sections).toContain("semio.sketchpad.app.home.settings");
  expect(sections).toContain("semio.sketchpad.settings");

  // Verify order
  const homeIndex = sections.indexOf("semio.sketchpad.app.home.settings");
  const sketchpadIndex = sections.indexOf("semio.sketchpad.settings");
  expect(homeIndex).toBeLessThan(sketchpadIndex);
});
```

## Files Modified

- `js/js/sketchpad/Home.tsx`: Added Home and Sketchpad settings sections
- `js/js/sketchpad/Kit.tsx`: Added Kit and Sketchpad settings sections
- `js/js/sketchpad/Design.tsx`: Added Design, Kit, and Sketchpad settings sections
- `js/js/sketchpad/Type.tsx`: Added Type, Kit, and Sketchpad settings sections
- `js/js/sketchpad.test.ts`: Added comprehensive tests for settings panel hierarchy
