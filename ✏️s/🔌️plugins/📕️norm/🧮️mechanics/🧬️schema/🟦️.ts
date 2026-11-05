/** 🧮️ Mechanics wire twin, mirrors `./🦀️.rs` (camelCase keys). */

export interface Pair {
  x: number;
  y: number;
}

export type LoadFrame = "global" | "local";

export type MassType = "consistent" | "lumped";

export interface Node {
  id: string;
  x: number;
  y: number;
  z: number;
}

export interface Material {
  id: string;
  e: number;
  g: number;
  rho: number;
}

export interface Section {
  id: string;
  a: number;
  iy: number;
  iz: number;
  j: number;
  iw: number;
  asy: number;
  asz: number;
}

export interface Member {
  id: string;
  nodeI: string;
  nodeJ: string;
  material: string;
  section: string;
  roll: number;
  releaseI: boolean[];
  releaseJ: boolean[];
  shearDeformable: boolean;
}

export interface Support {
  node: string;
  fixed: boolean[];
  spring: number[];
  prescribed: number[];
}

export interface NodalLoad {
  node: string;
  force: number[];
}

export interface DistributedLoad {
  member: string;
  frame: LoadFrame;
  start: number[];
  end: number[];
}

export interface TemperatureLoad {
  member: string;
  alpha: number;
  uniform: number;
  gradientY: number;
  gradientZ: number;
}

export interface LoadCase {
  id: string;
  nodal: NodalLoad[];
  distributed: DistributedLoad[];
  temperature: TemperatureLoad[];
  acceleration: number[];
}

export interface NodalMass {
  node: string;
  mass: number[];
}

export interface Model {
  nodes: Node[];
  materials: Material[];
  sections: Section[];
  members: Member[];
  supports: Support[];
  loadCases: LoadCase[];
  masses: NodalMass[];
  selfMass: boolean;
}

export interface NodeDisplacement {
  node: string;
  u: number[];
}

export interface Reaction {
  node: string;
  force: number[];
}

export interface DiagramPoint {
  x: number;
  n: number;
  vy: number;
  vz: number;
  t: number;
  my: number;
  mz: number;
}

export interface MemberForces {
  member: string;
  length: number;
  endI: number[];
  endJ: number[];
  diagram: DiagramPoint[];
}

export interface StaticResult {
  case: string;
  displacements: NodeDisplacement[];
  reactions: Reaction[];
  members: MemberForces[];
}

export interface BucklingMode {
  alphaCr: number;
  shape: NodeDisplacement[];
}

export interface BucklingResult {
  case: string;
  modes: BucklingMode[];
}

export interface ModalMode {
  omega: number;
  frequencyHz: number;
  participation: number[];
  effectiveMass: number[];
  effectiveMassRatio: number[];
  shape: NodeDisplacement[];
}

export interface ModalResult {
  totalMass: number[];
  modes: ModalMode[];
}

export interface LtbEnd {
  lateral: boolean;
  twist: boolean;
  minorRotation: boolean;
  warping: boolean;
}

export interface LtbRestraint {
  x: number;
  lateral: boolean;
  twist: boolean;
}

export interface LtbMember {
  length: number;
  e: number;
  g: number;
  iz: number;
  iw: number;
  j: number;
  polarRadiusSq: number;
  wagner: number;
  axial: number;
  endI: LtbEnd;
  endJ: LtbEnd;
  restraints: LtbRestraint[];
  moment: [number, number][];
  lineLoad: [number, number][];
  pointLoads: [number, number][];
  loadHeight: number;
  elements: number;
}

export interface LtbShapePoint {
  x: number;
  lateral: number;
  twist: number;
}

export interface LtbResult {
  alphaCr: number;
  mcr: number;
  shape: LtbShapePoint[];
}

export type SectionMethod = "finiteElement" | "thinWalled";

export interface SectionOptions {
  cellsAcrossWall: number;
  maxEdge: number;
  maxTriangles: number;
}

export interface ThinWall {
  y1: number;
  z1: number;
  y2: number;
  z2: number;
  thickness: number;
}

export interface SectionConstants {
  area: number;
  centroidY: number;
  centroidZ: number;
  iy: number;
  iz: number;
  iyz: number;
  iMajor: number;
  iMinor: number;
  j: number;
  iw: number;
  shearCentreY: number;
  shearCentreZ: number;
  aVy: number;
  aVz: number;
  wElYPos: number;
  wElYNeg: number;
  wElZPos: number;
  wElZNeg: number;
  wPlY: number;
  wPlZ: number;
  poisson: number;
  method: SectionMethod;
  triangles: number;
  nodes: number;
  maxEdge: number;
}

export interface Progress {
  stage: string;
  fraction: number;
}

export type MechanicsErrorKind = "cancelled" | "invalid" | "singular" | "unstable" | "noPositiveEigenvalue" | "noConvergence";

export interface MechanicsError {
  kind: MechanicsErrorKind;
  message: string;
  node: string;
  dof: number;
}
