// #region 🧲️Header

// 🥼️ .storybook/stories/ui/🔀SortableTreeItems.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import { createIconComponent, SortableTreeItems, Tree, TreeItem } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import { useState } from "react";
// #endregion 🔌️Adapters

// ↕️#region 🌳️SortableTreeItems
const File = createIconComponent("file-text");
const Folder = createIconComponent("folder");

const meta = {
  title: "🖱️ui⚛️react/SortableTreeItems",
  component: SortableTreeItems,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof SortableTreeItems>;

export default meta;

type Story = StoryObj<typeof meta>;

/** @emoji 🌳️ {@link SortableTreeItems} only wires dnd-kit's `DndContext`/`SortableContext` — the drag handle
 * itself belongs to each `TreeItem` (via `sortable`/`sortableId`), which needs the same `TreeContext` a real
 * `Tree` section provides. `TreeDataSection`/`TreeDataItem` have no free-form "content" slot, so this hosts
 * the sortable block via a single item's `control` (same mechanism as `Panel.stories.tsx`'s `leafTab` helper),
 * which `TreeDataItemView` renders as `TreeItem` children — already inside a fresh `TreeContext.Provider`. */
function SortableRowsDemo({ initialRows }: { readonly initialRows: { id: string; label: string }[] }) {
  const [items, setItems] = useState(initialRows);
  return (
    <Tree
      sections={[
        {
          id: "sortable.story.section",
          label: "Variants",
          icon: <Folder size={14} />,
          items: [
            {
              id: "sortable.story.section.host",
              label: "",
              control: (
                <SortableTreeItems
                  items={items}
                  onReorder={(oldIndex, newIndex) =>
                    setItems((prev) => {
                      const next = [...prev];
                      const [moved] = next.splice(oldIndex, 1);
                      next.splice(newIndex, 0, moved!);
                      return next;
                    })
                  }
                >
                  {(item) => <TreeItem key={item.id} id={item.id} label={item.label} icon={<File size={12} />} sortable sortableId={item.id} dragRoles={["sort"]} />}
                </SortableTreeItems>
              ),
            },
          ],
        },
      ]}
    />
  );
}

export const Default: Story = {
  args: { items: [], onReorder: () => {} },
  render: () => (
    <div className="w-72 border ui-surface p-single" data-level="panel">
      <SortableRowsDemo
        initialRows={[
          { id: "row-1", label: "Capsule J" },
          { id: "row-2", label: "Capsule L" },
          { id: "row-3", label: "Capsule P" },
        ]}
      />
    </div>
  ),
};

export const SingleItem: Story = {
  name: "Single item (drag handle still renders)",
  args: { items: [], onReorder: () => {} },
  render: () => (
    <div className="w-72 border ui-surface p-single" data-level="panel">
      <SortableRowsDemo initialRows={[{ id: "row-only", label: "Only Row" }]} />
    </div>
  ),
};
// #endregion 🌳️SortableTreeItems
