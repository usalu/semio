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
import { Bold, Italic, Lock, Underline } from "lucide-react";
import { useState } from "react";
import { Toggle } from "./Toggle";

const meta = {
  title: "Elements/Toggle",
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
    return (
      <Toggle pressed={pressed} onPressedChange={setPressed}>
        <Lock className="h-4 w-4 mr-2" />
        Lock Layer
      </Toggle>
    );
  },
};

export const Variants: Story = {
  render: () => (
    <div className="flex gap-8">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Default</p>
        {(() => {
          const [pressed, setPressed] = useState(false);
          return (
            <Toggle pressed={pressed} onPressedChange={setPressed}>
              <Bold />
            </Toggle>
          );
        })()}
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Outline</p>
        {(() => {
          const [pressed, setPressed] = useState(false);
          return (
            <Toggle variant="outline" pressed={pressed} onPressedChange={setPressed}>
              <Italic />
            </Toggle>
          );
        })()}
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">With Text</p>
        {(() => {
          const [pressed, setPressed] = useState(false);
          return (
            <Toggle pressed={pressed} onPressedChange={setPressed}>
              <Underline />
              Underline
            </Toggle>
          );
        })()}
      </div>
    </div>
  ),
};

export const Basic: Story = {
  render: () => {
    const [pressed, setPressed] = useState(false);
    return (
      <Toggle pressed={pressed} onPressedChange={setPressed}>
        <Bold />
      </Toggle>
    );
  },
};

export const WithText: Story = {
  render: () => {
    const [pressed, setPressed] = useState(false);
    return (
      <Toggle pressed={pressed} onPressedChange={setPressed}>
        <Bold />
        Bold
      </Toggle>
    );
  },
};

export const WithTooltip: Story = {
  render: () => {
    const [pressed, setPressed] = useState(false);
    return (
      <Toggle pressed={pressed} onPressedChange={setPressed} tooltip="Bold" hotkey="Ctrl+B">
        <Bold />
      </Toggle>
    );
  },
};

export const Outline: Story = {
  render: () => {
    const [pressed, setPressed] = useState(false);
    return (
      <Toggle variant="outline" pressed={pressed} onPressedChange={setPressed}>
        <Italic />
      </Toggle>
    );
  },
};

export const Sizes: Story = {
  render: () => {
    const [pressed, setPressed] = useState(false);
    return (
      <div className="flex items-center gap-4">
        <Toggle size="sm" pressed={pressed} onPressedChange={setPressed}>
          <Bold />
        </Toggle>
        <Toggle size="default" pressed={pressed} onPressedChange={setPressed}>
          <Bold />
        </Toggle>
        <Toggle size="lg" pressed={pressed} onPressedChange={setPressed}>
          <Bold />
        </Toggle>
      </div>
    );
  },
};

export const Disabled: Story = {
  args: {
    disabled: true,
    children: (
      <>
        <Bold />
      </>
    ),
  },
};

export const Multiple: Story = {
  render: () => {
    const [bold, setBold] = useState(false);
    const [italic, setItalic] = useState(false);
    const [underline, setUnderline] = useState(false);

    return (
      <div className="flex gap-2">
        <Toggle pressed={bold} onPressedChange={setBold} tooltip="Bold" hotkey="Ctrl+B">
          <Bold />
        </Toggle>
        <Toggle pressed={italic} onPressedChange={setItalic} tooltip="Italic" hotkey="Ctrl+I">
          <Italic />
        </Toggle>
        <Toggle pressed={underline} onPressedChange={setUnderline} tooltip="Underline" hotkey="Ctrl+U">
          <Underline />
        </Toggle>
      </div>
    );
  },
};
