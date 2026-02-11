// #region 🔖Header

// 🧪︎ semio/js/.storybook/stories/elements/Command.stories.tsx

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
import { Box, Circle, Cylinder, Settings, User } from "lucide-react";
import { useState } from "react";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut, Level, LevelProvider, getLevelBgClass } from "../../../sketchpad/elements";

// #region 🔖Command
const meta = {
  title: "Elements/Command",
  component: Command,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Command>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
  render: () => {
    const [value, setValue] = useState("");
    return (
      <Command className="border w-96" value={value} onValueChange={setValue}>
        <CommandInput placeholder="Search types..." />
        <CommandList>
          <CommandEmpty>No types found.</CommandEmpty>
          <CommandGroup heading="Types">
            <CommandItem value="capsule">
              <Box />
              <span>Capsule</span>
            </CommandItem>
            <CommandItem value="base">
              <Circle />
              <span>Base</span>
            </CommandItem>
            <CommandItem value="tambour">
              <Cylinder />
              <span>Tambour</span>
            </CommandItem>
          </CommandGroup>
          <CommandGroup heading="Settings">
            <CommandItem value="profile">
              <User />
              <span>Profile</span>
              <CommandShortcut>⌘P</CommandShortcut>
            </CommandItem>
            <CommandItem value="settings">
              <Settings />
              <span>Settings</span>
              <CommandShortcut>⌘S</CommandShortcut>
            </CommandItem>
          </CommandGroup>
        </CommandList>
      </Command>
    );
  },
};

const CommandDemo = () => {
  const [value, setValue] = useState("");
  return (
    <Command className="border w-96" value={value} onValueChange={setValue}>
      <CommandInput placeholder="Search types..." />
      <CommandList>
        <CommandEmpty>No types found.</CommandEmpty>
        <CommandGroup heading="Types">
          <CommandItem value="capsule">
            <Box />
            <span>Capsule</span>
          </CommandItem>
          <CommandItem value="base">
            <Circle />
            <span>Base</span>
          </CommandItem>
          <CommandItem value="tambour">
            <Cylinder />
            <span>Tambour</span>
          </CommandItem>
        </CommandGroup>
        <CommandGroup heading="Settings">
          <CommandItem value="profile">
            <User />
            <span>Profile</span>
            <CommandShortcut>⌘P</CommandShortcut>
          </CommandItem>
          <CommandItem value="settings">
            <Settings />
            <span>Settings</span>
            <CommandShortcut>⌘S</CommandShortcut>
          </CommandItem>
        </CommandGroup>
      </CommandList>
    </Command>
  );
};

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <CommandDemo />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  render: createLevelRender("base"),
};

export const Window: Story = {
  render: createLevelRender("window"),
};

export const Panel: Story = {
  render: createLevelRender("panel"),
};

export const Overlay: Story = {
  render: createLevelRender("overlay"),
};

export const Temporary: Story = {
  render: createLevelRender("temporary"),
};

// #endregion 🔖Command
