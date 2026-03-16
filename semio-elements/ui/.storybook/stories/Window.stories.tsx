// #region 🔖Header

// semio-elements/ui/.storybook/stories/elements/window/Window.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import { Window, Canvas, HorizontalWindows, VerticalWindows, LevelProvider } from "@semio-elements/ui";

// #region 🔖Window

const meta = {
  title: "semio-elements/Window",
  component: Window,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Window>;

export default meta;

type Story = StoryObj<typeof meta>;

const WindowContent = ({ title }: { title: string }) => (
  <div className="flex items-center justify-center h-full">
    <h2 className="text-xl font-bold">{title}</h2>
  </div>
);

export const Default: Story = {
  args: {
    id: "default-window",
    children: <WindowContent title="Window Content" />,
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} />
    </div>
  ),
};

export const WithControls: Story = {
  args: {
    id: "controls-window",
    children: <WindowContent title="Window with Controls" />,
    showControls: true,
    onMaximize: () => {},
    onMinimize: () => {},
    onClose: () => {},
    onOpenInNewWindow: () => {},
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} />
    </div>
  ),
};

export const Loading: Story = {
  args: {
    id: "loading-window",
    children: null,
    loading: true,
    skeleton: (
      <div className="flex items-center justify-center h-full animate-pulse">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    ),
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} />
    </div>
  ),
};

export const WithError: Story = {
  args: {
    id: "error-window",
    children: null,
    error: new Error("Something went wrong while loading the content."),
  },
  render: (args) => (
    <div className="h-[400px] w-[600px]">
      <Window {...args} />
    </div>
  ),
};

export const HorizontalLayout: Story = {
  args: { id: "h-layout", children: null },
  render: () => (
    <div className="h-[400px] w-full">
      <Canvas>
        <HorizontalWindows>
          <Window id="left" defaultSize={50}>
            <WindowContent title="Left" />
          </Window>
          <Window id="center" defaultSize={25}>
            <WindowContent title="Center" />
          </Window>
          <Window id="right" defaultSize={25}>
            <WindowContent title="Right" />
          </Window>
        </HorizontalWindows>
      </Canvas>
    </div>
  ),
};

export const VerticalLayout: Story = {
  args: { id: "v-layout", children: null },
  render: () => (
    <div className="h-[400px] w-full">
      <Canvas>
        <VerticalWindows>
          <Window id="top" defaultSize={50}>
            <WindowContent title="Top" />
          </Window>
          <Window id="bottom" defaultSize={50}>
            <WindowContent title="Bottom" />
          </Window>
        </VerticalWindows>
      </Canvas>
    </div>
  ),
};

export const NestedLayout: Story = {
  args: { id: "nested", children: null },
  render: () => (
    <div className="h-[500px] w-full">
      <Canvas>
        <HorizontalWindows>
          <Window id="left" defaultSize={40}>
            <WindowContent title="Left" />
          </Window>
          <Window id="right" defaultSize={60}>
            <VerticalWindows>
              <Window id="top-right" defaultSize={60}>
                <WindowContent title="Top Right" />
              </Window>
              <Window id="bottom-right" defaultSize={40}>
                <WindowContent title="Bottom Right" />
              </Window>
            </VerticalWindows>
          </Window>
        </HorizontalWindows>
      </Canvas>
    </div>
  ),
};

// #endregion 🔖Window
