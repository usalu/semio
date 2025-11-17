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
    noDivider: true,
    children: (
      <>
        <ToggleGroupItem id="toggle-default-standard" value="standard" icon={<Lock />} />
        <ToggleGroupItem 
          id="toggle-action-settings" 
          value="settings" 
          icon={<Settings />} 
          action={<Action id="toggle-action-settings-add" icon={<Plus />} level="base" />}
        />
        <ToggleGroupItem 
          id="toggle-dropdown-box" 
          value="box" 
          icon={<Box />} 
          action={
            <Action 
              id="toggle-dropdown-box-action" 
              icon={<Network />} 
              level="base" 
            />
          }
        />
      </>
    ),
  },
};

export const Multiple: Story = {
  args: {
    id: "toggle-group-multiple",
    kind: "single",
    defaultValue: "box",
    level: "base",
    showLabel: true,
    children: (
      <>
        <ToggleGroupItem id="toggle-multiple-standard" value="standard" icon={<Lock />} />
        <ToggleGroupItem id="toggle-multiple-box" value="box" icon={<Box />} />
        <ToggleGroupItem id="toggle-multiple-network" value="network" icon={<Network />} />
        <ToggleGroupItem id="toggle-multiple-list" value="list" icon={<List />} />
        <ToggleGroupItem id="toggle-multiple-settings" value="settings" icon={<Settings />} />
        <ToggleGroupItem id="toggle-multiple-plus" value="plus" icon={<Plus />} />
      </>
    ),
  },
};



// #endregion ToggleGroup
