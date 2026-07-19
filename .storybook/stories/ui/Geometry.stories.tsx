// #region 🧲Header

// 🥼︎ .storybook/stories/ui/Geometry.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Geometry, ThreeCanvas } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import type { ReactNode } from "react";

// 🧊#region 🧊Geometry
const meta = {
  title: "🖱️ui⚛️react/Geometry",
  component: Geometry,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Geometry>;

export default meta;

type Story = StoryObj<typeof meta>;

const GeometryScene = ({ children }: { readonly children: ReactNode }) => (
  <div className="h-100 w-full">
    <ThreeCanvas camera={{ fov: 45, position: [4, 4, 4], near: 0.1, far: 1000 }} style={{ width: "100%", height: "100%" }}>
      <ambientLight intensity={0.6} />
      <directionalLight position={[5, 8, 4]} intensity={1} />
      {children}
    </ThreeCanvas>
  </div>
);

export const Default: Story = {
  args: { children: null },
  render: () => (
    <GeometryScene>
      <Geometry />
    </GeometryScene>
  ),
};

export const SelectedAndHovered: Story = {
  name: "Selected & Hovered",
  args: { children: null },
  render: () => (
    <GeometryScene>
      <group position={[-1.5, 0, 0]}>
        <Geometry selected />
      </group>
      <group position={[0, 0, 0]}>
        <Geometry hovered />
      </group>
      <group position={[1.5, 0, 0]}>
        <Geometry showEdges={false} color="var(--accent-secondary)" />
      </group>
    </GeometryScene>
  ),
};

// #endregion 🧊Geometry
