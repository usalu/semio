// #region 🧲Header

// 🥼︎ .storybook/story/ui/VirtualFileSystem.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import {
  VirtualFileSystem,
  VIRTUAL_FILE_SYSTEM_DEMO_DESCRIPTOR_KINDS,
  VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS,
  buildVirtualFileSystemDescriptorValues,
  buildVirtualFileSystemVisibleRows,
  reactHostPort,
  type VirtualFileSystemNode,
  type VirtualFileSystemRow,
  type VirtualFileSystemSchema,
} from "@ui/react";
import type { Meta, StoryObj } from "@storybook/react";

// 📁#region 📁VirtualFileSystem
const meta = {
  title: "🖱️ui⚛️react/VirtualFileSystem",
  component: VirtualFileSystem,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof VirtualFileSystem>;

export default meta;

type Story = StoryObj<typeof meta>;

const semioKitSchema: VirtualFileSystemSchema = {
  descriptorKinds: VIRTUAL_FILE_SYSTEM_DEMO_DESCRIPTOR_KINDS,
  fileNodeKinds: {
    ...VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS,
    kit: { id: "kit", name: "Kit", icon: "layout-grid", descriptors: VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS.root!.descriptors },
    type: { id: "type", name: "Type", icon: "component", descriptors: VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS.branch!.descriptors },
    design: { id: "design", name: "Design", icon: "layout", descriptors: VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS.branch!.descriptors },
    representation: { id: "representation", name: "Representation", icon: "box", descriptors: VIRTUAL_FILE_SYSTEM_DEMO_FILE_NODE_KINDS.leaf!.descriptors },
  },
  descriptorColumnIds: ["path", "fileNodeKind"],
};

const demoRoot: VirtualFileSystemNode = {
  id: "root-demo",
  fileNodeKindId: "kit",
  name: "Workspace",
  path: "/",
  hasChildren: true,
  descriptorValues: buildVirtualFileSystemDescriptorValues(semioKitSchema, "kit", { path: "/" }),
};

const demoChildrenByParentId = new Map<string, readonly VirtualFileSystemNode[]>([
  [
    "root-demo",
    [
      {
        id: "branch-models",
        fileNodeKindId: "folder",
        name: "Models",
        path: "/Models",
        parentId: "root-demo",
        hasChildren: true,
        descriptorValues: buildVirtualFileSystemDescriptorValues(semioKitSchema, "folder", { path: "/Models" }),
      },
      {
        id: "type-alpha",
        fileNodeKindId: "type",
        name: "Alpha",
        path: "/Alpha",
        parentId: "root-demo",
        hasChildren: true,
        descriptorValues: buildVirtualFileSystemDescriptorValues(semioKitSchema, "type", { path: "/Alpha" }),
      },
      {
        id: "leaf-readme",
        fileNodeKindId: "leaf",
        name: "README.md",
        path: "/README.md",
        parentId: "root-demo",
        hasChildren: false,
        descriptorValues: buildVirtualFileSystemDescriptorValues(semioKitSchema, "file", { path: "/README.md" }),
      },
    ],
  ],
  [
    "branch-models",
    [
      {
        id: "design-capsule",
        fileNodeKindId: "design",
        name: "Capsule",
        path: "/Models/Capsule",
        parentId: "branch-models",
        hasChildren: false,
        descriptorValues: buildVirtualFileSystemDescriptorValues(semioKitSchema, "design", { path: "/Models/Capsule" }),
      },
    ],
  ],
  [
    "type-alpha",
    [
      {
        id: "rep-core",
        fileNodeKindId: "representation",
        name: "Core",
        path: "/Alpha/Core",
        parentId: "type-alpha",
        hasChildren: false,
        descriptorValues: buildVirtualFileSystemDescriptorValues(semioKitSchema, "representation", { path: "/Alpha/Core" }),
      },
      {
        id: "rep-bridge",
        fileNodeKindId: "representation",
        name: "Bridge",
        path: "/Alpha/Bridge",
        parentId: "type-alpha",
        hasChildren: false,
        descriptorValues: buildVirtualFileSystemDescriptorValues(semioKitSchema, "representation", { path: "/Alpha/Bridge" }),
      },
    ],
  ],
]);

const VirtualFileSystemDemo = ({ initialExpanded }: { readonly initialExpanded: readonly string[] }) => {
  const [expandedIds, setExpandedIds] = reactHostPort.useState<ReadonlySet<string>>(() => new Set(initialExpanded));
  const [selectedRowIds, setSelectedRowIds] = reactHostPort.useState<readonly string[]>(() => ["type-alpha"]);

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
    <div className="h-96 border border-normal">
      <VirtualFileSystem
        schema={semioKitSchema}
        rows={rows}
        selectedRowIds={selectedRowIds}
        onSelectionChange={(nextSelectedRowIds) => setSelectedRowIds(nextSelectedRowIds)}
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
  args: { schema: semioKitSchema, rows: [] },
  render: () => <VirtualFileSystemDemo initialExpanded={[]} />,
};

export const Expanded: Story = {
  args: { schema: semioKitSchema, rows: [] },
  render: () => <VirtualFileSystemDemo initialExpanded={["root-demo", "branch-models", "type-alpha"]} />,
};

export const StaticRows: Story = {
  args: { schema: semioKitSchema, rows: [] },
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
        descriptorValues: buildVirtualFileSystemDescriptorValues(semioKitSchema, "kit", { path: "/" }),
      },
      {
        id: "branch",
        fileNodeKindId: "branch",
        name: "Assets",
        path: "/Assets",
        level: 1,
        hasChildren: false,
        descriptorValues: buildVirtualFileSystemDescriptorValues(semioKitSchema, "folder", { path: "/Assets" }),
      },
    ];
    return (
      <div className="h-64 border border-normal">
        <VirtualFileSystem schema={semioKitSchema} rows={rows} />
      </div>
    );
  },
};

// #endregion 📁VirtualFileSystem
