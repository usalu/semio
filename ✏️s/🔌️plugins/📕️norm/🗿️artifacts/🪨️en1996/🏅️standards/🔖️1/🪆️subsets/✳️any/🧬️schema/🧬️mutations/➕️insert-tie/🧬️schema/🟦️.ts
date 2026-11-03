/** ➕️ `insert-tie` wire twin: the leaf payload `InsertTie`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WallTie, parseWallTie } from "../../../📸️snapshot/🟦️.ts";

export interface InsertTie {
  wallId: string;
  index: number;
  tie: WallTie;
}

export const parseInsertTie: NormWireReader<InsertTie> = normWireObject<InsertTie>({ wallId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), tie: normWireRequired(parseWallTie) });
