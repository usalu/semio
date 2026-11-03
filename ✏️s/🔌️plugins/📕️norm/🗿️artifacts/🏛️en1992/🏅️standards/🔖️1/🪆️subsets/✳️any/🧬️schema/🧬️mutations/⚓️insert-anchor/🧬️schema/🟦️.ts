/** ⚓️ `insert-anchor` wire twin: the leaf payload `InsertAnchor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Anchor, parseAnchor } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAnchor {
  index: number;
  anchor: Anchor;
}

export const parseInsertAnchor: NormWireReader<InsertAnchor> = normWireObject<InsertAnchor>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), anchor: normWireRequired(parseAnchor) });
