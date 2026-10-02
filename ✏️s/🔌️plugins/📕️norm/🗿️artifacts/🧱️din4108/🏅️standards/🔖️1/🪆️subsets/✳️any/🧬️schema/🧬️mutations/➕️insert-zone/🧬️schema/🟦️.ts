/** ➕️ `insert-zone` wire twin: the leaf payload `InsertZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din4108ThermalZone, parseDin4108ThermalZone } from "../../../📸️snapshot/🟦️.ts";

export interface InsertZone {
  index: number;
  zone: Din4108ThermalZone;
}

export const parseInsertZone: NormWireReader<InsertZone> = normWireObject<InsertZone>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), zone: normWireRequired(parseDin4108ThermalZone) });
