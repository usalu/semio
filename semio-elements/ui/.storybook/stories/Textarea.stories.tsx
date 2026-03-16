// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/input/Textarea.stories.tsx

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
import { Level, LevelProvider, Textarea, getLevelBgClass } from "@semio-elements/ui";

// #region 🔖Textarea
const meta = {
  title: "semio-elements/Textarea",
  component: Textarea,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Textarea>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "textarea-default",
  placeholder: "Describe the design configuration, spatial relationships, and architectural intent...",
  placeholderId: "textarea.placeholder",
  defaultValue: "The Nakagin Capsule Tower features 140 prefabricated capsules attached to two concrete cores. Each capsule is a self-contained living unit with standardized connection points.",
  rows: 4,
  lazy: true,
  showLabel: true,
  disabled: false,
  "aria-invalid": false,
  className: "w-96",
};

export const Default: Story = {
  args: defaultArgs,
};

const createLevelRender = (level: Level, id: string) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <Textarea {...defaultArgs} id={id} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { ...defaultArgs, id: "textarea-base" },
  render: createLevelRender("base", "textarea-base"),
};

export const Window: Story = {
  args: { ...defaultArgs, id: "textarea-window" },
  render: createLevelRender("window", "textarea-window"),
};

export const Panel: Story = {
  args: { ...defaultArgs, id: "textarea-panel" },
  render: createLevelRender("panel", "textarea-panel"),
};

export const Overlay: Story = {
  args: { ...defaultArgs, id: "textarea-overlay" },
  render: createLevelRender("overlay", "textarea-overlay"),
};

export const Temporary: Story = {
  args: { ...defaultArgs, id: "textarea-temporary" },
  render: createLevelRender("temporary", "textarea-temporary"),
};

// #endregion 🔖Textarea
