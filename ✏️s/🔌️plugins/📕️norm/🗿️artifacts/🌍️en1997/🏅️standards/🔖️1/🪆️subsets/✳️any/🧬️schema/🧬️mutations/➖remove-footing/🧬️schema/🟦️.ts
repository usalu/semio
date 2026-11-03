/** ➖ `remove-footing` wire twin: the leaf payload `RemoveFooting`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveFooting {
  mutation: "removeFooting";
  index: number;
}

export const parseRemoveFooting: NormWireReader<RemoveFooting> = normWireObject<RemoveFooting>({ mutation: normWireRequired(normWireLiteral("removeFooting")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
