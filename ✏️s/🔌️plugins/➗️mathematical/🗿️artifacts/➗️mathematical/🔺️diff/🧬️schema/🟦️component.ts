/** 🔺️ Mathematical diff schema — sparse field delta. */

export interface MathematicalDiff {
  /** @state persistent */
  artifact?: MathematicalArtifact;
  /** @state persistent */
  graph?: MathematicalGraph;
  /** @state persistent */
  geometry?: MathematicalGeometry;
  /** @state local-ui */
  cameraX?: number;
  /** @state local-ui */
  cameraY?: number;
  /** @state local-ui */
  cameraZoom?: number;
  /** @state local-ui */
  locale?: string;
}

export interface MathematicalArtifact {
  graph: MathematicalGraph;
  geometry: MathematicalGeometry;
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  locale: string;
}

export interface MathematicalGraph {
  directed: boolean;
  nodes: MathematicalNode[];
  edges: MathematicalEdge[];
  algorithm: string;
  algorithmSeed?: string;
}

export interface MathematicalNode {
  id: string;
  label: string;
  x: number;
  y: number;
}

export interface MathematicalEdge {
  id: string;
  source: string;
  target: string;
}

export interface MathematicalPoint {
  x: number;
  y: number;
}

export interface MathematicalGeometry {
  points: MathematicalPoint[];
}
