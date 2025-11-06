// #region Header

// Toggle.stories.tsx

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
import { Lock } from "lucide-react";
import { useState } from "react";
import { Toggle } from "../../../../sketchpad/elements";

// #region Toggle
const meta = {
  title: "Elements/Input/Toggle",
  component: Toggle,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Toggle>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [pressed, setPressed] = useState(true);
    return <Toggle id="toggle-default" type="default" pressed={pressed} onPressedChange={setPressed} i18nPressed="toggle.pressed" level="base" disabled={false} icon={<Lock />} />;
  },
};

// #endregion Toggle
