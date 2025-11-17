// #region Header

// Tabs.stories.tsx

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
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../../../../sketchpad/elements";

// #region Tabs
const meta = {
  title: "Elements/Aggregation/Tabs",
  component: Tabs,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Tabs>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <Tabs defaultValue="properties" className="w-[600px]">
      <TabsList>
        <TabsTrigger value="properties">Properties</TabsTrigger>
        <TabsTrigger value="connections">Connections</TabsTrigger>
        <TabsTrigger value="statistics">Statistics</TabsTrigger>
      </TabsList>
      <TabsContent value="properties" className="space-y-4">
        <div className="border p-small space-y-2">
          <div className="flex justify-between">
            <span className="text-sm font-medium">Volume</span>
            <span className="text-sm">25.0 m³</span>
          </div>
          <div className="flex justify-between">
            <span className="text-sm font-medium">Area</span>
            <span className="text-sm">10.0 m²</span>
          </div>
          <div className="flex justify-between">
            <span className="text-sm font-medium">Mass</span>
            <span className="text-sm">3500 kg</span>
          </div>
        </div>
      </TabsContent>
      <TabsContent value="connections" className="space-y-4">
        <div className="border p-small space-y-2">
          <p className="text-sm">3 active connections</p>
          <p className="text-sm text-muted-foreground">Base → Capsule J (Standard)</p>
          <p className="text-sm text-muted-foreground">Capsule J → Tambour A</p>
          <p className="text-sm text-muted-foreground">Tambour A → Capital</p>
        </div>
      </TabsContent>
      <TabsContent value="statistics" className="space-y-4">
        <div className="border p-small space-y-2">
          <p className="text-sm">Total pieces: 140</p>
          <p className="text-sm">Capsule count: 132</p>
          <p className="text-sm">Total height: 52.4m</p>
        </div>
      </TabsContent>
    </Tabs>
  ),
};

// #endregion Tabs
