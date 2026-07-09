// #region 🧲Header

// 🧪︎ compose/js/.storybook/story/elements/input/Ring.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import type { RingOrbData } from "@semio-tech/ui-react";
import { Ring } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";

// 🔷#region 🧫Ring
const meta = {
  title: "🖱️ui⚛️react/Ring",
  component: Ring,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Ring>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultOrbs: RingOrbData[] = [
  { id: "orb-a", t: 0 },
  { id: "orb-b", t: 0.25 },
  { id: "orb-c", t: 0.5 },
  { id: "orb-d", t: 0.75 },
];

const defaultArgs = {
  id: "ring-default",
  orbs: defaultOrbs,
  radius: 40,
  size: 100,
  onOrbChange: () => {},
  onOrbSelect: () => {},
  onOrbHoverChange: () => {},
  showLabel: true,
  className: "",
};

export const Default: Story = {
  args: defaultArgs,
  render: (args) => {
    const [orbs, setOrbs] = useState(args.orbs);
    const [selectedId, setSelectedId] = useState<string | null>(null);
    const [hoveredId, setHoveredId] = useState<string | null>(null);
    return (
      <Ring
        {...args}
        orbs={orbs.map((orb) => ({
          ...orb,
          selected: orb.id === selectedId,
          hovered: orb.id === hoveredId,
        }))}
        onOrbChange={(orbId, _oldT, newT) => {
          setOrbs((prev) => prev.map((orb) => (orb.id === orbId ? { ...orb, t: newT } : orb)));
        }}
        onOrbSelect={(orbId) => setSelectedId(orbId)}
        onOrbHoverChange={(orbId, hovered) => setHoveredId(hovered ? orbId : null)}
      />
    );
  },
};

// #endregion 🧫Ring
