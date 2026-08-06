// #region 🧲️Header

// 🥼️ .storybook/stories/ui/🔽ActionDropdown.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import { ActionDropdown, type ActionDropdownOption } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
// #endregion 🔌️Adapters

// 🌩️#region 🌩️ActionDropdown
const projectionOptions: ActionDropdownOption[] = [
  { value: "camera", icon: "camera", label: "Perspective" },
  { value: "orthographic", icon: "square", label: "Orthographic" },
];

const meta = {
  title: "🖱️ui⚛️react/ActionDropdown",
  component: ActionDropdown,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ActionDropdown>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [value, setValue] = useState("camera");
    return <ActionDropdown id="action-dropdown.story.projection" options={projectionOptions} value={value} onValueChange={setValue} />;
  },
};

export const WithTransaction: Story = {
  name: "With start/finalize transaction hooks",
  render: () => {
    const [value, setValue] = useState("camera");
    const [log, setLog] = useState<string[]>([]);
    return (
      <div className="flex flex-col items-center gap-double">
        <ActionDropdown
          id="action-dropdown.story.transactional"
          options={projectionOptions}
          value={value}
          onValueChange={setValue}
          startTransaction={() => setLog((prev) => [...prev, "start"])}
          finalizeTransaction={() => setLog((prev) => [...prev, "finalize"])}
        />
        <pre className="text-xs text-muted-foreground">{log.join(" → ") || "(no transaction yet)"}</pre>
      </div>
    );
  },
};
// #endregion 🌩️ActionDropdown
