/** ➕ `insert-retaining-wall` wire twin: the leaf payload `InsertRetainingWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997RetainingWall, parseEn1997RetainingWall } from "../../../📸️snapshot/🟦️.ts";

export interface InsertRetainingWall {
  mutation: "insertRetainingWall";
  index: number;
  retainingWall: En1997RetainingWall;
}

export const parseInsertRetainingWall: NormWireReader<InsertRetainingWall> = normWireObject<InsertRetainingWall>({ mutation: normWireRequired(normWireLiteral("insertRetainingWall")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), retainingWall: normWireRequired(parseEn1997RetainingWall) });
