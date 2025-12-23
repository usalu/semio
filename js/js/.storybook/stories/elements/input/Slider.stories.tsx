// #region Header

// js/js/.storybook/stories/elements/input/Slider.stories.tsx

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
import { Slider } from "../../../../sketchpad/elements";

// #region Slider
const meta = {
  title: "Elements/Input/Slider",
  component: Slider,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Slider>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "slider-default",
    value: [75],
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
    level: "base",
  },
  render: (args) => {
    const [value, setValue] = useState(args.value);
    return <Slider {...args} value={value} onValueChange={setValue} />;
  },
};

export const Base: Story = {
  args: { ...Default.args, id: "slider-base", level: "base" },
  render: Default.render,
};

export const Window: Story = {
  args: { ...Default.args, id: "slider-window", level: "window" },
  render: Default.render,
};

export const Panel: Story = {
  args: { ...Default.args, id: "slider-panel", level: "panel" },
  render: Default.render,
};

export const Overlay: Story = {
  args: { ...Default.args, id: "slider-overlay", level: "overlay" },
  render: Default.render,
};

export const Temporary: Story = {
  args: { ...Default.args, id: "slider-temporary", level: "temporary" },
  render: Default.render,
};

// #endregion Slider


