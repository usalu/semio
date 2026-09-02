/** 🧬️ Mathematical artifact schema — every field with its state class. */

export interface MathematicalArtifact {
  /** @state artifact */
  graph: MathematicalGraph;
  /** @state artifact */
  geometry: MathematicalGeometry;
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
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
