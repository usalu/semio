// #region 🧲Header

// .elements/ui/.storybook/story/elements/Panel.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { BottomPanel, Panel, PanelSection, SidePanel } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState, type ComponentType } from "react";

// #region 🦉Panel

const Info = createIconComponent("info");
const Layers = createIconComponent("layers");
const Settings = createIconComponent("settings");

const meta = {
  title: "🖱️ui⚛️react/Panel",
  component: Panel,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Panel>;

export default meta;

type Story = StoryObj<typeof meta>;

const sampleSections: PanelSection[] = [
  { id: "types", content: <div className="p-2">Types panel content with a list of items.</div>, defaultOpen: true, order: 0 },
  { id: "properties", content: <div className="p-2">Properties panel showing details.</div>, defaultOpen: false, order: 1 },
  { id: "actions", content: <div className="p-2">Actions available for the current selection.</div>, defaultOpen: false, order: 2 },
];

export const Default: Story = {
  args: {
    visible: true,
    sections: sampleSections,
    size: 250,
    resizeSide: "right" as const,
  },
  render: (args) => (
    <div className="relative h-[400px] w-[600px] border bg-base">
      <Panel {...args} />
    </div>
  ),
};

export const BottomPanelStory: Story = {
  name: "Bottom Panel",
  args: {
    visible: true,
    sections: [
      { id: "console", content: <div className="p-2 font-mono text-xs">$ npm run build\n✓ Built successfully</div>, defaultOpen: true, order: 0 },
      { id: "problems", content: <div className="p-2 text-sm">No problems detected.</div>, defaultOpen: false, order: 1 },
    ],
    size: 200,
  },
  render: (args) => (
    <div className="relative h-[400px] w-full border bg-base flex flex-col">
      <div className="flex-1" />
      <BottomPanel {...args} />
    </div>
  ),
};

const leafTab = (id: string, icon: ComponentType<{ size?: number }>, name: string, order: number, content: string) => ({
  kind: "leaf" as const,
  id,
  icon,
  name,
  order,
  tree: { sections: [{ id: `${id}.section`, label: "", items: [{ id: `${id}.item`, label: "", control: <div className="p-2">{content}</div> }] }] },
});

export const SidePanelStory: Story = {
  name: "Side Panel",
  args: {
    visible: true,
    sections: sampleSections,
    size: 250,
  },
  render: () => {
    const [size, setSize] = useState(300);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <SidePanel
          position="left"
          size={size}
          onSizeChange={setSize}
          tabs={[leafTab("types", Layers, "Types", 0, "Types panel content"), leafTab("settings", Settings, "Settings", 1, "Settings panel content"), leafTab("info", Info, "Info", 2, "Info panel content")]}
        />
      </div>
    );
  },
};

export const SidePanelRight: Story = {
  name: "Side Panel Right",
  args: {
    visible: true,
    sections: sampleSections,
    size: 250,
  },
  render: () => {
    const [size, setSize] = useState(300);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <SidePanel position="right" size={size} onSizeChange={setSize} tabs={[leafTab("properties", Info, "Properties", 0, "Properties content"), leafTab("layers", Layers, "Layers", 1, "Layers content")]} />
      </div>
    );
  },
};

export const SidePanelNestedTabs: Story = {
  name: "Side Panel — Nested Tabs (Ribbon Levels)",
  args: {
    visible: true,
    sections: sampleSections,
    size: 280,
  },
  render: () => {
    const [size, setSize] = useState(320);
    const [activeTabPath, setActiveTabPath] = useState<readonly string[]>([]);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <SidePanel
          position="left"
          size={size}
          onSizeChange={setSize}
          activeTabPath={activeTabPath}
          onActiveTabPathChange={setActiveTabPath}
          tabs={[
            {
              kind: "branch",
              id: "workbench",
              icon: Layers,
              name: "Workbench",
              order: 0,
              children: [leafTab("document", Info, "Document", 0, "Document tab content"), leafTab("catalogue", Layers, "Catalogue", 1, "Catalogue tab content")],
            },
            {
              kind: "branch",
              id: "display",
              icon: Settings,
              name: "Display",
              order: 1,
              children: [leafTab("windows", Settings, "Windows", 0, "Windows tab content")],
            },
          ]}
        />
      </div>
    );
  },
};

// #endregion 🦉Panel
