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
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "./Resizable";

const meta = {
  title: "Elements/Resizable",
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
  render: () => (
    <ResizablePanelGroup direction="horizontal" className="w-[800px] h-[500px] border">
      <ResizablePanel defaultSize={35} minSize={25}>
        <div className="flex flex-col h-full p-4 bg-muted/20">
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
      <ResizableHandle withHandle />
      <ResizablePanel defaultSize={65}>
        <ResizablePanelGroup direction="vertical">
          <ResizablePanel defaultSize={70} minSize={30}>
            <div className="flex h-full items-center justify-center p-6 bg-muted/10">
              <span className="font-semibold text-muted-foreground">3D Model View</span>
            </div>
          </ResizablePanel>
          <ResizableHandle withHandle />
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

export const Variants: Story = {
  args: {
    direction: "horizontal",
  },
  render: () => (
    <div className="flex flex-col gap-8">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Horizontal</p>
        <ResizablePanelGroup direction="horizontal" className="w-[600px] h-[200px] border">
          <ResizablePanel defaultSize={50}>
            <div className="flex h-full items-center justify-center p-6">
              <span className="font-semibold">Model</span>
            </div>
          </ResizablePanel>
          <ResizableHandle />
          <ResizablePanel defaultSize={50}>
            <div className="flex h-full items-center justify-center p-6">
              <span className="font-semibold">Diagram</span>
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Vertical</p>
        <ResizablePanelGroup direction="vertical" className="w-[600px] h-[400px] border">
          <ResizablePanel defaultSize={50}>
            <div className="flex h-full items-center justify-center p-6">
              <span className="font-semibold">Editor</span>
            </div>
          </ResizablePanel>
          <ResizableHandle />
          <ResizablePanel defaultSize={50}>
            <div className="flex h-full items-center justify-center p-6">
              <span className="font-semibold">Console</span>
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </div>
    </div>
  ),
};

export const Horizontal: Story = {
  args: {
    direction: "horizontal",
  },
  render: () => (
    <ResizablePanelGroup direction="horizontal" className="w-[600px] h-[200px] border">
      <ResizablePanel defaultSize={50}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Left Panel</span>
        </div>
      </ResizablePanel>
      <ResizableHandle />
      <ResizablePanel defaultSize={50}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Right Panel</span>
        </div>
      </ResizablePanel>
    </ResizablePanelGroup>
  ),
};

export const WithHandle: Story = {
  args: {
    direction: "horizontal",
  },
  render: () => (
    <ResizablePanelGroup direction="horizontal" className="w-[600px] h-[200px] border">
      <ResizablePanel defaultSize={50}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Left Panel</span>
        </div>
      </ResizablePanel>
      <ResizableHandle withHandle />
      <ResizablePanel defaultSize={50}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Right Panel</span>
        </div>
      </ResizablePanel>
    </ResizablePanelGroup>
  ),
};

export const ThreePanels: Story = {
  args: {
    direction: "horizontal",
  },
  render: () => (
    <ResizablePanelGroup direction="horizontal" className="w-[600px] h-[200px] border">
      <ResizablePanel defaultSize={25}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Sidebar</span>
        </div>
      </ResizablePanel>
      <ResizableHandle withHandle />
      <ResizablePanel defaultSize={50}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Main Content</span>
        </div>
      </ResizablePanel>
      <ResizableHandle withHandle />
      <ResizablePanel defaultSize={25}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Inspector</span>
        </div>
      </ResizablePanel>
    </ResizablePanelGroup>
  ),
};

export const Vertical: Story = {
  args: {
    direction: "vertical",
  },
  render: () => (
    <ResizablePanelGroup direction="vertical" className="w-[400px] h-[400px] border">
      <ResizablePanel defaultSize={50}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Top Panel</span>
        </div>
      </ResizablePanel>
      <ResizableHandle withHandle />
      <ResizablePanel defaultSize={50}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Bottom Panel</span>
        </div>
      </ResizablePanel>
    </ResizablePanelGroup>
  ),
};

export const Nested: Story = {
  args: {
    direction: "horizontal",
  },
  render: () => (
    <ResizablePanelGroup direction="horizontal" className="w-[700px] h-[400px] border">
      <ResizablePanel defaultSize={30}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Sidebar</span>
        </div>
      </ResizablePanel>
      <ResizableHandle withHandle />
      <ResizablePanel defaultSize={70}>
        <ResizablePanelGroup direction="vertical">
          <ResizablePanel defaultSize={60}>
            <div className="flex h-full items-center justify-center p-6">
              <span className="font-semibold">Main Content</span>
            </div>
          </ResizablePanel>
          <ResizableHandle withHandle />
          <ResizablePanel defaultSize={40}>
            <div className="flex h-full items-center justify-center p-6">
              <span className="font-semibold">Footer/Console</span>
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </ResizablePanel>
    </ResizablePanelGroup>
  ),
};

export const WithMinMax: Story = {
  args: {
    direction: "horizontal",
  },
  render: () => (
    <ResizablePanelGroup direction="horizontal" className="w-[600px] h-[200px] border">
      <ResizablePanel defaultSize={30} minSize={20} maxSize={40}>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold text-center">Constrained Panel (20-40%)</span>
        </div>
      </ResizablePanel>
      <ResizableHandle withHandle />
      <ResizablePanel>
        <div className="flex h-full items-center justify-center p-6">
          <span className="font-semibold">Flexible Panel</span>
        </div>
      </ResizablePanel>
    </ResizablePanelGroup>
  ),
};
