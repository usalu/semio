// #region Header

// Action.stories.tsx

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
import { Check, Plus, Trash2 } from "lucide-react";
import { Action } from "../../../../sketchpad/elements";

// #region Action
const meta = {
  title: "Elements/Input/Action",
  component: Action,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  argTypes: {
    variant: {
      control: "select",
      options: ["default", "primary", "destructive"],
    },
    level: {
      control: "select",
      options: ["base", "panel", "temporary"],
    },
    disabled: {
      control: "boolean",
    },
  },
} satisfies Meta<typeof Action>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "action-default",
    variant: "primary",
    level: "base",
    disabled: false,
    loading: false,
    icon: <Plus />,
  },
};

export const Variants: Story = {
  render: () => (
    <div className="flex items-center gap-double">
      <Action variant="default" icon={<Plus />} />
      <Action variant="primary" icon={<Check />} />
      <Action variant="destructive" icon={<Trash2 />} />
    </div>
  ),
};

// #endregion Action
