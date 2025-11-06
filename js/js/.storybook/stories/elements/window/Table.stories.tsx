// #region Header

// Table.stories.tsx

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
import { FileCode, FileImage, FolderIcon } from "lucide-react";
import { Table, TableAvatar, TableColumn } from "../../../../sketchpad/elements";

// #region Table
const meta = {
  title: "Elements/Windows/Table",
  component: Table,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Table>;

export default meta;

type Story = StoryObj<typeof meta>;

interface Person {
  id: string;
  name: string;
  icon?: string;
  role: string;
  email: string;
}

const sampleData: Person[] = [
  { id: "1", name: "Kisho Kurokawa", icon: "https://github.com/shadcn.png", role: "Lead Architect", email: "kisho@metabolism.jp" },
  { id: "2", name: "Kenzo Tange", role: "Urban Planner", email: "kenzo@tange.jp" },
  { id: "3", name: "Fumihiko Maki", role: "Architect", email: "fumihiko@maki.jp" },
  { id: "4", name: "Arata Isozaki", role: "Design Director", email: "arata@isozaki.jp" },
  { id: "5", name: "Kiyonori Kikutake", role: "Marine Architect", email: "kiyonori@kikutake.jp" },
];

export const WithAvatars: Story = {
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

// #endregion Table
