/** ➖ `remove-slope` wire twin: the leaf payload `RemoveSlope`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSlope {
  mutation: "removeSlope";
  index: number;
}

export const parseRemoveSlope: NormWireReader<RemoveSlope> = normWireObject<RemoveSlope>({ mutation: normWireRequired(normWireLiteral("removeSlope")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
