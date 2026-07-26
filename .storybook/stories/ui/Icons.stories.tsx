// #region 🧲Header

// 🥼︎ .storybook/stories/ui/Icons.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Cursor, Icon, LoadingRow, Spinner } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import { ICON_NAMES, ICON_CONCEPT_ASSIGNMENTS, type IconName } from "@semio-tech/ui-asset";
import type { Meta, StoryObj } from "@storybook/react";

// 🖼️#region 🛒Icons
const Box = createIconComponent("box");

const meta = {
  title: "🖱️ui⚛️react/Icons",
  component: Cursor,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Cursor>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: { color: "#000000" },
  render: () => (
    <div className="relative w-96 h-96 border bg-muted/10 p-4">
      <p className="text-sm font-medium mb-4">Collaborative Cursors</p>
      <div className="relative w-full h-full">
        <Cursor color="var(--accent)" x={120} y={80} />
        <div className="absolute left-28 top-20 ml-6 mt-1 bg-accent text-active-foreground border px-double py-1 text-xs">Alice</div>
        <Cursor color="var(--status-success)" x={200} y={150} />
        <div className="absolute left-48 top-36 ml-6 mt-1 bg-status-success text-active-foreground border px-double py-1 text-xs">Bob</div>
        <Cursor color="var(--status-warning)" x={280} y={200} />
        <div className="absolute left-68 top-small ml-6 mt-1 bg-status-warning text-active-foreground border px-double py-1 text-xs">Charlie</div>
        <Cursor color="var(--status-danger)" x={100} y={250} />
        <Cursor color="var(--accent-secondary)" x={250} y={300} />
      </div>
    </div>
  ),
};

// 🔷#region 🫨IconAnimations
const NON_CATALOG_KIND_TILES: { readonly label: string; readonly icon: React.ComponentProps<typeof Icon>["icon"] }[] = [
  { label: "kind:emoji", icon: { kind: "emoji", emoji: "🙂" } },
  { label: "kind:text", icon: { kind: "text", text: "Hi" } },
  { label: "kind:typst", icon: { kind: "typst", src: "$x^2$" } },
  { label: "kind:image", icon: { kind: "url", url: "https://picsum.photos/32" } },
  { label: "kind:svg", icon: { kind: "svg", svg: '<svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="9" fill="currentColor"/></svg>' } },
  { label: "kind:node", icon: { kind: "node", node: <strong>N</strong> } },
  { label: "kind:missing", icon: "definitely-not-a-vendored-icon" as IconName },
];

export const IconAnimations: Story = {
  args: { color: "#000000" },
  render: () => (
    <div className="max-w-4xl space-y-4">
      <p className="text-sm text-muted-foreground">Hover an icon, or its button, to trigger its closed micro animation once.</p>
      <div className="grid grid-cols-8 gap-2">
        {ICON_NAMES.map((name) => (
          <button key={name} type="button" className="flex flex-col items-center gap-1 rounded-sm border p-2 hover:bg-hover-interactive-fill" title={name}>
            <Icon icon={name} size="large" />
            <span className="truncate w-full text-center text-2xs text-muted-foreground">{name}</span>
          </button>
        ))}
      </div>
      <p className="text-sm font-medium">Non-catalog icon kinds</p>
      <div className="grid grid-cols-8 gap-2">
        {NON_CATALOG_KIND_TILES.map(({ label, icon }) => (
          <button key={label} type="button" className="flex flex-col items-center gap-1 rounded-sm border p-2 hover:bg-hover-interactive-fill" title={label}>
            <Icon icon={icon} size="large" />
            <span className="truncate w-full text-center text-2xs text-muted-foreground">{label}</span>
          </button>
        ))}
      </div>
    </div>
  ),
};
// #endregion 🫨IconAnimations

export const Concepts: Story = {
  args: { color: "#000000" },
  render: () => (
    <div className="grid max-w-5xl grid-cols-2 gap-2 text-2xs">
      {Object.entries(ICON_CONCEPT_ASSIGNMENTS).map(([concept, iconId]) => (
        <div key={concept} className="flex items-center gap-2 rounded-sm border px-2 py-1">
          <Icon icon={iconId} size="small" className="shrink-0" />
          <span className="font-mono text-muted-foreground">{concept}</span>
          <span className="ml-auto font-mono">{iconId}</span>
        </div>
      ))}
    </div>
  ),
};

// #endregion 🛒Icons

// 🔷#region 🎹Spinner
export const SpinnerSizes: Story = {
  args: { color: "#000000" },
  render: () => (
    <div className="flex items-center gap-4">
      <Spinner size="small" />
      <Spinner size="medium" />
      <Spinner size="large" />
    </div>
  ),
};
// #endregion 🎹Spinner

// 🔷#region 🎺LoadingRow
export const LoadingRowDefault: Story = {
  args: { color: "#000000" },
  render: () => (
    <div className="w-64 space-y-2">
      <LoadingRow name="Loading types..." icon={<Box className="size-tiny" />} />
      <LoadingRow name="Loading designs..." />
      <LoadingRow name="Processing..." icon={<Spinner size="small" />} />
    </div>
  ),
};
// #endregion 🎺LoadingRow
