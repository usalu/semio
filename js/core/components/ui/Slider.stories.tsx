// #region Header

// Slider.stories.tsx

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
import { useState } from "react";
import { Slider } from "./Slider";

const meta = {
  title: "Elements/Slider",
  component: Slider,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Slider>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Variants: Story = {
  render: () => (
    <div className="flex flex-col gap-6 w-96">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground">Default</p>
        {(() => {
          const [value, setValue] = useState([50]);
          return <Slider value={value} onValueChange={setValue} />;
        })()}
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground">With Label</p>
        {(() => {
          const [value, setValue] = useState([50]);
          return <Slider label="Volume" value={value} onValueChange={setValue} />;
        })()}
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground">With Step</p>
        {(() => {
          const [value, setValue] = useState([50]);
          return <Slider label="Scale" value={value} onValueChange={setValue} step={10} />;
        })()}
      </div>
    </div>
  ),
};

export const Basic: Story = {
  render: () => {
    const [value, setValue] = useState([50]);
    return <Slider value={value} onValueChange={setValue} className="w-96" />;
  },
};

export const WithLabel: Story = {
  render: () => {
    const [value, setValue] = useState([50]);
    return <Slider label="Volume" value={value} onValueChange={setValue} className="w-96" />;
  },
};

export const CustomRange: Story = {
  render: () => {
    const [value, setValue] = useState([25]);
    return <Slider label="Temperature" value={value} onValueChange={setValue} min={0} max={100} className="w-96" />;
  },
};

export const WithStep: Story = {
  render: () => {
    const [value, setValue] = useState([50]);
    return <Slider label="Brightness" value={value} onValueChange={setValue} min={0} max={100} step={10} className="w-96" />;
  },
};

export const Disabled: Story = {
  render: () => {
    const [value, setValue] = useState([50]);
    return <Slider label="Disabled" value={value} onValueChange={setValue} disabled className="w-96" />;
  },
};

export const MultipleValues: Story = {
  render: () => {
    const [value, setValue] = useState([25, 75]);
    return <Slider label="Range" value={value} onValueChange={setValue} className="w-96" />;
  },
};

export const SmallRange: Story = {
  render: () => {
    const [value, setValue] = useState([5]);
    return <Slider label="Scale" value={value} onValueChange={setValue} min={1} max={10} step={0.5} className="w-96" />;
  },
};

export const LargeRange: Story = {
  render: () => {
    const [value, setValue] = useState([500]);
    return <Slider label="Price" value={value} onValueChange={setValue} min={0} max={1000} step={50} className="w-96" />;
  },
};

export const Vertical: Story = {
  render: () => {
    const [value, setValue] = useState([50]);
    return (
      <div className="h-64">
        <Slider label="Vertical" value={value} onValueChange={setValue} orientation="vertical" />
      </div>
    );
  },
};
