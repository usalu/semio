# Elements - Generalized UI Components

This folder contains reusable, domain-agnostic UI components that are completely decoupled from Sketchpad-specific logic.

## Core Components

### Layout Components

#### `Navbar`
Top navigation bar with left, center, and right item groups.

```tsx
<Navbar
  leftItems={[{ id: "logo", content: <Logo />, order: 0 }]}
  centerItems={[{ id: "title", content: <Title />, order: 0 }]}
  rightItems={[{ id: "user", content: <UserMenu />, order: 0 }]}
  height={48}
/>
```

#### `Footer`
Bottom status bar with ordered items and optional tooltips.

```tsx
<Footer
  items={[
    { id: "status", content: "Ready", tooltip: "Application status", order: 0 },
    { id: "info", content: "Info", order: 1 }
  ]}
  height={20}
/>
```

#### `Canvas`
Container for windows with fullscreen support.

```tsx
<Canvas>
  <HorizontalWindows
    windows={[
      { id: "main", children: <MainView /> },
      { id: "side", children: <SideView /> }
    ]}
  />
</Canvas>
```

#### `Layout`
Complete application layout combining all major components.

```tsx
<Layout
  navbar={<Navbar {...navbarProps} />}
  footer={<Footer {...footerProps} />}
  leftPanel={{ visible: true, size: 250, sections: [] }}
  rightPanel={{ visible: true, size: 300, sections: [] }}
  bottomPanel={{ visible: false, size: 200, sections: [] }}
  canvas={<Canvas>...</Canvas>}
/>
```

### Panel Components

#### `Panel` (Base)
Generic resizable panel with sections support.

```tsx
<Panel
  visible={true}
  size={250}
  resizeSide="right"
  sections={[
    {
      id: "section1",
      label: "Section 1",
      content: <Content />,
      defaultOpen: true,
      order: 0
    }
  ]}
  emptyMessage="No content"
  onSizeChange={(size) => console.log(size)}
/>
```

#### `LeftPanel`, `RightPanel`, `MiddlePanel`, `BottomPanel`
Specialized panel components with predefined resize directions.

```tsx
<LeftPanel visible={true} size={250} sections={[...]} />
<RightPanel visible={true} size={300} sections={[...]} />
<MiddlePanel visible={true} size={200} resizeSide="left" sections={[...]} />
<BottomPanel visible={true} size={150} sections={[...]} />
```

#### `PanelGroup`
Container for organizing multiple panels.

```tsx
<PanelGroup position="left">
  <Panel {...panelProps1} />
  <Panel {...panelProps2} />
</PanelGroup>
```

### Data Components

#### `Table`
Generic data table with customizable columns.

```tsx
<Table
  columns={[
    {
      id: "name",
      header: "Name",
      accessor: (row) => row.name,
      width: "40%"
    },
    {
      id: "value",
      header: "Value",
      accessor: (row) => row.value
    }
  ]}
  data={dataArray}
  onRowClick={(row, index) => console.log(row)}
  emptyMessage="No data available"
/>
```

#### `Scene`
3D scene viewer component (reuses existing Scene component).

```tsx
<Scene {...sceneProps} />
```

### Window Components

#### `Window`
Individual window container.

```tsx
<Window
  id="main-window"
  onDoubleClick={() => toggleFullscreen("main-window")}
>
  <YourContent />
</Window>
```

#### `HorizontalWindows`
Horizontal split layout for multiple windows.

```tsx
<HorizontalWindows
  windows={[
    { id: "left", children: <LeftContent />, defaultSize: 50 },
    { id: "right", children: <RightContent />, defaultSize: 50 }
  ]}
  handleClassName="border-r"
/>
```

#### `VerticalWindows`
Vertical split layout for multiple windows.

```tsx
<VerticalWindows
  windows={[
    { id: "top", children: <TopContent />, defaultSize: 60 },
    { id: "bottom", children: <BottomContent />, defaultSize: 40 }
  ]}
  handleClassName="border-b"
/>
```

## Key Features

### Decoupled Design
- No dependencies on Sketchpad-specific logic
- Pure presentation components
- Domain-agnostic interfaces

### Flexible Composition
- Components can be combined in various ways
- Support for nested layouts
- Responsive to container sizes

### Customizable
- All styling through Tailwind CSS classes
- Configurable sizes, colors, and behaviors
- Optional callbacks for user interactions

### Type-Safe
- Full TypeScript support
- Exported types for all props
- Generic types for data components

## Usage in Sketchpad

The Sketchpad application uses these components as building blocks:

```tsx
// js/js/sketchpad/Sketchpad.tsx
import { Layout, Navbar, Footer, Canvas, LeftPanel, RightPanel } from "../elements";

const Sketchpad = () => {
  // Sketchpad-specific state and logic
  
  return (
    <Layout
      navbar={<Navbar {...buildNavbarItems()} />}
      footer={<Footer {...buildFooterItems()} />}
      leftPanel={{
        visible: panels.workbench.visible,
        size: panels.workbench.size,
        sections: buildWorkbenchSections(),
        onSizeChange: (size) => setPanelSize("workbench", size)
      }}
      rightPanel={{
        visible: panels.details.visible,
        size: panels.details.size,
        sections: buildDetailsSections(),
        onSizeChange: (size) => setPanelSize("details", size)
      }}
      canvas={<Canvas>{renderEditor()}</Canvas>}
    />
  );
};
```

## Styling

All components use semantic color variables from `globals.css`:

- `bg-base` - Base background
- `bg-panel` - Panel background (darker than base)
- `bg-temporary` - Temporary/overlay background (darkest)
- `border` - Standard borders
- `border-accent` - Accent/active borders
- `text-foreground` - Primary text color
- `text-muted-foreground` - Secondary text color

Components support dark mode automatically through CSS variables.

## Best Practices

1. **Always use semantic colors** - Never hardcode colors
2. **Provide unique IDs** - All items and sections need unique IDs
3. **Handle visibility** - Use `visible` prop to show/hide panels
4. **Manage state externally** - Components are controlled
5. **Use TypeScript** - Leverage type safety for better DX

## Examples

See individual component story files in the `elements` folder for detailed examples:
- `Navbar.stories.tsx`
- `Footer.stories.tsx`
- `Canvas.stories.tsx`
- `Panel.stories.tsx`
- `Table.stories.tsx`
