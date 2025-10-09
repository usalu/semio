// #region Header

// Toggle.stories.tsx

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
import { Bold, Box, Circle, Cylinder, Hexagon, List, Lock, Network } from "lucide-react";
import { useState } from "react";
import { Toggle } from "./Toggle";

const meta = {
  title: "Elements/Toggle",
  component: Toggle,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Toggle>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [pressed, setPressed] = useState(true);
    return (
      <Toggle pressed={pressed} onPressedChange={setPressed}>
        <Lock />
      </Toggle>
    );
  },
};

export const Basic: Story = {
  render: () => {
    const [pressed, setPressed] = useState(false);
    return (
      <Toggle pressed={pressed} onPressedChange={setPressed}>
        <Bold />
      </Toggle>
    );
  },
};

export const WithText: Story = {
  render: () => {
    const [pressed, setPressed] = useState(true);
    return (
      <Toggle pressed={pressed} onPressedChange={setPressed}>
        <Lock className="h-4 w-4 mr-2" />
        Lock Layer
      </Toggle>
    );
  },
};

export const WithTooltip: Story = {
  render: () => {
    const [pressed, setPressed] = useState(false);
    return (
      <Toggle pressed={pressed} onPressedChange={setPressed} tooltip="Bold" hotkey="Ctrl+B">
        <Bold />
      </Toggle>
    );
  },
};

export const Disabled: Story = {
  args: {
    disabled: true,
    children: (
      <>
        <Bold />
      </>
    ),
  },
};

export const Cycle: Story = {
  render: () => {
    const [value, setValue] = useState<"model" | "diagram" | "list">("model");
    return (
      <div className="space-y-2">
        <label className="text-sm font-medium">View Mode (Click to cycle)</label>
        <Toggle
          type="cycle"
          value={value}
          onValueChange={setValue}
          items={[
            { value: "model", label: <Box />, tooltip: "3D Model View" },
            { value: "diagram", label: <Network />, tooltip: "Diagram View" },
            { value: "list", label: <List />, tooltip: "List View" },
          ]}
        />
        <p className="text-xs text-muted-foreground">Selected: {value}</p>
      </div>
    );
  },
};

export const Dropdown: Story = {
  render: () => {
    const [pressed, setPressed] = useState(false);
    const [value, setValue] = useState<string | undefined>("box");
    return (
      <div className="space-y-2">
        <label className="text-sm font-medium">Enable Shape Layer</label>
        <Toggle
          type="dropdown"
          pressed={pressed}
          onPressedChange={setPressed}
          value={value}
          onValueChange={setValue}
          placeholder="Choose a shape..."
          items={[
            { value: "box", label: <Box />, tooltip: "Box shape" },
            { value: "cylinder", label: <Cylinder />, tooltip: "Cylinder shape" },
            { value: "hexagon", label: <Hexagon />, tooltip: "Hexagon shape" },
            { value: "circle", label: <Circle />, tooltip: "Circle shape" },
          ]}
        />
        <div className="text-xs text-muted-foreground space-y-1">
          <p>
            Click the button to toggle {value || "shape"} {pressed ? "ON" : "OFF"}
          </p>
          <p>Click the chevron to change which shape to toggle</p>
          <p className="font-medium">
            Status: {value || "none"} is {pressed ? "enabled" : "disabled"}
          </p>
        </div>
      </div>
    );
  },
};

export const WithLabel: Story = {
  render: () => {
    const [pressed, setPressed] = useState(true);
    const [value, setValue] = useState<"model" | "diagram" | "list">("diagram");
    return (
      <Toggle
        type="dropdown"
        label="View Mode"
        pressed={pressed}
        onPressedChange={setPressed}
        value={value}
        onValueChange={setValue}
        items={[
          {
            value: "model",
            label: (
              <>
                <Box className="h-4 w-4 mr-2" />
                3D Model
              </>
            ),
            tooltip: "3D Model View",
            hotkey: "Ctrl+1",
          },
          {
            value: "diagram",
            label: (
              <>
                <Network className="h-4 w-4 mr-2" />
                Diagram
              </>
            ),
            tooltip: "Diagram View",
            hotkey: "Ctrl+2",
          },
          {
            value: "list",
            label: (
              <>
                <List className="h-4 w-4 mr-2" />
                List
              </>
            ),
            tooltip: "List View",
            hotkey: "Ctrl+3",
          },
        ]}
      />
    );
  },
};
