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
import { AlignCenter, AlignLeft, AlignRight, Bold, Clipboard, Copy, Italic, Redo, Underline, Undo } from "lucide-react";
import { ButtonGroup, ButtonGroupItem } from "./ButtonGroup";

const meta = {
  title: "Elements/ButtonGroup",
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
    <ButtonGroup>
      <ButtonGroupItem>Option 1</ButtonGroupItem>
      <ButtonGroupItem>Option 2</ButtonGroupItem>
      <ButtonGroupItem>Option 3</ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const WithIcons: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem>
        <Bold />
      </ButtonGroupItem>
      <ButtonGroupItem>
        <Italic />
      </ButtonGroupItem>
      <ButtonGroupItem>
        <Underline />
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const WithTooltips: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem tooltip="Bold" hotkey="Ctrl+B">
        <Bold />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Italic" hotkey="Ctrl+I">
        <Italic />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Underline" hotkey="Ctrl+U">
        <Underline />
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const TextAlignment: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem tooltip="Align Left">
        <AlignLeft />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Align Center">
        <AlignCenter />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Align Right">
        <AlignRight />
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const EditingActions: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem tooltip="Undo" hotkey="Ctrl+Z">
        <Undo />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Redo" hotkey="Ctrl+Y">
        <Redo />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Copy" hotkey="Ctrl+C">
        <Copy />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Paste" hotkey="Ctrl+V">
        <Clipboard />
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const Outline: Story = {
  render: () => (
    <ButtonGroup variant="outline">
      <ButtonGroupItem>
        <Bold />
      </ButtonGroupItem>
      <ButtonGroupItem>
        <Italic />
      </ButtonGroupItem>
      <ButtonGroupItem>
        <Underline />
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const Sizes: Story = {
  render: () => (
    <div className="flex flex-col items-start gap-4">
      <ButtonGroup size="sm">
        <ButtonGroupItem>Small</ButtonGroupItem>
        <ButtonGroupItem>Small</ButtonGroupItem>
        <ButtonGroupItem>Small</ButtonGroupItem>
      </ButtonGroup>
      <ButtonGroup size="default">
        <ButtonGroupItem>Default</ButtonGroupItem>
        <ButtonGroupItem>Default</ButtonGroupItem>
        <ButtonGroupItem>Default</ButtonGroupItem>
      </ButtonGroup>
      <ButtonGroup size="lg">
        <ButtonGroupItem>Large</ButtonGroupItem>
        <ButtonGroupItem>Large</ButtonGroupItem>
        <ButtonGroupItem>Large</ButtonGroupItem>
      </ButtonGroup>
    </div>
  ),
};

export const Disabled: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem>Active</ButtonGroupItem>
      <ButtonGroupItem disabled>Disabled</ButtonGroupItem>
      <ButtonGroupItem>Active</ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const MixedContent: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem>
        <Undo />
        Undo
      </ButtonGroupItem>
      <ButtonGroupItem>
        <Redo />
        Redo
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};
