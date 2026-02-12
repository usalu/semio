import re
from collections import defaultdict

FILE = "/workspaces/semio/semio/js/sketchpad/elements.tsx"

with open(FILE, "r") as f:
    lines = f.readlines()

section_summaries = {
    "Imports": ("External library and internal module imports used across all sections.", "Consumers MUST NOT add non-tree-shakeable imports."),
    "Section Specificity": ("Enum defining priority levels for section content ownership.", "Consumers MUST use these constants for section precedence."),
    "Interaction Context": ("React context for tracking active UI interactions.", "Consumers MUST wrap interactive elements with InteractionProvider."),
    "Level Context": ("React context for UI depth level tracking.", "Consumers MUST wrap components with LevelProvider."),
    "Element": ("Core element types, transaction context, and level-based CSS class helpers.", "Consumers MUST use level functions for consistent styling."),
    "Command": ("Command palette UI built on cmdk primitives.", "Consumers MUST use CommandInput for search functionality."),
    "Footer": ("Status bar component at the bottom of the layout.", "Consumers MUST provide FooterItem entries for each action."),
    "Layout": ("Top-level layout orchestrating navbar, panels, canvas, and footer.", "Consumers MUST provide a canvas element."),
    "Popover": ("Floating popover component built on Radix primitives.", "Consumers MUST wrap content in PopoverContent."),
    "Tooltip": ("Tooltip components with expertise-level adaptive content.", "Consumers MUST configure the expertise mode provider."),
    "Base Components": ("Foundational internal components like Label.", "Consumers MUST use these as building blocks for inputs."),
    "Display Components": ("Read-only display wrappers for tooltips and callouts.", "Consumers MUST pass valid config objects."),
    "Aside": ("Callout boxes for notes, tips, cautions, and dangers.", "Consumers MUST specify a valid kind prop."),
    "Avatar": ("User avatar components with image, fallback, drag, and table variants.", "Consumers MUST provide content for the fallback."),
    "Card": ("Card container and grid layout for content blocks.", "Consumers MUST provide a title string."),
    "Spinner": ("Animated loading spinner in small, medium, or large sizes.", "Consumers MUST choose an appropriate size for the context."),
    "NotFound": ("404-style placeholder with icon, title, and back navigation.", "Consumers MUST provide a title for the error."),
    "LoadingRow": ("Skeleton loading row with pulsing icon and name.", "Consumers MUST provide a name for the placeholder."),
    "DiagramNode": ("Individual diagram node element with selection and hover states.", "Consumers MUST provide content for the node."),
    "HoverCard": ("Hover-triggered card built on Radix primitives.", "Consumers MUST use HoverCardTrigger to activate."),
    "Icons": ("Cursor icon component for collaborative pointer display.", "Consumers MUST provide position data for rendering."),
    "Section": ("Collapsible section container with heading and specificity.", "Consumers MUST provide a heading string."),
    "Steps": ("Ordered step list container for tutorial or wizard flows.", "Consumers MUST provide step children in order."),
    "ActionGroup": ("Compact action button group with dropdown support.", "Consumers MUST provide action items for the group."),
    "Combobox": ("Searchable dropdown with popover options list.", "Consumers MUST provide options and onValueChange handler."),
    "Input": ("Text input field with label, validation, and clear support.", "Consumers MUST provide an id for accessibility."),
    "Select": ("Dropdown select built on Radix primitives.", "Consumers MUST use SelectItem children for options."),
    "Slider": ("Range slider built on Radix primitives.", "Consumers MUST provide min and max values."),
    "Stepper": ("Numeric stepper with increment/decrement and drag adjustment.", "Consumers MUST provide min and max bounds."),
    "Textarea": ("Multi-line text input with label and validation.", "Consumers MUST provide an id for the field."),
    "Toggle": ("Toggle button with pressed/unpressed states.", "Consumers MUST handle onPressedChange events."),
    "ToggleGroup": ("Group of mutually exclusive or multi-select toggles.", "Consumers MUST provide items with distinct values."),
    "Accordion": ("Collapsible accordion built on Radix primitives.", "Consumers MUST use AccordionItem children."),
    "Collapsible": ("Collapsible section built on Radix primitives.", "Consumers MUST use CollapsibleTrigger."),
    "Dialog": ("Modal dialog built on Radix primitives.", "Consumers MUST use DialogTrigger to open."),
    "Resizable": ("Resizable panel layout built on react-resizable-panels.", "Consumers MUST use ResizableHandle between panels."),
    "Scrollable": ("Custom scrollable area built on Radix ScrollArea.", "Consumers MUST wrap content in Scrollable."),
    "Band": ("Horizontal band of navigation items with labels and icons.", "Consumers MUST provide BandItem entries."),
    "Strip": ("Vertical strip of icon items for compact navigation.", "Consumers MUST provide StripItem entries."),
    "Navbar": ("Top navigation bar with icon items.", "Consumers MUST provide NavbarItem entries."),
    "Tabs": ("Tab container built on Radix primitives.", "Consumers MUST use TabsTrigger and TabsContent."),
    "Tree": ("Hierarchical tree view with sections, items, and file trees.", "Consumers MUST wrap components in TreeStateProvider."),
    "Breadcrumb": ("Breadcrumb trail for hierarchical page navigation.", "Consumers MUST provide BreadcrumbItemData entries."),
    "PageNavigation": ("Previous/next page navigation links.", "Consumers MUST provide PageNavigationLink data."),
    "Panel": ("Resizable dockable panel with sections and collapse support.", "Consumers MUST set resizeSide for the handle."),
    "PanelGroup": ("Flex container grouping multiple panels together.", "Consumers MUST provide panel children."),
    "LeftPanel": ("Left-docked panel variant with right resize handle.", "Consumers MUST provide visible and children props."),
    "RightPanel": ("Right-docked panel variant with left resize handle.", "Consumers MUST provide visible and children props."),
    "MiddlePanel": ("Center panel variant without resize handles.", "Consumers MUST provide visible and children props."),
    "BottomPanel": ("Bottom-docked panel variant with top resize handle.", "Consumers MUST provide visible and children props."),
    "SidePanel": ("Collapsible side panel with tabbed content.", "Consumers MUST provide SidePanelTabConfig entries."),
    "HudPanel": ("Floating heads-up display panel with tabs.", "Consumers MUST provide HudPanelTabConfig entries."),
    "Window": ("Draggable, resizable floating window with dashed border.", "Consumers MUST provide a WindowConfig object."),
    "Page": ("Full-page content wrapper with frontmatter and footer.", "Consumers MUST provide frontmatter and children."),
    "Diagram": ("Interactive node-edge diagram built on ReactFlow and D3 force.", "Consumers MUST provide nodes and edges arrays."),
    "Scene": ("3D scene viewer built on React Three Fiber.", "Consumers MUST provide SceneGeometry data."),
    "Table": ("Sortable, hierarchical data table with drag-drop support.", "Consumers MUST provide columns and data arrays."),
}

