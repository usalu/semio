// #region Header

// Stepper.stories.tsx

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
import { useState } from "react";
import { Stepper } from "../../../../sketchpad/elements";

// #region Stepper
const meta = {
  title: "Elements/Input/Stepper",
  component: Stepper,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Stepper>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "stepper-default",
    value: 12,
    onChange: () => {},
    min: 1,
    max: 50,
    step: 1,
    onPointerDown: () => {},
    onPointerUp: () => {},
    onPointerCancel: () => {},
    interactionId: "stepper-interaction",
  },
  render: (args) => {
    const [value, setValue] = useState(args.value);
    return (
      <Stepper {...args} value={value} onChange={setValue} />
    );
  },
};

// #endregion Stepper
