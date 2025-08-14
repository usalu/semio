// #region Header

// cluster-design.test.ts

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

import { clusterDesign, Connection, Design, Kit, Piece, Type } from "@semio/js/semio";
import { beforeEach, describe, expect, it } from "vitest";

describe("clusterDesign", () => {
  let mockKit: Kit;
  let mockDesign: Design;
  let mockType: Type;

  beforeEach(() => {
    // Create a mock type
    mockType = {
      name: "test-type",
      unit: "m",
      ports: [
        {
          id_: "port1",
          t: 0,
          point: { x: 0, y: 0, z: 0 },
          direction: { x: 1, y: 0, z: 0 },
        },
        {
          id_: "port2",
          t: 1,
          point: { x: 1, y: 0, z: 0 },
          direction: { x: -1, y: 0, z: 0 },
        },
      ],
      created: new Date("2024-01-01"),
      updated: new Date("2024-01-01"),
    };

    // Create mock pieces with connections
    const pieces: Piece[] = [
      {
        id_: "piece1",
        type: { name: "test-type" },
        plane: {
          origin: { x: 0, y: 0, z: 0 },
          xAxis: { x: 1, y: 0, z: 0 },
          yAxis: { x: 0, y: 1, z: 0 },
        },
        center: { x: 0, y: 0 },
      },
      {
        id_: "piece2",
        type: { name: "test-type" },
        center: { x: 1, y: 0 },
      },
      {
        id_: "piece3",
        type: { name: "test-type" },
        center: { x: 2, y: 0 },
      },
      {
        id_: "piece4",
        type: { name: "test-type" },
        center: { x: 3, y: 0 },
      },
    ];

    const connections: Connection[] = [
      {
        connected: {
          piece: { id_: "piece1" },
          port: { id_: "port2" },
        },
        connecting: {
          piece: { id_: "piece2" },
          port: { id_: "port1" },
        },
      },
      {
        connected: {
          piece: { id_: "piece2" },
          port: { id_: "port2" },
        },
        connecting: {
          piece: { id_: "piece3" },
          port: { id_: "port1" },
        },
      },
      {
        connected: {
          piece: { id_: "piece3" },
          port: { id_: "port2" },
        },
        connecting: {
          piece: { id_: "piece4" },
          port: { id_: "port1" },
        },
      },
    ];

    mockDesign = {
      name: "test-design",
      unit: "m",
      pieces,
      connections,
      created: new Date("2024-01-01"),
      updated: new Date("2024-01-01"),
    };

    mockKit = {
      name: "test-kit",
      version: "1.0.0",
      types: [mockType],
      designs: [mockDesign],
      created: new Date("2024-01-01"),
      updated: new Date("2024-01-01"),
    };
  });

  it("should cluster connected pieces successfully", () => {
    const selectedPieceIds = ["piece2", "piece3"];

    const result = clusterDesign(mockKit, "test-design", selectedPieceIds, "test-cluster");

    // Check that clustered design was created
    expect(result.clusteredDesign.name).toBe("test-cluster");
    expect(result.clusteredDesign.pieces).toHaveLength(2);
    expect(result.clusteredDesign.pieces?.map((p) => p.id_)).toEqual(expect.arrayContaining(["piece2", "piece3"]));

    // Check that clustered design has internal connection
    expect(result.clusteredDesign.connections).toHaveLength(1);
    expect(result.clusteredDesign.connections?.[0].connected.piece.id_).toBe("piece2");
    expect(result.clusteredDesign.connections?.[0].connecting.piece.id_).toBe("piece3");

    // Check that source design has remaining pieces
    expect(result.updatedSourceDesign.pieces).toHaveLength(2);
    expect(result.updatedSourceDesign.pieces?.map((p) => p.id_)).toEqual(expect.arrayContaining(["piece1", "piece4"]));

    // Check that source design has external connections updated with designId
    const externalConnections = result.updatedSourceDesign.connections?.filter((c) => c.connected.designId || c.connecting.designId);
    expect(externalConnections).toHaveLength(2);

    // Check that the kit now contains the new clustered design
    expect(result.updatedKit.designs).toHaveLength(2);
    const clusteredDesignInKit = result.updatedKit.designs?.find((d) => d.name === "test-cluster");
    expect(clusteredDesignInKit).toBeDefined();
  });

  it("should use default cluster name when none provided", () => {
    const selectedPieceIds = ["piece2", "piece3"];

    const result = clusterDesign(mockKit, "test-design", selectedPieceIds);

    expect(result.clusteredDesign.name).toMatch(/^Cluster-\d+$/);
  });

  it("should throw error when no clusterable groups found", () => {
    const selectedPieceIds = ["piece1"]; // Single piece - not clusterable

    expect(() => {
      clusterDesign(mockKit, "test-design", selectedPieceIds);
    }).toThrow("No clusterable groups found with current selection");
  });

  it("should throw error when design not found", () => {
    const selectedPieceIds = ["piece2", "piece3"];

    expect(() => {
      clusterDesign(mockKit, "non-existent-design", selectedPieceIds);
    }).toThrow("Design non-existent-design not found in kit test-kit");
  });

  it("should ensure clustered design has at least one fixed piece", () => {
    // Create pieces without fixed planes/centers
    const unfixedPieces: Piece[] = [
      {
        id_: "unfixed1",
        type: { name: "test-type" },
      },
      {
        id_: "unfixed2",
        type: { name: "test-type" },
      },
    ];

    const unfixedConnections: Connection[] = [
      {
        connected: {
          piece: { id_: "unfixed1" },
          port: { id_: "port2" },
        },
        connecting: {
          piece: { id_: "unfixed2" },
          port: { id_: "port1" },
        },
      },
    ];

    const unfixedDesign: Design = {
      name: "unfixed-design",
      unit: "m",
      pieces: unfixedPieces,
      connections: unfixedConnections,
      created: new Date(),
      updated: new Date(),
    };

    const unfixedKit: Kit = {
      ...mockKit,
      designs: [unfixedDesign],
    };

    const result = clusterDesign(unfixedKit, "unfixed-design", ["unfixed1", "unfixed2"]);

    // Should have at least one piece with both plane and center set
    const fixedPieces = result.clusteredDesign.pieces?.filter((p) => p.plane && p.center);
    expect(fixedPieces).toHaveLength(1);
  });

  it("should handle clustering when source design has no pieces", () => {
    const emptyDesign: Design = {
      name: "empty-design",
      unit: "m",
      pieces: [],
      connections: [],
      created: new Date(),
      updated: new Date(),
    };

    const emptyKit: Kit = {
      ...mockKit,
      designs: [emptyDesign],
    };

    expect(() => {
      clusterDesign(emptyKit, "empty-design", []);
    }).toThrow("No clusterable groups found with current selection");
  });

  it("should cluster all selected pieces when multiple disconnected pieces are selected", () => {
    // Add disconnected pieces to the design
    const extendedPieces: Piece[] = [
      ...mockDesign.pieces!,
      {
        id_: "isolated1",
        type: { name: "test-type" },
        center: { x: 10, y: 10 },
      },
      {
        id_: "isolated2",
        type: { name: "test-type" },
        center: { x: 11, y: 10 },
      },
    ];

    const extendedDesign: Design = {
      ...mockDesign,
      pieces: extendedPieces,
    };

    const extendedKit: Kit = {
      ...mockKit,
      designs: [extendedDesign],
    };

    // Select pieces from both connected and isolated groups
    const selectedPieceIds = ["piece2", "piece3", "isolated1", "isolated2"];

    const result = clusterDesign(extendedKit, "test-design", selectedPieceIds);

    // Should cluster all selected pieces regardless of connectivity
    expect(result.clusteredDesign.pieces).toHaveLength(4);
    expect(result.clusteredDesign.pieces?.map((p) => p.id_)).toEqual(expect.arrayContaining(selectedPieceIds));
  });
});
