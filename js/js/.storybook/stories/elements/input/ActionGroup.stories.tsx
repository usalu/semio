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
import { Copy, Download, ExternalLink, Trash2, X } from "lucide-react";
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
  render: () => (
    <ActionGroup id="action-group-default" variant="default" level="base" showLabel>
      <ActionGroupItem id="action-group-default-copy" variant="default" level="base">
        <Copy />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-default-download" variant="default" level="base">
        <Download />
      </ActionGroupItem>
      <ActionGroupItem id="action-group-default-external" variant="default" level="base">
        <ExternalLink />
      </ActionGroupItem>
    </ActionGroup>
  ),
};

// #endregion ActionGroup
