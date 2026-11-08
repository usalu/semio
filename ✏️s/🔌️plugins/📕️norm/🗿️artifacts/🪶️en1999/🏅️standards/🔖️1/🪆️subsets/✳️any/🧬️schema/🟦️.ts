/** 🧬️ `En1999Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1999Artifact {
  /** @state artifact */
  annex: "En" | "De";
  /** @state artifact */
  materials: AluminiumMaterial[];
  /** @state artifact */
  sections: AluminiumSection[];
  /** @state artifact */
  members: AluminiumMember[];
  /** @state artifact */
  connections: AluminiumConnection[];
  /** @state artifact */
  fireScenarios: FireScenario[];
  /** @state artifact */
  fatigueDetails: FatigueDetail[];
  /** @state artifact */
  coldFormed: ColdFormedSheet[];
  /** @state artifact */
  shells: AluminiumShell[];
  /** @state artifact */
  placements: Placement[];
  /** @state artifact */
  weldRuns: WeldRun[];
  /** @state artifact */
  outlineVertices: SectionVertex[];
  /** @state artifact */
  fatigueCategories: FatigueCategory[];
  /** @state artifact */
  patchLoads: PatchLoad[];
  /** @state artifact */
  ringStiffeners: RingStiffener[];
  /** @state artifact */
  transverseStiffeners: TransverseStiffener[];
  /** @state artifact */
  reliabilityClass: "RC1" | "RC2" | "RC3";
  /** @state artifact */
  sheetProfiles: SheetProfile[];
  /** @state artifact */
  fireInsulations: FireInsulation[];
}

export type ProductForm = "sheet" | "extrudedProfile" | "extrudedTube" | "extrudedBar" | "drawnTube" | "forging";

export interface AluminiumMaterial {
  id: string;
  designation: string;
  productForm: ProductForm;
}

export type SectionKind = "extrudedI" | "channel" | "angle" | "rhs" | "box" | "chs" | "free";

export interface AluminiumSection {
  id: string;
  kind: SectionKind;
  height: number;
  width: number;
  flangeThickness: number;
  webThickness: number;
  outerDiameter: number;
  elements: PlateElement[];
}

export interface AluminiumMember {
  id: string;
  sectionId: string;
  materialId: string;
  length: number;
  support: "simplySupported" | "continuous" | "cantilever";
  bucklingLengthY: number;
  bucklingLengthZ: number;
  bucklingLengthT: number;
  ltbLength: number;
  c1: number;
  restrainedLtb: boolean;
  bearingLength: number;
  transverseWeld: boolean;
  transverseWeldOffset: number;
  actions: MemberAction[];
}

export type ConnectionKind = "bolted" | "welded" | "combined" | "pinned";

export interface AluminiumConnection {
  id: string;
  memberId: string;
  materialId: string;
  kind: ConnectionKind;
  actions: MemberAction[];
  bolts: BoltGroup;
  welds: WeldGroup;
  loadEccentricity: number;
  holeStagger: number;
  forceMethod: FastenerForceMethod;
}

export interface FireScenario {
  id: string;
  memberId: string;
  thetaA: number;
  durationS: number;
}

export interface FatigueDetail {
  id: string;
  memberId: string;
  detailCategory: string;
  deltaSigmaC: number;
  deltaSigmaEd: number;
  nCycles: number;
  m1: number;
  m2: number;
}

export interface ColdFormedSheet {
  id: string;
  materialId: string;
  thickness: number;
  width: number;
  span: number;
  welded: boolean;
  actions: MemberAction[];
}

export type FabricationClass = "a" | "b" | "c";

export interface AluminiumShell {
  id: string;
  materialId: string;
  radius: number;
  thickness: number;
  length: number;
  welded: boolean;
  fabricationClass: FabricationClass;
  actions: MemberAction[];
}

export interface Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export interface WeldRun {
  id: string;
  memberId: string;
  elementId: string;
  start: number;
  length: number;
  offset: number;
  throat: number;
  process: WeldProcess;
  tJunction: boolean;
  direction: WeldDirection;
  hazExtent: number;
}

export type OutlineRing = "outer" | "hole";

export interface SectionVertex {
  id: string;
  sectionId: string;
  ring: OutlineRing;
  elementId: string;
  y: number;
  z: number;
}

export type DetailType = "plain" | "buttWeld" | "filletWeld" | "bolted" | "riveted" | "other";

export interface FatigueCategory {
  id: string;
  memberId: string;
  elementId: string;
  detailCategory: string;
  detailType: DetailType;
  locationX: number;
  m1: number;
  m2: number;
}

export type PatchLoadPosition = "end" | "interior";

