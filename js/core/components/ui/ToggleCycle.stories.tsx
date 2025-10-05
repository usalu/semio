// #region Header

// ToggleCycle.stories.tsx

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
import { Eye, EyeOff, Grid2x2, List, Moon, Sun } from "lucide-react";
import { useState } from "react";
import { ToggleCycle } from "./ToggleCycle";

const meta = {
  title: "Elements/ToggleCycle",
  component: ToggleCycle,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ToggleCycle>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Basic: Story = {
  render: () => {
    const [view, setView] = useState<"list" | "grid">("list");
    return (
      <ToggleCycle
        value={view}
        onValueChange={setView}
        items={[
          { value: "list", label: <List /> },
          { value: "grid", label: <Grid2x2 /> },
        ]}
      />
    );
  },
};

export const ThreeStates: Story = {
  render: () => {
    const [theme, setTheme] = useState<"light" | "dark" | "system">("light");
    return (
      <ToggleCycle
        value={theme}
        onValueChange={setTheme}
        items={[
          { value: "light", label: <Sun /> },
          { value: "dark", label: <Moon /> },
          {
            value: "system",
            label: (
              <>
                <Sun />
                <Moon />
              </>
            ),
          },
        ]}
      />
    );
  },
};

export const WithTooltips: Story = {
  render: () => {
    const [view, setView] = useState<"list" | "grid">("list");
    return (
      <ToggleCycle
        value={view}
        onValueChange={setView}
        items={[
          { value: "list", label: <List />, tooltip: "List View" },
          { value: "grid", label: <Grid2x2 />, tooltip: "Grid View" },
        ]}
      />
    );
  },
};

export const WithText: Story = {
  render: () => {
    const [visibility, setVisibility] = useState<"visible" | "hidden">("visible");
    return (
      <ToggleCycle
        value={visibility}
        onValueChange={setVisibility}
        items={[
          {
            value: "visible",
            label: (
              <>
                <Eye />
                Visible
              </>
            ),
          },
          {
            value: "hidden",
            label: (
              <>
                <EyeOff />
                Hidden
              </>
            ),
          },
        ]}
      />
    );
  },
};

export const OutlineVariant: Story = {
  render: () => {
    const [view, setView] = useState<"list" | "grid">("list");
    return (
      <ToggleCycle
        variant="outline"
        value={view}
        onValueChange={setView}
        items={[
          { value: "list", label: <List /> },
          { value: "grid", label: <Grid2x2 /> },
        ]}
      />
    );
  },
};

export const Sizes: Story = {
  render: () => {
    const [view1, setView1] = useState<"list" | "grid">("list");
    const [view2, setView2] = useState<"list" | "grid">("list");
    const [view3, setView3] = useState<"list" | "grid">("list");

    const items = [
      { value: "list" as const, label: <List /> },
      { value: "grid" as const, label: <Grid2x2 /> },
    ];

    return (
      <div className="flex items-center gap-4">
        <ToggleCycle size="sm" value={view1} onValueChange={setView1} items={items} />
        <ToggleCycle size="default" value={view2} onValueChange={setView2} items={items} />
        <ToggleCycle size="lg" value={view3} onValueChange={setView3} items={items} />
      </div>
    );
  },
};

export const TextOnly: Story = {
  render: () => {
    const [sort, setSort] = useState<"asc" | "desc" | "none">("none");
    return (
      <ToggleCycle
        value={sort}
        onValueChange={setSort}
        items={[
          { value: "none", label: "None" },
          { value: "asc", label: "Asc ↑" },
          { value: "desc", label: "Desc ↓" },
        ]}
      />
    );
  },
};
