/** 🧱️ `insert-retaining-wall` wire twin: the leaf payload `InsertRetainingWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1998RetainingWall, parseEn1998RetainingWall } from "../../../📸️snapshot/🟦️.ts";

export interface InsertRetainingWall {
  mutation: "insertRetainingWall";
  index: number;
  wall: En1998RetainingWall;
}

export const parseInsertRetainingWall: NormWireReader<InsertRetainingWall> = normWireObject<InsertRetainingWall>({ mutation: normWireRequired(normWireLiteral("insertRetainingWall")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), wall: normWireRequired(parseEn1998RetainingWall) });
