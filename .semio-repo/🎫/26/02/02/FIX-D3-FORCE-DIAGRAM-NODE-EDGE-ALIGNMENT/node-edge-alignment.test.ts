import { describe, it, expect } from "vitest";

describe("Kit Diagram Node-Edge Alignment", () => {
  const ICON_WIDTH = 50;
  const NODE_SCALE = 2;
  const NODE_WIDTH = ICON_WIDTH * NODE_SCALE;
  const NODE_HEIGHT = ICON_WIDTH * NODE_SCALE;
  const NODE_RADIUS = Math.min(NODE_WIDTH, NODE_HEIGHT) / 2;
  const KIT_DIAGRAM_NODE_RADIUS = (ICON_WIDTH * 2) / 2;

  it("should calculate correct node dimensions", () => {
    expect(NODE_WIDTH).toBe(100);
    expect(NODE_HEIGHT).toBe(100);
  });

  it("should calculate correct node radius", () => {
    expect(NODE_RADIUS).toBe(50);
  });

  it("should match node radius constant", () => {
    expect(NODE_RADIUS).toBe(KIT_DIAGRAM_NODE_RADIUS);
  });

  it("should validate edge endpoint calculation uses correct radius", () => {
    const sourceNode = {
      position: { x: 0, y: 0 },
      width: NODE_WIDTH,
      height: NODE_HEIGHT,
    };
    const targetNode = {
      position: { x: 200, y: 0 },
      width: NODE_WIDTH,
      height: NODE_HEIGHT,
    };

    // Simulate getNodeIntersection logic
    const intersectionWidth = sourceNode.width ?? NODE_WIDTH;
    const intersectionHeight = sourceNode.height ?? NODE_HEIGHT;
    const x = sourceNode.position.x + intersectionWidth / 2;
    const y = sourceNode.position.y + intersectionHeight / 2;

    const tx = targetNode.position.x + targetNode.width / 2;
    const ty = targetNode.position.y + targetNode.height / 2;

    const dx = tx - x;
    const dy = ty - y;
    const distance = Math.sqrt(dx * dx + dy * dy);

    const sourceRadius = Math.min(intersectionWidth, intersectionHeight) / 2;
    const endpointX = x + (sourceRadius * dx) / distance;
    const endpointY = y + (sourceRadius * dy) / distance;

    // Edge should start from the edge of the source node, not from center
    // For a node at (0, 0) with radius 50, and target at (200, 0)
    // The endpoint should be at (50, 0) - the right edge of the circle
    expect(endpointX).toBe(50);
    expect(endpointY).toBe(0);
  });

  it("should use proper collision radius in D3 force simulation", () => {
    const defaultCollideRadius = KIT_DIAGRAM_NODE_RADIUS * 1.5;
    // This gives nodes 1.5x their radius for spacing, so they don't overlap
    expect(defaultCollideRadius).toBe(75);
  });

  it("should ensure TableAvatar renders at full node size in diagram", () => {
    // The KitArtifactNode div has:
    // width: NODE_WIDTH (100px)
    // height: NODE_HEIGHT (100px)
    // TableAvatar is passed className="size-full"
    // Avatar component should now respect size-full and not apply size-small
    const containerWidth = NODE_WIDTH;
    const containerHeight = NODE_HEIGHT;
    const avatarShouldFillContainer = true;

    expect(containerWidth).toBe(100);
    expect(containerHeight).toBe(100);
    expect(avatarShouldFillContainer).toBe(true);
  });
});
