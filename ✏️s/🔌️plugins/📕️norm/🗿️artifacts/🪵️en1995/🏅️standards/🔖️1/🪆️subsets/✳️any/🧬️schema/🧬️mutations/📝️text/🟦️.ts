/** 🧬️ En1995 document mutations — discriminated union mirroring `En1995Mutation` (WASM wiring). */

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface InsertMember {
  index: number;
  member: Record<string, unknown>;
}

export interface RemoveMember {
  index: number;
}

export interface ChangeMemberH {
  index: number;
  newHM: number;
}

export interface ChangeMemberB {
  index: number;
  newBM: number;
}

export interface ChangeMemberStrengthClass {
  index: number;
  newStrengthClass: string;
}

export interface ChangeConnectionNumber {
  index: number;
  newNumber: number;
}

export interface InsertConnection {
  index: number;
  connection: Record<string, unknown>;
}

export interface RemoveConnection {
  index: number;
}

export type En1995Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { InsertMember: InsertMember }
  | { RemoveMember: RemoveMember }
  | { ChangeMemberH: ChangeMemberH }
  | { ChangeMemberB: ChangeMemberB }
  | { ChangeMemberStrengthClass: ChangeMemberStrengthClass }
  | { ChangeConnectionNumber: ChangeConnectionNumber }
  | { InsertConnection: InsertConnection }
  | { RemoveConnection: RemoveConnection };
