// #region 🧲Header

// 🥼︎ .storybook/stories/ui/UnifiedGumball.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🔌Adapters
import { Scene, UnifiedGumball, type GumballConfig } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import type * as THREE from "three";
// #endregion 🔌Adapters

// 🎛#region 🔖UnifiedGumball
/** @emoji 📦 `UnifiedGumball` needs a live `THREE.Object3D` target — this mounts a box, captures its ref
 * once R3F assigns it, and only then renders the gumball attached to it. */
function GumballTargetDemo({ config }: { readonly config?: GumballConfig }) {
  const [target, setTarget] = useState<THREE.Object3D | null>(null);
  return (
    <Scene showGrid>
      <mesh
        ref={(mesh) => {
          if (mesh && mesh !== target) setTarget(mesh);
        }}
        position={[0, 0.5, 0]}
      >
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial color="#3b82f6" />
      </mesh>
      {target ? <UnifiedGumball target={target} config={config} /> : null}
    </Scene>
  );
}

const meta = {
  title: "🖱️ui⚛️react/UnifiedGumball",
  component: UnifiedGumball,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof UnifiedGumball>;

export default meta;

type Story = StoryObj<typeof meta>;

export const MoveRotateScale: Story = {
  name: "Full gumball (move / rotate / scale)",
  render: () => (
    <div className="h-[480px] w-full">
      <GumballTargetDemo />
    </div>
  ),
};

export const MoveOnly: Story = {
  render: () => (
    <div className="h-[480px] w-full">
      <GumballTargetDemo config={{ moveAxes: true, movePlanes: true, rotate: false, scaleAxes: false, scalePlanes: false, scaleUniform: false }} />
    </div>
  ),
};
// #endregion 🔖UnifiedGumball
