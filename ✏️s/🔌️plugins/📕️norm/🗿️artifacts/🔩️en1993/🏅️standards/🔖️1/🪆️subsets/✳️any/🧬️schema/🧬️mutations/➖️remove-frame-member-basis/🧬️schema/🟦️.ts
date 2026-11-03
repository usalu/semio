/** ➖️ `remove-frame-member-basis` wire twin: the leaf payload `RemoveFrameMemberBasis`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFrameMemberBasis {
  index: number;
}

export const parseRemoveFrameMemberBasis: NormWireReader<RemoveFrameMemberBasis> = normWireObject<RemoveFrameMemberBasis>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
