// #region 🧲Header
// .storybook/stories/elements/ui/Mode.stories.tsx
// #endregion 🧲Header

import { Mode, createEvenWindowLayout } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";
import { expect, userEvent, within } from "storybook/test";

const Pane = ({ label }: { label: string }) => (
  <div className="flex h-full items-center justify-center bg-window">
    <span className="text-lg font-semibold">{label}</span>
  </div>
);

const meta = {
  title: "elements/react/Mode",
  component: Mode,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof Mode>;

export default meta;

type Story = StoryObj<typeof meta>;

export const QuadLayout: Story = {
  render: () => {
    const [activeWindowId, setActiveWindowId] = React.useState<string | null>("overview");
    return (
      <div className="h-[500px] w-full p-single">
        <Mode
          windows={[
            { id: "overview", children: <Pane label="Overview" /> },
            { id: "detail", children: <Pane label="Detail" /> },
            { id: "selection", children: <Pane label="Selection" /> },
            { id: "context", children: <Pane label="Context" /> },
          ]}
          layout={{
            kind: "row",
            children: [
              {
                kind: "column",
                size: 50,
                children: [
                  { kind: "stack", children: [{ kind: "window", id: "overview" }] },
                  { kind: "stack", children: [{ kind: "window", id: "detail" }] },
                ],
              },
              {
                kind: "column",
                size: 50,
                children: [
                  { kind: "stack", children: [{ kind: "window", id: "selection" }] },
                  { kind: "stack", children: [{ kind: "window", id: "context" }] },
                ],
              },
            ],
          }}
          activeWindowId={activeWindowId}
          onActiveWindowChange={setActiveWindowId}
        />
      </div>
    );
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    expect(canvas.getByText("Overview")).toBeTruthy();
    expect(canvas.getByText("Context")).toBeTruthy();
    await userEvent.click(canvas.getByText("Context"));
    expect(canvasElement.querySelector('[data-slot="window"][data-active="true"]')).toBeTruthy();
  },
};

export const EvenSplit: Story = {
  render: () => (
    <div className="h-[400px] w-full p-single">
      <Mode
        windows={[
          { id: "a", children: <Pane label="A" /> },
          { id: "b", children: <Pane label="B" /> },
        ]}
        layout={createEvenWindowLayout(["a", "b"])}
        activeWindowId="a"
      />
    </div>
  ),
};
