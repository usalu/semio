// #region 🧲️Header

// 🥼️ .storybook/stories/ui/Label.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { Input, Label, Toggle } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

// 🪧️#region 🏷️Label
const meta = {
  title: "🖱️ui⚛️react/Label",
  component: Label,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Label>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "label-story-width",
    label: "Width",
    children: <Input id="label-story-width" defaultValue="240" />,
  },
  render: (args) => (
    <div className="w-70">
      <Label {...args} />
    </div>
  ),
};

export const TreeGroupHeader: Story = {
  name: "Tree Group Header",
  args: {
    id: "label-story-visibility",
    label: "Visibility",
    labelLayoutKind: "treeGroupHeader",
    children: <Toggle id="label-story-visibility-toggle" pressed text="On" />,
  },
  render: (args) => (
    <div className="w-70 border">
      <Label {...args} />
    </div>
  ),
};

export const FallbackFromId: Story = {
  name: "Fallback From Id",
  args: {
    id: "label.story.corner-radius",
    children: <Input id="label-story-corner-radius" defaultValue="4" />,
  },
  render: (args) => (
    <div className="w-70">
      <Label {...args} />
    </div>
  ),
};

// #endregion 🏷️Label
