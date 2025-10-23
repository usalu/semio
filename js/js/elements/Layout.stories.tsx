// #region Header

// Layout.stories.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

import type { Meta, StoryObj } from "@storybook/react";
import { Home, Settings, User } from "lucide-react";
import { useState } from "react";
import { Canvas, HorizontalWindows } from "./Canvas";
import Footer from "./Footer";
import Layout from "./Layout";
import Navbar from "./Navbar";

const meta = {
  title: "Elements/Layout",
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

export const Complete: Story = {
  render: () => {
    const [leftSize, setLeftSize] = useState(250);
    const [rightSize, setRightSize] = useState(300);
    const [bottomSize, setBottomSize] = useState(200);

    return (
      <Layout
        navbar={
          <Navbar
            leftItems={[
              { id: "home", content: <Home size={20} />, onClick: () => console.log("Home"), order: 0 },
              { id: "title", content: <span className="font-bold">Application</span>, order: 1 },
            ]}
            centerItems={[{ id: "search", content: <input type="text" placeholder="Search..." className="px-2 py-1 bg-panel border rounded" />, order: 0 }]}
            rightItems={[
              { id: "settings", content: <Settings size={20} />, onClick: () => console.log("Settings"), order: 0 },
              { id: "user", content: <User size={20} />, onClick: () => console.log("User"), order: 1 },
            ]}
            height={48}
          />
        }
        footer={
          <Footer
            items={[
              { id: "status", content: "Ready", tooltip: "Application status", order: 0 },
              { id: "cursor", content: "Ln 1, Col 1", order: 1 },
              { id: "selection", content: "UTF-8", order: 2 },
            ]}
            height={20}
          />
        }
        leftPanel={{
          visible: true,
          size: leftSize,
          onSizeChange: setLeftSize,
          sections: [
            {
              id: "explorer",
              label: "Explorer",
              content: <div className="p-2">Explorer content</div>,
              defaultOpen: true,
              order: 0,
            },
            {
              id: "search",
              label: "Search",
              content: <div className="p-2">Search content</div>,
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
              label: "Properties",
              content: <div className="p-2">Properties content</div>,
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
              label: "Console",
              content: <div className="p-2 font-mono text-xs">Console output...</div>,
              defaultOpen: true,
              order: 0,
            },
          ],
        }}
        canvas={
          <Canvas>
            <HorizontalWindows
              windows={[
                { id: "main", children: <ExampleContent title="Main Window" />, defaultSize: 50 },
                { id: "side", children: <ExampleContent title="Side Window" />, defaultSize: 50 },
              ]}
            />
          </Canvas>
        }
      />
    );
  },
};

export const MinimalLayout: Story = {
  render: () => (
    <Layout
      navbar={<Navbar centerItems={[{ id: "title", content: <span className="font-bold">Minimal App</span>, order: 0 }]} />}
      canvas={
        <Canvas>
          <ExampleContent title="Main Content" />
        </Canvas>
      }
    />
  ),
};

export const WithoutFooter: Story = {
  render: () => {
    const [leftSize, setLeftSize] = useState(250);

    return (
      <Layout
        navbar={<Navbar leftItems={[{ id: "title", content: <span className="font-bold">App Without Footer</span>, order: 0 }]} />}
        leftPanel={{
          visible: true,
          size: leftSize,
          onSizeChange: setLeftSize,
          sections: [
            {
              id: "nav",
              label: "Navigation",
              content: <div className="p-2">Navigation items</div>,
              defaultOpen: true,
              order: 0,
            },
          ],
        }}
        canvas={
          <Canvas>
            <ExampleContent title="Content Area" />
          </Canvas>
        }
      />
    );
  },
};
