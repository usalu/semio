// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/input/Stepper.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion 🔖Header

import { Level, LevelProvider, Stepper, getLevelBgClass } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";

// #region 🔖Stepper
const meta = {
  title: "elements/Stepper",
  component: Stepper,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Stepper>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "stepper-default",
  value: 12,
  onChange: () => {},
  min: 1,
  max: 50,
  step: 1,
  onPointerDown: () => {},
  onPointerUp: () => {},
  onPointerCancel: () => {},
  interactionId: "stepper-interaction",
};

export const Default: Story = {
  args: defaultArgs,
  render: (args) => {
    const [value, setValue] = useState(args.value);
    return <Stepper {...args} value={value} onChange={setValue} />;
  },
};

const StepperDemo = ({ id }: { id: string }) => {
  const [value, setValue] = useState(12);
  return <Stepper {...defaultArgs} id={id} value={value} onChange={setValue} />;
};

const createLevelRender = (level: Level, id: string) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <StepperDemo id={id} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { ...defaultArgs, id: "stepper-base" },
  render: createLevelRender("base", "stepper-base"),
};

export const Window: Story = {
  args: { ...defaultArgs, id: "stepper-window" },
  render: createLevelRender("window", "stepper-window"),
};

export const Panel: Story = {
  args: { ...defaultArgs, id: "stepper-panel" },
  render: createLevelRender("panel", "stepper-panel"),
};

export const Overlay: Story = {
  args: { ...defaultArgs, id: "stepper-overlay" },
  render: createLevelRender("overlay", "stepper-overlay"),
};

export const Temporary: Story = {
  args: { ...defaultArgs, id: "stepper-temporary" },
  render: createLevelRender("temporary", "stepper-temporary"),
};

// #endregion 🔖Stepper
