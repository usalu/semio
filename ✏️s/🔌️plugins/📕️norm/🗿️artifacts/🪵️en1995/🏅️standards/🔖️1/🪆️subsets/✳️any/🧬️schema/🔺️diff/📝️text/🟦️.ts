/** 🧬️ EN 1995 sparse diff TypeScript mirror. */
export type AnnexChoice = "en" | "de";
export interface En1995MemberList { values: Record<string, unknown>[]; }
export interface En1995ConnectionList { values: Record<string, unknown>[]; }
export interface En1995Diff {
  artifact?: Record<string, unknown>;
  annex?: AnnexChoice;
  members?: En1995MemberList;
  connections?: En1995ConnectionList;
}
