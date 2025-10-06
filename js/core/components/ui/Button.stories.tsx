// #region Header

// Button.stories.tsx

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
import { ChevronRight, Download, Mail, Plus, Settings, Trash2 } from "lucide-react";
import { Button } from "./Button";

const meta = {
  title: "Elements/Button",
  component: Button,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Button>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <Button size="default" variant="default">
      <Plus />
      Add Capsule to Design
    </Button>
  ),
};

export const Variants: Story = {
  render: () => (
    <div className="flex flex-wrap gap-4">
      <Button variant="default">Default</Button>
      <Button variant="destructive">Destructive</Button>
      <Button variant="outline">Outline</Button>
      <Button variant="secondary">Secondary</Button>
      <Button variant="ghost">Ghost</Button>
      <Button variant="link">Link</Button>
    </div>
  ),
};

export const Sizes: Story = {
  render: () => (
    <div className="flex flex-wrap items-center gap-4">
      <Button size="sm">Add Port</Button>
      <Button size="default">Create Type</Button>
      <Button size="lg">Export Design</Button>
    </div>
  ),
};

export const WithIcons: Story = {
  render: () => (
    <div className="flex flex-wrap gap-4">
      <Button>
        <Plus />
        Add Capsule
      </Button>
      <Button>
        Export Kit
        <Download />
      </Button>
      <Button>
        <Settings />
        Kit Settings
        <ChevronRight />
      </Button>
    </div>
  ),
};

export const IconOnly: Story = {
  render: () => (
    <div className="flex flex-wrap gap-4">
      <Button size="icon" variant="default">
        <Plus />
      </Button>
      <Button size="icon" variant="outline">
        <Settings />
      </Button>
      <Button size="icon" variant="ghost">
        <Mail />
      </Button>
      <Button size="icon" variant="destructive">
        <Trash2 />
      </Button>
    </div>
  ),
};

export const Disabled: Story = {
  render: () => (
    <div className="flex flex-wrap gap-4">
      <Button disabled>Add Connection</Button>
      <Button variant="outline" disabled>
        Edit Piece
      </Button>
      <Button variant="secondary" disabled>
        Save Design
      </Button>
    </div>
  ),
};

export const Loading: Story = {
  render: () => (
    <div className="flex flex-wrap gap-4">
      <Button disabled>
        <svg className="animate-spin size-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
          <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        Loading Kit
      </Button>
      <Button variant="outline" disabled>
        <svg className="animate-spin size-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
          <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        Exporting Design
      </Button>
    </div>
  ),
};

export const AsChild: Story = {
  render: () => (
    <Button asChild>
      <a href="https://semio.tech" target="_blank" rel="noopener noreferrer">
        Visit Semio
      </a>
    </Button>
  ),
};

export const FullWidth: Story = {
  render: () => (
    <div className="w-96">
      <Button className="w-full">Create New Design</Button>
    </div>
  ),
};

export const WithBadge: Story = {
  render: () => (
    <div className="flex gap-4">
      <Button className="relative">
        Errors
        <span className="absolute -top-1 -right-1 flex size-5 items-center justify-center rounded-full bg-red-500 text-white text-xs">3</span>
      </Button>
      <Button variant="outline" className="relative">
        Pieces
        <span className="absolute -top-1 -right-1 flex size-5 items-center justify-center rounded-full bg-blue-500 text-white text-xs">24</span>
      </Button>
    </div>
  ),
};
