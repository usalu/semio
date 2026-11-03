/** ➕️ `insert-accelerogram` wire twin: the leaf payload `InsertAccelerogram`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Accelerogram, parseEn1998Accelerogram } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAccelerogram {
  mutation: "insertAccelerogram";
  index?: number | null;
  accelerogram: En1998Accelerogram;
}

export const parseInsertAccelerogram: NormWireReader<InsertAccelerogram> = normWireObject<InsertAccelerogram>({ mutation: normWireRequired(normWireLiteral("insertAccelerogram")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), accelerogram: normWireRequired(parseEn1998Accelerogram) });
