/** 🧬️ Text facet — En1995Mutation externally tagged union (66 id-addressed kinds from Rust). */

import type { CharacteristicAction, ConnectionAction, TimberConnection, TimberMember } from "../../📸️snapshot/🟦️.ts";

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface InsertMember {
  index: number;
  member: TimberMember;
}

export interface RemoveMember {
  index: number;
}

export interface ChangeMemberLabelEn {
  memberId: string;
  newValue: string;
}

export interface ChangeMemberLabelDe {
  memberId: string;
  newValue: string;
}

export interface ChangeMemberRole {
  memberId: string;
  newValue: "beam" | "column" | "floor" | "bridge";
}

export interface ChangeMemberStrengthClass {
  memberId: string;
  newValue: string;
}

export interface ChangeMemberServiceClass {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberSupport {
  memberId: string;
  newValue: "simplySupported" | "cantilever" | "continuousTwoSpan";
}

export interface ChangeMemberB {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberH {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberSpan {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberSupportLength {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberBearingLength {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberBucklingY {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberBucklingZ {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberLateralRestraint {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberNotchDepth {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberNotchDistance {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberMCrit {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberMassPerM {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberMassPerM2 {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberDamping {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberFireDuration {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberBridgeNObs {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberBridgeTLYears {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberBridgeBeta {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberBridgeA {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberBridgeB {
  memberId: string;
  newValue: number;
}

export interface ChangeMemberBridgeCrowd {
  memberId: string;
  newValue: number;
}

export interface InsertMemberAction {
  memberId: string;
  index: number;
  action: CharacteristicAction;
}

export interface RemoveMemberAction {
  memberId: string;
  index: number;
}

export interface ChangeMemberActionKind {
  memberId: string;
  actionId: string;
  newValue: string;
}

export interface ChangeMemberActionCategory {
  memberId: string;
  actionId: string;
  newValue: string;
}

export interface ChangeMemberActionLoadDuration {
  memberId: string;
  actionId: string;
  newValue: string;
}

export interface ChangeMemberActionQLine {
  memberId: string;
  actionId: string;
  newValue: number;
}

export interface ChangeMemberActionFPoint {
  memberId: string;
  actionId: string;
  newValue: number;
}

export interface ChangeMemberActionMK {
  memberId: string;
  actionId: string;
  newValue: number;
}

export interface ChangeMemberActionVK {
  memberId: string;
  actionId: string;
  newValue: number;
}

export interface ChangeMemberActionNK {
  memberId: string;
  actionId: string;
  newValue: number;
}

export interface ChangeMemberActionNTK {
  memberId: string;
  actionId: string;
  newValue: number;
}

export interface ChangeMemberActionFC90K {
  memberId: string;
  actionId: string;
  newValue: number;
}

export interface InsertConnection {
  index: number;
  connection: TimberConnection;
}

export interface RemoveConnection {
  index: number;
}

export interface ChangeConnectionLabelEn {
  connectionId: string;
  newValue: string;
}

export interface ChangeConnectionLabelDe {
  connectionId: string;
  newValue: string;
}

export interface ChangeConnectionFastenerType {
  connectionId: string;
  newValue: string;
}

export interface ChangeConnectionStrengthClass {
  connectionId: string;
  newValue: string;
}

export interface ChangeConnectionServiceClass {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionDiameter {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionNumber {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionRows {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionSpacing {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionEdgeDistance {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionEndDistance {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionT1 {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionT2 {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionSteelPlate {
  connectionId: string;
  newValue: boolean;
}

export interface ChangeConnectionSteelPlateThickness {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionShearPlanes {
  connectionId: string;
  newValue: number;
}

export interface ChangeConnectionFUK {
  connectionId: string;
  newValue: number;
}

export interface InsertConnectionAction {
  connectionId: string;
  index: number;
  action: ConnectionAction;
}

export interface RemoveConnectionAction {
  connectionId: string;
  index: number;
}

export interface ChangeConnectionActionKind {
  connectionId: string;
  actionId: string;
  newValue: string;
}

export interface ChangeConnectionActionLoadDuration {
  connectionId: string;
  actionId: string;
  newValue: string;
}

export interface ChangeConnectionActionFK {
  connectionId: string;
  actionId: string;
  newValue: number;
}

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
  | { ChangeMemberBucklingY: ChangeMemberBucklingY }
  | { ChangeMemberBucklingZ: ChangeMemberBucklingZ }
  | { ChangeMemberLateralRestraint: ChangeMemberLateralRestraint }
  | { ChangeMemberNotchDepth: ChangeMemberNotchDepth }
  | { ChangeMemberNotchDistance: ChangeMemberNotchDistance }
  | { ChangeMemberMCrit: ChangeMemberMCrit }
  | { ChangeMemberMassPerM: ChangeMemberMassPerM }
  | { ChangeMemberMassPerM2: ChangeMemberMassPerM2 }
  | { ChangeMemberDamping: ChangeMemberDamping }
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
  | { ChangeMemberActionLoadDuration: ChangeMemberActionLoadDuration }
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
  | { ChangeConnectionSteelPlateThickness: ChangeConnectionSteelPlateThickness }
  | { ChangeConnectionShearPlanes: ChangeConnectionShearPlanes }
  | { ChangeConnectionFUK: ChangeConnectionFUK }
  | { InsertConnectionAction: InsertConnectionAction }
  | { RemoveConnectionAction: RemoveConnectionAction }
  | { ChangeConnectionActionKind: ChangeConnectionActionKind }
  | { ChangeConnectionActionLoadDuration: ChangeConnectionActionLoadDuration }
  | { ChangeConnectionActionFK: ChangeConnectionActionFK };
