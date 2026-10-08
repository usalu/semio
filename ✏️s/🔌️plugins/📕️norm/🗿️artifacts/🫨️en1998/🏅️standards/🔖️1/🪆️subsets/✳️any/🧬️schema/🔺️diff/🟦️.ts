/** 🔺️ `En1998Diff` wire twin: the sparse delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1998Assessment, type En1998Bridge, type En1998Building, type En1998Foundation, type En1998Member, type En1998RetainingWall, type En1998Silo, type En1998Site, type En1998Storey, type En1998System, type En1998Tank, type En1998Tower, parseEn1998Assessment, parseEn1998Bridge, parseEn1998Building, parseEn1998Foundation, parseEn1998Member, parseEn1998RetainingWall, parseEn1998Silo, parseEn1998Site, parseEn1998Storey, parseEn1998System, parseEn1998Tank, parseEn1998Tower } from "../📸️snapshot/🟦️.ts";

export type En1998RowOp = "Insert" | "Remove" | "Replace" | "Patch";
export interface En1998BuildingEdit { op: En1998RowOp; index: number; id: string; value: En1998Building | null; patch: En1998BuildingPatch | null; }
export interface En1998BridgeEdit { op: En1998RowOp; index: number; id: string; value: En1998Bridge | null; patch: En1998BridgePatch | null; }
export interface En1998AssessmentEdit { op: En1998RowOp; index: number; id: string; value: En1998Assessment | null; patch: En1998AssessmentPatch | null; }
export interface En1998SiloEdit { op: En1998RowOp; index: number; id: string; value: En1998Silo | null; }
export interface En1998TankEdit { op: En1998RowOp; index: number; id: string; value: En1998Tank | null; }
export interface En1998FoundationEdit { op: En1998RowOp; index: number; id: string; value: En1998Foundation | null; }
export interface En1998RetainingWallEdit { op: En1998RowOp; index: number; id: string; value: En1998RetainingWall | null; }
export interface En1998TowerEdit { op: En1998RowOp; index: number; id: string; value: En1998Tower | null; patch: En1998TowerPatch | null; }
export interface En1998SystemEdit { op: En1998RowOp; index: number; id: string; value: En1998System | null; patch: En1998SystemPatch | null; }
export interface En1998StoreyEdit { op: En1998RowOp; index: number; id: string; value: En1998Storey | null; patch: En1998StoreyPatch | null; }
export interface En1998MemberEdit { op: En1998RowOp; index: number; id: string; value: En1998Member | null; patch: En1998MemberPatch | null; }
export interface En1998BuildingPatch { planRegular: boolean | null; elevationRegular: boolean | null; masonryWallAreaRatio: number | null; systems: En1998SystemEdit[]; storeys: En1998StoreyEdit[]; members: En1998MemberEdit[]; }
export interface En1998BridgePatch { vRdN: number | null; }
export interface En1998AssessmentPatch { rKN: number | null; }
export interface En1998TowerPatch { mRdNm: number | null; }
export interface En1998SystemPatch { baseShearResistanceN: number | null; }
export interface En1998StoreyPatch { permanentGkN: number | null; stiffnessX: number | null; driftXM: number | null; }
export interface En1998MemberPatch { detailingCompatibleWithQ: boolean | null; }

export interface En1998Diff {
  /** @state artifact */
  annex: string | null;
  /** @state artifact */
  site: En1998Site | null;
  /** @state artifact */
  buildings: En1998BuildingEdit[];
  /** @state artifact */
  bridges: En1998BridgeEdit[];
  /** @state artifact */
  assessments: En1998AssessmentEdit[];
  /** @state artifact */
  silos: En1998SiloEdit[];
  /** @state artifact */
  tanks: En1998TankEdit[];
  /** @state artifact */
  foundations: En1998FoundationEdit[];
  /** @state artifact */
  retainingWalls: En1998RetainingWallEdit[];
  /** @state artifact */
  towers: En1998TowerEdit[];
}

