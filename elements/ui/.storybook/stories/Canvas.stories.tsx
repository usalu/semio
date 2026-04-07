// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/Canvas.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖Header

import { Canvas, DiagramNode, DiagramSkeleton, HorizontalWindows, Window } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";

// #region 🔖Canvas
const meta = {
  title: "elements/Canvas",
  component: Canvas,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Canvas>;

export default meta;

type Story = StoryObj<typeof meta>;

const WindowContent = ({ title, color = "bg-panel" }: { title: string; color?: string }) => (
  <div className={`flex items-center justify-center h-full ${color}`}>
    <h2 className="text-2xl font-bold">{title}</h2>
  </div>
);

export const Default: Story = {
  args: {
    children: (
      <HorizontalWindows>
        <Window id="left" defaultSize={50}>
          <WindowContent title="Left Window" color="bg-panel" />
        </Window>
        <Window id="right" defaultSize={50}>
          <WindowContent title="Right Window" color="bg-base" />
        </Window>
      </HorizontalWindows>
    ),
  },
  render: (args) => (
    <div className="h-screen">
      <Canvas {...args} />
    </div>
  ),
};

// #endregion 🔖Canvas

// #region 🔖DiagramNode
export const DiagramNodeDefault: Story = {
  args: { children: null },
  render: () => (
    <div className="flex items-center gap-4 p-8">
      <DiagramNode content="Capsule J" />
      <DiagramNode content="Selected" selected />
      <DiagramNode content="Hovered" hovered />
      <DiagramNode content="Placeholder" isPlaceholder />
      <DiagramNode content="Clickable" onClick={() => {}} />
    </div>
  ),
};
// #endregion 🔖DiagramNode

// #region 🔖DiagramSkeleton
export const DiagramSkeletonDefault: Story = {
  args: { children: null },
  render: () => (
    <div className="h-[400px] w-full">
      <DiagramSkeleton nodeCount={5} edgeCount={4} />
    </div>
  ),
};
// #endregion 🔖DiagramSkeleton
