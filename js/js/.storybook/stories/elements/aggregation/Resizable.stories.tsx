// #region Header

// Resizable.stories.tsx

// 2025 Ueli Saluz

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

// #endregion

import type { Meta, StoryObj } from "@storybook/react";
import { Level, LevelProvider, ResizableHandle, ResizablePanel, ResizablePanelGroup, getLevelBgClass } from "../../../../sketchpad/elements";

// #region Resizable
const meta = {
  title: "Elements/Aggregation/Resizable",
  component: ResizablePanelGroup,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  argTypes: {
    direction: { control: false },
  },
} satisfies Meta<typeof ResizablePanelGroup>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    direction: "horizontal",
  },
  render: (args) => (
    <ResizablePanelGroup {...args} className="w-[800px] h-[500px] border">
      <ResizablePanel defaultSize={35} minSize={25}>
        <div className="flex flex-col h-full p-small bg-muted/20">
          <h3 className="text-sm font-semibold mb-4">Type Library</h3>
          <div className="space-y-2 text-sm">
            <div>Capsule J</div>
            <div>Capsule K</div>
            <div>Base</div>
            <div>Tambour A</div>
            <div>Capital</div>
          </div>
        </div>
      </ResizablePanel>
      <ResizableHandle />
      <ResizablePanel defaultSize={65}>
        <ResizablePanelGroup direction="vertical">
          <ResizablePanel defaultSize={70} minSize={30}>
            <div className="flex h-full items-center justify-center p-6 bg-muted/10">
              <span className="font-semibold text-muted-foreground">3D Model View</span>
            </div>
          </ResizablePanel>
          <ResizableHandle />
          <ResizablePanel defaultSize={30} minSize={20}>
            <div className="flex flex-col h-full p-4">
              <h3 className="text-sm font-semibold mb-2">Properties</h3>
              <div className="space-y-1 text-xs text-muted-foreground">
                <div>Volume: 25.0 m³</div>
                <div>Area: 10.0 m²</div>
                <div>Connections: 4</div>
              </div>
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </ResizablePanel>
    </ResizablePanelGroup>
  ),
};

const createLevelRender = (level: Level) => (args: { direction: "horizontal" | "vertical" }) => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <ResizablePanelGroup {...args} className="w-[800px] h-[500px] border">
        <ResizablePanel defaultSize={35} minSize={25}>
          <div className="flex flex-col h-full p-small bg-muted/20">
            <h3 className="text-sm font-semibold mb-4">Type Library</h3>
            <div className="space-y-2 text-sm">
              <div>Capsule J</div>
              <div>Capsule K</div>
              <div>Base</div>
              <div>Tambour A</div>
              <div>Capital</div>
            </div>
          </div>
        </ResizablePanel>
        <ResizableHandle />
        <ResizablePanel defaultSize={65}>
          <ResizablePanelGroup direction="vertical">
            <ResizablePanel defaultSize={70} minSize={30}>
              <div className="flex h-full items-center justify-center p-6 bg-muted/10">
                <span className="font-semibold text-muted-foreground">3D Model View</span>
              </div>
            </ResizablePanel>
            <ResizableHandle />
            <ResizablePanel defaultSize={30} minSize={20}>
              <div className="flex flex-col h-full p-4">
                <h3 className="text-sm font-semibold mb-2">Properties</h3>
                <div className="space-y-1 text-xs text-muted-foreground">
                  <div>Volume: 25.0 m³</div>
                  <div>Area: 10.0 m²</div>
                  <div>Connections: 4</div>
                </div>
              </div>
            </ResizablePanel>
          </ResizablePanelGroup>
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { ...Default.args },
  render: createLevelRender("base"),
};

export const Window: Story = {
  args: { ...Default.args },
  render: createLevelRender("window"),
};

export const Panel: Story = {
  args: { ...Default.args },
  render: createLevelRender("panel"),
};

export const Overlay: Story = {
  args: { ...Default.args },
  render: createLevelRender("overlay"),
};

export const Temporary: Story = {
  args: { ...Default.args },
  render: createLevelRender("temporary"),
};

// #endregion Resizable
