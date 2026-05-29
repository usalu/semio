// #region 🧲Header
// .storybook/stories/elements/ui/App.stories.tsx
// #endregion 🧲Header

import { App, Mode } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

const meta = {
  title: "elements/react/App",
  component: App,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof App>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [activeModeId, setActiveModeId] = React.useState("design");
    return (
      <div className="h-[400px] w-full">
        <App
          modes={[
            {
              id: "design",
              label: "Design",
              children: (
                <Mode
                  windows={[
                    { id: "left", children: <div className="flex h-full items-center justify-center">Design Left</div> },
                    { id: "right", children: <div className="flex h-full items-center justify-center">Design Right</div> },
                  ]}
                  activeWindowId="left"
                />
              ),
            },
            {
              id: "review",
              label: "Review",
              children: (
                <Mode windows={[{ id: "preview", children: <div className="flex h-full items-center justify-center">Review Preview</div> }]} activeWindowId="preview" />
              ),
            },
          ]}
          activeModeId={activeModeId}
          onActiveModeChange={setActiveModeId}
        />
      </div>
    );
  },
};
