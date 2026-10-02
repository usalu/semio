/** ❄ `change-altitude` wire twin: the leaf payload `ChangeAltitude`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAltitude {
  newAltitude: number;
}

export const parseChangeAltitude: NormWireReader<ChangeAltitude> = normWireObject<ChangeAltitude>({ newAltitude: normWireRequired(normWireNumber) });
