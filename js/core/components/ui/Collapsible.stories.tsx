// #region Header

// Collapsible.stories.tsx

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
import { Box, ChevronDown } from "lucide-react";
import { useState } from "react";
import { Button } from "./Button";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "./Collapsible";

const meta = {
  title: "Elements/Collapsible",
  component: Collapsible,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Collapsible>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(true);
    return (
      <Collapsible open={isOpen} onOpenChange={setIsOpen} className="w-96 space-y-2">
        <div className="flex items-center justify-between space-x-4 pb-2">
          <div className="flex items-center gap-2">
            <Box className="h-4 w-4" />
            <h4 className="text-sm font-semibold">Capsule Variants (3)</h4>
          </div>
          <CollapsibleTrigger asChild>
            <Button variant="ghost" size="sm">
              <ChevronDown className={`h-4 w-4 transition-transform ${isOpen ? "rotate-180" : ""}`} />
              <span className="sr-only">Toggle</span>
            </Button>
          </CollapsibleTrigger>
        </div>
        <div className="border rounded-md px-4 py-2 text-sm flex items-center justify-between">
          <span>Capsule J</span>
          <span className="text-xs text-muted-foreground">2.5m × 4.0m</span>
        </div>
        <CollapsibleContent className="space-y-2">
          <div className="border rounded-md px-4 py-2 text-sm">Capsule L</div>
          <div className="border rounded-md px-4 py-2 text-sm">Capsule P</div>
        </CollapsibleContent>
      </Collapsible>
    );
  },
};

export const Variants: Story = {
  render: () => (
    <div className="flex flex-col gap-8 w-96">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Controlled</p>
        {(() => {
          const [isOpen, setIsOpen] = useState(false);
          return (
            <Collapsible open={isOpen} onOpenChange={setIsOpen} className="space-y-2">
              <div className="flex items-center justify-between space-x-4">
                <h4 className="text-sm font-semibold">Capsule Types</h4>
                <CollapsibleTrigger asChild>
                  <Button variant="ghost" size="sm">
                    <ChevronDown className={`transition-transform ${isOpen ? "rotate-180" : ""}`} />
                    <span className="sr-only">Toggle</span>
                  </Button>
                </CollapsibleTrigger>
              </div>
              <div className="border rounded-md px-4 py-2 text-sm">Capsule J</div>
              <CollapsibleContent className="space-y-2">
                <div className="border rounded-md px-4 py-2 text-sm">Capsule L</div>
                <div className="border rounded-md px-4 py-2 text-sm">Capsule P</div>
              </CollapsibleContent>
            </Collapsible>
          );
        })()}
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Uncontrolled</p>
        <Collapsible className="space-y-2">
          <div className="flex items-center justify-between space-x-4">
            <h4 className="text-sm font-semibold">Base Types</h4>
            <CollapsibleTrigger asChild>
              <Button variant="ghost" size="sm">
                <ChevronDown />
                <span className="sr-only">Toggle</span>
              </Button>
            </CollapsibleTrigger>
          </div>
          <div className="border rounded-md px-4 py-2 text-sm">Blob Base</div>
          <CollapsibleContent className="space-y-2">
            <div className="border rounded-md px-4 py-2 text-sm">Standard Base</div>
          </CollapsibleContent>
        </Collapsible>
      </div>
    </div>
  ),
};

export const DefaultOpen: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(true);
    return (
      <Collapsible open={isOpen} onOpenChange={setIsOpen} className="w-96 space-y-2">
        <div className="flex items-center justify-between space-x-4">
          <h4 className="text-sm font-semibold">Design Properties</h4>
          <CollapsibleTrigger asChild>
            <Button variant="ghost" size="sm">
              <ChevronDown className={`transition-transform ${isOpen ? "rotate-180" : ""}`} />
            </Button>
          </CollapsibleTrigger>
        </div>
        <CollapsibleContent className="space-y-2">
          <div className="border rounded-md px-4 py-2 text-sm">Piece Count: 24</div>
          <div className="border rounded-md px-4 py-2 text-sm">Connection Count: 48</div>
          <div className="border rounded-md px-4 py-2 text-sm">Total Volume: 3240m³</div>
        </CollapsibleContent>
      </Collapsible>
    );
  },
};

export const WithForm: Story = {
  render: () => {
    const [isOpen, setIsOpen] = useState(false);
    return (
      <Collapsible open={isOpen} onOpenChange={setIsOpen} className="w-96 space-y-2 border p-4 rounded-lg">
        <div className="flex items-center justify-between">
          <h4 className="text-sm font-semibold">Connection Parameters</h4>
          <CollapsibleTrigger asChild>
            <Button variant="outline" size="sm">
              {isOpen ? "Hide" : "Show"}
            </Button>
          </CollapsibleTrigger>
        </div>
        <CollapsibleContent className="space-y-4 pt-2">
          <div>
            <label className="text-sm font-medium">Gap (mm)</label>
            <input className="w-full border rounded px-3 py-2 text-sm mt-1" placeholder="10" />
          </div>
          <div>
            <label className="text-sm font-medium">Rotation (degrees)</label>
            <input type="number" className="w-full border rounded px-3 py-2 text-sm mt-1" placeholder="45" />
          </div>
        </CollapsibleContent>
      </Collapsible>
    );
  },
};

export const NestedCollapsibles: Story = {
  render: () => {
    const [isParentOpen, setIsParentOpen] = useState(false);
    const [isChild1Open, setIsChild1Open] = useState(false);
    const [isChild2Open, setIsChild2Open] = useState(false);

    return (
      <Collapsible open={isParentOpen} onOpenChange={setIsParentOpen} className="w-96 space-y-2 border p-4 rounded-lg">
        <div className="flex items-center justify-between">
          <h4 className="text-sm font-semibold">Metabolism Kit</h4>
          <CollapsibleTrigger asChild>
            <Button variant="ghost" size="sm">
              <ChevronDown className={`transition-transform ${isParentOpen ? "rotate-180" : ""}`} />
            </Button>
          </CollapsibleTrigger>
        </div>
        <CollapsibleContent className="space-y-2 pl-4">
          <Collapsible open={isChild1Open} onOpenChange={setIsChild1Open} className="space-y-2">
            <div className="flex items-center justify-between">
              <h5 className="text-sm font-medium">Capsule Types</h5>
              <CollapsibleTrigger asChild>
                <Button variant="ghost" size="sm">
                  <ChevronDown className={`size-3 transition-transform ${isChild1Open ? "rotate-180" : ""}`} />
                </Button>
              </CollapsibleTrigger>
            </div>
            <CollapsibleContent className="space-y-1 pl-4">
              <div className="text-sm">Capsule J</div>
              <div className="text-sm">Capsule L</div>
            </CollapsibleContent>
          </Collapsible>

          <Collapsible open={isChild2Open} onOpenChange={setIsChild2Open} className="space-y-2">
            <div className="flex items-center justify-between">
              <h5 className="text-sm font-medium">Base Types</h5>
              <CollapsibleTrigger asChild>
                <Button variant="ghost" size="sm">
                  <ChevronDown className={`size-3 transition-transform ${isChild2Open ? "rotate-180" : ""}`} />
                </Button>
              </CollapsibleTrigger>
            </div>
            <CollapsibleContent className="space-y-1 pl-4">
              <div className="text-sm">Base Blob</div>
            </CollapsibleContent>
          </Collapsible>
        </CollapsibleContent>
      </Collapsible>
    );
  },
};
