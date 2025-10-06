// #region Header

// Select.stories.tsx

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
import { Box, Circle, Cylinder, Hexagon } from "lucide-react";
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger, SelectValue } from "./Select";

const meta = {
  title: "Elements/Select",
  component: Select,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Select>;

export default meta;
type Story = StoryObj<typeof meta>;

const TypeIcons = { Box, Circle, Cylinder, Hexagon };

export const Basic: Story = {
  render: () => (
    <Select>
      <SelectTrigger className="w-[180px]">
        <SelectValue placeholder="Select a type" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="capsule">Capsule</SelectItem>
        <SelectItem value="base">Base</SelectItem>
        <SelectItem value="tambour">Tambour</SelectItem>
        <SelectItem value="capital">Capital</SelectItem>
        <SelectItem value="cluster">Cluster</SelectItem>
      </SelectContent>
    </Select>
  ),
};

export const WithGroups: Story = {
  render: () => (
    <Select>
      <SelectTrigger className="w-[200px]">
        <SelectValue placeholder="Select a piece" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>Capsules</SelectLabel>
          <SelectItem value="capsule-j">Capsule J</SelectItem>
          <SelectItem value="capsule-l">Capsule L</SelectItem>
          <SelectItem value="capsule-p">Capsule P</SelectItem>
        </SelectGroup>
        <SelectSeparator />
        <SelectGroup>
          <SelectLabel>Bases</SelectLabel>
          <SelectItem value="base-blob">Base Blob</SelectItem>
          <SelectItem value="base-standard">Base Standard</SelectItem>
          <SelectItem value="base-circular">Base Circular</SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
  ),
};

export const WithIcons: Story = {
  render: () => (
    <Select>
      <SelectTrigger className="w-[200px]">
        <SelectValue placeholder="Select a type" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="capsule">
          <Box />
          <span>Capsule</span>
        </SelectItem>
        <SelectItem value="base">
          <Circle />
          <span>Base</span>
        </SelectItem>
        <SelectItem value="tambour">
          <Cylinder />
          <span>Tambour</span>
        </SelectItem>
        <SelectItem value="capital">
          <Hexagon />
          <span>Capital</span>
        </SelectItem>
      </SelectContent>
    </Select>
  ),
};

export const SmallSize: Story = {
  render: () => (
    <Select>
      <SelectTrigger size="sm" className="w-[180px]">
        <SelectValue placeholder="Select..." />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="layer1">Layer 1</SelectItem>
        <SelectItem value="layer2">Layer 2</SelectItem>
        <SelectItem value="layer3">Layer 3</SelectItem>
      </SelectContent>
    </Select>
  ),
};

export const Disabled: Story = {
  render: () => (
    <Select disabled>
      <SelectTrigger className="w-[180px]">
        <SelectValue placeholder="Select a type" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="capsule">Capsule</SelectItem>
        <SelectItem value="base">Base</SelectItem>
      </SelectContent>
    </Select>
  ),
};

export const DisabledItems: Story = {
  render: () => (
    <Select>
      <SelectTrigger className="w-[180px]">
        <SelectValue placeholder="Select..." />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="capsule">Capsule</SelectItem>
        <SelectItem value="base" disabled>
          Base (Unavailable)
        </SelectItem>
        <SelectItem value="tambour">Tambour</SelectItem>
      </SelectContent>
    </Select>
  ),
};

export const DefaultValue: Story = {
  render: () => (
    <Select defaultValue="base">
      <SelectTrigger className="w-[180px]">
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="capsule">Capsule</SelectItem>
        <SelectItem value="base">Base</SelectItem>
        <SelectItem value="tambour">Tambour</SelectItem>
      </SelectContent>
    </Select>
  ),
};

export const ManyOptions: Story = {
  render: () => (
    <Select>
      <SelectTrigger className="w-[200px]">
        <SelectValue placeholder="Select a quality" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="volume">Volume</SelectItem>
        <SelectItem value="area">Area</SelectItem>
        <SelectItem value="height">Height</SelectItem>
        <SelectItem value="width">Width</SelectItem>
        <SelectItem value="depth">Depth</SelectItem>
        <SelectItem value="mass">Mass</SelectItem>
        <SelectItem value="density">Density</SelectItem>
        <SelectItem value="cost">Cost</SelectItem>
        <SelectItem value="energy">Energy</SelectItem>
        <SelectItem value="carbon">Carbon</SelectItem>
        <SelectItem value="materials">Materials</SelectItem>
        <SelectItem value="pieces">Pieces</SelectItem>
        <SelectItem value="connections">Connections</SelectItem>
        <SelectItem value="ports">Ports</SelectItem>
      </SelectContent>
    </Select>
  ),
};
