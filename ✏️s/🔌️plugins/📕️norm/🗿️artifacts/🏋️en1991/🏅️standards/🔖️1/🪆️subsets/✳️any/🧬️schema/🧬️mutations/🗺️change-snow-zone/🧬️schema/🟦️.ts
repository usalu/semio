/** 🗺️ `change-snow-zone` wire twin: the leaf payload `ChangeSnowZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSnowZone {
  newSnowZone: string;
}

export const parseChangeSnowZone: NormWireReader<ChangeSnowZone> = normWireObject<ChangeSnowZone>({ newSnowZone: normWireRequired(normWireString) });
