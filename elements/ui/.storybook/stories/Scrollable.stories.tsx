// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/aggregation/Scrollable.stories.tsx

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

import { Level, LevelProvider, Scrollable, getLevelBgClass } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";

// #region 🔖Scrollable
const meta = {
  title: "elements/Scrollable",
  component: Scrollable,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Scrollable>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <Scrollable className="h-96 w-[600px] border">
      <div className="p-4 space-y-4">
        <div>
          <h4 className="text-sm font-semibold mb-2">Nakagin Capsule Tower</h4>
          <p className="text-sm text-muted-foreground mb-4">
            The Nakagin Capsule Tower Building is a mixed-use residential and office tower in Tokyo, Japan designed by architect Kisho Kurokawa. Completed in 1972, the building is a rare remaining example of Japanese Metabolism architecture.
          </p>
        </div>
        <div>
          <h4 className="text-sm font-semibold mb-2">Design Specifications</h4>
          <div className="text-sm space-y-1">
            <div>Total Capsules: 140</div>
            <div>Capsule Dimensions: 2.5m × 4.0m × 2.5m</div>
            <div>Building Height: 52.4m</div>
            <div>Total Floors: 13</div>
            <div>Construction: Prefabricated steel frame</div>
          </div>
        </div>
        <div>
          <h4 className="text-sm font-semibold mb-2">Structural System</h4>
          <p className="text-sm text-muted-foreground">
            Each capsule was designed to be replaceable and fully self-contained with built-in bathroom and storage. The capsules were attached to two interconnected concrete towers with high-tension bolts, allowing for individual replacement.
          </p>
        </div>
        <div>
          <h4 className="text-sm font-semibold mb-2">Historical Context</h4>
          <p className="text-sm text-muted-foreground">
            The Metabolism movement emerged in 1960s Japan, proposing buildings that could adapt to changing needs through modular, replaceable components. The Nakagin Tower represents one of the few built realizations of these principles.
          </p>
        </div>
      </div>
    </Scrollable>
  ),
};

const ScrollableContent = () => (
  <div className="p-4 space-y-4">
    <div>
      <h4 className="text-sm font-semibold mb-2">Nakagin Capsule Tower</h4>
      <p className="text-sm text-muted-foreground mb-4">
        The Nakagin Capsule Tower Building is a mixed-use residential and office tower in Tokyo, Japan designed by architect Kisho Kurokawa. Completed in 1972, the building is a rare remaining example of Japanese Metabolism architecture.
      </p>
    </div>
    <div>
      <h4 className="text-sm font-semibold mb-2">Design Specifications</h4>
      <div className="text-sm space-y-1">
        <div>Total Capsules: 140</div>
        <div>Capsule Dimensions: 2.5m × 4.0m × 2.5m</div>
        <div>Building Height: 52.4m</div>
        <div>Total Floors: 13</div>
        <div>Construction: Prefabricated steel frame</div>
      </div>
    </div>
    <div>
      <h4 className="text-sm font-semibold mb-2">Structural System</h4>
      <p className="text-sm text-muted-foreground">
        Each capsule was designed to be replaceable and fully self-contained with built-in bathroom and storage. The capsules were attached to two interconnected concrete towers with high-tension bolts, allowing for individual replacement.
      </p>
    </div>
    <div>
      <h4 className="text-sm font-semibold mb-2">Historical Context</h4>
      <p className="text-sm text-muted-foreground">
        The Metabolism movement emerged in 1960s Japan, proposing buildings that could adapt to changing needs through modular, replaceable components. The Nakagin Tower represents one of the few built realizations of these principles.
      </p>
    </div>
  </div>
);

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <Scrollable className="h-96 w-[600px] border">
        <ScrollableContent />
      </Scrollable>
    </div>
  </LevelProvider>
);

export const Base: Story = {
  render: createLevelRender("base"),
};

export const Window: Story = {
  render: createLevelRender("window"),
};

export const Panel: Story = {
  render: createLevelRender("panel"),
};

export const Overlay: Story = {
  render: createLevelRender("overlay"),
};

export const Temporary: Story = {
  render: createLevelRender("temporary"),
};

// #endregion 🔖Scrollable
