/** 🔺️ `En1999Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireArray, normWireJson, normWireMap, normWireObject, normWireOptional, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1999Diff {
  artifact?: { [key: string]: NormJson };
  annex?: string;
  materials?: NormJson[];
  sections?: NormJson[];
  members?: NormJson[];
  connections?: NormJson[];
  fireScenarios?: NormJson[];
  fatigueDetails?: NormJson[];
  coldFormed?: NormJson[];
  shells?: NormJson[];
}

export const parseEn1999Diff: NormWireReader<En1999Diff> = normWireObject<En1999Diff>({ artifact: normWireOptional(normWireMap(normWireJson)), annex: normWireOptional(normWireString), materials: normWireOptional(normWireArray(normWireJson)), sections: normWireOptional(normWireArray(normWireJson)), members: normWireOptional(normWireArray(normWireJson)), connections: normWireOptional(normWireArray(normWireJson)), fireScenarios: normWireOptional(normWireArray(normWireJson)), fatigueDetails: normWireOptional(normWireArray(normWireJson)), coldFormed: normWireOptional(normWireArray(normWireJson)), shells: normWireOptional(normWireArray(normWireJson)) });
