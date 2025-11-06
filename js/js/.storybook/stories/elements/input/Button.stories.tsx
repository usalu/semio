// #region Header

// Button.stories.tsx

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
import { Plus } from "lucide-react";
import { Button } from "../../../../sketchpad/elements";

// #region Button
const meta = {
  title: "Elements/Input/Button",
  component: Button,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Button>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => <Button id="button-default" variant="primary" level="base" icon={<Plus />} />,
};

export const Variants: Story = {
  render: () => (
    <div className="flex flex-wrap gap-4">
      <Button variant="default" icon={<Plus />} />
      <Button variant="primary" icon={<Plus />} />
      <Button variant="secondary" icon={<Plus />} />
      <Button variant="destructive" icon={<Plus />} />
      <Button variant="ghost" icon={<Plus />} />
      <Button variant="link" icon={<Plus />} />
    </div>
  ),
};

// #endregion Button
