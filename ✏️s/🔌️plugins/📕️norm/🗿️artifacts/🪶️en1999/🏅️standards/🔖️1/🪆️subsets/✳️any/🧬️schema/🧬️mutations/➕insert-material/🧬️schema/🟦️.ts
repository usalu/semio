/** 🗂️ `insert-material` wire twin: the leaf payload `InsertMaterial`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AluminiumMaterial, parseAluminiumMaterial } from "../../../📸️snapshot/🟦️.ts";

export interface InsertMaterial {
  mutation: "insertMaterial";
  index: number;
  material: AluminiumMaterial;
}

export const parseInsertMaterial: NormWireReader<InsertMaterial> = normWireObject<InsertMaterial>({ mutation: normWireRequired(normWireLiteral("insertMaterial")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), material: normWireRequired(parseAluminiumMaterial) });
