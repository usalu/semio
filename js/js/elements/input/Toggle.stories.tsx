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
import { Bold, Box, Circle, Copy, Cylinder, Hexagon, List, Lock, Network, Settings, Trash2 } from "lucide-react";
import { useState } from "react";
import { Toggle } from "./Toggle";

const meta = {
  title: "Elements/Input/Toggle",
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

export const WithValueBasedTooltip: Story = {
  render: () => {
    const [pressed, setPressed] = useState(false);
    return (
      <div className="space-y-2">
        <label className="text-sm font-medium">Lock Layer (Hover to see tooltip change)</label>
        <Toggle pressed={pressed} onPressedChange={setPressed} tooltip="Click to lock layer" tooltipPressed="Click to unlock layer" hotkey="Ctrl+L">
          <Lock />
          {pressed ? "Locked" : "Unlocked"}
        </Toggle>
        <p className="text-xs text-muted-foreground">Current state: {pressed ? "Locked" : "Unlocked"}</p>
      </div>
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
          tooltip="Toggle shape layer visibility"
          dropdownTooltip="Change shape type"
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

export const WithAction: Story = {
  render: () => {
    const [pressed, setPressed] = useState(false);
    return (
      <div className="space-y-2">
        <label className="text-sm font-medium">Enable Layer with Settings</label>
        <Toggle
          type="withAction"
          pressed={pressed}
          onPressedChange={setPressed}
          actionIcon={<Settings className="size-3.5 opacity-50" />}
          onActionClick={() => alert("Settings clicked!")}
          tooltip="Toggle layer lock"
          actionTooltip="Open layer settings"
        >
          <Lock />
          Lock Layer
        </Toggle>
        <div className="text-xs text-muted-foreground space-y-1">
          <p>Click the button to toggle the layer {pressed ? "ON" : "OFF"}</p>
          <p>Click the settings icon to open layer settings</p>
          <p className="font-medium">Status: Layer is {pressed ? "locked" : "unlocked"}</p>
        </div>
      </div>
    );
  },
};

export const WithActionVariants: Story = {
  render: () => {
    const [pressed1, setPressed1] = useState(true);
    const [pressed2, setPressed2] = useState(false);
    const [pressed3, setPressed3] = useState(true);
    return (
      <div className="space-y-4">
        <div className="space-y-2">
          <Toggle
            type="withAction"
            pressed={pressed1}
            onPressedChange={setPressed1}
            actionIcon={<Copy className="size-3.5 opacity-50" />}
            onActionClick={() => alert("Duplicate layer")}
            tooltip="Toggle layer visibility"
            actionTooltip="Duplicate layer"
          >
            <Box />
            Base Layer
          </Toggle>
        </div>
        <div className="space-y-2">
          <Toggle type="withAction" pressed={pressed2} onPressedChange={setPressed2} actionIcon={<Trash2 className="size-3.5 opacity-50" />} onActionClick={() => alert("Delete layer")} tooltip="Toggle object visibility" actionTooltip="Delete object">
            <Cylinder />
            Object
          </Toggle>
        </div>
        <div className="space-y-2">
          <Toggle
            type="withAction"
            label="Annotations"
            pressed={pressed3}
            onPressedChange={setPressed3}
            actionIcon={<Settings className="size-3.5 opacity-50" />}
            onActionClick={() => alert("Configure annotations")}
            tooltip="Annotation visibility"
            actionTooltip="Configure annotation settings"
            hotkey="Ctrl+A"
          >
            Show
          </Toggle>
        </div>
      </div>
    );
  },
};

export const AllTypesWithValueBasedTooltips: Story = {
  render: () => {
    const [standardPressed, setStandardPressed] = useState(false);
    const [cycleValue, setCycleValue] = useState<"model" | "diagram" | "list">("model");
    const [dropdownPressed, setDropdownPressed] = useState(false);
    const [dropdownValue, setDropdownValue] = useState<string>("box");

    return (
      <div className="space-y-6">
        <div className="space-y-2">
          <p className="text-sm font-medium">Standard Toggle (value-based tooltip)</p>
          <Toggle pressed={standardPressed} onPressedChange={setStandardPressed} tooltip="Enable layer" tooltipPressed="Disable layer" hotkey="Ctrl+E">
            <Lock />
            {standardPressed ? "Enabled" : "Disabled"}
          </Toggle>
          <p className="text-xs text-muted-foreground">Tooltip changes based on pressed state</p>
        </div>

        <div className="space-y-2">
          <p className="text-sm font-medium">Cycle Toggle (value-based tooltip)</p>
          <Toggle
            type="cycle"
            value={cycleValue}
            onValueChange={setCycleValue}
            items={[
              { value: "model", label: <Box />, tooltip: "Currently in 3D Model view" },
              { value: "diagram", label: <Network />, tooltip: "Currently in Diagram view" },
              { value: "list", label: <List />, tooltip: "Currently in List view" },
            ]}
          />
          <p className="text-xs text-muted-foreground">Tooltip shows current view mode: {cycleValue}</p>
        </div>

        <div className="space-y-2">
          <p className="text-sm font-medium">Dropdown Toggle (value-based tooltip)</p>
          <Toggle
            type="dropdown"
            pressed={dropdownPressed}
            onPressedChange={setDropdownPressed}
            value={dropdownValue}
            onValueChange={setDropdownValue}
            dropdownTooltip="Change shape"
            items={[
              { value: "box", label: <Box />, tooltip: "Box shape selected" },
              { value: "cylinder", label: <Cylinder />, tooltip: "Cylinder shape selected" },
              { value: "hexagon", label: <Hexagon />, tooltip: "Hexagon shape selected" },
              { value: "circle", label: <Circle />, tooltip: "Circle shape selected" },
            ]}
          />
          <p className="text-xs text-muted-foreground">
            Tooltip shows selected shape: {dropdownValue} ({dropdownPressed ? "enabled" : "disabled"})
          </p>
        </div>
      </div>
    );
  },
};
