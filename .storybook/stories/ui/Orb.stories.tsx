// #region 🧲Header

// 🧪︎ semio/js/.storybook/stories/elements/input/Orb.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Orb } from "@ui/react";
import type { Meta, StoryObj } from "@storybook/react";

// 🔷#region 🎄Orb
const meta = {
  title: "🖱️ui⚛️react/Orb",
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

// #endregion 🎄Orb
