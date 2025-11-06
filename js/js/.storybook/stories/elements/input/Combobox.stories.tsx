// #region Header

// Combobox.stories.tsx

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
import { Combobox } from "../../../../sketchpad/elements";

// #region Combobox
const meta = {
  title: "Elements/Input/Combobox",
  component: Combobox,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Combobox>;

export default meta;

type Story = StoryObj<typeof meta>;

const types = [
  { value: "capsule", label: "Capsule" },
  { value: "base", label: "Base" },
  { value: "tambour", label: "Tambour" },
  { value: "capital", label: "Capital" },
  { value: "cluster", label: "Cluster" },
];

export const Default: Story = {
  render: () => {
    const [value, setValue] = useState("capsule");
    return <Combobox id="combobox-default" options={types} value={value} onValueChange={setValue} placeholder="Select type..." placeholderId="combobox.placeholder" emptyMessage="No types match your search." allowClear showLabel className="w-96" />;
  },
};

// #endregion Combobox
