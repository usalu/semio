/** 🏋️ `change-action-vyk` wire twin: the leaf payload `ChangeActionVYK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionVYK {
  mutation: "changeActionVYK";
  memberId: string;
  actionId: string;
  newVYK: number;
}

export const parseChangeActionVYK: NormWireReader<ChangeActionVYK> = normWireObject<ChangeActionVYK>({ mutation: normWireRequired(normWireLiteral("changeActionVYK")), memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newVYK: normWireRequired(normWireNumber) });
