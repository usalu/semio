/** 📤 `remove-variable` wire twin: the leaf payload `RemoveVariable`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveVariable {
  mutation: "removeVariable";
  index: number;
}

export const parseRemoveVariable: NormWireReader<RemoveVariable> = normWireObject<RemoveVariable>({ mutation: normWireRequired(normWireLiteral("removeVariable")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
