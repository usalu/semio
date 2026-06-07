// #region 🧲Header

// 🥼︎ semio/js/.storybook/stories/elements/input/Stepper.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Stepper } from "@ui/react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";

// 🔷#region 🏬Stepper
const meta = {
  title: "🖱️ui⚛️react/Stepper",
  component: Stepper,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Stepper>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "stepper-default",
  value: 12,
  onChange: () => { },
  min: 1,
  max: 50,
  step: 1,
  onPointerDown: () => { },
  onPointerUp: () => { },
  onPointerCancel: () => { },
  interactionId: "stepper-interaction",
};

export const Default: Story = {
  args: defaultArgs,
  render: (args) => {
    const [value, setValue] = useState(args.value);
    return <Stepper {...args} value={value} onChange={setValue} />;
  },
};

// #endregion 🏬Stepper
