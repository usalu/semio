// #region 🧲Header

// 🥼︎ .storybook/stories/ui/BasicChatPanel.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { BasicChatPanel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

// 💬#region 🎇BasicChatPanel
const meta = {
  title: "🖱️ui⚛️react/BasicChatPanel",
  component: BasicChatPanel,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof BasicChatPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "storybook.basicChatPanel.default",
    title: "Design Assistant",
  },
  render: (args) => (
    <div className="h-[420px] w-[360px] border p-single">
      <BasicChatPanel {...args} />
    </div>
  ),
};

export const Narrow: Story = {
  name: "Narrow Panel",
  args: {
    id: "storybook.basicChatPanel.narrow",
    title: "Notes",
  },
  render: (args) => (
    <div className="h-[420px] w-[240px] border p-single">
      <BasicChatPanel {...args} />
    </div>
  ),
};

// #endregion 🎇BasicChatPanel
