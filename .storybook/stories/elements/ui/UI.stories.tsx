// #region 🧲Header

// .elements/ui/.storybook/stories/elements/UI.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { BreadcrumbItemData, Tree, UI, UIAppConfig, UIFindItem, UISearchItem, UIToolbarItem, createDefaultLayout } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";
import { BarChart, BookOpen, ClipboardPaste, Copy, File, FileText, FolderOpen, Home, Info, Layers, Redo, Save, Scissors, Settings, Undo } from "lucide-react";
import { expect, userEvent, within } from "storybook/test";

// #region 🎊UI

const meta = {
  title: "elements/UI",
  component: UI,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof UI>;

export default meta;

type Story = StoryObj<typeof meta>;

// #region 🧿Windows

const EditorWindow = () => (
  <div className="flex items-center justify-center h-full bg-window">
    <h2 className="text-xl font-bold">Editor Window</h2>
  </div>
);

const PreviewWindow = () => (
  <div className="flex items-center justify-center h-full bg-panel">
    <h2 className="text-xl font-bold">Preview Window</h2>
  </div>
);

const StatsWindow = () => (
  <div className="flex items-center justify-center h-full bg-window">
    <h2 className="text-xl font-bold">Statistics</h2>
  </div>
);

// #endregion 🧿Windows

// #region 🏆TreePanels

const ExplorerTree = () => (
  <Tree
    sections={[
      {
        id: "explorer.src",
        label: "src",
        icon: <FolderOpen size={14} />,
        defaultOpen: true,
        items: [
          { id: "explorer.src.index", label: "index.ts", icon: <File size={14} /> },
          { id: "explorer.src.app", label: "app.tsx", icon: <File size={14} /> },
          {
            id: "explorer.src.components",
            label: "components",
            icon: <FolderOpen size={14} />,
            defaultOpen: true,
            items: [
              { id: "explorer.src.components.button", label: "Button.tsx", icon: <File size={14} /> },
              { id: "explorer.src.components.card", label: "Card.tsx", icon: <File size={14} /> },
              { id: "explorer.src.components.layout", label: "Layout.tsx", icon: <File size={14} /> },
            ],
          },
          {
            id: "explorer.src.utils",
            label: "utils",
            icon: <FolderOpen size={14} />,
            items: [
              { id: "explorer.src.utils.helpers", label: "helpers.ts", icon: <File size={14} /> },
              { id: "explorer.src.utils.constants", label: "constants.ts", icon: <File size={14} /> },
            ],
          },
        ],
      },
      {
        id: "explorer.public",
        label: "public",
        icon: <FolderOpen size={14} />,
        items: [{ id: "explorer.public.favicon", label: "favicon.ico", icon: <File size={14} /> }],
      },
    ]}
  />
);

const PropertiesTree = () => (
  <Tree
    sections={[
      {
        id: "properties.element",
        label: "Element",
        icon: <Info size={14} />,
        defaultOpen: true,
        items: [
          { id: "properties.element.id", label: "id: editor-1" },
          { id: "properties.element.kind", label: "kind: text" },
          { id: "properties.element.visible", label: "visible: true" },
        ],
      },
      {
        id: "properties.style",
        label: "Style",
        icon: <Settings size={14} />,
        defaultOpen: true,
        items: [
          { id: "properties.style.width", label: "width: 100%" },
          { id: "properties.style.height", label: "height: auto" },
          { id: "properties.style.padding", label: "padding: 16px" },
        ],
      },
    ]}
  />
);

const MetricsTree = () => (
  <Tree
    sections={[
      {
        id: "metrics.performance",
        label: "Performance",
        icon: <BarChart size={14} />,
        defaultOpen: true,
        items: [
          { id: "metrics.performance.fps", label: "FPS: 60" },
          { id: "metrics.performance.memory", label: "Memory: 128MB" },
          { id: "metrics.performance.cpu", label: "CPU: 12%" },
        ],
      },
      {
        id: "metrics.network",
        label: "Network",
        icon: <BarChart size={14} />,
        items: [
          { id: "metrics.network.requests", label: "Requests: 42" },
          { id: "metrics.network.latency", label: "Latency: 12ms" },
        ],
      },
    ]}
  />
);

// #endregion 🏆TreePanels

// #region 🌟SearchItems

const searchItems: UISearchItem[] = [
  { id: "s1", label: "index.ts", description: "Main entry point", icon: <File size={14} />, category: "Files", onSelect: () => {} },
  { id: "s2", label: "app.tsx", description: "Root application component", icon: <File size={14} />, category: "Files", onSelect: () => {} },
  { id: "s3", label: "Button.tsx", description: "Button component", icon: <File size={14} />, category: "Components", onSelect: () => {} },
  { id: "s4", label: "Card.tsx", description: "Card component", icon: <File size={14} />, category: "Components", onSelect: () => {} },
  { id: "s5", label: "Layout.tsx", description: "Layout component", icon: <File size={14} />, category: "Components", onSelect: () => {} },
  { id: "s6", label: "Settings", description: "Application settings", icon: <Settings size={14} />, category: "Pages", onSelect: () => {} },
  { id: "s7", label: "Documentation", description: "Read the docs", icon: <BookOpen size={14} />, category: "Pages", onSelect: () => {} },
];

// #endregion 🌟SearchItems

// #region 🖲️FindItems

const editorFindItems: UIFindItem[] = [
  { id: "f1", label: "function handleClick", description: "Line 42", category: "Functions" },
  { id: "f2", label: "function renderEditor", description: "Line 87", category: "Functions" },
  { id: "f3", label: "const EDITOR_CONFIG", description: "Line 12", category: "Constants" },
  { id: "f4", label: "interface EditorProps", description: "Line 5", category: "Interfaces" },
  { id: "f5", label: "class EditorState", description: "Line 120", category: "Classes" },
];

// #endregion 🖲️FindItems

// #region 👓ToolbarItems

const editorToolbarItems: UIToolbarItem[] = [
  { id: "undo", icon: <Undo size={14} />, label: "Undo", onClick: () => {}, order: 0 },
  { id: "redo", icon: <Redo size={14} />, label: "Redo", onClick: () => {}, order: 1 },
  { id: "sep1", kind: "separator", order: 2 },
  { id: "cut", icon: <Scissors size={14} />, onClick: () => {}, order: 3 },
  { id: "copy", icon: <Copy size={14} />, onClick: () => {}, order: 4 },
  { id: "paste", icon: <ClipboardPaste size={14} />, onClick: () => {}, order: 5 },
  { id: "sep2", kind: "separator", order: 6 },
  { id: "save", icon: <Save size={14} />, label: "Save", onClick: () => {}, order: 7 },
];

// #endregion 👓ToolbarItems

// #region 🦉Apps

const editorApp: UIAppConfig = {
  id: "editor",
  label: "Editor",
  icon: <FileText size={16} />,
  windowKinds: [
    { id: "editor", label: "Editor", component: EditorWindow },
    { id: "preview", label: "Preview", component: PreviewWindow },
  ],
  defaultLayout: createDefaultLayout(["editor", "preview"], "row", [60, 40]),
  leftPanelTabs: [
    { id: "explorer", icon: Layers, order: 0, content: <ExplorerTree /> },
    { id: "settings", icon: Settings, order: 1, content: <div className="p-2">Settings content.</div> },
  ],
  rightPanelTabs: [{ id: "properties", icon: Info, order: 0, content: <PropertiesTree /> }],
  toolbarItems: editorToolbarItems,
  footerItems: [
    { id: "status", content: "Ready", order: 0 },
    { id: "line", content: "Ln 42, Col 8", order: 1 },
  ],
  findItems: editorFindItems,
  onFindSelect: (itemId) => console.log("Find selected:", itemId),
};

const dashboardApp: UIAppConfig = {
  id: "dashboard",
  label: "Dashboard",
  icon: <BarChart size={16} />,
  windowKinds: [{ id: "stats", label: "Statistics", component: StatsWindow }],
  defaultLayout: createDefaultLayout(["stats"]),
  leftPanelTabs: [{ id: "metrics", icon: BarChart, order: 0, content: <MetricsTree /> }],
  footerItems: [{ id: "last-updated", content: "Updated 2m ago", order: 0 }],
};

// #endregion 🦉Apps

// #region 💡Breadcrumb

const breadcrumbItems: BreadcrumbItemData[] = [
  {
    id: "home",
    content: (
      <a className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
        <Home size={16} />
      </a>
    ),
    options: [
      { label: "Local Projects", href: "/?kind=local" },
      { label: "Remote Projects", href: "/?kind=remote" },
      { label: "Recent", href: "/?kind=recent" },
    ],
    onNavigate: (href) => console.log("Navigate to:", href),
  },
  {
    id: "project",
    content: <a className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">My Project</a>,
    options: [
      { label: "Project A", href: "/projects/a" },
      { label: "Project B", href: "/projects/b" },
      { label: "Project C", href: "/projects/c" },
    ],
    onNavigate: (href) => console.log("Navigate to:", href),
  },
  {
    id: "artifactKind",
    content: (
      <a className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
        <FileText size={16} />
      </a>
    ),
    options: [
      { label: "Files", href: "/projects/a?kind=files" },
      { label: "Components", href: "/projects/a?kind=components" },
      { label: "Settings", href: "/projects/a?kind=settings" },
    ],
    onNavigate: (href) => console.log("Navigate to:", href),
  },
  {
    id: "file",
    content: <span className="text-foreground px-single flex items-center gap-single h-full">main.ts</span>,
  },
];

// #endregion 💡Breadcrumb

// #region 📮Stories

export const Default: Story = {
  args: {
    apps: [editorApp, dashboardApp],
    breadcrumbItems,
    searchItems,
    footerItems: [{ id: "version", content: "v1.0.0", order: 100 }],
  },
};

export const SingleApp: Story = {
  args: {
    apps: [editorApp],
    breadcrumbItems,
    searchItems,
  },
};

export const NoBreadcrumb: Story = {
  args: {
    apps: [editorApp, dashboardApp],
    defaultAppId: "dashboard",
    searchItems,
  },
};

export const WithToolbarItems: Story = {
  args: {
    apps: [editorApp],
    breadcrumbItems,
    searchItems,
    toolbarItems: [{ id: "global-save", icon: <Save size={14} />, label: "Save All", onClick: () => {}, order: 100 }],
  },
};

export const WithToolbarContent: Story = {
  args: {
    apps: [
      {
        ...editorApp,
        toolbarItems: undefined,
        toolbarContent: (
          <div className="flex items-center gap-2 px-3 py-1 bg-panel border rounded-md shadow-sm pointer-events-auto">
            <button className="px-2 py-1 hover:bg-hover-panel rounded text-sm">Undo</button>
            <button className="px-2 py-1 hover:bg-hover-panel rounded text-sm">Redo</button>
            <div className="w-px h-4 bg-border" />
            <button className="px-2 py-1 hover:bg-hover-panel rounded text-sm">Save</button>
          </div>
        ),
      },
    ],
    breadcrumbItems,
  },
};

export const WithSearch: Story = {
  args: {
    apps: [
      {
        id: "minimal",
        label: "Minimal",
        windowKinds: [{ id: "main", label: "Main", component: () => <div className="flex items-center justify-center h-full">Press Ctrl+P to search</div> }],
        defaultLayout: createDefaultLayout(["main"]),
      },
    ],
    searchItems,
  },
};

export const WithFind: Story = {
  args: {
    apps: [
      {
        id: "code",
        label: "Code",
        windowKinds: [{ id: "main", label: "Main", component: () => <div className="flex items-center justify-center h-full">Press Ctrl+F to find</div> }],
        defaultLayout: createDefaultLayout(["main"]),
        findItems: editorFindItems,
        onFindSelect: (itemId) => console.log("Find selected:", itemId),
      },
    ],
  },
};

export const WithTreePanels: Story = {
  args: {
    apps: [
      {
        id: "tree-demo",
        label: "Tree Demo",
        windowKinds: [{ id: "main", label: "Main", component: () => <div className="flex items-center justify-center h-full">Every panel has a tree</div> }],
        defaultLayout: createDefaultLayout(["main"]),
        leftPanelTabs: [{ id: "explorer", icon: Layers, order: 0, content: <ExplorerTree /> }],
        rightPanelTabs: [{ id: "properties", icon: Info, order: 0, content: <PropertiesTree /> }],
      },
    ],
    breadcrumbItems: [
      {
        id: "root",
        content: (
          <a className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
            <Home size={16} />
          </a>
        ),
      },
      {
        id: "section",
        content: <span className="text-foreground px-single flex items-center gap-single h-full">Workspace</span>,
      },
    ],
    searchItems,
  },
};

export const FullFeatured: Story = {
  args: {
    apps: [editorApp, dashboardApp],
    breadcrumbItems,
    searchItems,
    footerItems: [{ id: "version", content: "v1.0.0", order: 100 }],
    toolbarItems: [{ id: "global-save", icon: <Save size={14} />, label: "Save All", onClick: () => {}, order: 100 }],
  },
  play: async ({ canvasElement }) => {
    const documentBody = canvasElement.ownerDocument.body;
    const leftPanelToggle = canvasElement.ownerDocument.getElementById("ui.panelToggle.left");
    const searchToggle = canvasElement.ownerDocument.getElementById("ui.search.toggle");
    const findToggle = canvasElement.ownerDocument.getElementById("ui.find.toggle");

    expect(leftPanelToggle).toBeTruthy();
    expect(searchToggle).toBeTruthy();
    expect(findToggle).toBeTruthy();

    await userEvent.click(leftPanelToggle!);
    expect(documentBody.querySelector('[data-panel="leftSidePanel"]')).toBeTruthy();
    expect(within(documentBody).getByText("src")).toBeTruthy();

    await userEvent.click(searchToggle!);
    expect(canvasElement.ownerDocument.getElementById("ui.search.input")).toBeTruthy();

    await userEvent.click(searchToggle!);
    await userEvent.click(findToggle!);
    expect(canvasElement.ownerDocument.getElementById("ui.find.input")).toBeTruthy();
  },
};

export const ThreeColumnLayout: Story = {
  args: {
    apps: [
      {
        id: "three-col",
        label: "Three Columns",
        windowKinds: [
            { id: "left", label: "Left", component: () => <div className="flex items-center justify-center h-full">Left</div> },
            { id: "center", label: "Center", component: () => <div className="flex items-center justify-center h-full">Center</div> },
            { id: "right", label: "Right", component: () => <div className="flex items-center justify-center h-full">Right</div> },
          ],
        defaultLayout: createDefaultLayout(["left", "center", "right"], "row", [25, 50, 25]),
        leftPanelTabs: [{ id: "nav", icon: Layers, order: 0, content: <ExplorerTree /> }],
        rightPanelTabs: [{ id: "props", icon: Info, order: 0, content: <PropertiesTree /> }],
      },
    ],
    breadcrumbItems: [
      {
        id: "root",
        content: (
          <a className="text-foreground transition-colors px-single flex items-center gap-single h-full hover:bg-hover-base cursor-selectable">
            <Home size={16} />
          </a>
        ),
      },
      {
        id: "section",
        content: <span className="text-foreground px-single flex items-center gap-single h-full">Workspace</span>,
      },
    ],
    searchItems,
  },
};

export const MinimalApp: Story = {
  args: {
    apps: [
      {
        id: "minimal",
        label: "Minimal",
        windowKinds: [{ id: "main", label: "Main", component: () => <div className="flex items-center justify-center h-full">Minimal App</div> }],
        defaultLayout: createDefaultLayout(["main"]),
      },
    ],
  },
};

export const Mobile: Story = {
  args: {
    apps: [editorApp, dashboardApp],
    breadcrumbItems: [breadcrumbItems[0], breadcrumbItems[3]],
    searchItems,
    footerItems: [{ id: "version", content: "v1.0.0", order: 100 }],
    mobile: true,
  },
  parameters: {
    viewport: { defaultViewport: "mobile1" },
    layout: "fullscreen",
  },
  decorators: [
    (Story) => (
      <div style={{ width: "375px", height: "667px", overflow: "hidden", border: "1px solid var(--border-color)" }}>
        <Story />
      </div>
    ),
  ],
  play: async ({ canvasElement }) => {
    const documentBody = canvasElement.ownerDocument.body;
    const mobilePanelToggle = canvasElement.ownerDocument.getElementById("ui.panelToggle.mobile");

    expect(mobilePanelToggle).toBeTruthy();

    await userEvent.click(mobilePanelToggle!);
    expect(documentBody.querySelector('[data-panel="mobilePanel"]')).toBeTruthy();
    expect(within(documentBody).getByText("src")).toBeTruthy();
  },
};

export const MobileSingleApp: Story = {
  args: {
    apps: [editorApp],
    breadcrumbItems: [breadcrumbItems[0], breadcrumbItems[3]],
    searchItems,
    mobile: true,
  },
  parameters: {
    viewport: { defaultViewport: "mobile1" },
    layout: "fullscreen",
  },
  decorators: [
    (Story) => (
      <div style={{ width: "375px", height: "667px", overflow: "hidden", border: "1px solid var(--border-color)" }}>
        <Story />
      </div>
    ),
  ],
};

export const MobileNoPanels: Story = {
  args: {
    apps: [
      {
        id: "viewer",
        label: "Viewer",
        windowKinds: [{ id: "main", label: "Main", component: () => <div className="flex items-center justify-center h-full text-lg">Full Screen Canvas</div> }],
        defaultLayout: createDefaultLayout(["main"]),
      },
    ],
    breadcrumbItems: [breadcrumbItems[0]],
    mobile: true,
  },
  parameters: {
    viewport: { defaultViewport: "mobile1" },
    layout: "fullscreen",
  },
  decorators: [
    (Story) => (
      <div style={{ width: "375px", height: "667px", overflow: "hidden", border: "1px solid var(--border-color)" }}>
        <Story />
      </div>
    ),
  ],
};

// #endregion 📮Stories

// #endregion 🎊UI