export interface PatchLoad {
  id: string;
  subjectId: string;
  position: PatchLoadPosition;
  x: number;
  bearingLength: number;
  webHeight: number;
  webAngle: number;
  cornerRadius: number;
  load: number;
}

export interface RingStiffener {
  id: string;
  shellId: string;
  x: number;
  depth: number;
  thickness: number;
  internal: boolean;
}

export type StiffenerSides = "one" | "two";

export interface TransverseStiffener {
  id: string;
  girderId: string;
  position: number;
  width: number;
  thickness: number;
  sides: StiffenerSides;
}

export type ActionKind = "permanent" | "imposed" | "snow" | "wind" | "temperature" | "fire";

export type ActionCategory = "selfWeight" | "office" | "snow" | "wind";

export type ActionSource = "udl" | "external";

export interface MemberAction {
  id: string;
  kind: ActionKind;
  category: ActionCategory;
  source: ActionSource;
  gKLine: number;
  qKLine: number;
  nK: number;
  vYK: number;
  vZK: number;
  mYK: number;
  mZK: number;
  torsionK: number;
  localLoadK: number;
}

export interface BoltGroup {
  material: string;
  diameter: number;
  rows: number;
  boltsPerRow: number;
  edgeDistance: number;
  pitch: number;
  gauge: number;
  plateThickness: number;
}

export interface WeldGroup {
  fillerAlloy: string;
  throat: number;
  length: number;
  betaW: number;
  hazExtent: number;
}

export type WeldProcess = "mig" | "tig";

export type WeldDirection = "longitudinal" | "transverse";

export interface PlateElement {
  id: string;
  width: number;
  thickness: number;
  outstand: boolean;
  welded: boolean;
  weldPosition: number;
  weldProcess: WeldProcess;
  weldTJunction: boolean;
  weldDirection: WeldDirection;
  edgeStiffenerDepth: number;
  edgeStiffenerThickness: number;
}

export type FastenerForceMethod = "elastic" | "instantaneousCentre";

export interface SheetProfile {
  id: string;
  sheetId: string;
  pitch: number;
  topFlange: number;
  bottomFlange: number;
  webHeight: number;
}

export interface FireInsulation {
  id: string;
  fireId: string;
  thickness: number;
  conductivity: number;
  specificHeat: number;
  density: number;
}

