// #region Header

// js/js/.storybook/stories/elements/input/Stepper.stories.tsx

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
    level: "base",
  },
  render: (args) => {
    const [value, setValue] = useState(args.value);
    return <Stepper {...args} value={value} onChange={setValue} />;
  },
};

export const Base: Story = {
  args: { ...Default.args, id: "stepper-base", level: "base" },
  render: Default.render,
};

export const Window: Story = {
  args: { ...Default.args, id: "stepper-window", level: "window" },
  render: Default.render,
};

export const Panel: Story = {
  args: { ...Default.args, id: "stepper-panel", level: "panel" },
  render: Default.render,
};

export const Overlay: Story = {
  args: { ...Default.args, id: "stepper-overlay", level: "overlay" },
  render: Default.render,
};

export const Temporary: Story = {
  args: { ...Default.args, id: "stepper-temporary", level: "temporary" },
  render: Default.render,
};

// #endregion Stepper


