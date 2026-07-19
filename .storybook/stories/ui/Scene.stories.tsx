// #region 🧲Header

// 🥼︎ .storybook/stories/ui/Scene.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🔌Adapters
import { Scene } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
// #endregion 🔌Adapters

// 📍#region 📍Scene
/** @emoji 📦 Minimal R3F mesh — enough for {@link Scene}'s real `<Canvas>` (via `HostThreeCanvas`) to have something to render/orbit around. */
function StoryBox() {
  return (
    <mesh position={[0, 0.5, 0]}>
      <boxGeometry args={[1, 1, 1]} />
      <meshStandardMaterial color="#3b82f6" />
    </mesh>
  );
}

const meta = {
  title: "🖱️ui⚛️react/Scene",
  component: Scene,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Scene>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <div className="h-[480px] w-full">
      <Scene>
        <StoryBox />
      </Scene>
    </div>
  ),
};

export const Orthographic: Story = {
  render: () => (
    <div className="h-[480px] w-full">
      <Scene orthographic>
        <StoryBox />
      </Scene>
    </div>
  ),
};

export const WithoutChrome: Story = {
  name: "No grid / gizmo (bare canvas)",
  render: () => (
    <div className="h-[480px] w-full">
      <Scene showGrid={false} showGizmo={false}>
        <StoryBox />
      </Scene>
    </div>
  ),
};
// #endregion 📍Scene
