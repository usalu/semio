// #region 🧲Header
// 💻 semio/ui/.storybook/story/Vec.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. Default story is uncontrolled.
// Summary: Vec stories: Default, NoAxes, NoOrigin, PositiveDomain, Large.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { Vec, type VecValue } from "@semio/ui";
import type { Meta, StoryObj } from "@storybook/react";
// #region 🏪Vec

const meta = {
  title: "🏘️semio⚛️react/Vec",
  component: Vec,
  parameters: { layout: "centered" },
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
  className: "",
};

export const Default: Story = {
  args: defaultArgs,
};

export const NoAxes: Story = {
  args: { ...defaultArgs, id: "vec-no-axes", showAxes: false },
};

export const NoOrigin: Story = {
  args: { ...defaultArgs, id: "vec-no-origin", showOrigin: false },
};

export const PositiveDomain: Story = {
  args: (() => {
    const { vec: _ignored, ...withoutVec } = defaultArgs;
    return {
      ...withoutVec,
      id: "vec-positive-domain",
      minU: 0,
      maxU: 10,
      minV: 0,
      maxV: 10,
      vec: { u: 5, v: 5 } as VecValue,
    };
  })(),
};

export const Large: Story = {
  args: { ...defaultArgs, id: "vec-large", size: 200 },
};

// #endregion 🏪Vec
