// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/input/ButtonGroup.stories.tsx

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

// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import { Box, List, Network } from "lucide-react";
import { ButtonGroup, ButtonGroupItem, Level, LevelProvider, getLevelBgClass } from "../../../../sketchpad/elements";

// #region 🔖ButtonGroup
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

const ButtonGroupDemo = ({ id }: { id: string }) => (
  <ButtonGroup id={id} showLabel>
    <ButtonGroupItem id={`${id}-model`} icon={<Box />} onClick={() => { }} />
    <ButtonGroupItem id={`${id}-diagram`} icon={<Network />} onClick={() => { }} />
    <ButtonGroupItem id={`${id}-details`} icon={<List />} onClick={() => { }} />
  </ButtonGroup>
);

export const Default: Story = {
  args: {
    id: "button-group-default",
    showLabel: true,
    children: null,
  },
  render: (args) => (
    <ButtonGroup {...args}>
      <ButtonGroupItem id="button-group-default-model" icon={<Box />} onClick={() => { }} />
      <ButtonGroupItem id="button-group-default-diagram" icon={<Network />} onClick={() => { }} />
      <ButtonGroupItem id="button-group-default-details" icon={<List />} onClick={() => { }} />
    </ButtonGroup>
  ),
};

const createLevelRender = (level: Level, id: string) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <ButtonGroupDemo id={id} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { id: "button-group-base", showLabel: true, children: null },
  render: createLevelRender("base", "button-group-base"),
};

export const Window: Story = {
  args: { id: "button-group-window", showLabel: true, children: null },
  render: createLevelRender("window", "button-group-window"),
};

export const Panel: Story = {
  args: { id: "button-group-panel", showLabel: true, children: null },
  render: createLevelRender("panel", "button-group-panel"),
};

export const Overlay: Story = {
  args: { id: "button-group-overlay", showLabel: true, children: null },
  render: createLevelRender("overlay", "button-group-overlay"),
};

export const Temporary: Story = {
  args: { id: "button-group-temporary", showLabel: true, children: null },
  render: createLevelRender("temporary", "button-group-temporary"),
};

// #endregion 🔖ButtonGroup
