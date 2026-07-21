// #region 🧲Header

// 🥼︎ .storybook/stories/ui/MobilePanel.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🔌Adapters
import { createIconComponent, MobilePanel, singleTreeLeaf } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
// #endregion 🔌Adapters

// 📱#region 💧MobilePanel
const Layers = createIconComponent("layers");
const Info = createIconComponent("info");
const Settings = createIconComponent("settings");

const mobileTabs = [
  singleTreeLeaf({ id: "mobile.document", icon: Layers, name: "Document", order: 0, tree: { sections: [{ id: "mobile.document.section", label: "", items: [{ id: "mobile.document.item", label: "", control: <div className="p-2">Document body</div> }] }] } }),
  singleTreeLeaf({ id: "mobile.info", icon: Info, name: "Info", order: 1, tree: { sections: [{ id: "mobile.info.section", label: "", items: [{ id: "mobile.info.item", label: "", control: <div className="p-2">Info body</div> }] }] } }),
  singleTreeLeaf({ id: "mobile.settings", icon: Settings, name: "Settings", order: 2, tree: { sections: [{ id: "mobile.settings.section", label: "", items: [{ id: "mobile.settings.item", label: "", control: <div className="p-2">Settings body</div> }] }] } }),
];

const meta = {
  title: "🖱️ui⚛️react/MobilePanel",
  component: MobilePanel,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof MobilePanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: { visible: true, tabs: mobileTabs },
  render: (args) => {
    const [activeTabPath, setActiveTabPath] = useState<readonly string[]>([]);
    return (
      <div className="flex flex-col h-[420px] w-[375px] border bg-base mx-auto">
        <MobilePanel {...args} activeTabPath={activeTabPath} onActiveTabPathChange={setActiveTabPath} />
      </div>
    );
  },
};

export const Hidden: Story = {
  args: { visible: false, tabs: mobileTabs },
  render: (args) => (
    <div className="flex flex-col h-[420px] w-[375px] border bg-base mx-auto">
      <MobilePanel {...args} />
      <div className="p-4 text-xs text-muted-foreground">visible=false renders nothing — this text is the only content.</div>
    </div>
  ),
};
// #endregion 💧MobilePanel
