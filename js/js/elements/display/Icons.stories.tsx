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
  render: () => (
    <div className="relative w-96 h-96 border bg-muted/10 p-4">
      <p className="text-sm font-medium mb-4">Collaborative Cursors</p>
      <div className="relative w-full h-full">
        <Cursor color="var(--accent)" x={120} y={80} />
        <Cursor color="var(--status-success)" x={200} y={150} />
        <Cursor color="var(--status-warning)" x={280} y={200} />
      </div>
    </div>
  ),
};

export const Variants: Story = {
  render: () => (
    <div className="flex gap-8">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Default</p>
        <div className="relative w-64 h-64 border bg-muted/20">
          <Cursor color="var(--foreground)" x={50} y={50} />
        </div>
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Color Variants</p>
        <div className="relative w-64 h-64 border bg-muted/20">
          <Cursor color="var(--status-danger)" x={20} y={20} />
          <Cursor color="var(--status-success)" x={80} y={80} />
          <Cursor color="var(--accent-secondary)" x={140} y={140} />
        </div>
      </div>
    </div>
  ),
};

export const DefaultCursor: Story = {
  render: () => (
    <div className="relative w-64 h-64 border bg-muted/20">
      <Cursor color="var(--foreground)" x={50} y={50} />
    </div>
  ),
};

export const ColorVariants: Story = {
  render: () => (
    <div className="relative w-64 h-64 border bg-muted/20">
      <Cursor color="var(--status-danger)" x={20} y={20} />
      <Cursor color="var(--status-success)" x={80} y={80} />
      <Cursor color="var(--accent-secondary)" x={140} y={140} />
    </div>
  ),
};

export const MultipleCursors: Story = {
  render: () => (
    <div className="relative w-96 h-96 border bg-muted/20">
      <Cursor color="var(--accent)" x={50} y={50} />
      <Cursor color="var(--accent-secondary)" x={150} y={100} />
      <Cursor color="var(--status-info)" x={250} y={150} />
      <Cursor color="var(--accent-tertiary)" x={100} y={200} />
      <Cursor color="color-mix(in srgb, var(--status-success) 60%, var(--base) 40%)" x={200} y={250} />
    </div>
  ),
};

export const Positioned: Story = {
  render: () => (
    <div className="relative w-64 h-64 border bg-muted/20">
      <Cursor color="color-mix(in srgb, var(--accent) 60%, var(--foreground) 40%)" x={100} y={120} />
      <div className="absolute left-24 top-32 ml-6 mt-1 border px-2 py-1 text-xs">User A</div>
    </div>
  ),
};

export const WithLabels: Story = {
  render: () => (
    <div className="relative w-96 h-64 border bg-muted/20">
      <Cursor color="var(--status-danger)" x={50} y={50} />
      <div className="absolute left-12 top-12 ml-6 mt-1 bg-status-danger text-active-foreground border px-2 py-1 text-xs">Alice</div>

      <Cursor color="var(--status-success)" x={200} y={100} />
      <div className="absolute left-48 top-24 ml-6 mt-1 bg-status-success text-active-foreground border px-2 py-1 text-xs">Bob</div>

      <Cursor color="var(--accent-secondary)" x={100} y={150} />
      <div className="absolute left-24 top-36 ml-6 mt-1 bg-accent-secondary text-active-foreground border px-2 py-1 text-xs">Charlie</div>
    </div>
  ),
};
