// #region Header

// ActionGroup.stories.tsx

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
import { Copy, Download, ExternalLink } from "lucide-react";
import { ActionGroup, ActionGroupItem } from "../../../../sketchpad/elements";

// #region ActionGroup
const meta = {
  title: "Elements/Input/ActionGroup",
  component: ActionGroup,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ActionGroup>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "action-group-default",
    level: "base",
    children: null,
  },
  render: (args) => (
    <ActionGroup {...args}>
      <ActionGroupItem id="action-group-default-copy">
        <Copy />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-default-download">
        <Download />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-default-external">
        <ExternalLink />
      </ActionGroupItem>
    </ActionGroup>
  ),
};

export const Base: Story = {
  args: { ...Default.args, id: "action-group-base", level: "base" },
  render: Default.render,
};

export const Window: Story = {
  args: { ...Default.args, id: "action-group-window", level: "window" },
  render: Default.render,
};

export const Panel: Story = {
  args: { ...Default.args, id: "action-group-panel", level: "panel" },
  render: Default.render,
};

export const Overlay: Story = {
  args: { ...Default.args, id: "action-group-overlay", level: "overlay" },
  render: Default.render,
};

export const Temporary: Story = {
  args: { ...Default.args, id: "action-group-temporary", level: "temporary" },
  render: Default.render,
};

// #endregion ActionGroup
