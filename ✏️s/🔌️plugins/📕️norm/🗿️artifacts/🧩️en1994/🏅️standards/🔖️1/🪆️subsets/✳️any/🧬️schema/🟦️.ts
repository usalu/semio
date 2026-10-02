/** 🧬️ `En1994Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1994Artifact {
  /** @state artifact */
  annex: AnnexChoice;
  /** @state artifact */
  structureKind: string;
  /** @state artifact */
  steelFYPa: number;
  /** @state artifact */
  beams: CompositeBeam[];
  /** @state artifact */
  columns: CompositeColumn[];
  /** @state artifact */
  slabs: CompositeSlab[];
  /** @state artifact */
  fireRating: string;
  /** @state artifact */
  insulationThicknessM: number;
  /** @state artifact */
  fatigueDetail: string;
}

export interface CharacteristicAction {
  id: string;
  kind: string;
  category: string;
  stage: string;
  qAreaPa: number;
  fKN: number;
  deltaSigmaKPa: number;
  deltaTauKPa: number;
}

export interface SteelSection {
  designation: string;
  heightM: number;
  widthM: number;
  twM: number;
  tfM: number;
  aM2: number;
  wPlYM3: number;
  iYM4: number;
  aVM2: number;
}

export interface ProfiledSheeting {
  profile: string;
  heightM: number;
  ribWidthM: number;
  thicknessM: number;
  ribsParallelToBeam: boolean;
}

export interface HeadedStuds {
  diameterM: number;
  heightM: number;
  fUPa: number;
  countPerRib: number;
  spacingM: number;
  totalCount: number;
}

export interface CompositeBeam {
  id: string;
  spanM: number;
  spacingM: number;
  support: string;
  construction: string;
  steel: SteelSection;
  slabThicknessM: number;
  concreteFCkPa: number;
  concreteECmPa: number;
  sheeting: ProfiledSheeting;
  studs: HeadedStuds;
  transverseAsM2PerM: number;
  ltbLengthM: number;
  asHoggingM2PerM: number;
  barSpacingM: number;
  wkLimitM: number;
  nCycles: number;
  actions: CharacteristicAction[];
}

export interface CompositeColumn {
  id: string;
  kind: string;
  lengthM: number;
  outerSizeM: number;
  wallThicknessM: number;
  steelAM2: number;
  steelFYPa: number;
  concreteAM2: number;
  concreteFCkPa: number;
  reinforcementAsM2: number;
  reinforcementFYkPa: number;
  iM4: number;
  bucklingCurve: string;
  actions: ColumnAction[];
}

export interface CompositeSlab {
  id: string;
  spanM: number;
  support: string;
  sheeting: ProfiledSheeting;
  concreteThicknessM: number;
  fCkPa: number;
  mFactor: number;
  kFactor: number;
  asM2PerM: number;
  actions: CharacteristicAction[];
}

export interface ColumnAction {
  id: string;
  kind: string;
  category: string;
  stage: string;
  nKN: number;
  mKNm: number;
}

export type AnnexChoice = "En" | "De";

