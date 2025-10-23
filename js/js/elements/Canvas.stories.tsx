// #region Header

// Canvas.stories.tsx

// 2025 Ueli Saluz

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

// #endregion

import type { Meta, StoryObj } from "@storybook/react";
import { Canvas, HorizontalWindows, VerticalWindows, Window } from "./Canvas";

const meta = {
  title: "Elements/Canvas",
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

export const SingleWindow: Story = {
  render: () => (
    <div className="h-screen">
      <Canvas>
        <Window id="main">
          <WindowContent title="Single Window" />
        </Window>
      </Canvas>
    </div>
  ),
};

export const HorizontalSplit: Story = {
  render: () => (
    <div className="h-screen">
      <Canvas>
        <HorizontalWindows
          windows={[
            { id: "left", children: <WindowContent title="Left Window" color="bg-panel" />, defaultSize: 50 },
            { id: "right", children: <WindowContent title="Right Window" color="bg-base" />, defaultSize: 50 },
          ]}
        />
      </Canvas>
    </div>
  ),
};

export const VerticalSplit: Story = {
  render: () => (
    <div className="h-screen">
      <Canvas>
        <VerticalWindows
          windows={[
            { id: "top", children: <WindowContent title="Top Window" color="bg-panel" />, defaultSize: 60 },
            { id: "bottom", children: <WindowContent title="Bottom Window" color="bg-base" />, defaultSize: 40 },
          ]}
        />
      </Canvas>
    </div>
  ),
};

export const ThreeHorizontal: Story = {
  render: () => (
    <div className="h-screen">
      <Canvas>
        <HorizontalWindows
          windows={[
            { id: "left", children: <WindowContent title="Left" color="bg-panel" />, defaultSize: 33 },
            { id: "center", children: <WindowContent title="Center" color="bg-base" />, defaultSize: 34 },
            { id: "right", children: <WindowContent title="Right" color="bg-panel" />, defaultSize: 33 },
          ]}
        />
      </Canvas>
    </div>
  ),
};
