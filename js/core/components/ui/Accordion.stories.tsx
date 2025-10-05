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

export const Single: Story = {
  render: () => (
    <div className="w-96">
      <Accordion type="single" collapsible>
        <AccordionItem value="item-1">
          <AccordionTrigger>What is Semio?</AccordionTrigger>
          <AccordionContent>Semio is a parametric design and engineering platform that enables collaborative creation of complex systems.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-2">
          <AccordionTrigger>How does it work?</AccordionTrigger>
          <AccordionContent>It combines visual programming, parametric modeling, and real-time collaboration to streamline the design process.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-3">
          <AccordionTrigger>Who can use it?</AccordionTrigger>
          <AccordionContent>Engineers, architects, designers, and anyone working on complex parametric designs.</AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  ),
};

export const Multiple: Story = {
  render: () => (
    <div className="w-96">
      <Accordion type="multiple">
        <AccordionItem value="features">
          <AccordionTrigger>Features</AccordionTrigger>
          <AccordionContent>
            <ul className="list-disc list-inside space-y-1">
              <li>Parametric modeling</li>
              <li>Real-time collaboration</li>
              <li>Version control</li>
              <li>Export to multiple formats</li>
            </ul>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="pricing">
          <AccordionTrigger>Pricing</AccordionTrigger>
          <AccordionContent>
            <p>Free for individual use. Contact us for team and enterprise pricing.</p>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="support">
          <AccordionTrigger>Support</AccordionTrigger>
          <AccordionContent>
            <p>Documentation, community forums, and email support available.</p>
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
          <AccordionTrigger>Closed by default</AccordionTrigger>
          <AccordionContent>This item starts closed.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-2">
          <AccordionTrigger>Open by default</AccordionTrigger>
          <AccordionContent>This item starts open because its value matches defaultValue.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-3">
          <AccordionTrigger>Also closed</AccordionTrigger>
          <AccordionContent>This item also starts closed.</AccordionContent>
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
          <AccordionTrigger>Code Example</AccordionTrigger>
          <AccordionContent>
            <pre className="bg-muted p-2 rounded text-xs">
              {`function greet(name: string) {
  return \`Hello, \${name}!\`;
}`}
            </pre>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="table">
          <AccordionTrigger>Data Table</AccordionTrigger>
          <AccordionContent>
            <table className="w-full text-xs">
              <thead>
                <tr className="border-b">
                  <th className="text-left py-1">Name</th>
                  <th className="text-left py-1">Value</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <td>Width</td>
                  <td>100px</td>
                </tr>
                <tr>
                  <td>Height</td>
                  <td>200px</td>
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
          <AccordionTrigger>Active Item</AccordionTrigger>
          <AccordionContent>This item is active and can be toggled.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-2" disabled>
          <AccordionTrigger>Disabled Item</AccordionTrigger>
          <AccordionContent>This content won't be accessible.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-3">
          <AccordionTrigger>Another Active Item</AccordionTrigger>
          <AccordionContent>This item is also active.</AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  ),
};
