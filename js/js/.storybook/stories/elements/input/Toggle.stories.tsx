// #region Header

// js/js/.storybook/stories/elements/input/Toggle.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion Header

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
import { Toggle } from "../../../../sketchpad/elements";

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
    defaultPressed: true,
    icon: <Lock />,
    showLabel: true,
    level: "base",
  },
};

export const Base: Story = {
  args: { ...Default.args, id: "toggle-base", level: "base" },
};

export const Window: Story = {
  args: { ...Default.args, id: "toggle-window", level: "window" },
};

export const Panel: Story = {
  args: { ...Default.args, id: "toggle-panel", level: "panel" },
};

export const Overlay: Story = {
  args: { ...Default.args, id: "toggle-overlay", level: "overlay" },
};

export const Temporary: Story = {
  args: { ...Default.args, id: "toggle-temporary", level: "temporary" },
};

export const WithAction: Story = {
  args: {
    id: "toggle-action",
    kind: "withAction",
    defaultPressed: false,
    icon: <Settings />,
    actionIcon: <Plus />,
    onActionClick: () => console.log("Action clicked"),
    actionId: "toggle-action-button",
    showLabel: true,
    level: "base",
  },
};

export const WithActionBase: Story = {
  args: { ...WithAction.args, id: "toggle-action-base", level: "base" },
};

export const WithActionWindow: Story = {
  args: { ...WithAction.args, id: "toggle-action-window", level: "window" },
};

export const WithActionPanel: Story = {
  args: { ...WithAction.args, id: "toggle-action-panel", level: "panel" },
};

export const WithActionOverlay: Story = {
  args: { ...WithAction.args, id: "toggle-action-overlay", level: "overlay" },
};

export const WithActionTemporary: Story = {
  args: { ...WithAction.args, id: "toggle-action-temporary", level: "temporary" },
};

export const Dropdown: Story = {
  args: {
    id: "toggle-dropdown",
    kind: "dropdown",
    defaultValue: "option1",
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

export const DropdownBase: Story = {
  args: { ...Dropdown.args, id: "toggle-dropdown-base", level: "base" },
};

export const DropdownWindow: Story = {
  args: { ...Dropdown.args, id: "toggle-dropdown-window", level: "window" },
};

export const DropdownPanel: Story = {
  args: { ...Dropdown.args, id: "toggle-dropdown-panel", level: "panel" },
};

export const DropdownOverlay: Story = {
  args: { ...Dropdown.args, id: "toggle-dropdown-overlay", level: "overlay" },
};

export const DropdownTemporary: Story = {
  args: { ...Dropdown.args, id: "toggle-dropdown-temporary", level: "temporary" },
};

// #endregion Toggle
