/** 📍️ `insert-placement` wire twin: the leaf payload `InsertPlacement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Placement, parseEn1998Placement } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPlacement {
  mutation: "insertPlacement";
  index: number;
  item: En1998Placement;
}

export const parseInsertPlacement: NormWireReader<InsertPlacement> = normWireObject<InsertPlacement>({ mutation: normWireRequired(normWireLiteral("insertPlacement")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1998Placement) });
