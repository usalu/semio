/** 🧬️ EN 1995 sparse diff TypeScript mirror. */
import type { En1995Artifact } from "../🟦️";
import type { AnnexChoice, TimberConnection, TimberMember } from "../📸️snapshot/🟦️";

export interface En1995MemberList { values: TimberMember[]; }
export interface En1995ConnectionList { values: TimberConnection[]; }
export interface En1995Diff {
  artifact?: En1995Artifact;
  annex?: AnnexChoice;
  members?: En1995MemberList;
  connections?: En1995ConnectionList;
}
