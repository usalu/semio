/** 🧬️ En1992 diff schema — sparse hierarchical field delta. */

import type { Anchor, ConcreteGrade, PrestressSteel, RcMember, ReinforcementGrade } from "../🟦️.ts";

export interface ValueList<T> {
  values: T[];
}

export interface En1992Diff {
  /** @state artifact */
  artifact?: import("../🟦️.ts").En1992Artifact;
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  designWorkingLifeYears?: number;
  /** @state artifact */
  deltaCDev?: number;
  /** @state artifact */
  cementType?: string;
  /** @state artifact */
  concreteGrades?: ValueList<ConcreteGrade>;
  /** @state artifact */
  reinforcementGrades?: ValueList<ReinforcementGrade>;
  /** @state artifact */
  prestressSteels?: ValueList<PrestressSteel>;
  /** @state artifact */
  members?: ValueList<RcMember>;
  /** @state artifact */
  anchors?: ValueList<Anchor>;
}
