// #region 🧲Header

// .elements/ui/.storybook/stories/elements/display/DiagramNode.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲Header

import { DiagramNode, DiagramSkeleton } from "@ui/react";
import type { Meta, StoryObj } from "@storybook/react";

// #region 🔓DiagramNode

const meta = {
  title: "elements/react/DiagramNode",
  component: DiagramNode,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof DiagramNode>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    content: "Node Label",
  },
};

export const States: Story = {
  args: { content: "" },
  render: () => (
    <div className="flex items-center gap-4 p-4">
      <DiagramNode content="Default" />
      <DiagramNode content="Selected" selected />
      <DiagramNode content="Hovered" hovered />
      <DiagramNode content="Placeholder" isPlaceholder />
      <DiagramNode content="Clickable" onClick={() => {}} />
    </div>
  ),
};

export const DiagramSkeletonStory: Story = {
  name: "DiagramSkeleton",
  args: { content: "" },
  render: () => (
    <div className="h-[400px] w-full">
      <DiagramSkeleton nodeCount={6} edgeCount={5} />
    </div>
  ),
};

// #endregion 🔓DiagramNode
