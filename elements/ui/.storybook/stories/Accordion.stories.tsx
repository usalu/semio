// #region 🔖Header

// 🥼︎ semio/js/.storybook/stories/elements/aggregation/Accordion.stories.tsx

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

// #endregion 🔖Header

import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "@elements/ui";
import type { Meta, StoryObj } from "@storybook/react";

// #region 🔖Accordion
const meta = {
  title: "elements/Accordion",
  component: Accordion,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Accordion>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    type: "multiple",
    defaultValue: ["item-2", "item-3"],
  },
  render: (args) => (
    <div className="w-96">
      <Accordion {...args}>
        <AccordionItem value="item-1" disabled={false}>
          <AccordionTrigger>What is a Kit?</AccordionTrigger>
          <AccordionContent>
            A Kit is a collection of types, designs, and qualities that define a modular building system with reusable components. The Metabolism kit, for example, includes capsule types, base types, tambour types, and connection definitions.
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-2" disabled={false}>
          <AccordionTrigger>What is a Type?</AccordionTrigger>
          <AccordionContent>A Type is a reusable component with models, connectors, and properties that can be instantiated as pieces in a design. Types can include 3D models, metadata, and connection points for assembly.</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-3" disabled={false}>
          <AccordionTrigger>What is a Connection?</AccordionTrigger>
          <AccordionContent>A Connection is a 3D link between two pieces with translation and rotation parameters defining their spatial relationship. Connections have gap, shift, rise, rotation, turn, and tilt parameters.</AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  ),
};

// #endregion 🔖Accordion
