// #region 🔖Header

// js/semio/.storybook/stories/elements/navigation/Breadcrumb.stories.tsx

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
import { Home } from "lucide-react";
import { Breadcrumb, BreadcrumbItemData, Level, LevelProvider, getLevelBgClass } from "../../../../sketchpad/elements";

// #region 🔖Breadcrumb
const meta = {
  title: "Elements/Navigation/Breadcrumb",
  component: Breadcrumb,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Breadcrumb>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    items: [
      {
        id: "breadcrumb-link-home",
        content: (
          <span className="flex items-center gap-unit">
            <Home className="size-tiny" />
            Kits
          </span>
        ),
        options: [
          { label: "Temporary Kits", href: "/?kind=temporary" },
          { label: "Local Kits", href: "/?kind=local" },
          { label: "Remote Kits", href: "/?kind=remote" },
        ],
        onNavigate: (href) => console.log("Navigate to:", href),
      },
      { id: "breadcrumb-metabolism", content: "Metabolism" },
      {
        id: "breadcrumb-kind",
        content: "Types",
        options: [
          { label: "Types", href: "/metabolism/types" },
          { label: "Designs", href: "/metabolism/designs" },
          { label: "Qualities", href: "/metabolism/qualities" },
        ],
        onNavigate: (href) => console.log("Navigate to:", href),
      },
      { id: "breadcrumb-page", content: "Capsule J" },
    ] satisfies BreadcrumbItemData[],
  },
  render: (args) => <Breadcrumb {...args} />,
};

const defaultItems: BreadcrumbItemData[] = [
  {
    id: "breadcrumb-link-home",
    content: (
      <span className="flex items-center gap-unit">
        <Home className="size-tiny" />
        Kits
      </span>
    ),
    options: [
      { label: "Temporary Kits", href: "/?kind=temporary" },
      { label: "Local Kits", href: "/?kind=local" },
      { label: "Remote Kits", href: "/?kind=remote" },
    ],
    onNavigate: (href) => console.log("Navigate to:", href),
  },
  { id: "breadcrumb-metabolism", content: "Metabolism" },
  {
    id: "breadcrumb-kind",
    content: "Types",
    options: [
      { label: "Types", href: "/metabolism/types" },
      { label: "Designs", href: "/metabolism/designs" },
      { label: "Qualities", href: "/metabolism/qualities" },
    ],
    onNavigate: (href) => console.log("Navigate to:", href),
  },
  { id: "breadcrumb-page", content: "Capsule J" },
];

const createLevelRender = (level: Level) => () => (
  <LevelProvider level={level}>
    <div className={`p-4 ${getLevelBgClass(level)}`}>
      <Breadcrumb items={defaultItems} />
    </div>
  </LevelProvider>
);

export const Base: Story = {
  args: { items: defaultItems },
  render: createLevelRender("base"),
};

export const Window: Story = {
  args: { items: defaultItems },
  render: createLevelRender("window"),
};

export const Panel: Story = {
  args: { items: defaultItems },
  render: createLevelRender("panel"),
};

export const Overlay: Story = {
  args: { items: defaultItems },
  render: createLevelRender("overlay"),
};

export const Temporary: Story = {
  args: { items: defaultItems },
  render: createLevelRender("temporary"),
};

// #endregion 🔖Breadcrumb
