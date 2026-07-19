// #region 🧲Header

// 🥼︎ .storybook/stories/ui/SelectionMarquee.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🔌Adapters
import { SelectionMarquee } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
// #endregion 🔌Adapters

// ⬚#region 🔖SelectionMarquee
const meta = {
  title: "🖱️ui⚛️react/SelectionMarquee",
  component: SelectionMarquee,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof SelectionMarquee>;

export default meta;

type Story = StoryObj<typeof meta>;

export const RectFull: Story = {
  name: 'shape="rect", coverage="full" (drag left-to-right)',
  render: () => (
    <div className="relative h-64 w-96 border bg-canvas">
      <SelectionMarquee shape="rect" coverage="full" rect={{ x: 40, y: 30, width: 220, height: 140 }} />
    </div>
  ),
};

export const RectPartial: Story = {
  name: 'shape="rect", coverage="partial" (drag right-to-left — dashed)',
  render: () => (
    <div className="relative h-64 w-96 border bg-canvas">
      <SelectionMarquee shape="rect" coverage="partial" rect={{ x: 40, y: 30, width: 220, height: 140 }} />
    </div>
  ),
};

export const Polygon: Story = {
  name: 'shape="polygon" (lasso gesture)',
  render: () => (
    <div className="relative h-64 w-96 border bg-canvas">
      <SelectionMarquee
        shape="polygon"
        coverage="full"
        points={[
          { x: 40, y: 160 },
          { x: 120, y: 30 },
          { x: 260, y: 50 },
          { x: 300, y: 150 },
          { x: 180, y: 190 },
        ]}
      />
    </div>
  ),
};
// #endregion 🔖SelectionMarquee
