// #region 🔖Header

// semio-elements/ui/.storybook/stories/elements/UI.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import { Home, Info, Layers, Settings, User, FileText, BarChart } from "lucide-react";
import { UI, UIAppConfig, WindowKind, FooterItem, LevelProvider, Level, getLevelBgClass } from "@semio-elements/ui";

// #region 🔖UI

const meta = {
  title: "Elements/UI",
  component: UI,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof UI>;

export default meta;

type Story = StoryObj<typeof meta>;

const ExampleWindowContent = ({ title, color = "bg-window" }: { title: string; color?: string }) => (
  <div className={`flex items-center justify-center h-full ${color}`}>
    <h2 className="text-xl font-bold">{title}</h2>
  </div>
);

const simpleApps: UIAppConfig[] = [
  {
    id: "editor",
    label: "Editor",
    icon: <FileText size={16} />,
    windows: [
      { id: "main-editor", label: "Main Editor", content: <ExampleWindowContent title="Main Editor Window" />, defaultSize: 60 },
      { id: "preview", label: "Preview", content: <ExampleWindowContent title="Preview Window" color="bg-panel" />, defaultSize: 40 },
    ],
    leftPanelTabs: [
      { id: "explorer", icon: Layers, order: 0, content: <div className="p-2">Explorer content</div> },
      { id: "settings", icon: Settings, order: 1, content: <div className="p-2">Settings content</div> },
    ],
    rightPanelTabs: [
      { id: "properties", icon: Info, order: 0, content: <div className="p-2">Properties content</div> },
    ],
    bottomPanelSections: [
      { id: "console", content: <div className="p-2 font-mono text-xs">Console output...</div>, defaultOpen: true, order: 0 },
    ],
    footerItems: [
      { id: "status", content: "Ready", order: 0 },
      { id: "line", content: "Ln 42, Col 8", order: 1 },
    ],
  },
  {
    id: "dashboard",
    label: "Dashboard",
    icon: <BarChart size={16} />,
    windows: [
      { id: "stats", label: "Statistics", content: <ExampleWindowContent title="Statistics" />, windowKind: WindowKind.TABLE },
    ],
    leftPanelTabs: [
      { id: "metrics", icon: BarChart, order: 0, content: <div className="p-2">Metrics list</div> },
    ],
    footerItems: [
      { id: "last-updated", content: "Updated 2m ago", order: 0 },
    ],
  },
];

export const Default: Story = {
  args: {
    apps: simpleApps,
    navbarLeading: [{ key: "home", content: <Home size={18} /> }],
    navbarTrailing: [{ key: "user", content: <User size={18} /> }],
    footerItems: [{ id: "version", content: "v1.0.0", order: 100 }],
  },
};

export const SingleApp: Story = {
  args: {
    apps: [simpleApps[0]],
  },
};

export const MultipleApps: Story = {
  args: {
    apps: simpleApps,
    defaultAppId: "dashboard",
  },
};

export const MinimalApp: Story = {
  args: {
    apps: [
      {
        id: "minimal",
        label: "Minimal",
        windows: [{ id: "main", label: "Main", content: <ExampleWindowContent title="Minimal App" /> }],
      },
    ],
  },
};

const createLevelRender = (level: Level): Story["render"] => (args) => (
  <LevelProvider level={level}>
    <div className={`h-screen ${getLevelBgClass(level)}`}>
      <UI {...args} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { apps: simpleApps },
  render: createLevelRender("base"),
};

export const WithToolbar: Story = {
  args: {
    apps: [
      {
        ...simpleApps[0],
        toolbarContent: (
          <div className="flex items-center gap-2 px-3 py-1 bg-panel border rounded-md shadow-sm">
            <button className="px-2 py-1 hover:bg-hover-panel rounded text-sm">Undo</button>
            <button className="px-2 py-1 hover:bg-hover-panel rounded text-sm">Redo</button>
            <div className="w-px h-4 bg-border" />
            <button className="px-2 py-1 hover:bg-hover-panel rounded text-sm">Save</button>
          </div>
        ),
      },
    ],
  },
};

// #endregion 🔖UI
