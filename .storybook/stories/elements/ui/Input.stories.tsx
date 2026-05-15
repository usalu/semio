// #region 🧲Header

// 🥼︎ semio/js/.storybook/stories/elements/input/Input.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Input } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";

// 📝#region 🩺Input
const meta = {
  title: "elements/react/Input",
  component: Input,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Input>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "input-default",
  placeholder: "e.g., Nakagin Tower Configuration",
  defaultValue: "Metabolism Kit - Capsule Cluster",
  type: "text" as const,
  lazy: true,
  showLabel: true,
  disabled: false,
  "aria-invalid": false,
  className: "w-96",
};

export const Default: Story = {
  args: defaultArgs,
};

export const OverflowCollapsed: Story = {
  args: {
    ...defaultArgs,
    value: "Nakagin Capsule Tower Clustered Residential Aggregation With Extended Metadata",
    className: "w-40",
    readOnly: true,
  },
};

// #endregion 🩺Input
