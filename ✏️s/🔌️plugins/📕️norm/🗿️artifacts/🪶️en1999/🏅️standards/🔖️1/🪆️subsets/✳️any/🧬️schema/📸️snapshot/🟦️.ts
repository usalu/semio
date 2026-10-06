/** 📸️ `En1999Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1999Snapshot {
  /** @state artifact */
  annex: AnnexChoice;
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

export interface AluminiumMember {
  id: string;
  sectionId: string;
  materialId: string;
  length: number;
  support: SupportCondition;
  bucklingLengthY: number;
  bucklingLengthZ: number;
  bucklingLengthT: number;
  ltbLength: number;
  c1: number;
  restrainedLtb: boolean;
  actions: MemberAction[];
}

export type AnnexChoice = "En" | "De";

export interface AluminiumSection {
  id: string;
  kind: string;
  height: number;
  width: number;
  flangeThickness: number;
  webThickness: number;
  outerDiameter: number;
  elements: PlateElement[];
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

export interface AluminiumConnection {
  id: string;
  memberId: string;
  materialId: string;
  kind: string;
  actions: MemberAction[];
  bolts: BoltGroup;
  welds: WeldGroup;
}

export interface FireScenario {
  id: string;
  memberId: string;
  thetaA: number;
  durationS: number;
}

export interface AluminiumMaterial {
  id: string;
  designation: string;
}

export interface AluminiumShell {
  id: string;
  materialId: string;
  radius: number;
  thickness: number;
  length: number;
  actions: MemberAction[];
}

export interface MemberAction {
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
}

export interface PlateElement {
  id: string;
  width: number;
  thickness: number;
  outstand: boolean;
  welded: boolean;
  weldPosition: number;
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

export type SupportCondition = "simplySupported" | "continuous" | "cantilever";

export const parseEn1999Snapshot: NormWireReader<En1999Snapshot> = normWireObject<En1999Snapshot>({ annex: normWireRequired(normWireRef(() => parseAnnexChoice)), materials: normWireRequired(normWireArray(normWireRef(() => parseAluminiumMaterial))), sections: normWireRequired(normWireArray(normWireRef(() => parseAluminiumSection))), members: normWireRequired(normWireArray(normWireRef(() => parseAluminiumMember))), connections: normWireRequired(normWireArray(normWireRef(() => parseAluminiumConnection))), fireScenarios: normWireRequired(normWireArray(normWireRef(() => parseFireScenario))), fatigueDetails: normWireRequired(normWireArray(normWireRef(() => parseFatigueDetail))), coldFormed: normWireRequired(normWireArray(normWireRef(() => parseColdFormedSheet))), shells: normWireRequired(normWireArray(normWireRef(() => parseAluminiumShell))) });
export const parseColdFormedSheet: NormWireReader<ColdFormedSheet> = normWireObject<ColdFormedSheet>({ id: normWireRequired(normWireString), materialId: normWireRequired(normWireString), thickness: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), span: normWireRequired(normWireNumber), welded: normWireRequired(normWireBoolean), actions: normWireRequired(normWireArray(normWireRef(() => parseMemberAction))) });
export const parseAluminiumMember: NormWireReader<AluminiumMember> = normWireObject<AluminiumMember>({ id: normWireRequired(normWireString), sectionId: normWireRequired(normWireString), materialId: normWireRequired(normWireString), length: normWireRequired(normWireNumber), support: normWireRequired(normWireRef(() => parseSupportCondition)), bucklingLengthY: normWireRequired(normWireNumber), bucklingLengthZ: normWireRequired(normWireNumber), bucklingLengthT: normWireRequired(normWireNumber), ltbLength: normWireRequired(normWireNumber), c1: normWireRequired(normWireNumber), restrainedLtb: normWireRequired(normWireBoolean), actions: normWireRequired(normWireArray(normWireRef(() => parseMemberAction))) });
export const parseAnnexChoice: NormWireReader<AnnexChoice> = normWireLiteral("En", "De");
export const parseAluminiumSection: NormWireReader<AluminiumSection> = normWireObject<AluminiumSection>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), height: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), flangeThickness: normWireRequired(normWireNumber), webThickness: normWireRequired(normWireNumber), outerDiameter: normWireRequired(normWireNumber), elements: normWireRequired(normWireArray(normWireRef(() => parsePlateElement))) });
export const parseFatigueDetail: NormWireReader<FatigueDetail> = normWireObject<FatigueDetail>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), detailCategory: normWireRequired(normWireString), deltaSigmaC: normWireRequired(normWireNumber), deltaSigmaEd: normWireRequired(normWireNumber), nCycles: normWireRequired(normWireNumber), m1: normWireRequired(normWireNumber), m2: normWireRequired(normWireNumber) });
export const parseAluminiumConnection: NormWireReader<AluminiumConnection> = normWireObject<AluminiumConnection>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), materialId: normWireRequired(normWireString), kind: normWireRequired(normWireString), actions: normWireRequired(normWireArray(normWireRef(() => parseMemberAction))), bolts: normWireRequired(normWireRef(() => parseBoltGroup)), welds: normWireRequired(normWireRef(() => parseWeldGroup)) });
export const parseFireScenario: NormWireReader<FireScenario> = normWireObject<FireScenario>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), thetaA: normWireRequired(normWireNumber), durationS: normWireRequired(normWireNumber) });
export const parseAluminiumMaterial: NormWireReader<AluminiumMaterial> = normWireObject<AluminiumMaterial>({ id: normWireRequired(normWireString), designation: normWireRequired(normWireString) });
export const parseAluminiumShell: NormWireReader<AluminiumShell> = normWireObject<AluminiumShell>({ id: normWireRequired(normWireString), materialId: normWireRequired(normWireString), radius: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseMemberAction))) });
export const parseMemberAction: NormWireReader<MemberAction> = normWireObject<MemberAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), category: normWireRequired(normWireString), source: normWireRequired(normWireString), gKLine: normWireRequired(normWireNumber), qKLine: normWireRequired(normWireNumber), nK: normWireRequired(normWireNumber), vYK: normWireRequired(normWireNumber), vZK: normWireRequired(normWireNumber), mYK: normWireRequired(normWireNumber), mZK: normWireRequired(normWireNumber) });
export const parsePlateElement: NormWireReader<PlateElement> = normWireObject<PlateElement>({ id: normWireRequired(normWireString), width: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), outstand: normWireRequired(normWireBoolean), welded: normWireRequired(normWireBoolean), weldPosition: normWireRequired(normWireNumber) });
export const parseBoltGroup: NormWireReader<BoltGroup> = normWireObject<BoltGroup>({ material: normWireRequired(normWireString), diameter: normWireRequired(normWireNumber), rows: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), boltsPerRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), edgeDistance: normWireRequired(normWireNumber), pitch: normWireRequired(normWireNumber), gauge: normWireRequired(normWireNumber), plateThickness: normWireRequired(normWireNumber) });
export const parseWeldGroup: NormWireReader<WeldGroup> = normWireObject<WeldGroup>({ fillerAlloy: normWireRequired(normWireString), throat: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), betaW: normWireRequired(normWireNumber), hazExtent: normWireRequired(normWireNumber) });
export const parseSupportCondition: NormWireReader<SupportCondition> = normWireLiteral("simplySupported", "continuous", "cantilever");
