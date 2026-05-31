// #region 🧲Header

// 🥼︎ .storybook/stories/ui/VirtualFileSystem.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import {
  VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA,
  VirtualFileSystem,
  buildVirtualFileSystemDescriptorValues,
  buildVirtualFileSystemVisibleRows,
  reactHostPort,
  type VirtualFileSystemNode,
  type VirtualFileSystemRow,
} from "@ui/react";
import type { Meta, StoryObj } from "@storybook/react";

// 📁#region 📁VirtualFileSystem
const meta = {
  title: "elements/react/VirtualFileSystem",
  component: VirtualFileSystem,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof VirtualFileSystem>;

export default meta;

type Story = StoryObj<typeof meta>;

const demoRoot: VirtualFileSystemNode = {
  id: "root-demo",
  fileNodeKindId: "root",
  name: "Workspace",
  path: "/",
  hasChildren: true,
  descriptorValues: buildVirtualFileSystemDescriptorValues(VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, "root", { path: "/" }),
};

const demoChildrenByParentId = new Map<string, readonly VirtualFileSystemNode[]>([
  [
    "root-demo",
    [
      {
        id: "branch-models",
        fileNodeKindId: "branch",
        name: "Models",
        path: "/Models",
        parentId: "root-demo",
        hasChildren: true,
        descriptorValues: buildVirtualFileSystemDescriptorValues(VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, "branch", { path: "/Models" }),
      },
      {
        id: "leaf-alpha",
        fileNodeKindId: "leaf",
        name: "Alpha",
        path: "/Alpha",
        parentId: "root-demo",
        hasChildren: true,
        descriptorValues: buildVirtualFileSystemDescriptorValues(VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, "leaf", { path: "/Alpha" }),
      },
      {
        id: "leaf-readme",
        fileNodeKindId: "leaf",
        name: "README.md",
        path: "/README.md",
        parentId: "root-demo",
        hasChildren: false,
        descriptorValues: buildVirtualFileSystemDescriptorValues(VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, "leaf", { path: "/README.md" }),
      },
    ],
  ],
  [
    "branch-models",
    [
      {
        id: "leaf-capsule",
        fileNodeKindId: "leaf",
        name: "Capsule",
        path: "/Models/Capsule",
        parentId: "branch-models",
        hasChildren: false,
        descriptorValues: buildVirtualFileSystemDescriptorValues(VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, "leaf", { path: "/Models/Capsule" }),
      },
    ],
  ],
  [
    "leaf-alpha",
    [
      {
        id: "leaf-core",
        fileNodeKindId: "leaf",
        name: "Core",
        path: "/Alpha/Core",
        parentId: "leaf-alpha",
        hasChildren: false,
        descriptorValues: buildVirtualFileSystemDescriptorValues(VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, "leaf", { path: "/Alpha/Core" }),
      },
      {
        id: "leaf-bridge",
        fileNodeKindId: "leaf",
        name: "Bridge",
        path: "/Alpha/Bridge",
        parentId: "leaf-alpha",
        hasChildren: false,
        descriptorValues: buildVirtualFileSystemDescriptorValues(VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, "leaf", { path: "/Alpha/Bridge" }),
      },
    ],
  ],
]);

const VirtualFileSystemDemo = ({ initialExpanded }: { readonly initialExpanded: readonly string[] }) => {
  const [expandedIds, setExpandedIds] = reactHostPort.useState<ReadonlySet<string>>(() => new Set(initialExpanded));
  const [selectedRowIds, setSelectedRowIds] = reactHostPort.useState<Set<string>>(() => new Set(["leaf-alpha"]));

  const rows = buildVirtualFileSystemVisibleRows("root-demo", demoChildrenByParentId, expandedIds, demoRoot);

  const onToggleExpand = (rowId: string) => {
    setExpandedIds((previous) => {
      const next = new Set(previous);
      if (next.has(rowId)) next.delete(rowId);
      else next.add(rowId);
      return next;
    });
  };

  return (
    <div className="h-96 border border-element">
      <VirtualFileSystem
        schema={VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA}
        rows={rows}
        selectedRowIds={selectedRowIds}
        onRowClick={(row, _index, event) => {
          if (event.metaKey || event.ctrlKey) {
            setSelectedRowIds((previous) => {
              const next = new Set(previous);
              if (next.has(row.id)) next.delete(row.id);
              else next.add(row.id);
              return next;
            });
            return;
          }
          setSelectedRowIds(new Set([row.id]));
        }}
        onToggleExpand={onToggleExpand}
        dragDrop={{
          enabled: true,
          canDrag: (rowId) => rowId !== "root-demo",
          canDrop: (draggedId, targetId) => draggedId !== targetId && targetId !== "leaf-readme",
          onDragEnd: ({ active, over }) => {
            if (!over) return;
            console.log("[story] vfs drag", { active, over });
          },
        }}
      />
    </div>
  );
};

export const Collapsed: Story = {
  args: { schema: VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, rows: [] },
  render: () => <VirtualFileSystemDemo initialExpanded={["root-demo"]} />,
};

export const Expanded: Story = {
  args: { schema: VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, rows: [] },
  render: () => <VirtualFileSystemDemo initialExpanded={["root-demo", "branch-models", "leaf-alpha"]} />,
};

export const StaticRows: Story = {
  args: { schema: VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, rows: [] },
  render: () => {
    const rows: VirtualFileSystemRow[] = [
      {
        id: "root",
        fileNodeKindId: "root",
        name: "Root",
        path: "/",
        level: 0,
        hasChildren: true,
        isExpanded: true,
        descriptorValues: buildVirtualFileSystemDescriptorValues(VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, "root", { path: "/" }),
      },
      {
        id: "branch",
        fileNodeKindId: "branch",
        name: "Assets",
        path: "/Assets",
        level: 1,
        hasChildren: false,
        descriptorValues: buildVirtualFileSystemDescriptorValues(VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA, "branch", { path: "/Assets" }),
      },
    ];
    return (
      <div className="h-64 border border-element">
        <VirtualFileSystem schema={VIRTUAL_FILE_SYSTEM_DEMO_SCHEMA} rows={rows} />
      </div>
    );
  },
};

// #endregion 📁VirtualFileSystem
