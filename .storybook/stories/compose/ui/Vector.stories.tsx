// #region 🧲️Header
// 💻️ compose/ui/.storybook/story/Vector.stories.tsx
// Specs: One component per stories file. First story is Default with max features and minimal setup. Default story is uncontrolled.
// Summary: Vector stories: Default, PartialSelectionAndDisplay.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { Vector, type VectorValue } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
// #region 🔤️Vector

const meta = {
  title: "🏘️compose⚛️react/Vector",
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
    defaultVector: defaultValue,
    minX: -1,
    maxX: 1,
    minY: -1,
    maxY: 1,
    minZ: -1,
    maxZ: 1,
    step: 0.01,
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

// #endregion 🔤️Vector
