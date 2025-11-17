// #region Header

// Icons.stories.tsx

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
import { Cursor } from "../../../../sketchpad/elements";

// #region Icons
const meta = {
  title: "Elements/Display/Icons",
  component: Cursor,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Cursor>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: { color: "#000000" },
  render: () => (
    <div className="relative w-96 h-96 border bg-muted/10 p-4">
      <p className="text-sm font-medium mb-4">Collaborative Cursors</p>
      <div className="relative w-full h-full">
        <Cursor color="var(--accent)" x={120} y={80} />
        <div className="absolute left-28 top-20 ml-6 mt-1 bg-accent text-active-foreground border px-double py-1 text-xs">Alice</div>
        <Cursor color="var(--status-success)" x={200} y={150} />
        <div className="absolute left-48 top-36 ml-6 mt-1 bg-status-success text-active-foreground border px-double py-1 text-xs">Bob</div>
        <Cursor color="var(--status-warning)" x={280} y={200} />
        <div className="absolute left-68 top-small ml-6 mt-1 bg-status-warning text-active-foreground border px-double py-1 text-xs">Charlie</div>
        <Cursor color="var(--status-danger)" x={100} y={250} />
        <Cursor color="var(--accent-secondary)" x={250} y={300} />
      </div>
    </div>
  ),
};

// #endregion Icons
