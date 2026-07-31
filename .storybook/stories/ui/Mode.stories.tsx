// #region 🧲️Header
// .storybook/story/elements/ui/Mode.stories.tsx
// #endregion 🧲️Header

// #region 🔌️Adapters
import { Mode, createEvenWindowLayout, reactHostPort } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { expect, userEvent, within } from "storybook/test";
// #endregion 🔌️Adapters

// `Mode` mounts each `windows[].children` inside a `mode-dock-stack-body` (level="base") wrapping
// a `<Window>` (level="window"), both of which already fill `ui-surface`/`ui-glass` — this stays
// bg-transparent so it doesn't double-tint either ancestor fill.
const Pane = ({ label }: { label: string }) => (
  <div className="flex h-full items-center justify-center bg-transparent">
    <span className="text-lg font-semibold">{label}</span>
  </div>
);

const meta = {
  title: "🖱️ui⚛️react/Mode",
  component: Mode,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof Mode>;

export default meta;

type Story = StoryObj<typeof meta>;

export const QuadLayout: Story = {
  render: () => {
    const [activeWindowId, setActiveWindowId] = reactHostPort.useState<string | null>("overview");
    return (
      <div className="h-[500px] w-full p-single">
        <Mode
          windows={[
            { id: "overview", title: "Overview", children: <Pane label="Overview" /> },
            { id: "detail", title: "Detail", children: <Pane label="Detail" /> },
            { id: "selection", title: "Selection", children: <Pane label="Selection" /> },
            { id: "context", title: "Context", children: <Pane label="Context" /> },
          ]}
          layout={{
            kind: "row",
            children: [
              {
                kind: "column",
                size: 50,
                children: [
                  { kind: "stack", children: [{ kind: "window", id: "overview" }], activeId: "overview" },
                  { kind: "stack", children: [{ kind: "window", id: "detail" }], activeId: "detail" },
                ],
              },
              {
                kind: "column",
                size: 50,
                children: [
                  { kind: "stack", children: [{ kind: "window", id: "selection" }], activeId: "selection" },
                  { kind: "stack", children: [{ kind: "window", id: "context" }], activeId: "context" },
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
    expect(canvasElement.querySelector('[data-slot="mode-dock-tab"][data-window-id="context"][data-active="true"]')).toBeTruthy();
    expect(canvasElement.querySelector('[data-slot="mode-dock-tab"][data-window-id="overview"][data-active="true"]')).toBeNull();
  },
};

export const TabStack: Story = {
  render: () => {
    const [activeWindowId, setActiveWindowId] = reactHostPort.useState<string | null>("design");
    return (
      <div className="h-[400px] w-full p-single">
        <Mode
          windows={[
            { id: "design", title: "Design", children: <Pane label="Design Pane" /> },
            { id: "review", title: "Review", children: <Pane label="Review Pane" /> },
            { id: "notes", title: "Notes", children: <Pane label="Notes Pane" /> },
          ]}
          layout={{
            kind: "stack",
            activeId: "design",
            children: [
              { kind: "window", id: "design" },
              { kind: "window", id: "review" },
              { kind: "window", id: "notes" },
            ],
          }}
          activeWindowId={activeWindowId}
          onActiveWindowChange={setActiveWindowId}
        />
      </div>
    );
  },
};

export const MaximizeStack: Story = {
  render: () => (
    <div className="h-[400px] w-full p-single">
      <Mode
        windows={[
          { id: "a", title: "Alpha", children: <Pane label="Alpha" /> },
          { id: "b", title: "Beta", children: <Pane label="Beta" /> },
        ]}
        layout={{
          kind: "row",
          children: [
            { kind: "stack", children: [{ kind: "window", id: "a" }], activeId: "a" },
            { kind: "stack", children: [{ kind: "window", id: "b" }], activeId: "b" },
          ],
        }}
        activeWindowId="a"
      />
    </div>
  ),
  play: async ({ canvasElement }) => {
    const maximize = canvasElement.querySelector("[data-slot='mode-dock-maximize']");
    expect(maximize).toBeTruthy();
    await userEvent.click(maximize!);
    expect(within(canvasElement).getByText("Alpha")).toBeTruthy();
    expect(within(canvasElement).queryByText("Beta")).toBeNull();
  },
};

export const EvenSplit: Story = {
  render: () => (
    <div className="h-[400px] w-full p-single">
      <Mode
        windows={[
          { id: "a", title: "A", children: <Pane label="A" /> },
          { id: "b", title: "B", children: <Pane label="B" /> },
        ]}
        layout={createEvenWindowLayout(["a", "b"])}
        activeWindowId="a"
      />
    </div>
  ),
};
