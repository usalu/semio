// #region 🧲Header

// 🥼︎ compose/js/.storybook/story/elements/Navbar.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Button, LevelProvider, Navbar, Toggle, ToolbarDivider, ToolbarGroup, ToolbarItem, ToolbarZone } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";

// 🔷#region 🩺Navbar
const Bell = createIconComponent("bell");
const ChevronDown = createIconComponent("chevron-down");
const Home = createIconComponent("home");
const Menu = createIconComponent("list");
const PanelLeft = createIconComponent("panel-left");
const PanelRight = createIconComponent("panel-right");
const Redo = createIconComponent("rotate-cw");
const Search = createIconComponent("search");
const Settings = createIconComponent("settings");
const Undo = createIconComponent("rotate-ccw");
const User = createIconComponent("user");
const ZoomIn = createIconComponent("zoom-in");
const ZoomOut = createIconComponent("zoom-out");

const meta = {
  title: "🖱️ui⚛️react/Navbar",
  component: Navbar,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Navbar>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    items: [
      { key: "menu", content: <Menu size={20} /> },
      { key: "home", content: <Home size={20} /> },
      { key: "title", content: <span className="font-bold ml-2">Application</span> },
      {
        key: "search",
        content: (
          <div className="flex items-center gap-double bg-panel px-3 py-1 rounded border w-full">
            <Search size={16} />
            <input type="text" placeholder="Search..." className="bg-transparent outline-none w-full" />
          </div>
        ),
        className: "flex-1",
      },
      { key: "bell", content: <Bell size={20} /> },
      { key: "settings", content: <Settings size={20} /> },
      {
        key: "user",
        content: (
          <div className="flex items-center gap-double">
            <User size={20} />
            <ChevronDown size={16} />
          </div>
        ),
      },
    ],
  },
};

// #endregion 🩺Navbar

// 🔷#region 🎛TwoToggleNavbar
// Each panel toggle sits at its own navbar edge — left toggle leading, right toggle trailing (past the
// fullscreen toggle) — and stays put while its panel folds out flush beneath it, mirroring a window's
// options-rail chrome (toggle stays, body unfolds).
export const TwoToggleNavbar: Story = {
  name: "Two-Toggle Navbar",
  args: { items: [] },
  render: () => {
    const [leftVisible, setLeftVisible] = useState(true);
    const [rightVisible, setRightVisible] = useState(false);
    return (
      <div>
        <Navbar
          items={[
            { key: "leftPanelToggle", className: "shrink-0", content: <Toggle id="ui.panelToggle.left" icon={<PanelLeft size={16} />} pressed={leftVisible} onPressedChange={setLeftVisible} /> },
            { key: "home", content: <Home size={20} /> },
            { key: "title", content: <span className="font-bold ml-2">Application</span> },
            { key: "fill", className: "flex-1 min-w-0", content: null },
          ]}
          showFullscreenToggle={false}
          trailingContent={<Toggle id="ui.panelToggle.right" icon={<PanelRight size={16} />} pressed={rightVisible} onPressedChange={setRightVisible} />}
        />
        <div className="relative h-64 bg-canvas">
          {leftVisible ? <div className="absolute left-single top-0 bottom-single w-56 border !border-normal bg-panel" /> : null}
          {rightVisible ? <div className="absolute right-single top-0 bottom-single w-56 border !border-normal bg-panel" /> : null}
        </div>
      </div>
    );
  },
};
// #endregion 🎛TwoToggleNavbar

// 🔷#region 🌙Toolbar
export const ToolbarDefault: Story = {
  args: { items: defaultItems },
  render: () => (
    <LevelProvider level="panel">
      <div className="p-4 bg-panel">
        <ToolbarZone>
          <ToolbarGroup>
            <ToolbarItem>
              <Button id="toolbar-undo" variant="ghost" icon={<Undo className="size-tiny" />} onClick={() => {}} />
            </ToolbarItem>
            <ToolbarItem>
              <Button id="toolbar-redo" variant="ghost" icon={<Redo className="size-tiny" />} onClick={() => {}} />
            </ToolbarItem>
          </ToolbarGroup>
          <ToolbarDivider />
          <ToolbarGroup>
            <ToolbarItem>
              <Toggle id="toolbar-zoom-in" pressed={false} onPressedChange={() => {}} icon={<ZoomIn className="size-tiny" />} />
            </ToolbarItem>
            <ToolbarItem>
              <Toggle id="toolbar-zoom-out" pressed={false} onPressedChange={() => {}} icon={<ZoomOut className="size-tiny" />} />
            </ToolbarItem>
          </ToolbarGroup>
          <ToolbarDivider />
          <ToolbarGroup>
            <ToolbarItem>
              <span className="text-xs text-muted-foreground">100%</span>
            </ToolbarItem>
          </ToolbarGroup>
        </ToolbarZone>
      </div>
    </LevelProvider>
  ),
};
// #endregion 🌙Toolbar
