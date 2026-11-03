/** ⭕️ `change-ring-shell-id` wire twin: the leaf payload `ChangeRingShellId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRingShellId {
  mutation: "changeRingShellId";
  ringStiffenerId: string;
  newShellId: string;
}

export const parseChangeRingShellId: NormWireReader<ChangeRingShellId> = normWireObject<ChangeRingShellId>({ mutation: normWireRequired(normWireLiteral("changeRingShellId")), ringStiffenerId: normWireRequired(normWireString), newShellId: normWireRequired(normWireString) });
