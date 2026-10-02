/** ➕️ `insert-zone` wire twin: the leaf payload `InsertZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din16798Zone, parseDin16798Zone } from "../../../📸️snapshot/🟦️.ts";

export interface InsertZone {
  index: number;
  zone: Din16798Zone;
}

export const parseInsertZone: NormWireReader<InsertZone> = normWireObject<InsertZone>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), zone: normWireRequired(parseDin16798Zone) });
