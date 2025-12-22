// #region Header

// js/js/.storybook/stories/elements/input/ButtonGroup.stories.tsx

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

// ButtonGroup.stories.tsx

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
import { Box, List, Network } from "lucide-react";
import { ButtonGroup, ButtonGroupItem } from "../../../../sketchpad/elements";

// #region ButtonGroup
const meta = {
  title: "Elements/Input/ButtonGroup",
  component: ButtonGroup,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ButtonGroup>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "button-group-default",
    level: "base",
    showLabel: true,
    children: null,
  },
  render: (args) => (
    <ButtonGroup {...args}>
      <ButtonGroupItem id="button-group-default-model" icon={<Box />} onClick={() => {}} />
      <ButtonGroupItem id="button-group-default-diagram" icon={<Network />} onClick={() => {}} />
      <ButtonGroupItem id="button-group-default-details" icon={<List />} onClick={() => {}} />
    </ButtonGroup>
  ),
};

export const Base: Story = {
  args: { ...Default.args, id: "button-group-base", level: "base" },
  render: Default.render,
};

export const Window: Story = {
  args: { ...Default.args, id: "button-group-window", level: "window" },
  render: Default.render,
};

export const Panel: Story = {
  args: { ...Default.args, id: "button-group-panel", level: "panel" },
  render: Default.render,
};

export const Overlay: Story = {
  args: { ...Default.args, id: "button-group-overlay", level: "overlay" },
  render: Default.render,
};

export const Temporary: Story = {
  args: { ...Default.args, id: "button-group-temporary", level: "temporary" },
  render: Default.render,
};

// #endregion ButtonGroup
