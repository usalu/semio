// #region 🧲Header

// 🥼︎ .storybook/stories/ui/Layout.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Canvas, Panel, Footer, HorizontalWindows, Layout, Navbar, Page, singleTreeLeaf, Window } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState, type ComponentType } from "react";

// 🔷#region 🪨Layout
const Home = createIconComponent("home");
const Info = createIconComponent("info");
const Layers = createIconComponent("layers");
const Settings = createIconComponent("settings");
const User = createIconComponent("user");

const meta = {
  title: "🖱️ui⚛️react/Layout",
  component: Layout,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Layout>;

export default meta;

type Story = StoryObj<typeof meta>;

const ExampleContent = ({ title }: { title: string }) => (
  <div className="flex items-center justify-center h-full bg-panel">
    <h2 className="text-2xl font-bold">{title}</h2>
  </div>
);

export const Default: Story = {
  args: { canvas: null },
  render: () => {
    const [topLeftSize, setTopLeftSize] = useState(250);
    const [topMiddleSize, setTopMiddleSize] = useState(360);
    const [topRightSize, setTopRightSize] = useState(300);

    return (
      <Layout
        navbar={
          <Navbar
            items={[
              { content: <Home size={20} />, key: "home" },
              { content: <span className="font-bold">Application</span>, key: "title" },
              { content: <input type="text" placeholder="Search..." className="px-2 py-1 bg-panel border rounded w-full" />, key: "search", className: "flex-1" },
              { content: <Settings size={20} />, key: "settings" },
              { content: <User size={20} />, key: "user" },
            ]}
          />
        }
        footer={
          <Footer
            items={[
              { content: "Ready", key: "status" },
              { content: "Ln 1, Col 1", key: "cursor" },
              { content: "UTF-8", key: "selection" },
            ]}
          />
        }
        panels={{
          "top-left": {
            visible: true,
            size: topLeftSize,
            onSizeChange: setTopLeftSize,
            tabs: [
              singleTreeLeaf({
                id: "explorer",
                icon: Layers,
                name: "Explorer",
                order: 0,
                tree: { sections: [{ id: "explorer.section", label: "", items: [{ id: "explorer.item", label: "", control: <div className="p-double">Explorer content</div> }] }] },
              }),
              singleTreeLeaf({ id: "search", icon: Info, name: "Search", order: 1, tree: { sections: [{ id: "search.section", label: "", items: [{ id: "search.item", label: "", control: <div className="p-double">Search content</div> }] }] } }),
            ],
          },
          "top-middle": {
            visible: true,
            size: topMiddleSize,
            onSizeChange: setTopMiddleSize,
            tabs: [singleTreeLeaf({ id: "console", icon: Info, name: "Console", order: 0, tree: { sections: [{ id: "console.section", label: "", items: [{ id: "console.item", label: "", control: <div className="p-double font-mono text-xs">Console output...</div> }] }] } })],
          },
          "top-right": {
            visible: true,
            size: topRightSize,
            onSizeChange: setTopRightSize,
            tabs: [
              singleTreeLeaf({
                id: "properties",
                icon: Settings,
                name: "Properties",
                order: 0,
                tree: { sections: [{ id: "properties.section", label: "", items: [{ id: "properties.item", label: "", control: <div className="p-double">Properties content</div> }] }] },
              }),
            ],
          },
        }}
        canvas={
          <Canvas>
            <HorizontalWindows>
              <Window id="main" defaultSize={50}>
                <ExampleContent title="Main Window" />
              </Window>
              <Window id="side" defaultSize={50}>
                <ExampleContent title="Side Window" />
              </Window>
            </HorizontalWindows>
          </Canvas>
        }
      />
    );
  },
};

// #endregion 🪨Layout

// 🔷#region 🌈Page
export const PageDefault: Story = {
  args: { canvas: null },
  render: () => (
    <div className="h-[400px] w-[600px] border">
      <Page frontmatter={{ title: "Getting Started", description: "Learn how to use compose to design modular architecture." }}>
        <h2>Introduction</h2>
        <p>Compose is a platform for kit-of-parts architecture. It helps you model, design and collaborate on modular buildings.</p>
        <h2>Prerequisites</h2>
        <p>You need a modern web browser and basic understanding of architectural concepts.</p>
      </Page>
    </div>
  ),
};
// #endregion 🌈Page

// 💻#region 🧭Panel
const layoutPanelLeafTab = (id: string, icon: ComponentType<{ size?: number }>, name: string, order: number, content: string) =>
  singleTreeLeaf({ id, icon, name, order, tree: { sections: [{ id: `${id}.section`, label: "", items: [{ id: `${id}.item`, label: "", control: <div className="p-2">{content}</div> }] }] } });

export const PanelDefault: Story = {
  args: { canvas: null },
  render: () => {
    const [size, setSize] = useState(300);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <Panel
          anchor="top-left"
          size={size}
          onSizeChange={setSize}
          tabs={[layoutPanelLeafTab("types", Layers, "Types", 0, "Types panel content"), layoutPanelLeafTab("settings", Settings, "Settings", 1, "Settings panel content"), layoutPanelLeafTab("info", Info, "Info", 2, "Info panel content")]}
        />
      </div>
    );
  },
};
// #endregion 🧭Panel
