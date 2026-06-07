// #region 🧲Header
// .storybook/stories/elements/ui/Engagement.stories.tsx
// #endregion 🧲Header

import { Engagement, Window } from "@ui/react";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import { expect, within } from "storybook/test";

const meta = {
  title: "🖱️ui⚛️react/Engagement",
  component: Engagement,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
} satisfies Meta<typeof Engagement>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Standalone: Story = {
  render: () => (
    <Engagement
      options={[
        { id: "snap", label: "Snap", pressed: true, onPress: () => {} },
        { id: "grid", label: "Grid", onPress: () => {} },
      ]}
      input={{ placeholder: "Ask or command…", onSubmit: () => {} }}
      status={[
        { id: "ready", content: "Ready" },
        { id: "selection", content: "3 selected" },
      ]}
    />
  ),
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    expect(canvas.getByPlaceholderText("Ask or command…")).toBeTruthy();
    expect(canvas.getByText("Ready")).toBeTruthy();
  },
};

export const WithControl: Story = {
  render: () => (
    <Engagement
      sessionActive
      input={{ placeholder: "Height", value: "3" }}
      status={[{ id: "engagement-step", content: "Step: Height" }]}
      control={{
        kind: "stepper",
        id: "height",
        label: "Height",
        value: 3,
        min: 0,
        step: 0.1,
        unit: "m",
        onChange: () => {},
      }}
    />
  ),
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    expect(canvas.getByText("Height")).toBeTruthy();
    expect(canvasElement.querySelector('[data-slot="engagement-control"][data-control-kind="stepper"]')).toBeTruthy();
  },
};

export const InWindow: Story = {
  render: () => (
    <div className="relative h-[320px] w-[480px]">
      <Window
        id="engagement-window"
        active
        engagement={{
          options: [{ id: "tool-a", label: "Tool A", onPress: () => {} }],
          input: { placeholder: "Type a command" },
          status: [{ id: "status", content: "Idle" }],
        }}
      >
        <div className="flex h-full items-center justify-center bg-panel">Window body</div>
      </Window>
    </div>
  ),
};
