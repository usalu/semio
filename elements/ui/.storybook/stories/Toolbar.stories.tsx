// #region 🔖Header

// .elements/ui/.storybook/stories/elements/Toolbar.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🔖Header

import { ToolbarDivider, ToolbarGroup, ToolbarItem, ToolbarZone } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";
import { Eye, EyeOff, Hand, Maximize2, MousePointer, Move, RotateCcw, RotateCw, ZoomIn, ZoomOut } from "lucide-react";

// #region 🔖Toolbar

const meta = {
  title: "semio-elements/Toolbar",
  component: ToolbarZone,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ToolbarZone>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    children: null,
  },
  render: () => (
    <ToolbarZone>
      <ToolbarGroup>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <MousePointer size={16} />
          </button>
        </ToolbarItem>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <Hand size={16} />
          </button>
        </ToolbarItem>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <Move size={16} />
          </button>
        </ToolbarItem>
      </ToolbarGroup>
      <ToolbarDivider />
      <ToolbarGroup>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <ZoomIn size={16} />
          </button>
        </ToolbarItem>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <ZoomOut size={16} />
          </button>
        </ToolbarItem>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <Maximize2 size={16} />
          </button>
        </ToolbarItem>
      </ToolbarGroup>
    </ToolbarZone>
  ),
};

export const WithUndoRedo: Story = {
  args: { children: null },
  render: () => (
    <ToolbarZone>
      <ToolbarGroup>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <RotateCcw size={16} />
          </button>
        </ToolbarItem>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <RotateCw size={16} />
          </button>
        </ToolbarItem>
      </ToolbarGroup>
      <ToolbarDivider />
      <ToolbarGroup>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <MousePointer size={16} />
          </button>
        </ToolbarItem>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <Hand size={16} />
          </button>
        </ToolbarItem>
      </ToolbarGroup>
      <ToolbarDivider />
      <ToolbarGroup>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <Eye size={16} />
          </button>
        </ToolbarItem>
        <ToolbarItem>
          <button className="p-1 hover:bg-hover-panel rounded">
            <EyeOff size={16} />
          </button>
        </ToolbarItem>
      </ToolbarGroup>
    </ToolbarZone>
  ),
};

export const MultipleZones: Story = {
  args: { children: null },
  render: () => (
    <div className="flex gap-4 items-center">
      <ToolbarZone>
        <ToolbarGroup>
          <ToolbarItem>
            <button className="p-1 hover:bg-hover-panel rounded">
              <MousePointer size={16} />
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button className="p-1 hover:bg-hover-panel rounded">
              <Hand size={16} />
            </button>
          </ToolbarItem>
        </ToolbarGroup>
      </ToolbarZone>
      <ToolbarZone>
        <ToolbarGroup>
          <ToolbarItem>
            <button className="p-1 hover:bg-hover-panel rounded">
              <ZoomIn size={16} />
            </button>
          </ToolbarItem>
          <ToolbarItem>
            <button className="p-1 hover:bg-hover-panel rounded">
              <ZoomOut size={16} />
            </button>
          </ToolbarItem>
        </ToolbarGroup>
      </ToolbarZone>
    </div>
  ),
};

// #endregion 🔖Toolbar
