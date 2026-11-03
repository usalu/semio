/** ➕️ `insert-prestress` wire twin: the leaf payload `InsertPrestress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WallPrestress, parseWallPrestress } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPrestress {
  wallId: string;
  index: number;
  prestress: WallPrestress;
}

export const parseInsertPrestress: NormWireReader<InsertPrestress> = normWireObject<InsertPrestress>({ wallId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), prestress: normWireRequired(parseWallPrestress) });
