// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/Navbar.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import { Bell, ChevronDown, Home, Menu, Redo, Search, Settings, Undo, User, ZoomIn, ZoomOut } from "lucide-react";
import { Button, Level, LevelProvider, Navbar, NavbarItem, Toggle, ToolbarDivider, ToolbarGroup, ToolbarItem, ToolbarZone, getLevelBgClass } from "@semio-elements/ui";

// #region 🔖Navbar
const meta = {
  title: "Elements/Navbar",
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

const defaultItems: NavbarItem[] = [
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
];

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <Navbar items={defaultItems} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { items: defaultItems },
  render: createLevelRender("base"),
};

export const Window: Story = {
  args: { items: defaultItems },
  render: createLevelRender("window"),
};

export const Panel: Story = {
  args: { items: defaultItems },
  render: createLevelRender("panel"),
};

export const Overlay: Story = {
  args: { items: defaultItems },
  render: createLevelRender("overlay"),
};

export const Temporary: Story = {
  args: { items: defaultItems },
  render: createLevelRender("temporary"),
};

// #endregion 🔖Navbar

// #region 🔖Toolbar
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
// #endregion 🔖Toolbar

