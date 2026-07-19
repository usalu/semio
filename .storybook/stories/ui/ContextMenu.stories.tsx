// #region 🧲Header

// 🥼︎ .storybook/stories/ui/ContextMenu.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Button, ContextMenu, ContextMenuController, type ContextMenuItem } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

// 🖱️#region 🖱️ContextMenu
const meta = {
  title: "🖱️ui⚛️react/ContextMenu",
  component: ContextMenu,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ContextMenu>;

export default meta;

type Story = StoryObj<typeof meta>;

const sampleItems: ContextMenuItem[] = [
  { id: "rename", label: "Rename", shortcut: "F2" },
  { id: "duplicate", label: "Duplicate", shortcut: "⌘D" },
  { id: "visible", label: "Visible", checked: true },
  { id: "sep-1", label: "", separator: true },
  {
    id: "transform",
    label: "Transform",
    children: [
      { id: "transform.move", label: "Move" },
      { id: "transform.rotate", label: "Rotate" },
      { id: "transform.scale", label: "Scale", disabled: true },
    ],
  },
  { id: "sep-2", label: "", separator: true },
  { id: "delete", label: "Delete", destructive: true, shortcut: "⌫" },
];

export const Default: Story = {
  args: {
    items: sampleItems,
    children: <div className="flex size-40 items-center justify-center border text-sm text-muted-foreground">Right-click me</div>,
  },
};

export const NoItems: Story = {
  name: "No Items (Native Menu Suppressed)",
  args: {
    items: [],
    children: <div className="flex size-40 items-center justify-center border text-sm text-muted-foreground">Right-click does nothing</div>,
  },
};

// #endregion 🖱️ContextMenu

// #region 🎯ContextMenuController
const controllerItems: ContextMenuItem[] = [
  { id: "focus", label: "Focus", shortcut: "F" },
  { id: "select", label: "Select", checked: false },
  { id: "sep", label: "", separator: true },
  { id: "delete", label: "Delete", destructive: true },
];

/** @emoji 🎯 Controlled fixed-position menu — mirrors how puzzle 2d canvas surfaces open a right-click menu at pointer coordinates. */
const ControlledContextMenuDemo = () => {
  const [open, setOpen] = useState(false);
  const [position, setPosition] = useState<{ x: number; y: number } | null>(null);
  return (
    <div className="relative flex size-40 items-center justify-center border text-sm text-muted-foreground">
      <Button
        id="context-menu-controller-story-open"
        text="Open at center"
        onClick={(event) => {
          const rect = event.currentTarget.closest(".relative")?.getBoundingClientRect();
          setPosition({ x: (rect?.left ?? 0) + (rect?.width ?? 0) / 2, y: (rect?.top ?? 0) + (rect?.height ?? 0) / 2 });
          setOpen(true);
        }}
      />
      <ContextMenuController open={open} position={position} items={controllerItems} onOpenChange={setOpen} />
    </div>
  );
};

type ControllerStory = StoryObj<typeof ContextMenuController>;

export const Controlled: ControllerStory = {
  name: "ContextMenuController",
  args: {
    open: false,
    position: null,
    items: controllerItems,
    onOpenChange: () => {},
  },
  render: () => <ControlledContextMenuDemo />,
};

// #endregion 🎯ContextMenuController
