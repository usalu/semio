// #region Header

// Stepper.stories.tsx

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
import Stepper from "./Stepper";

const meta = {
  title: "Elements/Stepper",
  component: Stepper,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Stepper>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [value, setValue] = useState(12);
    return (
      <div className="w-96 space-y-2">
        <label className="text-sm font-medium">Capsule Count</label>
        <Stepper value={value} onChange={setValue} min={1} max={50} />
        <p className="text-xs text-muted-foreground">Number of capsule instances in the current design cluster.</p>
      </div>
    );
  },
};

export const Variants: Story = {
  render: () => (
    <div className="flex flex-col gap-6 w-96">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground">Default</p>
        {(() => {
          const [value, setValue] = useState(0);
          return <Stepper value={value} onChange={setValue} />;
        })()}
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground">With Label</p>
        {(() => {
          const [value, setValue] = useState(5);
          return <Stepper label="Piece Count" value={value} onChange={setValue} />;
        })()}
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground">With Min/Max</p>
        {(() => {
          const [value, setValue] = useState(5);
          return <Stepper label="Connections" value={value} onChange={setValue} min={0} max={10} />;
        })()}
      </div>
    </div>
  ),
};

export const Basic: Story = {
  render: () => {
    const [value, setValue] = useState(0);
    return <Stepper value={value} onChange={setValue} className="w-96" />;
  },
};

export const WithLabel: Story = {
  render: () => {
    const [value, setValue] = useState(5);
    return <Stepper label="Quantity" value={value} onChange={setValue} className="w-96" />;
  },
};

export const WithMinMax: Story = {
  render: () => {
    const [value, setValue] = useState(5);
    return <Stepper label="Count" value={value} onChange={setValue} min={0} max={10} className="w-96" />;
  },
};

export const WithStep: Story = {
  render: () => {
    const [value, setValue] = useState(0);
    return <Stepper label="Value" value={value} onChange={setValue} step={5} className="w-96" />;
  },
};

export const DecimalStep: Story = {
  render: () => {
    const [value, setValue] = useState(0);
    return <Stepper label="Price" value={value} onChange={setValue} step={0.1} min={0} max={100} className="w-96" />;
  },
};

export const NegativeRange: Story = {
  render: () => {
    const [value, setValue] = useState(0);
    return <Stepper label="Offset" value={value} onChange={setValue} min={-10} max={10} className="w-96" />;
  },
};

export const LargeStep: Story = {
  render: () => {
    const [value, setValue] = useState(0);
    return <Stepper label="Width" value={value} onChange={setValue} step={100} min={0} max={1000} className="w-96" />;
  },
};

export const Multiple: Story = {
  render: () => {
    const [width, setWidth] = useState(100);
    const [height, setHeight] = useState(100);
    const [padding, setPadding] = useState(10);

    return (
      <div className="w-96 space-y-4">
        <Stepper label="Width" value={width} onChange={setWidth} min={0} max={500} />
        <Stepper label="Height" value={height} onChange={setHeight} min={0} max={500} />
        <Stepper label="Padding" value={padding} onChange={setPadding} min={0} max={50} step={5} />
      </div>
    );
  },
};

export const WithCallbacks: Story = {
  render: () => {
    const [value, setValue] = useState(0);
    const [isEditing, setIsEditing] = useState(false);

    return (
      <div className="w-96 space-y-2">
        <Stepper label="Value" value={value} onChange={setValue} onPointerDown={() => setIsEditing(true)} onPointerUp={() => setIsEditing(false)} onPointerCancel={() => setIsEditing(false)} />
        <div className="text-sm text-muted-foreground">Editing: {isEditing ? "Yes" : "No"}</div>
      </div>
    );
  },
};
