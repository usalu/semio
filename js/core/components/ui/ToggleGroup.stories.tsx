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
import { AlignCenter, AlignLeft, AlignRight, Bold, Italic, Underline } from "lucide-react";
import { ToggleGroup, ToggleGroupItem } from "./ToggleGroup";

const meta = {
  title: "Elements/ToggleGroup",
  component: ToggleGroup,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ToggleGroup>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Single: Story = {
  render: () => (
    <ToggleGroup type="single">
      <ToggleGroupItem value="left">
        <AlignLeft />
      </ToggleGroupItem>
      <ToggleGroupItem value="center">
        <AlignCenter />
      </ToggleGroupItem>
      <ToggleGroupItem value="right">
        <AlignRight />
      </ToggleGroupItem>
    </ToggleGroup>
  ),
};

export const Multiple: Story = {
  render: () => (
    <ToggleGroup type="multiple">
      <ToggleGroupItem value="bold">
        <Bold />
      </ToggleGroupItem>
      <ToggleGroupItem value="italic">
        <Italic />
      </ToggleGroupItem>
      <ToggleGroupItem value="underline">
        <Underline />
      </ToggleGroupItem>
    </ToggleGroup>
  ),
};

export const WithTooltips: Story = {
  render: () => (
    <ToggleGroup type="multiple">
      <ToggleGroupItem value="bold" tooltip="Bold" hotkey="Ctrl+B">
        <Bold />
      </ToggleGroupItem>
      <ToggleGroupItem value="italic" tooltip="Italic" hotkey="Ctrl+I">
        <Italic />
      </ToggleGroupItem>
      <ToggleGroupItem value="underline" tooltip="Underline" hotkey="Ctrl+U">
        <Underline />
      </ToggleGroupItem>
    </ToggleGroup>
  ),
};

export const Outline: Story = {
  render: () => (
    <ToggleGroup type="single" variant="outline">
      <ToggleGroupItem value="left">
        <AlignLeft />
      </ToggleGroupItem>
      <ToggleGroupItem value="center">
        <AlignCenter />
      </ToggleGroupItem>
      <ToggleGroupItem value="right">
        <AlignRight />
      </ToggleGroupItem>
    </ToggleGroup>
  ),
};

export const Sizes: Story = {
  render: () => (
    <div className="flex flex-col items-start gap-4">
      <ToggleGroup type="single" size="sm">
        <ToggleGroupItem value="left">Left</ToggleGroupItem>
        <ToggleGroupItem value="center">Center</ToggleGroupItem>
        <ToggleGroupItem value="right">Right</ToggleGroupItem>
      </ToggleGroup>
      <ToggleGroup type="single" size="default">
        <ToggleGroupItem value="left">Left</ToggleGroupItem>
        <ToggleGroupItem value="center">Center</ToggleGroupItem>
        <ToggleGroupItem value="right">Right</ToggleGroupItem>
      </ToggleGroup>
      <ToggleGroup type="single" size="lg">
        <ToggleGroupItem value="left">Left</ToggleGroupItem>
        <ToggleGroupItem value="center">Center</ToggleGroupItem>
        <ToggleGroupItem value="right">Right</ToggleGroupItem>
      </ToggleGroup>
    </div>
  ),
};

export const WithText: Story = {
  render: () => (
    <ToggleGroup type="multiple">
      <ToggleGroupItem value="bold">
        <Bold />
        Bold
      </ToggleGroupItem>
      <ToggleGroupItem value="italic">
        <Italic />
        Italic
      </ToggleGroupItem>
      <ToggleGroupItem value="underline">
        <Underline />
        Underline
      </ToggleGroupItem>
    </ToggleGroup>
  ),
};

export const Disabled: Story = {
  render: () => (
    <ToggleGroup type="single" disabled>
      <ToggleGroupItem value="left">
        <AlignLeft />
      </ToggleGroupItem>
      <ToggleGroupItem value="center">
        <AlignCenter />
      </ToggleGroupItem>
      <ToggleGroupItem value="right">
        <AlignRight />
      </ToggleGroupItem>
    </ToggleGroup>
  ),
};

export const DefaultValue: Story = {
  render: () => (
    <ToggleGroup type="single" defaultValue="center">
      <ToggleGroupItem value="left">
        <AlignLeft />
      </ToggleGroupItem>
      <ToggleGroupItem value="center">
        <AlignCenter />
      </ToggleGroupItem>
      <ToggleGroupItem value="right">
        <AlignRight />
      </ToggleGroupItem>
    </ToggleGroup>
  ),
};
