// #region 🧲Header

// .elements/ui/.storybook/story/elements/Panel.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { Panel, singleTreeLeaf } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState, type ComponentType } from "react";

// #region 🧭Panel

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

const leafTab = (id: string, icon: ComponentType<{ size?: number }>, name: string, order: number, content: string) =>
  singleTreeLeaf({ id, icon, name, order, tree: { sections: [{ id: `${id}.section`, label: "", items: [{ id: `${id}.item`, label: "", control: <div className="p-2">{content}</div> }] }] } });

// The toggle is the panel's own chrome (its first row) — same fold/unfold pattern as a window's options
// rail: the toggle always renders and stays put, while the tab bar and content only mount when open.
export const PanelTopLeft: Story = {
  name: "Panel — Top Left",
  render: () => {
    const [size, setSize] = useState(300);
    const [visible, setVisible] = useState(true);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <Panel
          anchor="top-left"
          visible={visible}
          onVisibleChange={setVisible}
          size={size}
          onSizeChange={setSize}
          tabs={[leafTab("types", Layers, "Types", 0, "Types panel content"), leafTab("settings", Settings, "Settings", 1, "Settings panel content"), leafTab("info", Info, "Info", 2, "Info panel content")]}
        />
      </div>
    );
  },
};

export const PanelTopRight: Story = {
  name: "Panel — Top Right",
  render: () => {
    const [size, setSize] = useState(300);
    const [visible, setVisible] = useState(true);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <Panel
          anchor="top-right"
          visible={visible}
          onVisibleChange={setVisible}
          size={size}
          onSizeChange={setSize}
          tabs={[leafTab("properties", Info, "Properties", 0, "Properties content"), leafTab("layers", Layers, "Layers", 1, "Layers content")]}
        />
      </div>
    );
  },
};

export const PanelBottomRight: Story = {
  name: "Panel — Bottom Right (stacks upward)",
  render: () => {
    const [size, setSize] = useState(300);
    const [visible, setVisible] = useState(true);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <Panel anchor="bottom-right" visible={visible} onVisibleChange={setVisible} size={size} onSizeChange={setSize} tabs={[leafTab("actions", Layers, "Actions", 0, "Action tools"), leafTab("history", Info, "History", 1, "History tools")]} />
      </div>
    );
  },
};

export const PanelFolded: Story = {
  name: "Panel — Folded (toggle only)",
  render: () => {
    const [visible, setVisible] = useState(false);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <Panel anchor="top-left" visible={visible} onVisibleChange={setVisible} tabs={[leafTab("types", Layers, "Types", 0, "Types panel content")]} />
      </div>
    );
  },
};

export const PanelNestedTabs: Story = {
  name: "Panel — Nested Tabs (Ribbon Levels)",
  render: () => {
    const [size, setSize] = useState(320);
    const [visible, setVisible] = useState(true);
    const [activeTabPath, setActiveTabPath] = useState<readonly string[]>([]);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <Panel
          anchor="top-left"
          visible={visible}
          onVisibleChange={setVisible}
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

// Middle panels center on the navbar/footer, grow both left and right, and are resizable from either edge — in
// contrast to a corner panel, which grows in one horizontal direction and resizes only on its single inner edge.
export const PanelTopMiddle: Story = {
  name: "Panel — Top Middle (centered, dual resize handles)",
  render: () => {
    const [size, setSize] = useState(360);
    const [visible, setVisible] = useState(true);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <Panel
          anchor="top-middle"
          visible={visible}
          onVisibleChange={setVisible}
          size={size}
          onSizeChange={setSize}
          tabs={[leafTab("search", Info, "Search", 0, "Centered search content"), leafTab("filters", Settings, "Filters", 1, "Filter content")]}
        />
      </div>
    );
  },
};

export const PanelBottomMiddle: Story = {
  name: "Panel — Bottom Middle (centered, stacks upward)",
  render: () => {
    const [size, setSize] = useState(360);
    const [visible, setVisible] = useState(true);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <Panel anchor="bottom-middle" visible={visible} onVisibleChange={setVisible} size={size} onSizeChange={setSize} tabs={[leafTab("status", Layers, "Status", 0, "Status content")]} />
      </div>
    );
  },
};

// #endregion 🧭Panel
