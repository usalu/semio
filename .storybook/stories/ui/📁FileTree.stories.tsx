// #region 🧲️Header

// 🥼️ .storybook/stories/ui/📁FileTree.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { FileTree, type FileTreeNode } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

// 🌲️#region 🌲️FileTree
const meta = {
  title: "🖱️ui⚛️react/FileTree",
  component: FileTree,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof FileTree>;

export default meta;

type Story = StoryObj<typeof meta>;

const kitNodes: FileTreeNode[] = [
  {
    title: "capsules",
    path: "kit/capsules",
    isFolder: true,
    children: [
      { title: "capsule-j.json", path: "kit/capsules/capsule-j.json", isFolder: false },
      { title: "capsule-l.json", path: "kit/capsules/capsule-l.json", isFolder: false },
      {
        title: "balcony",
        path: "kit/capsules/balcony",
        isFolder: true,
        children: [{ title: "balcony-j.json", path: "kit/capsules/balcony/balcony-j.json", isFolder: false }],
      },
    ],
  },
  {
    title: "bases",
    path: "kit/bases",
    isFolder: true,
    children: [{ title: "base-blob.json", path: "kit/bases/base-blob.json", isFolder: false }],
  },
  { title: "README.md", path: "kit/README.md", isFolder: false },
];

export const Default: Story = {
  args: {
    title: "Nakagin Kit",
    nodes: kitNodes,
    as: "div",
  },
};

export const WithCurrentPath: Story = {
  name: "Current Path Highlighted",
  args: {
    title: "Nakagin Kit",
    nodes: kitNodes,
    currentPath: "kit/capsules/capsule-l.json",
    as: "div",
  },
};

export const Navigable: Story = {
  args: {
    title: "Nakagin Kit",
    nodes: kitNodes,
    as: "div",
  },
  render: (args) => {
    const [currentPath, setCurrentPath] = useState<string | undefined>(kitNodes[0]!.children![0]!.path);
    return <FileTree {...args} currentPath={currentPath} onNavigate={setCurrentPath} />;
  },
};

// #endregion 🌲️FileTree
