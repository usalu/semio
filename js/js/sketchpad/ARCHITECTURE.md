# Sketchpad Architecture

## Open/Closed Principle Implementation

The Sketchpad follows the **Open/Closed Principle**: the system is **closed for modification** but **open for extension**.

### Problem (Before)

Adding a new editor (e.g., a new type of content editor) required modifying multiple core files:

1. **Navbar.tsx** - Add panel configuration to `getPanelConfigs()`
2. **Sketchpad.tsx** - Add hardcoded `<Route>` components
3. **store.tsx** - Add to `EditorType` enum

This created:
- **Merge conflicts** when multiple developers added editors
- **Coupling** between editors and core system
- **Violation** of Open/Closed Principle
- **Risk** of breaking existing editors when adding new ones

### Solution (After)

Implemented a **Registry Pattern** where editors self-register with the system.

#### Registry Pattern

```
┌─────────────────────────────────────┐
│       Editor Registry               │
│  (Central registration system)      │
└─────────────────────────────────────┘
         ▲         ▲         ▲
         │         │         │
    ┌────┴───┐ ┌──┴────┐ ┌──┴────┐
    │ Home   │ │ Kit   │ │Design │
    │ Editor │ │Editor │ │Editor │
    └────────┘ └───────┘ └───────┘
    Self-      Self-     Self-
    registers  registers registers
```

Core components read from the registry:
- **Navbar** - Reads panel configs from registry
- **Sketchpad** - Generates routes from registry
- **Store** - EditorType enum remains for compatibility

### Architecture Components

#### 1. Editor Registry (`editors/registry.tsx`)

Central registration system:
- Stores editor metadata (routes, panels, components)
- Provides lookup by editor ID or URL path
- Generates route and panel configurations

#### 2. Editor Registration (`editors/*/registration.tsx`)

Each editor has a registration file that self-registers:
```tsx
editorRegistry.register({
  id: "design",
  component: DesignEditor,
  routeSegments: [...],
  getPanels: (t) => [...],
  matchesPath: (pathParts) => ...,
  order: 20,
});
```

#### 3. Auto-Import (`editors/index.tsx`)

Imports all registrations to trigger side effects:
```tsx
import "./home/registration";
import "./kit/registration";
import "./design/registration";
// Add new editors here
```

#### 4. Dynamic Route Generation (`Sketchpad.tsx`)

`RouteGenerator` component reads registry and generates routes:
```tsx
<Routes>
  <Route element={<SketchpadBase />}>
    <RouteGenerator />  {/* Dynamically generates all routes */}
  </Route>
</Routes>
```

#### 5. Dynamic Panel Configuration (`Navbar.tsx`)

Panel configs read from registry:
```tsx
const getPanelConfigs = (t) => editorRegistry.getPanelConfigs(t);
```

### Adding a New Editor

**Before (3+ files to modify):**
```
✗ Modify Navbar.tsx (getPanelConfigs)
✗ Modify Sketchpad.tsx (add routes)
✗ Modify store.tsx (EditorType enum)
```

**After (1 line to add):**
```
✓ Create editor directory
✓ Create registration.tsx
✓ Add import to editors/index.tsx
```

### Benefits

1. **Decoupled** - Editors don't depend on core system
2. **Extensible** - Add editors without modifying core
3. **Maintainable** - Each editor is self-contained
4. **Scalable** - No merge conflicts when adding editors
5. **Flexible** - Easy to enable/disable editors
6. **Testable** - Can test editors in isolation

### File Organization

```
sketchpad/
├── editors/
│   ├── registry.tsx          # Central registry
│   ├── index.tsx             # Auto-imports registrations
│   ├── README.md             # Developer guide
│   ├── home/
│   │   ├── Editor.tsx        # Editor component
│   │   ├── registration.tsx  # Self-registration
│   │   ├── store.tsx         # Editor state
│   │   └── commands.ts       # Editor commands
│   ├── kit/
│   ├── design/
│   ├── type/
│   └── quality/
├── Navbar.tsx                # Reads from registry
├── Sketchpad.tsx             # Generates routes from registry
├── store.tsx                 # Core store (EditorType enum)
└── ARCHITECTURE.md           # This file
```

### Migration Path

The refactoring maintains backward compatibility:

1. **EditorType enum** still exists in `store.tsx`
2. Editor IDs match enum values (lowercase)
3. `getPanelConfigs()` function signature unchanged
4. Panel and route behavior identical to before

This allows gradual adoption and ensures no breaking changes.

### Design Patterns Used

1. **Registry Pattern** - Central registration of editors
2. **Factory Pattern** - Dynamic route generation
3. **Dependency Inversion** - Core depends on abstractions (registry), not concrete editors
4. **Open/Closed Principle** - Open for extension (new editors), closed for modification (core)

### Future Extensions

This architecture enables:

- **Plugin system** - Load editors dynamically at runtime
- **Lazy loading** - Load editor code on demand
- **Editor marketplace** - Third-party editors
- **A/B testing** - Enable/disable editors per user
- **Feature flags** - Control editor availability

### Key Takeaway

**Adding a new editor is now as simple as:**
1. Create editor directory with `Editor.tsx` and `registration.tsx`
2. Add one import line to `editors/index.tsx`
3. Done! No core files modified.
