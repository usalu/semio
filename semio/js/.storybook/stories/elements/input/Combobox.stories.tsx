// #region 🔖Header

// 🧪︎ semio/js/.storybook/stories/elements/input/Combobox.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion 🔖Header

import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Combobox, Level, LevelProvider, getLevelBgClass } from "../../../../sketchpad/elements";

// #region 🔖Combobox
const meta = {
  title: "Elements/Input/Combobox",
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

const defaultArgs = {
  id: "combobox-default",
  options: types,
  value: "capsule",
  onValueChange: () => { },
  placeholder: "Select type...",
  placeholderId: "combobox.placeholder",
  emptyMessage: "No types match your search.",
  allowClear: true,
  showLabel: true,
  className: "w-96",
};

export const Default: Story = {
  args: defaultArgs,
  render: (args) => {
    const [value, setValue] = useState(args.value);
    return <Combobox {...args} value={value} onValueChange={setValue} />;
  },
};

const ComboboxDemo = ({ id }: { id: string }) => {
  const [value, setValue] = useState("capsule");
  return <Combobox {...defaultArgs} id={id} value={value} onValueChange={setValue} />;
};

const createLevelRender = (level: Level, id: string) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <ComboboxDemo id={id} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { ...defaultArgs, id: "combobox-base" },
  render: createLevelRender("base", "combobox-base"),
};

export const Window: Story = {
  args: { ...defaultArgs, id: "combobox-window" },
  render: createLevelRender("window", "combobox-window"),
};

export const Panel: Story = {
  args: { ...defaultArgs, id: "combobox-panel" },
  render: createLevelRender("panel", "combobox-panel"),
};

export const Overlay: Story = {
  args: { ...defaultArgs, id: "combobox-overlay" },
  render: createLevelRender("overlay", "combobox-overlay"),
};

export const Temporary: Story = {
  args: { ...defaultArgs, id: "combobox-temporary" },
  render: createLevelRender("temporary", "combobox-temporary"),
};

// #endregion 🔖Combobox
