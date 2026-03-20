// #region 🔖Header
// 💻 semio/ui/.storybook/stories/Vec.stories.tsx
// Specs: One component per stories file with default and domain-variant stories.
// Summary: Showcases the Vec 2D vector input with draggable handle, axes, and origin.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import { Vec, type VecValue } from "@semio/ui";
import { useState } from "react";

// #region 🔖Vec
const meta = {
  title: "semio/Vec",
  component: Vec,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Vec>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "vec-default",
  vec: { u: 0.3, v: 0.5 } as VecValue,
  minU: -1,
  maxU: 1,
  minV: -1,
  maxV: 1,
  showAxes: true,
  showOrigin: true,
  size: 120,
  onVecChange: () => {},
  className: "",
};

export const Default: Story = {
  args: defaultArgs,
  render: (args) => {
    const [vec, setVec] = useState(args.vec);
    return (
      <div className="flex flex-col items-center gap-2">
        <Vec {...args} vec={vec} onVecChange={setVec} />
        <span className="text-xs text-muted-foreground">
          u: {vec.u.toFixed(2)}, v: {vec.v.toFixed(2)}
        </span>
      </div>
    );
  },
};

export const NoAxes: Story = {
  args: { ...defaultArgs, id: "vec-no-axes", showAxes: false },
  render: (args) => {
    const [vec, setVec] = useState(args.vec);
    return <Vec {...args} vec={vec} onVecChange={setVec} />;
  },
};

export const NoOrigin: Story = {
  args: { ...defaultArgs, id: "vec-no-origin", showOrigin: false },
  render: (args) => {
    const [vec, setVec] = useState(args.vec);
    return <Vec {...args} vec={vec} onVecChange={setVec} />;
  },
};

export const PositiveDomain: Story = {
  args: { ...defaultArgs, id: "vec-positive-domain", minU: 0, maxU: 10, minV: 0, maxV: 10, vec: { u: 5, v: 5 } },
  render: (args) => {
    const [vec, setVec] = useState(args.vec);
    return (
      <div className="flex flex-col items-center gap-2">
        <Vec {...args} vec={vec} onVecChange={setVec} />
        <span className="text-xs text-muted-foreground">
          u: {vec.u.toFixed(1)}, v: {vec.v.toFixed(1)}
        </span>
      </div>
    );
  },
};

export const Large: Story = {
  args: { ...defaultArgs, id: "vec-large", size: 200 },
  render: (args) => {
    const [vec, setVec] = useState(args.vec);
    return <Vec {...args} vec={vec} onVecChange={setVec} />;
  },
};

// #endregion 🔖Vec
