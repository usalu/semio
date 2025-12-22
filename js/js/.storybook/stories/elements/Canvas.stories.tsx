// #region Header

// js/js/.storybook/stories/elements/Canvas.stories.tsx

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

// #endregion Header

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
import { Canvas, HorizontalWindows } from "../../../sketchpad/Sketchpad";
import { Level, LevelProvider, Window, getLevelBgClass } from "../../../sketchpad/elements";

// #region Canvas
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

const CanvasDemo = () => (
  <HorizontalWindows>
    <Window id="left" defaultSize={50}>
      <WindowContent title="Left Window" color="bg-panel" />
    </Window>
    <Window id="right" defaultSize={50}>
      <WindowContent title="Right Window" color="bg-base" />
    </Window>
  </HorizontalWindows>
);

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`h-screen ${getLevelBgClass(level)}`}>
      <Canvas>
        <CanvasDemo />
      </Canvas>
    </div>
  </LevelProvider>
);

export const Base: Story = {
  render: createLevelRender("base"),
};

export const Window_: Story = {
  name: "Window",
  render: createLevelRender("window"),
};

export const Panel: Story = {
  render: createLevelRender("panel"),
};

export const Overlay: Story = {
  render: createLevelRender("overlay"),
};

export const Temporary: Story = {
  render: createLevelRender("temporary"),
};

// #endregion Canvas
