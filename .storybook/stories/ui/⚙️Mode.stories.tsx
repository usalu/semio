// #region 🧲️Header
// .storybook/story/elements/ui/⚙️Mode.stories.tsx
// #endregion 🧲️Header

// #region 🔌️Adapters
import { Mode, createEvenWindowLayout, reactHostPort, uiDataLabel } from "@semio-tech/ui-react";
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

// #region 🧪️SilhouetteVisualFixture
const SilhouetteVisualFixture = () => (
  <div data-testid="silhouette-visual-content" data-window-content-layout="edgeless" aria-label="Continuous content / Fortlaufender Inhalt" className="relative h-full min-h-0 overflow-hidden bg-transparent">
    <div
      aria-hidden
      className="pointer-events-none absolute inset-x-0 bottom-0"
      style={{
        top: -48,
        backgroundColor: "#0d2742",
        backgroundImage:
          "linear-gradient(90deg, transparent 0 47px, #ffd43b 47px 55px, transparent 55px 103px, #52e5ff 103px 111px, transparent 111px), linear-gradient(0deg, rgb(255 255 255 / 12%) 1px, transparent 1px)",
        backgroundSize: "160px 100%, 100% 24px",
      }}
    />
    <div aria-hidden className="pointer-events-none absolute bottom-0 w-2 bg-[#ff4d6d]" style={{ insetInlineStart: 23, top: -48 }} />
    <div data-testid="silhouette-visual-label" className="absolute text-sm font-semibold text-white" style={{ insetInlineStart: 72, top: 16 }}>
      0123456789 · Text · Szene · Scene
    </div>
  </div>
);
// #endregion 🧪️SilhouetteVisualFixture

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

export const ContentThroughGlass: Story = {
  args: { windows: [], activeWindowId: "flow" },
  render: () => {
    const [activeWindowId, setActiveWindowId] = reactHostPort.useState<string | null>("flow");
    return (
      <div
        data-testid="silhouette-visual-floor"
        className="h-[420px] w-full p-single"
        style={{
          backgroundColor: "#641f45",
          backgroundImage: "linear-gradient(135deg, rgb(255 255 255 / 10%) 25%, transparent 25% 50%, rgb(255 255 255 / 10%) 50% 75%, transparent 75%)",
          backgroundSize: "32px 32px",
        }}
      >
        <Mode
          windows={[
            { id: "flow", title: uiDataLabel("Flow / Fluss"), children: <SilhouetteVisualFixture /> },
            { id: "reference", title: uiDataLabel("Reference / Referenz"), children: <Pane label="Reference / Referenz" /> },
          ]}
          layout={{
            kind: "stack",
            activeId: "flow",
            children: [
              { kind: "window", id: "flow" },
              { kind: "window", id: "reference" },
            ],
          }}
          activeWindowId={activeWindowId}
          onActiveWindowChange={setActiveWindowId}
        />
      </div>
    );
  },
  play: async ({ canvasElement }) => {
    expect(canvasElement.querySelector('[data-testid="silhouette-visual-content"]')).toBeTruthy();
    expect(canvasElement.querySelectorAll("[data-window-silhouette-chip]").length).toBeGreaterThan(1);
    expect(canvasElement.querySelector("[data-window-silhouette-gap]")).toBeTruthy();
  },
};
