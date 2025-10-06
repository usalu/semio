// #region Header

// Accordion.stories.tsx

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
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "./Accordion";

const meta = {
  title: "Elements/Accordion",
  component: Accordion,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Accordion>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <div className="w-96">
      <Accordion type="single" collapsible defaultValue="item-1">
        <AccordionItem value="item-1">
          <AccordionTrigger>What is a Kit?</AccordionTrigger>
          <AccordionContent>A Kit is a collection of types, designs, and qualities that define a modular building system with reusable components. The Metabolism kit, for example, includes capsule types, base types, tambour types, and connection definitions.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-2">
          <AccordionTrigger>What is a Type?</AccordionTrigger>
          <AccordionContent>A Type is a reusable component with representations, ports, and properties that can be instantiated as pieces in a design. Types can include 3D models, metadata, and connection points for assembly.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-3">
          <AccordionTrigger>What is a Connection?</AccordionTrigger>
          <AccordionContent>A Connection is a 3D link between two pieces with translation and rotation parameters defining their spatial relationship. Connections have gap, shift, rise, rotation, turn, and tilt parameters.</AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  ),
};

export const Variants: Story = {
  render: () => (
    <div className="flex gap-4">
      <div className="w-96">
        <Accordion type="single" collapsible>
          <AccordionItem value="single-1">
            <AccordionTrigger>Single Type</AccordionTrigger>
            <AccordionContent>Only one item can be open at a time with single type accordion.</AccordionContent>
          </AccordionItem>
          <AccordionItem value="single-2">
            <AccordionTrigger>Collapsible</AccordionTrigger>
            <AccordionContent>Items can be collapsed when clicked again.</AccordionContent>
          </AccordionItem>
        </Accordion>
      </div>
      <div className="w-96">
        <Accordion type="multiple">
          <AccordionItem value="multiple-1">
            <AccordionTrigger>Multiple Type</AccordionTrigger>
            <AccordionContent>Multiple items can be open simultaneously.</AccordionContent>
          </AccordionItem>
          <AccordionItem value="multiple-2">
            <AccordionTrigger>Independent</AccordionTrigger>
            <AccordionContent>Each item toggles independently.</AccordionContent>
          </AccordionItem>
        </Accordion>
      </div>
    </div>
  ),
};

export const Multiple: Story = {
  render: () => (
    <div className="w-96">
      <Accordion type="multiple">
        <AccordionItem value="features">
          <AccordionTrigger>Capsule Types</AccordionTrigger>
          <AccordionContent>
            <ul className="list-disc list-inside space-y-1">
              <li>Capsule J</li>
              <li>Capsule L</li>
              <li>Capsule P with Balcony</li>
              <li>Capsule Z variant</li>
            </ul>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="pricing">
          <AccordionTrigger>Base Types</AccordionTrigger>
          <AccordionContent>
            <p>Base Blob and Base Standard variants available for different foundation requirements.</p>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="support">
          <AccordionTrigger>Tambour Types</AccordionTrigger>
          <AccordionContent>
            <p>Cylindric tambour variants including first storey, last storey, and single storey configurations.</p>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  ),
};

export const DefaultOpen: Story = {
  render: () => (
    <div className="w-96">
      <Accordion type="single" defaultValue="item-2" collapsible>
        <AccordionItem value="item-1">
          <AccordionTrigger>Design Properties</AccordionTrigger>
          <AccordionContent>View and edit design-level properties and metadata.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-2">
          <AccordionTrigger>Piece Count</AccordionTrigger>
          <AccordionContent>This section shows the count of pieces in the design. Currently 24 capsules are used.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-3">
          <AccordionTrigger>Connection Statistics</AccordionTrigger>
          <AccordionContent>Overview of connections between pieces in the current design.</AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  ),
};

export const WithComplexContent: Story = {
  render: () => (
    <div className="w-96">
      <Accordion type="single" collapsible>
        <AccordionItem value="code">
          <AccordionTrigger>Connection Parameters</AccordionTrigger>
          <AccordionContent>
            <pre className="bg-muted p-2 rounded text-xs">
              {`gap: 10mm
shift: 5mm
rise: 0mm
rotation: 45deg
turn: 0deg
tilt: 0deg`}
            </pre>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="table">
          <AccordionTrigger>Design Qualities</AccordionTrigger>
          <AccordionContent>
            <table className="w-full text-xs">
              <thead>
                <tr className="border-b">
                  <th className="text-left py-1">Quality</th>
                  <th className="text-left py-1">Value</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <td>Volume</td>
                  <td>3240m³</td>
                </tr>
                <tr>
                  <td>Height</td>
                  <td>54m</td>
                </tr>
              </tbody>
            </table>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  ),
};

export const Disabled: Story = {
  render: () => (
    <div className="w-96">
      <Accordion type="single" collapsible>
        <AccordionItem value="item-1">
          <AccordionTrigger>Editable Properties</AccordionTrigger>
          <AccordionContent>These properties can be modified and updated.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-2" disabled>
          <AccordionTrigger>Locked Layer</AccordionTrigger>
          <AccordionContent>This layer is locked and cannot be modified.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-3">
          <AccordionTrigger>Type Attributes</AccordionTrigger>
          <AccordionContent>View and edit type-level attributes and metadata.</AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  ),
};
