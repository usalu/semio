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
import { Box, Clipboard, Copy, Eye, Grid, Layers, Move, Redo, RotateCw, Undo } from "lucide-react";
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

export const Variants: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <ButtonGroup>
        <ButtonGroupItem>Default</ButtonGroupItem>
        <ButtonGroupItem>Default</ButtonGroupItem>
        <ButtonGroupItem>Default</ButtonGroupItem>
      </ButtonGroup>
      <ButtonGroup variant="outline">
        <ButtonGroupItem>Outline</ButtonGroupItem>
        <ButtonGroupItem>Outline</ButtonGroupItem>
        <ButtonGroupItem>Outline</ButtonGroupItem>
      </ButtonGroup>
    </div>
  ),
};

export const Default: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem>Model</ButtonGroupItem>
      <ButtonGroupItem>Diagram</ButtonGroupItem>
      <ButtonGroupItem>Details</ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const WithIcons: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem>
        <Box />
      </ButtonGroupItem>
      <ButtonGroupItem>
        <Grid />
      </ButtonGroupItem>
      <ButtonGroupItem>
        <Layers />
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const WithTooltips: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem tooltip="3D View" hotkey="Ctrl+1">
        <Box />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Grid View" hotkey="Ctrl+2">
        <Grid />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Layers" hotkey="Ctrl+3">
        <Layers />
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const TextAlignment: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem tooltip="Move">
        <Move />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Rotate">
        <RotateCw />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Preview">
        <Eye />
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
      <ButtonGroupItem tooltip="Copy Piece" hotkey="Ctrl+C">
        <Copy />
      </ButtonGroupItem>
      <ButtonGroupItem tooltip="Paste Piece" hotkey="Ctrl+V">
        <Clipboard />
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const Outline: Story = {
  render: () => (
    <ButtonGroup variant="outline">
      <ButtonGroupItem>
        <Box />
      </ButtonGroupItem>
      <ButtonGroupItem>
        <Grid />
      </ButtonGroupItem>
      <ButtonGroupItem>
        <Layers />
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const Sizes: Story = {
  render: () => (
    <div className="flex flex-col items-start gap-4">
      <ButtonGroup size="sm">
        <ButtonGroupItem>Model</ButtonGroupItem>
        <ButtonGroupItem>Diagram</ButtonGroupItem>
        <ButtonGroupItem>Details</ButtonGroupItem>
      </ButtonGroup>
      <ButtonGroup size="default">
        <ButtonGroupItem>Model</ButtonGroupItem>
        <ButtonGroupItem>Diagram</ButtonGroupItem>
        <ButtonGroupItem>Details</ButtonGroupItem>
      </ButtonGroup>
      <ButtonGroup size="lg">
        <ButtonGroupItem>Model</ButtonGroupItem>
        <ButtonGroupItem>Diagram</ButtonGroupItem>
        <ButtonGroupItem>Details</ButtonGroupItem>
      </ButtonGroup>
    </div>
  ),
};

export const Disabled: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem>Model</ButtonGroupItem>
      <ButtonGroupItem disabled>Locked View</ButtonGroupItem>
      <ButtonGroupItem>Details</ButtonGroupItem>
    </ButtonGroup>
  ),
};

export const MixedContent: Story = {
  render: () => (
    <ButtonGroup>
      <ButtonGroupItem>
        <Undo />
        Undo Edit
      </ButtonGroupItem>
      <ButtonGroupItem>
        <Redo />
        Redo Edit
      </ButtonGroupItem>
    </ButtonGroup>
  ),
};
