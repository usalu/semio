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
import { Canvas, HorizontalWindows } from "../../../sketchpad/App";
import { Window } from "../../../sketchpad/elements";

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
  render: () => (
    <div className="h-screen">
      <Canvas>
        <HorizontalWindows>
          <Window id="left" defaultSize={50}>
            <WindowContent title="Left Window" color="bg-panel" />
          </Window>
          <Window id="right" defaultSize={50}>
            <WindowContent title="Right Window" color="bg-base" />
          </Window>
        </HorizontalWindows>
      </Canvas>
    </div>
  ),
};

// #endregion Canvas
