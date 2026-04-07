// #region 🧲Header

// .elements/ui/.storybook/stories/elements/Panel.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { BottomPanel, Panel, PanelSection, SidePanel } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";
import { Info, Layers, Settings } from "lucide-react";
import { useState } from "react";

// #region 🦉Panel

const meta = {
  title: "elements/Panel",
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
          tabs={[
            { id: "types", icon: Layers, order: 0, content: <div className="p-2">Types panel content</div> },
            { id: "settings", icon: Settings, order: 1, content: <div className="p-2">Settings panel content</div> },
            { id: "info", icon: Info, order: 2, content: <div className="p-2">Info panel content</div> },
          ]}
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
        <SidePanel
          position="right"
          size={size}
          onSizeChange={setSize}
          tabs={[
            { id: "properties", icon: Info, order: 0, content: <div className="p-2">Properties content</div> },
            { id: "layers", icon: Layers, order: 1, content: <div className="p-2">Layers content</div> },
          ]}
        />
      </div>
    );
  },
};

// #endregion 🦉Panel
