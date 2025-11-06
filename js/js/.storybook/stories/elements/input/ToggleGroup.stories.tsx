// #region Header

// ToggleGroup.stories.tsx

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
import { Box, List, Network } from "lucide-react";
import { useState } from "react";
import { ToggleGroup, ToggleGroupItem } from "../../../../sketchpad/elements";

// #region ToggleGroup
const meta = {
  title: "Elements/Input/ToggleGroup",
  component: ToggleGroup,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ToggleGroup>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [value, setValue] = useState("model");
    return (
      <ToggleGroup id="toggle-group-default" type="single" value={value} onValueChange={setValue} level="base">
        <ToggleGroupItem id="toggle-group-default-model" value="model" icon={<Box />} />
        <ToggleGroupItem id="toggle-group-default-diagram" value="diagram" icon={<Network />} />
        <ToggleGroupItem id="toggle-group-default-list" value="list" icon={<List />} />
      </ToggleGroup>
    );
  },
};

// #endregion ToggleGroup
