// #region 🧲️Header

// 🥼️ .storybook/stories/ui/DragAndDrop.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// 🫳️ `DraggableAvatar` already has dedicated coverage in `Avatar.stories.tsx` (DraggableAvatarDefault) — this file
// only stories the still-uncovered `DragHandle` affordance, plus one composition showing both together.
import { DragHandle, DraggableAvatar } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";

// 🫳️#region 🫳️DragHandle
const meta = {
  title: "🖱️ui⚛️react/DragAndDrop",
  component: DragHandle,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof DragHandle>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {},
};

export const Emphasized: Story = {
  args: {
    emphasized: true,
  },
};

// #endregion 🫳️DragHandle

// #region 🧺️ReorderableList
const initialRows = ["Capsule J", "Capsule L", "Capsule P", "Balcony J"];

/** @emoji 🧺️ Pointer-driven reorder demo pairing {@link DragHandle} with {@link DraggableAvatar} rows — no dnd-kit wiring required for the story, just local list state. */
const ReorderableList = () => {
  const [rows, setRows] = useState(initialRows);
  const [draggedRow, setDraggedRow] = useState<string | null>(null);

  const onDropOn = (targetRow: string) => {
    if (!draggedRow || draggedRow === targetRow) return;
    setRows((current) => {
      const withoutDragged = current.filter((row) => row !== draggedRow);
      const targetIndex = withoutDragged.indexOf(targetRow);
      return [...withoutDragged.slice(0, targetIndex), draggedRow, ...withoutDragged.slice(targetIndex)];
    });
    setDraggedRow(null);
  };

  return (
    <div className="flex w-64 flex-col gap-1" data-hover-scope>
      {rows.map((row) => (
        <div
          key={row}
          draggable
          onDragStart={() => setDraggedRow(row)}
          onDragOver={(event) => event.preventDefault()}
          onDrop={() => onDropOn(row)}
          className="flex items-center gap-single rounded-sm border p-single"
        >
          <DragHandle />
          <DraggableAvatar content={row.slice(0, 2).toUpperCase()} title={row} isSelected={draggedRow === row} />
          <span className="text-sm">{row}</span>
        </div>
      ))}
    </div>
  );
};

export const ReorderableRows: Story = {
  name: "Reorderable Rows",
  args: {},
  render: () => <ReorderableList />,
};

// #endregion 🧺️ReorderableList