definition_summaries = {
    "SectionSpecificity": "Priority enum for section content ownership across apps.",
    "InteractionProvider": "Context provider for UI interaction commands and active state.",
    "Level": "Union type for UI depth levels.",
    "LevelProvider": "Context provider that sets the current UI level.",
    "useLevel": "Hook returning the current UI depth level.",
    "Transaction": "Interface for start/finalize/abort lifecycle of a UI transaction.",
    "TransactionProvider": "Context provider that supplies a Transaction to descendants.",
    "useTransaction": "Hook returning the current Transaction context.",
    "ElementBaseProps": "Base props interface requiring an id string.",
    "ElementProps": "Extended element props inheriting ElementBaseProps.",
    "getLevelBgClass": "Returns the Tailwind background class for a given level.",
    "getLevelHoverClass": "Returns the Tailwind hover background class for a given level.",
    "getLevelActiveHoverClass": "Returns the Tailwind active-state hover class for a given level.",
    "getLevelZClass": "Returns the Tailwind z-index class for a given level.",
    "getLevelBorderElementClass": "Returns the Tailwind border class for a given level.",
    "getLevelDivideElementClass": "Returns the Tailwind divide class for a given level.",
    "FooterItem": "Configuration interface for a single footer action item.",
    "FooterProps": "Props interface for the Footer component.",
    "LayoutProps": "Props interface for the top-level Layout component.",
    "TooltipConfig": "Configuration for enhanced tooltip with label, paths, and hotkey.",
    "DescriptionTooltipData": "Data interface for description-based tooltip content.",
    "setTooltipModeProvider": "Registers the expertise provider function for tooltips.",
    "useTooltipMode": "Hook returning the current expertise level for tooltips.",
    "AsideProps": "Props interface for the Aside callout component.",
    "Aside": "Callout component rendering note, tip, caution, or danger boxes.",
    "DraggableAvatarProps": "Props interface for the DraggableAvatar component.",
    "DraggableAvatar": "Avatar component with drag-and-drop support and selection styling.",
    "TableAvatarProps": "Props interface for the TableAvatar component.",
    "TableAvatar": "Avatar component optimized for table row display.",
    "CardProps": "Props interface for the Card component.",
    "Card": "Content card with title, icon, and children.",
    "CardGridProps": "Props interface for the CardGrid component.",
    "CardGrid": "Responsive grid layout for Card components.",
    "SpinnerProps": "Props interface for the Spinner component.",
    "Spinner": "Animated SVG loading spinner.",
    "NotFoundProps": "Props interface for the NotFound component.",
    "NotFound": "Not-found placeholder page with navigation link.",
    "LoadingRowProps": "Props interface for the LoadingRow component.",
    "LoadingRow": "Skeleton row showing pulsing icon and name placeholder.",
    "DiagramNodeProps": "Props interface for the DiagramNode component.",
    "DiagramNode": "Individual node element within a diagram graph.",
    "PlaceholderDiagramNode": "Empty placeholder node for adding new diagram entries.",
    "SectionProps": "Props interface for the Section component.",
    "StepsProps": "Props interface for the Steps component.",
    "Steps": "Ordered step list container rendering numbered children.",
    "Combobox": "Searchable combobox dropdown with autocomplete filtering.",
    "Stepper": "Numeric stepper with increment, decrement, and drag-to-adjust.",
    "ToggleItem": "Configuration interface for a single toggle option with value and label.",
    "BandItem": "Configuration interface for a single band item.",
    "BandProps": "Props interface for the Band component.",
    "StripItem": "Configuration interface for a single strip item.",
    "StripProps": "Props interface for the Strip component.",
    "NavbarItem": "Configuration interface for a single navbar item.",
    "NavbarProps": "Props interface for the Navbar component.",
    "TreeStateProvider": "Context provider managing tree expansion state.",
    "useTreeState": "Hook returning tree expansion state and toggle functions.",
    "TreeContent": "Wrapper rendering tree children with connecting lines.",
    "TreeSectionAction": "Configuration interface for an action button on a tree section.",
    "TreeSection": "Collapsible tree section header with optional action buttons.",
    "SortableTreeItems": "Drag-and-drop sortable container for tree items.",
    "TreeItem": "Single tree item row with icon, label, and interaction handlers.",
    "TreeItems": "Iterator rendering a list of tree item children.",
    "FileTreeNode": "Data interface for a node in a file tree.",
    "Tree": "Hierarchical tree view component with optional file tree rendering.",
    "FileTree": "Alias for Tree.Files rendering a file tree from FileTreeNode data.",
    "BreadcrumbItemData": "Data interface for a single breadcrumb entry.",
    "PageNavigationLink": "Configuration interface for a previous/next page link.",
    "PageNavigationProps": "Props interface for the PageNavigation component.",
    "ResizeSide": "Union type for panel resize handle positions.",
    "PanelSection": "Configuration interface for a collapsible section within a panel.",
    "PanelProps": "Props interface for the Panel component.",
    "PanelGroupProps": "Props interface for the PanelGroup component.",
    "LeftPanelProps": "Props type for LeftPanel omitting resizeSide.",
    "RightPanelProps": "Props type for RightPanel omitting resizeSide.",
    "MiddlePanelProps": "Props type for MiddlePanel omitting resizeSide.",
    "BottomPanelProps": "Props type for BottomPanel omitting resizeSide.",
    "SidePanelTabConfig": "Configuration interface for a side panel tab.",
    "SidePanelProps": "Props interface for the SidePanel component.",
    "HudPanelTabConfig": "Configuration interface for a HUD panel tab.",
    "HudPanelProps": "Props interface for the HudPanel component.",
    "WindowConfig": "Configuration interface for a floating window instance.",
    "PageFrontmatter": "Frontmatter metadata interface for a documentation page.",
    "PageProps": "Props interface for the Page component.",
    "Page": "Full-page wrapper with frontmatter header and footer.",
    "DIAGRAM_UNIT": "Base pixel unit for diagram node sizing.",
    "DiagramLayoutDirection": "Union type for diagram layout directions (TB/BT/LR/RL).",
    "DiagramLayoutOptions": "Configuration interface for dagre-based diagram layout.",
    "calculateDiagramLayout": "Computes dagre layout positions for diagram nodes and edges.",
    "DiagramForceConfig": "Configuration interface for D3 force simulation parameters.",
    "defaultDiagramForceConfig": "Default D3 force configuration values.",
    "DiagramProps": "Props interface for the Diagram component.",
    "useDiagramLayout": "Hook computing and memoizing diagram layout from nodes and edges.",
    "DiagramSkeleton": "Skeleton loading placeholder for a diagram.",
    "SceneGeometry": "Interface for a geometry entry in a 3D scene.",
    "TransformableGeometry": "Extended SceneGeometry with transform delta support.",
    "PlaneTransformDelta": "Interface for an incremental plane transformation delta.",
    "OnPlaneUpdate": "Callback type for a single plane update.",
    "OnMultiPlaneUpdate": "Callback type for batch plane updates.",
    "planeFromPointAndDirection": "Constructs a Plane from a point and direction vector.",
    "getPlanePosition": "Extracts the THREE.Vector3 position from a Plane.",
    "hasValidPlane": "Checks whether a geometry has a non-null plane.",
    "isGeometryFocusable": "Checks whether a geometry has a valid plane for camera focus.",
    "Geometry": "3D geometry mesh component with selection, hover, and edge rendering.",
    "Scene": "3D scene viewer with orbit controls, grid, and geometry rendering.",
    "SceneSkeleton": "Skeleton loading placeholder for a 3D scene.",
    "SortDirection": "Union type for ascending or descending sort order.",
    "TableColumn": "Configuration interface for a table column definition.",
    "HierarchicalRowData": "Interface for hierarchical row data with parent/child relations.",
    "DragDropConfig": "Configuration interface for table drag-and-drop behavior.",
    "TableProps": "Props interface for the Table component.",
    "TableSkeletonProps": "Props interface for the TableSkeleton component.",
    "TableSkeleton": "Skeleton loading placeholder for a table.",
}

