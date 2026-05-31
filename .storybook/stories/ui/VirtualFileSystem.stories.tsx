// #region 🧲Header

// 🥼︎ .storybook/stories/ui/VirtualFileSystem.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import {
  VirtualFileSystem,
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
  id: "kit-demo",
  kind: "kit",
  name: "Metabolism",
  path: "/",
  hasChildren: true,
};

const demoChildrenByParentId = new Map<string, readonly VirtualFileSystemNode[]>([
  [
    "kit-demo",
    [
      {
        id: "folder-types",
        kind: "folder",
        name: "Types",
        path: "/Types",
        parentId: "kit-demo",
        hasChildren: true,
      },
      {
        id: "design-tower",
        kind: "design",
        name: "Tower A",
        path: "/Tower A",
        parentId: "kit-demo",
        hasChildren: true,
      },
      {
        id: "file-readme",
        kind: "file",
        name: "README.md",
        path: "/README.md",
        parentId: "kit-demo",
        hasChildren: false,
      },
    ],
  ],
  [
    "folder-types",
    [
      {
        id: "type-capsule",
        kind: "type",
        name: "Capsule",
        path: "/Types/Capsule",
        parentId: "folder-types",
        hasChildren: false,
      },
    ],
  ],
  [
    "design-tower",
    [
      {
        id: "piece-core",
        kind: "piece",
        name: "Core",
        path: "/Tower A/Core",
        parentId: "design-tower",
        hasChildren: false,
      },
      {
        id: "conn-bridge",
        kind: "connection",
        name: "Bridge",
        path: "/Tower A/Bridge",
        parentId: "design-tower",
        hasChildren: false,
      },
    ],
  ],
]);

const VirtualFileSystemDemo = ({ initialExpanded }: { readonly initialExpanded: readonly string[] }) => {
  const [expandedIds, setExpandedIds] = reactHostPort.useState<ReadonlySet<string>>(() => new Set(initialExpanded));
  const [selectedRowIds, setSelectedRowIds] = reactHostPort.useState<Set<string>>(() => new Set(["design-tower"]));

  const rows = buildVirtualFileSystemVisibleRows("kit-demo", demoChildrenByParentId, expandedIds, demoRoot);

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
          canDrag: (rowId) => rowId !== "kit-demo",
          canDrop: (draggedId, targetId) => draggedId !== targetId && targetId !== "file-readme",
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
  args: { rows: [] },
  render: () => <VirtualFileSystemDemo initialExpanded={["kit-demo"]} />,
};

export const Expanded: Story = {
  args: { rows: [] },
  render: () => <VirtualFileSystemDemo initialExpanded={["kit-demo", "folder-types", "design-tower"]} />,
};

export const StaticRows: Story = {
  args: { rows: [] },
  render: () => {
    const rows: VirtualFileSystemRow[] = [
      { id: "kit", kind: "kit", name: "Kit", path: "/", level: 0, hasChildren: true, isExpanded: true },
      { id: "folder", kind: "folder", name: "Assets", path: "/Assets", level: 1, hasChildren: false },
    ];
    return (
      <div className="h-64 border border-element">
        <VirtualFileSystem rows={rows} />
      </div>
    );
  },
};

// #endregion 📁VirtualFileSystem
