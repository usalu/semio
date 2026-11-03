/** ➕️ `insert-opening` wire twin: the leaf payload `InsertOpening`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WallOpening, parseWallOpening } from "../../../📸️snapshot/🟦️.ts";

export interface InsertOpening {
  wallId: string;
  index: number;
  opening: WallOpening;
}

export const parseInsertOpening: NormWireReader<InsertOpening> = normWireObject<InsertOpening>({ wallId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), opening: normWireRequired(parseWallOpening) });
