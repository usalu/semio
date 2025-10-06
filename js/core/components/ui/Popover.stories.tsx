// #region Header

// Popover.stories.tsx

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
import { Settings } from "lucide-react";
import { Button } from "./Button";
import { Input } from "./Input";
import { Popover, PopoverContent, PopoverTrigger } from "./Popover";

const meta = {
  title: "Elements/Popover",
  component: Popover,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Popover>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="outline">Connection Settings</Button>
      </PopoverTrigger>
      <PopoverContent className="w-80">
        <div className="space-y-4">
          <div className="space-y-2">
            <h4 className="font-medium text-sm">Connection Parameters</h4>
            <p className="text-sm text-muted-foreground">Configure spatial relationship between pieces.</p>
          </div>
          <div className="space-y-2">
            <label className="text-sm">Gap (mm)</label>
            <input type="number" defaultValue={0} className="w-full p-2 border rounded" />
          </div>
          <div className="space-y-2">
            <label className="text-sm">Rotation (°)</label>
            <input type="number" defaultValue={0} min={0} max={360} className="w-full p-2 border rounded" />
          </div>
        </div>
      </PopoverContent>
    </Popover>
  ),
};

export const Variants: Story = {
  render: () => (
    <div className="flex gap-4">
      <Popover>
        <PopoverTrigger asChild>
          <Button variant="outline">Basic</Button>
        </PopoverTrigger>
        <PopoverContent>
          <div className="space-y-2">
            <h4 className="font-medium text-sm">Dimensions</h4>
            <p className="text-sm text-muted-foreground">Set the dimensions for the layer.</p>
          </div>
        </PopoverContent>
      </Popover>
      <Popover>
        <PopoverTrigger asChild>
          <Button variant="outline">With Align</Button>
        </PopoverTrigger>
        <PopoverContent align="start">
          <div className="space-y-2">
            <h4 className="font-medium text-sm">Connection</h4>
            <p className="text-sm text-muted-foreground">Configure connection parameters.</p>
          </div>
        </PopoverContent>
      </Popover>
    </div>
  ),
};

export const Basic: Story = {
  render: () => (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="outline">Open Popover</Button>
      </PopoverTrigger>
      <PopoverContent>
        <div className="space-y-2">
          <h4 className="font-medium text-sm">Dimensions</h4>
          <p className="text-sm text-muted-foreground">Set the dimensions for the layer.</p>
        </div>
      </PopoverContent>
    </Popover>
  ),
};

export const WithForm: Story = {
  render: () => (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="outline">
          <Settings />
          Settings
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-80">
        <div className="space-y-4">
          <div className="space-y-2">
            <h4 className="font-medium text-sm">Dimensions</h4>
            <p className="text-sm text-muted-foreground">Set the dimensions for the layer.</p>
          </div>
          <div className="grid gap-2">
            <Input label="Width" defaultValue="100" type="number" />
            <Input label="Height" defaultValue="200" type="number" />
          </div>
        </div>
      </PopoverContent>
    </Popover>
  ),
};

export const DifferentSides: Story = {
  render: () => (
    <div className="flex gap-4">
      <Popover>
        <PopoverTrigger asChild>
          <Button variant="outline">Top</Button>
        </PopoverTrigger>
        <PopoverContent side="top">
          <p className="text-sm">Popover on top</p>
        </PopoverContent>
      </Popover>

      <Popover>
        <PopoverTrigger asChild>
          <Button variant="outline">Right</Button>
        </PopoverTrigger>
        <PopoverContent side="right">
          <p className="text-sm">Popover on right</p>
        </PopoverContent>
      </Popover>

      <Popover>
        <PopoverTrigger asChild>
          <Button variant="outline">Bottom</Button>
        </PopoverTrigger>
        <PopoverContent side="bottom">
          <p className="text-sm">Popover on bottom</p>
        </PopoverContent>
      </Popover>

      <Popover>
        <PopoverTrigger asChild>
          <Button variant="outline">Left</Button>
        </PopoverTrigger>
        <PopoverContent side="left">
          <p className="text-sm">Popover on left</p>
        </PopoverContent>
      </Popover>
    </div>
  ),
};

export const CustomWidth: Story = {
  render: () => (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="outline">Wide Popover</Button>
      </PopoverTrigger>
      <PopoverContent className="w-[500px]">
        <div className="space-y-2">
          <h4 className="font-medium">Extra Wide Content</h4>
          <p className="text-sm text-muted-foreground">This popover has a custom width of 500px to accommodate more content or complex layouts.</p>
        </div>
      </PopoverContent>
    </Popover>
  ),
};

export const WithActions: Story = {
  render: () => (
    <Popover>
      <PopoverTrigger asChild>
        <Button>Configure</Button>
      </PopoverTrigger>
      <PopoverContent className="w-80">
        <div className="space-y-4">
          <div>
            <h4 className="font-medium text-sm mb-2">Configuration</h4>
            <p className="text-sm text-muted-foreground">Adjust your settings below.</p>
          </div>
          <div className="space-y-2">
            <Input label="Name" placeholder="Enter name..." />
            <Input label="Value" type="number" placeholder="0" />
          </div>
          <div className="flex justify-end gap-2">
            <Button variant="outline" size="sm">
              Cancel
            </Button>
            <Button size="sm">Save</Button>
          </div>
        </div>
      </PopoverContent>
    </Popover>
  ),
};
