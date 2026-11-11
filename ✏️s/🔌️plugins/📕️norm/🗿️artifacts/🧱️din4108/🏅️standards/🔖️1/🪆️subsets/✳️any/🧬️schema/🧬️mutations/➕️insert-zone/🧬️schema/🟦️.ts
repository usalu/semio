/** ➕️ `insert-zone` wire twin: the leaf payload `InsertZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";
import { type Din4108ThermalZone, parseDin4108ThermalZone } from "../../../📸️snapshot/🟦️.ts";

export interface InsertZone {
  index: number | null;
  zone: Din4108ThermalZone;
}

export const parseInsertZone: NormWireReader<InsertZone> = normWireObject<InsertZone>({ index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null), zone: normWireRequired(parseDin4108ThermalZone) });
