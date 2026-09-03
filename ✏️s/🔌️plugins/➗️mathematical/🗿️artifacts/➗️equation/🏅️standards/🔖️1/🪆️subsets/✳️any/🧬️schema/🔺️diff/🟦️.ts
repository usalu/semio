/** 🔺️ Equation diff schema — sparse field delta. */

export interface EquationDiff {
  /** @state artifact */
  artifact?: EquationArtifact;
  /** @state artifact */
  graph?: EquationGraph;
  /** @state artifact */
  geometry?: EquationGeometry;
  /** @state config */
  cameraX?: number;
  /** @state config */
  cameraY?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  locale?: string;
}

export interface EquationArtifact {
  graph: EquationGraph;
  geometry: EquationGeometry;
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
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
