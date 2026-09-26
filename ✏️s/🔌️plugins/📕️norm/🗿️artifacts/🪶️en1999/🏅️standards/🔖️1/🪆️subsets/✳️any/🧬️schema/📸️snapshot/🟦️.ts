/** 📸️ EN 1999 snapshot wire types — mirrors `📸️snapshot/🔣️.json` (SI camelCase). */

export type AnnexChoice = "en" | "de";

export type AluminiumMaterial = {
  id: string;
  designation: string;
};

export type PlateElement = {
  id: string;
  width: number;
  thickness: number;
  outstand: boolean;
  welded: boolean;
  weldPosition: number;
};

/** Discriminated by `kind`: CHS/tube use `outerDiameter`+`webThickness`; I/H/channel/angle/rhs/box use height/width/flange/web. */
export type AluminiumSection = {
  id: string;
  kind: string;
  height: number;
  width: number;
  flangeThickness: number;
  webThickness: number;
  outerDiameter: number;
  elements: PlateElement[];
};

export type MemberAction = {
  id: string;
  kind: string;
  category: string;
  source: string;
  gKLine: number;
  qKLine: number;
  nK: number;
  vYK: number;
  vZK: number;
  mYK: number;
  mZK: number;
};

export type AluminiumMember = {
  id: string;
  sectionId: string;
  materialId: string;
  length: number;
  support: string;
  bucklingLengthY: number;
  bucklingLengthZ: number;
  bucklingLengthT: number;
  ltbLength: number;
  c1: number;
  restrainedLtb: boolean;
  actions: MemberAction[];
};

export type BoltGroup = {
  material: string;
  diameter: number;
  rows: number;
  boltsPerRow: number;
  edgeDistance: number;
  pitch: number;
  gauge: number;
  plateThickness: number;
};

export type WeldGroup = {
  fillerAlloy: string;
  throat: number;
  length: number;
  betaW: number;
  hazExtent: number;
};

export type AluminiumConnection = {
  id: string;
  memberId: string;
  materialId: string;
  kind: string;
  actions: MemberAction[];
  bolts: BoltGroup;
  welds: WeldGroup;
};

export type FireScenario = {
  id: string;
  memberId: string;
  thetaA: number;
  durationS: number;
};

export type FatigueDetail = {
  id: string;
  memberId: string;
  detailCategory: string;
  deltaSigmaC: number;
  deltaSigmaEd: number;
  nCycles: number;
  m1: number;
  m2: number;
};

export type ColdFormedSheet = {
  id: string;
  materialId: string;
  thickness: number;
  width: number;
  span: number;
  welded: boolean;
  actions: MemberAction[];
};

export type AluminiumShell = {
  id: string;
  materialId: string;
  radius: number;
  thickness: number;
  length: number;
  actions: MemberAction[];
};

export type En1999Snapshot = {
  annex: AnnexChoice | string;
  materials: AluminiumMaterial[];
  sections: AluminiumSection[];
  members: AluminiumMember[];
  connections: AluminiumConnection[];
  fireScenarios: FireScenario[];
  fatigueDetails: FatigueDetail[];
  coldFormed: ColdFormedSheet[];
  shells: AluminiumShell[];
};
