// #region Header

// js/js/.storybook/stories/elements/input/ToggleGroup.stories.tsx

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
import { Box, List, Lock, Network, Plus, Settings } from "lucide-react";
import { Action, Level, LevelProvider, ToggleGroup, getLevelBgClass } from "../../../../sketchpad/elements";

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

const defaultItems = [
  { id: "toggle-default-standard", value: "standard", icon: <Lock /> },
  { id: "toggle-action-settings", value: "settings", icon: <Settings />, action: <Action id="toggle-action-settings-add" icon={<Plus />} /> },
  { id: "toggle-dropdown-box", value: "box", icon: <Box />, action: <Action id="toggle-dropdown-box-action" icon={<Network />} /> },
];

const multipleItems = [
  { id: "toggle-multiple-standard", value: "standard", icon: <Lock /> },
  { id: "toggle-multiple-box", value: "box", icon: <Box /> },
  { id: "toggle-multiple-network", value: "network", icon: <Network /> },
  { id: "toggle-multiple-list", value: "list", icon: <List /> },
  { id: "toggle-multiple-settings", value: "settings", icon: <Settings /> },
  { id: "toggle-multiple-plus", value: "plus", icon: <Plus /> },
];

export const Default: Story = {
  args: {
    id: "toggle-group-default",
    kind: "single",
    defaultValue: "standard",
    showLabel: true,
    items: defaultItems,
  },
};

const createSingleLevelRender = (level: Level, id: string) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <ToggleGroup id={id} kind="single" defaultValue="standard" showLabel items={defaultItems} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { items: defaultItems },
  render: createSingleLevelRender("base", "toggle-group-base"),
};

export const Window: Story = {
  args: { items: defaultItems },
  render: createSingleLevelRender("window", "toggle-group-window"),
};

export const Panel: Story = {
  args: { items: defaultItems },
  render: createSingleLevelRender("panel", "toggle-group-panel"),
};

export const Overlay: Story = {
  args: { items: defaultItems },
  render: createSingleLevelRender("overlay", "toggle-group-overlay"),
};

export const Temporary: Story = {
  args: { items: defaultItems },
  render: createSingleLevelRender("temporary", "toggle-group-temporary"),
};

export const Multiple: Story = {
  args: {
    id: "toggle-group-multiple",
    kind: "multiple",
    defaultValue: ["box"],
    showLabel: true,
    items: multipleItems,
  },
};

const createMultipleLevelRender = (level: Level, id: string) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <ToggleGroup id={id} kind="multiple" defaultValue={["box"]} showLabel items={multipleItems} />
    </div>
  </LevelProvider>
);

export const MultipleBase: Story = {
  args: { items: multipleItems },
  render: createMultipleLevelRender("base", "toggle-group-multiple-base"),
};

export const MultipleWindow: Story = {
  args: { items: multipleItems },
  render: createMultipleLevelRender("window", "toggle-group-multiple-window"),
};

export const MultiplePanel: Story = {
  args: { items: multipleItems },
  render: createMultipleLevelRender("panel", "toggle-group-multiple-panel"),
};

export const MultipleOverlay: Story = {
  args: { items: multipleItems },
  render: createMultipleLevelRender("overlay", "toggle-group-multiple-overlay"),
};

export const MultipleTemporary: Story = {
  args: { items: multipleItems },
  render: createMultipleLevelRender("temporary", "toggle-group-multiple-temporary"),
};

// #endregion ToggleGroup


