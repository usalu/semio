// #region 🧲Header

// .elements/ui/.storybook/story/elements/window/Window.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { ButtonGroup, ButtonGroupItem, Canvas, createIconComponent, HorizontalWindows, Ribbon, ToggleGroup, ToolbarGroup, ToolbarItem, ToolbarZone, VerticalWindows, Window, type RibbonRow } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";

// #region 🌊Window

const meta = {
  title: "🖱️ui⚛️react/Window",
  component: Window,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Window>;

export default meta;

type Story = StoryObj<typeof meta>;

const WindowContent = ({ title }: { title: string }) => (
  <div className="flex items-center justify-center h-full">
    <h2 className="text-xl font-bold">{title}</h2>
  </div>
);

export const Default: Story = {
  args: {
    id: "default-window",
    children: <WindowContent title="Window Content" />,
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} fill />
    </div>
  ),
};

export const WithControls: Story = {
  args: {
    id: "controls-window",
    children: <WindowContent title="Window with Controls" />,
    showControls: true,
    onMaximize: () => {},
    onMinimize: () => {},
    onClose: () => {},
    onOpenInNewWindow: () => {},
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} fill />
    </div>
  ),
};

export const Loading: Story = {
  args: {
    id: "loading-window",
    children: null,
    loading: true,
    skeleton: (
      <div className="flex items-center justify-center h-full animate-pulse">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    ),
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} fill />
    </div>
  ),
};

export const WithError: Story = {
  args: {
    id: "error-window",
    children: null,
    error: new Error("Something went wrong while loading the content."),
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} fill />
    </div>
  ),
};

export const WithEngagement: Story = {
  args: {
    id: "engagement-window",
    active: true,
    children: <WindowContent title="Window with Engagement" />,
    engagement: {
      options: [{ id: "opt-grid", label: "Grid", onPress: () => {} }],
      input: { placeholder: "Action…" },
      status: [{ id: "status", content: "Ready" }],
    },
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} fill />
    </div>
  ),
};

export const WithToolbar: Story = {
  args: {
    id: "toolbar-window",
    children: <WindowContent title="Window with Toolbar" />,
    fill: true,
    toolbar: (
      <ToolbarZone>
        <ToolbarItem>Select</ToolbarItem>
        <ToolbarItem>Move</ToolbarItem>
        <ToolbarItem>Extrude</ToolbarItem>
      </ToolbarZone>
    ),
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} fill />
    </div>
  ),
};

export const NoTools: Story = {
  name: "No Tools (bottom-left chrome still present, disabled)",
  args: {
    id: "no-tools-window",
    children: <WindowContent title="Window without Tools" />,
    fill: true,
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} fill />
    </div>
  ),
};

const MousePointer = createIconComponent("mouse-pointer");
const Wrench = createIconComponent("wrench");
const Sparkles = createIconComponent("sparkles");
const Move = createIconComponent("move");
const RotateCw = createIconComponent("rotate-cw");
const Maximize2 = createIconComponent("maximize2");

type CategoryDemoLeaf = { readonly id: string; readonly icon: typeof MousePointer };
type CategoryDemoNode = { readonly id: string; readonly label: string; readonly icon: typeof MousePointer } & (
  | { readonly kind: "leaves"; readonly leaves: readonly CategoryDemoLeaf[] }
  | { readonly kind: "group"; readonly children: readonly CategoryDemoNode[] }
);

const WINDOW_CATEGORY_DEMO_TREE: readonly CategoryDemoNode[] = [
  { id: "selection", label: "Selection", icon: MousePointer, kind: "leaves", leaves: [{ id: "direct", icon: MousePointer }] },
  {
    id: "tools",
    label: "Tools",
    icon: Wrench,
    kind: "group",
    children: [{ id: "transform", label: "Transform", icon: Move, kind: "leaves", leaves: [{ id: "move", icon: Move }, { id: "rotate", icon: RotateCw }, { id: "scale", icon: Maximize2 }] }],
  },
  { id: "actions", label: "Actions", icon: Sparkles, kind: "leaves", leaves: [{ id: "run", icon: Sparkles }] },
];

