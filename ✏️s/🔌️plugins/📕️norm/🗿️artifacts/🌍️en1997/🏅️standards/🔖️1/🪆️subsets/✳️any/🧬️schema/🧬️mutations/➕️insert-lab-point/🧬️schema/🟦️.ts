/** ➕ `insert-lab-point` wire twin: the leaf payload `InsertLabPoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1997LabPoint, parseEn1997LabPoint } from "../../../📸️snapshot/🟦️.ts";

export interface InsertLabPoint {
  mutation: "insertLabPoint";
  labTestId: string;
  index: number;
  labPoint: En1997LabPoint;
}

export const parseInsertLabPoint: NormWireReader<InsertLabPoint> = normWireObject<InsertLabPoint>({ mutation: normWireRequired(normWireLiteral("insertLabPoint")), labTestId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), labPoint: normWireRequired(parseEn1997LabPoint) });
