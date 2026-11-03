/** ➕️ `insert-bar` wire twin: the leaf payload `InsertBar`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WallBar, parseWallBar } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBar {
  wallId: string;
  index: number;
  bar: WallBar;
}

export const parseInsertBar: NormWireReader<InsertBar> = normWireObject<InsertBar>({ wallId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), bar: normWireRequired(parseWallBar) });
