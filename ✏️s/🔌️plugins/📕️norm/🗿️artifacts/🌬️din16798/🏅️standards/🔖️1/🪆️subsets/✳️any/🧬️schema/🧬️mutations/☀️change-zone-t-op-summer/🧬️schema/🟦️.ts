/** ☀️ `change-zone-t-op-summer` wire twin: the leaf payload `ChangeZoneTOpSummer`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneTOpSummer {
  zoneId: string;
  newTOpSummerC: number;
}

export const parseChangeZoneTOpSummer: NormWireReader<ChangeZoneTOpSummer> = normWireObject<ChangeZoneTOpSummer>({ zoneId: normWireRequired(normWireString), newTOpSummerC: normWireRequired(normWireNumber) });
