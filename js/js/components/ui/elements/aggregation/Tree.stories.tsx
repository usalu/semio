// #region Header

// Tree.stories.tsx

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
import { File, Folder, FolderOpen, Minus, Plus, Settings } from "lucide-react";
import { Button } from "../input/Button";
import { Input } from "../input/Input";
import { SortableTreeItems, Tree, TreeContent, TreeItem, TreeSection } from "./Tree";

const meta = {
  title: "Elements/Tree",
  component: Tree,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Tree>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    children: null,
  },
  render: () => (
    <div className="border p-4">
      <Tree>
        <TreeSection label="Kit" icon={<Folder size={14} />}>
          <TreeItem label="Types" icon={<Folder size={12} />}>
            <TreeItem label="Capsules" icon={<Folder size={12} />}>
              <TreeItem label="Variants" icon={<Folder size={12} />}>
                <TreeItem label="Capsule J" icon={<File size={12} />} />
                <TreeItem label="Capsule L" icon={<File size={12} />} />
                <TreeItem label="Capsule P" icon={<File size={12} />} />
              </TreeItem>
              <TreeItem label="Balcony" icon={<Folder size={12} />}>
                <TreeItem label="With Balcony J" icon={<File size={12} />} />
                <TreeItem label="With Balcony L" icon={<File size={12} />} />
              </TreeItem>
            </TreeItem>
            <TreeItem label="Bases" icon={<Folder size={12} />}>
              <TreeItem label="Base Blob" icon={<File size={12} />} />
            </TreeItem>
          </TreeItem>
          <TreeItem label="Designs" icon={<File size={12} />} />
          <TreeItem label="Qualities" icon={<File size={12} />} />
        </TreeSection>
        <TreeSection label="Settings" icon={<Settings size={14} />}>
          <TreeItem label="General">
            <TreeContent>
              <Input label="Kit Name" value="Metabolism" />
              <Input label="Version" value="1.0.0" />
            </TreeContent>
          </TreeItem>
          <TreeItem label="Advanced">
            <TreeContent>
              <Button>Export Kit</Button>
            </TreeContent>
          </TreeItem>
        </TreeSection>
      </Tree>
    </div>
  ),
};

export const Variants: Story = {
  args: {
    children: null,
  },
  render: () => (
    <div className="flex gap-8">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Basic</p>
        <div className="border p-4">
          <Tree>
            <TreeSection label="Types" icon={<Folder size={14} />}>
              <TreeItem label="Capsule" icon={<File size={12} />} />
              <TreeItem label="Base" icon={<File size={12} />} />
              <TreeItem label="Tambour" icon={<File size={12} />} />
            </TreeSection>
          </Tree>
        </div>
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Nested</p>
        <div className="border p-4">
          <Tree>
            <TreeSection label="Kit" icon={<Folder size={14} />}>
              <TreeItem label="Types" icon={<Folder size={12} />}>
                <TreeItem label="Capsules" icon={<Folder size={12} />}>
                  <TreeItem label="Capsule J" icon={<File size={12} />} />
                  <TreeItem label="Capsule L" icon={<File size={12} />} />
                </TreeItem>
              </TreeItem>
            </TreeSection>
          </Tree>
        </div>
      </div>
    </div>
  ),
};

export const WithActions: Story = {
  args: {
    children: null,
  },
  render: () => (
    <div className="w-80 h-96 border p-4">
      <Tree>
        <TreeSection
          label="Pieces"
          icon={<Folder size={14} />}
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => console.log("Add piece"),
              title: "Add piece",
            },
          ]}
        >
          <TreeItem label="Capsule J" icon={<File size={12} />} />
          <TreeItem label="Base Blob" icon={<File size={12} />} />
        </TreeSection>
        <TreeSection
          label="Empty Layer"
          icon={<FolderOpen size={14} />}
          actions={[
            {
              icon: <Minus size={12} />,
              onClick: () => console.log("Remove layer"),
              title: "Remove layer",
            },
          ]}
        />
      </Tree>
    </div>
  ),
};

export const DeepNesting: Story = {
  args: {
    children: null,
  },
  render: () => (
    <div className="w-80 h-96 border p-4">
      <Tree>
        <TreeSection label="Nakagin Tower" icon={<Folder size={14} />}>
          <TreeItem label="Ground Floor" icon={<Folder size={12} />}>
            <TreeItem label="Lobby" icon={<Folder size={12} />}>
              <TreeItem label="Reception" icon={<Folder size={12} />}>
                <TreeItem label="Desk" icon={<File size={12} />} />
                <TreeItem label="Seating" icon={<File size={12} />} />
              </TreeItem>
              <TreeItem label="Entrance" icon={<File size={12} />} />
            </TreeItem>
            <TreeItem label="Parking" icon={<File size={12} />} />
          </TreeItem>
          <TreeItem label="Upper Floors" icon={<Folder size={12} />}>
            <TreeItem label="Capsule 101" icon={<File size={12} />} />
            <TreeItem label="Capsule 102" icon={<File size={12} />} />
          </TreeItem>
          <TreeItem label="Roof" icon={<File size={12} />} />
        </TreeSection>
      </Tree>
    </div>
  ),
};

export const WithoutLines: Story = {
  args: {
    children: null,
    showLines: false,
  },
  render: () => (
    <div className="w-80 h-96 border p-4">
      <Tree showLines={false}>
        <TreeSection label="Types" icon={<Folder size={14} />}>
          <TreeItem label="Capsule" icon={<File size={12} />}>
            <TreeItem label="Variant J" icon={<File size={12} />} />
            <TreeItem label="Variant L" icon={<File size={12} />} />
          </TreeItem>
          <TreeItem label="Base" icon={<File size={12} />} />
        </TreeSection>
      </Tree>
    </div>
  ),
};

export const SortableExample: Story = {
  args: {
    children: null,
  },
  render: () => {
    const items = [
      { id: "1", name: "Capsule J" },
      { id: "2", name: "Capsule L" },
      { id: "3", name: "Capsule P" },
    ];

    return (
      <div className="w-80 h-96 border p-4">
        <Tree>
          <TreeSection label="Sortable Pieces" icon={<Folder size={14} />}>
            <SortableTreeItems
              items={items}
              onReorder={(oldIndex, newIndex) => {
                console.log(`Move item from ${oldIndex} to ${newIndex}`);
              }}
            >
              {(item, index) => <TreeItem key={item.id} label={item.name} icon={<File size={12} />} sortable={true} sortableId={item.id} isDragHandle={true} isLastItem={index === items.length - 1} />}
            </SortableTreeItems>
          </TreeSection>
        </Tree>
      </div>
    );
  },
};
