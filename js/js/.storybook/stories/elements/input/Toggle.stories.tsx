// #region Header

// Toggle.stories.tsx

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
import { Box, List, Lock, Network, Plus, Settings } from "lucide-react";
import { Action, Toggle } from "../../../../sketchpad/elements";

// #region Toggle
const meta = {
  title: "Elements/Input/Toggle",
  component: Toggle,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Toggle>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "toggle-default",
    pressed: true,
    icon: <Lock />,
    showLabel: true,
    level: "base",
  },
};

export const WithAction: Story = {
  args: {
    id: "toggle-action",
    kind: "withAction",
    pressed: false,
    icon: <Settings />,
    actionIcon: <Plus />,
    onActionClick: () => console.log("Action clicked"),
    actionId: "toggle-action-button",
    showLabel: true,
    level: "base",
  },
};

export const Dropdown: Story = {
  args: {
    id: "toggle-dropdown",
    kind: "dropdown",
    value: "option1",
    items: [
      { value: "option1", label: <Box /> },
      { value: "option2", label: <Network /> },
      { value: "option3", label: <List /> },
    ],
    dropdownId: "toggle-dropdown-action",
    showLabel: true,
    level: "base",
  },
};

// #endregion Toggle
