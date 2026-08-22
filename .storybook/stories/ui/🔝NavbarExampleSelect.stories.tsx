// #region 🧲️Header

// 🥼️ .storybook/stories/ui/🔝NavbarExampleSelect.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import { NavbarExampleSelect } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";
// #endregion 🔌️Adapters

// 🧪️#region 🩺️NavbarExampleSelect
const exampleOptions = [
  { id: "nakagin", label: "Nakagin Capsule Tower", icon: "building" },
  { id: "villa", label: "Villa Savoye", icon: "landmark" },
  { id: "farnsworth", label: "Farnsworth House", icon: "home" },
] as const;

const meta = {
  title: "🖱️ui⚛️react/NavbarExampleSelect",
  component: NavbarExampleSelect,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof NavbarExampleSelect>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [value, setValue] = useState("");
    return (
      <div className="w-64 border ui-surface" data-level="window">
        <NavbarExampleSelect id="navbar.story.example" value={value} options={exampleOptions} onValueChange={setValue} />
      </div>
    );
  },
};

export const WithoutNoExampleOption: Story = {
  name: 'includeNoExample={false}',
  render: () => {
    const [value, setValue] = useState("nakagin");
    return (
      <div className="w-64 border ui-surface" data-level="window">
        <NavbarExampleSelect id="navbar.story.example-no-empty" value={value} options={exampleOptions} onValueChange={setValue} includeNoExample={false} />
      </div>
    );
  },
};
// #endregion 🩺️NavbarExampleSelect
