/** 📸️ Mathematical snapshot schema — persistent fields only. */

export interface MathematicalSnapshot {
  /** @state artifact */
  graph: MathematicalGraph;
  /** @state artifact */
  geometry: MathematicalGeometry;
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
