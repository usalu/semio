// #region Header

// Dialog.stories.tsx

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
import { Button } from "./Button";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./Dialog";
import { Input } from "./Input";

const meta = {
  title: "Elements/Dialog",
  component: Dialog,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Dialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Variants: Story = {
  render: () => (
    <div className="flex gap-4">
      <Dialog>
        <DialogTrigger asChild>
          <Button>Basic</Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>New Design</DialogTitle>
            <DialogDescription>Create a new design in the current kit.</DialogDescription>
          </DialogHeader>
          <div className="py-4">Enter design parameters and properties.</div>
        </DialogContent>
      </Dialog>
      <Dialog>
        <DialogTrigger asChild>
          <Button variant="outline">With Footer</Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Edit Type</DialogTitle>
            <DialogDescription>Modify the type properties and representations.</DialogDescription>
          </DialogHeader>
          <div className="py-4">Update type configuration.</div>
          <DialogFooter>
            <Button variant="outline">Cancel</Button>
            <Button>Save Changes</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  ),
};

export const Basic: Story = {
  render: () => (
    <Dialog>
      <DialogTrigger asChild>
        <Button>Create Design</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New Design</DialogTitle>
          <DialogDescription>Create a new design in the current kit.</DialogDescription>
        </DialogHeader>
        <div className="py-4">Enter design parameters and properties.</div>
      </DialogContent>
    </Dialog>
  ),
};

export const WithForm: Story = {
  render: () => (
    <Dialog>
      <DialogTrigger asChild>
        <Button>Edit Type</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Edit Type Properties</DialogTitle>
          <DialogDescription>Modify the type properties and metadata. Changes will apply to all instances.</DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <Input label="Type Name" defaultValue="Capsule J" />
          <Input label="Variant" defaultValue="Standard" />
        </div>
        <DialogFooter>
          <Button variant="outline">Cancel</Button>
          <Button>Save Changes</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  ),
};

export const Controlled: Story = {
  render: () => {
    const [open, setOpen] = useState(false);

    return (
      <>
        <Button onClick={() => setOpen(true)}>Open Controlled Dialog</Button>
        <Dialog open={open} onOpenChange={setOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Controlled Dialog</DialogTitle>
              <DialogDescription>This dialog's state is controlled externally.</DialogDescription>
            </DialogHeader>
            <div className="py-4">You can control when this dialog opens and closes programmatically.</div>
            <DialogFooter>
              <Button onClick={() => setOpen(false)}>Close</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </>
    );
  },
};

export const WithoutCloseButton: Story = {
  render: () => (
    <Dialog>
      <DialogTrigger asChild>
        <Button>Open</Button>
      </DialogTrigger>
      <DialogContent showCloseButton={false}>
        <DialogHeader>
          <DialogTitle>No Close Button</DialogTitle>
          <DialogDescription>This dialog doesn't show the X close button.</DialogDescription>
        </DialogHeader>
        <div className="py-4">You must click a button to close this dialog.</div>
        <DialogFooter>
          <Button>Confirm</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  ),
};

export const Confirmation: Story = {
  render: () => (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="destructive">Delete Account</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Are you absolutely sure?</DialogTitle>
          <DialogDescription>This action cannot be undone. This will permanently delete your account and remove your data from our servers.</DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button variant="outline">Cancel</Button>
          <Button variant="destructive">Delete Account</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  ),
};

export const LongContent: Story = {
  render: () => (
    <Dialog>
      <DialogTrigger asChild>
        <Button>View Design Details</Button>
      </DialogTrigger>
      <DialogContent className="max-h-[80vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Nakagin Capsule Tower Documentation</DialogTitle>
          <DialogDescription>Complete design specifications and architectural details.</DialogDescription>
        </DialogHeader>
        <div className="space-y-4 py-4 text-sm">
          <p>The Nakagin Capsule Tower is a mixed-use residential and office tower designed by architect Kisho Kurokawa and located in Shimbashi, Tokyo, Japan.</p>
          <p>Completed in 1972, the building is a rare remaining example of Japanese Metabolism, a post-war architectural movement that fused ideas about architectural megastructures with those of organic biological growth.</p>
          <p>The building was made of prefabricated capsules which could be plugged in to the concrete towers. Each capsule measures 2.5 m × 4.0 m × 2.5 m and was designed to be replaceable.</p>
          <p>The capsules were intended to be replaced every 25 years, but this never happened. The building became a symbol of the Metabolist movement and its vision of sustainable, adaptable architecture.</p>
          <p>The tower consists of two interconnected concrete cores with 140 prefabricated capsules inserted into the cores. The capsules can be individually removed and replaced without affecting the integrity of the building.</p>
          <p>Each capsule features a circular window, built-in storage, a bathroom, and was originally equipped with a bed, desk, and reel-to-reel tape deck. The modular design represented a radical approach to urban living.</p>
        </div>
        <DialogFooter>
          <Button>Close</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  ),
};
