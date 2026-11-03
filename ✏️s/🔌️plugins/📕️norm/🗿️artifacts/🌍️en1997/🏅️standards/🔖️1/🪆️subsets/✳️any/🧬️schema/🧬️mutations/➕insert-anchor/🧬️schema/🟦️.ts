/** ➕ `insert-anchor` wire twin: the leaf payload `InsertAnchor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997Anchor, parseEn1997Anchor } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAnchor {
  mutation: "insertAnchor";
  index: number;
  anchor: En1997Anchor;
}

export const parseInsertAnchor: NormWireReader<InsertAnchor> = normWireObject<InsertAnchor>({ mutation: normWireRequired(normWireLiteral("insertAnchor")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), anchor: normWireRequired(parseEn1997Anchor) });
