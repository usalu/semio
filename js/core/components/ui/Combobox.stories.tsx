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

const frameworks = [
  { value: "react", label: "React" },
  { value: "vue", label: "Vue" },
  { value: "angular", label: "Angular" },
  { value: "svelte", label: "Svelte" },
  { value: "solid", label: "Solid" },
];

export const Basic: Story = {
  render: () => {
    const [value, setValue] = useState("");
    return <Combobox options={frameworks} value={value} onValueChange={setValue} placeholder="Select framework..." className="w-96" />;
  },
};

export const WithLabel: Story = {
  render: () => {
    const [value, setValue] = useState("");
    return <Combobox options={frameworks} value={value} onValueChange={setValue} label="Framework" placeholder="Choose one..." className="w-96" />;
  },
};

export const PreSelected: Story = {
  render: () => {
    const [value, setValue] = useState("react");
    return <Combobox options={frameworks} value={value} onValueChange={setValue} label="Framework" className="w-96" />;
  },
};

export const AllowClear: Story = {
  render: () => {
    const [value, setValue] = useState("react");
    return <Combobox options={frameworks} value={value} onValueChange={setValue} label="Framework" allowClear={true} className="w-96" />;
  },
};

export const CustomEmptyMessage: Story = {
  render: () => {
    const [value, setValue] = useState("");
    return <Combobox options={frameworks} value={value} onValueChange={setValue} placeholder="Select..." emptyMessage="No frameworks match your search." className="w-96" />;
  },
};

export const ManyOptions: Story = {
  render: () => {
    const [value, setValue] = useState("");
    const countries = [
      { value: "us", label: "United States" },
      { value: "uk", label: "United Kingdom" },
      { value: "ca", label: "Canada" },
      { value: "au", label: "Australia" },
      { value: "de", label: "Germany" },
      { value: "fr", label: "France" },
      { value: "it", label: "Italy" },
      { value: "es", label: "Spain" },
      { value: "jp", label: "Japan" },
      { value: "cn", label: "China" },
      { value: "in", label: "India" },
      { value: "br", label: "Brazil" },
      { value: "mx", label: "Mexico" },
      { value: "kr", label: "South Korea" },
      { value: "nl", label: "Netherlands" },
    ];
    return <Combobox options={countries} value={value} onValueChange={setValue} label="Country" placeholder="Select country..." className="w-96" />;
  },
};
