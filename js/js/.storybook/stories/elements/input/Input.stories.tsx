// #region Header

// js/js/.storybook/stories/elements/input/Input.stories.tsx

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
import { Input } from "../../../../sketchpad/elements";

// #region Input
const meta = {
  title: "Elements/Input/Input",
  component: Input,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Input>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "input-default",
    placeholder: "e.g., Nakagin Tower Configuration",
    defaultValue: "Metabolism Kit - Capsule Cluster",
    type: "text",
    lazy: true,
    showLabel: true,
    disabled: false,
    "aria-invalid": false,
    className: "w-96",
    level: "base",
  },
};

export const Base: Story = {
  args: { ...Default.args, id: "input-base", level: "base" },
};

export const Window: Story = {
  args: { ...Default.args, id: "input-window", level: "window" },
};

export const Panel: Story = {
  args: { ...Default.args, id: "input-panel", level: "panel" },
};

export const Overlay: Story = {
  args: { ...Default.args, id: "input-overlay", level: "overlay" },
};

export const Temporary: Story = {
  args: { ...Default.args, id: "input-temporary", level: "temporary" },
};

// #endregion Input


