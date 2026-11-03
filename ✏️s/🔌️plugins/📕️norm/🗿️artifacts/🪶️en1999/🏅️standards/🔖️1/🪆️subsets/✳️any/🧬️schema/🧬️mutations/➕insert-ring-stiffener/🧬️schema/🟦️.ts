/** 🗂️ `insert-ring-stiffener` wire twin: the leaf payload `InsertRingStiffener`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type RingStiffener, parseRingStiffener } from "../../../📸️snapshot/🟦️.ts";

export interface InsertRingStiffener {
  mutation: "insertRingStiffener";
  index: number;
  ringStiffener: RingStiffener;
}

export const parseInsertRingStiffener: NormWireReader<InsertRingStiffener> = normWireObject<InsertRingStiffener>({ mutation: normWireRequired(normWireLiteral("insertRingStiffener")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), ringStiffener: normWireRequired(parseRingStiffener) });
