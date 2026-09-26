/** 🧬️ EN 1999 diff schema — sparse field delta over the aluminium-structure subject. */

import type { En1999Artifact } from "../🟦️.ts";
import type {
  AluminiumConnection,
  AluminiumMaterial,
  AluminiumMember,
  AluminiumSection,
  AluminiumShell,
  AnnexChoice,
  ColdFormedSheet,
  FatigueDetail,
  FireScenario,
} from "../📸️snapshot/🟦️.ts";

export interface En1999Diff {
  /** @state artifact */
  artifact?: En1999Artifact;
  /** @state artifact */
  annex?: AnnexChoice | string;
  /** @state artifact */
  materials?: AluminiumMaterial[];
  /** @state artifact */
  sections?: AluminiumSection[];
  /** @state artifact */
  members?: AluminiumMember[];
  /** @state artifact */
  connections?: AluminiumConnection[];
  /** @state artifact */
  fireScenarios?: FireScenario[];
  /** @state artifact */
  fatigueDetails?: FatigueDetail[];
  /** @state artifact */
  coldFormed?: ColdFormedSheet[];
  /** @state artifact */
  shells?: AluminiumShell[];
}
