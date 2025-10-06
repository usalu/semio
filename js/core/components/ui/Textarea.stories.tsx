// #region Header

// Textarea.stories.tsx

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
import { Textarea } from "./Textarea";

const meta = {
  title: "Elements/Textarea",
  component: Textarea,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Textarea>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Variants: Story = {
  render: () => (
    <div className="flex flex-col gap-4 w-96">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground">Default</p>
        <Textarea placeholder="Enter design description..." />
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground">With Label</p>
        <Textarea label="Type Description" placeholder="Describe the type..." />
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground">Disabled</p>
        <Textarea placeholder="Cannot edit" disabled />
      </div>
    </div>
  ),
};

export const Basic: Story = {
  args: {
    placeholder: "Enter design description...",
    className: "w-96",
  },
};

export const WithLabel: Story = {
  args: {
    label: "Type Description",
    placeholder: "Describe the type...",
    className: "w-96",
  },
};

export const WithValue: Story = {
  args: {
    label: "Design Notes",
    defaultValue: "The Nakagin Capsule Tower is a mixed-use residential and office tower designed by architect Kisho Kurokawa.",
    className: "w-96",
  },
};

export const Disabled: Story = {
  args: {
    label: "Locked Notes",
    placeholder: "These notes are locked",
    disabled: true,
    className: "w-96",
  },
};

export const Invalid: Story = {
  args: {
    label: "Port Description",
    defaultValue: "Missing",
    "aria-invalid": true,
    className: "w-96",
  },
};

export const WithRows: Story = {
  args: {
    label: "Kit Documentation",
    placeholder: "Enter documentation...",
    rows: 8,
    className: "w-96",
  },
};

export const AutoHeight: Story = {
  render: () => <Textarea label="Connection Description" placeholder="Describe the connection parameters..." className="w-96" defaultValue="This connection links two capsule pieces with a gap of 10mm, shift of 5mm, and rotation of 45 degrees." />,
};

export const MaxLength: Story = {
  args: {
    label: "Short Message",
    placeholder: "Max 100 characters...",
    maxLength: 100,
    className: "w-96",
  },
};

export const MultipleTextareas: Story = {
  render: () => (
    <div className="w-96 space-y-4">
      <Textarea label="Title" placeholder="Enter title..." />
      <Textarea label="Description" placeholder="Enter description..." rows={4} />
      <Textarea label="Notes" placeholder="Additional notes..." />
    </div>
  ),
};
