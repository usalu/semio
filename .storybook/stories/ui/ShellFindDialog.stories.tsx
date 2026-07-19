// #region 🧲Header

// 🥼︎ .storybook/stories/ui/ShellFindDialog.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 🔌Adapters
import { ShellFindDialog, type ShellCommandResult } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useMemo, useState } from "react";
// #endregion 🔌Adapters

// 🔎#region 🔎ShellFindDialog
const ALL_RESULTS: ShellCommandResult[] = [
  { id: "node.capsule-j", label: "Capsule J" },
  { id: "node.capsule-l", label: "Capsule L" },
  { id: "node.base-blob", label: "Base Blob" },
];

const meta = {
  title: "🖱️ui⚛️react/ShellFindDialog",
  component: ShellFindDialog,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ShellFindDialog>;

export default meta;

type Story = StoryObj<typeof meta>;

function ShellFindDialogDemo({ initialQuery = "" }: { readonly initialQuery?: string }) {
  const [open, setOpen] = useState(true);
  const [query, setQuery] = useState(initialQuery);
  const results = useMemo(() => ALL_RESULTS.filter((result) => result.label.toLowerCase().includes(query.toLowerCase())), [query]);
  return <ShellFindDialog open={open} query={query} onQueryChange={setQuery} results={results} onPick={() => setOpen(false)} onClose={() => setOpen(false)} />;
}

export const Default: Story = {
  render: () => <ShellFindDialogDemo />,
};

export const Filtered: Story = {
  name: "Pre-filtered query",
  render: () => <ShellFindDialogDemo initialQuery="capsule" />,
};
// #endregion 🔎ShellFindDialog
