/** ➕️ `insert-material` wire twin: the leaf payload `InsertMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SteelMaterial, parseSteelMaterial } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMaterial {
  index?: number | null;
  material: SteelMaterial;
}

export const parseInsertMaterial: NormWireReader<InsertMaterial> = normWireObject<InsertMaterial>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), material: normWireRequired(parseSteelMaterial) });
