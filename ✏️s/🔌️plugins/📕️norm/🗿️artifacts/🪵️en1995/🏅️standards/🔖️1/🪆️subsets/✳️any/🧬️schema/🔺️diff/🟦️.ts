/** 🔺️ `En1995Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireArray, normWireJson, normWireMap, normWireObject, normWireOptional, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1995Diff {
  /** @state artifact */
  artifact?: { [key: string]: NormJson };
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  members?: { values?: { [key: string]: NormJson }[]; };
  /** @state artifact */
  connections?: { values?: { [key: string]: NormJson }[]; };
}

export const parseEn1995Diff: NormWireReader<En1995Diff> = normWireObject<En1995Diff>({ artifact: normWireOptional(normWireMap(normWireJson)), annex: normWireOptional(normWireString), members: normWireOptional(normWireObject<{ values?: { [key: string]: NormJson }[]; }>({ values: normWireOptional(normWireArray(normWireMap(normWireJson))) }, false)), connections: normWireOptional(normWireObject<{ values?: { [key: string]: NormJson }[]; }>({ values: normWireOptional(normWireArray(normWireMap(normWireJson))) }, false)) });
