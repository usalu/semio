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
import Cursor from "./Icons";

const meta = {
  title: "Elements/Icons",
  component: Cursor,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Cursor>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Variants: Story = {
  render: () => (
    <div className="flex gap-8">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Default</p>
        <div className="relative w-64 h-64 border rounded-md bg-muted/20">
          <Cursor color="#000000" x={50} y={50} />
        </div>
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Color Variants</p>
        <div className="relative w-64 h-64 border rounded-md bg-muted/20">
          <Cursor color="#FF0000" x={20} y={20} />
          <Cursor color="#00FF00" x={80} y={80} />
          <Cursor color="#0000FF" x={140} y={140} />
        </div>
      </div>
    </div>
  ),
};

export const DefaultCursor: Story = {
  render: () => (
    <div className="relative w-64 h-64 border rounded-md bg-muted/20">
      <Cursor color="#000000" x={50} y={50} />
    </div>
  ),
};

export const ColorVariants: Story = {
  render: () => (
    <div className="relative w-64 h-64 border rounded-md bg-muted/20">
      <Cursor color="#FF0000" x={20} y={20} />
      <Cursor color="#00FF00" x={80} y={80} />
      <Cursor color="#0000FF" x={140} y={140} />
    </div>
  ),
};

export const MultipleCursors: Story = {
  render: () => (
    <div className="relative w-96 h-96 border rounded-md bg-muted/20">
      <Cursor color="#FF6B6B" x={50} y={50} />
      <Cursor color="#4ECDC4" x={150} y={100} />
      <Cursor color="#45B7D1" x={250} y={150} />
      <Cursor color="#FFA07A" x={100} y={200} />
      <Cursor color="#98D8C8" x={200} y={250} />
    </div>
  ),
};

export const Positioned: Story = {
  render: () => (
    <div className="relative w-64 h-64 border rounded-md bg-muted/20">
      <Cursor color="#8B5CF6" x={100} y={120} />
      <div className="absolute left-24 top-32 ml-6 mt-1 bg-background border rounded px-2 py-1 text-xs">User A</div>
    </div>
  ),
};

export const WithLabels: Story = {
  render: () => (
    <div className="relative w-96 h-64 border rounded-md bg-muted/20">
      <Cursor color="#EF4444" x={50} y={50} />
      <div className="absolute left-12 top-12 ml-6 mt-1 bg-red-500 text-white rounded px-2 py-1 text-xs">Alice</div>

      <Cursor color="#10B981" x={200} y={100} />
      <div className="absolute left-48 top-24 ml-6 mt-1 bg-green-500 text-white rounded px-2 py-1 text-xs">Bob</div>

      <Cursor color="#3B82F6" x={100} y={150} />
      <div className="absolute left-24 top-36 ml-6 mt-1 bg-blue-500 text-white rounded px-2 py-1 text-xs">Charlie</div>
    </div>
  ),
};
