// #region 🧲Header

// 🥼︎ .storybook/stories/ui/PanelTabBar.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🔌Adapters
import { createIconComponent, PanelChromeTabBar, PanelTabBar, type PanelTabNode } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
// #endregion 🔌Adapters

// 📑#region 📑PanelTabBar
const Layers = createIconComponent("layers");
const Info = createIconComponent("info");
const Settings = createIconComponent("settings");

const nestedTabs: PanelTabNode[] = [
  {
    kind: "branch",
    id: "workbench",
    icon: Layers,
    name: "Workbench",
    order: 0,
    children: [
      { kind: "leaf", id: "document", icon: Info, name: "Document", order: 0, trees: [] },
      { kind: "leaf", id: "catalogue", icon: Layers, name: "Catalogue", order: 1, trees: [] },
    ],
  },
  { kind: "leaf", id: "settings", icon: Settings, name: "Settings", order: 1, trees: [] },
];

const meta = {
  title: "🖱️ui⚛️react/PanelTabBar",
  component: PanelTabBar,
  parameters: {
    layout: "padded",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof PanelTabBar>;

export default meta;

type Story = StoryObj<typeof meta>;

export const PanelVariant: Story = {
  name: "variant=panel (nested rows stack downward)",
  render: () => {
    const [activePath, setActivePath] = useState<readonly string[]>([]);
    return (
      <div className="w-[360px] border bg-base p-single">
        <PanelTabBar variant="panel" tabs={nestedTabs} activePath={activePath} onActivePathChange={setActivePath} direction="down" />
      </div>
    );
  },
};

export const MobileVariant: Story = {
  name: "variant=mobile (single flat row)",
  render: () => {
    const [activePath, setActivePath] = useState<readonly string[]>([]);
    return (
      <div className="w-[360px] border bg-base p-single">
        <PanelTabBar variant="mobile" tabs={nestedTabs} activePath={activePath} onActivePathChange={setActivePath} />
      </div>
    );
  },
};

function ChromeTabBarDemo() {
  const [visible, setVisible] = useState(false);
  const [activeTabPath, setActiveTabPath] = useState<readonly string[]>([]);
  return (
    <div className="flex h-large w-[420px] items-center border bg-window p-single">
      <PanelChromeTabBar anchor="top-left" tabs={nestedTabs} visible={visible} onVisibleChange={setVisible} activeTabPath={activeTabPath} onActiveTabPathChange={setActiveTabPath} />
    </div>
  );
}

export const ChromeHosted: Story = {
  name: "PanelChromeTabBar — hosted inline in navbar chrome",
  render: () => <ChromeTabBarDemo />,
};
// #endregion 📑PanelTabBar
