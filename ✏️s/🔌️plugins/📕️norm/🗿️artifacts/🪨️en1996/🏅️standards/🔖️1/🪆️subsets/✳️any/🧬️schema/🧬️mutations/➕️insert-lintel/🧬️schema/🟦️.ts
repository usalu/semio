/** ➕️ `insert-lintel` wire twin: the leaf payload `InsertLintel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WallLintel, parseWallLintel } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLintel {
  wallId: string;
  index: number;
  lintel: WallLintel;
}

export const parseInsertLintel: NormWireReader<InsertLintel> = normWireObject<InsertLintel>({ wallId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), lintel: normWireRequired(parseWallLintel) });
