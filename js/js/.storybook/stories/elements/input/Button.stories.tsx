// #region Header

// js/js/.storybook/stories/elements/input/Button.stories.tsx

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

// Button.stories.tsx

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
import { Box, List, Network, Plus } from "lucide-react";
import { useState } from "react";
import { Button, ButtonCycle } from "../../../../sketchpad/elements";

// #region Button
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
    level: "base",
  },
};

export const Base: Story = {
  args: { ...Default.args, id: "button-base", level: "base" },
};

export const Window: Story = {
  args: { ...Default.args, id: "button-window", level: "window" },
};

export const Panel: Story = {
  args: { ...Default.args, id: "button-panel", level: "panel" },
};

export const Overlay: Story = {
  args: { ...Default.args, id: "button-overlay", level: "overlay" },
};

export const Temporary: Story = {
  args: { ...Default.args, id: "button-temporary", level: "temporary" },
};

// #endregion Button

// #region ButtonCycle

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
    level: "base",
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

// #endregion ButtonCycle
