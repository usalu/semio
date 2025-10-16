// #region Header

// Action.stories.tsx

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
import { Check, ChevronDown, ChevronRight, Copy, Download, Edit, Eye, EyeOff, Plus, RefreshCw, Settings, Trash2, Upload, X } from "lucide-react";
import { Action } from "./Action";

const meta = {
  title: "Elements/Input/Action",
  component: Action,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  argTypes: {
    variant: {
      control: "select",
      options: ["default", "primary", "destructive"],
    },
    level: {
      control: "select",
      options: ["base", "panel", "temporary"],
    },
    disabled: {
      control: "boolean",
    },
  },
} satisfies Meta<typeof Action>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    children: <Plus />,
    tooltip: "Add item",
  },
};

export const Variants: Story = {
  render: () => (
    <div className="flex items-center gap-2">
      <Action variant="default" tooltip="Default">
        <Plus />
      </Action>
      <Action variant="primary" tooltip="Primary">
        <Check />
      </Action>
      <Action variant="destructive" tooltip="Delete">
        <Trash2 />
      </Action>
    </div>
  ),
};

export const Levels: Story = {
  render: () => (
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-2 p-4 bg-base">
        <span className="text-xs">Base Level:</span>
        <Action level="base" tooltip="Add">
          <Plus />
        </Action>
        <Action level="base" tooltip="Edit">
          <Edit />
        </Action>
        <Action level="base" tooltip="Delete">
          <Trash2 />
        </Action>
      </div>
      <div className="flex items-center gap-2 p-4 bg-panel">
        <span className="text-xs">Panel Level:</span>
        <Action level="panel" tooltip="Add">
          <Plus />
        </Action>
        <Action level="panel" tooltip="Edit">
          <Edit />
        </Action>
        <Action level="panel" tooltip="Delete">
          <Trash2 />
        </Action>
      </div>
      <div className="flex items-center gap-2 p-4 bg-temporary">
        <span className="text-xs">Temporary Level:</span>
        <Action level="temporary" tooltip="Add">
          <Plus />
        </Action>
        <Action level="temporary" tooltip="Edit">
          <Edit />
        </Action>
        <Action level="temporary" tooltip="Delete">
          <Trash2 />
        </Action>
      </div>
    </div>
  ),
};

export const CommonIcons: Story = {
  render: () => (
    <div className="flex flex-wrap gap-2">
      <Action tooltip="Add">
        <Plus />
      </Action>
      <Action tooltip="Remove">
        <X />
      </Action>
      <Action tooltip="Edit">
        <Edit />
      </Action>
      <Action tooltip="Delete" variant="destructive">
        <Trash2 />
      </Action>
      <Action tooltip="Copy">
        <Copy />
      </Action>
      <Action tooltip="Download">
        <Download />
      </Action>
      <Action tooltip="Upload">
        <Upload />
      </Action>
      <Action tooltip="Refresh">
        <RefreshCw />
      </Action>
      <Action tooltip="Settings">
        <Settings />
      </Action>
      <Action tooltip="Confirm" variant="primary">
        <Check />
      </Action>
      <Action tooltip="Show">
        <Eye />
      </Action>
      <Action tooltip="Hide">
        <EyeOff />
      </Action>
      <Action tooltip="Expand">
        <ChevronRight />
      </Action>
      <Action tooltip="Collapse">
        <ChevronDown />
      </Action>
    </div>
  ),
};

export const WithHotkey: Story = {
  render: () => (
    <div className="flex items-center gap-2">
      <Action tooltip="Add" hotkey="Ctrl+N">
        <Plus />
      </Action>
      <Action tooltip="Delete" hotkey="Del" variant="destructive">
        <Trash2 />
      </Action>
      <Action tooltip="Refresh" hotkey="F5">
        <RefreshCw />
      </Action>
    </div>
  ),
};

export const Disabled: Story = {
  render: () => (
    <div className="flex items-center gap-2">
      <Action tooltip="Add" disabled>
        <Plus />
      </Action>
      <Action tooltip="Edit" disabled>
        <Edit />
      </Action>
      <Action tooltip="Delete" variant="destructive" disabled>
        <Trash2 />
      </Action>
    </div>
  ),
};

export const InTreeContext: Story = {
  render: () => (
    <div className="border p-2 space-y-1">
      <div className="flex items-center gap-1 py-1 px-2 hover:bg-hover-base">
        <ChevronRight className="size-3.5" />
        <span className="flex-1 text-sm">Tree Item</span>
        <Action tooltip="Add child">
          <Plus />
        </Action>
        <Action tooltip="Edit">
          <Edit />
        </Action>
        <Action tooltip="Delete" variant="destructive">
          <Trash2 />
        </Action>
      </div>
      <div className="flex items-center gap-1 py-1 px-2 hover:bg-hover-base">
        <ChevronDown className="size-3.5" />
        <span className="flex-1 text-sm">Expanded Item</span>
        <Action tooltip="Add child">
          <Plus />
        </Action>
        <Action tooltip="Edit">
          <Edit />
        </Action>
        <Action tooltip="Delete" variant="destructive">
          <Trash2 />
        </Action>
      </div>
    </div>
  ),
};

export const InDropdown: Story = {
  render: () => (
    <div className="border p-2">
      <div className="flex items-center gap-2">
        <span className="text-sm">Select:</span>
        <div className="flex items-center border">
          <span className="px-2 text-sm">Option 1</span>
          <Action tooltip="Dropdown">
            <ChevronDown />
          </Action>
        </div>
      </div>
    </div>
  ),
};

export const InToolbar: Story = {
  render: () => (
    <div className="border-b flex items-center gap-1 p-1">
      <Action tooltip="New">
        <Plus />
      </Action>
      <Action tooltip="Copy">
        <Copy />
      </Action>
      <Action tooltip="Download">
        <Download />
      </Action>
      <Action tooltip="Upload">
        <Upload />
      </Action>
      <div className="w-px h-6 bg-border mx-1" />
      <Action tooltip="Refresh">
        <RefreshCw />
      </Action>
      <Action tooltip="Settings">
        <Settings />
      </Action>
      <div className="flex-1" />
      <Action tooltip="Delete" variant="destructive">
        <Trash2 />
      </Action>
    </div>
  ),
};
