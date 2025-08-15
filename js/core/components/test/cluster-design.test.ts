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

import { clusterDesign, Connection, Design, explodeDesign, Kit, Piece, Type } from "@semio/js/semio";
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

describe("expandDesign", () => {
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

  it("should expand a clustered design back into its parent design", () => {
    // First cluster some pieces
    const selectedPieceIds = ["piece2", "piece3"];
    const clusterResult = clusterDesign(mockKit, "test-design", selectedPieceIds, "test-cluster");

    // Verify we have a clustered design
    expect(clusterResult.updatedKit.designs).toHaveLength(2);
    expect(clusterResult.clusteredDesign.pieces).toHaveLength(2);
    expect(clusterResult.updatedSourceDesign.pieces).toHaveLength(2); // piece1 and piece4

    // Now expand the clustered design back
    const expandResult = explodeDesign(clusterResult.updatedKit, "test-cluster");

    // Check that the clustered design was removed
    expect(expandResult.updatedKit.designs).toHaveLength(1);
    expect(expandResult.removedDesignName).toBe("test-cluster");

    // Check that the expanded design contains all original pieces
    expect(expandResult.expandedDesign.pieces).toHaveLength(4);
    expect(expandResult.expandedDesign.pieces?.map((p) => p.id_)).toEqual(expect.arrayContaining(["piece1", "piece2", "piece3", "piece4"]));

    // Check that the expanded design contains all original connections
    expect(expandResult.expandedDesign.connections).toHaveLength(3);

    // Verify that the design is the original test-design
    expect(expandResult.expandedDesign.name).toBe("test-design");
  });

  it("should throw error when trying to expand a non-clustered design", () => {
    expect(() => {
      explodeDesign(mockKit, "test-design"); // This design is not clustered
    }).toThrow("No affected designs found for expansion - this design may not be clustered");
  });

  it("should throw error when design to expand does not exist", () => {
    expect(() => {
      explodeDesign(mockKit, "non-existent-design");
    }).toThrow("Design non-existent-design not found in kit test-kit");
  });

  it("should properly restore external connections when expanding", () => {
    // First cluster some pieces
    const selectedPieceIds = ["piece2", "piece3"];
    const clusterResult = clusterDesign(mockKit, "test-design", selectedPieceIds, "test-cluster");

    // Verify external connections were created with designId
    const externalConnections = clusterResult.updatedSourceDesign.connections?.filter((c) => c.connected.designId || c.connecting.designId);
    expect(externalConnections).toHaveLength(2);

    // Now expand the clustered design back
    const expandResult = explodeDesign(clusterResult.updatedKit, "test-cluster");

    // Check that external connections were restored (no designId references)
    const connectionsWithDesignId = expandResult.expandedDesign.connections?.filter((c) => c.connected.designId || c.connecting.designId);
    expect(connectionsWithDesignId).toHaveLength(0);

    // All connections should be regular connections again
    expect(expandResult.expandedDesign.connections).toHaveLength(3);
  });

  it("should preserve external connections with designId when clustering", () => {
    // Start with original design
    expect(mockDesign.pieces).toHaveLength(4);
    expect(mockDesign.connections).toHaveLength(3);

    // Cluster pieces 2 and 3 (use correct IDs: piece2, piece3)
    const result = clusterDesign(mockKit, "test-design", ["piece2", "piece3"]);

    // Check that the updated source design has the right structure
    expect(result.updatedSourceDesign.pieces).toHaveLength(2); // piece1 and piece4 remain
    expect(result.updatedSourceDesign.connections).toHaveLength(2); // 2 external connections

    // Find the external connections that should reference the clustered design
    const externalConnections = result.updatedSourceDesign.connections.filter((conn: Connection) => conn.connected.designId || conn.connecting.designId);

    expect(externalConnections).toHaveLength(2); // One connection from piece1 to cluster, one from cluster to piece4

    // Verify that external connections reference the clustered design
    for (const connection of externalConnections) {
      const hasDesignId = connection.connected.designId || connection.connecting.designId;
      expect(hasDesignId).toBeTruthy();

      if (connection.connected.designId) {
        expect(connection.connected.designId).toBe(result.clusteredDesign.name);
        expect(["piece2", "piece3"]).toContain(connection.connected.piece.id_);
      } else if (connection.connecting.designId) {
        expect(connection.connecting.designId).toBe(result.clusteredDesign.name);
        expect(["piece2", "piece3"]).toContain(connection.connecting.piece.id_);
      }
    }

    // Verify the clustered design has the expected pieces
    expect(result.clusteredDesign.pieces).toHaveLength(2);
    expect(result.clusteredDesign.connections).toHaveLength(1); // The internal connection between piece2 and piece3
  });

  it("should create external connections with designId properties when clustering", () => {
    // Start with original design
    expect(mockDesign.pieces).toHaveLength(4);
    expect(mockDesign.connections).toHaveLength(3);

    // Cluster pieces 2 and 3
    const result = clusterDesign(mockKit, "test-design", ["piece2", "piece3"]);

    // Check external connections in the updated source design
    const externalConnections = result.updatedSourceDesign.connections.filter((conn: Connection) => conn.connected.designId || conn.connecting.designId);

    expect(externalConnections).toHaveLength(2); // Should have 2 external connections

    // Verify the external connections have the correct designId
    externalConnections.forEach((conn: Connection) => {
      const hasDesignId = conn.connected.designId || conn.connecting.designId;
      expect(hasDesignId).toBeTruthy();

      if (conn.connected.designId) {
        expect(conn.connected.designId).toMatch(/^Cluster-\d+$/);
      }
      if (conn.connecting.designId) {
        expect(conn.connecting.designId).toMatch(/^Cluster-\d+$/);
      }
    });
  });

  it("should handle complex cluster and expand workflow", () => {
    // Start with original design
    expect(mockKit.designs).toHaveLength(1);
    expect(mockDesign.pieces).toHaveLength(4);

    // Step 1: Cluster pieces 2 and 3
    const clusterResult1 = clusterDesign(mockKit, "test-design", ["piece2", "piece3"], "cluster-1");
    expect(clusterResult1.updatedKit.designs).toHaveLength(2);
    expect(clusterResult1.clusteredDesign.pieces).toHaveLength(2);
    expect(clusterResult1.updatedSourceDesign.pieces).toHaveLength(2);

    // Step 2: Cluster piece1 and piece4 from the updated source design
    const clusterResult2 = clusterDesign(clusterResult1.updatedKit, "test-design", ["piece1", "piece4"], "cluster-2");
    expect(clusterResult2.updatedKit.designs).toHaveLength(3);
    expect(clusterResult2.clusteredDesign.pieces).toHaveLength(2);
    expect(clusterResult2.updatedSourceDesign.pieces).toHaveLength(0); // All pieces are now clustered

    // Step 3: Expand cluster-1 back
    const expandResult1 = explodeDesign(clusterResult2.updatedKit, "cluster-1");
    expect(expandResult1.updatedKit.designs).toHaveLength(2); // cluster-1 removed
    expect(expandResult1.removedDesignName).toBe("cluster-1");
    expect(expandResult1.expandedDesign.pieces).toHaveLength(2); // piece2 and piece3 restored

    // Step 4: Expand cluster-2 back
    const expandResult2 = explodeDesign(expandResult1.updatedKit, "cluster-2");
    expect(expandResult2.updatedKit.designs).toHaveLength(1); // Back to single design
    expect(expandResult2.removedDesignName).toBe("cluster-2");
    expect(expandResult2.expandedDesign.pieces).toHaveLength(4); // All pieces restored

    // Final check: Should be back to original state
    expect(expandResult2.expandedDesign.pieces?.map((p) => p.id_)).toEqual(expect.arrayContaining(["piece1", "piece2", "piece3", "piece4"]));
    expect(expandResult2.expandedDesign.connections).toHaveLength(3);
  });

  it("should preserve external connections to nested clustered designs when expanding", () => {
    // This test reproduces the exact scenario described by the user:
    // Original design #0 -> clustered design #1 -> nested clustered design #2
    // When expanding #1, connections to #2 should be preserved

    // Start with original design with 6 pieces to create a more complex scenario
    const extendedPieces: Piece[] = [
      ...mockDesign.pieces!,
      {
        id_: "piece5",
        type: { name: "test-type" },
        center: { x: 4, y: 0 },
      },
      {
        id_: "piece6",
        type: { name: "test-type" },
        center: { x: 5, y: 0 },
      },
    ];

    const extendedConnections: Connection[] = [
      ...mockDesign.connections!,
      {
        connected: {
          piece: { id_: "piece4" },
          port: { id_: "port2" },
        },
        connecting: {
          piece: { id_: "piece5" },
          port: { id_: "port1" },
        },
      },
      {
        connected: {
          piece: { id_: "piece5" },
          port: { id_: "port2" },
        },
        connecting: {
          piece: { id_: "piece6" },
          port: { id_: "port1" },
        },
      },
    ];

    const extendedDesign: Design = {
      ...mockDesign,
      pieces: extendedPieces,
      connections: extendedConnections,
    };

    const extendedKit: Kit = {
      ...mockKit,
      designs: [extendedDesign],
    };

    // Step 1: Create clustered design #1 with pieces 2, 3, 4
    const clusterResult1 = clusterDesign(extendedKit, "test-design", ["piece2", "piece3", "piece4"], "cluster-1");
    expect(clusterResult1.updatedKit.designs).toHaveLength(2);
    expect(clusterResult1.clusteredDesign.pieces).toHaveLength(3);
    expect(clusterResult1.updatedSourceDesign.pieces).toHaveLength(3); // piece1, piece5, piece6

    // Step 2: Create nested clustered design #2 inside cluster-1 with pieces 3, 4
    const clusterResult2 = clusterDesign(clusterResult1.updatedKit, "cluster-1", ["piece3", "piece4"], "cluster-2");
    expect(clusterResult2.updatedKit.designs).toHaveLength(3);
    expect(clusterResult2.clusteredDesign.pieces).toHaveLength(2); // piece3, piece4
    expect(clusterResult2.updatedSourceDesign.pieces).toHaveLength(1); // piece2 remains in cluster-1

    // Step 3: Go back to original design #0 and expand cluster-1
    const expandResult = explodeDesign(clusterResult2.updatedKit, "cluster-1");

    // After expansion, we should have:
    // - Original design with piece1, piece2, piece5, piece6
    // - cluster-2 should still exist as a separate design
    // - External connections to cluster-2 should be preserved
    expect(expandResult.updatedKit.designs).toHaveLength(2); // original + cluster-2
    expect(expandResult.expandedDesign.pieces).toHaveLength(4); // piece1, piece2, piece5, piece6
    expect(expandResult.expandedDesign.pieces?.map((p) => p.id_)).toEqual(expect.arrayContaining(["piece1", "piece2", "piece5", "piece6"]));

    // Most importantly: Check that external connections to cluster-2 are preserved
    const externalConnectionsToCluster2 = expandResult.expandedDesign.connections?.filter((c) => c.connected.designId === "cluster-2" || c.connecting.designId === "cluster-2");
    expect(externalConnectionsToCluster2).toHaveLength(1); // Connection from piece2 to cluster-2

    // Verify cluster-2 still exists
    const cluster2 = expandResult.updatedKit.designs?.find((d) => d.name === "cluster-2");
    expect(cluster2).toBeDefined();
    expect(cluster2?.pieces).toHaveLength(2); // piece3, piece4
  });

  it("should preserve bidirectional external connections to nested clustered designs when expanding", () => {
    // This test creates a scenario with bidirectional connections to nested clusters
    // Original chain: piece1 - piece2 - piece3 - piece4 - piece5 - piece6 - piece7

    const extendedPieces: Piece[] = [
      ...mockDesign.pieces!,
      {
        id_: "piece5",
        type: { name: "test-type" },
        center: { x: 4, y: 0 },
      },
      {
        id_: "piece6",
        type: { name: "test-type" },
        center: { x: 5, y: 0 },
      },
      {
        id_: "piece7",
        type: { name: "test-type" },
        center: { x: 6, y: 0 },
      },
    ];

    const extendedConnections: Connection[] = [
      ...mockDesign.connections!,
      {
        connected: {
          piece: { id_: "piece4" },
          port: { id_: "port2" },
        },
        connecting: {
          piece: { id_: "piece5" },
          port: { id_: "port1" },
        },
      },
      {
        connected: {
          piece: { id_: "piece5" },
          port: { id_: "port2" },
        },
        connecting: {
          piece: { id_: "piece6" },
          port: { id_: "port1" },
        },
      },
      {
        connected: {
          piece: { id_: "piece6" },
          port: { id_: "port2" },
        },
        connecting: {
          piece: { id_: "piece7" },
          port: { id_: "port1" },
        },
      },
    ];

    const extendedDesign: Design = {
      ...mockDesign,
      pieces: extendedPieces,
      connections: extendedConnections,
    };

    const extendedKit: Kit = {
      ...mockKit,
      designs: [extendedDesign],
    };

    // Step 1: Create clustered design #1 with pieces 2, 3, 4, 5
    const clusterResult1 = clusterDesign(extendedKit, "test-design", ["piece2", "piece3", "piece4", "piece5"], "cluster-1");

    // Step 2: Create nested clustered design #2 inside cluster-1 with pieces 3, 4
    // This creates bidirectional connections: piece2 -> cluster-2 and cluster-2 -> piece5
    const clusterResult2 = clusterDesign(clusterResult1.updatedKit, "cluster-1", ["piece3", "piece4"], "cluster-2");

    // Step 3: Expand cluster-1 back to original design
    const expandResult = explodeDesign(clusterResult2.updatedKit, "cluster-1");

    // Should preserve both directions of external connections to cluster-2
    const externalConnectionsToCluster2 = expandResult.expandedDesign.connections?.filter((c) => c.connected.designId === "cluster-2" || c.connecting.designId === "cluster-2");
    expect(externalConnectionsToCluster2).toHaveLength(2); // piece2 -> cluster-2 and cluster-2 -> piece5

    // Verify the specific connections
    const incomingToCluster2 = externalConnectionsToCluster2?.find((c) => c.connecting.designId === "cluster-2");
    const outgoingFromCluster2 = externalConnectionsToCluster2?.find((c) => c.connected.designId === "cluster-2");

    expect(incomingToCluster2).toBeDefined();
    expect(outgoingFromCluster2).toBeDefined();
    expect(incomingToCluster2?.connected.piece.id_).toBe("piece2");
    expect(outgoingFromCluster2?.connecting.piece.id_).toBe("piece5");
  });
});
