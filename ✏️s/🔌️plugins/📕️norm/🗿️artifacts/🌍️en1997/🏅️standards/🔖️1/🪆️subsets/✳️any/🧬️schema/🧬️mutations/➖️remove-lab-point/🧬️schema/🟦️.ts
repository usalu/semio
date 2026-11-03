/** ➖ `remove-lab-point` wire twin: the leaf payload `RemoveLabPoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveLabPoint {
  mutation: "removeLabPoint";
  labTestId: string;
  index: number;
}

export const parseRemoveLabPoint: NormWireReader<RemoveLabPoint> = normWireObject<RemoveLabPoint>({ mutation: normWireRequired(normWireLiteral("removeLabPoint")), labTestId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
