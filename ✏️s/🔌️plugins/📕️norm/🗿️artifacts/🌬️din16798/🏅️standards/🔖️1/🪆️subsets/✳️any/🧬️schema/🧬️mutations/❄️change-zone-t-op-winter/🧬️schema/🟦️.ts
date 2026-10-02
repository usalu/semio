/** ❄️ `change-zone-t-op-winter` wire twin: the leaf payload `ChangeZoneTOpWinter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneTOpWinter {
  zoneId: string;
  newTOpWinterC: number;
}

export const parseChangeZoneTOpWinter: NormWireReader<ChangeZoneTOpWinter> = normWireObject<ChangeZoneTOpWinter>({ zoneId: normWireRequired(normWireString), newTOpWinterC: normWireRequired(normWireNumber) });