/** @emoji 🗂️ Same at-most-one-active-per-level recursion as the "Recursive Category Groups" Toolbar story, sized for a window's bottom-left toolbar slot. */
function buildWindowCategoryRows(tree: readonly CategoryDemoNode[], activePath: readonly string[], onActivate: (depth: number, value: string) => void): RibbonRow[] {
  const rows: RibbonRow[] = [];
  let level = tree;
  let depth = 0;
  while (true) {
    rows.push({
      key: `picker-${depth}`,
      content: (
        <ToolbarZone>
          <ToolbarGroup>
            <ToolbarItem>
              <ToggleGroup
                kind="single"
                value={activePath[depth] ?? ""}
                onValueChange={(value) => onActivate(depth, value)}
                items={level.map((node) => ({ value: node.id, id: `ui.toolbar.demo-window.group.${node.id}`, icon: <node.icon className="size-tiny" aria-hidden />, text: node.label }))}
              />
            </ToolbarItem>
          </ToolbarGroup>
        </ToolbarZone>
      ),
    });
    const active = level.find((node) => node.id === activePath[depth]);
    if (!active) break;
    if (active.kind === "leaves") {
      rows.push({
        key: `leaves-${depth}`,
        content: (
          <ToolbarZone>
            <ToolbarGroup>
              <ToolbarItem>
                <ButtonGroup>
                  {active.leaves.map((leaf) => (
                    <ButtonGroupItem key={leaf.id} icon={<leaf.icon className="size-tiny" aria-hidden />} />
                  ))}
                </ButtonGroup>
              </ToolbarItem>
            </ToolbarGroup>
          </ToolbarZone>
        ),
      });
      break;
    }
    level = active.children;
    depth += 1;
  }
  return rows;
}

const WindowWithRecursiveCategoryToolbar = () => {
  const [activePath, setActivePath] = useState<readonly string[]>([]);
  const onActivate = (depth: number, value: string) => {
    setActivePath((previous) => (value ? [...previous.slice(0, depth), value] : previous.slice(0, depth)));
  };
  return (
    <div className="h-[400px] w-[600px]">
      <Window id="category-toolbar-window" fill toolbar={<Ribbon id="ui.toolbar.demo-window" direction="up" rows={buildWindowCategoryRows(WINDOW_CATEGORY_DEMO_TREE, activePath, onActivate)} />}>
        <WindowContent title="Window with Recursive Category Toolbar" />
      </Window>
    </div>
  );
};

export const WithRecursiveCategoryToolbar: Story = {
  name: "With Recursive Category Toolbar (selection / tools / actions)",
  args: { id: "category-toolbar-window", children: null },
  render: () => <WindowWithRecursiveCategoryToolbar />,
};

export const WithControlsMeasuresEngagementAndToolbar: Story = {
  args: {
    id: "full-chrome-window",
    active: true,
    children: <WindowContent title="Every Rail at Once" />,
    fill: true,
    showControls: true,
    onMaximize: () => {},
    onClose: () => {},
    measures: <div className="p-tiny text-sm">LOD 2</div>,
    engagement: {
      options: [{ id: "opt-grid", label: "Grid", onPress: () => {} }],
      input: { placeholder: "Action…" },
      status: [{ id: "status", content: "Ready" }],
    },
    toolbar: (
      <ToolbarZone>
        <ToolbarItem>Select</ToolbarItem>
        <ToolbarItem>Move</ToolbarItem>
        <ToolbarItem>Extrude</ToolbarItem>
      </ToolbarZone>
    ),
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} fill />
    </div>
  ),
};

export const HorizontalLayout: Story = {
  args: { id: "h-layout", children: null },
  render: () => (
    <div className="h-[400px] w-full">
      <Canvas>
        <HorizontalWindows>
          <Window id="left" defaultSize={50}>
            <WindowContent title="Left" />
          </Window>
          <Window id="center" defaultSize={25}>
            <WindowContent title="Center" />
          </Window>
          <Window id="right" defaultSize={25}>
            <WindowContent title="Right" />
          </Window>
        </HorizontalWindows>
      </Canvas>
    </div>
  ),
};

export const VerticalLayout: Story = {
  args: { id: "v-layout", children: null },
  render: () => (
    <div className="h-[400px] w-full">
      <Canvas>
        <VerticalWindows>
          <Window id="top" defaultSize={50}>
            <WindowContent title="Top" />
          </Window>
          <Window id="bottom" defaultSize={50}>
            <WindowContent title="Bottom" />
          </Window>
        </VerticalWindows>
      </Canvas>
    </div>
  ),
};

export const NestedLayout: Story = {
  args: { id: "nested", children: null },
  render: () => (
    <div className="h-[500px] w-full">
      <Canvas>
        <HorizontalWindows>
          <Window id="left" defaultSize={40}>
            <WindowContent title="Left" />
          </Window>
          <Window id="right" defaultSize={60}>
            <VerticalWindows>
              <Window id="top-right" defaultSize={60}>
                <WindowContent title="Top Right" />
              </Window>
              <Window id="bottom-right" defaultSize={40}>
                <WindowContent title="Bottom Right" />
              </Window>
            </VerticalWindows>
          </Window>
        </HorizontalWindows>
      </Canvas>
    </div>
  ),
};

// #endregion 🌊Window