insertions = []
processed_definitions = set()

region_pattern = re.compile(r'^// #region 🔖(.+)$')
for i, line in enumerate(lines):
    m = region_pattern.match(line.rstrip('\n'))
    if m and m.group(1) in section_summaries:
        summary, spec = section_summaries[m.group(1)]
        insertions.append((i + 1, f'// {summary}\n'))
        insertions.append((i + 1, f'// {spec}\n'))

export_pattern = re.compile(r'^export\s+(?:enum|const|type|interface|function|class)\s+(\w+)')
for i, line in enumerate(lines):
    m = export_pattern.match(line.rstrip('\n'))
    if m and m.group(1) in definition_summaries and m.group(1) not in processed_definitions:
        processed_definitions.add(m.group(1))
        insertions.append((i, f'// {definition_summaries[m.group(1)]}\n'))

grouped = defaultdict(list)
for idx, text in insertions:
    grouped[idx].append(text)

for idx in sorted(grouped.keys(), reverse=True):
    for text in reversed(grouped[idx]):
        lines.insert(idx, text)

endregion_idx = None
orphan_idx = None
orphan_content = 'export { Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut };'
for i, line in enumerate(lines):
    s = line.rstrip('\n')
    if s == '// #endregion 🔖Command':
        endregion_idx = i
    if endregion_idx is not None and i > endregion_idx and orphan_content in s:
        orphan_idx = i
        break

if orphan_idx is not None and endregion_idx is not None:
    orphan_line = lines.pop(orphan_idx)
    if orphan_idx - 1 >= 0 and lines[orphan_idx - 1].strip() == '':
        lines.pop(orphan_idx - 1)
    for i, line in enumerate(lines):
        if line.rstrip('\n') == '// #endregion 🔖Command':
            endregion_idx = i
            break
    lines.insert(endregion_idx, orphan_line)
    lines.insert(endregion_idx, '\n')

with open(FILE, 'w') as f:
    f.writelines(lines)

print(f"Done. Sections: {len(section_summaries)}, Definitions: {len(processed_definitions)}, Orphan: 1")
