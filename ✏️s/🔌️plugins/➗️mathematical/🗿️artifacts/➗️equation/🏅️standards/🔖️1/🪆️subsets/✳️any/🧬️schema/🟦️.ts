/** 🧬️ Equation artifact schema — every field with its state class. */

export interface EquationArtifact {
  /** @state artifact */
  graph: EquationGraph;
  /** @state artifact */
  geometry: EquationGeometry;
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
  locale: string;
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
