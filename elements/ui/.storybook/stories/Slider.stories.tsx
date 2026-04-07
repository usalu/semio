// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/input/Slider.stories.tsx

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

import { Slider } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";

// #region 🔖Slider
const meta = {
  title: "elements/Slider",
  component: Slider,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Slider>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "slider-default",
  value: [75] as number[],
  onValueChange: () => {},
  min: 50,
  max: 150,
  step: 5,
  showLabel: true,
  onPointerDown: () => {},
  onPointerUp: () => {},
  onPointerCancel: () => {},
  interactionId: "slider-interaction",
  className: "w-96",
};

export const Default: Story = {
  args: defaultArgs,
  render: (args) => {
    const [value, setValue] = useState(args.value);
    return <Slider {...args} value={value} onValueChange={setValue} />;
  },
};

// #endregion 🔖Slider
