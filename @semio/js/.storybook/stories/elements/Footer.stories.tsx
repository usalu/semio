// #region 🔖Header

// js/semio/.storybook/stories/elements/Footer.stories.tsx

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
import { CheckCircle2 } from "lucide-react";
import { Footer, FooterItem, Level, LevelProvider, getLevelBgClass } from "../../../sketchpad/elements";

// #region 🔖Footer
const meta = {
  title: "Elements/Footer",
  component: Footer,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Footer>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    items: [
      {
        id: "success",
        content: (
          <div className="flex items-center gap-unit">
            <CheckCircle2 size={14} className="text-green-500" />
            <span>Success</span>
          </div>
        ),
        order: 0,
      },
      { id: "status", content: "Ready", order: 1 },
      { id: "cursor", content: "Ln 1, Col 1", order: 2 },
      { id: "encoding", content: "UTF-8", order: 3 },
      { id: "language", content: "TypeScript", order: 4 },
    ],
    isVisible: true,
  },
};

const defaultItems: FooterItem[] = [
  {
    id: "success",
    content: (
      <div className="flex items-center gap-unit">
        <CheckCircle2 size={14} className="text-green-500" />
        <span>Success</span>
      </div>
    ),
    order: 0,
  },
  { id: "status", content: "Ready", order: 1 },
  { id: "cursor", content: "Ln 1, Col 1", order: 2 },
  { id: "encoding", content: "UTF-8", order: 3 },
  { id: "language", content: "TypeScript", order: 4 },
];

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <Footer items={defaultItems} isVisible />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  render: createLevelRender("base"),
};

export const Window: Story = {
  render: createLevelRender("window"),
};

export const Panel: Story = {
  render: createLevelRender("panel"),
};

export const Overlay: Story = {
  render: createLevelRender("overlay"),
};

export const Temporary: Story = {
  render: createLevelRender("temporary"),
};

// #endregion 🔖Footer
