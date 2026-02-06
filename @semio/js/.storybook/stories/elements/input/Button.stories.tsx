// #region 🔖Header

// js/semio/.storybook/stories/elements/input/Button.stories.tsx

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
import { Box, List, Network, Plus } from "lucide-react";
import { useState } from "react";
import { Button, ButtonCycle, Level, LevelProvider, getLevelBgClass } from "../../../../sketchpad/elements";

// #region 🔖Button
const meta = {
  title: "Elements/Input/Button",
  component: Button,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Button>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "button-default",
    icon: <Plus />,
  },
};

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <Button id={`button-${level}`} icon={<Plus />} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { id: "button-base", icon: <Plus /> },
  render: createLevelRender("base"),
};

export const Window: Story = {
  args: { id: "button-window", icon: <Plus /> },
  render: createLevelRender("window"),
};

export const Panel: Story = {
  args: { id: "button-panel", icon: <Plus /> },
  render: createLevelRender("panel"),
};

export const Overlay: Story = {
  args: { id: "button-overlay", icon: <Plus /> },
  render: createLevelRender("overlay"),
};

export const Temporary: Story = {
  args: { id: "button-temporary", icon: <Plus /> },
  render: createLevelRender("temporary"),
};

// #endregion 🔖Button

// #region 🔖ButtonCycle

const cycleMeta = {
  title: "Elements/Input/Button",
  component: ButtonCycle,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ButtonCycle>;

type CycleStory = StoryObj<typeof cycleMeta>;

export const Cycle: CycleStory = {
  args: {
    id: "button-cycle",
    showLabel: true,
    items: [],
  },
  render: (args) => {
    const [value, setValue] = useState("view1");
    return (
      <ButtonCycle
        {...args}
        value={value}
        onValueChange={setValue}
        items={[
          { value: "view1", label: <Box /> },
          { value: "view2", label: <Network /> },
          { value: "view3", label: <List /> },
        ]}
      />
    );
  },
};

// #endregion 🔖ButtonCycle
