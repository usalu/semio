/** 🔺️ `En1993Diff` wire twin: the sparse delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type BridgeFatigue, type ColdFormedMember, type CraneRunway, type FatigueDetail, type FireExposure, type LoadCase, type MemberAction, type PlatedPanel, type SiloShell, type SteelJoint, type SteelMaterial, type SteelMember, type SteelPile, type SteelSection, type TensionComponent, type TowerLeg, parseBridgeFatigue, parseColdFormedMember, parseCraneRunway, parseFatigueDetail, parseFireExposure, parseLoadCase, parseMemberAction, parsePlatedPanel, parseSiloShell, parseSteelJoint, parseSteelMaterial, parseSteelMember, parseSteelPile, parseSteelSection, parseTensionComponent, parseTowerLeg } from "../📸️snapshot/🟦️.ts";

export type En1993RowOp = "Insert" | "Remove" | "Replace";
export interface En1993MaterialEdit { op: En1993RowOp; index: number; id: string; value: SteelMaterial | null; }
export interface En1993SectionEdit { op: En1993RowOp; index: number; id: string; value: SteelSection | null; }
export interface En1993MemberEdit { op: En1993RowOp; index: number; id: string; value: SteelMember | null; }
export interface En1993LoadCaseEdit { op: En1993RowOp; index: number; id: string; value: LoadCase | null; }
export interface En1993MemberActionEdit { op: En1993RowOp; index: number; id: string; value: MemberAction | null; }
export interface En1993JointEdit { op: En1993RowOp; index: number; id: string; value: SteelJoint | null; }
export interface En1993FatigueDetailEdit { op: En1993RowOp; index: number; id: string; value: FatigueDetail | null; }
export interface En1993FireExposureEdit { op: En1993RowOp; index: number; id: string; value: FireExposure | null; }
export interface En1993ColdFormedMemberEdit { op: En1993RowOp; index: number; id: string; value: ColdFormedMember | null; }
export interface En1993PlatedPanelEdit { op: En1993RowOp; index: number; id: string; value: PlatedPanel | null; }
export interface En1993SiloShellEdit { op: En1993RowOp; index: number; id: string; value: SiloShell | null; }
export interface En1993TensionComponentEdit { op: En1993RowOp; index: number; id: string; value: TensionComponent | null; }
export interface En1993BridgeFatigueEdit { op: En1993RowOp; index: number; id: string; value: BridgeFatigue | null; }
export interface En1993TowerLegEdit { op: En1993RowOp; index: number; id: string; value: TowerLeg | null; }
export interface En1993PileEdit { op: En1993RowOp; index: number; id: string; value: SteelPile | null; }
export interface En1993CraneRunwayEdit { op: En1993RowOp; index: number; id: string; value: CraneRunway | null; }

export interface En1993Diff {
  /** @state artifact */
  annex: ("En" | "De") | null;
  /** @state artifact */
  materials: En1993MaterialEdit[];
  /** @state artifact */
  sections: En1993SectionEdit[];
  /** @state artifact */
  members: En1993MemberEdit[];
  /** @state artifact */
  loadCases: En1993LoadCaseEdit[];
  /** @state artifact */
  memberActions: En1993MemberActionEdit[];
  /** @state artifact */
  joints: En1993JointEdit[];
  /** @state artifact */
  fatigueDetails: En1993FatigueDetailEdit[];
  /** @state artifact */
  fireExposures: En1993FireExposureEdit[];
  /** @state artifact */
  coldFormedMembers: En1993ColdFormedMemberEdit[];
  /** @state artifact */
  platedPanels: En1993PlatedPanelEdit[];
  /** @state artifact */
  siloShells: En1993SiloShellEdit[];
  /** @state artifact */
  tensionComponents: En1993TensionComponentEdit[];
  /** @state artifact */
  bridgeFatigue: En1993BridgeFatigueEdit[];
  /** @state artifact */
  towerLegs: En1993TowerLegEdit[];
  /** @state artifact */
  piles: En1993PileEdit[];
  /** @state artifact */
  craneRunways: En1993CraneRunwayEdit[];
}

