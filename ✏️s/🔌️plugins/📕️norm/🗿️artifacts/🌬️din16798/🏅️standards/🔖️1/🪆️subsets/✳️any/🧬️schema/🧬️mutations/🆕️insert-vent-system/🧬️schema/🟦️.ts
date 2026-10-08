/** 🆕️ `insert-vent-system` wire twin: the leaf payload `InsertVentSystem`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Din16798VentSystem, parseDin16798VentSystem } from "../../../📸️snapshot/🟦️.ts";

export interface InsertVentSystem {
  index?: number | null;
  vent: Din16798VentSystem;
}

export const parseInsertVentSystem: NormWireReader<InsertVentSystem> = normWireObject<InsertVentSystem>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), vent: normWireRequired(parseDin16798VentSystem) });
