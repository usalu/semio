/** 🧺️ `En1995Mutation` wire twin: the mutation aggregate, branch for branch as `./🔣️.json` spells it, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireExternal, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ChangeConnectionDiameter, parseChangeConnectionDiameter } from "./↔️change-connection-diameter/🧬️schema/🟦️.ts";
import { type ChangeConnectionEdgeDistance, parseChangeConnectionEdgeDistance } from "./↔️change-connection-edge-distance/🧬️schema/🟦️.ts";
import { type ChangeConnectionEndDistance, parseChangeConnectionEndDistance } from "./↔️change-connection-end-distance/🧬️schema/🟦️.ts";
import { type ChangeConnectionPlateThickness, parseChangeConnectionPlateThickness } from "./↔️change-connection-plate-thickness/🧬️schema/🟦️.ts";
import { type ChangeConnectionSpacing, parseChangeConnectionSpacing } from "./↔️change-connection-spacing/🧬️schema/🟦️.ts";
import { type ChangeConnectionT1, parseChangeConnectionT1 } from "./↔️change-connection-t1/🧬️schema/🟦️.ts";
import { type ChangeConnectionT2, parseChangeConnectionT2 } from "./↔️change-connection-t2/🧬️schema/🟦️.ts";
import { type ChangeMemberB, parseChangeMemberB } from "./↔️change-member-b/🧬️schema/🟦️.ts";
import { type ChangeMemberBearingLength, parseChangeMemberBearingLength } from "./↔️change-member-bearing-length/🧬️schema/🟦️.ts";
import { type ChangeMemberBucklingLengthY, parseChangeMemberBucklingLengthY } from "./↔️change-member-buckling-length-y/🧬️schema/🟦️.ts";
import { type ChangeMemberBucklingLengthZ, parseChangeMemberBucklingLengthZ } from "./↔️change-member-buckling-length-z/🧬️schema/🟦️.ts";
import { type ChangeMemberNotchDepth, parseChangeMemberNotchDepth } from "./↔️change-member-notch-depth/🧬️schema/🟦️.ts";
import { type ChangeMemberNotchDistance, parseChangeMemberNotchDistance } from "./↔️change-member-notch-distance/🧬️schema/🟦️.ts";
import { type ChangeMemberRestraintSpacing, parseChangeMemberRestraintSpacing } from "./↔️change-member-restraint-spacing/🧬️schema/🟦️.ts";
import { type ChangeMemberSpan, parseChangeMemberSpan } from "./↔️change-member-span/🧬️schema/🟦️.ts";
import { type ChangeMemberSupportLength, parseChangeMemberSupportLength } from "./↔️change-member-support-length/🧬️schema/🟦️.ts";
import { type ChangeMemberActionVK, parseChangeMemberActionVK } from "./↕️change-member-action-vk/🧬️schema/🟦️.ts";
import { type ChangeMemberH, parseChangeMemberH } from "./↕️change-member-h/🧬️schema/🟦️.ts";
import { type ChangeConnectionLoadDuration, parseChangeConnectionLoadDuration } from "./⏳️change-connection-load-duration/🧬️schema/🟦️.ts";
import { type ChangeMemberLoadDuration, parseChangeMemberLoadDuration } from "./⏳️change-member-load-duration/🧬️schema/🟦️.ts";
import { type ChangeConnectionActionKind, parseChangeConnectionActionKind } from "./⚖️change-connection-action-kind/🧬️schema/🟦️.ts";
import { type ChangeMemberActionKind, parseChangeMemberActionKind } from "./⚖️change-member-action-kind/🧬️schema/🟦️.ts";
import { type ChangeMemberMassKgPerM, parseChangeMemberMassKgPerM } from "./⚖️change-member-mass-kg-per-m/🧬️schema/🟦️.ts";
import { type ChangeMemberMassKgPerM2, parseChangeMemberMassKgPerM2 } from "./⚖️change-member-mass-kg-per-m2/🧬️schema/🟦️.ts";
import { type ChangeMemberMCrit, parseChangeMemberMCrit } from "./⚠️change-member-m-crit/🧬️schema/🟦️.ts";
import { type InsertConnectionAction, parseInsertConnectionAction } from "./➕️insert-connection-action/🧬️schema/🟦️.ts";
import { type InsertConnection, parseInsertConnection } from "./➕️insert-connection/🧬️schema/🟦️.ts";
import { type InsertMemberAction, parseInsertMemberAction } from "./➕️insert-member-action/🧬️schema/🟦️.ts";
import { type InsertMember, parseInsertMember } from "./➕️insert-member/🧬️schema/🟦️.ts";
import { parseRemoveConnectionAction, type RemoveConnectionAction } from "./➖️remove-connection-action/🧬️schema/🟦️.ts";
import { parseRemoveConnection, type RemoveConnection } from "./➖️remove-connection/🧬️schema/🟦️.ts";
import { parseRemoveMemberAction, type RemoveMemberAction } from "./➖️remove-member-action/🧬️schema/🟦️.ts";
import { parseRemoveMember, type RemoveMember } from "./➖️remove-member/🧬️schema/🟦️.ts";
import { type ChangeMemberActionMK, parseChangeMemberActionMK } from "./⤴️change-member-action-mk/🧬️schema/🟦️.ts";
import { type ChangeMemberActionFPoint, parseChangeMemberActionFPoint } from "./⬇️change-member-action-f-point/🧬️schema/🟦️.ts";
import { type ChangeMemberActionQLine, parseChangeMemberActionQLine } from "./⬇️change-member-action-q-line/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeA, parseChangeMemberBridgeA } from "./🌉️change-member-bridge-a/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeB, parseChangeMemberBridgeB } from "./🌉️change-member-bridge-b/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeBeta, parseChangeMemberBridgeBeta } from "./🌉️change-member-bridge-beta/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeNObs, parseChangeMemberBridgeNObs } from "./🌉️change-member-bridge-n-obs/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeTLYears, parseChangeMemberBridgeTLYears } from "./🌉️change-member-bridge-tl-years/🧬️schema/🟦️.ts";
import { type ChangeMemberDampingXi, parseChangeMemberDampingXi } from "./🌊️change-member-damping-xi/🧬️schema/🟦️.ts";
import { type ChangeAnnex, parseChangeAnnex } from "./🌍️change-annex/🧬️schema/🟦️.ts";
import { type ChangeConnectionServiceClass, parseChangeConnectionServiceClass } from "./🌧️change-connection-service-class/🧬️schema/🟦️.ts";
import { type ChangeMemberServiceClass, parseChangeMemberServiceClass } from "./🌧️change-member-service-class/🧬️schema/🟦️.ts";
import { type ChangeMemberRole, parseChangeMemberRole } from "./🎯️change-member-role/🧬️schema/🟦️.ts";
import { type ChangeMemberActionFC90K, parseChangeMemberActionFC90K } from "./🏋️change-member-action-fc90-k/🧬️schema/🟦️.ts";
import { type ChangeMemberActionNK, parseChangeMemberActionNK } from "./🏋️change-member-action-nk/🧬️schema/🟦️.ts";
import { type ChangeMemberActionNTK, parseChangeMemberActionNTK } from "./🏋️change-member-action-ntk/🧬️schema/🟦️.ts";
import { type ChangeMemberActionCategory, parseChangeMemberActionCategory } from "./🏢️change-member-action-category/🧬️schema/🟦️.ts";
import { type ChangeConnectionLabelDe, parseChangeConnectionLabelDe } from "./🏷️change-connection-label-de/🧬️schema/🟦️.ts";
import { type ChangeConnectionLabelEn, parseChangeConnectionLabelEn } from "./🏷️change-connection-label-en/🧬️schema/🟦️.ts";
import { type ChangeMemberLabelDe, parseChangeMemberLabelDe } from "./🏷️change-member-label-de/🧬️schema/🟦️.ts";
import { type ChangeMemberLabelEn, parseChangeMemberLabelEn } from "./🏷️change-member-label-en/🧬️schema/🟦️.ts";
import { type ChangeMemberSupport, parseChangeMemberSupport } from "./📍️change-member-support/🧬️schema/🟦️.ts";
import { type ChangeConnectionNumber, parseChangeConnectionNumber } from "./🔢️change-connection-number/🧬️schema/🟦️.ts";
import { type ChangeConnectionRows, parseChangeConnectionRows } from "./🔢️change-connection-rows/🧬️schema/🟦️.ts";
import { type ChangeConnectionShearPlanes, parseChangeConnectionShearPlanes } from "./🔢️change-connection-shear-planes/🧬️schema/🟦️.ts";
import { type ChangeMemberFireDuration, parseChangeMemberFireDuration } from "./🔥️change-member-fire-duration/🧬️schema/🟦️.ts";
import { type ChangeConnectionActionFK, parseChangeConnectionActionFK } from "./🔩️change-connection-action-fk/🧬️schema/🟦️.ts";
import { type ChangeConnectionFastenerType, parseChangeConnectionFastenerType } from "./🔩️change-connection-fastener-type/🧬️schema/🟦️.ts";
import { type ChangeConnectionSteelPlate, parseChangeConnectionSteelPlate } from "./🔩️change-connection-steel-plate/🧬️schema/🟦️.ts";
import { type ChangeMemberBridgeCrowd, parseChangeMemberBridgeCrowd } from "./🚶️change-member-bridge-crowd/🧬️schema/🟦️.ts";
import { type ChangeConnectionFUK, parseChangeConnectionFUK } from "./🛡️change-connection-fuk/🧬️schema/🟦️.ts";
import { type ChangeConnectionStrengthClass, parseChangeConnectionStrengthClass } from "./🛡️change-connection-strength-class/🧬️schema/🟦️.ts";
import { type ChangeMemberStrengthClass, parseChangeMemberStrengthClass } from "./🛡️change-member-strength-class/🧬️schema/🟦️.ts";

export type En1995Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { InsertMember: InsertMember }
  | { RemoveMember: RemoveMember }
  | { ChangeMemberLabelEn: ChangeMemberLabelEn }
  | { ChangeMemberLabelDe: ChangeMemberLabelDe }
  | { ChangeMemberRole: ChangeMemberRole }
  | { ChangeMemberStrengthClass: ChangeMemberStrengthClass }
  | { ChangeMemberServiceClass: ChangeMemberServiceClass }
  | { ChangeMemberSupport: ChangeMemberSupport }
  | { ChangeMemberB: ChangeMemberB }
  | { ChangeMemberH: ChangeMemberH }
  | { ChangeMemberSpan: ChangeMemberSpan }
  | { ChangeMemberSupportLength: ChangeMemberSupportLength }
  | { ChangeMemberBearingLength: ChangeMemberBearingLength }
  | { ChangeMemberBucklingLengthY: ChangeMemberBucklingLengthY }
  | { ChangeMemberBucklingLengthZ: ChangeMemberBucklingLengthZ }
  | { ChangeMemberRestraintSpacing: ChangeMemberRestraintSpacing }
  | { ChangeMemberNotchDepth: ChangeMemberNotchDepth }
  | { ChangeMemberNotchDistance: ChangeMemberNotchDistance }
  | { ChangeMemberMCrit: ChangeMemberMCrit }
  | { ChangeMemberMassKgPerM: ChangeMemberMassKgPerM }
  | { ChangeMemberMassKgPerM2: ChangeMemberMassKgPerM2 }
  | { ChangeMemberDampingXi: ChangeMemberDampingXi }
  | { ChangeMemberFireDuration: ChangeMemberFireDuration }
  | { ChangeMemberBridgeNObs: ChangeMemberBridgeNObs }
  | { ChangeMemberBridgeTLYears: ChangeMemberBridgeTLYears }
  | { ChangeMemberBridgeBeta: ChangeMemberBridgeBeta }
  | { ChangeMemberBridgeA: ChangeMemberBridgeA }
  | { ChangeMemberBridgeB: ChangeMemberBridgeB }
  | { ChangeMemberBridgeCrowd: ChangeMemberBridgeCrowd }
  | { InsertMemberAction: InsertMemberAction }
  | { RemoveMemberAction: RemoveMemberAction }
  | { ChangeMemberActionKind: ChangeMemberActionKind }
  | { ChangeMemberActionCategory: ChangeMemberActionCategory }
  | { ChangeMemberLoadDuration: ChangeMemberLoadDuration }
  | { ChangeMemberActionQLine: ChangeMemberActionQLine }
  | { ChangeMemberActionFPoint: ChangeMemberActionFPoint }
  | { ChangeMemberActionMK: ChangeMemberActionMK }
  | { ChangeMemberActionVK: ChangeMemberActionVK }
  | { ChangeMemberActionNK: ChangeMemberActionNK }
  | { ChangeMemberActionNTK: ChangeMemberActionNTK }
  | { ChangeMemberActionFC90K: ChangeMemberActionFC90K }
  | { InsertConnection: InsertConnection }
  | { RemoveConnection: RemoveConnection }
  | { ChangeConnectionLabelEn: ChangeConnectionLabelEn }
  | { ChangeConnectionLabelDe: ChangeConnectionLabelDe }
  | { ChangeConnectionFastenerType: ChangeConnectionFastenerType }
  | { ChangeConnectionStrengthClass: ChangeConnectionStrengthClass }
  | { ChangeConnectionServiceClass: ChangeConnectionServiceClass }
  | { ChangeConnectionDiameter: ChangeConnectionDiameter }
  | { ChangeConnectionNumber: ChangeConnectionNumber }
  | { ChangeConnectionRows: ChangeConnectionRows }
  | { ChangeConnectionSpacing: ChangeConnectionSpacing }
  | { ChangeConnectionEdgeDistance: ChangeConnectionEdgeDistance }
  | { ChangeConnectionEndDistance: ChangeConnectionEndDistance }
  | { ChangeConnectionT1: ChangeConnectionT1 }
  | { ChangeConnectionT2: ChangeConnectionT2 }
  | { ChangeConnectionSteelPlate: ChangeConnectionSteelPlate }
  | { ChangeConnectionPlateThickness: ChangeConnectionPlateThickness }
  | { ChangeConnectionShearPlanes: ChangeConnectionShearPlanes }
  | { ChangeConnectionFUK: ChangeConnectionFUK }
  | { InsertConnectionAction: InsertConnectionAction }
  | { RemoveConnectionAction: RemoveConnectionAction }
  | { ChangeConnectionActionKind: ChangeConnectionActionKind }
  | { ChangeConnectionLoadDuration: ChangeConnectionLoadDuration }
  | { ChangeConnectionActionFK: ChangeConnectionActionFK };

export const parseEn1995Mutation: NormWireReader<En1995Mutation> = normWireExternal<En1995Mutation>({
  ChangeAnnex: parseChangeAnnex,
  InsertMember: parseInsertMember,
  RemoveMember: parseRemoveMember,
  ChangeMemberLabelEn: parseChangeMemberLabelEn,
  ChangeMemberLabelDe: parseChangeMemberLabelDe,
  ChangeMemberRole: parseChangeMemberRole,
  ChangeMemberStrengthClass: parseChangeMemberStrengthClass,
  ChangeMemberServiceClass: parseChangeMemberServiceClass,
  ChangeMemberSupport: parseChangeMemberSupport,
  ChangeMemberB: parseChangeMemberB,
  ChangeMemberH: parseChangeMemberH,
  ChangeMemberSpan: parseChangeMemberSpan,
  ChangeMemberSupportLength: parseChangeMemberSupportLength,
  ChangeMemberBearingLength: parseChangeMemberBearingLength,
  ChangeMemberBucklingLengthY: parseChangeMemberBucklingLengthY,
  ChangeMemberBucklingLengthZ: parseChangeMemberBucklingLengthZ,
  ChangeMemberRestraintSpacing: parseChangeMemberRestraintSpacing,
  ChangeMemberNotchDepth: parseChangeMemberNotchDepth,
  ChangeMemberNotchDistance: parseChangeMemberNotchDistance,
  ChangeMemberMCrit: parseChangeMemberMCrit,
  ChangeMemberMassKgPerM: parseChangeMemberMassKgPerM,
  ChangeMemberMassKgPerM2: parseChangeMemberMassKgPerM2,
  ChangeMemberDampingXi: parseChangeMemberDampingXi,
  ChangeMemberFireDuration: parseChangeMemberFireDuration,
  ChangeMemberBridgeNObs: parseChangeMemberBridgeNObs,
  ChangeMemberBridgeTLYears: parseChangeMemberBridgeTLYears,
  ChangeMemberBridgeBeta: parseChangeMemberBridgeBeta,
  ChangeMemberBridgeA: parseChangeMemberBridgeA,
  ChangeMemberBridgeB: parseChangeMemberBridgeB,
  ChangeMemberBridgeCrowd: parseChangeMemberBridgeCrowd,
  InsertMemberAction: parseInsertMemberAction,
  RemoveMemberAction: parseRemoveMemberAction,
  ChangeMemberActionKind: parseChangeMemberActionKind,
  ChangeMemberActionCategory: parseChangeMemberActionCategory,
  ChangeMemberLoadDuration: parseChangeMemberLoadDuration,
  ChangeMemberActionQLine: parseChangeMemberActionQLine,
  ChangeMemberActionFPoint: parseChangeMemberActionFPoint,
  ChangeMemberActionMK: parseChangeMemberActionMK,
  ChangeMemberActionVK: parseChangeMemberActionVK,
  ChangeMemberActionNK: parseChangeMemberActionNK,
  ChangeMemberActionNTK: parseChangeMemberActionNTK,
  ChangeMemberActionFC90K: parseChangeMemberActionFC90K,
  InsertConnection: parseInsertConnection,
  RemoveConnection: parseRemoveConnection,
  ChangeConnectionLabelEn: parseChangeConnectionLabelEn,
  ChangeConnectionLabelDe: parseChangeConnectionLabelDe,
  ChangeConnectionFastenerType: parseChangeConnectionFastenerType,
  ChangeConnectionStrengthClass: parseChangeConnectionStrengthClass,
  ChangeConnectionServiceClass: parseChangeConnectionServiceClass,
  ChangeConnectionDiameter: parseChangeConnectionDiameter,
  ChangeConnectionNumber: parseChangeConnectionNumber,
  ChangeConnectionRows: parseChangeConnectionRows,
  ChangeConnectionSpacing: parseChangeConnectionSpacing,
  ChangeConnectionEdgeDistance: parseChangeConnectionEdgeDistance,
  ChangeConnectionEndDistance: parseChangeConnectionEndDistance,
  ChangeConnectionT1: parseChangeConnectionT1,
  ChangeConnectionT2: parseChangeConnectionT2,
  ChangeConnectionSteelPlate: parseChangeConnectionSteelPlate,
  ChangeConnectionPlateThickness: parseChangeConnectionPlateThickness,
  ChangeConnectionShearPlanes: parseChangeConnectionShearPlanes,
  ChangeConnectionFUK: parseChangeConnectionFUK,
  InsertConnectionAction: parseInsertConnectionAction,
  RemoveConnectionAction: parseRemoveConnectionAction,
  ChangeConnectionActionKind: parseChangeConnectionActionKind,
  ChangeConnectionLoadDuration: parseChangeConnectionLoadDuration,
  ChangeConnectionActionFK: parseChangeConnectionActionFK,
});
