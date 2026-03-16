// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/input/Toggle.stories.tsx

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
import { Box, List, Lock, Network, Plus, Settings } from "lucide-react";
import { Level, LevelProvider, Toggle, getLevelBgClass } from "@semio-elements/ui";

// #region 🔖Toggle
const meta = {
  title: "semio-elements/Toggle",
  component: Toggle,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Toggle>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "toggle-default",
  defaultPressed: true,
  icon: <Lock />,
  showLabel: true,
};

export const Default: Story = {
  args: defaultArgs,
};

const createLevelRender = (level: Level, id: string) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <Toggle {...defaultArgs} id={id} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { ...defaultArgs, id: "toggle-base" },
  render: createLevelRender("base", "toggle-base"),
};

export const Window: Story = {
  args: { ...defaultArgs, id: "toggle-window" },
  render: createLevelRender("window", "toggle-window"),
};

export const Panel: Story = {
  args: { ...defaultArgs, id: "toggle-panel" },
  render: createLevelRender("panel", "toggle-panel"),
};

export const Overlay: Story = {
  args: { ...defaultArgs, id: "toggle-overlay" },
  render: createLevelRender("overlay", "toggle-overlay"),
};

export const Temporary: Story = {
  args: { ...defaultArgs, id: "toggle-temporary" },
  render: createLevelRender("temporary", "toggle-temporary"),
};

const withActionArgs = {
  id: "toggle-action",
  kind: "withAction" as const,
  defaultPressed: false,
  icon: <Settings />,
  actionIcon: <Plus />,
  onActionClick: () => console.log("Action clicked"),
  actionId: "toggle-action-button",
  showLabel: true,
};

export const WithAction: Story = {
  args: withActionArgs,
};

const createWithActionLevelRender = (level: Level, id: string) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <Toggle {...withActionArgs} id={id} />
    </div>
  </LevelProvider>
);

export const WithActionBase: Story = {
  args: { ...withActionArgs, id: "toggle-action-base" },
  render: createWithActionLevelRender("base", "toggle-action-base"),
};

export const WithActionWindow: Story = {
  args: { ...withActionArgs, id: "toggle-action-window" },
  render: createWithActionLevelRender("window", "toggle-action-window"),
};

export const WithActionPanel: Story = {
  args: { ...withActionArgs, id: "toggle-action-panel" },
  render: createWithActionLevelRender("panel", "toggle-action-panel"),
};

export const WithActionOverlay: Story = {
  args: { ...withActionArgs, id: "toggle-action-overlay" },
  render: createWithActionLevelRender("overlay", "toggle-action-overlay"),
};

export const WithActionTemporary: Story = {
  args: { ...withActionArgs, id: "toggle-action-temporary" },
  render: createWithActionLevelRender("temporary", "toggle-action-temporary"),
};

const dropdownArgs = {
  id: "toggle-dropdown",
  kind: "dropdown" as const,
  defaultValue: "option1",
  items: [
    { value: "option1", label: <Box /> },
    { value: "option2", label: <Network /> },
    { value: "option3", label: <List /> },
  ],
  dropdownId: "toggle-dropdown-action",
  showLabel: true,
};

export const Dropdown: Story = {
  args: dropdownArgs,
};

const createDropdownLevelRender = (level: Level, id: string) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <Toggle {...dropdownArgs} id={id} />
    </div>
  </LevelProvider>
);

export const DropdownBase: Story = {
  args: { ...dropdownArgs, id: "toggle-dropdown-base" },
  render: createDropdownLevelRender("base", "toggle-dropdown-base"),
};

export const DropdownWindow: Story = {
  args: { ...dropdownArgs, id: "toggle-dropdown-window" },
  render: createDropdownLevelRender("window", "toggle-dropdown-window"),
};

export const DropdownPanel: Story = {
  args: { ...dropdownArgs, id: "toggle-dropdown-panel" },
  render: createDropdownLevelRender("panel", "toggle-dropdown-panel"),
};

export const DropdownOverlay: Story = {
  args: { ...dropdownArgs, id: "toggle-dropdown-overlay" },
  render: createDropdownLevelRender("overlay", "toggle-dropdown-overlay"),
};

export const DropdownTemporary: Story = {
  args: { ...dropdownArgs, id: "toggle-dropdown-temporary" },
  render: createDropdownLevelRender("temporary", "toggle-dropdown-temporary"),
};

// #endregion 🔖Toggle
