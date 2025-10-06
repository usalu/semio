// #region Header

// ScrollArea.stories.tsx

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
import { ScrollArea } from "./ScrollArea";

const meta = {
  title: "Elements/ScrollArea",
  component: ScrollArea,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ScrollArea>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <ScrollArea className="h-96 w-[600px] border rounded-md">
      <div className="p-4 space-y-4">
        <div>
          <h4 className="text-sm font-semibold mb-2">Nakagin Capsule Tower</h4>
          <p className="text-sm text-muted-foreground mb-4">
            The Nakagin Capsule Tower Building is a mixed-use residential and office tower in Tokyo, Japan designed by architect Kisho Kurokawa.
            Completed in 1972, the building is a rare remaining example of Japanese Metabolism architecture.
          </p>
        </div>
        <div>
          <h4 className="text-sm font-semibold mb-2">Design Specifications</h4>
          <div className="text-sm space-y-1">
            <div>Total Capsules: 140</div>
            <div>Capsule Dimensions: 2.5m × 4.0m × 2.5m</div>
            <div>Building Height: 52.4m</div>
            <div>Total Floors: 13</div>
            <div>Construction: Prefabricated steel frame</div>
          </div>
        </div>
        <div>
          <h4 className="text-sm font-semibold mb-2">Structural System</h4>
          <p className="text-sm text-muted-foreground">
            Each capsule was designed to be replaceable and fully self-contained with built-in bathroom and storage.
            The capsules were attached to two interconnected concrete towers with high-tension bolts, allowing for individual replacement.
          </p>
        </div>
        <div>
          <h4 className="text-sm font-semibold mb-2">Historical Context</h4>
          <p className="text-sm text-muted-foreground">
            The Metabolism movement emerged in 1960s Japan, proposing buildings that could adapt to changing needs through modular, replaceable components.
            The Nakagin Tower represents one of the few built realizations of these principles.
          </p>
        </div>
      </div>
    </ScrollArea>
  ),
};

export const Variants: Story = {
  render: () => (
    <div className="flex gap-8">
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Vertical</p>
        <ScrollArea className="h-72 w-80 border rounded-md">
          <div className="p-4 space-y-2">
            <h4 className="text-sm font-medium mb-4">Types</h4>
            {Array.from({ length: 30 }, (_, i) => (
              <div key={i} className="text-sm">
                Type {i + 1}
              </div>
            ))}
          </div>
        </ScrollArea>
      </div>
      <div className="space-y-2">
        <p className="text-xs text-muted-foreground mb-4">Horizontal</p>
        <ScrollArea className="w-96 h-32 border rounded-md" orientation="horizontal">
          <div className="p-4 flex space-x-4">
            {Array.from({ length: 20 }, (_, i) => (
              <div key={i} className="text-sm whitespace-nowrap">
                Piece {i + 1}
              </div>
            ))}
          </div>
        </ScrollArea>
      </div>
    </div>
  ),
};

export const Vertical: Story = {
  render: () => (
    <ScrollArea className="h-72 w-96 border rounded-md">
      <div className="p-4 space-y-4">
        <h4 className="text-sm font-medium">Scroll down to see more content</h4>
        {Array.from({ length: 50 }, (_, i) => (
          <div key={i} className="text-sm">
            Item {i + 1}
          </div>
        ))}
      </div>
    </ScrollArea>
  ),
};

export const LongText: Story = {
  render: () => (
    <ScrollArea className="h-64 w-96 border rounded-md">
      <div className="p-4">
        <h4 className="mb-4 text-sm font-medium">Design Documentation</h4>
        <div className="text-sm space-y-4">
          <p>The Nakagin Capsule Tower is a mixed-use residential and office tower designed by architect Kisho Kurokawa and located in Shimbashi, Tokyo, Japan.</p>
          <p>Completed in 1972, the building is a rare remaining example of Japanese Metabolism, a post-war architectural movement that fused ideas about architectural megastructures with those of organic biological growth.</p>
          <p>The building was made of prefabricated capsules which could be plugged in to the concrete towers. Each capsule measures 2.5 m × 4.0 m × 2.5 m and was designed to be replaceable.</p>
          <p>The capsules were intended to be replaced every 25 years, but this never happened. The building became a symbol of the Metabolist movement and its vision of sustainable, adaptable architecture.</p>
          <p>The tower consists of two interconnected concrete cores with 140 prefabricated capsules inserted into the cores. The capsules can be individually removed and replaced without affecting the integrity of the building.</p>
          <p>Each capsule features a circular window, built-in storage, a bathroom, and was originally equipped with a bed, desk, and reel-to-reel tape deck. The modular design represented a radical approach to urban living.</p>
          <p>Despite its architectural significance and influence on modern modular design, the building was demolished in 2022 due to asbestos concerns and the difficulty of maintaining the aging capsules.</p>
        </div>
      </div>
    </ScrollArea>
  ),
};

