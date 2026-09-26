/** 🧬️ En1992 hierarchical mutations (no flat mEd stubs). */
export interface En1992ActionMutation {
  id: string; kind?: string; category?: string; source?: string;
  gKLine?: number; qKLine?: number; pointForce?: number;
  mK?: number; nK?: number; vK?: number; tK?: number; vKPunch?: number;
}
export interface En1992BarMutation { id: string; diameter?: number; count?: number; anchorageLength?: number; lapLength?: number; }
export interface En1992PunchingMutation { columnWidth?: number; columnDepth?: number; columnPosition?: string; asw?: number; }
export interface En1992PrestressMutation { force?: number; area?: number; eccentricity?: number; lossRatio?: number; }
export interface En1992MemberMutation {
  id: string; cover?: number; width?: number; height?: number; effectiveDepth?: number; span?: number;
  exposure?: string; fireAxisDistance?: number; fireRating?: string; stirrupSpacing?: number;
  useFem?: boolean; udl?: number; deflectionSensitive?: boolean;
  actions?: En1992ActionMutation[]; longitudinal?: En1992BarMutation[];
  punching?: En1992PunchingMutation; prestress?: En1992PrestressMutation;
}
export interface En1992AnchorMutation { id: string; hEf?: number; aS?: number; }
export interface En1992ConcreteMutation { id: string; fCk?: number; }
export interface En1992ReinfMutation { id: string; fYk?: number; }
export interface En1992Mutation {
  annex?: string; title?: string; designWorkingLifeYears?: number; deltaCDev?: number; cementType?: string;
  members?: En1992MemberMutation[]; anchors?: En1992AnchorMutation[];
  concreteGrades?: En1992ConcreteMutation[]; reinforcementGrades?: En1992ReinfMutation[];
}
export type ChangeActionMk = { memberId: string; actionId: string; newValue: number };
export type ChangeActionVEd = { memberId: string; actionId: string; newValue: number };
export type ChangeActionNEd = { memberId: string; actionId: string; newValue: number };
