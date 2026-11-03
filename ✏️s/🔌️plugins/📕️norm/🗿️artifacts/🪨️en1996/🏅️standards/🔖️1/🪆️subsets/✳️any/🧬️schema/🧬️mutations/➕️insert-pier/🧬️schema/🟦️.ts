/** ➕️ `insert-pier` wire twin: the leaf payload `InsertPier`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WallPier, parseWallPier } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPier {
  wallId: string;
  index: number;
  pier: WallPier;
}

export const parseInsertPier: NormWireReader<InsertPier> = normWireObject<InsertPier>({ wallId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), pier: normWireRequired(parseWallPier) });
