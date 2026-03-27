// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Vector.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. Fully controlled with all three axes.
// Summary: Vector stories: Default, Uncontrolled, PartialAxisControl, PartialSelectionAndDisplay.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import { Vector, type VectorValue } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";

// #region 🔖Vector

const meta = {
  title: "semio/Vector",
  component: Vector,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
} satisfies Meta<typeof Vector>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultValue: VectorValue = { x: 0.2, y: -0.4, z: 0.7 };

export const Default: Story = {
  args: {
    id: "vector-default",
    minX: -1,
    maxX: 1,
    minY: -1,
    maxY: 1,
    minZ: -1,
    maxZ: 1,
    step: 0.01,
  },
  render: (args) => {
    const [vector, setVector] = useState<VectorValue>(defaultValue);
    return (
      <div className="flex min-w-96 flex-col gap-2">
        <Vector {...args} vector={vector} onVectorChange={setVector} />
        <div className="text-xs text-muted-foreground">
          x: {vector.x.toFixed(2)} | y: {vector.y.toFixed(2)} | z: {vector.z.toFixed(2)}
        </div>
      </div>
    );
  },
};

export const Uncontrolled: Story = {
  args: {
    id: "vector-uncontrolled",
    defaultVector: defaultValue,
    minX: -10,
    maxX: 10,
    minY: -10,
    maxY: 10,
    minZ: -10,
    maxZ: 10,
    step: 0.1,
  },
};

export const PartialAxisControl: Story = {
  args: {
    id: "vector-partial-axis-control",
    defaultVector: { x: 0, y: 0, z: 0 },
    x: 0.5,
    minX: -1,
    maxX: 1,
    minY: -2,
    maxY: 2,
    minZ: -2,
    maxZ: 2,
  },
  render: (args) => {
    const [x, setX] = useState(0.5);
    return (
      <div className="flex min-w-96 flex-col gap-2">
        <Vector {...args} x={x} onXChange={setX} />
        <div className="text-xs text-muted-foreground">x is controlled externally: {x.toFixed(2)}</div>
      </div>
    );
  },
};

export const PartialSelectionAndDisplay: Story = {
  args: {
    id: "vector-partial-selection-display",
    defaultVector: { x: 0.5, y: -0.25, z: 0.9 },
    xSelectionEnabled: true,
    ySelectionEnabled: false,
    zSelectionEnabled: false,
    xDisplayEnabled: true,
    yDisplayEnabled: true,
    zDisplayEnabled: false,
  },
};

// #endregion 🔖Vector
