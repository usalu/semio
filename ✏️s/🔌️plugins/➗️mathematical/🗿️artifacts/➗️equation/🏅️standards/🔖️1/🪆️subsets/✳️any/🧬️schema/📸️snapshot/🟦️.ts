/** 📸️ Equation snapshot schema — artifact-lane fields only. */

export interface EquationSnapshot {
  /** @state artifact */
  graph: EquationGraph;
  /** @state artifact */
  geometry: EquationGeometry;
}

export interface EquationGraph {
  directed: boolean;
  nodes: EquationNode[];
  edges: EquationEdge[];
  algorithm: string;
  algorithmSeed?: string;
}

export interface EquationNode {
  id: string;
  label: string;
  x: number;
  y: number;
}

export interface EquationEdge {
  id: string;
  source: string;
  target: string;
}

export interface EquationPoint {
  x: number;
  y: number;
}

export interface EquationGeometry {
  points: EquationPoint[];
}