export const parseEn1994Artifact: NormWireReader<En1994Artifact> = normWireObject<En1994Artifact>({ annex: normWireRequired(normWireRef(() => parseAnnexChoice)), structureKind: normWireRequired(normWireString), steelFYPa: normWireRequired(normWireNumber), beams: normWireRequired(normWireArray(normWireRef(() => parseCompositeBeam))), columns: normWireRequired(normWireArray(normWireRef(() => parseCompositeColumn))), slabs: normWireRequired(normWireArray(normWireRef(() => parseCompositeSlab))), fireRating: normWireRequired(normWireString), insulationThicknessM: normWireRequired(normWireNumber), fatigueDetail: normWireRequired(normWireString) });
export const parseCharacteristicAction: NormWireReader<CharacteristicAction> = normWireObject<CharacteristicAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), category: normWireRequired(normWireString), stage: normWireRequired(normWireString), qAreaPa: normWireRequired(normWireNumber), fKN: normWireRequired(normWireNumber), deltaSigmaKPa: normWireRequired(normWireNumber), deltaTauKPa: normWireRequired(normWireNumber) });
export const parseSteelSection: NormWireReader<SteelSection> = normWireObject<SteelSection>({ designation: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), widthM: normWireRequired(normWireNumber), twM: normWireRequired(normWireNumber), tfM: normWireRequired(normWireNumber), aM2: normWireRequired(normWireNumber), wPlYM3: normWireRequired(normWireNumber), iYM4: normWireRequired(normWireNumber), aVM2: normWireRequired(normWireNumber) });
export const parseProfiledSheeting: NormWireReader<ProfiledSheeting> = normWireObject<ProfiledSheeting>({ profile: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), ribWidthM: normWireRequired(normWireNumber), thicknessM: normWireRequired(normWireNumber), ribsParallelToBeam: normWireRequired(normWireBoolean) });
export const parseHeadedStuds: NormWireReader<HeadedStuds> = normWireObject<HeadedStuds>({ diameterM: normWireRequired(normWireNumber), heightM: normWireRequired(normWireNumber), fUPa: normWireRequired(normWireNumber), countPerRib: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), spacingM: normWireRequired(normWireNumber), totalCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseCompositeBeam: NormWireReader<CompositeBeam> = normWireObject<CompositeBeam>({ id: normWireRequired(normWireString), spanM: normWireRequired(normWireNumber), spacingM: normWireRequired(normWireNumber), support: normWireRequired(normWireString), construction: normWireRequired(normWireString), steel: normWireRequired(normWireRef(() => parseSteelSection)), slabThicknessM: normWireRequired(normWireNumber), concreteFCkPa: normWireRequired(normWireNumber), concreteECmPa: normWireRequired(normWireNumber), sheeting: normWireRequired(normWireRef(() => parseProfiledSheeting)), studs: normWireRequired(normWireRef(() => parseHeadedStuds)), transverseAsM2PerM: normWireRequired(normWireNumber), ltbLengthM: normWireRequired(normWireNumber), asHoggingM2PerM: normWireRequired(normWireNumber), barSpacingM: normWireRequired(normWireNumber), wkLimitM: normWireRequired(normWireNumber), nCycles: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseCharacteristicAction))) });
export const parseCompositeColumn: NormWireReader<CompositeColumn> = normWireObject<CompositeColumn>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), lengthM: normWireRequired(normWireNumber), outerSizeM: normWireRequired(normWireNumber), wallThicknessM: normWireRequired(normWireNumber), steelAM2: normWireRequired(normWireNumber), steelFYPa: normWireRequired(normWireNumber), concreteAM2: normWireRequired(normWireNumber), concreteFCkPa: normWireRequired(normWireNumber), reinforcementAsM2: normWireRequired(normWireNumber), reinforcementFYkPa: normWireRequired(normWireNumber), iM4: normWireRequired(normWireNumber), bucklingCurve: normWireRequired(normWireString), actions: normWireRequired(normWireArray(normWireRef(() => parseColumnAction))) });
export const parseCompositeSlab: NormWireReader<CompositeSlab> = normWireObject<CompositeSlab>({ id: normWireRequired(normWireString), spanM: normWireRequired(normWireNumber), support: normWireRequired(normWireString), sheeting: normWireRequired(normWireRef(() => parseProfiledSheeting)), concreteThicknessM: normWireRequired(normWireNumber), fCkPa: normWireRequired(normWireNumber), mFactor: normWireRequired(normWireNumber), kFactor: normWireRequired(normWireNumber), asM2PerM: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseCharacteristicAction))) });
export const parseColumnAction: NormWireReader<ColumnAction> = normWireObject<ColumnAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), category: normWireRequired(normWireString), stage: normWireRequired(normWireString), nKN: normWireRequired(normWireNumber), mKNm: normWireRequired(normWireNumber) });
export const parseAnnexChoice: NormWireReader<AnnexChoice> = normWireLiteral("En", "De");
