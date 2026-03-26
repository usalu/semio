// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Vec3.stories.tsx
// Specs: One component per stories file with default and constrained-variant stories.
// Summary: Showcases Vec3 with full/partial controlled and display/select constrained modes.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import { Vec3, type Vec3Value } from "@semio/ui";
import { useState } from "react";

const meta = {
  title: "semio/Vec3",
  component: Vec3,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Vec3>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultValue: Vec3Value = { u: 0.2, v: -0.4, w: 0.7 };

export const FullControlled: Story = {
  args: {
    id: "vec3-full-controlled",
    minU: -1,
    maxU: 1,
    minV: -1,
    maxV: 1,
    minW: -1,
    maxW: 1,
    step: 0.01,
  },
  render: (args) => {
    const [vec, setVec] = useState<Vec3Value>(defaultValue);
    return (
      <div className="flex min-w-96 flex-col gap-2">
        <Vec3 {...args} vec={vec} onVecChange={setVec} />
        <div className="text-xs text-muted-foreground">
          u: {vec.u.toFixed(2)} | v: {vec.v.toFixed(2)} | w: {vec.w.toFixed(2)}
        </div>
      </div>
    );
  },
};

export const FullUncontrolled: Story = {
  args: {
    id: "vec3-full-uncontrolled",
    defaultVec: defaultValue,
    minU: -10,
    maxU: 10,
    minV: -10,
    maxV: 10,
    minW: -10,
    maxW: 10,
    step: 0.1,
  },
};

export const PartialAxisControl: Story = {
  args: {
    id: "vec3-partial-axis-control",
    defaultVec: { u: 0, v: 0, w: 0 },
    u: 0.5,
    minU: -1,
    maxU: 1,
    minV: -2,
    maxV: 2,
    minW: -2,
    maxW: 2,
  },
  render: (args) => {
    const [u, setU] = useState(0.5);
    return (
      <div className="flex min-w-96 flex-col gap-2">
        <Vec3 {...args} u={u} onUChange={setU} />
        <div className="text-xs text-muted-foreground">u is controlled externally: {u.toFixed(2)}</div>
      </div>
    );
  },
};

export const PartialSelectionAndDisplay: Story = {
  args: {
    id: "vec3-partial-selection-display",
    defaultVec: { u: 0.5, v: -0.25, w: 0.9 },
    uSelectionEnabled: true,
    vSelectionEnabled: false,
    wSelectionEnabled: false,
    uDisplayEnabled: true,
    vDisplayEnabled: true,
    wDisplayEnabled: false,
  },
};
