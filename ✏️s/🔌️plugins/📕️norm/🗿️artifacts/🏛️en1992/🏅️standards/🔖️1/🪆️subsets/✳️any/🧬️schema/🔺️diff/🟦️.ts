/** 🔺️ `En1992Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Anchor, type AnnexChoice, type ConcreteGrade, parseAnchor, parseAnnexChoice, parseConcreteGrade, parsePrestressSteel, parseRcMember, parseReinforcementGrade, type PrestressSteel, type RcMember, type ReinforcementGrade } from "../📸️snapshot/🟦️.ts";
import { type En1992Artifact, parseEn1992Artifact } from "../🟦️.ts";

export interface En1992Diff {
  /** @state artifact */
  artifact: En1992Artifact | null;
  /** @state artifact */
  annex: AnnexChoice | null;
  /** @state artifact */
  title: string | null;
  /** @state artifact */
  designWorkingLifeYears: number | null;
  /** @state artifact */
  deltaCDev: number | null;
  /** @state artifact */
  cementType: string | null;
  /** @state artifact */
  concreteGrades: { values: ConcreteGrade[]; } | null;
  /** @state artifact */
  reinforcementGrades: { values: ReinforcementGrade[]; } | null;
  /** @state artifact */
  prestressSteels: { values: PrestressSteel[]; } | null;
  /** @state artifact */
  members: { values: RcMember[]; } | null;
  /** @state artifact */
  anchors: { values: Anchor[]; } | null;
}

export const parseEn1992Diff: NormWireReader<En1992Diff> = normWireObject<En1992Diff>({ artifact: normWireDefault(normWireNullable(parseEn1992Artifact), () => null), annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), title: normWireDefault(normWireNullable(normWireString), () => null), designWorkingLifeYears: normWireDefault(normWireNullable(normWireNumber), () => null), deltaCDev: normWireDefault(normWireNullable(normWireNumber), () => null), cementType: normWireDefault(normWireNullable(normWireString), () => null), concreteGrades: normWireDefault(normWireNullable(normWireObject<{ values: ConcreteGrade[]; }>({ values: normWireRequired(normWireArray(parseConcreteGrade)) })), () => null), reinforcementGrades: normWireDefault(normWireNullable(normWireObject<{ values: ReinforcementGrade[]; }>({ values: normWireRequired(normWireArray(parseReinforcementGrade)) })), () => null), prestressSteels: normWireDefault(normWireNullable(normWireObject<{ values: PrestressSteel[]; }>({ values: normWireRequired(normWireArray(parsePrestressSteel)) })), () => null), members: normWireDefault(normWireNullable(normWireObject<{ values: RcMember[]; }>({ values: normWireRequired(normWireArray(parseRcMember)) })), () => null), anchors: normWireDefault(normWireNullable(normWireObject<{ values: Anchor[]; }>({ values: normWireRequired(normWireArray(parseAnchor)) })), () => null) });
