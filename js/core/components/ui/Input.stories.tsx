// #region Header

// Input.stories.tsx

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
import { Input } from "./Input";

const meta = {
  title: "Elements/Input",
  component: Input,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Input>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Basic: Story = {
  args: {
    placeholder: "Enter text...",
    className: "w-96",
  },
};

export const WithLabel: Story = {
  args: {
    label: "Username",
    placeholder: "Enter username...",
    className: "w-96",
  },
};

export const Email: Story = {
  args: {
    type: "email",
    label: "Email",
    placeholder: "user@example.com",
    className: "w-96",
  },
};

export const Password: Story = {
  args: {
    type: "password",
    label: "Password",
    placeholder: "Enter password...",
    className: "w-96",
  },
};

export const Number: Story = {
  args: {
    type: "number",
    label: "Quantity",
    placeholder: "0",
    defaultValue: "5",
    className: "w-96",
  },
};

export const Disabled: Story = {
  args: {
    label: "Disabled Input",
    placeholder: "This is disabled",
    disabled: true,
    className: "w-96",
  },
};

export const WithValue: Story = {
  args: {
    label: "Project Name",
    defaultValue: "Semio Platform",
    className: "w-96",
  },
};

export const File: Story = {
  args: {
    type: "file",
    label: "Upload File",
    className: "w-96",
  },
};

export const Date: Story = {
  args: {
    type: "date",
    label: "Due Date",
    className: "w-96",
  },
};

export const Search: Story = {
  args: {
    type: "search",
    placeholder: "Search...",
    className: "w-96",
  },
};

export const Invalid: Story = {
  args: {
    label: "Email",
    type: "email",
    defaultValue: "invalid-email",
    "aria-invalid": true,
    className: "w-96",
  },
};

export const MultipleInputs: Story = {
  render: () => (
    <div className="w-96 space-y-4">
      <Input label="First Name" placeholder="John" />
      <Input label="Last Name" placeholder="Doe" />
      <Input label="Email" type="email" placeholder="john.doe@example.com" />
      <Input label="Phone" type="tel" placeholder="+1 (555) 000-0000" />
    </div>
  ),
};
