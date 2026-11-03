/** 🚦 `remove-traffic-group` wire twin: the leaf payload `RemoveTrafficGroup`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveTrafficGroup {
  mutation: "removeTrafficGroup";
  index: number;
}

export const parseRemoveTrafficGroup: NormWireReader<RemoveTrafficGroup> = normWireObject<RemoveTrafficGroup>({ mutation: normWireRequired(normWireLiteral("removeTrafficGroup")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
