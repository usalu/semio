// #region Header

// Combobox.stories.tsx

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
import { useState } from "react";
import Combobox from "./Combobox";

const meta = {
  title: "Elements/Combobox",
  component: Combobox,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Combobox>;

export default meta;
type Story = StoryObj<typeof meta>;

const types = [
  { value: "capsule", label: "Capsule" },
  { value: "base", label: "Base" },
  { value: "tambour", label: "Tambour" },
  { value: "capital", label: "Capital" },
  { value: "cluster", label: "Cluster" },
];

export const Basic: Story = {
  render: () => {
    const [value, setValue] = useState("");
    return <Combobox options={types} value={value} onValueChange={setValue} placeholder="Select type..." className="w-96" />;
  },
};

export const WithLabel: Story = {
  render: () => {
    const [value, setValue] = useState("");
    return <Combobox options={types} value={value} onValueChange={setValue} label="Type" placeholder="Choose one..." className="w-96" />;
  },
};

export const PreSelected: Story = {
  render: () => {
    const [value, setValue] = useState("capsule");
    return <Combobox options={types} value={value} onValueChange={setValue} label="Type" className="w-96" />;
  },
};

export const AllowClear: Story = {
  render: () => {
    const [value, setValue] = useState("capsule");
    return <Combobox options={types} value={value} onValueChange={setValue} label="Type" allowClear={true} className="w-96" />;
  },
};

export const CustomEmptyMessage: Story = {
  render: () => {
    const [value, setValue] = useState("");
    return <Combobox options={types} value={value} onValueChange={setValue} placeholder="Select..." emptyMessage="No types match your search." className="w-96" />;
  },
};

export const ManyOptions: Story = {
  render: () => {
    const [value, setValue] = useState("");
    const pieces = [
      { value: "capsule-j", label: "Capsule J" },
      { value: "capsule-l", label: "Capsule L" },
      { value: "capsule-p", label: "Capsule P" },
      { value: "capsule-q", label: "Capsule Q" },
      { value: "capsule-s", label: "Capsule S" },
      { value: "capsule-z", label: "Capsule Z" },
      { value: "base-blob", label: "Base Blob" },
      { value: "base-standard", label: "Base Standard" },
      { value: "tambour-cylindric", label: "Tambour Cylindric" },
      { value: "tambour-first", label: "Tambour First Storey" },
      { value: "tambour-last", label: "Tambour Last Storey" },
      { value: "capital-standard", label: "Capital Standard" },
      { value: "capital-cylindric", label: "Capital Cylindric" },
      { value: "cluster-small", label: "Cluster Small" },
      { value: "cluster-medium", label: "Cluster Medium" },
    ];
    return <Combobox options={pieces} value={value} onValueChange={setValue} label="Piece" placeholder="Select piece..." className="w-96" />;
  },
};