export const parseEn1993RowOp: NormWireReader<En1993RowOp> = normWireLiteral("Insert", "Remove", "Replace");
export const parseEn1993MaterialEdit: NormWireReader<En1993MaterialEdit> = normWireObject<En1993MaterialEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseSteelMaterial)) });
export const parseEn1993SectionEdit: NormWireReader<En1993SectionEdit> = normWireObject<En1993SectionEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseSteelSection)) });
export const parseEn1993MemberEdit: NormWireReader<En1993MemberEdit> = normWireObject<En1993MemberEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseSteelMember)) });
export const parseEn1993LoadCaseEdit: NormWireReader<En1993LoadCaseEdit> = normWireObject<En1993LoadCaseEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseLoadCase)) });
export const parseEn1993MemberActionEdit: NormWireReader<En1993MemberActionEdit> = normWireObject<En1993MemberActionEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseMemberAction)) });
export const parseEn1993JointEdit: NormWireReader<En1993JointEdit> = normWireObject<En1993JointEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseSteelJoint)) });
export const parseEn1993FatigueDetailEdit: NormWireReader<En1993FatigueDetailEdit> = normWireObject<En1993FatigueDetailEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseFatigueDetail)) });
export const parseEn1993FireExposureEdit: NormWireReader<En1993FireExposureEdit> = normWireObject<En1993FireExposureEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseFireExposure)) });
export const parseEn1993ColdFormedMemberEdit: NormWireReader<En1993ColdFormedMemberEdit> = normWireObject<En1993ColdFormedMemberEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseColdFormedMember)) });
export const parseEn1993PlatedPanelEdit: NormWireReader<En1993PlatedPanelEdit> = normWireObject<En1993PlatedPanelEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parsePlatedPanel)) });
export const parseEn1993SiloShellEdit: NormWireReader<En1993SiloShellEdit> = normWireObject<En1993SiloShellEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseSiloShell)) });
export const parseEn1993TensionComponentEdit: NormWireReader<En1993TensionComponentEdit> = normWireObject<En1993TensionComponentEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseTensionComponent)) });
export const parseEn1993BridgeFatigueEdit: NormWireReader<En1993BridgeFatigueEdit> = normWireObject<En1993BridgeFatigueEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseBridgeFatigue)) });
export const parseEn1993TowerLegEdit: NormWireReader<En1993TowerLegEdit> = normWireObject<En1993TowerLegEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseTowerLeg)) });
export const parseEn1993PileEdit: NormWireReader<En1993PileEdit> = normWireObject<En1993PileEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseSteelPile)) });
export const parseEn1993CraneRunwayEdit: NormWireReader<En1993CraneRunwayEdit> = normWireObject<En1993CraneRunwayEdit>({ op: normWireRequired(parseEn1993RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseCraneRunway)) });
export const parseEn1993Diff: NormWireReader<En1993Diff> = normWireObject<En1993Diff>({ annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), materials: normWireDefault(normWireArray(parseEn1993MaterialEdit), () => []), sections: normWireDefault(normWireArray(parseEn1993SectionEdit), () => []), members: normWireDefault(normWireArray(parseEn1993MemberEdit), () => []), loadCases: normWireDefault(normWireArray(parseEn1993LoadCaseEdit), () => []), memberActions: normWireDefault(normWireArray(parseEn1993MemberActionEdit), () => []), joints: normWireDefault(normWireArray(parseEn1993JointEdit), () => []), fatigueDetails: normWireDefault(normWireArray(parseEn1993FatigueDetailEdit), () => []), fireExposures: normWireDefault(normWireArray(parseEn1993FireExposureEdit), () => []), coldFormedMembers: normWireDefault(normWireArray(parseEn1993ColdFormedMemberEdit), () => []), platedPanels: normWireDefault(normWireArray(parseEn1993PlatedPanelEdit), () => []), siloShells: normWireDefault(normWireArray(parseEn1993SiloShellEdit), () => []), tensionComponents: normWireDefault(normWireArray(parseEn1993TensionComponentEdit), () => []), bridgeFatigue: normWireDefault(normWireArray(parseEn1993BridgeFatigueEdit), () => []), towerLegs: normWireDefault(normWireArray(parseEn1993TowerLegEdit), () => []), piles: normWireDefault(normWireArray(parseEn1993PileEdit), () => []), craneRunways: normWireDefault(normWireArray(parseEn1993CraneRunwayEdit), () => []) });
