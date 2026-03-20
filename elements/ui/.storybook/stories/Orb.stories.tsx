// #region 🔖Header

// 🧪︎ semio/js/.storybook/stories/elements/input/Orb.stories.tsx

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

import { Level, LevelProvider, Orb, getLevelBgClass } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";

// #region 🔖Orb
const meta = {
  title: "semio-elements/Orb",
  component: Orb,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Orb>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "orb-default",
  t: 0,
  disabled: false,
  selected: false,
  hovered: false,
  radius: 40,
};

const OrbInRing = ({ id, t, disabled, selected, hovered, radius }: { id: string; t: number; disabled?: boolean; selected?: boolean; hovered?: boolean; radius?: number }) => {
  const r = radius ?? 40;
  const size = r * 2 + 20;
  const center = size / 2;
  return (
    <svg width={size} height={size} viewBox={`${-center} ${-center} ${size} ${size}`}>
      <circle cx={0} cy={0} r={r} className="fill-none stroke-muted-foreground/30 stroke-[2px]" />
      <Orb id={id} t={t} disabled={disabled} selected={selected} hovered={hovered} radius={r} />
    </svg>
  );
};

export const Default: Story = {
  args: defaultArgs,
  render: (args) => <OrbInRing {...args} />,
};

export const Selected: Story = {
  args: { ...defaultArgs, id: "orb-selected", t: 0.25, selected: true },
  render: (args) => <OrbInRing {...args} />,
};

export const Hovered: Story = {
  args: { ...defaultArgs, id: "orb-hovered", t: 0.5, hovered: true },
  render: (args) => <OrbInRing {...args} />,
};

export const Disabled: Story = {
  args: { ...defaultArgs, id: "orb-disabled", t: 0.75, disabled: true },
  render: (args) => <OrbInRing {...args} />,
};

const createLevelRender = (level: Level, id: string, t: number) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <OrbInRing id={id} t={t} radius={40} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { ...defaultArgs, id: "orb-base" },
  render: createLevelRender("base", "orb-base", 0),
};

export const Window: Story = {
  args: { ...defaultArgs, id: "orb-window", t: 0.25 },
  render: createLevelRender("window", "orb-window", 0.25),
};

export const Panel: Story = {
  args: { ...defaultArgs, id: "orb-panel", t: 0.5 },
  render: createLevelRender("panel", "orb-panel", 0.5),
};

export const Overlay: Story = {
  args: { ...defaultArgs, id: "orb-overlay", t: 0.75 },
  render: createLevelRender("overlay", "orb-overlay", 0.75),
};

export const Temporary: Story = {
  args: { ...defaultArgs, id: "orb-temporary", t: 1 },
  render: createLevelRender("temporary", "orb-temporary", 1),
};

// #endregion 🔖Orb
