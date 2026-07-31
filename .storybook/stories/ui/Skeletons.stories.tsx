// #region 🧲️Header

// 🥼️ .storybook/stories/ui/Skeletons.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import { createIconComponent, DiagramSkeleton, LoadingRow, SceneSkeleton, TableSkeleton, type TableColumn } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
// #endregion 🔌️Adapters

// 💀️#region 💀️Skeletons
// One file for every loading-placeholder component in the barrel — none of these are interactive, so
// there is little value in four separate one-story files; grouped here per the ticket's explicit scope.
const FileIcon = createIconComponent("file-text");

const skeletonColumns: TableColumn[] = [
  { id: "name", header: "Name", accessor: () => null, width: "50%" },
  { id: "role", header: "Role", accessor: () => null, width: "30%" },
  { id: "status", header: "Status", accessor: () => null },
];

const meta = {
  title: "🖱️ui⚛️react/Skeletons",
  component: TableSkeleton,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof TableSkeleton>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Table: Story = {
  name: "TableSkeleton",
  render: () => (
    <div className="h-64 w-full">
      <TableSkeleton columns={skeletonColumns} rowCount={5} />
    </div>
  ),
};

export const Diagram: Story = {
  name: "DiagramSkeleton",
  render: () => (
    <div className="h-64 w-full">
      <DiagramSkeleton nodeCount={5} edgeCount={4} />
    </div>
  ),
};

export const LoadingRowStory: Story = {
  name: "LoadingRow",
  render: () => (
    <div className="flex w-64 flex-col gap-single">
      <LoadingRow name="Loading document.json…" icon={<FileIcon size={12} />} />
      <LoadingRow name="Loading kit.json…" icon={<FileIcon size={12} />} />
    </div>
  ),
};

export const Scene: Story = {
  name: "SceneSkeleton",
  render: () => (
    <div className="h-64 w-full">
      <SceneSkeleton />
    </div>
  ),
};
// #endregion 💀️Skeletons
