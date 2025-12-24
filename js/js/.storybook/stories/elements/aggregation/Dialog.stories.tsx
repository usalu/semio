// #region Header

// js/js/.storybook/stories/elements/aggregation/Dialog.stories.tsx

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

// #endregion Header

import type { Meta, StoryObj } from "@storybook/react";
import { useState } from "react";
import { Button, Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger, Level, LevelProvider, getLevelBgClass } from "../../../../sketchpad/elements";

// #region Dialog
const meta = {
  title: "Elements/Aggregation/Dialog",
  component: Dialog,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Dialog>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogTrigger asChild>
          <Button id="dialog-trigger-default">Add Capsule to Design</Button>
        </DialogTrigger>
        <DialogContent showCloseButton className="max-w-lg">
          <DialogHeader>
            <DialogTitle>Add Capsule Instance</DialogTitle>
            <DialogDescription>Configure the new capsule piece and its placement in the design.</DialogDescription>
          </DialogHeader>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <label className="text-sm font-medium">Capsule Type</label>
              <select className="w-full p-double border">
                <option>Capsule J (Standard)</option>
                <option>Capsule K (Corner)</option>
              </select>
            </div>
            <div className="space-y-2">
              <label className="text-sm font-medium">Quantity</label>
              <input type="number" defaultValue={1} min={1} className="w-full p-double border rounded" />
            </div>
          </div>
          <DialogFooter>
            <Button variant="default" onClick={() => setOpen(false)}>
              Cancel
            </Button>
            <Button onClick={() => setOpen(false)}>Add to Design</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    );
  },
};

const DialogDemo: React.FC<{ level: Level }> = ({ level }) => {
  const [open, setOpen] = useState(false);
  return (
    <LevelProvider level={level}>
      <div className={`p-4 ${getLevelBgClass(level)}`}>
        <Dialog open={open} onOpenChange={setOpen}>
          <DialogTrigger asChild>
            <Button id={`dialog-trigger-${level}`}>Add Capsule to Design</Button>
          </DialogTrigger>
          <DialogContent showCloseButton className="max-w-lg">
            <DialogHeader>
              <DialogTitle>Add Capsule Instance</DialogTitle>
              <DialogDescription>Configure the new capsule piece and its placement in the design.</DialogDescription>
            </DialogHeader>
            <div className="space-y-4 py-4">
              <div className="space-y-2">
                <label className="text-sm font-medium">Capsule Type</label>
                <select className="w-full p-double border">
                  <option>Capsule J (Standard)</option>
                  <option>Capsule K (Corner)</option>
                </select>
              </div>
              <div className="space-y-2">
                <label className="text-sm font-medium">Quantity</label>
                <input type="number" defaultValue={1} min={1} className="w-full p-double border rounded" />
              </div>
            </div>
            <DialogFooter>
              <Button variant="default" onClick={() => setOpen(false)}>
                Cancel
              </Button>
              <Button onClick={() => setOpen(false)}>Add to Design</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>
    </LevelProvider>
  );
};

export const Base: Story = {
  render: () => <DialogDemo level="base" />,
};

export const Window: Story = {
  render: () => <DialogDemo level="window" />,
};

export const Panel: Story = {
  render: () => <DialogDemo level="panel" />,
};

export const Overlay: Story = {
  render: () => <DialogDemo level="overlay" />,
};

export const Temporary: Story = {
  render: () => <DialogDemo level="temporary" />,
};

// #endregion Dialog
