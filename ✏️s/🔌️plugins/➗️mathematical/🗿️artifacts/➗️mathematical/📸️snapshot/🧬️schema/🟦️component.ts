/** 📸️ Mathematical snapshot schema — persistent fields only. */

export interface MathematicalSnapshot {
  /** @state persistent */
  graph: MathematicalGraph;
  /** @state persistent */
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
