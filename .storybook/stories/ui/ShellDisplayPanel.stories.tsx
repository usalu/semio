// #region 🧲Header

// 🥼︎ .storybook/stories/ui/ShellDisplayPanel.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🔌Adapters
import { ShellDisplayPanel } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
// #endregion 🔌Adapters

// 🧭#region 🧭ShellDisplayPanel
const layouts = [
  { id: "default", label: "Default" },
  { id: "wide", label: "Wide" },
  { id: "focus", label: "Focus" },
];

const meta = {
  title: "🖱️ui⚛️react/ShellDisplayPanel",
  component: ShellDisplayPanel,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ShellDisplayPanel>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [activeLayoutId, setActiveLayoutId] = useState("default");
    const [compact, setCompact] = useState(false);
    return (
      <div className="w-80 border bg-panel">
        <ShellDisplayPanel layouts={layouts} activeLayoutId={activeLayoutId} onLayoutChange={setActiveLayoutId} onSaveLayout={() => {}} onApplyLayout={() => {}} compact={compact} onCompactChange={setCompact} />
      </div>
    );
  },
};

export const CompactOn: Story = {
  render: () => {
    const [activeLayoutId, setActiveLayoutId] = useState("wide");
    const [compact, setCompact] = useState(true);
    return (
      <div className="w-80 border bg-panel">
        <ShellDisplayPanel layouts={layouts} activeLayoutId={activeLayoutId} onLayoutChange={setActiveLayoutId} onSaveLayout={() => {}} onApplyLayout={() => {}} compact={compact} onCompactChange={setCompact} />
      </div>
    );
  },
};
// #endregion 🧭ShellDisplayPanel