export const parseEn1998RowOp: NormWireReader<En1998RowOp> = normWireLiteral("Insert", "Remove", "Replace", "Patch");
export const parseEn1998BuildingEdit: NormWireReader<En1998BuildingEdit> = normWireObject<En1998BuildingEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998Building)), patch: normWireRequired(normWireNullable(parseEn1998BuildingPatch)) });
export const parseEn1998BridgeEdit: NormWireReader<En1998BridgeEdit> = normWireObject<En1998BridgeEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998Bridge)), patch: normWireRequired(normWireNullable(parseEn1998BridgePatch)) });
export const parseEn1998AssessmentEdit: NormWireReader<En1998AssessmentEdit> = normWireObject<En1998AssessmentEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998Assessment)), patch: normWireRequired(normWireNullable(parseEn1998AssessmentPatch)) });
export const parseEn1998SiloEdit: NormWireReader<En1998SiloEdit> = normWireObject<En1998SiloEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998Silo)) });
export const parseEn1998TankEdit: NormWireReader<En1998TankEdit> = normWireObject<En1998TankEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998Tank)) });
export const parseEn1998FoundationEdit: NormWireReader<En1998FoundationEdit> = normWireObject<En1998FoundationEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998Foundation)) });
export const parseEn1998RetainingWallEdit: NormWireReader<En1998RetainingWallEdit> = normWireObject<En1998RetainingWallEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998RetainingWall)) });
export const parseEn1998TowerEdit: NormWireReader<En1998TowerEdit> = normWireObject<En1998TowerEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998Tower)), patch: normWireRequired(normWireNullable(parseEn1998TowerPatch)) });
export const parseEn1998SystemEdit: NormWireReader<En1998SystemEdit> = normWireObject<En1998SystemEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998System)), patch: normWireRequired(normWireNullable(parseEn1998SystemPatch)) });
export const parseEn1998StoreyEdit: NormWireReader<En1998StoreyEdit> = normWireObject<En1998StoreyEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998Storey)), patch: normWireRequired(normWireNullable(parseEn1998StoreyPatch)) });
export const parseEn1998MemberEdit: NormWireReader<En1998MemberEdit> = normWireObject<En1998MemberEdit>({ op: normWireRequired(parseEn1998RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1998Member)), patch: normWireRequired(normWireNullable(parseEn1998MemberPatch)) });
export const parseEn1998BuildingPatch: NormWireReader<En1998BuildingPatch> = normWireObject<En1998BuildingPatch>({ planRegular: normWireDefault(normWireNullable(normWireBoolean), () => null), elevationRegular: normWireDefault(normWireNullable(normWireBoolean), () => null), masonryWallAreaRatio: normWireDefault(normWireNullable(normWireNumber), () => null), systems: normWireDefault(normWireArray(parseEn1998SystemEdit), () => []), storeys: normWireDefault(normWireArray(parseEn1998StoreyEdit), () => []), members: normWireDefault(normWireArray(parseEn1998MemberEdit), () => []) });
export const parseEn1998BridgePatch: NormWireReader<En1998BridgePatch> = normWireObject<En1998BridgePatch>({ vRdN: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1998AssessmentPatch: NormWireReader<En1998AssessmentPatch> = normWireObject<En1998AssessmentPatch>({ rKN: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1998TowerPatch: NormWireReader<En1998TowerPatch> = normWireObject<En1998TowerPatch>({ mRdNm: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1998SystemPatch: NormWireReader<En1998SystemPatch> = normWireObject<En1998SystemPatch>({ baseShearResistanceN: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1998StoreyPatch: NormWireReader<En1998StoreyPatch> = normWireObject<En1998StoreyPatch>({ permanentGkN: normWireDefault(normWireNullable(normWireNumber), () => null), stiffnessX: normWireDefault(normWireNullable(normWireNumber), () => null), driftXM: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1998MemberPatch: NormWireReader<En1998MemberPatch> = normWireObject<En1998MemberPatch>({ detailingCompatibleWithQ: normWireDefault(normWireNullable(normWireBoolean), () => null) });
export const parseEn1998Diff: NormWireReader<En1998Diff> = normWireObject<En1998Diff>({ annex: normWireDefault(normWireNullable(normWireString), () => null), site: normWireDefault(normWireNullable(parseEn1998Site), () => null), buildings: normWireDefault(normWireArray(parseEn1998BuildingEdit), () => []), bridges: normWireDefault(normWireArray(parseEn1998BridgeEdit), () => []), assessments: normWireDefault(normWireArray(parseEn1998AssessmentEdit), () => []), silos: normWireDefault(normWireArray(parseEn1998SiloEdit), () => []), tanks: normWireDefault(normWireArray(parseEn1998TankEdit), () => []), foundations: normWireDefault(normWireArray(parseEn1998FoundationEdit), () => []), retainingWalls: normWireDefault(normWireArray(parseEn1998RetainingWallEdit), () => []), towers: normWireDefault(normWireArray(parseEn1998TowerEdit), () => []) });
