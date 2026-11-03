/** 🚛 `insert-special-vehicle` wire twin: the leaf payload `InsertSpecialVehicle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SpecialVehicle, parseSpecialVehicle } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSpecialVehicle {
  index: number;
  item: SpecialVehicle;
}

export const parseInsertSpecialVehicle: NormWireReader<InsertSpecialVehicle> = normWireObject<InsertSpecialVehicle>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseSpecialVehicle) });
