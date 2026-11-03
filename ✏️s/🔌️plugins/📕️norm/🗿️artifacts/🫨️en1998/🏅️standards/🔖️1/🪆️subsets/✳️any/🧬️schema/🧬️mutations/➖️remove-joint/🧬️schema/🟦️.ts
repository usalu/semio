/** ➖️ `remove-joint` wire twin: the leaf payload `RemoveJoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveJoint {
  mutation: "removeJoint";
  id: string;
}

export const parseRemoveJoint: NormWireReader<RemoveJoint> = normWireObject<RemoveJoint>({ mutation: normWireRequired(normWireLiteral("removeJoint")), id: normWireRequired(normWireString) });
