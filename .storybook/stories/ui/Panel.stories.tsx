// #region 🧲Header

// .elements/ui/.storybook/story/elements/Panel.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { Panel, PanelChromeTabBar, singleTreeLeaf } from "@semio-tech/ui-react";
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
        <Panel anchor="bottom-right" visible={visible} onVisibleChange={setVisible} size={size} onSizeChange={setSize} tabs={[leafTab("actions", Layers, "Actions", 0, "Actions"), leafTab("history", Info, "History", 1, "History")]} />
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

function ChromeHostedPanelDemo({ anchor }: { readonly anchor: "top-left" | "top-middle" | "top-right" | "bottom-left" | "bottom-middle" | "bottom-right" }) {
  const [size, setSize] = useState(320);
  const [visible, setVisible] = useState(false);
  const [activeTabPath, setActiveTabPath] = useState<readonly string[]>([]);
  const tabs: Parameters<typeof Panel>[0]["tabs"] = [
    {
      kind: "branch",
      id: "category",
      icon: Layers,
      name: "Category",
      order: 0,
      children: [leafTab("document", Info, "Document", 0, "Document body"), leafTab("catalogue", Layers, "Catalogue", 1, "Catalogue body")],
    },
    leafTab("settings", Settings, "Settings", 1, "Settings body"),
  ];
  const selection = { tabs, visible, onVisibleChange: setVisible, activeTabPath, onActiveTabPathChange: setActiveTabPath };
  const chromeBar = <PanelChromeTabBar anchor={anchor} {...selection} />;
  return (
    <div className="relative flex h-[420px] w-[720px] flex-col border bg-window">
      {(anchor === "top-left" || anchor === "top-middle" || anchor === "top-right") && (
        <div className="flex h-large shrink-0 items-center gap-single border-b bg-window p-single">
          {anchor === "top-left" ? chromeBar : <span className="text-xs text-muted-foreground">Navbar</span>}
          {anchor === "top-middle" ? <div className="mx-auto">{chromeBar}</div> : null}
          {anchor === "top-right" ? <div className="ms-auto">{chromeBar}</div> : null}
        </div>
      )}
      <div className="relative min-h-0 flex-1 bg-canvas">
        <Panel anchor={anchor} tabBarHost="chrome" size={size} onSizeChange={setSize} {...selection} />
      </div>
      {(anchor === "bottom-left" || anchor === "bottom-middle" || anchor === "bottom-right") && (
        <div className="flex h-large shrink-0 items-center gap-single border-t bg-window p-single">
          {anchor === "bottom-left" ? chromeBar : null}
          {anchor === "bottom-middle" ? <div className="mx-auto">{chromeBar}</div> : null}
          {anchor === "bottom-right" ? <div className="ms-auto">{chromeBar}</div> : null}
        </div>
      )}
    </div>
  );
}

export const PanelChromeHostedTopMiddle: Story = {
  name: "Panel — Chrome Hosted Top Middle",
  render: () => <ChromeHostedPanelDemo anchor="top-middle" />,
};

export const PanelChromeHostedBottomMiddle: Story = {
  name: "Panel — Chrome Hosted Bottom Middle (nested tabs)",
  render: () => <ChromeHostedPanelDemo anchor="bottom-middle" />,
};

// #endregion 🧭Panel
