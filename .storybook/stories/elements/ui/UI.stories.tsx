// #region 🧲Header
// .storybook/stories/elements/ui/Ui.stories.tsx
// #endregion 🧲Header

import { App, Mode, Ui } from "@ui/react";
import type { Meta, StoryObj } from "@storybook/react";
import * as React from "react";

const Pane = ({ title }: { title: string }) => (
  <div className="flex h-full items-center justify-center bg-window">
    <h2 className="text-xl font-bold">{title}</h2>
  </div>
);

const meta = {
  title: "elements/react/Ui",
  component: Ui,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof Ui>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [activeAppId, setActiveAppId] = React.useState("editor");
    return (
      <div className="h-[480px] w-full">
        <Ui
          apps={[
            {
              id: "editor",
              label: "Editor",
              children: (
                <App
                  modes={[
                    {
                      id: "design",
                      label: "Design",
                      children: (
                        <Mode
                          windows={[
                            { id: "scene", children: <Pane title="Scene" /> },
                            { id: "tree", children: <Pane title="Tree" /> },
                          ]}
                          activeWindowId="scene"
                          onActiveWindowChange={() => {}}
                        />
                      ),
                    },
                    {
                      id: "review",
                      label: "Review",
                      children: (
                        <Mode
                          windows={[{ id: "preview", children: <Pane title="Preview" /> }]}
                          activeWindowId="preview"
                        />
                      ),
                    },
                  ]}
                  activeModeId="design"
                  onActiveModeChange={() => {}}
                />
              ),
            },
            {
              id: "dashboard",
              label: "Dashboard",
              children: (
                <Mode windows={[{ id: "stats", children: <Pane title="Statistics" /> }]} activeWindowId="stats" />
              ),
            },
          ]}
          activeAppId={activeAppId}
          onActiveAppChange={setActiveAppId}
        />
      </div>
    );
  },
};
