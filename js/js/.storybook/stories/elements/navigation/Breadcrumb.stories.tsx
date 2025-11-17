// #region Header

// Breadcrumb.stories.tsx

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
import { Home } from "lucide-react";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from "../../../../sketchpad/elements";

// #region Breadcrumb
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
  render: () => (
    <Breadcrumb>
      <BreadcrumbList>
        <BreadcrumbItem
          id="breadcrumb-link-home"
          items={[
            { label: "Temporary Kits", href: "/?kind=temporary" },
            { label: "Local Kits", href: "/?kind=local" },
            { label: "Remote Kits", href: "/?kind=remote" },
          ]}
          onNavigate={(href) => console.log("Navigate to:", href)}
        >
          <BreadcrumbLink href="/">
            <Home className="size-tiny" />
            Kits
          </BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbItem>
          <BreadcrumbLink href="/metabolism">Metabolism</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbItem
          items={[
            { label: "Types", href: "/metabolism/types" },
            { label: "Designs", href: "/metabolism/designs" },
            { label: "Qualities", href: "/metabolism/qualities" },
          ]}
          onNavigate={(href) => console.log("Navigate to:", href)}
        >
          <BreadcrumbLink href="/metabolism/types">Types</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbItem>
          <BreadcrumbPage>Capsule J</BreadcrumbPage>
        </BreadcrumbItem>
      </BreadcrumbList>
    </Breadcrumb>
  ),
};

// #endregion Breadcrumb
