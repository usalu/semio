// #region 🧲️Header

// 🥼️ .storybook/stories/ui/Table.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { Table, TableAvatar, TableColumn, TableSkeleton } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";

// 📊️#region 🛎️Table
const FileCode = createIconComponent("file-code");
const FileImage = createIconComponent("file-image");
const FolderIcon = createIconComponent("folder");

const meta = {
  title: "🖱️ui⚛️react/Table",
  component: Table,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Table>;

export default meta;

type Story = StoryObj<typeof meta>;

import { architects } from "../../fixture/nakagin";

interface Person {
  id: string;
  name: string;
  icon?: string;
  role: string;
  email: string;
}

const sampleData: Person[] = architects.map((a) => ({ ...a }));

export const WithAvatars: Story = {
  args: { columns: [], data: [] },
  render: () => {
    const columns: TableColumn<Person>[] = [
      {
        id: "name",
        header: "Name",
        accessor: (row) => (
          <div className="flex items-center gap-double">
            <TableAvatar name={row.name} icon={row.icon} />
            <span>{row.name}</span>
          </div>
        ),
        width: "35%",
      },
      {
        id: "role",
        header: "Role",
        accessor: (row) => row.role,
        width: "30%",
      },
      {
        id: "email",
        header: "Email",
        accessor: (row) => row.email,
        width: "35%",
      },
    ];

    return (
      <div className="h-96">
        <Table columns={columns} data={sampleData} getRowId={(row) => row.id} />
      </div>
    );
  },
};

export const SelectableWithAvatars: Story = {
  args: { columns: [], data: [] },
  render: () => {
    const columns: TableColumn<Person>[] = [
      {
        id: "name",
        header: "Name",
        accessor: (row) => (
          <div className="flex items-center gap-double">
            <TableAvatar name={row.name} icon={row.icon} />
            <span>{row.name}</span>
          </div>
        ),
        width: "35%",
      },
      {
        id: "role",
        header: "Role",
        accessor: (row) => row.role,
        width: "30%",
      },
      {
        id: "email",
        header: "Email",
        accessor: (row) => row.email,
        width: "35%",
      },
    ];

    return (
      <div className="h-96">
        <Table columns={columns} data={sampleData} getRowId={(row) => row.id} selectedRows={new Set(["1", "3"])} onRowClick={(row) => console.log("Clicked:", row.name)} />
      </div>
    );
  },
};

export const CompactWithAvatars: Story = {
  args: { columns: [], data: [] },
  render: () => {
    const columns: TableColumn<Person>[] = [
      {
        id: "name",
        header: "Name",
        accessor: (row) => (
          <div className="flex items-center gap-double">
            <TableAvatar name={row.name} icon={row.icon} />
            <span>{row.name}</span>
          </div>
        ),
        width: "40%",
      },
      {
        id: "role",
        header: "Role",
        accessor: (row) => row.role,
        width: "30%",
      },
      {
        id: "email",
        header: "Email",
        accessor: (row) => row.email,
        width: "30%",
      },
    ];

    return (
      <div className="h-96">
        <Table columns={columns} data={sampleData} getRowId={(row) => row.id} rowHeight="compact" />
      </div>
    );
  },
};

interface FileItem {
  id: string;
  name: string;
  type: "folder" | "image" | "code" | "document";
  size: string;
}

const fileData: FileItem[] = [
  { id: "1", name: "Components", type: "folder", size: "-" },
  { id: "2", name: "Avatar.tsx", type: "code", size: "4.2 KB" },
  { id: "3", name: "logo.png", type: "image", size: "125 KB" },
  { id: "4", name: "Utils", type: "folder", size: "-" },
  { id: "5", name: "README.md", type: "document", size: "2.1 KB" },
];

export const WithIconAvatars: Story = {
  args: { columns: [], data: [] },
  render: () => {
    const getIconForType = (type: FileItem["type"]) => {
      switch (type) {
        case "folder":
          return <FolderIcon className="size-tiny" />;
        case "image":
          return <FileImage className="size-tiny" />;
        case "code":
          return <FileCode className="size-tiny" />;
        default:
          return undefined;
      }
    };

    const columns: TableColumn<FileItem>[] = [
      {
        id: "name",
        header: "Name",
        accessor: (row) => (
          <div className="flex items-center gap-double">
            <TableAvatar name={row.name} icon={getIconForType(row.type)} />
            <span>{row.name}</span>
          </div>
        ),
        width: "50%",
      },
      {
        id: "type",
        header: "Type",
        accessor: (row) => <span className="capitalize">{row.type}</span>,
        width: "25%",
      },
      {
        id: "size",
        header: "Size",
        accessor: (row) => row.size,
        width: "25%",
      },
    ];

    return (
      <div className="h-96">
        <Table columns={columns} data={fileData} getRowId={(row) => row.id} />
      </div>
    );
  },
};

// #endregion 🛎️Table

// 📊️#region ⏰️TableSkeleton
export const SkeletonDefault: Story = {
  args: { columns: [], data: [] },
  render: () => (
    <div className="h-64">
      <TableSkeleton columns={3} rowCount={5} />
    </div>
  ),
};

export const SkeletonMinimal: Story = {
  args: { columns: [], data: [] },
  render: () => (
    <div className="h-32">
      <TableSkeleton columns={2} rowCount={3} />
    </div>
  ),
};
// #endregion ⏰️TableSkeleton
