// #region Header

// js/js/.storybook/stories/elements/input/Select.stories.tsx

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

import type { Meta, StoryObj } from "@storybook/react";
import { Box, Circle, Cylinder, Hexagon } from "lucide-react";
import { Level, LevelProvider, Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectScrollDownButton, SelectScrollUpButton, SelectSeparator, SelectTrigger, SelectValue, getLevelBgClass } from "../../../../sketchpad/elements";

// #region Select
const meta = {
  title: "Elements/Input/Select",
  component: Select,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Select>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "select-default",
  showLabel: true,
  defaultValue: "capsule",
};

export const Default: Story = {
  args: defaultArgs,
  render: (args) => (
    <Select {...args}>
      <SelectTrigger id="select-trigger-default" size="default" className="w-[220px]">
        <SelectValue placeholder="Select a type" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>Modular Components</SelectLabel>
          <SelectItem value="capsule">
            <div className="flex items-center gap-double">
              <Box className="size-tiny" />
              Capsule
            </div>
          </SelectItem>
          <SelectItem value="base">
            <div className="flex items-center gap-double">
              <Circle className="size-tiny" />
              Base
            </div>
          </SelectItem>
          <SelectItem value="tambour">
            <div className="flex items-center gap-double">
              <Cylinder className="size-tiny" />
              Tambour
            </div>
          </SelectItem>
        </SelectGroup>
        <SelectSeparator />
        <SelectGroup>
          <SelectLabel>Structural</SelectLabel>
          <SelectItem value="capital">
            <div className="flex items-center gap-double">
              <Hexagon className="size-tiny" />
              Capital
            </div>
          </SelectItem>
        </SelectGroup>
        <SelectScrollUpButton />
        <SelectScrollDownButton />
      </SelectContent>
    </Select>
  ),
};

const SelectDemo = ({ id }: { id: string }) => (
  <Select {...defaultArgs} id={id}>
    <SelectTrigger id={`select-trigger-${id}`} size="default" className="w-[220px]">
      <SelectValue placeholder="Select a type" />
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        <SelectLabel>Modular Components</SelectLabel>
        <SelectItem value="capsule">
          <div className="flex items-center gap-double">
            <Box className="size-tiny" />
            Capsule
          </div>
        </SelectItem>
        <SelectItem value="base">
          <div className="flex items-center gap-double">
            <Circle className="size-tiny" />
            Base
          </div>
        </SelectItem>
      </SelectGroup>
    </SelectContent>
  </Select>
);

const createLevelRender = (level: Level, id: string) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <SelectDemo id={id} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { ...defaultArgs, id: "select-base" },
  render: createLevelRender("base", "select-base"),
};

export const Window: Story = {
  args: { ...defaultArgs, id: "select-window" },
  render: createLevelRender("window", "select-window"),
};

export const Panel: Story = {
  args: { ...defaultArgs, id: "select-panel" },
  render: createLevelRender("panel", "select-panel"),
};

export const Overlay: Story = {
  args: { ...defaultArgs, id: "select-overlay" },
  render: createLevelRender("overlay", "select-overlay"),
};

export const Temporary: Story = {
  args: { ...defaultArgs, id: "select-temporary" },
  render: createLevelRender("temporary", "select-temporary"),
};

// #endregion Select
