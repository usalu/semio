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
import { File, Folder, Settings } from "lucide-react";
import { Button, Input, Tree, TreeContent, TreeItem, TreeSection } from "../../../../sketchpad/elements";

// #region Tree
const meta = {
  title: "Elements/Aggregation/Tree",
  component: Tree,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Tree>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
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
              <Input id="kit-name-input" value="Metabolism" />
              <Input id="version-input" value="1.0.0" />
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

// #endregion Tree
