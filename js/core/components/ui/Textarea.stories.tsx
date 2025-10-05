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

export const Basic: Story = {
  args: {
    placeholder: "Enter text here...",
    className: "w-96",
  },
};

export const WithLabel: Story = {
  args: {
    label: "Description",
    placeholder: "Enter description...",
    className: "w-96",
  },
};

export const WithValue: Story = {
  args: {
    label: "Comments",
    defaultValue: "This is a pre-filled textarea with some initial content.",
    className: "w-96",
  },
};

export const Disabled: Story = {
  args: {
    label: "Disabled",
    placeholder: "This textarea is disabled",
    disabled: true,
    className: "w-96",
  },
};

export const Invalid: Story = {
  args: {
    label: "Bio",
    defaultValue: "Too short",
    "aria-invalid": true,
    className: "w-96",
  },
};

export const WithRows: Story = {
  args: {
    label: "Long Text",
    placeholder: "Enter long text...",
    rows: 8,
    className: "w-96",
  },
};

export const AutoHeight: Story = {
  render: () => (
    <Textarea label="Auto-sizing" placeholder="This textarea will grow with content..." className="w-96" defaultValue="Type more text and this will grow automatically based on the field-sizing-content CSS property." />
  ),
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
