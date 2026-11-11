/** 🆕️ `insert-vent-system` wire twin: the leaf payload `InsertVentSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";
import { type Din16798VentSystem, parseDin16798VentSystem } from "../../../📸️snapshot/🟦️.ts";

export interface InsertVentSystem {
  index: number | null;
  vent: Din16798VentSystem;
}

export const parseInsertVentSystem: NormWireReader<InsertVentSystem> = normWireObject<InsertVentSystem>({ index: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null), vent: normWireRequired(parseDin16798VentSystem) });