export const Horizontal: Story = {
  render: () => (
    <ScrollArea className="w-96 border rounded-md whitespace-nowrap">
      <div className="p-4">
        <div className="flex gap-4">
          {Array.from({ length: 20 }, (_, i) => (
            <div key={i} className="inline-flex h-20 w-40 items-center justify-center rounded-md border bg-muted">
              Card {i + 1}
            </div>
          ))}
        </div>
      </div>
    </ScrollArea>
  ),
};

export const CodeBlock: Story = {
  render: () => (
    <ScrollArea className="h-72 w-96 border rounded-md">
      <pre className="p-4 text-sm">
        {`// Connection calculation
function calculateConnection(piece1, piece2, port1, port2) {
  const plane1 = piece1.getPortPlane(port1);
  const plane2 = piece2.getPortPlane(port2);
  
  const gap = plane1.origin.distanceTo(plane2.origin);
  const shift = plane1.xAxis.dot(plane2.xAxis);
  const rise = plane1.yAxis.dot(plane2.yAxis);
  
  return { gap, shift, rise };
}

// Quality aggregation
function aggregateQuality(design, qualityKey) {
  let total = 0;
  for (const piece of design.pieces) {
    const value = piece.getQualityValue(qualityKey);
    if (value !== undefined) {
      total += value;
    }
  }
  return total;
}

// Design validation
function validateDesign(design) {
  const errors = [];
  
  for (const piece of design.pieces) {
    if (!piece.plane && !isConnected(piece, design)) {
      errors.push(\`Piece \${piece.id} is floating\`);
    }
  }
  
  for (const connection of design.connections) {
    if (!isValidConnection(connection)) {
      errors.push(\`Invalid connection: \${connection.id}\`);
    }
  }
  
  return errors;
}`}
      </pre>
    </ScrollArea>
  ),
};

export const List: Story = {
  render: () => (
    <ScrollArea className="h-80 w-96 border rounded-md">
      <div className="p-4">
        <h4 className="mb-4 text-sm font-medium">Kit Resources</h4>
        <div className="space-y-2">
          {[
            "kit.json",
            "types/capsule-j.glb",
            "types/capsule-l.glb",
            "types/capsule-p.glb",
            "types/base-blob.glb",
            "types/base-standard.glb",
            "types/tambour-cylindric.glb",
            "types/tambour-first-storey.glb",
            "types/tambour-last-storey.glb",
            "types/capital.glb",
            "designs/nakagin-tower.json",
            "designs/cluster-a.json",
            "designs/cluster-b.json",
            "qualities.json",
            "README.md",
            "LICENSE.txt",
            "preview.png",
          ].map((file) => (
            <div key={file} className="flex items-center gap-2 p-2 hover:bg-accent rounded-sm cursor-pointer text-sm">
              <span className="font-mono">{file}</span>
            </div>
          ))}
        </div>
      </div>
    </ScrollArea>
  ),
};

export const BothDirections: Story = {
  render: () => (
    <ScrollArea className="h-72 w-96 border rounded-md">
      <div className="p-4" style={{ width: "800px" }}>
        <h4 className="mb-4 text-sm font-medium">Wide Table</h4>
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b">
              <th className="text-left p-2">Column 1</th>
              <th className="text-left p-2">Column 2</th>
              <th className="text-left p-2">Column 3</th>
              <th className="text-left p-2">Column 4</th>
              <th className="text-left p-2">Column 5</th>
              <th className="text-left p-2">Column 6</th>
            </tr>
          </thead>
          <tbody>
            {Array.from({ length: 20 }, (_, i) => (
              <tr key={i} className="border-b">
                <td className="p-2">Row {i + 1} Col 1</td>
                <td className="p-2">Data {i + 1}.2</td>
                <td className="p-2">Data {i + 1}.3</td>
                <td className="p-2">Data {i + 1}.4</td>
                <td className="p-2">Data {i + 1}.5</td>
                <td className="p-2">Data {i + 1}.6</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </ScrollArea>
  ),
};
