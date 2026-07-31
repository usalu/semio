// #region 🧲️Header
// .storybook/story/elements/ui/App.stories.tsx
// #endregion 🧲️Header

// #region 🔌️Adapters
import { App, Mode, reactHostPort } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
// #endregion 🔌️Adapters

const meta = {
  title: "🖱️ui⚛️react/App",
  component: App,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof App>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [activeModeId, setActiveModeId] = reactHostPort.useState("design");
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
              children: <Mode windows={[{ id: "preview", children: <div className="flex h-full items-center justify-center">Review Preview</div> }]} activeWindowId="preview" />,
            },
          ]}
          activeModeId={activeModeId}
          onActiveModeChange={setActiveModeId}
        />
      </div>
    );
  },
};
