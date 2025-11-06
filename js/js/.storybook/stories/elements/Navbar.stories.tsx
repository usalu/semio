// #region Header

// Navbar.stories.tsx

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
import { Bell, ChevronDown, Home, Menu, Search, Settings, User } from "lucide-react";
import { Navbar } from "../../../sketchpad/elements";

// #region Navbar
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
    leftItems: [
      { id: "menu", content: <Menu size={20} />, onClick: () => alert("Menu"), order: 0 },
      { id: "home", content: <Home size={20} />, onClick: () => alert("Home"), order: 1 },
      { id: "title", content: <span className="font-bold ml-2">Application</span>, order: 2 },
    ],
    centerItems: [
      {
        id: "search",
        content: (
          <div className="flex items-center gap-double bg-panel px-3 py-1 rounded border">
            <Search size={16} />
            <input type="text" placeholder="Search..." className="bg-transparent outline-none w-64" />
          </div>
        ),
        order: 0,
      },
    ],
    rightItems: [
      { id: "bell", content: <Bell size={20} />, onClick: () => alert("Notifications"), order: 0 },
      { id: "settings", content: <Settings size={20} />, onClick: () => alert("Settings"), order: 1 },
      {
        id: "user",
        content: (
          <div className="flex items-center gap-double">
            <User size={20} />
            <ChevronDown size={16} />
          </div>
        ),
        onClick: () => alert("Profile"),
        order: 2,
      },
    ],
    height: 64,
  },
};

// #endregion Navbar
