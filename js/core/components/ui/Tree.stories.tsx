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
import { Button } from "./Button";
import { Input } from "./Input";
import { SortableTreeItems, Tree, TreeContent, TreeItem, TreeSection } from "./Tree";

const meta = {
  title: "UI/Tree",
  component: Tree,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Tree>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Basic: Story = {
  args: {
    children: null,
  },
  render: () => (
    <div className="border p-4">
      <Tree>
        <TreeSection label="Project" icon={<Folder size={14} />}>
          <TreeItem label="src" icon={<Folder size={12} />}>
            <TreeItem label="components" icon={<Folder size={12} />}>
              <TreeItem label="ui" icon={<Folder size={12} />}>
                <TreeItem label="Tree.tsx" icon={<File size={12} />} />
                <TreeItem label="Button.tsx" icon={<File size={12} />} />
                <TreeItem label="Input.tsx" icon={<File size={12} />} />
              </TreeItem>
              <TreeItem label="layout" icon={<Folder size={12} />}>
                <TreeItem label="Header.tsx" icon={<File size={12} />} />
                <TreeItem label="Footer.tsx" icon={<File size={12} />} />
              </TreeItem>
            </TreeItem>
            <TreeItem label="utils" icon={<Folder size={12} />}>
              <TreeItem label="helpers.ts" icon={<File size={12} />} />
            </TreeItem>
          </TreeItem>
          <TreeItem label="package.json" icon={<File size={12} />} />
          <TreeItem label="README.md" icon={<File size={12} />} />
        </TreeSection>
        <TreeSection label="Settings" icon={<Settings size={14} />}>
          <TreeItem label="General">
            <TreeContent>
              <Input label="Name" value="My Project" />
              <Input label="Version" value="1.0.0" />
            </TreeContent>
          </TreeItem>
          <TreeItem label="Advanced">
            <TreeContent>
              <Button>Reset Settings</Button>
            </TreeContent>
          </TreeItem>
        </TreeSection>
      </Tree>
    </div>
  ),
};

export const WithActions: Story = {
  args: {
    children: null,
  },
  render: () => (
    <div className="w-80 h-96 border rounded-lg p-4">
      <Tree>
        <TreeSection
          label="Files"
          icon={<Folder size={14} />}
          actions={[
            {
              icon: <Plus size={12} />,
              onClick: () => console.log("Add file"),
              title: "Add file",
            },
          ]}
        >
          <TreeItem label="document.txt" icon={<File size={12} />} />
          <TreeItem label="image.jpg" icon={<File size={12} />} />
        </TreeSection>
        <TreeSection
          label="Empty Folder"
          icon={<FolderOpen size={14} />}
          actions={[
            {
              icon: <Minus size={12} />,
              onClick: () => console.log("Remove folder"),
              title: "Remove folder",
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
    <div className="w-80 h-96 border rounded-lg p-4">
      <Tree>
        <TreeSection label="Level 0" icon={<Folder size={14} />}>
          <TreeItem label="Level 1A" icon={<Folder size={12} />}>
            <TreeItem label="Level 2A" icon={<Folder size={12} />}>
              <TreeItem label="Level 3A" icon={<Folder size={12} />}>
                <TreeItem label="Level 4A" icon={<File size={12} />} />
                <TreeItem label="Level 4B" icon={<File size={12} />} />
              </TreeItem>
              <TreeItem label="Level 3B" icon={<File size={12} />} />
            </TreeItem>
            <TreeItem label="Level 2B" icon={<File size={12} />} />
          </TreeItem>
          <TreeItem label="Level 1B" icon={<Folder size={12} />}>
            <TreeItem label="Level 2C" icon={<File size={12} />} />
            <TreeItem label="Level 2D" icon={<File size={12} />} />
          </TreeItem>
          <TreeItem label="Level 1C" icon={<File size={12} />} />
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
    <div className="w-80 h-96 border rounded-lg p-4">
      <Tree showLines={false}>
        <TreeSection label="No Lines" icon={<Folder size={14} />}>
          <TreeItem label="Item 1" icon={<File size={12} />}>
            <TreeItem label="Sub Item 1" icon={<File size={12} />} />
            <TreeItem label="Sub Item 2" icon={<File size={12} />} />
          </TreeItem>
          <TreeItem label="Item 2" icon={<File size={12} />} />
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
      { id: "1", name: "First Item" },
      { id: "2", name: "Second Item" },
      { id: "3", name: "Third Item" },
    ];

    return (
      <div className="w-80 h-96 border rounded-lg p-4">
        <Tree>
          <TreeSection label="Sortable Items" icon={<Folder size={14} />}>
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
