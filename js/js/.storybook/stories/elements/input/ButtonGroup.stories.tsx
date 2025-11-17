// #region Header

// ButtonGroup.stories.tsx

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
import { ButtonGroup, ButtonGroupItem } from "../../../../sketchpad/elements";

// #region ButtonGroup
const meta = {
  title: "Elements/Input/ButtonGroup",
  component: ButtonGroup,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ButtonGroup>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <ButtonGroup id="button-group-default" variant="default" level="base" defaultValue="model" showLabel>
      <ButtonGroupItem id="button-group-default-model" value="model" icon={<Box />} />
      <ButtonGroupItem id="button-group-default-diagram" value="diagram" icon={<Network />} />
      <ButtonGroupItem id="button-group-default-details" value="details" icon={<List />} />
    </ButtonGroup>
  ),
};

// #endregion ButtonGroup
