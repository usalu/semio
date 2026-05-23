// #region 🧲Header

// 🥼︎ semio/js/.storybook/stories/elements/Layout.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Canvas, Footer, HorizontalWindows, Layout, Navbar, Page, SidePanel, Window } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";
import { Home, Info, Layers, Settings, User } from "lucide-react";
import { useState } from "react";

// 🔷#region 🪨Layout
const meta = {
  title: "elements/react/Layout",
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
    const [leftSize, setLeftSize] = useState(250);
    const [rightSize, setRightSize] = useState(300);
    const [bottomSize, setBottomSize] = useState(200);

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
              { id: "status", content: "Ready", order: 0 },
              { id: "cursor", content: "Ln 1, Col 1", order: 1 },
              { id: "selection", content: "UTF-8", order: 2 },
            ]}
          />
        }
        leftPanel={{
          visible: true,
          size: leftSize,
          onSizeChange: setLeftSize,
          sections: [
            {
              id: "explorer",
              content: <div className="p-double">Explorer content</div>,
              defaultOpen: true,
              order: 0,
            },
            {
              id: "search",
              content: <div className="p-double">Search content</div>,
              defaultOpen: false,
              order: 1,
            },
          ],
        }}
        rightPanel={{
          visible: true,
          size: rightSize,
          onSizeChange: setRightSize,
          sections: [
            {
              id: "properties",
              content: <div className="p-double">Properties content</div>,
              defaultOpen: true,
              order: 0,
            },
          ],
        }}
        bottomPanel={{
          visible: true,
          size: bottomSize,
          onSizeChange: setBottomSize,
          sections: [
            {
              id: "console",
              content: <div className="p-double font-mono text-xs">Console output...</div>,
              defaultOpen: true,
              order: 0,
            },
          ],
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
      <Page frontmatter={{ title: "Getting Started", description: "Learn how to use semio to design modular architecture." }}>
        <h2>Introduction</h2>
        <p>Semio is a platform for kit-of-parts architecture. It helps you model, design and collaborate on modular buildings.</p>
        <h2>Prerequisites</h2>
        <p>You need a modern web browser and basic understanding of architectural concepts.</p>
      </Page>
    </div>
  ),
};
// #endregion 🌈Page

// 💻#region 📌SidePanel
export const SidePanelDefault: Story = {
  args: { canvas: null },
  render: () => {
    const [size, setSize] = useState(300);
    return (
      <div className="relative h-[400px] w-[600px] border bg-base">
        <SidePanel
          position="left"
          size={size}
          onSizeChange={setSize}
          tabs={[
            { id: "types", icon: Layers, order: 0, content: <div className="p-2">Types panel content</div> },
            { id: "settings", icon: Settings, order: 1, content: <div className="p-2">Settings panel content</div> },
            { id: "info", icon: Info, order: 2, content: <div className="p-2">Info panel content</div> },
          ]}
        />
      </div>
    );
  },
};
// #endregion 📌SidePanel
