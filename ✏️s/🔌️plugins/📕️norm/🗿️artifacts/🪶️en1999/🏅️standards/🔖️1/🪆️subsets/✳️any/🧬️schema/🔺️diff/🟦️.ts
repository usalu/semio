/** 🔺️ `En1999Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireNullable, normWireObject, type NormWireReader } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AluminiumConnection, type AluminiumMaterial, type AluminiumMember, type AluminiumSection, type AluminiumShell, type AnnexChoice, type ColdFormedSheet, type FatigueDetail, type FireScenario, parseAluminiumConnection, parseAluminiumMaterial, parseAluminiumMember, parseAluminiumSection, parseAluminiumShell, parseAnnexChoice, parseColdFormedSheet, parseFatigueDetail, parseFireScenario } from "../📸️snapshot/🟦️.ts";
import { type En1999Artifact, parseEn1999Artifact } from "../🟦️.ts";

export interface En1999Diff {
  /** @state artifact */
  artifact: En1999Artifact | null;
  /** @state artifact */
  annex: AnnexChoice | null;
  /** @state artifact */
  materials: AluminiumMaterial[] | null;
  /** @state artifact */
  sections: AluminiumSection[] | null;
  /** @state artifact */
  members: AluminiumMember[] | null;
  /** @state artifact */
  connections: AluminiumConnection[] | null;
  /** @state artifact */
  fireScenarios: FireScenario[] | null;
  /** @state artifact */
  fatigueDetails: FatigueDetail[] | null;
  /** @state artifact */
  coldFormed: ColdFormedSheet[] | null;
  /** @state artifact */
  shells: AluminiumShell[] | null;
}

export const parseEn1999Diff: NormWireReader<En1999Diff> = normWireObject<En1999Diff>({ artifact: normWireDefault(normWireNullable(parseEn1999Artifact), () => null), annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), materials: normWireDefault(normWireNullable(normWireArray(parseAluminiumMaterial)), () => null), sections: normWireDefault(normWireNullable(normWireArray(parseAluminiumSection)), () => null), members: normWireDefault(normWireNullable(normWireArray(parseAluminiumMember)), () => null), connections: normWireDefault(normWireNullable(normWireArray(parseAluminiumConnection)), () => null), fireScenarios: normWireDefault(normWireNullable(normWireArray(parseFireScenario)), () => null), fatigueDetails: normWireDefault(normWireNullable(normWireArray(parseFatigueDetail)), () => null), coldFormed: normWireDefault(normWireNullable(normWireArray(parseColdFormedSheet)), () => null), shells: normWireDefault(normWireNullable(normWireArray(parseAluminiumShell)), () => null) });
