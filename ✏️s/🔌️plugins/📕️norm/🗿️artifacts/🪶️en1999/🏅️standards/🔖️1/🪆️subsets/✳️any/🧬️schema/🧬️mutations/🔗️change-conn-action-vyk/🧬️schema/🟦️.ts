/** 🔗️ `change-conn-action-vyk` wire twin: the leaf payload `ChangeConnActionVYK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeConnActionVYK {
  mutation: "changeConnActionVYK";
  connectionId: string;
  actionId: string;
  newVYK: number;
}

export const parseChangeConnActionVYK: NormWireReader<ChangeConnActionVYK> = normWireObject<ChangeConnActionVYK>({ mutation: normWireRequired(normWireLiteral("changeConnActionVYK")), connectionId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newVYK: normWireRequired(normWireNumber) });
