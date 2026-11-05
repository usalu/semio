/** ➕️ `insert-material-ductility` wire twin: the leaf payload `InsertMaterialDuctility`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type MaterialDuctility, parseMaterialDuctility } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMaterialDuctility {
  index?: number | null;
  materialDuctility: MaterialDuctility;
}

export const parseInsertMaterialDuctility: NormWireReader<InsertMaterialDuctility> = normWireObject<InsertMaterialDuctility>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), materialDuctility: normWireRequired(parseMaterialDuctility) });
