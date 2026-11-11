/** ➕ `insert-zone` wire twin: the leaf payload `InsertZone`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";
import { parseThermalZone, type ThermalZone } from "../../../📸️snapshot/🟦️.ts";

export interface InsertZone {
  mutation: "insertzone";
  index: number;
  zone: ThermalZone;
}

export const parseInsertZone: NormWireReader<InsertZone> = normWireObject<InsertZone>({ mutation: normWireRequired(normWireLiteral("insertzone")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), zone: normWireRequired(parseThermalZone) });
