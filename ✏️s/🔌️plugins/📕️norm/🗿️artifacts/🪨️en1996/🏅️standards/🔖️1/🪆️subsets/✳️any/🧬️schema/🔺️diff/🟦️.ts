/** 🔺️ `En1996Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireInteger, normWireJson, normWireMap, normWireObject, normWireOptional, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1996Diff {
  artifact?: { [key: string]: NormJson };
  annex?: string;
  masonryClass?: string;
  designSituation?: string;
  storeys?: number;
  walls?: { [key: string]: NormJson };
}

export const parseEn1996Diff: NormWireReader<En1996Diff> = normWireObject<En1996Diff>({ artifact: normWireOptional(normWireMap(normWireJson)), annex: normWireOptional(normWireString), masonryClass: normWireOptional(normWireString), designSituation: normWireOptional(normWireString), storeys: normWireOptional(normWireInteger), walls: normWireOptional(normWireMap(normWireJson)) }, false);
