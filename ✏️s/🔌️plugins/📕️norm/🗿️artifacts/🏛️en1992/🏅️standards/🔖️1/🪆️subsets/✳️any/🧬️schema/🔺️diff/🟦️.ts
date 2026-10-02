/** 🔺️ `En1992Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireArray, normWireJson, normWireMap, normWireNumber, normWireObject, normWireOptional, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1992Diff {
  /** @state artifact */
  artifact?: { [key: string]: NormJson };
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
  concreteGrades?: { values?: NormJson[]; };
  /** @state artifact */
  reinforcementGrades?: { values?: NormJson[]; };
  /** @state artifact */
  prestressSteels?: { values?: NormJson[]; };
  /** @state artifact */
  members?: { values?: NormJson[]; };
  /** @state artifact */
  anchors?: { values?: NormJson[]; };
}

export const parseEn1992Diff: NormWireReader<En1992Diff> = normWireObject<En1992Diff>({ artifact: normWireOptional(normWireMap(normWireJson)), annex: normWireOptional(normWireString), title: normWireOptional(normWireString), designWorkingLifeYears: normWireOptional(normWireNumber), deltaCDev: normWireOptional(normWireNumber), cementType: normWireOptional(normWireString), concreteGrades: normWireOptional(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), reinforcementGrades: normWireOptional(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), prestressSteels: normWireOptional(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), members: normWireOptional(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)), anchors: normWireOptional(normWireObject<{ values?: NormJson[]; }>({ values: normWireOptional(normWireArray(normWireJson)) }, false)) });
