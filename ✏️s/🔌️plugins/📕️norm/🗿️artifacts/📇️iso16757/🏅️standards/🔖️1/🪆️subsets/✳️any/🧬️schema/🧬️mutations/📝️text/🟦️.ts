/** 📝️ `Iso16757MutationsText` wire twin: the serialized text form this facet's grammar and codec speak, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireJson, normWireMap, type NormWireReader } from "../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export type Iso16757MutationsText = { [key: string]: NormJson };

export const parseIso16757MutationsText: NormWireReader<Iso16757MutationsText> = normWireMap(normWireJson);