export const parseEn1999Artifact: NormWireReader<En1999Artifact> = normWireObject<En1999Artifact>({ annex: normWireRequired(normWireLiteral("En", "De")), materials: normWireRequired(normWireArray(normWireRef(() => parseAluminiumMaterial))), sections: normWireRequired(normWireArray(normWireRef(() => parseAluminiumSection))), members: normWireRequired(normWireArray(normWireRef(() => parseAluminiumMember))), connections: normWireRequired(normWireArray(normWireRef(() => parseAluminiumConnection))), fireScenarios: normWireRequired(normWireArray(normWireRef(() => parseFireScenario))), fatigueDetails: normWireRequired(normWireArray(normWireRef(() => parseFatigueDetail))), coldFormed: normWireRequired(normWireArray(normWireRef(() => parseColdFormedSheet))), shells: normWireRequired(normWireArray(normWireRef(() => parseAluminiumShell))), placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))), weldRuns: normWireRequired(normWireArray(normWireRef(() => parseWeldRun))), outlineVertices: normWireRequired(normWireArray(normWireRef(() => parseSectionVertex))), fatigueCategories: normWireRequired(normWireArray(normWireRef(() => parseFatigueCategory))), patchLoads: normWireRequired(normWireArray(normWireRef(() => parsePatchLoad))), ringStiffeners: normWireRequired(normWireArray(normWireRef(() => parseRingStiffener))), transverseStiffeners: normWireRequired(normWireArray(normWireRef(() => parseTransverseStiffener))), reliabilityClass: normWireRequired(normWireLiteral("RC1", "RC2", "RC3")), sheetProfiles: normWireRequired(normWireArray(normWireRef(() => parseSheetProfile))), fireInsulations: normWireRequired(normWireArray(normWireRef(() => parseFireInsulation))) });
export const parseProductForm: NormWireReader<ProductForm> = normWireLiteral("sheet", "extrudedProfile", "extrudedTube", "extrudedBar", "drawnTube", "forging");
export const parseAluminiumMaterial: NormWireReader<AluminiumMaterial> = normWireObject<AluminiumMaterial>({ id: normWireRequired(normWireString), designation: normWireRequired(normWireString), productForm: normWireRequired(normWireRef(() => parseProductForm)) });
export const parseSectionKind: NormWireReader<SectionKind> = normWireLiteral("extrudedI", "channel", "angle", "rhs", "box", "chs", "free");
export const parseAluminiumSection: NormWireReader<AluminiumSection> = normWireObject<AluminiumSection>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseSectionKind)), height: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), flangeThickness: normWireRequired(normWireNumber), webThickness: normWireRequired(normWireNumber), outerDiameter: normWireRequired(normWireNumber), elements: normWireRequired(normWireArray(normWireRef(() => parsePlateElement))) });
export const parseAluminiumMember: NormWireReader<AluminiumMember> = normWireObject<AluminiumMember>({ id: normWireRequired(normWireString), sectionId: normWireRequired(normWireString), materialId: normWireRequired(normWireString), length: normWireRequired(normWireNumber), support: normWireRequired(normWireLiteral("simplySupported", "continuous", "cantilever")), bucklingLengthY: normWireRequired(normWireNumber), bucklingLengthZ: normWireRequired(normWireNumber), bucklingLengthT: normWireRequired(normWireNumber), ltbLength: normWireRequired(normWireNumber), c1: normWireRequired(normWireNumber), restrainedLtb: normWireRequired(normWireBoolean), bearingLength: normWireRequired(normWireNumber), transverseWeld: normWireRequired(normWireBoolean), transverseWeldOffset: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseMemberAction))) });
export const parseConnectionKind: NormWireReader<ConnectionKind> = normWireLiteral("bolted", "welded", "combined", "pinned");
export const parseAluminiumConnection: NormWireReader<AluminiumConnection> = normWireObject<AluminiumConnection>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), materialId: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseConnectionKind)), actions: normWireRequired(normWireArray(normWireRef(() => parseMemberAction))), bolts: normWireRequired(normWireRef(() => parseBoltGroup)), welds: normWireRequired(normWireRef(() => parseWeldGroup)), loadEccentricity: normWireRequired(normWireNumber), holeStagger: normWireRequired(normWireNumber), forceMethod: normWireRequired(normWireRef(() => parseFastenerForceMethod)) });
export const parseFireScenario: NormWireReader<FireScenario> = normWireObject<FireScenario>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), thetaA: normWireRequired(normWireNumber), durationS: normWireRequired(normWireNumber) });
export const parseFatigueDetail: NormWireReader<FatigueDetail> = normWireObject<FatigueDetail>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), detailCategory: normWireRequired(normWireString), deltaSigmaC: normWireRequired(normWireNumber), deltaSigmaEd: normWireRequired(normWireNumber), nCycles: normWireRequired(normWireNumber), m1: normWireRequired(normWireNumber), m2: normWireRequired(normWireNumber) });
export const parseColdFormedSheet: NormWireReader<ColdFormedSheet> = normWireObject<ColdFormedSheet>({ id: normWireRequired(normWireString), materialId: normWireRequired(normWireString), thickness: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), span: normWireRequired(normWireNumber), welded: normWireRequired(normWireBoolean), actions: normWireRequired(normWireArray(normWireRef(() => parseMemberAction))) });
export const parseFabricationClass: NormWireReader<FabricationClass> = normWireLiteral("a", "b", "c");
export const parseAluminiumShell: NormWireReader<AluminiumShell> = normWireObject<AluminiumShell>({ id: normWireRequired(normWireString), materialId: normWireRequired(normWireString), radius: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), welded: normWireRequired(normWireBoolean), fabricationClass: normWireRequired(normWireRef(() => parseFabricationClass)), actions: normWireRequired(normWireArray(normWireRef(() => parseMemberAction))) });
export const parsePlacement: NormWireReader<Placement> = normWireObject<Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseWeldRun: NormWireReader<WeldRun> = normWireObject<WeldRun>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), start: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), offset: normWireRequired(normWireNumber), throat: normWireRequired(normWireNumber), process: normWireRequired(normWireRef(() => parseWeldProcess)), tJunction: normWireRequired(normWireBoolean), direction: normWireRequired(normWireRef(() => parseWeldDirection)), hazExtent: normWireRequired(normWireNumber) });
export const parseOutlineRing: NormWireReader<OutlineRing> = normWireLiteral("outer", "hole");
export const parseSectionVertex: NormWireReader<SectionVertex> = normWireObject<SectionVertex>({ id: normWireRequired(normWireString), sectionId: normWireRequired(normWireString), ring: normWireRequired(normWireRef(() => parseOutlineRing)), elementId: normWireRequired(normWireString), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber) });
export const parseDetailType: NormWireReader<DetailType> = normWireLiteral("plain", "buttWeld", "filletWeld", "bolted", "riveted", "other");
export const parseFatigueCategory: NormWireReader<FatigueCategory> = normWireObject<FatigueCategory>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), elementId: normWireRequired(normWireString), detailCategory: normWireRequired(normWireString), detailType: normWireRequired(normWireRef(() => parseDetailType)), locationX: normWireRequired(normWireNumber), m1: normWireRequired(normWireNumber), m2: normWireRequired(normWireNumber) });
export const parsePatchLoadPosition: NormWireReader<PatchLoadPosition> = normWireLiteral("end", "interior");
export const parsePatchLoad: NormWireReader<PatchLoad> = normWireObject<PatchLoad>({ id: normWireRequired(normWireString), subjectId: normWireRequired(normWireString), position: normWireRequired(normWireRef(() => parsePatchLoadPosition)), x: normWireRequired(normWireNumber), bearingLength: normWireRequired(normWireNumber), webHeight: normWireRequired(normWireNumber), webAngle: normWireRequired(normWireNumber), cornerRadius: normWireRequired(normWireNumber), load: normWireRequired(normWireNumber) });
export const parseRingStiffener: NormWireReader<RingStiffener> = normWireObject<RingStiffener>({ id: normWireRequired(normWireString), shellId: normWireRequired(normWireString), x: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), internal: normWireRequired(normWireBoolean) });
export const parseStiffenerSides: NormWireReader<StiffenerSides> = normWireLiteral("one", "two");
export const parseTransverseStiffener: NormWireReader<TransverseStiffener> = normWireObject<TransverseStiffener>({ id: normWireRequired(normWireString), girderId: normWireRequired(normWireString), position: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), sides: normWireRequired(normWireRef(() => parseStiffenerSides)) });
export const parseActionKind: NormWireReader<ActionKind> = normWireLiteral("permanent", "imposed", "snow", "wind", "temperature", "fire");
export const parseActionCategory: NormWireReader<ActionCategory> = normWireLiteral("selfWeight", "office", "snow", "wind");
export const parseActionSource: NormWireReader<ActionSource> = normWireLiteral("udl", "external");
export const parseMemberAction: NormWireReader<MemberAction> = normWireObject<MemberAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseActionKind)), category: normWireRequired(normWireRef(() => parseActionCategory)), source: normWireRequired(normWireRef(() => parseActionSource)), gKLine: normWireRequired(normWireNumber), qKLine: normWireRequired(normWireNumber), nK: normWireRequired(normWireNumber), vYK: normWireRequired(normWireNumber), vZK: normWireRequired(normWireNumber), mYK: normWireRequired(normWireNumber), mZK: normWireRequired(normWireNumber), torsionK: normWireRequired(normWireNumber), localLoadK: normWireRequired(normWireNumber) });
export const parseBoltGroup: NormWireReader<BoltGroup> = normWireObject<BoltGroup>({ material: normWireRequired(normWireString), diameter: normWireRequired(normWireNumber), rows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), boltsPerRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), edgeDistance: normWireRequired(normWireNumber), pitch: normWireRequired(normWireNumber), gauge: normWireRequired(normWireNumber), plateThickness: normWireRequired(normWireNumber) });
export const parseWeldGroup: NormWireReader<WeldGroup> = normWireObject<WeldGroup>({ fillerAlloy: normWireRequired(normWireString), throat: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), betaW: normWireRequired(normWireNumber), hazExtent: normWireRequired(normWireNumber) });
export const parseWeldProcess: NormWireReader<WeldProcess> = normWireLiteral("mig", "tig");
export const parseWeldDirection: NormWireReader<WeldDirection> = normWireLiteral("longitudinal", "transverse");
export const parsePlateElement: NormWireReader<PlateElement> = normWireObject<PlateElement>({ id: normWireRequired(normWireString), width: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), outstand: normWireRequired(normWireBoolean), welded: normWireRequired(normWireBoolean), weldPosition: normWireRequired(normWireNumber), weldProcess: normWireRequired(normWireRef(() => parseWeldProcess)), weldTJunction: normWireRequired(normWireBoolean), weldDirection: normWireRequired(normWireRef(() => parseWeldDirection)), edgeStiffenerDepth: normWireRequired(normWireNumber), edgeStiffenerThickness: normWireRequired(normWireNumber) });
export const parseFastenerForceMethod: NormWireReader<FastenerForceMethod> = normWireLiteral("elastic", "instantaneousCentre");
export const parseSheetProfile: NormWireReader<SheetProfile> = normWireObject<SheetProfile>({ id: normWireRequired(normWireString), sheetId: normWireRequired(normWireString), pitch: normWireRequired(normWireNumber), topFlange: normWireRequired(normWireNumber), bottomFlange: normWireRequired(normWireNumber), webHeight: normWireRequired(normWireNumber) });
export const parseFireInsulation: NormWireReader<FireInsulation> = normWireObject<FireInsulation>({ id: normWireRequired(normWireString), fireId: normWireRequired(normWireString), thickness: normWireRequired(normWireNumber), conductivity: normWireRequired(normWireNumber), specificHeat: normWireRequired(normWireNumber), density: normWireRequired(normWireNumber) });
