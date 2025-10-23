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
import Navbar from "./Navbar";

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
      { id: "search", content: <Search size={20} />, onClick: () => alert("Search"), order: 0 },
    ],
    rightItems: [
      { id: "bell", content: <Bell size={20} />, onClick: () => alert("Notifications"), order: 0 },
      { id: "settings", content: <Settings size={20} />, onClick: () => alert("Settings"), order: 1 },
      { id: "user", content: <User size={20} />, onClick: () => alert("Profile"), order: 2 },
    ],
    height: 48,
  },
};

export const WithSearchBar: Story = {
  args: {
    leftItems: [
      { id: "logo", content: <span className="font-bold text-lg">Logo</span>, order: 0 },
    ],
    centerItems: [
      {
        id: "search",
        content: (
          <div className="flex items-center gap-2 bg-panel px-3 py-1 rounded border">
            <Search size={16} />
            <input type="text" placeholder="Search..." className="bg-transparent outline-none w-64" />
          </div>
        ),
        order: 0,
      },
    ],
    rightItems: [
      { id: "user", content: <div className="flex items-center gap-2"><User size={20} /><ChevronDown size={16} /></div>, order: 0 },
    ],
    height: 48,
  },
};

export const MinimalCentered: Story = {
  args: {
    centerItems: [
      { id: "title", content: <h1 className="text-xl font-bold">Centered Title</h1>, order: 0 },
    ],
    height: 48,
  },
};

export const Tall: Story = {
  args: {
    leftItems: [
      { id: "logo", content: <div className="text-2xl font-bold">BRAND</div>, order: 0 },
    ],
    rightItems: [
      { id: "nav1", content: <button className="px-4 py-2">Home</button>, order: 0 },
      { id: "nav2", content: <button className="px-4 py-2">About</button>, order: 1 },
      { id: "nav3", content: <button className="px-4 py-2">Contact</button>, order: 2 },
    ],
    height: 64,
  },
};
