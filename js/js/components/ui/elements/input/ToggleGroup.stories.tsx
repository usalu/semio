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
import { AlignCenter, AlignLeft, AlignRight, Bold, Box, Circle, Cylinder, Hexagon, Italic, List, Lock, Network, Underline } from "lucide-react";
import { useState } from "react";
import { Toggle } from "./Toggle";
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

export const Default: Story = {
  render: () => {
    const [value, setValue] = useState("model");
    return (
      <div className="space-y-2">
        <label className="text-sm font-medium">View Mode</label>
        <ToggleGroup type="single" value={value} onValueChange={setValue}>
          <ToggleGroupItem value="model">
            <Box className="h-4 w-4 mr-2" />
            3D Model
          </ToggleGroupItem>
          <ToggleGroupItem value="diagram">
            <Network className="h-4 w-4 mr-2" />
            Diagram
          </ToggleGroupItem>
          <ToggleGroupItem value="list">
            <List className="h-4 w-4 mr-2" />
            List
          </ToggleGroupItem>
        </ToggleGroup>
        <p className="text-xs text-muted-foreground">Selected: {value}</p>
      </div>
    );
  },
};

export const Variants: Story = {
  render: () => (
    <div className="flex flex-col gap-8">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Single</p>
        <ToggleGroup type="single">
          <ToggleGroupItem value="model">
            <Box />
          </ToggleGroupItem>
          <ToggleGroupItem value="diagram">
            <Network />
          </ToggleGroupItem>
          <ToggleGroupItem value="details">
            <List />
          </ToggleGroupItem>
        </ToggleGroup>
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Multiple</p>
        <ToggleGroup type="multiple">
          <ToggleGroupItem value="types">
            <Cylinder />
          </ToggleGroupItem>
          <ToggleGroupItem value="designs">
            <Hexagon />
          </ToggleGroupItem>
          <ToggleGroupItem value="pieces">
            <Circle />
          </ToggleGroupItem>
        </ToggleGroup>
      </div>
    </div>
  ),
};

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

export const AllTypes: Story = {
  render: () => {
    const [alignment, setAlignment] = useState<string>("left");
    const [textFormat, setTextFormat] = useState<string[]>([]);
    const [shape, setShape] = useState<string>("box");

    return (
      <div className="space-y-6">
        <div className="space-y-2">
          <p className="text-sm font-medium">Single Selection (Alignment)</p>
          <ToggleGroup type="single" value={alignment} onValueChange={(val) => val && setAlignment(val)}>
            <ToggleGroupItem value="left" tooltip="Align Left">
              <AlignLeft />
            </ToggleGroupItem>
            <ToggleGroupItem value="center" tooltip="Align Center">
              <AlignCenter />
            </ToggleGroupItem>
            <ToggleGroupItem value="right" tooltip="Align Right">
              <AlignRight />
            </ToggleGroupItem>
          </ToggleGroup>
          <p className="text-xs text-muted-foreground">Selected: {alignment}</p>
        </div>

        <div className="space-y-2">
          <p className="text-sm font-medium">Multiple Selection (Text Format)</p>
          <ToggleGroup type="multiple" value={textFormat} onValueChange={setTextFormat}>
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
          <p className="text-xs text-muted-foreground">Selected: {textFormat.join(", ") || "none"}</p>
        </div>

        <div className="space-y-2">
          <p className="text-sm font-medium">With Text Labels</p>
          <ToggleGroup type="single" value={shape} onValueChange={(val) => val && setShape(val)}>
            <ToggleGroupItem value="box">
              <Box className="h-4 w-4 mr-2" />
              Box
            </ToggleGroupItem>
            <ToggleGroupItem value="cylinder">
              <Cylinder className="h-4 w-4 mr-2" />
              Cylinder
            </ToggleGroupItem>
            <ToggleGroupItem value="hexagon">
              <Hexagon className="h-4 w-4 mr-2" />
              Hexagon
            </ToggleGroupItem>
            <ToggleGroupItem value="circle">
              <Circle className="h-4 w-4 mr-2" />
              Circle
            </ToggleGroupItem>
          </ToggleGroup>
          <p className="text-xs text-muted-foreground">Selected: {shape}</p>
        </div>

        <div className="space-y-2">
          <p className="text-sm font-medium">Disabled State</p>
          <ToggleGroup type="single" disabled defaultValue="center">
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
        </div>
      </div>
    );
  },
};

export const MixedToggleTypes: Story = {
  render: () => {
    const [locked, setLocked] = useState(false);
    const [viewMode, setViewMode] = useState<"model" | "diagram" | "list">("model");
    const [shapePressed, setShapePressed] = useState(false);
    const [shape, setShape] = useState<string>("box");

    return (
      <div className="space-y-4">
        <p className="text-sm font-medium">Mixed Toggle Types in a Visual Group</p>
        <div className="flex w-fit items-center border overflow-hidden">
          {/* Standard Toggle */}
          <Toggle pressed={locked} onPressedChange={setLocked} tooltip="Lock Layer" className="border-0 border-l first:border-l-0">
            <Lock />
          </Toggle>

          {/* Cycle Toggle */}
          <Toggle
            type="cycle"
            value={viewMode}
            onValueChange={setViewMode}
            items={[
              { value: "model", label: <Box />, tooltip: "3D Model View" },
              { value: "diagram", label: <Network />, tooltip: "Diagram View" },
              { value: "list", label: <List />, tooltip: "List View" },
            ]}
            className="border-0 border-l"
          />

          {/* Dropdown Toggle */}
          <Toggle
            type="dropdown"
            pressed={shapePressed}
            onPressedChange={setShapePressed}
            value={shape}
            onValueChange={setShape}
            items={[
              { value: "box", label: <Box />, tooltip: "Box shape" },
              { value: "cylinder", label: <Cylinder />, tooltip: "Cylinder shape" },
              { value: "hexagon", label: <Hexagon />, tooltip: "Hexagon shape" },
              { value: "circle", label: <Circle />, tooltip: "Circle shape" },
            ]}
            className="border-0 border-l"
          />
        </div>
        <div className="text-xs text-muted-foreground space-y-1">
          <p>Locked: {locked ? "Yes" : "No"}</p>
          <p>View Mode: {viewMode}</p>
          <p>Shape: {shape} ({shapePressed ? "enabled" : "disabled"})</p>
        </div>
      </div>
    );
  },
};
