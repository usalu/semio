// #region Header

// ToggleGroup.stories.tsx

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
import { Action, ToggleGroup, ToggleGroupItem } from "../../../../sketchpad/elements";

// #region ToggleGroup
const meta = {
  title: "Elements/Input/ToggleGroup",
  component: ToggleGroup,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ToggleGroup>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "toggle-group-default",
    kind: "single",
    defaultValue: "standard",
    level: "base",
    showLabel: true,
    items: [
      { id: "toggle-default-standard", value: "standard", icon: <Lock /> },
      { id: "toggle-action-settings", value: "settings", icon: <Settings />, action: <Action id="toggle-action-settings-add" icon={<Plus />} level="base" /> },
      { id: "toggle-dropdown-box", value: "box", icon: <Box />, action: <Action id="toggle-dropdown-box-action" icon={<Network />} level="base" /> },
    ],
  },
};

export const Base: Story = {
  args: { ...Default.args, id: "toggle-group-base", level: "base" },
};

export const Window: Story = {
  args: { ...Default.args, id: "toggle-group-window", level: "window" },
};

export const Panel: Story = {
  args: { ...Default.args, id: "toggle-group-panel", level: "panel" },
};

export const Overlay: Story = {
  args: { ...Default.args, id: "toggle-group-overlay", level: "overlay" },
};

export const Temporary: Story = {
  args: { ...Default.args, id: "toggle-group-temporary", level: "temporary" },
};

export const Multiple: Story = {
  args: {
    id: "toggle-group-multiple",
    kind: "multiple",
    defaultValue: ["box"],
    level: "base",
    showLabel: true,
    items: [
      { id: "toggle-multiple-standard", value: "standard", icon: <Lock /> },
      { id: "toggle-multiple-box", value: "box", icon: <Box /> },
      { id: "toggle-multiple-network", value: "network", icon: <Network /> },
      { id: "toggle-multiple-list", value: "list", icon: <List /> },
      { id: "toggle-multiple-settings", value: "settings", icon: <Settings /> },
      { id: "toggle-multiple-plus", value: "plus", icon: <Plus /> },
    ],
  },
};

export const MultipleBase: Story = {
  args: { ...Multiple.args, id: "toggle-group-multiple-base", level: "base" },
};

export const MultipleWindow: Story = {
  args: { ...Multiple.args, id: "toggle-group-multiple-window", level: "window" },
};

export const MultiplePanel: Story = {
  args: { ...Multiple.args, id: "toggle-group-multiple-panel", level: "panel" },
};

export const MultipleOverlay: Story = {
  args: { ...Multiple.args, id: "toggle-group-multiple-overlay", level: "overlay" },
};

export const MultipleTemporary: Story = {
  args: { ...Multiple.args, id: "toggle-group-multiple-temporary", level: "temporary" },
};

// #endregion ToggleGroup
