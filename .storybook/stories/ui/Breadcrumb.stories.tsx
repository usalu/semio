// #region 🧲Header

// 🥼︎ .storybook/stories/ui/Breadcrumb.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

import { Breadcrumb, BreadcrumbItemData, NotFound, PageNavigation } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { MemoryRouter } from "react-router";

// 📖#region 💡Breadcrumb
const AlertCircle = createIconComponent("alert-circle");
const Home = createIconComponent("home");

const meta = {
  title: "🖱️ui⚛️react/Breadcrumb",
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
  tags: ["some tag"],
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

// #endregion 💡Breadcrumb

// 🔷#region 🎍NotFound
export const NotFoundDefault: Story = {
  args: { items: defaultItems },
  render: () => (
    <MemoryRouter>
      <div className="h-64 w-96 border">
        <NotFound title="Kit Not Found" description="The kit you're looking for doesn't exist or has been removed." parentPath="/" parentLabel="Back to Kits" />
      </div>
    </MemoryRouter>
  ),
};

export const NotFoundMinimal: Story = {
  args: { items: defaultItems },
  render: () => (
    <MemoryRouter>
      <div className="h-48 w-80 border">
        <NotFound title="Type Not Found" icon={<AlertCircle className="size-huge" />} />
      </div>
    </MemoryRouter>
  ),
};
// #endregion 🎍NotFound

// 🔷#region 🪩PageNavigation
export const PageNavigationDefault: Story = {
  args: { items: defaultItems },
  render: () => (
    <MemoryRouter>
      <div className="w-[600px]">
        <PageNavigation prev={{ path: "getting-started", title: "Getting Started", section: "Basics" }} next={{ path: "tutorials/hello-compose", title: "Hello Compose", section: "Tutorials" }} />
      </div>
    </MemoryRouter>
  ),
};

export const PageNavigationNextOnly: Story = {
  args: { items: defaultItems },
  render: () => (
    <MemoryRouter>
      <div className="w-[600px]">
        <PageNavigation next={{ path: "tutorials", title: "Tutorials" }} />
      </div>
    </MemoryRouter>
  ),
};
// #endregion 🪩PageNavigation
