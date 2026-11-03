/** ➖ `remove-anchor` wire twin: the leaf payload `RemoveAnchor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveAnchor {
  mutation: "removeAnchor";
  index: number;
}

export const parseRemoveAnchor: NormWireReader<RemoveAnchor> = normWireObject<RemoveAnchor>({ mutation: normWireRequired(normWireLiteral("removeAnchor")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
